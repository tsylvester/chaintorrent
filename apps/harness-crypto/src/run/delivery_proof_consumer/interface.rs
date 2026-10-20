use core::convert::Infallible;
use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

use crate::config::provides::HarnessStatements;
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

pub struct DeliveryProofConsumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) kem: &'a K,
    pub(super) key_agreement: &'a A,
    pub(super) generate: &'a GenerateConcrete,
    pub(super) kinds: &'a [VectorKindConfiguration],
    pub(super) scope: IdentityScope,
    pub(super) statement_version: u16,
    pub(super) statements: &'a HarnessStatements,
}

pub struct DeliveryProofConsumerConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub generate: &'a GenerateConcrete,
    pub kinds: &'a [VectorKindConfiguration],
    pub scope: IdentityScope,
    pub statement_version: u16,
    pub statements: &'a HarnessStatements,
}

pub type DeliveryProofConsumerTryNewReturn<'a, P, E, K, A> =
    Result<DeliveryProofConsumer<'a, P, E, K, A>, Infallible>;
