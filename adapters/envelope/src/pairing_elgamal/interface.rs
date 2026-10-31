use domain::Secret;
#[cfg(test)]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
use encoding::{EncodeErrorReturn, IEncoderAdapter};
#[cfg(test)]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
use hash_to_scalar::{
    DomainTag, DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter,
};
#[cfg(test)]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingReference};
use pairing::{IPairingAdapter, IPairingArithmetic, SampleUniformScalarErrorReturn};
#[cfg(test)]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::possession_statement::provides::{
    PossessionG1StatementDescription, PossessionG2StatementDescription,
};

pub const PAIRING_ELGAMAL_POSSESSION_G1_TAG: &[u8] = b"ChainTorrent-v1-envelope-possession-g1";
pub const PAIRING_ELGAMAL_POSSESSION_G2_TAG: &[u8] = b"ChainTorrent-v1-envelope-possession-g2";

pub struct PairingElGamalKeyAgreement<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) g1_statement_description: PossessionG1StatementDescription<'a, P>,
    pub(super) g2_statement_description: PossessionG2StatementDescription<'a, P>,
    pub(super) g1_tag: DomainTag,
    pub(super) g2_tag: DomainTag,
}

pub struct PairingElGamalKeyAgreementConstructorParams<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalKeyAgreementTryNewErrorReturn {
    PossessionG1Tag(DomainTagTryNewErrorReturn),
    PossessionG2Tag(DomainTagTryNewErrorReturn),
}

pub type PairingElGamalKeyAgreementTryNewReturn<'a, P, E> =
    Result<PairingElGamalKeyAgreement<'a, P, E>, PairingElGamalKeyAgreementTryNewErrorReturn>;

pub struct PairingElGamalKeyPair<P: IPairingAdapter> {
    pub(super) x: Secret<P::Scalar>,
    pub(super) y: Secret<P::Scalar>,
    pub(super) pk1: P::G1,
    pub(super) pk2: P::G2,
}

pub struct PairingElGamalPublicKeys<P: IPairingAdapter> {
    pub(super) pk1: P::G1,
    pub(super) pk2: P::G2,
}

pub struct PairingElGamalPossession<P: IPairingAdapter> {
    pub(super) r1: P::G1,
    pub(super) z1: P::Scalar,
    pub(super) r2: P::G2,
    pub(super) z2: P::Scalar,
}

pub struct PairingElGamalEnvelope<P: IPairingAdapter> {
    pub(super) c1: P::G1,
    pub(super) c2: P::G1,
    pub(super) d1: P::G2,
    pub(super) d2: P::G2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalGenerateKeysErrorReturn {
    NonceDraw(FillBytesErrorReturn),
    NonceSampling(SampleUniformScalarErrorReturn),
    Encoding(EncodeErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalPublicKeysFromComponentsErrorReturn {
    Encoding(EncodeErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalWrapToErrorReturn {
    CoinDraw(FillBytesErrorReturn),
    CoinSampling(SampleUniformScalarErrorReturn),
}

#[cfg(test)]
pub trait IPairingElGamalKeyAgreementCaseConsumer {
    type Output;

    fn consume_pairing_elgamal_key_agreement_case<
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
    >(
        &self,
        pairing: &P,
        hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &E,
        random: &dyn IRandomSourceAdapter,
        payload: &PairingElGamalKeyAgreementCasePayload,
    ) -> Self::Output;
}

#[cfg(test)]
pub struct PairingElGamalKeyAgreementCaseDeps<C: IPairingElGamalKeyAgreementCaseConsumer> {
    pub consumer: C,
}

#[cfg(test)]
pub struct PairingElGamalKeyAgreementCaseParams {
    pub pairing: CreatePairingParams,
    pub encoding: CreateEncodingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(test)]
pub struct PairingElGamalKeyAgreementCasePayload;

#[cfg(test)]
pub struct PairingElGamalKeyAgreementCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalKeyAgreementCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(test)]
pub type PairingElGamalKeyAgreementCaseReturn<O> = Result<
    PairingElGamalKeyAgreementCaseSuccessReturn<O>,
    PairingElGamalKeyAgreementCaseErrorReturn,
>;

#[cfg(test)]
pub type RunPairingElGamalKeyAgreementCaseFn<C> = fn(
    &PairingElGamalKeyAgreementCaseDeps<C>,
    PairingElGamalKeyAgreementCaseParams,
    PairingElGamalKeyAgreementCasePayload,
) -> PairingElGamalKeyAgreementCaseReturn<
    <C as IPairingElGamalKeyAgreementCaseConsumer>::Output,
>;
