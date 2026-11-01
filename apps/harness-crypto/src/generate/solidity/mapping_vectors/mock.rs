#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use core::marker::PhantomData;
use envelope::KeyAgreementDeclaration;
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
use proof::DeliveryProofDeclaration;
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};

use super::super::group_vectors::provides::{VectorCount, build_vector_count};
use super::interface::{
    HashToScalarVector, HashToScalarVectorDescription,
    HashToScalarVectorDescriptionConstructorParams, IMappingVectorsCaseConsumer,
    IdentityMappingVector, IdentityMappingVectorDescription,
    IdentityMappingVectorDescriptionConstructorParams, MappingVectorsCaseDeps,
    MappingVectorsCaseErrorReturn, MappingVectorsCaseParams, MappingVectorsCasePayload,
    MappingVectorsCaseReturn, MappingVectorsCaseSuccessReturn, MappingVectorsDeps,
    MappingVectorsParams, MappingVectorsPayload, MappingVectorsReturn, MappingVectorsSuccessReturn,
    RunMappingVectorsCaseFn,
};
use crate::delivery::provides::AdmittedKemIdentity;

#[derive(Default)]
pub struct HashToScalarVectorOverrides {
    pub tag: Option<Vec<u8>>,
    pub message: Option<Vec<u8>>,
    pub scalar: Option<[u8; 32]>,
}

pub fn build_hash_to_scalar_vector(overrides: HashToScalarVectorOverrides) -> HashToScalarVector {
    HashToScalarVector {
        tag: overrides.tag.unwrap_or_else(|| b"test-tag".to_vec()),
        message: overrides.message.unwrap_or_else(|| vec![0x01; 32]),
        scalar: overrides.scalar.unwrap_or([0x02; 32]),
    }
}

#[derive(Default)]
pub struct IdentityMappingVectorOverrides {
    pub identity: Option<Vec<u8>>,
    pub scalar: Option<[u8; 32]>,
}

pub fn build_identity_mapping_vector(
    overrides: IdentityMappingVectorOverrides,
) -> IdentityMappingVector {
    IdentityMappingVector {
        identity: overrides.identity.unwrap_or_else(|| vec![0x03; 32]),
        scalar: overrides.scalar.unwrap_or([0x04; 32]),
    }
}

#[derive(Default)]
pub struct MappingVectorsParamsOverrides {
    pub hash_to_scalar_count: Option<VectorCount>,
    pub message_length: Option<usize>,
}

pub fn build_mapping_vectors_params(
    overrides: MappingVectorsParamsOverrides,
) -> MappingVectorsParams {
    MappingVectorsParams {
        hash_to_scalar_count: overrides
            .hash_to_scalar_count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        message_length: overrides.message_length.unwrap_or(32),
    }
}

#[derive(Default)]
pub struct HashToScalarVectorDescriptionConstructorParamsOverrides;

pub fn build_hash_to_scalar_vector_description_constructor_params(
    _overrides: HashToScalarVectorDescriptionConstructorParamsOverrides,
) -> HashToScalarVectorDescriptionConstructorParams {
    HashToScalarVectorDescriptionConstructorParams
}

pub fn build_hash_to_scalar_vector_description(
    overrides: HashToScalarVectorDescriptionConstructorParamsOverrides,
) -> HashToScalarVectorDescription {
    let Ok(description) = HashToScalarVectorDescription::try_new(
        build_hash_to_scalar_vector_description_constructor_params(overrides),
    );
    description
}

#[derive(Default)]
pub struct IdentityMappingVectorDescriptionConstructorParamsOverrides;

pub fn build_identity_mapping_vector_description_constructor_params(
    _overrides: IdentityMappingVectorDescriptionConstructorParamsOverrides,
) -> IdentityMappingVectorDescriptionConstructorParams {
    IdentityMappingVectorDescriptionConstructorParams
}

pub fn build_identity_mapping_vector_description(
    overrides: IdentityMappingVectorDescriptionConstructorParamsOverrides,
) -> IdentityMappingVectorDescription {
    let Ok(description) = IdentityMappingVectorDescription::try_new(
        build_identity_mapping_vector_description_constructor_params(overrides),
    );
    description
}

#[derive(Default)]
pub struct MappingVectorsSuccessReturnOverrides {
    pub hash_to_scalar: Option<Vec<HashToScalarVector>>,
    pub identity_mapping: Option<Vec<IdentityMappingVector>>,
}

