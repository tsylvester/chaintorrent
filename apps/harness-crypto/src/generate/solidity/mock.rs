#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use core::marker::PhantomData;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams,
    IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::Root;
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, KemIdentity, SetupScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParams,
    DELIVERY_STATEMENT_VERSION_ONE, IDeliveryProofAdapter, IDeliveryProofConsumer,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use std::path::PathBuf;
use std::{env, process, thread};

use super::super::factory::provides::{
    IGenerateAdapter, MsmTermCountConstructorParamsOverrides,
    VectorCountConstructorParamsOverrides, VectorKindSettings, VectorKindSize, VectorKindSizing,
    build_msm_term_count, build_vector_count,
};
use super::challenge_vectors::provides::{ChallengeVectorsFn, mock_challenge_vectors};
use super::constants::provides::{ConstantsFn, mock_constants};
use super::group_vectors::provides::{GroupVectorsFn, mock_group_vectors};
use super::interface::{
    ConsumeSolidityGenerateCaseParams, ConsumeSolidityGenerateCasePayload,
    ISolidityGenerateCaseConsumer, RunSolidityGenerateCaseFn, SolidityGenerate,
    SolidityGenerateCaseDeps, SolidityGenerateCaseErrorReturn, SolidityGenerateCaseParams,
    SolidityGenerateCasePayload, SolidityGenerateCaseReturn, SolidityGenerateCaseSuccessReturn,
    SolidityGenerateConstructorParams,
};
use super::mapping_vectors::provides::{MappingVectorsFn, mock_mapping_vectors};
use super::possession_vectors::provides::{PossessionVectorsFn, mock_possession_vectors};
use super::proof_vectors::provides::{ProofVectorsFn, mock_proof_vectors};
use super::rejection_vectors::provides::{RejectionVectorsFn, mock_rejection_vectors};
use super::render::provides::{RenderFn, mock_render};
use super::settings::provides::{
    SolidityGenerateSettings, SolidityGenerateSettingsOverrides, build_solidity_generate_settings,
};
use crate::delivery::provides::AdmittedKemIdentity;

pub(crate) fn build_solidity_generate_root() -> PathBuf {
    env::temp_dir().join(format!(
        "chaintorrent-generate-{}-{:?}",
        process::id(),
        thread::current().id()
    ))
}

pub struct SolidityGenerateConstructorParamsOverrides<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub statement_version: Option<u16>,
    pub settings: Option<SolidityGenerateSettings>,
    pub sizes: Option<Vec<VectorKindSettings>>,
    pub render: Option<RenderFn>,
    pub constants: Option<ConstantsFn<'a, P, E>>,
    pub group_vectors: Option<GroupVectorsFn<'a, P>>,
    pub rejection_vectors: Option<RejectionVectorsFn<'a, P>>,
    pub mapping_vectors: Option<MappingVectorsFn<'a, P, K>>,
    pub possession_vectors: Option<PossessionVectorsFn<'a, P, A>>,
    pub challenge_vectors:
        Option<ChallengeVectorsFn<'a, P, E, <D as IDeliveryProofAdapter>::Forms>>,
    pub proof_vectors: Option<ProofVectorsFn<'a, P, K, A, D>>,
}

impl<'a, P, E, K, A, D> Default for SolidityGenerateConstructorParamsOverrides<'a, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    fn default() -> Self {
        Self {
            statement_version: None,
            settings: None,
            sizes: None,
            render: None,
            constants: None,
            group_vectors: None,
            rejection_vectors: None,
            mapping_vectors: None,
            possession_vectors: None,
            challenge_vectors: None,
            proof_vectors: None,
        }
    }
}

