use core::convert::Infallible;
use encoding::{CanonicalFieldKind, CanonicalFieldReaderErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter, IEncoderAdapter};
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
use envelope::{GenerateKeysErrorReturn, IKeyAgreementAdapter};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
use pairing::IPairingArithmetic;
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingReference};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::group_vectors::provides::VectorCount;
use super::super::render::provides::{SolidityMemberName, SolidityMemberNameTryNewErrorReturn};

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
    Fields(CanonicalFieldReaderErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PossessionVectorMemberNamesErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
}

pub type PossessionVectorMemberNamesReturn =
    Result<Vec<SolidityMemberName>, PossessionVectorMemberNamesErrorReturn>;

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

#[cfg(any(test, feature = "mocks"))]
pub trait IPossessionVectorsCaseConsumer {
    type Output;

    fn consume_possession_vectors_case<P, H, E, K>(
        &self,
        pairing: &P,
        hash_to_scalar: &H,
        encoder: &E,
        decoder: &E,
        key_agreement: &K,
        random: &dyn IRandomSourceAdapter,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        H: ?Sized + IHashToScalarAdapter<P::Scalar>,
        E: IEncoderAdapter + IDecoderAdapter,
        K: IKeyAgreementAdapter<Pairing = P>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct PossessionVectorsCaseDeps<C: IPossessionVectorsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct PossessionVectorsCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct PossessionVectorsCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct PossessionVectorsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum PossessionVectorsCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateKeyAgreement(CreateKeyAgreementErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type PossessionVectorsCaseReturn<O> =
    Result<PossessionVectorsCaseSuccessReturn<O>, PossessionVectorsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunPossessionVectorsCaseFn<C> =
    fn(
        &PossessionVectorsCaseDeps<C>,
        PossessionVectorsCaseParams,
        PossessionVectorsCasePayload,
    ) -> PossessionVectorsCaseReturn<<C as IPossessionVectorsCaseConsumer>::Output>;
