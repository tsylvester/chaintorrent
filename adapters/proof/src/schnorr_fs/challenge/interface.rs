use crate::mint_statement::provides::MintStatement;
use crate::transfer_statement::provides::TransferStatement;
use chain::IChainForms;
use encoding::{EncodeErrorReturn, IEncoderAdapter};
use hash_to_scalar::{DomainTag, HashToScalarErrorReturn, IHashToScalarAdapter};
use pairing::IPairingAdapter;

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
