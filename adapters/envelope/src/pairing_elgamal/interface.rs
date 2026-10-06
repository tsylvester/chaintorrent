use domain::Secret;
use encoding::{EncodeErrorReturn, IEncoderAdapter};
use hash_to_scalar::{
    DomainTag, DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter,
};
use pairing::{IPairingAdapter, IPairingArithmetic, SampleUniformScalarErrorReturn};
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
    XSampling(SampleUniformScalarErrorReturn),
    YSampling(SampleUniformScalarErrorReturn),
    IdentityKeyG1,
    IdentityKeyG2,
    SharedSecret,
    NonceDraw(FillBytesErrorReturn),
    NonceSampling(SampleUniformScalarErrorReturn),
    Encoding(EncodeErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalKeyPairFromComponentsErrorReturn {
    IdentityKeyG1,
    IdentityKeyG2,
    SharedSecret,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalPublicKeysFromComponentsErrorReturn {
    IdentityKeyG1,
    IdentityKeyG2,
    SharedSecret,
    PossessionG1,
    PossessionG2,
    Encoding(EncodeErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingElGamalWrapToErrorReturn {
    CoinDraw(FillBytesErrorReturn),
    CoinSampling(SampleUniformScalarErrorReturn),
}
