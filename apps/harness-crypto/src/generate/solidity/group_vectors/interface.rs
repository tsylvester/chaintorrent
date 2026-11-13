use core::convert::Infallible;
use encoding::{CanonicalFieldKind, CanonicalFieldReaderErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingReference};
use pairing::{IPairingArithmetic, SampleUniformScalarErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::super::factory::provides::{MsmTermCount, VectorCount};
use super::super::render::provides::{SolidityMemberName, SolidityMemberNameTryNewErrorReturn};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirstGroupVectorCounts {
    pub g1_add: VectorCount,
    pub g1_mul: VectorCount,
    pub pairing_check: VectorCount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsmVectorSettings {
    pub count: VectorCount,
    pub terms: MsmTermCount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SecondGroupVectorCounts {
    pub g2_add: VectorCount,
    pub g1_msm: MsmVectorSettings,
    pub g2_msm: MsmVectorSettings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupOperationVector {
    pub draws: Vec<u8>,
    pub input: Vec<u8>,
    pub output: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairingCheckVector {
    pub draws: Vec<u8>,
    pub input: Vec<u8>,
    pub output: [u8; 32],
}

pub const VECTOR_RECORD_FIELD_COUNT: usize = 3;

pub const VECTOR_RECORD_MEMBER_NAMES: [&str; VECTOR_RECORD_FIELD_COUNT] =
    ["draws", "input", "output"];

pub const GROUP_OPERATION_VECTOR_FIELD_KINDS: [CanonicalFieldKind; VECTOR_RECORD_FIELD_COUNT] = [
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
];

pub const PAIRING_CHECK_VECTOR_FIELD_KINDS: [CanonicalFieldKind; VECTOR_RECORD_FIELD_COUNT] = [
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::FixedBytes32,
];

pub struct GroupOperationVectorDescription;

pub struct GroupOperationVectorDescriptionConstructorParams;

pub type GroupOperationVectorDescriptionTryNewReturn =
    Result<GroupOperationVectorDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupOperationVectorFromFieldsErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
}

pub struct PairingCheckVectorDescription;

pub struct PairingCheckVectorDescriptionConstructorParams;

pub type PairingCheckVectorDescriptionTryNewReturn =
    Result<PairingCheckVectorDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairingCheckVectorFromFieldsErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorRecordMemberNamesErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
}

pub type VectorRecordMemberNamesReturn =
    Result<Vec<SolidityMemberName>, VectorRecordMemberNamesErrorReturn>;

pub struct GroupVectorsDeps<'a, P: IPairingArithmetic> {
    pub pairing: &'a P,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupVectorsParams {
    pub first_group: FirstGroupVectorCounts,
    pub second_group: Option<SecondGroupVectorCounts>,
}

pub struct GroupVectorsPayload;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstGroupVectors {
    pub g1_add: Vec<GroupOperationVector>,
    pub g1_mul: Vec<GroupOperationVector>,
    pub pairing_check: Vec<PairingCheckVector>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecondGroupVectors {
    pub g2_add: Vec<GroupOperationVector>,
    pub g1_msm: Vec<GroupOperationVector>,
    pub g2_msm: Vec<GroupOperationVector>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupVectorsSuccessReturn {
    pub first_group: FirstGroupVectors,
    pub second_group: Option<SecondGroupVectors>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GroupVectorsErrorReturn {
    SecondGroupSettingsForFirstGroupOnly,
    SecondGroupSettingsMissing,
    FillBytes(FillBytesErrorReturn),
    SampleScalar(SampleUniformScalarErrorReturn),
}

pub type GroupVectorsReturn = Result<GroupVectorsSuccessReturn, GroupVectorsErrorReturn>;

pub type GroupVectorsFn<'a, P> =
    fn(&GroupVectorsDeps<'a, P>, GroupVectorsParams, GroupVectorsPayload) -> GroupVectorsReturn;

#[cfg(any(test, feature = "mocks"))]
pub trait IGroupVectorsCaseConsumer {
    type Output;

    fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
        &self,
        pairing: &P,
        random: &dyn IRandomSourceAdapter,
        payload: &GroupVectorsCasePayload,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct GroupVectorsCaseDeps<C: IGroupVectorsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct GroupVectorsCaseParams {
    pub pairing: CreatePairingParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct GroupVectorsCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct GroupVectorsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum GroupVectorsCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type GroupVectorsCaseReturn<O> =
    Result<GroupVectorsCaseSuccessReturn<O>, GroupVectorsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunGroupVectorsCaseFn<C> =
    fn(
        &GroupVectorsCaseDeps<C>,
        GroupVectorsCaseParams,
        GroupVectorsCasePayload,
    ) -> GroupVectorsCaseReturn<<C as IGroupVectorsCaseConsumer>::Output>;
