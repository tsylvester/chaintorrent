#![allow(dead_code, clippy::too_many_arguments)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
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
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    AlgebraicTransferStatement, ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload,
    CreateDeliveryProofParams, DeliveryContext, IDeliveryProofAdapter, IDeliveryProofConsumer,
    MintPurpose, TransferPurpose, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceParams, IRandomSourceAdapter, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};
use serde_json::{Map, Value};

use super::interface::{
    IReplayCaseConsumer, ReplayCaseDeps, ReplayCaseErrorReturn, ReplayCaseParams,
    ReplayCasePayload, ReplayCaseReturn, ReplayCaseSuccessReturn, ReplayDeps, ReplayFn,
    ReplayParams, ReplayPayload, ReplayReturn, ReplaySuccessReturn, ReplayVector, RunReplayCaseFn,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct ReplayVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub transfer_is_valid: Option<bool>,
    pub next_is_valid: Option<bool>,
    pub replay_is_valid: Option<bool>,
}

pub fn build_replay_vector<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>>(
    transfer_statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    transfer_proof: D::Proof,
    next_statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    next_proof: D::Proof,
    overrides: ReplayVectorOverrides,
) -> ReplayVector<P, D> {
    ReplayVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        transfer_statement,
        transfer_proof,
        transfer_is_valid: overrides.transfer_is_valid.unwrap_or(true),
        next_statement,
        next_proof,
        next_is_valid: overrides.next_is_valid.unwrap_or(true),
        replay_is_valid: overrides.replay_is_valid.unwrap_or(false),
    }
}

#[derive(Default)]
pub struct ReplayParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_replay_params(overrides: ReplayParamsOverrides) -> ReplayParams {
    ReplayParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

fn replay_params_fields(params: &ReplayParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields
}

#[derive(Default)]
pub struct ReplayParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_replay_params(corruptions: ReplayParamsCorruptions) -> Value {
    let mut fields = replay_params_fields(&build_replay_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ReplayPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_replay_payload<'a, F: IChainForms>(
    mint: &'a DeliveryContext<F, MintPurpose>,
    transfer: &'a DeliveryContext<F, TransferPurpose>,
    next: &'a DeliveryContext<F, TransferPurpose>,
    overrides: ReplayPayloadOverrides<'a>,
) -> ReplayPayload<'a, F> {
    ReplayPayload {
        mint,
        transfer,
        next,
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
    }
}

#[derive(Default)]
pub struct ReplayDepsOverrides;

pub fn build_replay_deps<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: ReplayDepsOverrides,
) -> ReplayDeps<'a, P, K, A, D> {
    ReplayDeps {
        kem,
        key_agreement,
        delivery_proof,
        random,
    }
}

pub struct ReplaySuccessReturnOverrides<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> {
    pub vectors: Option<Vec<ReplayVector<P, D>>>,
}

impl<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> Default
    for ReplaySuccessReturnOverrides<P, D>
{
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_replay_success_return<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>>(
    overrides: ReplaySuccessReturnOverrides<P, D>,
) -> ReplaySuccessReturn<P, D> {
    ReplaySuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_replay<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    _deps: &ReplayDeps<'a, P, K, A, D>,
    _params: ReplayParams,
    _payload: ReplayPayload<'_, D::Forms>,
) -> ReplayReturn<P, D> {
    let _: ReplayFn<'a, P, K, A, D> = mock_replay;
    Ok(build_replay_success_return(Default::default()))
}

#[derive(Default)]
pub struct ReplayCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_replay_case_params(overrides: ReplayCaseParamsOverrides) -> ReplayCaseParams {
    ReplayCaseParams {
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
pub struct ReplayCaseDepsOverrides;

pub fn build_replay_case_deps<C: IReplayCaseConsumer>(
    consumer: C,
    _overrides: ReplayCaseDepsOverrides,
) -> ReplayCaseDeps<C> {
    ReplayCaseDeps { consumer }
}

#[derive(Default)]
pub struct ReplayCasePayloadOverrides;

pub fn build_replay_case_payload<'a>(
    canonical: &'a [u8],
    root: &'a Root,
    _overrides: ReplayCasePayloadOverrides,
) -> ReplayCasePayload<'a> {
    ReplayCasePayload { canonical, root }
}

#[derive(Default)]
pub struct ReplayCaseSuccessReturnOverrides;

pub fn build_replay_case_success_return<O>(
    output: O,
    _overrides: ReplayCaseSuccessReturnOverrides,
) -> ReplayCaseSuccessReturn<O> {
    ReplayCaseSuccessReturn { output }
}

struct CaseAtDeliveryProof<'a, P: IPairingArithmetic, E, F, K, A, C> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    forms: &'a F,
    kem: &'a K,
    key_agreement: &'a A,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, K, A, C> IDeliveryProofConsumer<P, F>
    for CaseAtDeliveryProof<'a, P, E, F, K, A, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    C: IReplayCaseConsumer,
{
    type Output = ReplayCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(ReplayCaseSuccessReturn {
            output: self.consumer.consume_replay_case(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.forms,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                self.random,
                self.independent,
                self.identity,
            ),
        })
    }
}

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, F, K, C> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    forms: &'a F,
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    delivery_proof: &'a CreateDeliveryProofParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: IReplayCaseConsumer,
{
    type Output = ReplayCaseReturn<C::Output>;

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
                    forms: self.forms,
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    random: self.random,
                    independent: self.independent,
                    identity: self.identity,
                    consumer: self.consumer,
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
            Err(error) => Err(ReplayCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F, C> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    forms: &'a F,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a ReplayCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    C: IReplayCaseConsumer,
{
    type Output = ReplayCaseReturn<C::Output>;

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
                    root: self.payload.root,
                },
                identity: KemIdentity::Asset {
                    root: self.payload.root,
                },
            },
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
                    forms: self.forms,
                    kem: &payload.adapter,
                    random: self.random,
                    independent: self.independent,
                    delivery_proof: self.delivery_proof,
                    identity,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ReplayCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a ReplayCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: IReplayCaseConsumer,
{
    type Output = ReplayCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hash_to_scalar,
                CaseAtKem {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    forms: &payload.forms,
                    random: self.random,
                    independent: self.independent,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ReplayCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a ReplayCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IReplayCaseConsumer,
{
    type Output = ReplayCaseReturn<C::Output>;

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
                    random: self.random,
                    independent: self.independent,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
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
            Err(error) => Err(ReplayCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    random_source: &'a CreateRandomSourceParams,
    payload: &'a ReplayCasePayload<'a>,
    consumer: &'a C,
}

impl<C: IReplayCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = ReplayCaseReturn<C::Output>;

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
                return Err(ReplayCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let random = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(ReplayCaseErrorReturn::CreateRandomSource(error));
            }
        };
        let independent = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(ReplayCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hash_to_scalar: &*hashing.adapter,
                random: &*random.adapter,
                independent: &*independent.adapter,
                chain_forms: self.chain_forms,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
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
            Err(error) => Err(ReplayCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_replay_case<C: IReplayCaseConsumer>(
    deps: &ReplayCaseDeps<C>,
    params: ReplayCaseParams,
    payload: ReplayCasePayload<'_>,
) -> ReplayCaseReturn<C::Output> {
    let _: RunReplayCaseFn<C> = run_replay_case;
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
                payload: &payload,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(ReplayCaseErrorReturn::CreatePairing(error)),
    }
}
