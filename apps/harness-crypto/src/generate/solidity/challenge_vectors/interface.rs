use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use core::convert::Infallible;
use domain::{ParameterSetIdentifier, SuiteIdentifier};
use encoding::{
    CanonicalFieldKind, CanonicalFieldReaderErrorReturn, CanonicalFieldValue, EncodeErrorReturn,
    IEncoderAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
use hash_to_scalar::{DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter};
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
use pairing::{IPairingAdapter, SampleUniformScalarErrorReturn};
use proof::{DeliveryContext, DeliveryProofDeclaration, MintPurpose, TransferPurpose};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::group_vectors::provides::VectorCount;
use super::super::render::provides::{SolidityMemberName, SolidityMemberNameTryNewErrorReturn};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalDeliveryContext {
    pub suite_identifier: SuiteIdentifier,
    pub chain: CanonicalFieldValue,
    pub entitlement_contract: CanonicalFieldValue,
    pub asset_root: Root,
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
    Fields(CanonicalFieldReaderErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChallengeVectorMemberNamesErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
}

pub type ChallengeVectorMemberNamesReturn =
    Result<Vec<SolidityMemberName>, ChallengeVectorMemberNamesErrorReturn>;

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

pub struct ChallengeVectorsPayload<'a, F: IChainForms> {
    pub delivery_proof: &'a DeliveryProofDeclaration,
    pub mint: &'a DeliveryContext<F, MintPurpose>,
    pub transfer: &'a DeliveryContext<F, TransferPurpose>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChallengeVectorsSuccessReturn {
    pub records: Vec<ChallengeVector>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeVectorsErrorReturn {
    UnsupportedStatementVersion { version: u16 },
    DomainTag(DomainTagTryNewErrorReturn),
    FillBytes(FillBytesErrorReturn),
    SampleScalar(SampleUniformScalarErrorReturn),
    Encoding(EncodeErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
    ScalarLength { actual: usize },
}

pub type ChallengeVectorsReturn =
    Result<ChallengeVectorsSuccessReturn, ChallengeVectorsErrorReturn>;

pub type ChallengeVectorsFn<'a, P, E, F> = fn(
    &ChallengeVectorsDeps<'a, P, E>,
    ChallengeVectorsParams,
    ChallengeVectorsPayload<'_, F>,
) -> ChallengeVectorsReturn;

#[cfg(any(test, feature = "mocks"))]
pub trait IChallengeVectorsCaseConsumer {
    type Output;

    fn consume_challenge_vectors_case<P, E, F>(
        &self,
        pairing: &P,
        hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &E,
        random: &dyn IRandomSourceAdapter,
        forms: &F,
        payload: &ChallengeVectorsCasePayload,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeVectorsCaseDeps<C: IChallengeVectorsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeVectorsCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeVectorsCasePayload {
    pub delivery_proof: DeliveryProofDeclaration,
    pub mint: CanonicalDeliveryContext,
    pub mint_purpose: MintPurpose,
    pub transfer: CanonicalDeliveryContext,
    pub transfer_purpose: TransferPurpose,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeVectorsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeVectorsCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type ChallengeVectorsCaseReturn<O> =
    Result<ChallengeVectorsCaseSuccessReturn<O>, ChallengeVectorsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunChallengeVectorsCaseFn<C> =
    fn(
        &ChallengeVectorsCaseDeps<C>,
        ChallengeVectorsCaseParams,
        ChallengeVectorsCasePayload,
    ) -> ChallengeVectorsCaseReturn<<C as IChallengeVectorsCaseConsumer>::Output>;
