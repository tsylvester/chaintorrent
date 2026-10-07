use core::convert::Infallible;
use domain::AssetIdentityHashTryNewErrorReturn;
use encoding::CanonicalFieldKind;
use envelope::KeyAgreementDeclaration;
use hash_to_scalar::{DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter};
use kem::{DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope, SetupErrorReturn};
use pairing::IPairingArithmetic;
use proof::DeliveryProofDeclaration;
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::group_vectors::provides::VectorCount;
use super::super::render::provides::{
    SolidityMemberNameTryNewErrorReturn, SolidityRecordMembers,
    SolidityRecordMembersTryNewErrorReturn,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashToScalarVector {
    pub tag: Vec<u8>,
    pub message: Vec<u8>,
    pub scalar: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityMappingVector {
    pub identity: Vec<u8>,
    pub scalar: [u8; 32],
}

pub const HASH_TO_SCALAR_VECTOR_FIELD_COUNT: usize = 3;

pub const HASH_TO_SCALAR_VECTOR_MEMBER_NAMES: [&str; HASH_TO_SCALAR_VECTOR_FIELD_COUNT] =
    ["tag", "message", "scalar"];

pub const HASH_TO_SCALAR_VECTOR_FIELD_KINDS: [CanonicalFieldKind;
    HASH_TO_SCALAR_VECTOR_FIELD_COUNT] = [
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Unsigned256,
];

pub const IDENTITY_MAPPING_VECTOR_FIELD_COUNT: usize = 2;

pub const IDENTITY_MAPPING_VECTOR_MEMBER_NAMES: [&str; IDENTITY_MAPPING_VECTOR_FIELD_COUNT] =
    ["identity", "scalar"];

pub const IDENTITY_MAPPING_VECTOR_FIELD_KINDS: [CanonicalFieldKind;
    IDENTITY_MAPPING_VECTOR_FIELD_COUNT] =
    [CanonicalFieldKind::Bytes, CanonicalFieldKind::Unsigned256];

pub struct HashToScalarVectorDescription;

pub struct HashToScalarVectorDescriptionConstructorParams;

pub type HashToScalarVectorDescriptionTryNewReturn =
    Result<HashToScalarVectorDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashToScalarVectorFromFieldsErrorReturn {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
}

pub struct IdentityMappingVectorDescription;

pub struct IdentityMappingVectorDescriptionConstructorParams;

pub type IdentityMappingVectorDescriptionTryNewReturn =
    Result<IdentityMappingVectorDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityMappingVectorFromFieldsErrorReturn {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MappingVectorMembersErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
    RecordMembers(SolidityRecordMembersTryNewErrorReturn),
}

pub type MappingVectorMembersReturn =
    Result<SolidityRecordMembers, MappingVectorMembersErrorReturn>;

pub struct MappingVectorsDeps<'a, P: IPairingArithmetic, K: ICredentialKemAdapter<Pairing = P>> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub kem: &'a K,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MappingVectorsParams {
    pub hash_to_scalar_count: VectorCount,
    pub message_length: usize,
    pub scope: IdentityScope,
}

pub struct MappingVectorsPayload<'a> {
    pub key_agreement: &'a KeyAgreementDeclaration,
    pub delivery_proof: &'a DeliveryProofDeclaration,
    pub identities: &'a [Vec<u8>],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MappingVectorsSuccessReturn {
    pub hash_to_scalar: Vec<HashToScalarVector>,
    pub identity_mapping: Vec<IdentityMappingVector>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MappingVectorsErrorReturn {
    DomainTag(DomainTagTryNewErrorReturn),
    FillBytes(FillBytesErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
    AssetIdentityLength { index: usize, actual: usize },
    AssetIdentityHash(AssetIdentityHashTryNewErrorReturn),
    Setup(SetupErrorReturn),
    DeriveIdentity(DeriveIdentityErrorReturn),
    ScalarLength { actual: usize },
}

pub type MappingVectorsReturn = Result<MappingVectorsSuccessReturn, MappingVectorsErrorReturn>;

pub type MappingVectorsFn<'a, P, K> = fn(
    &MappingVectorsDeps<'a, P, K>,
    MappingVectorsParams,
    MappingVectorsPayload<'_>,
) -> MappingVectorsReturn;
