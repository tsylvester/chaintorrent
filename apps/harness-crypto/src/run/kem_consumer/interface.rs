use chain::IChainForms;
use core::convert::Infallible;
use core::num::NonZeroU32;
use encoding::IEncoderAdapter;
use envelope::CreateKeyAgreementErrorReturn;
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryContext, DeliveryProofConcrete, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;

use crate::config::provides::{AdmittedIdentities, AdmittedKeyAgreement};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};
use crate::run::provides::{DeliveryProofConsumerSuccessReturn, KeyAgreementConsumerErrorReturn};
use crate::verifier::provides::VerifierConcrete;

pub struct KemConsumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) mint: DeliveryContext<F, MintPurpose>,
    pub(super) transfer: DeliveryContext<F, TransferPurpose>,
    pub(super) identities: &'a AdmittedIdentities,
    pub(super) key_agreement: AdmittedKeyAgreement,
    pub(super) delivery_proof: DeliveryProofConcrete,
    pub(super) generate: GenerateConcrete,
    pub(super) kinds: Vec<VectorKindConfiguration>,
    pub(super) verifier: VerifierConcrete,
    pub(super) statement_version: u16,
    pub(super) iterations: NonZeroU32,
}

pub struct KemConsumerConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub mint: DeliveryContext<F, MintPurpose>,
    pub transfer: DeliveryContext<F, TransferPurpose>,
    pub identities: &'a AdmittedIdentities,
    pub key_agreement: AdmittedKeyAgreement,
    pub delivery_proof: DeliveryProofConcrete,
    pub generate: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
    pub verifier: VerifierConcrete,
    pub statement_version: u16,
    pub iterations: NonZeroU32,
}

pub type KemConsumerTryNewReturn<'a, P, E, F> = Result<KemConsumer<'a, P, E, F>, Infallible>;

#[derive(Debug)]
pub enum KemConsumerErrorReturn {
    CreateKeyAgreement(CreateKeyAgreementErrorReturn),
    KeyAgreement(KeyAgreementConsumerErrorReturn),
}

pub type KemConsumerReturn = Result<DeliveryProofConsumerSuccessReturn, KemConsumerErrorReturn>;
