#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use domain::{Secret, SecretConstructorParams};
use envelope::{
    EnvelopeCoins, EnvelopeComponentsParams, EnvelopeComponentsPayload, GenerateKeysParams,
    GenerateKeysPayload, IKeyAgreementAdapter, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, WrapToParams, WrapToPayload,
};
use kem::{
    CredentialComponentsParams, CredentialComponentsPayload, DeriveIdentityParams,
    DeriveIdentityPayload, ICredentialKemAdapter, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, IssueParams, IssuePayload, MasterScalarComponentsParams,
    MasterScalarComponentsPayload, ParameterSetComponentsParams, ParameterSetComponentsPayload,
    SetupParams, SetupPayload,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};
use proof::{AlgebraicMintStatement, IDeliveryProofAdapter, ProveMintParams, ProveMintPayload};
use random::{FillBytesParams, FillBytesPayload};

use interface::{
    DeliveryHolder, MintDelivery, MintDeps, MintErrorReturn, MintParams, MintPayload, MintReturn,
    MintSuccessReturn, MintWitnesses,
};

pub fn mint<'a, P, K, A, D>(
    deps: &MintDeps<'a, K, A, D>,
    _params: MintParams,
    payload: MintPayload<'_, D::Forms>,
) -> MintReturn<P, K, A, D>
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    let mut draws = Vec::new();
    let draw = |draws: &mut Vec<u8>| {
        deps.random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            )
            .map(|success| {
                draws.extend_from_slice(success.bytes.expose());
                success.bytes
            })
            .map_err(MintErrorReturn::FillBytes)
    };
    let copy_secret = |secret: &Secret<P::Scalar>| {
        let Ok(copied) = Secret::try_new(SecretConstructorParams {
            value: secret.expose().clone(),
        });
        copied
    };
    let setup = deps
        .kem
        .setup(
            SetupParams {
                scope: payload.identity.setup_scope,
            },
            SetupPayload {
                master_uniform: draw(&mut draws)?,
                u0_uniform: draw(&mut draws)?,
                u1_uniform: draw(&mut draws)?,
            },
        )
        .map_err(MintErrorReturn::Setup)?;
    let derived = deps
        .kem
        .derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &setup.parameter_set,
                identity: payload.identity.identity,
            },
        )
        .map_err(MintErrorReturn::DeriveIdentity)?;
    let issued = deps
        .kem
        .issue(
            IssueParams,
            IssuePayload {
                parameter_set: &setup.parameter_set,
                master_scalar: &setup.master_scalar,
                identity_element: &derived.identity_element,
                uniform: draw(&mut draws)?,
            },
        )
        .map_err(MintErrorReturn::Issue)?;
    let generated = deps
        .key_agreement
        .generate_keys(
            GenerateKeysParams,
            GenerateKeysPayload {
                x_uniform: draw(&mut draws)?,
                y_uniform: draw(&mut draws)?,
            },
        )
        .map_err(MintErrorReturn::GenerateKeys)?;
    let Ok(key_public_keys) = deps.key_agreement.key_pair_public_keys(
        KeyPairPublicKeysParams,
        KeyPairPublicKeysPayload {
            key_pair: &generated.key_pair,
        },
    );
    let admitted = deps
        .key_agreement
        .public_keys_from_components(
            PublicKeysFromComponentsParams,
            PublicKeysFromComponentsPayload {
                components: key_public_keys.components,
                possession: &generated.possession,
            },
        )
        .map_err(MintErrorReturn::PublicKeysFromComponents)?;
    let Ok(credential) = deps.kem.credential_components(
        CredentialComponentsParams,
        CredentialComponentsPayload {
            credential: &issued.credential,
        },
    );
    let wrapped = deps
        .key_agreement
        .wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &admitted.public_keys,
                credential: credential.components,
            },
        )
        .map_err(MintErrorReturn::WrapTo)?;
    let Ok(parameter_set_components) = deps.kem.parameter_set_components(
        ParameterSetComponentsParams,
        ParameterSetComponentsPayload {
            parameter_set: &setup.parameter_set,
        },
    );
    let Ok(identity_element_components) = deps.kem.identity_element_components(
        IdentityElementComponentsParams,
        IdentityElementComponentsPayload {
            identity_element: &derived.identity_element,
        },
    );
    let Ok(buyer_public_keys) = deps.key_agreement.key_pair_public_keys(
        KeyPairPublicKeysParams,
        KeyPairPublicKeysPayload {
            key_pair: &generated.key_pair,
        },
    );
    let Ok(envelope_components) = deps.key_agreement.envelope_components(
        EnvelopeComponentsParams,
        EnvelopeComponentsPayload {
            envelope: &wrapped.envelope,
        },
    );
    let statement = AlgebraicMintStatement {
        context: payload.context.clone(),
        hpub: parameter_set_components.components.hpub,
        identity_element: identity_element_components.components.element,
        buyer_keys: buyer_public_keys.components,
        envelope: envelope_components.components,
    };
    let Ok(prover_master_scalar) = deps.kem.master_scalar_components(
        MasterScalarComponentsParams,
        MasterScalarComponentsPayload {
            master_scalar: &setup.master_scalar,
        },
    );
    let Ok(witness_master_scalar) = deps.kem.master_scalar_components(
        MasterScalarComponentsParams,
        MasterScalarComponentsPayload {
            master_scalar: &setup.master_scalar,
        },
    );
    let credential_randomness = copy_secret(&issued.randomness);
    let rho = copy_secret(&wrapped.coins.rho);
    let sigma = copy_secret(&wrapped.coins.sigma);
    let proved = deps
        .delivery_proof
        .prove_mint(
            ProveMintParams,
            ProveMintPayload {
                master_scalar: prover_master_scalar.components,
                credential_randomness: issued.randomness,
                coins: wrapped.coins,
                statement: &statement,
            },
        )
        .map_err(MintErrorReturn::ProveMint)?;
    Ok(MintSuccessReturn {
        delivery: MintDelivery {
            statement,
            proof: proved.proof,
            parameter_set: setup.parameter_set,
            identity_element: derived.identity_element,
            credential: issued.credential,
            witnesses: MintWitnesses {
                master_scalar: witness_master_scalar.components,
                credential_randomness,
                coins: EnvelopeCoins { rho, sigma },
            },
            recipient: DeliveryHolder {
                key_pair: generated.key_pair,
                public_keys: admitted.public_keys,
                envelope: wrapped.envelope,
            },
            draws,
        },
    })
}
