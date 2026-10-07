use core::convert::Infallible;
use encoding::CanonicalFieldKind;
use pairing::{
    G1OutsideSubgroupEncodingErrorReturn, G2OutsideSubgroupEncodingErrorReturn, IPairingReference,
};

use super::super::render::provides::{
    SolidityMemberNameTryNewErrorReturn, SolidityRecordMembers,
    SolidityRecordMembersTryNewErrorReturn,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeTarget {
    G1,
    G2,
    Scalar,
}

pub struct DecodeTargetConstructorParams {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeTargetTryNewErrorReturn {
    Unknown { text: String },
}

pub type DecodeTargetTryNewReturn = Result<DecodeTarget, DecodeTargetTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectionVector {
    pub target: DecodeTarget,
    pub input: Vec<u8>,
}

pub const REJECTION_VECTOR_FIELD_COUNT: usize = 2;

pub const REJECTION_VECTOR_MEMBER_NAMES: [&str; REJECTION_VECTOR_FIELD_COUNT] = ["target", "input"];

pub const REJECTION_VECTOR_FIELD_KINDS: [CanonicalFieldKind; REJECTION_VECTOR_FIELD_COUNT] =
    [CanonicalFieldKind::Text, CanonicalFieldKind::Bytes];

pub struct RejectionVectorDescription;

pub struct RejectionVectorDescriptionConstructorParams;

pub type RejectionVectorDescriptionTryNewReturn = Result<RejectionVectorDescription, Infallible>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RejectionVectorFromFieldsErrorReturn {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
    Target(DecodeTargetTryNewErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RejectionVectorMembersErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
    RecordMembers(SolidityRecordMembersTryNewErrorReturn),
}

pub type RejectionVectorMembersReturn =
    Result<SolidityRecordMembers, RejectionVectorMembersErrorReturn>;

pub struct RejectionVectorsDeps<'a, P: IPairingReference> {
    pub pairing: &'a P,
}

pub struct RejectionVectorsParams;

pub struct RejectionVectorsPayload;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectionVectorsSuccessReturn {
    pub records: Vec<RejectionVector>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RejectionVectorsErrorReturn {
    G1OutsideSubgroupEncoding(G1OutsideSubgroupEncodingErrorReturn),
    G2OutsideSubgroupEncoding(G2OutsideSubgroupEncodingErrorReturn),
}

pub type RejectionVectorsReturn =
    Result<RejectionVectorsSuccessReturn, RejectionVectorsErrorReturn>;

pub type RejectionVectorsFn<'a, P> = fn(
    &RejectionVectorsDeps<'a, P>,
    RejectionVectorsParams,
    RejectionVectorsPayload,
) -> RejectionVectorsReturn;
