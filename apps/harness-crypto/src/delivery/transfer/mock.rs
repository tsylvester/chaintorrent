#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_payload,
    create_chain_forms,
};
use core::marker::PhantomData;
use domain::{Secret, SecretConstructorParams};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_payload,
    create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams, EnvelopeCoins,
    EnvelopeComponentsParams, EnvelopeComponentsPayload, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, IKeyAgreementConsumer, KeyPairComponents, KeyPairComponentsParams,
    KeyPairComponentsPayload, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    PublicKeysComponentsParams, PublicKeysComponentsPayload, PublicKeysFromComponentsParams,
    PublicKeysFromComponentsPayload, UnwrapParams, UnwrapPayload, WrapToParams, WrapToPayload,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, build_envelope_coins, build_key_pair_components,
    create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::Root;
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, CredentialComponentsParams,
    CredentialComponentsPayload, CredentialFromComponentsParams, CredentialFromComponentsPayload,
    ICredentialKemAdapter, IKemConsumer, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, IdentityScope, KemIdentity, RerandomizeParams,
    RerandomizePayload, SetupScope, build_create_kem_deps, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, IPairingAdapter, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_deps, build_create_pairing_payload,
    create_pairing,
};
use proof::{
    AlgebraicTransferStatement, ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload,
    CreateDeliveryProofParams, DeliveryContext, IDeliveryProofAdapter, IDeliveryProofConsumer,
    ProveTransferParams, ProveTransferPayload, TransferPurpose, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceParams, IRandomSourceAdapter, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};

use crate::delivery::mint::provides::{
    AdmittedKemIdentity, DeliveryHolder, MintCaseParams, build_mint_case_params,
};

use super::interface::{
    ITransferCaseConsumer, RunTransferCaseFn, TransferCaseDeps, TransferCaseErrorReturn,
    TransferCaseParams, TransferCasePayload, TransferCaseReturn, TransferCaseSuccessReturn,
    TransferDelivery, TransferDeps, TransferErrorReturn, TransferFn, TransferParams,
    TransferPayload, TransferReturn, TransferSuccessReturn, TransferWitnesses,
};

#[derive(Default)]
pub struct TransferParamsOverrides;

pub fn build_transfer_params(_overrides: TransferParamsOverrides) -> TransferParams {
    TransferParams
}

#[derive(Default)]
pub struct TransferPayloadOverrides;

pub fn build_transfer_payload<
    'a,
    K: ICredentialKemAdapter,
    A: IKeyAgreementAdapter,
    F: IChainForms,
>(
    context: &'a DeliveryContext<F, TransferPurpose>,
    parameter_set: &'a K::ParameterSet,
    identity_element: &'a K::IdentityElement,
    holder: &'a DeliveryHolder<A>,
    _overrides: TransferPayloadOverrides,
) -> TransferPayload<'a, K, A, F> {
    TransferPayload {
        context,
        parameter_set,
        identity_element,
        holder,
    }
}

#[derive(Default)]
pub struct TransferDepsOverrides;

pub fn build_transfer_deps<
    'a,
    K: ICredentialKemAdapter,
    A: IKeyAgreementAdapter<Pairing = K::Pairing>,
    D: IDeliveryProofAdapter<Pairing = K::Pairing>,
>(
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: TransferDepsOverrides,
) -> TransferDeps<'a, K, A, D> {
    TransferDeps {
        kem,
        key_agreement,
        delivery_proof,
        random,
    }
}

pub struct TransferWitnessesOverrides<P: IPairingAdapter> {
    pub seller_secrets: Option<KeyPairComponents<P::Scalar>>,
    pub offset: Option<Secret<P::Scalar>>,
    pub coins: Option<EnvelopeCoins<P::Scalar>>,
}

impl<P: IPairingAdapter> Default for TransferWitnessesOverrides<P> {
    fn default() -> Self {
        Self {
            seller_secrets: None,
            offset: None,
            coins: None,
        }
    }
}

