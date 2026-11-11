use chain::IChainForms;
use core::convert::Infallible;
use core::num::NonZeroU32;
use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryContext, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;

use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    CreateGenerateErrorReturn, EmitErrorReturn, EmittedFile, GenerateConcrete,
    VectorKindConfiguration,
};
use crate::measure::provides::{DeliveryMeasurement, MeasureErrorReturn};
use crate::verifier::provides::{CreateVerifierErrorReturn, VerifierConcrete};

pub struct DeliveryProofConsumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    F: IChainForms,
> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) kem: &'a K,
    pub(super) key_agreement: &'a A,
    pub(super) mint: DeliveryContext<F, MintPurpose>,
    pub(super) transfer: DeliveryContext<F, TransferPurpose>,
    pub(super) identities: Vec<AdmittedKemIdentity<'a>>,
    pub(super) proof_identity: AdmittedKemIdentity<'a>,
    pub(super) generate: GenerateConcrete,
    pub(super) kinds: Vec<VectorKindConfiguration>,
    pub(super) verifier: VerifierConcrete,
    pub(super) statement_version: u16,
    pub(super) iterations: NonZeroU32,
}

pub struct DeliveryProofConsumerConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    F: IChainForms,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub mint: DeliveryContext<F, MintPurpose>,
    pub transfer: DeliveryContext<F, TransferPurpose>,
    pub identities: Vec<AdmittedKemIdentity<'a>>,
    pub proof_identity: AdmittedKemIdentity<'a>,
    pub generate: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
    pub verifier: VerifierConcrete,
    pub statement_version: u16,
    pub iterations: NonZeroU32,
}

pub type DeliveryProofConsumerTryNewReturn<'a, P, E, K, A, F> =
    Result<DeliveryProofConsumer<'a, P, E, K, A, F>, Infallible>;

pub struct DeliveryProofConsumerSuccessReturn {
    pub files: Vec<EmittedFile>,
    pub measurement: DeliveryMeasurement,
}

#[derive(Debug)]
pub enum DeliveryProofConsumerErrorReturn {
    CreateGenerate(CreateGenerateErrorReturn),
    Emit(EmitErrorReturn),
    CreateVerifier(CreateVerifierErrorReturn),
    Measure(MeasureErrorReturn),
}

pub type DeliveryProofConsumerReturn =
    Result<DeliveryProofConsumerSuccessReturn, DeliveryProofConsumerErrorReturn>;