pub fn build_mapping_vectors_success_return(
    overrides: MappingVectorsSuccessReturnOverrides,
) -> MappingVectorsSuccessReturn {
    MappingVectorsSuccessReturn {
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| vec![build_hash_to_scalar_vector(Default::default())]),
        identity_mapping: overrides
            .identity_mapping
            .unwrap_or_else(|| vec![build_identity_mapping_vector(Default::default())]),
    }
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn hash_to_scalar_vector_value(vector: &HashToScalarVector) -> Value {
    let mut fields = Map::new();
    fields.insert("tag".to_string(), byte_array(&vector.tag));
    fields.insert("message".to_string(), byte_array(&vector.message));
    fields.insert("scalar".to_string(), byte_array(&vector.scalar));
    Value::Object(fields)
}

fn identity_mapping_vector_value(vector: &IdentityMappingVector) -> Value {
    let mut fields = Map::new();
    fields.insert("identity".to_string(), byte_array(&vector.identity));
    fields.insert("scalar".to_string(), byte_array(&vector.scalar));
    Value::Object(fields)
}

#[derive(Default)]
pub struct HashToScalarVectorCorruptions {
    pub tag: Option<Value>,
    pub message: Option<Value>,
    pub scalar: Option<Value>,
}

pub fn invalidate_hash_to_scalar_vector(corruptions: HashToScalarVectorCorruptions) -> Value {
    let vector = build_hash_to_scalar_vector(Default::default());
    let mut fields = Map::new();
    fields.insert("tag".to_string(), byte_array(&vector.tag));
    fields.insert("message".to_string(), byte_array(&vector.message));
    fields.insert("scalar".to_string(), byte_array(&vector.scalar));
    if let Some(tag) = corruptions.tag {
        fields.insert("tag".to_string(), tag);
    }
    if let Some(message) = corruptions.message {
        fields.insert("message".to_string(), message);
    }
    if let Some(scalar) = corruptions.scalar {
        fields.insert("scalar".to_string(), scalar);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct IdentityMappingVectorCorruptions {
    pub identity: Option<Value>,
    pub scalar: Option<Value>,
}

pub fn invalidate_identity_mapping_vector(corruptions: IdentityMappingVectorCorruptions) -> Value {
    let vector = build_identity_mapping_vector(Default::default());
    let mut fields = Map::new();
    fields.insert("identity".to_string(), byte_array(&vector.identity));
    fields.insert("scalar".to_string(), byte_array(&vector.scalar));
    if let Some(identity) = corruptions.identity {
        fields.insert("identity".to_string(), identity);
    }
    if let Some(scalar) = corruptions.scalar {
        fields.insert("scalar".to_string(), scalar);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MappingVectorsParamsCorruptions {
    pub hash_to_scalar_count: Option<Value>,
    pub message_length: Option<Value>,
}

pub fn invalidate_mapping_vectors_params(corruptions: MappingVectorsParamsCorruptions) -> Value {
    let params = build_mapping_vectors_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "hash_to_scalar_count".to_string(),
        Value::from(params.hash_to_scalar_count.get()),
    );
    fields.insert(
        "message_length".to_string(),
        Value::from(params.message_length),
    );
    if let Some(hash_to_scalar_count) = corruptions.hash_to_scalar_count {
        fields.insert("hash_to_scalar_count".to_string(), hash_to_scalar_count);
    }
    if let Some(message_length) = corruptions.message_length {
        fields.insert("message_length".to_string(), message_length);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MappingVectorsSuccessReturnCorruptions {
    pub hash_to_scalar: Option<Value>,
    pub identity_mapping: Option<Value>,
}

pub fn invalidate_mapping_vectors_success_return(
    corruptions: MappingVectorsSuccessReturnCorruptions,
) -> Value {
    let success = build_mapping_vectors_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "hash_to_scalar".to_string(),
        Value::Array(
            success
                .hash_to_scalar
                .iter()
                .map(hash_to_scalar_vector_value)
                .collect(),
        ),
    );
    fields.insert(
        "identity_mapping".to_string(),
        Value::Array(
            success
                .identity_mapping
                .iter()
                .map(identity_mapping_vector_value)
                .collect(),
        ),
    );
    if let Some(hash_to_scalar) = corruptions.hash_to_scalar {
        fields.insert("hash_to_scalar".to_string(), hash_to_scalar);
    }
    if let Some(identity_mapping) = corruptions.identity_mapping {
        fields.insert("identity_mapping".to_string(), identity_mapping);
    }
    Value::Object(fields)
}

pub fn mock_mapping_vectors<'a, P: IPairingArithmetic, K: ICredentialKemAdapter<Pairing = P>>(
    _deps: &MappingVectorsDeps<'a, P, K>,
    _params: MappingVectorsParams,
    _payload: MappingVectorsPayload<'_>,
) -> MappingVectorsReturn {
    Ok(build_mapping_vectors_success_return(Default::default()))
}

#[derive(Default)]
pub struct MappingVectorsDepsOverrides;

pub fn build_mapping_vectors_deps<
    'a,
    P: IPairingArithmetic,
    K: ICredentialKemAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: MappingVectorsDepsOverrides,
) -> MappingVectorsDeps<'a, P, K> {
    MappingVectorsDeps {
        pairing,
        hash_to_scalar,
        kem,
        random,
    }
}

#[derive(Default)]
pub struct MappingVectorsPayloadOverrides;

pub fn build_mapping_vectors_payload<'a>(
    key_agreement: &'a KeyAgreementDeclaration,
    delivery_proof: &'a DeliveryProofDeclaration,
    identities: &'a [AdmittedKemIdentity<'a>],
    _overrides: MappingVectorsPayloadOverrides,
) -> MappingVectorsPayload<'a> {
    MappingVectorsPayload {
        key_agreement,
        delivery_proof,
        identities,
    }
}

