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
use kdf::{
    CreateKeyDerivationParams, IKeyDerivationAdapter, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload, create_key_derivation,
};
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
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParams,
    DeliveryContext, IDeliveryProofAdapter, IDeliveryProofConsumer, MintPurpose, TransferPurpose,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};
use workflows::DerivationContext;

use super::cross_holder::provides::{CrossHolderSuccessReturn, build_cross_holder_success_return};
use super::cross_set::provides::{CrossSetSuccessReturn, build_cross_set_success_return};
use super::interface::{
    IVectorsCaseConsumer, RunVectorsCaseFn, VectorsCaseDeps, VectorsCaseErrorReturn,
    VectorsCaseParams, VectorsCasePayload, VectorsCaseReturn, VectorsCaseSuccessReturn,
    VectorsDeps, VectorsFn, VectorsParams, VectorsPayload, VectorsReturn, VectorsSuccessReturn,
};
use super::malformed_capsules::provides::{
    MalformedCapsulesSuccessReturn, build_malformed_capsules_success_return,
};
use super::non_convertibility::provides::{
    NonConvertibilitySuccessReturn, build_non_convertibility_success_return,
};
use super::replay::provides::{ReplaySuccessReturn, build_replay_success_return};
use super::rerandomization::provides::{
    RerandomizationSuccessReturn, build_rerandomization_success_return,
};
use super::statement_mutations::provides::{
    StatementMutationsSuccessReturn, build_statement_mutations_success_return,
};
use crate::delivery::provides::{
    AdmittedKemIdentity, AdmittedKemIdentityOverrides, build_admitted_kem_identity,
};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct VectorsParamsOverrides {
    pub cross_holder: Option<VectorCount>,
    pub rerandomization: Option<VectorCount>,
    pub non_convertibility: Option<VectorCount>,
    pub malformed_capsules: Option<VectorCount>,
    pub cross_set: Option<VectorCount>,
    pub statement_mutations: Option<VectorCount>,
    pub replay: Option<VectorCount>,
}

pub fn build_vectors_params(overrides: VectorsParamsOverrides) -> VectorsParams {
    VectorsParams {
        cross_holder: overrides
            .cross_holder
            .unwrap_or_else(|| build_vector_count(Default::default())),
        rerandomization: overrides
            .rerandomization
            .unwrap_or_else(|| build_vector_count(Default::default())),
        non_convertibility: overrides
            .non_convertibility
            .unwrap_or_else(|| build_vector_count(Default::default())),
        malformed_capsules: overrides
            .malformed_capsules
            .unwrap_or_else(|| build_vector_count(Default::default())),
        cross_set: overrides
            .cross_set
            .unwrap_or_else(|| build_vector_count(Default::default())),
        statement_mutations: overrides
            .statement_mutations
            .unwrap_or_else(|| build_vector_count(Default::default())),
        replay: overrides
            .replay
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

fn vectors_params_fields(params: &VectorsParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "cross_holder".to_string(),
        Value::from(params.cross_holder.get()),
    );
    fields.insert(
        "rerandomization".to_string(),
        Value::from(params.rerandomization.get()),
    );
    fields.insert(
        "non_convertibility".to_string(),
        Value::from(params.non_convertibility.get()),
    );
    fields.insert(
        "malformed_capsules".to_string(),
        Value::from(params.malformed_capsules.get()),
    );
    fields.insert("cross_set".to_string(), Value::from(params.cross_set.get()));
    fields.insert(
        "statement_mutations".to_string(),
        Value::from(params.statement_mutations.get()),
    );
    fields.insert("replay".to_string(), Value::from(params.replay.get()));
    fields
}

#[derive(Default)]
pub struct VectorsParamsCorruptions {
    pub cross_holder: Option<Value>,
    pub rerandomization: Option<Value>,
    pub non_convertibility: Option<Value>,
    pub malformed_capsules: Option<Value>,
    pub cross_set: Option<Value>,
    pub statement_mutations: Option<Value>,
    pub replay: Option<Value>,
}

pub fn invalidate_vectors_params(corruptions: VectorsParamsCorruptions) -> Value {
    let mut fields = vectors_params_fields(&build_vectors_params(Default::default()));
    if let Some(cross_holder) = corruptions.cross_holder {
        fields.insert("cross_holder".to_string(), cross_holder);
    }
    if let Some(rerandomization) = corruptions.rerandomization {
        fields.insert("rerandomization".to_string(), rerandomization);
    }
    if let Some(non_convertibility) = corruptions.non_convertibility {
        fields.insert("non_convertibility".to_string(), non_convertibility);
    }
    if let Some(malformed_capsules) = corruptions.malformed_capsules {
        fields.insert("malformed_capsules".to_string(), malformed_capsules);
    }
    if let Some(cross_set) = corruptions.cross_set {
        fields.insert("cross_set".to_string(), cross_set);
    }
    if let Some(statement_mutations) = corruptions.statement_mutations {
        fields.insert("statement_mutations".to_string(), statement_mutations);
    }
    if let Some(replay) = corruptions.replay {
        fields.insert("replay".to_string(), replay);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct VectorsPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
    pub other_identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_vectors_payload<'a, F: IChainForms>(
    context: &'a DerivationContext,
    mint: &'a DeliveryContext<F, MintPurpose>,
    transfer: &'a DeliveryContext<F, TransferPurpose>,
    next: &'a DeliveryContext<F, TransferPurpose>,
    overrides: VectorsPayloadOverrides<'a>,
) -> VectorsPayload<'a, F> {
    VectorsPayload {
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
        other_identity: overrides.other_identity.unwrap_or_else(|| {
            build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                identity: Some(KemIdentity::Entitlement {
                    canonical: b"pkg@2.0.0#7",
                }),
                ..Default::default()
            })
        }),
        context,
        mint,
        transfer,
        next,
    }
}

