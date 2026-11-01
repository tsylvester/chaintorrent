use crate::mint_statement::provides::MintStatement;
use crate::transfer_statement::provides::TransferStatement;
use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
use encoding::{EncodeErrorReturn, IEncoderAdapter};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
use hash_to_scalar::{DomainTag, HashToScalarErrorReturn, IHashToScalarAdapter};
use pairing::IPairingAdapter;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};

pub const SCHNORR_FS_CHALLENGE_TAG: &[u8] = b"ChainTorrent-v1-proof-challenge";

pub struct ChallengeDeps<'a, P: IPairingAdapter, E: IEncoderAdapter> {
    pub pairing: &'a P,
    pub encoder: &'a E,
    pub tag: &'a DomainTag,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
}

pub struct ChallengeParams;

pub enum ChallengeTranscript<'a, P: IPairingAdapter, F: IChainForms> {
    Mint(&'a MintStatement<P, F>),
    Transfer(&'a TransferStatement<P, F>),
}

pub struct ChallengePayload<'a, P: IPairingAdapter, F: IChainForms> {
    pub transcript: ChallengeTranscript<'a, P, F>,
}

pub struct ChallengeSuccessReturn<S> {
    pub challenge: S,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeErrorReturn {
    Encoding(EncodeErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
}

pub type ChallengeReturn<S> = Result<ChallengeSuccessReturn<S>, ChallengeErrorReturn>;

pub type ChallengeFn<S, E, P, F> =
    fn(&ChallengeDeps<'_, P, E>, ChallengeParams, ChallengePayload<'_, P, F>) -> ChallengeReturn<S>;

#[cfg(any(test, feature = "mocks"))]
pub trait IChallengeCaseConsumer {
    type Output;

    fn consume_challenge_case<
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
        E: IEncoderAdapter + IDecoderAdapter,
    >(
        &self,
        pairing: &P,
        forms: &F,
        encoder: &E,
        hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
        payload: &ChallengeCasePayload,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeCaseDeps<C: IChallengeCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeCaseParams {
    pub pairing: CreatePairingParams,
    pub chain_forms: CreateChainFormsParams,
    pub encoding: CreateEncodingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct ChallengeCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type ChallengeCaseReturn<O> = Result<ChallengeCaseSuccessReturn<O>, ChallengeCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunChallengeCaseFn<C> = fn(
    &ChallengeCaseDeps<C>,
    ChallengeCaseParams,
    ChallengeCasePayload,
) -> ChallengeCaseReturn<<C as IChallengeCaseConsumer>::Output>;
