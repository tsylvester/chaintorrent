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
    GenerateKeysPayload, IKeyAgreementAdapter, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, PublicKeysComponentsParams,
    PublicKeysComponentsPayload, PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload,
    UnwrapParams, UnwrapPayload, WrapToParams, WrapToPayload,
};
use kem::{
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, ICredentialKemAdapter, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, RerandomizeParams, RerandomizePayload,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};
use proof::{
    AlgebraicTransferStatement, IDeliveryProofAdapter, ProveTransferParams, ProveTransferPayload,
};
use random::{FillBytesParams, FillBytesPayload};

use crate::delivery::mint::provides::DeliveryHolder;
use interface::{
    TransferDelivery, TransferDeps, TransferErrorReturn, TransferParams, TransferPayload,
    TransferReturn, TransferSuccessReturn, TransferWitnesses,
};

pub fn transfer<'a, P, K, A, D>(
    deps: &TransferDeps<'a, K, A, D>,
    _params: TransferParams,
    payload: TransferPayload<'_, K, A, D::Forms>,
) -> TransferReturn<P, K, A, D>
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
            .map_err(TransferErrorReturn::FillBytes)
    };
    let copy_secret = |secret: &Secret<P::Scalar>| {
        let Ok(copied) = Secret::try_new(SecretConstructorParams {
            value: secret.expose().clone(),
        });
        copied
    };
    let Ok(opened) = deps.key_agreement.unwrap(
        UnwrapParams,
        UnwrapPayload {
            key_pair: &payload.holder.key_pair,
            envelope: &payload.holder.envelope,
        },
    );
    let Ok(opened_credential) = deps.kem.credential_from_components(
        CredentialFromComponentsParams,
        CredentialFromComponentsPayload {
            components: opened.credential,
        },
    );
    let generated = deps
        .key_agreement
        .generate_keys(
            GenerateKeysParams,
            GenerateKeysPayload {
                x_uniform: draw(&mut draws)?,
                y_uniform: draw(&mut draws)?,
            },
        )
        .map_err(TransferErrorReturn::GenerateKeys)?;
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
        .map_err(TransferErrorReturn::PublicKeysFromComponents)?;
    let rerandomized = deps
        .kem
        .rerandomize(
            RerandomizeParams,
            RerandomizePayload {
                parameter_set: payload.parameter_set,
                identity_element: payload.identity_element,
                credential: &opened_credential.credential,
                uniform: draw(&mut draws)?,
            },
        )
        .map_err(TransferErrorReturn::Rerandomize)?;
    let Ok(credential) = deps.kem.credential_components(
        CredentialComponentsParams,
        CredentialComponentsPayload {
            credential: &rerandomized.credential,
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
        .map_err(TransferErrorReturn::WrapTo)?;
    let Ok(identity_element_components) = deps.kem.identity_element_components(
        IdentityElementComponentsParams,
        IdentityElementComponentsPayload {
            identity_element: payload.identity_element,
        },
    );
    let Ok(seller_keys) = deps.key_agreement.public_keys_components(
        PublicKeysComponentsParams,
        PublicKeysComponentsPayload {
            public_keys: &payload.holder.public_keys,
        },
    );
    let Ok(buyer_keys) = deps.key_agreement.key_pair_public_keys(
        KeyPairPublicKeysParams,
        KeyPairPublicKeysPayload {
            key_pair: &generated.key_pair,
        },
    );
    let Ok(old_envelope) = deps.key_agreement.envelope_components(
        EnvelopeComponentsParams,
        EnvelopeComponentsPayload {
            envelope: &payload.holder.envelope,
        },
    );
    let Ok(new_envelope) = deps.key_agreement.envelope_components(
        EnvelopeComponentsParams,
        EnvelopeComponentsPayload {
            envelope: &wrapped.envelope,
        },
    );
    let statement = AlgebraicTransferStatement {
        context: payload.context.clone(),
        identity_element: identity_element_components.components.element,
        seller_keys: seller_keys.components,
        buyer_keys: buyer_keys.components,
        old_envelope: old_envelope.components,
        new_envelope: new_envelope.components,
    };
    let Ok(prover_secrets) = deps.key_agreement.key_pair_components(
        KeyPairComponentsParams,
        KeyPairComponentsPayload {
            key_pair: &payload.holder.key_pair,
        },
    );
    let Ok(witness_secrets) = deps.key_agreement.key_pair_components(
        KeyPairComponentsParams,
        KeyPairComponentsPayload {
            key_pair: &payload.holder.key_pair,
        },
    );
    let offset = copy_secret(&rerandomized.offset);
    let rho = copy_secret(&wrapped.coins.rho);
    let sigma = copy_secret(&wrapped.coins.sigma);
    let proved = deps
        .delivery_proof
        .prove_transfer(
            ProveTransferParams,
            ProveTransferPayload {
                seller_secrets: prover_secrets.components,
                offset: rerandomized.offset,
                coins: wrapped.coins,
                statement: &statement,
            },
        )
        .map_err(TransferErrorReturn::ProveTransfer)?;
    Ok(TransferSuccessReturn {
        delivery: TransferDelivery {
            statement,
            proof: proved.proof,
            credential: rerandomized.credential,
            witnesses: TransferWitnesses {
                seller_secrets: witness_secrets.components,
                offset,
                coins: EnvelopeCoins { rho, sigma },
            },
            buyer: DeliveryHolder {
                key_pair: generated.key_pair,
                public_keys: admitted.public_keys,
                envelope: wrapped.envelope,
            },
            draws,
        },
    })
}
