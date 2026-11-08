use chain::IChainForms;
use core::convert::Infallible;
use core::num::NonZeroU32;
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    CreateDeliveryProofErrorReturn, DeliveryContext, DeliveryProofConcrete, MintPurpose,
    TransferPurpose,
};
use random::IRandomSourceAdapter;

use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};
use crate::run::provides::{DeliveryProofConsumerErrorReturn, DeliveryProofConsumerSuccessReturn};
use crate::verifier::provides::VerifierConcrete;

pub struct KeyAgreementConsumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) kem: &'a K,
    pub(super) mint: DeliveryContext<F, MintPurpose>,
    pub(super) transfer: DeliveryContext<F, TransferPurpose>,
    pub(super) identities: Vec<AdmittedKemIdentity<'a>>,
    pub(super) proof_identity: AdmittedKemIdentity<'a>,
    pub(super) delivery_proof: DeliveryProofConcrete,
    pub(super) generate: GenerateConcrete,
    pub(super) kinds: Vec<VectorKindConfiguration>,
    pub(super) verifier: VerifierConcrete,
    pub(super) statement_version: u16,
    pub(super) iterations: NonZeroU32,
}

pub struct KeyAgreementConsumerConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kem: &'a K,
    pub mint: DeliveryContext<F, MintPurpose>,
    pub transfer: DeliveryContext<F, TransferPurpose>,
    pub identities: Vec<AdmittedKemIdentity<'a>>,
    pub proof_identity: AdmittedKemIdentity<'a>,
    pub delivery_proof: DeliveryProofConcrete,
    pub generate: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
    pub verifier: VerifierConcrete,
    pub statement_version: u16,
    pub iterations: NonZeroU32,
}

pub type KeyAgreementConsumerTryNewReturn<'a, P, E, K, F> =
    Result<KeyAgreementConsumer<'a, P, E, K, F>, Infallible>;

#[derive(Debug)]
pub enum KeyAgreementConsumerErrorReturn {
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    DeliveryProof(DeliveryProofConsumerErrorReturn),
}

pub type KeyAgreementConsumerReturn =
    Result<DeliveryProofConsumerSuccessReturn, KeyAgreementConsumerErrorReturn>;
