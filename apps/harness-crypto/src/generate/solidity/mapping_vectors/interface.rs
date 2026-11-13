use core::convert::Infallible;
use encoding::{CanonicalFieldKind, CanonicalFieldReaderErrorReturn};
use envelope::KeyAgreementDeclaration;
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
use hash_to_scalar::{DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use kem::{DeriveIdentityErrorReturn, ICredentialKemAdapter, SetupErrorReturn};
use pairing::IPairingArithmetic;
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingReference};
use proof::DeliveryProofDeclaration;
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::super::factory::provides::VectorCount;
use super::super::render::provides::{SolidityMemberName, SolidityMemberNameTryNewErrorReturn};
use crate::delivery::provides::AdmittedKemIdentity;

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

pub struct IdentityMappingVectorDescription;

pub struct IdentityMappingVectorDescriptionConstructorParams;

pub type IdentityMappingVectorDescriptionTryNewReturn =
    Result<IdentityMappingVectorDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashToScalarVectorFromFieldsErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityMappingVectorFromFieldsErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MappingVectorMemberNamesErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
}

pub type MappingVectorMemberNamesReturn =
    Result<Vec<SolidityMemberName>, MappingVectorMemberNamesErrorReturn>;

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
}

pub struct MappingVectorsPayload<'a> {
    pub key_agreement: &'a KeyAgreementDeclaration,
    pub delivery_proof: &'a DeliveryProofDeclaration,
    pub identities: &'a [AdmittedKemIdentity<'a>],
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

#[cfg(any(test, feature = "mocks"))]
pub trait IMappingVectorsCaseConsumer {
    type Output;

    fn consume_mapping_vectors_case<P, K>(
        &self,
        pairing: &P,
        hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
        kem: &K,
        random: &dyn IRandomSourceAdapter,
        identities: Vec<AdmittedKemIdentity<'_>>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        K: ICredentialKemAdapter<Pairing = P>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct MappingVectorsCaseDeps<C: IMappingVectorsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MappingVectorsCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub kem_hash_to_scalar: CreateHashToScalarParams,
    pub kem: CreateKemParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MappingVectorsCasePayload<'a> {
    pub canonicals: &'a [&'a [u8]],
    pub roots: &'a [&'a Root],
}

#[cfg(any(test, feature = "mocks"))]
pub struct MappingVectorsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum MappingVectorsCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type MappingVectorsCaseReturn<O> =
    Result<MappingVectorsCaseSuccessReturn<O>, MappingVectorsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunMappingVectorsCaseFn<C> =
    fn(
        &MappingVectorsCaseDeps<C>,
        MappingVectorsCaseParams,
        MappingVectorsCasePayload<'_>,
    ) -> MappingVectorsCaseReturn<<C as IMappingVectorsCaseConsumer>::Output>;
