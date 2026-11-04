use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use core::num::NonZeroU32;
use core::time::Duration;
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use kem::{EncapsulateErrorReturn, ICredentialKemAdapter};
use pairing::IPairingAdapter;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use proof::{
    DeliveryContext, DeliveryRelation, IDeliveryProofAdapter, MintPurpose, ProveMintErrorReturn,
    ProveTransferErrorReturn, TransferPurpose, VerifyErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn, TransferErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use crate::verifier::provides::{CreateVerifierErrorReturn, CreateVerifierParams};
use crate::verifier::provides::{IVerifierAdapter, VerifyCost, VerifyOnChainErrorReturn};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeliveryOperationTimings {
    pub prove_mint: Duration,
    pub prove_transfer: Duration,
    pub verify_mint: Duration,
    pub verify_transfer: Duration,
    pub decapsulate: Duration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeliverySizes {
    pub capsule: usize,
    pub envelope: usize,
    pub mint_proof: usize,
    pub transfer_proof: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeliveryCosts {
    pub mint: VerifyCost,
    pub transfer: VerifyCost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeliveryMeasurement {
    pub iterations: NonZeroU32,
    pub timings: DeliveryOperationTimings,
    pub sizes: DeliverySizes,
    pub costs: DeliveryCosts,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RejectedDelivery {
    Rejected,
    Reverted { data: Vec<u8> },
}

pub struct MeasureDeps<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    V: IVerifierAdapter<DeliveryProof = D>,
> {
    pub pairing: &'a P,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub verifier: &'a V,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeasureParams {
    pub iterations: NonZeroU32,
}

pub struct MeasurePayload<'a, F: IChainForms> {
    pub mint: &'a DeliveryContext<F, MintPurpose>,
    pub transfer: &'a DeliveryContext<F, TransferPurpose>,
    pub identity: AdmittedKemIdentity<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeasureSuccessReturn {
    pub measurement: DeliveryMeasurement,
}

#[derive(Debug)]
pub enum MeasureErrorReturn {
    FillBytes(FillBytesErrorReturn),
    Mint(MintErrorReturn),
    Transfer(TransferErrorReturn),
    Encapsulate(EncapsulateErrorReturn),
    ProveMint(ProveMintErrorReturn),
    ProveTransfer(ProveTransferErrorReturn),
    Verify {
        relation: DeliveryRelation,
        error: VerifyErrorReturn,
    },
    OnChain {
        relation: DeliveryRelation,
        error: VerifyOnChainErrorReturn,
    },
    NotAccepted {
        relation: DeliveryRelation,
        verdict: RejectedDelivery,
    },
}

pub type MeasureReturn = Result<MeasureSuccessReturn, MeasureErrorReturn>;

pub type MeasureFn<'a, P, K, A, D, V> = fn(
    &MeasureDeps<'a, P, K, A, D, V>,
    MeasureParams,
    MeasurePayload<'_, <D as IDeliveryProofAdapter>::Forms>,
) -> MeasureReturn;

#[cfg(any(test, feature = "mocks"))]
pub trait IMeasureCaseConsumer {
    type Output;

    fn consume_measure_case<P, H, E, F, K, A, D, V>(
        &self,
        pairing: &P,
        hash_to_scalar: &H,
        encoder: &E,
        forms: &F,
        kem: &K,
        key_agreement: &A,
        delivery_proof: &D,
        verifier: &V,
        random: &dyn IRandomSourceAdapter,
        independent: &dyn IRandomSourceAdapter,
        identity: AdmittedKemIdentity<'_>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        H: ?Sized + IHashToScalarAdapter<P::Scalar>,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        V: IVerifierAdapter<DeliveryProof = D>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct MeasureCaseDeps<C: IMeasureCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MeasureCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub verifier: CreateVerifierParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MeasureCasePayload<'a> {
    pub canonical: &'a [u8],
    pub root: &'a Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MeasureCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug)]
pub enum MeasureCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateKeyAgreement(CreateKeyAgreementErrorReturn),
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    CreateVerifier(CreateVerifierErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type MeasureCaseReturn<O> = Result<MeasureCaseSuccessReturn<O>, MeasureCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunMeasureCaseFn<C> = fn(
    &MeasureCaseDeps<C>,
    MeasureCaseParams,
    MeasureCasePayload<'_>,
) -> MeasureCaseReturn<<C as IMeasureCaseConsumer>::Output>;