#[derive(Default)]
pub struct VectorsDepsOverrides;

pub fn build_vectors_deps<
    'a,
    P: IPairingArithmetic,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    E: IEncoderAdapter,
>(
    pairing: &'a P,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    kdf: &'a dyn IKeyDerivationAdapter,
    encoder: &'a E,
    _overrides: VectorsDepsOverrides,
) -> VectorsDeps<'a, P, K, A, D, E> {
    VectorsDeps {
        pairing,
        kem,
        key_agreement,
        delivery_proof,
        random,
        kdf,
        encoder,
    }
}

pub struct VectorsSuccessReturnOverrides<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub cross_holder: Option<CrossHolderSuccessReturn<K>>,
    pub rerandomization: Option<RerandomizationSuccessReturn<K>>,
    pub non_convertibility: Option<NonConvertibilitySuccessReturn<K>>,
    pub malformed_capsules: Option<MalformedCapsulesSuccessReturn<K>>,
    pub cross_set: Option<CrossSetSuccessReturn<K>>,
    pub statement_mutations: Option<StatementMutationsSuccessReturn<P, D>>,
    pub replay: Option<ReplaySuccessReturn<P, D>>,
}

impl<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> Default for VectorsSuccessReturnOverrides<P, K, D>
{
    fn default() -> Self {
        Self {
            cross_holder: None,
            rerandomization: None,
            non_convertibility: None,
            malformed_capsules: None,
            cross_set: None,
            statement_mutations: None,
            replay: None,
        }
    }
}

pub fn build_vectors_success_return<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    overrides: VectorsSuccessReturnOverrides<P, K, D>,
) -> VectorsSuccessReturn<P, K, D> {
    VectorsSuccessReturn {
        cross_holder: overrides
            .cross_holder
            .unwrap_or_else(|| build_cross_holder_success_return(Default::default())),
        rerandomization: overrides
            .rerandomization
            .unwrap_or_else(|| build_rerandomization_success_return(Default::default())),
        non_convertibility: overrides
            .non_convertibility
            .unwrap_or_else(|| build_non_convertibility_success_return(Default::default())),
        malformed_capsules: overrides
            .malformed_capsules
            .unwrap_or_else(|| build_malformed_capsules_success_return(Default::default())),
        cross_set: overrides
            .cross_set
            .unwrap_or_else(|| build_cross_set_success_return(Default::default())),
        statement_mutations: overrides
            .statement_mutations
            .unwrap_or_else(|| build_statement_mutations_success_return(Default::default())),
        replay: overrides
            .replay
            .unwrap_or_else(|| build_replay_success_return(Default::default())),
    }
}

pub fn mock_vectors<
    'a,
    P: IPairingArithmetic,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    E: IEncoderAdapter,
>(
    _deps: &VectorsDeps<'a, P, K, A, D, E>,
    _params: VectorsParams,
    _payload: VectorsPayload<'_, D::Forms>,
) -> VectorsReturn<P, K, D> {
    let _: VectorsFn<'a, P, K, A, D, E> = mock_vectors;
    Ok(build_vectors_success_return(Default::default()))
}

