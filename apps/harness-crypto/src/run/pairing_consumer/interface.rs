use core::convert::Infallible;
use core::num::NonZeroU32;
use encoding::CreateEncodingErrorReturn;
use hash_to_scalar::CreateHashToScalarErrorReturn;
use proof::DeliveryProofConcrete;
use random::IRandomSourceAdapter;

use crate::config::provides::{
    AdmittedChainForms, AdmittedEncoding, AdmittedHashToScalar, AdmittedIdentities, AdmittedKem,
    AdmittedKeyAgreement, AdmittedStatements,
};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};
use crate::run::provides::{DeliveryProofConsumerSuccessReturn, EncodingConsumerErrorReturn};
use crate::verifier::provides::VerifierConcrete;

pub struct PairingConsumer<'a> {
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) statements: &'a AdmittedStatements,
    pub(super) identities: &'a AdmittedIdentities,
    pub(super) hash_to_scalar: AdmittedHashToScalar,
    pub(super) encoding: AdmittedEncoding,
    pub(super) chain_forms: AdmittedChainForms,
    pub(super) kem: AdmittedKem,
    pub(super) key_agreement: AdmittedKeyAgreement,
    pub(super) delivery_proof: DeliveryProofConcrete,
    pub(super) generate: GenerateConcrete,
    pub(super) kinds: Vec<VectorKindConfiguration>,
    pub(super) verifier: VerifierConcrete,
    pub(super) statement_version: u16,
    pub(super) iterations: NonZeroU32,
}

pub struct PairingConsumerConstructorParams<'a> {
    pub random: &'a dyn IRandomSourceAdapter,
    pub statements: &'a AdmittedStatements,
    pub identities: &'a AdmittedIdentities,
    pub hash_to_scalar: AdmittedHashToScalar,
    pub encoding: AdmittedEncoding,
    pub chain_forms: AdmittedChainForms,
    pub kem: AdmittedKem,
    pub key_agreement: AdmittedKeyAgreement,
    pub delivery_proof: DeliveryProofConcrete,
    pub generate: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
    pub verifier: VerifierConcrete,
    pub statement_version: u16,
    pub iterations: NonZeroU32,
}

pub type PairingConsumerTryNewReturn<'a> = Result<PairingConsumer<'a>, Infallible>;

#[derive(Debug)]
pub enum PairingConsumerErrorReturn {
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    Encoding(EncodingConsumerErrorReturn),
}

pub type PairingConsumerReturn =
    Result<DeliveryProofConsumerSuccessReturn, PairingConsumerErrorReturn>;
