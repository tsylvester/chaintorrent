#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use kem::IdentityScope;

use super::super::group_vectors::provides::{VectorCount, build_vector_count};
use super::interface::{
    HashToScalarVector, HashToScalarVectorDescription,
    HashToScalarVectorDescriptionConstructorParams, IdentityMappingVector,
    IdentityMappingVectorDescription, IdentityMappingVectorDescriptionConstructorParams,
    MappingVectorsParams,
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

pub fn build_hash_to_scalar_vector_description() -> HashToScalarVectorDescription {
    let Ok(description) =
        HashToScalarVectorDescription::try_new(HashToScalarVectorDescriptionConstructorParams);
    description
}

pub fn build_identity_mapping_vector_description() -> IdentityMappingVectorDescription {
    let Ok(description) = IdentityMappingVectorDescription::try_new(
        IdentityMappingVectorDescriptionConstructorParams,
    );
    description
}