pub(crate) fn build_solidity_generate_constructor_params<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    overrides: SolidityGenerateConstructorParamsOverrides<'a, P, E, K, A, D>,
) -> SolidityGenerateConstructorParams<'a, P, E, K, A, D> {
    SolidityGenerateConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        delivery_proof,
        render: overrides.render.unwrap_or(mock_render),
        constants: overrides.constants.unwrap_or(mock_constants),
        group_vectors: overrides.group_vectors.unwrap_or(mock_group_vectors),
        rejection_vectors: overrides
            .rejection_vectors
            .unwrap_or(mock_rejection_vectors),
        mapping_vectors: overrides.mapping_vectors.unwrap_or(mock_mapping_vectors),
        possession_vectors: overrides
            .possession_vectors
            .unwrap_or(mock_possession_vectors),
        challenge_vectors: overrides
            .challenge_vectors
            .unwrap_or(mock_challenge_vectors),
        proof_vectors: overrides.proof_vectors.unwrap_or(mock_proof_vectors),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
        settings: overrides.settings.unwrap_or_else(|| {
            build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
                foundry_root: Some(build_solidity_generate_root()),
                ..Default::default()
            })
        }),
        sizes: overrides.sizes.unwrap_or_else(|| {
            <SolidityGenerate<'a, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                .kinds
                .iter()
                .map(|declaration| VectorKindSettings {
                    kind: declaration.kind,
                    size: match declaration.sizing {
                        VectorKindSizing::Drawn => VectorKindSize::Drawn {
                            count: build_vector_count(
                                VectorCountConstructorParamsOverrides::default(),
                            ),
                        },
                        VectorKindSizing::DrawnTerms => VectorKindSize::DrawnTerms {
                            count: build_vector_count(
                                VectorCountConstructorParamsOverrides::default(),
                            ),
                            terms: build_msm_term_count(
                                MsmTermCountConstructorParamsOverrides::default(),
                            ),
                        },
                        VectorKindSizing::DrawnMessages => VectorKindSize::DrawnMessages {
                            count: build_vector_count(
                                VectorCountConstructorParamsOverrides::default(),
                            ),
                            message_length: 32,
                        },
                        VectorKindSizing::Enumerated => VectorKindSize::Enumerated,
                        VectorKindSizing::OverConfiguredInputs => {
                            VectorKindSize::OverConfiguredInputs
                        }
                    },
                })
                .collect()
        }),
    }
}

pub(crate) fn build_solidity_generate<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    overrides: SolidityGenerateConstructorParamsOverrides<'a, P, E, K, A, D>,
) -> SolidityGenerate<'a, P, E, K, A, D> {
    let Ok(generate) = SolidityGenerate::try_new(build_solidity_generate_constructor_params(
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        delivery_proof,
        overrides,
    ));
    generate
}

#[derive(Default)]
pub struct SolidityGenerateCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_solidity_generate_case_params(
    overrides: SolidityGenerateCaseParamsOverrides,
) -> SolidityGenerateCaseParams {
    SolidityGenerateCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_create_chain_forms_params(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
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
pub struct SolidityGenerateCaseDepsOverrides;

pub fn build_solidity_generate_case_deps<C: ISolidityGenerateCaseConsumer>(
    consumer: C,
    _overrides: SolidityGenerateCaseDepsOverrides,
) -> SolidityGenerateCaseDeps<C> {
    SolidityGenerateCaseDeps { consumer }
}

#[derive(Default)]
pub struct SolidityGenerateCasePayloadOverrides<'a> {
    pub marker: PhantomData<&'a ()>,
}

pub fn build_solidity_generate_case_payload<'a>(
    canonicals: &'a [&'a [u8]],
    roots: &'a [&'a Root],
    _overrides: SolidityGenerateCasePayloadOverrides<'a>,
) -> SolidityGenerateCasePayload<'a> {
    SolidityGenerateCasePayload { canonicals, roots }
}

#[derive(Default)]
pub struct SolidityGenerateCaseSuccessReturnOverrides;

pub fn build_solidity_generate_case_success_return<O>(
    output: O,
    _overrides: SolidityGenerateCaseSuccessReturnOverrides,
) -> SolidityGenerateCaseSuccessReturn<O> {
    SolidityGenerateCaseSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeSolidityGenerateCaseParamsOverrides;

pub fn build_consume_solidity_generate_case_params(
    _overrides: ConsumeSolidityGenerateCaseParamsOverrides,
) -> ConsumeSolidityGenerateCaseParams {
    ConsumeSolidityGenerateCaseParams
}

#[derive(Default)]
pub struct ConsumeSolidityGenerateCasePayloadOverrides<'a> {
    pub marker: PhantomData<&'a ()>,
}

