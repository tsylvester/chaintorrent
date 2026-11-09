use chain::CreateChainFormsErrorReturn;
use core::convert::Infallible;
use core::num::NonZeroU32;
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::DeliveryProofConcrete;
use random::IRandomSourceAdapter;

use crate::config::provides::{
    AdmittedChainForms, AdmittedIdentities, AdmittedKem, AdmittedKeyAgreement, AdmittedStatements,
};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};
use crate::run::provides::{ChainFormsConsumerErrorReturn, DeliveryProofConsumerSuccessReturn};
use crate::verifier::provides::VerifierConcrete;

pub struct EncodingConsumer<'a, P: IPairingArithmetic + IPairingReference> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) statements: &'a AdmittedStatements,
    pub(super) identities: &'a AdmittedIdentities,
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

pub struct EncodingConsumerConstructorParams<'a, P: IPairingArithmetic + IPairingReference> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub random: &'a dyn IRandomSourceAdapter,
    pub statements: &'a AdmittedStatements,
    pub identities: &'a AdmittedIdentities,
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

pub type EncodingConsumerTryNewReturn<'a, P> = Result<EncodingConsumer<'a, P>, Infallible>;

#[derive(Debug)]
pub enum EncodingConsumerErrorReturn {
    CreateChainForms(CreateChainFormsErrorReturn),
    ChainForms(ChainFormsConsumerErrorReturn),
}

pub type EncodingConsumerReturn =
    Result<DeliveryProofConsumerSuccessReturn, EncodingConsumerErrorReturn>;
