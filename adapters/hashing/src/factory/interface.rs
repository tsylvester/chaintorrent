#[cfg(any(test, feature = "mocks"))]
use super::mock::MockICommitmentAdapterErrorReturn;
use crate::blake3_bao::provides::Blake3BaoCommitmentErrorReturn;
use core::convert::Infallible;
use domain::Secret;

pub const COMMITMENT_INTERFACE_VERSION: u32 = 1;

pub const CHAINING_VALUE_LENGTH: usize = 32;
pub const OUTBOARD_HEADER_LENGTH: usize = 8;
pub const OUTBOARD_PARENT_LENGTH: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitmentSchemeIdentifier {
    Blake3BaoV1,
    #[cfg(any(test, feature = "mocks"))]
    MockV1,
}

pub struct CommitmentDeclaration {
    pub identifier: CommitmentSchemeIdentifier,
    pub chunk_length: u64,
    pub key_length: usize,
    pub adapter_version: u32,
    pub interface_version: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Root {
    pub(super) bytes: [u8; 32],
}

pub struct RootConstructorParams {
    pub bytes: [u8; 32],
}

#[derive(Debug, PartialEq, Eq)]
pub enum RootTryNewErrorReturn {
    AllZero,
}

pub type RootTryNewReturn = Result<Root, RootTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outboard {
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticationPath {
    pub siblings: Vec<[u8; 32]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub bytes: Vec<u8>,
}

pub enum CommitmentKey<'a> {
    Unkeyed,
    Keyed(&'a Secret<Vec<u8>>),
}

pub struct CommitParams;
pub struct VerifyParams;
pub struct ExtractPathParams;
pub struct VerifyChunkParams;
pub struct ChallengeParams;
pub struct RespondParams;

pub struct CommitPayload<'a> {
    pub content: &'a [u8],
    pub key: CommitmentKey<'a>,
}

pub struct CommitSuccessReturn {
    pub root: Root,
    pub outboard: Outboard,
}

pub struct VerifyPayload<'a> {
    pub content: &'a [u8],
    pub outboard: &'a Outboard,
    pub root: &'a Root,
    pub key: CommitmentKey<'a>,
}

pub struct VerifySuccessReturn;

pub struct ExtractPathPayload<'a> {
    pub outboard: &'a Outboard,
    pub chunk_index: u64,
}

pub struct ExtractPathSuccessReturn {
    pub path: AuthenticationPath,
}

pub struct VerifyChunkPayload<'a> {
    pub root: &'a Root,
    pub content_length: u64,
    pub chunk_index: u64,
    pub chunk: &'a Chunk,
    pub path: &'a AuthenticationPath,
    pub key: CommitmentKey<'a>,
}

pub struct VerifyChunkSuccessReturn;

pub struct ChallengePayload {
    pub uniform: [u8; 8],
    pub content_length: u64,
}

pub struct ChallengeSuccessReturn {
    pub chunk_index: u64,
}

pub struct RespondPayload<'a> {
    pub content: &'a [u8],
    pub outboard: &'a Outboard,
    pub chunk_index: u64,
}

