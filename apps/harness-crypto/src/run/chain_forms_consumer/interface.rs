use chain::IChainForms;
use core::convert::Infallible;
use core::num::NonZeroU32;
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::CreateKemErrorReturn;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryContext, DeliveryProofConcrete, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;

use crate::config::provides::{
    AdmittedIdentities, AdmittedKem, AdmittedKeyAgreement, AdmittedStatements,
};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};
use crate::run::provides::{
    AdmitContextsErrorReturn, DeliveryProofConsumerSuccessReturn, KemConsumerErrorReturn,
};
use crate::verifier::provides::VerifierConcrete;

pub struct ChainFormsConsumer<'a, P: IPairingArithmetic + IPairingReference, E: IEncoderAdapter> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) statements: &'a AdmittedStatements,
    pub(super) identities: &'a AdmittedIdentities,
    pub(super) kem: AdmittedKem,
    pub(super) key_agreement: AdmittedKeyAgreement,
    pub(super) delivery_proof: DeliveryProofConcrete,
    pub(super) generate: GenerateConcrete,
    pub(super) kinds: Vec<VectorKindConfiguration>,
    pub(super) verifier: VerifierConcrete,
    pub(super) statement_version: u16,
    pub(super) iterations: NonZeroU32,
}

pub struct ChainFormsConsumerConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub statements: &'a AdmittedStatements,
    pub identities: &'a AdmittedIdentities,
    pub kem: AdmittedKem,
    pub key_agreement: AdmittedKeyAgreement,
    pub delivery_proof: DeliveryProofConcrete,
    pub generate: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
    pub verifier: VerifierConcrete,
    pub statement_version: u16,
    pub iterations: NonZeroU32,
}

pub type ChainFormsConsumerTryNewReturn<'a, P, E> =
    Result<ChainFormsConsumer<'a, P, E>, Infallible>;

#[derive(Debug)]
pub enum ChainFormsConsumerErrorReturn {
    AdmitMint(AdmitContextsErrorReturn),
    AdmitTransfer(AdmitContextsErrorReturn),
    CreateKem(CreateKemErrorReturn),
    Kem(KemConsumerErrorReturn),
}

pub type ChainFormsConsumerReturn =
    Result<DeliveryProofConsumerSuccessReturn, ChainFormsConsumerErrorReturn>;

pub struct AdmitStatementContextsSuccessReturn<F: IChainForms> {
    pub mint: DeliveryContext<F, MintPurpose>,
    pub transfer: DeliveryContext<F, TransferPurpose>,
}

pub type AdmitStatementContextsReturn<F> =
    Result<AdmitStatementContextsSuccessReturn<F>, ChainFormsConsumerErrorReturn>;
