use core::convert::Infallible;
use domain::{AssetIdentityHash, ParameterSetIdentifier, SuiteIdentifier};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, EncodeErrorReturn, FromFieldErrorReturn,
    IEncoderAdapter,
};
use hash_to_scalar::{DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter};
use pairing::{IPairingAdapter, SampleUniformScalarErrorReturn};
use proof::{DeliveryProofDeclaration, DeliveryRelation, MintPurpose, TransferPurpose};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::group_vectors::provides::VectorCount;
use super::super::render::provides::{
    SolidityMemberNameTryNewErrorReturn, SolidityRecordMembers,
    SolidityRecordMembersTryNewErrorReturn,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalDeliveryContext {
    pub suite_identifier: SuiteIdentifier,
    pub chain: CanonicalFieldValue,
    pub entitlement_contract: CanonicalFieldValue,
    pub asset_identity_hash: AssetIdentityHash,
    pub parameter_set_identifier: ParameterSetIdentifier,
    pub source_entitlement: CanonicalFieldValue,
    pub target_entitlement: CanonicalFieldValue,
    pub old_interval: CanonicalFieldValue,
    pub new_interval: CanonicalFieldValue,
    pub seller: CanonicalFieldValue,
    pub buyer: CanonicalFieldValue,
    pub expiry: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryContextField {
    EntitlementContract,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    NewInterval,
    Seller,
    Buyer,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChallengeVector {
    pub purpose: u16,
    pub draws: Vec<u8>,
    pub encoding: Vec<u8>,
    pub challenge: [u8; 32],
}

pub const CHALLENGE_VECTOR_FIELD_COUNT: usize = 4;

pub const CHALLENGE_VECTOR_MEMBER_NAMES: [&str; CHALLENGE_VECTOR_FIELD_COUNT] =
    ["purpose", "draws", "encoding", "challenge"];

pub const CHALLENGE_VECTOR_FIELD_KINDS: [CanonicalFieldKind; CHALLENGE_VECTOR_FIELD_COUNT] = [
    CanonicalFieldKind::Unsigned16,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Unsigned256,
];

pub struct ChallengeVectorDescription;

pub struct ChallengeVectorDescriptionConstructorParams;

pub type ChallengeVectorDescriptionTryNewReturn = Result<ChallengeVectorDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChallengeVectorFromFieldsErrorReturn {
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
pub enum ChallengeVectorMembersErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
    RecordMembers(SolidityRecordMembersTryNewErrorReturn),
}

pub type ChallengeVectorMembersReturn =
    Result<SolidityRecordMembers, ChallengeVectorMembersErrorReturn>;

pub struct ChallengeVectorsDeps<'a, P: IPairingAdapter, E: IEncoderAdapter> {
    pub pairing: &'a P,
    pub encoder: &'a E,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChallengeVectorsParams {
    pub count: VectorCount,
    pub statement_version: u16,
}

pub struct ChallengeVectorsPayload<'a> {
    pub delivery_proof: &'a DeliveryProofDeclaration,
    pub mint: &'a CanonicalDeliveryContext,
    pub mint_purpose: MintPurpose,
    pub transfer: &'a CanonicalDeliveryContext,
    pub transfer_purpose: TransferPurpose,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChallengeVectorsSuccessReturn {
    pub records: Vec<ChallengeVector>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeVectorsErrorReturn {
    UnsupportedStatementVersion {
        version: u16,
    },
    DomainTag(DomainTagTryNewErrorReturn),
    ChainIdentifier {
        relation: DeliveryRelation,
        error: FromFieldErrorReturn,
    },
    Identity {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    Entitlement {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    Interval {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    FillBytes(FillBytesErrorReturn),
    SampleScalar(SampleUniformScalarErrorReturn),
    Encoding(EncodeErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
    ScalarLength {
        actual: usize,
    },
}

pub type ChallengeVectorsReturn =
    Result<ChallengeVectorsSuccessReturn, ChallengeVectorsErrorReturn>;

pub type ChallengeVectorsFn<'a, P, E> = fn(
    &ChallengeVectorsDeps<'a, P, E>,
    ChallengeVectorsParams,
    ChallengeVectorsPayload<'_>,
) -> ChallengeVectorsReturn;