pub fn build_consume_solidity_generate_case_payload<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    identities: Vec<AdmittedKemIdentity<'a>>,
    _overrides: ConsumeSolidityGenerateCasePayloadOverrides<'a>,
) -> ConsumeSolidityGenerateCasePayload<'a, P, E, K, A, D> {
    ConsumeSolidityGenerateCasePayload {
        pairing,
        hash_to_scalar,
        encoder,
        decoder,
        kem,
        key_agreement,
        delivery_proof,
        random,
        identities,
    }
}

struct CaseAtDeliveryProof<'a, P: IPairingArithmetic, E, F, K, A, C: ISolidityGenerateCaseConsumer>
{
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    consumer: &'a C,
    identities: Vec<AdmittedKemIdentity<'a>>,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, A, C> IDeliveryProofConsumer<P, F>
    for CaseAtDeliveryProof<'a, P, E, F, K, A, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(SolidityGenerateCaseSuccessReturn {
            output: self.consumer.consume_solidity_generate_case(
                build_consume_solidity_generate_case_params(Default::default()),
                build_consume_solidity_generate_case_payload(
                    self.pairing,
                    self.hash_to_scalar,
                    self.encoder,
                    self.decoder,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.random,
                    self.identities.clone(),
                    Default::default(),
                ),
            ),
        })
    }
}

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, F, K, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    identities: Vec<AdmittedKemIdentity<'a>>,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.random,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    decoder: self.decoder,
                    random: self.random,
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    consumer: self.consumer,
                    identities: self.identities.clone(),
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
            Err(error) => Err(SolidityGenerateCaseErrorReturn::DeliveryProof(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let identities: Vec<AdmittedKemIdentity<'a>> = match payload.scope() {
            IdentityScope::Entitlement => self
                .payload
                .canonicals
                .iter()
                .map(|canonical| AdmittedKemIdentity {
                    setup_scope: SetupScope::Entitlement,
                    identity: KemIdentity::Entitlement { canonical },
                })
                .collect(),
            IdentityScope::Asset => self
                .payload
                .roots
                .iter()
                .map(|root| AdmittedKemIdentity {
                    setup_scope: SetupScope::Asset { root },
                    identity: KemIdentity::Asset { root },
                })
                .collect(),
        };
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.random,
                CaseAtKeyAgreement {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    decoder: self.decoder,
                    random: self.random,
                    kem: &payload.adapter,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                    identities,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(SolidityGenerateCaseErrorReturn::KeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hash_to_scalar,
                CaseAtKem {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    decoder: self.decoder,
                    random: self.random,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                    payload: self.payload,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(SolidityGenerateCaseErrorReturn::Kem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: &payload.adapter,
                    decoder: &payload.adapter,
                    random: self.random,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                    payload: self.payload,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(SolidityGenerateCaseErrorReturn::ChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: ISolidityGenerateCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    random_source: &'a CreateRandomSourceParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
}

impl<C: ISolidityGenerateCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = SolidityGenerateCaseReturn<C::Output>;

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
                return Err(SolidityGenerateCaseErrorReturn::HashToScalar(error));
            }
        };
        let source = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(SolidityGenerateCaseErrorReturn::RandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hash_to_scalar: &*hashing.adapter,
                random: &*source.adapter,
                chain_forms: self.chain_forms,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
                consumer: self.consumer,
                payload: self.payload,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(SolidityGenerateCaseErrorReturn::Encoding(error)),
        }
    }
}

pub fn run_solidity_generate_case<C: ISolidityGenerateCaseConsumer>(
    deps: &SolidityGenerateCaseDeps<C>,
    params: SolidityGenerateCaseParams,
    payload: SolidityGenerateCasePayload<'_>,
) -> SolidityGenerateCaseReturn<C::Output> {
    let _: RunSolidityGenerateCaseFn<C> = run_solidity_generate_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                kem: &params.kem,
                key_agreement: &params.key_agreement,
                delivery_proof: &params.delivery_proof,
                random_source: &params.random_source,
                consumer: &deps.consumer,
                payload: &payload,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(SolidityGenerateCaseErrorReturn::Pairing(error)),
    }
}