#[derive(Default)]
pub struct MappingVectorsCaseDepsOverrides;

pub fn build_mapping_vectors_case_deps<C: IMappingVectorsCaseConsumer>(
    consumer: C,
    _overrides: MappingVectorsCaseDepsOverrides,
) -> MappingVectorsCaseDeps<C> {
    MappingVectorsCaseDeps { consumer }
}

#[derive(Default)]
pub struct MappingVectorsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub kem_hash_to_scalar: Option<CreateHashToScalarParams>,
    pub kem: Option<CreateKemParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_mapping_vectors_case_params(
    overrides: MappingVectorsCaseParamsOverrides,
) -> MappingVectorsCaseParams {
    MappingVectorsCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        kem_hash_to_scalar: overrides
            .kem_hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct MappingVectorsCasePayloadOverrides<'a> {
    marker: PhantomData<&'a ()>,
}

pub fn build_mapping_vectors_case_payload<'a>(
    canonicals: &'a [&'a [u8]],
    roots: &'a [&'a Root],
    _overrides: MappingVectorsCasePayloadOverrides<'a>,
) -> MappingVectorsCasePayload<'a> {
    MappingVectorsCasePayload { canonicals, roots }
}

#[derive(Default)]
pub struct MappingVectorsCaseSuccessReturnOverrides;

pub fn build_mapping_vectors_case_success_return<O>(
    output: O,
    _overrides: MappingVectorsCaseSuccessReturnOverrides,
) -> MappingVectorsCaseSuccessReturn<O> {
    MappingVectorsCaseSuccessReturn { output }
}

struct CaseAtKem<'a, P: IPairingArithmetic, C: IMappingVectorsCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    payload: &'a MappingVectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, C> IKemConsumer<P> for CaseAtKem<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IMappingVectorsCaseConsumer,
{
    type Output = MappingVectorsCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let identities = match payload.scope() {
            IdentityScope::Entitlement => self
                .payload
                .canonicals
                .iter()
                .copied()
                .map(|canonical| AdmittedKemIdentity {
                    setup_scope: SetupScope::Entitlement,
                    identity: KemIdentity::Entitlement { canonical },
                })
                .collect(),
            IdentityScope::Asset => self
                .payload
                .roots
                .iter()
                .copied()
                .map(|root| AdmittedKemIdentity {
                    setup_scope: SetupScope::Asset { root },
                    identity: KemIdentity::Asset { root },
                })
                .collect(),
        };
        Ok(MappingVectorsCaseSuccessReturn {
            output: self.consumer.consume_mapping_vectors_case(
                self.pairing,
                self.hashing,
                &payload.adapter,
                self.source,
                identities,
            ),
        })
    }
}

struct CaseAtPairing<'a, C: IMappingVectorsCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    kem_hash_to_scalar: &'a CreateHashToScalarParams,
    kem: &'a CreateKemParams,
    random_source: &'a CreateRandomSourceParams,
    payload: &'a MappingVectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<C: IMappingVectorsCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = MappingVectorsCaseReturn<C::Output>;

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
                return Err(MappingVectorsCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let kem_hashing = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            *self.kem_hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(kem_hashing) => kem_hashing,
            Err(error) => {
                return Err(MappingVectorsCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(MappingVectorsCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_kem(
            &build_create_kem_deps(
                &payload.adapter,
                &*kem_hashing.adapter,
                CaseAtKem {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    source: &*source.adapter,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MappingVectorsCaseErrorReturn::CreateKem(error)),
        }
    }
}

pub fn run_mapping_vectors_case<C: IMappingVectorsCaseConsumer>(
    deps: &MappingVectorsCaseDeps<C>,
    params: MappingVectorsCaseParams,
    payload: MappingVectorsCasePayload<'_>,
) -> MappingVectorsCaseReturn<C::Output> {
    let _: RunMappingVectorsCaseFn<C> = run_mapping_vectors_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                kem_hash_to_scalar: &params.kem_hash_to_scalar,
                kem: &params.kem,
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
        Err(error) => Err(MappingVectorsCaseErrorReturn::CreatePairing(error)),
    }
}
