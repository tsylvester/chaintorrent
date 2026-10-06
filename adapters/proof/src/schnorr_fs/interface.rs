use chain::IChainForms;
use core::marker::PhantomData;
use encoding::IEncoderAdapter;
use hash_to_scalar::{
    DomainTag, DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter,
};
use pairing::{
    IPairingAdapter, IPairingArithmetic, SampleUniformScalarErrorReturn, VerifierGroupArithmetic,
};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::challenge::provides::ChallengeErrorReturn;
use crate::factory::provides::{
    MintResponses, MintSecondGroupFirstMessages, TransferResponses,
    TransferSecondGroupFirstMessages,
};

pub const SCHNORR_FS_WEIGHT_TAG: &[u8] = b"ChainTorrent-v1-proof-weight";

pub struct SchnorrFsDeliveryProof<'a, P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) verifier_form: VerifierGroupArithmetic,
    pub(super) challenge_tag: DomainTag,
    pub(super) weight_tag: DomainTag,
    pub(super) forms: PhantomData<F>,
}

pub struct SchnorrFsDeliveryProofConstructorParams<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub verifier_form: VerifierGroupArithmetic,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SchnorrFsDeliveryProofTryNewErrorReturn {
    ChallengeTag(DomainTagTryNewErrorReturn),
    WeightTag(DomainTagTryNewErrorReturn),
}

pub type SchnorrFsDeliveryProofTryNewReturn<'a, P, E, F> =
    Result<SchnorrFsDeliveryProof<'a, P, E, F>, SchnorrFsDeliveryProofTryNewErrorReturn>;

pub enum SchnorrFsProof<P: IPairingAdapter> {
    MintBothGroups {
        challenge: P::Scalar,
        responses: MintResponses<P::Scalar>,
    },
    MintFirstGroupOnly {
        challenge: P::Scalar,
        responses: MintResponses<P::Scalar>,
        first_messages: MintSecondGroupFirstMessages<P::G2>,
    },
    TransferBothGroups {
        challenge: P::Scalar,
        responses: TransferResponses<P::Scalar>,
    },
    TransferFirstGroupOnly {
        challenge: P::Scalar,
        responses: TransferResponses<P::Scalar>,
        first_messages: TransferSecondGroupFirstMessages<P::G2>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum SchnorrFsProveMintErrorReturn {
    NonceDraw(FillBytesErrorReturn),
    NonceSampling(SampleUniformScalarErrorReturn),
    Challenge(ChallengeErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SchnorrFsProveTransferErrorReturn {
    NonceDraw(FillBytesErrorReturn),
    NonceSampling(SampleUniformScalarErrorReturn),
    Challenge(ChallengeErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SchnorrFsVerifyErrorReturn {
    Challenge(ChallengeErrorReturn),
    Weight(HashToScalarErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SchnorrFsProofFromComponentsErrorReturn {
    VerifierFormMismatch,
}
