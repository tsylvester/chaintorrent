#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use envelope::KeyAgreementDeclaration;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::IPairingArithmetic;
use proof::DeliveryProofDeclaration;
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};

use super::super::group_vectors::provides::{VectorCount, build_vector_count};
use super::interface::{
    HashToScalarVector, HashToScalarVectorDescription,
    HashToScalarVectorDescriptionConstructorParams, IdentityMappingVector,
    IdentityMappingVectorDescription, IdentityMappingVectorDescriptionConstructorParams,
    MappingVectorsDeps, MappingVectorsFn, MappingVectorsParams, MappingVectorsPayload,
    MappingVectorsReturn, MappingVectorsSuccessReturn,
};

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
    pub scope: Option<IdentityScope>,
}

pub fn build_mapping_vectors_params(
    overrides: MappingVectorsParamsOverrides,
) -> MappingVectorsParams {
    MappingVectorsParams {
        hash_to_scalar_count: overrides
            .hash_to_scalar_count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        message_length: overrides.message_length.unwrap_or(32),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
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
    identities: &'a [Vec<u8>],
    _overrides: MappingVectorsPayloadOverrides,
) -> MappingVectorsPayload<'a> {
    MappingVectorsPayload {
        key_agreement,
        delivery_proof,
        identities,
    }
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

fn hash_to_scalar_vector_fields(vector: &HashToScalarVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("tag".to_string(), byte_array(&vector.tag));
    fields.insert("message".to_string(), byte_array(&vector.message));
    fields.insert("scalar".to_string(), byte_array(&vector.scalar));
    fields
}

fn identity_mapping_vector_fields(vector: &IdentityMappingVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("identity".to_string(), byte_array(&vector.identity));
    fields.insert("scalar".to_string(), byte_array(&vector.scalar));
    fields
}

fn mapping_vectors_params_fields(params: &MappingVectorsParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "hash_to_scalar_count".to_string(),
        Value::from(params.hash_to_scalar_count.get()),
    );
    fields.insert(
        "message_length".to_string(),
        Value::from(params.message_length),
    );
    fields.insert(
        "scope".to_string(),
        Value::from(match params.scope {
            IdentityScope::Entitlement => "Entitlement",
            IdentityScope::Asset => "Asset",
        }),
    );
    fields
}

fn mapping_vectors_success_return_fields(
    value: &MappingVectorsSuccessReturn,
) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "hash_to_scalar".to_string(),
        Value::Array(
            value
                .hash_to_scalar
                .iter()
                .map(|vector| Value::Object(hash_to_scalar_vector_fields(vector)))
                .collect(),
        ),
    );
    fields.insert(
        "identity_mapping".to_string(),
        Value::Array(
            value
                .identity_mapping
                .iter()
                .map(|vector| Value::Object(identity_mapping_vector_fields(vector)))
                .collect(),
        ),
    );
    fields
}

#[derive(Default)]
pub struct HashToScalarVectorCorruptions {
    pub tag: Option<Value>,
    pub message: Option<Value>,
    pub scalar: Option<Value>,
}

pub fn invalidate_hash_to_scalar_vector(corruptions: HashToScalarVectorCorruptions) -> Value {
    let mut fields = hash_to_scalar_vector_fields(&build_hash_to_scalar_vector(Default::default()));
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
    let mut fields =
        identity_mapping_vector_fields(&build_identity_mapping_vector(Default::default()));
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
    pub scope: Option<Value>,
}

pub fn invalidate_mapping_vectors_params(corruptions: MappingVectorsParamsCorruptions) -> Value {
    let mut fields =
        mapping_vectors_params_fields(&build_mapping_vectors_params(Default::default()));
    if let Some(hash_to_scalar_count) = corruptions.hash_to_scalar_count {
        fields.insert("hash_to_scalar_count".to_string(), hash_to_scalar_count);
    }
    if let Some(message_length) = corruptions.message_length {
        fields.insert("message_length".to_string(), message_length);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
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
    let mut fields = mapping_vectors_success_return_fields(&build_mapping_vectors_success_return(
        Default::default(),
    ));
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
    let _: MappingVectorsFn<'a, P, K> = mock_mapping_vectors;
    Ok(build_mapping_vectors_success_return(Default::default()))
}