fn sampled_scalar<P: IPairingAdapter>(byte: u8) -> Secret<P::Scalar> {
    let Ok(uniform) = Secret::try_new(SecretConstructorParams {
        value: vec![byte; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
    });
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload { uniform },
    )
    .expect("constant bytes of the declared length are a valid scalar")
    .scalar
}

pub fn build_transfer_witnesses<P: IPairingAdapter>(
    pairing: &P,
    overrides: TransferWitnessesOverrides<P>,
) -> TransferWitnesses<P> {
    TransferWitnesses {
        seller_secrets: overrides
            .seller_secrets
            .unwrap_or_else(|| build_key_pair_components(pairing, Default::default())),
        offset: overrides
            .offset
            .unwrap_or_else(|| sampled_scalar::<P>(0x03)),
        coins: overrides
            .coins
            .unwrap_or_else(|| build_envelope_coins(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct TransferDeliveryOverrides {
    pub draws: Option<Vec<u8>>,
}

pub fn build_transfer_delivery<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    proof: D::Proof,
    credential: K::Credential,
    witnesses: TransferWitnesses<P>,
    buyer: DeliveryHolder<A>,
    overrides: TransferDeliveryOverrides,
) -> TransferDelivery<P, K, A, D> {
    TransferDelivery {
        statement,
        proof,
        credential,
        witnesses,
        buyer,
        draws: overrides.draws.unwrap_or_else(|| {
            (0x01u8..=0x03u8)
                .flat_map(|byte| {
                    vec![byte; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH]
                })
                .collect()
        }),
    }
}

#[derive(Default)]
pub struct TransferSuccessReturnOverrides;

pub fn build_transfer_success_return<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    delivery: TransferDelivery<P, K, A, D>,
    _overrides: TransferSuccessReturnOverrides,
) -> TransferSuccessReturn<P, K, A, D> {
    TransferSuccessReturn { delivery }
}

fn uniform<S: ISampleUniformScalar>(byte: u8) -> Secret<Vec<u8>> {
    let Ok(uniform) = Secret::try_new(SecretConstructorParams {
        value: vec![byte; S::UNIFORM_BYTES_LENGTH],
    });
    uniform
}

fn copy_secret<P: IPairingAdapter>(secret: &Secret<P::Scalar>) -> Secret<P::Scalar> {
    let Ok(copy) = Secret::try_new(SecretConstructorParams {
        value: secret.expose().clone(),
    });
    copy
}

pub fn mock_transfer<'a, P, K, A, D>(
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
    let _: TransferFn<'a, P, K, A, D> = mock_transfer;
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
    let generated = match deps.key_agreement.generate_keys(
        GenerateKeysParams,
        GenerateKeysPayload {
            x_uniform: uniform::<P::Scalar>(0x01),
            y_uniform: uniform::<P::Scalar>(0x02),
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(TransferErrorReturn::GenerateKeys(error)),
    };
    let Ok(key_public_keys) = deps.key_agreement.key_pair_public_keys(
        KeyPairPublicKeysParams,
        KeyPairPublicKeysPayload {
            key_pair: &generated.key_pair,
        },
    );
    let admitted = match deps.key_agreement.public_keys_from_components(
        PublicKeysFromComponentsParams,
        PublicKeysFromComponentsPayload {
            components: key_public_keys.components,
            possession: &generated.possession,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(TransferErrorReturn::PublicKeysFromComponents(error)),
    };
    let rerandomized = match deps.kem.rerandomize(
        RerandomizeParams,
        RerandomizePayload {
            parameter_set: payload.parameter_set,
            identity_element: payload.identity_element,
            credential: &opened_credential.credential,
            uniform: uniform::<P::Scalar>(0x03),
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(TransferErrorReturn::Rerandomize(error)),
    };
    let Ok(credential) = deps.kem.credential_components(
        CredentialComponentsParams,
        CredentialComponentsPayload {
            credential: &rerandomized.credential,
        },
    );
    let wrapped = match deps.key_agreement.wrap_to(
        WrapToParams,
        WrapToPayload {
            public_keys: &admitted.public_keys,
            credential: credential.components,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(TransferErrorReturn::WrapTo(error)),
    };
    let Ok(identity_element) = deps.kem.identity_element_components(
        IdentityElementComponentsParams,
        IdentityElementComponentsPayload {
            identity_element: payload.identity_element,
        },
    );
    let Ok(seller_public_keys) = deps.key_agreement.public_keys_components(
        PublicKeysComponentsParams,
        PublicKeysComponentsPayload {
            public_keys: &payload.holder.public_keys,
        },
    );
    let Ok(buyer_public_keys) = deps.key_agreement.key_pair_public_keys(
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
        identity_element: identity_element.components.element,
        seller_keys: seller_public_keys.components,
        buyer_keys: buyer_public_keys.components,
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
    let offset = copy_secret::<P>(&rerandomized.offset);
    let rho = copy_secret::<P>(&wrapped.coins.rho);
    let sigma = copy_secret::<P>(&wrapped.coins.sigma);
    let proved = match deps.delivery_proof.prove_transfer(
        ProveTransferParams,
        ProveTransferPayload {
            seller_secrets: prover_secrets.components,
            offset: rerandomized.offset,
            coins: wrapped.coins,
            statement: &statement,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(TransferErrorReturn::ProveTransfer(error)),
    };
    Ok(build_transfer_success_return(
        build_transfer_delivery(
            statement,
            proved.proof,
            rerandomized.credential,
            TransferWitnesses {
                seller_secrets: witness_secrets.components,
                offset,
                coins: EnvelopeCoins { rho, sigma },
            },
            DeliveryHolder {
                key_pair: generated.key_pair,
                public_keys: admitted.public_keys,
                envelope: wrapped.envelope,
            },
            Default::default(),
        ),
        Default::default(),
    ))
}

#[derive(Default)]
pub struct TransferCaseParamsOverrides {
    pub mint: Option<MintCaseParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_transfer_case_params(overrides: TransferCaseParamsOverrides) -> TransferCaseParams {
    TransferCaseParams {
        mint: overrides
            .mint
            .unwrap_or_else(|| build_mint_case_params(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_create_key_agreement_params(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct TransferCaseDepsOverrides;

pub fn build_transfer_case_deps<C: ITransferCaseConsumer>(
    consumer: C,
    _overrides: TransferCaseDepsOverrides,
) -> TransferCaseDeps<C> {
    TransferCaseDeps { consumer }
}

#[derive(Default)]
pub struct TransferCasePayloadOverrides;

pub fn build_transfer_case_payload<'a>(
    canonical: &'a [u8],
    identity_hash: &'a Root,
    _overrides: TransferCasePayloadOverrides,
) -> TransferCasePayload<'a> {
    TransferCasePayload {
        canonical,
        identity_hash,
    }
}

#[derive(Default)]
pub struct TransferCaseSuccessReturnOverrides;

pub fn build_transfer_case_success_return<O>(
    output: O,
    _overrides: TransferCaseSuccessReturnOverrides,
) -> TransferCaseSuccessReturn<O> {
    TransferCaseSuccessReturn { output }
}

struct CaseAtTransferDeliveryProof<
    'a,
    P: IPairingArithmetic,
    F: IChainForms,
    K,
    A1,
    A2,
    D1,
    C: ITransferCaseConsumer,
> {
    pairing: &'a P,
    kem: &'a K,
    key_agreement: &'a A1,
    transfer_key_agreement: &'a A2,
    delivery_proof: &'a D1,
    mint_source: &'a dyn IRandomSourceAdapter,
    transfer_source: &'a dyn IRandomSourceAdapter,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, F, K, A1, A2, D1, C> IDeliveryProofConsumer<P, F>
    for CaseAtTransferDeliveryProof<'a, P, F, K, A1, A2, D1, C>
where
    P: IPairingArithmetic + IPairingReference,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A1: IKeyAgreementAdapter<Pairing = P>,
    A2: IKeyAgreementAdapter<Pairing = P>,
    D1: IDeliveryProofAdapter<Pairing = P, Forms = F>,
    C: ITransferCaseConsumer,
{
    type Output = TransferCaseReturn<C::Output>;

    fn consume_delivery_proof<D2: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D2>,
    ) -> Self::Output {
        Ok(TransferCaseSuccessReturn {
            output: self.consumer.consume_transfer_case(
                self.pairing,
                self.kem,
                self.key_agreement,
                self.transfer_key_agreement,
                self.delivery_proof,
                &payload.adapter,
                self.mint_source,
                self.transfer_source,
                self.identity,
            ),
        })
    }
}

struct CaseAtDeliveryProof<
    'a,
    P: IPairingArithmetic,
    E,
    F: IChainForms,
    K,
    A1,
    A2,
    C: ITransferCaseConsumer,
> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    mint_source: &'a dyn IRandomSourceAdapter,
    transfer_source: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A1,
    transfer_key_agreement: &'a A2,
    transfer_delivery_proof: &'a CreateDeliveryProofParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, A1, A2, C> IDeliveryProofConsumer<P, F>
    for CaseAtDeliveryProof<'a, P, E, F, K, A1, A2, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A1: IKeyAgreementAdapter<Pairing = P>,
    A2: IKeyAgreementAdapter<Pairing = P>,
    C: ITransferCaseConsumer,
{
    type Output = TransferCaseReturn<C::Output>;

    fn consume_delivery_proof<D1: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D1>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.transfer_source,
                CaseAtTransferDeliveryProof {
                    pairing: self.pairing,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    transfer_key_agreement: self.transfer_key_agreement,
                    delivery_proof: &payload.adapter,
                    mint_source: self.mint_source,
                    transfer_source: self.transfer_source,
                    identity: self.identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            CreateDeliveryProofParams {
                concrete: self.transfer_delivery_proof.concrete,
                algebra: self.transfer_delivery_proof.algebra,
                statement_version: self.transfer_delivery_proof.statement_version,
            },
            build_create_delivery_proof_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(TransferCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtTransferKeyAgreement<
    'a,
    P: IPairingArithmetic,
    E,
    F: IChainForms,
    K,
    A1,
    C: ITransferCaseConsumer,
> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    mint_source: &'a dyn IRandomSourceAdapter,
    transfer_source: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A1,
    delivery_proof: &'a CreateDeliveryProofParams,
    transfer_delivery_proof: &'a CreateDeliveryProofParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, A1, C> IKeyAgreementConsumer<P>
    for CaseAtTransferKeyAgreement<'a, P, E, F, K, A1, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A1: IKeyAgreementAdapter<Pairing = P>,
    C: ITransferCaseConsumer,
{
    type Output = TransferCaseReturn<C::Output>;

    fn consume_key_agreement<A2: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A2>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.mint_source,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    mint_source: self.mint_source,
                    transfer_source: self.transfer_source,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    transfer_key_agreement: &payload.adapter,
                    transfer_delivery_proof: self.transfer_delivery_proof,
                    identity: self.identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            CreateDeliveryProofParams {
                concrete: self.delivery_proof.concrete,
                algebra: self.delivery_proof.algebra,
                statement_version: self.delivery_proof.statement_version,
            },
            build_create_delivery_proof_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(TransferCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, F: IChainForms, K, C: ITransferCaseConsumer>
{
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    mint_source: &'a dyn IRandomSourceAdapter,
    transfer_source: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    transfer_key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    transfer_delivery_proof: &'a CreateDeliveryProofParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: ITransferCaseConsumer,
{
    type Output = TransferCaseReturn<C::Output>;

    fn consume_key_agreement<A1: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A1>,
    ) -> Self::Output {
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.transfer_source,
                CaseAtTransferKeyAgreement {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    mint_source: self.mint_source,
                    transfer_source: self.transfer_source,
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    delivery_proof: self.delivery_proof,
                    transfer_delivery_proof: self.transfer_delivery_proof,
                    identity: self.identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.transfer_key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(TransferCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F: IChainForms, C: ITransferCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    mint_source: &'a dyn IRandomSourceAdapter,
    transfer_source: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    transfer_key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    transfer_delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a TransferCasePayload<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
    C: ITransferCaseConsumer,
{
    type Output = TransferCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let identity = match payload.scope() {
            IdentityScope::Entitlement => AdmittedKemIdentity {
                setup_scope: SetupScope::Entitlement,
                identity: KemIdentity::Entitlement {
                    canonical: self.payload.canonical,
                },
            },
            IdentityScope::Asset => AdmittedKemIdentity {
                setup_scope: SetupScope::Asset {
                    root: self.payload.identity_hash,
                },
                identity: KemIdentity::Asset {
                    root: self.payload.identity_hash,
                },
            },
        };
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.mint_source,
                CaseAtKeyAgreement {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    mint_source: self.mint_source,
                    transfer_source: self.transfer_source,
                    kem: &payload.adapter,
                    transfer_key_agreement: self.transfer_key_agreement,
                    delivery_proof: self.delivery_proof,
                    transfer_delivery_proof: self.transfer_delivery_proof,
                    identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(TransferCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: ITransferCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    mint_source: &'a dyn IRandomSourceAdapter,
    transfer_source: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    transfer_key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    transfer_delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a TransferCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    C: ITransferCaseConsumer,
{
    type Output = TransferCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hashing,
                CaseAtKem {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    mint_source: self.mint_source,
                    transfer_source: self.transfer_source,
                    key_agreement: self.key_agreement,
                    transfer_key_agreement: self.transfer_key_agreement,
                    delivery_proof: self.delivery_proof,
                    transfer_delivery_proof: self.transfer_delivery_proof,
                    payload: self.payload,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(TransferCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: ITransferCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    mint_source: &'a dyn IRandomSourceAdapter,
    transfer_source: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    transfer_key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    transfer_delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a TransferCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: ITransferCaseConsumer,
{
    type Output = TransferCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: &payload.adapter,
                    mint_source: self.mint_source,
                    transfer_source: self.transfer_source,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    transfer_key_agreement: self.transfer_key_agreement,
                    delivery_proof: self.delivery_proof,
                    transfer_delivery_proof: self.transfer_delivery_proof,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(TransferCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: ITransferCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    transfer_random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    transfer_key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    transfer_delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a TransferCasePayload<'a>,
    consumer: &'a C,
}

impl<C: ITransferCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = TransferCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hashing = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            *self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(hashing) => hashing,
            Err(error) => {
                return Err(TransferCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let mint_source = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(TransferCaseErrorReturn::CreateRandomSource(error));
            }
        };
        let transfer_source = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.transfer_random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(TransferCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hashing: &*hashing.adapter,
                mint_source: &*mint_source.adapter,
                transfer_source: &*transfer_source.adapter,
                chain_forms: self.chain_forms,
                kem: self.kem,
                key_agreement: self.key_agreement,
                transfer_key_agreement: self.transfer_key_agreement,
                delivery_proof: self.delivery_proof,
                transfer_delivery_proof: self.transfer_delivery_proof,
                payload: self.payload,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(TransferCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_transfer_case<C: ITransferCaseConsumer>(
    deps: &TransferCaseDeps<C>,
    params: TransferCaseParams,
    payload: TransferCasePayload<'_>,
) -> TransferCaseReturn<C::Output> {
    let _: RunTransferCaseFn<C> = run_transfer_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.mint.hash_to_scalar,
                random_source: &params.mint.random_source,
                transfer_random_source: &params.random_source,
                encoding: &params.mint.encoding,
                chain_forms: &params.mint.chain_forms,
                kem: &params.mint.kem,
                key_agreement: &params.mint.key_agreement,
                transfer_key_agreement: &params.key_agreement,
                delivery_proof: &params.mint.delivery_proof,
                transfer_delivery_proof: &params.delivery_proof,
                payload: &payload,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.mint.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(TransferCaseErrorReturn::CreatePairing(error)),
    }
}