pub struct RespondSuccessReturn {
    pub chunk: Chunk,
    pub path: AuthenticationPath,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommitErrorReturn {
    KeyLengthMismatch {
        expected: usize,
        actual: usize,
    },
    Root(RootTryNewErrorReturn),
    Blake3Bao(Blake3BaoCommitmentErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockICommitmentAdapterErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum VerifyErrorReturn {
    KeyLengthMismatch {
        expected: usize,
        actual: usize,
    },
    OutboardTruncated {
        length: usize,
    },
    ContentLengthMismatch {
        declared: u64,
        actual: u64,
    },
    OutboardLengthMismatch {
        expected: u64,
        actual: u64,
    },
    RootMismatch,
    ParentMismatch {
        offset: u64,
        length: u64,
    },
    ChunkMismatch {
        chunk_index: u64,
    },
    Blake3Bao(Blake3BaoCommitmentErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockICommitmentAdapterErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExtractPathErrorReturn {
    OutboardTruncated {
        length: usize,
    },
    OutboardLengthMismatch {
        expected: u64,
        actual: u64,
    },
    ChunkIndexOutOfRange {
        chunk_index: u64,
        chunk_count: u64,
    },
    Blake3Bao(Blake3BaoCommitmentErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockICommitmentAdapterErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum VerifyChunkErrorReturn {
    KeyLengthMismatch {
        expected: usize,
        actual: usize,
    },
    ChunkIndexOutOfRange {
        chunk_index: u64,
        chunk_count: u64,
    },
    ChunkLengthMismatch {
        expected: u64,
        actual: u64,
    },
    PathLengthMismatch {
        expected: usize,
        actual: usize,
    },
    RootMismatch,
    Blake3Bao(Blake3BaoCommitmentErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockICommitmentAdapterErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeErrorReturn {}

#[derive(Debug, PartialEq, Eq)]
pub enum RespondErrorReturn {
    OutboardTruncated {
        length: usize,
    },
    ContentLengthMismatch {
        declared: u64,
        actual: u64,
    },
    OutboardLengthMismatch {
        expected: u64,
        actual: u64,
    },
    ChunkIndexOutOfRange {
        chunk_index: u64,
        chunk_count: u64,
    },
    Blake3Bao(Blake3BaoCommitmentErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockICommitmentAdapterErrorReturn),
}

pub type CommitReturn = Result<CommitSuccessReturn, CommitErrorReturn>;
pub type VerifyReturn = Result<VerifySuccessReturn, VerifyErrorReturn>;
pub type ExtractPathReturn = Result<ExtractPathSuccessReturn, ExtractPathErrorReturn>;
pub type VerifyChunkReturn = Result<VerifyChunkSuccessReturn, VerifyChunkErrorReturn>;
pub type ChallengeReturn = Result<ChallengeSuccessReturn, ChallengeErrorReturn>;
pub type RespondReturn = Result<RespondSuccessReturn, RespondErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockICommitmentAdapterFailureMode {
    NoFailure,
    ContentLengthUnrepresentable,
    OffsetUnrepresentable,
}

pub trait ICommitmentAdapter {
    fn declaration(&self) -> CommitmentDeclaration;
    fn commit(&self, params: CommitParams, payload: CommitPayload<'_>) -> CommitReturn;
    fn verify(&self, params: VerifyParams, payload: VerifyPayload<'_>) -> VerifyReturn;
    fn extract_path(
        &self,
        params: ExtractPathParams,
        payload: ExtractPathPayload<'_>,
    ) -> ExtractPathReturn;
    fn verify_chunk(
        &self,
        params: VerifyChunkParams,
        payload: VerifyChunkPayload<'_>,
    ) -> VerifyChunkReturn;
    fn challenge(&self, params: ChallengeParams, payload: ChallengePayload) -> ChallengeReturn;
    fn respond(&self, params: RespondParams, payload: RespondPayload<'_>) -> RespondReturn;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitmentConcrete {
    Blake3Bao,
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockICommitmentAdapterFailureMode),
}

pub struct CreateCommitmentDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CreateCommitmentParams {
    pub concrete: CommitmentConcrete,
    pub identifier: CommitmentSchemeIdentifier,
}

pub struct CreateCommitmentPayload;

pub struct CreateCommitmentSuccessReturn {
    pub adapter: Box<dyn ICommitmentAdapter>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateCommitmentErrorReturn {
    UnsupportedCommitmentScheme,
    Blake3Bao(Infallible),
    #[cfg(any(test, feature = "mocks"))]
    Mock(Infallible),
}

pub type CreateCommitmentReturn =
    Result<CreateCommitmentSuccessReturn, CreateCommitmentErrorReturn>;

pub type CreateCommitmentFn = fn(
    &CreateCommitmentDeps,
    CreateCommitmentParams,
    CreateCommitmentPayload,
) -> CreateCommitmentReturn;

pub const DECLARED_COMMITMENT_SELECTIONS: &[CreateCommitmentParams] = &[CreateCommitmentParams {
    concrete: CommitmentConcrete::Blake3Bao,
    identifier: CommitmentSchemeIdentifier::Blake3BaoV1,
}];