#[derive(Default)]
pub struct VectorsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub key_derivation: Option<CreateKeyDerivationParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_vectors_case_params(overrides: VectorsCaseParamsOverrides) -> VectorsCaseParams {
    VectorsCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        key_derivation: overrides
            .key_derivation
            .unwrap_or_else(|| build_create_key_derivation_params(Default::default())),
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
pub struct VectorsCaseDepsOverrides;

pub fn build_vectors_case_deps<C: IVectorsCaseConsumer>(
    consumer: C,
    _overrides: VectorsCaseDepsOverrides,
) -> VectorsCaseDeps<C> {
    VectorsCaseDeps { consumer }
}

#[derive(Default)]
pub struct VectorsCasePayloadOverrides;

pub fn build_vectors_case_payload<'a>(
    canonical: &'a [u8],
    other_canonical: &'a [u8],
    root: &'a Root,
    other_root: &'a Root,
    _overrides: VectorsCasePayloadOverrides,
) -> VectorsCasePayload<'a> {
    VectorsCasePayload {
        canonical,
        other_canonical,
        root,
        other_root,
    }
}

#[derive(Default)]
pub struct VectorsCaseSuccessReturnOverrides;

pub fn build_vectors_case_success_return<O>(
    output: O,
    _overrides: VectorsCaseSuccessReturnOverrides,
) -> VectorsCaseSuccessReturn<O> {
    VectorsCaseSuccessReturn { output }
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
    kdf: &'a dyn IKeyDerivationAdapter,
    identity: AdmittedKemIdentity<'a>,
    other_identity: AdmittedKemIdentity<'a>,
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
    C: IVectorsCaseConsumer,
{
    type Output = VectorsCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(VectorsCaseSuccessReturn {
            output: self.consumer.consume_vectors_case(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.forms,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                self.random,
                self.independent,
                self.kdf,
                self.identity,
                self.other_identity,
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
    kdf: &'a dyn IKeyDerivationAdapter,
    delivery_proof: &'a CreateDeliveryProofParams,
    identity: AdmittedKemIdentity<'a>,
    other_identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: IVectorsCaseConsumer,
{
    type Output = VectorsCaseReturn<C::Output>;

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
                    kdf: self.kdf,
                    identity: self.identity,
                    other_identity: self.other_identity,
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
            Err(error) => Err(VectorsCaseErrorReturn::CreateDeliveryProof(error)),
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
    kdf: &'a dyn IKeyDerivationAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a VectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    C: IVectorsCaseConsumer,
{
    type Output = VectorsCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let (identity, other_identity) = match payload.scope() {
            IdentityScope::Entitlement => (
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Entitlement,
                    identity: KemIdentity::Entitlement {
                        canonical: self.payload.canonical,
                    },
                },
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Entitlement,
                    identity: KemIdentity::Entitlement {
                        canonical: self.payload.other_canonical,
                    },
                },
            ),
            IdentityScope::Asset => (
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Asset {
                        root: self.payload.root,
                    },
                    identity: KemIdentity::Asset {
                        root: self.payload.root,
                    },
                },
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Asset {
                        root: self.payload.other_root,
                    },
                    identity: KemIdentity::Asset {
                        root: self.payload.other_root,
                    },
                },
            ),
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
                    kdf: self.kdf,
                    delivery_proof: self.delivery_proof,
                    identity,
                    other_identity,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(VectorsCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    kdf: &'a dyn IKeyDerivationAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a VectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: IVectorsCaseConsumer,
{
    type Output = VectorsCaseReturn<C::Output>;

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
                    kdf: self.kdf,
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
            Err(error) => Err(VectorsCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    kdf: &'a dyn IKeyDerivationAdapter,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a VectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IVectorsCaseConsumer,
{
    type Output = VectorsCaseReturn<C::Output>;

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
                    kdf: self.kdf,
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
            Err(error) => Err(VectorsCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    encoding: &'a CreateEncodingParams,
    key_derivation: &'a CreateKeyDerivationParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    random_source: &'a CreateRandomSourceParams,
    payload: &'a VectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<C: IVectorsCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = VectorsCaseReturn<C::Output>;

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
                return Err(VectorsCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let kdf = match create_key_derivation(
            &build_create_key_derivation_deps(Default::default()),
            *self.key_derivation,
            build_create_key_derivation_payload(Default::default()),
        ) {
            Ok(kdf) => kdf,
            Err(error) => {
                return Err(VectorsCaseErrorReturn::CreateKeyDerivation(error));
            }
        };
        let random = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(VectorsCaseErrorReturn::CreateRandomSource(error));
            }
        };
        let independent = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(VectorsCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hash_to_scalar: &*hashing.adapter,
                kdf: &*kdf.adapter,
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
            Err(error) => Err(VectorsCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_vectors_case<C: IVectorsCaseConsumer>(
    deps: &VectorsCaseDeps<C>,
    params: VectorsCaseParams,
    payload: VectorsCasePayload<'_>,
) -> VectorsCaseReturn<C::Output> {
    let _: RunVectorsCaseFn<C> = run_vectors_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                encoding: &params.encoding,
                key_derivation: &params.key_derivation,
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
        Err(error) => Err(VectorsCaseErrorReturn::CreatePairing(error)),
    }
}
