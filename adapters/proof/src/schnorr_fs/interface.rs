use chain::IChainForms;
#[cfg(test)]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use core::marker::PhantomData;
use encoding::IEncoderAdapter;
#[cfg(test)]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
#[cfg(test)]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
use hash_to_scalar::{
    DomainTag, DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter,
};
#[cfg(test)]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingReference};
use pairing::{
    IPairingAdapter, IPairingArithmetic, SampleUniformScalarErrorReturn, VerifierGroupArithmetic,
};
#[cfg(test)]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
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

#[cfg(test)]
pub trait ISchnorrFsCaseConsumer {
    type Output;

    fn consume_schnorr_fs_case<
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
        E: IEncoderAdapter + IDecoderAdapter,
    >(
        &self,
        pairing: &P,
        forms: &F,
        encoder: &E,
        hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
        random: &dyn IRandomSourceAdapter,
        payload: &SchnorrFsCasePayload,
    ) -> Self::Output;
}

#[cfg(test)]
pub struct SchnorrFsCaseDeps<C: ISchnorrFsCaseConsumer> {
    pub consumer: C,
}

#[cfg(test)]
pub struct SchnorrFsCaseParams {
    pub pairing: CreatePairingParams,
    pub chain_forms: CreateChainFormsParams,
    pub encoding: CreateEncodingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(test)]
pub struct SchnorrFsCasePayload;

#[cfg(test)]
pub struct SchnorrFsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
pub enum SchnorrFsCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(test)]
pub type SchnorrFsCaseReturn<O> = Result<SchnorrFsCaseSuccessReturn<O>, SchnorrFsCaseErrorReturn>;

#[cfg(test)]
pub type RunSchnorrFsCaseFn<C> = fn(
    &SchnorrFsCaseDeps<C>,
    SchnorrFsCaseParams,
    SchnorrFsCasePayload,
) -> SchnorrFsCaseReturn<<C as ISchnorrFsCaseConsumer>::Output>;
