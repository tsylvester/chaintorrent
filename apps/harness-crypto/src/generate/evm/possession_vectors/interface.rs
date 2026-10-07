use core::convert::Infallible;
use encoding::CanonicalFieldKind;
use envelope::{GenerateKeysErrorReturn, IKeyAgreementAdapter};
use pairing::IPairingArithmetic;
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::group_vectors::provides::VectorCount;
use super::super::render::provides::{
    SolidityMemberNameTryNewErrorReturn, SolidityRecordMembers,
    SolidityRecordMembersTryNewErrorReturn,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PossessionVector {
    pub x: [u8; 32],
    pub y: [u8; 32],
    pub pk1: Vec<u8>,
    pub pk2: Vec<u8>,
    pub r1: Vec<u8>,
    pub z1: [u8; 32],
    pub r2: Vec<u8>,
    pub z2: [u8; 32],
}

pub const POSSESSION_VECTOR_FIELD_COUNT: usize = 8;

pub const POSSESSION_VECTOR_MEMBER_NAMES: [&str; POSSESSION_VECTOR_FIELD_COUNT] =
    ["x", "y", "pk1", "pk2", "r1", "z1", "r2", "z2"];

pub const POSSESSION_VECTOR_FIELD_KINDS: [CanonicalFieldKind; POSSESSION_VECTOR_FIELD_COUNT] = [
    CanonicalFieldKind::Unsigned256,
    CanonicalFieldKind::Unsigned256,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Unsigned256,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Unsigned256,
];

pub struct PossessionVectorDescription;

pub struct PossessionVectorDescriptionConstructorParams;

pub type PossessionVectorDescriptionTryNewReturn = Result<PossessionVectorDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PossessionVectorFromFieldsErrorReturn {
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
pub enum PossessionVectorMembersErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
    RecordMembers(SolidityRecordMembersTryNewErrorReturn),
}

pub type PossessionVectorMembersReturn =
    Result<SolidityRecordMembers, PossessionVectorMembersErrorReturn>;

pub struct PossessionVectorsDeps<'a, P: IPairingArithmetic, K: IKeyAgreementAdapter<Pairing = P>> {
    pub pairing: &'a P,
    pub key_agreement: &'a K,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PossessionVectorsParams {
    pub count: VectorCount,
}

pub struct PossessionVectorsPayload;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PossessionVectorsSuccessReturn {
    pub records: Vec<PossessionVector>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PossessionVectorsErrorReturn {
    FillBytes(FillBytesErrorReturn),
    GenerateKeys(GenerateKeysErrorReturn),
    ScalarLength { actual: usize },
}

pub type PossessionVectorsReturn =
    Result<PossessionVectorsSuccessReturn, PossessionVectorsErrorReturn>;

pub type PossessionVectorsFn<'a, P, K> = fn(
    &PossessionVectorsDeps<'a, P, K>,
    PossessionVectorsParams,
    PossessionVectorsPayload,
) -> PossessionVectorsReturn;
