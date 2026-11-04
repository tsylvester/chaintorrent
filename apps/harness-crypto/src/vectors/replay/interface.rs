use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
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
use kem::ICredentialKemAdapter;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use pairing::IPairingAdapter;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
use proof::{
    AlgebraicTransferStatement, DeliveryContext, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, VerifyErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use random::IRandomSourceAdapter;
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};

use crate::VectorCount;
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn, TransferErrorReturn};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplaySettlement {
    Transfer,
    Next,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayVerdict {
    Transfer,
    Next,
    Replay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NextSettlementField {
    Chain,
    EntitlementContract,
    AssetRoot,
    ParameterSetIdentifier,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    Seller,
}

pub struct ReplayVector<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> {
    pub draws: Vec<u8>,
    pub transfer_statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    pub transfer_proof: D::Proof,
    pub transfer_is_valid: bool,
    pub next_statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    pub next_proof: D::Proof,
    pub next_is_valid: bool,
    pub replay_is_valid: bool,
}

pub struct ReplayDeps<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplayParams {
    pub count: VectorCount,
}

pub struct ReplayPayload<'a, F: IChainForms> {
    pub mint: &'a DeliveryContext<F, MintPurpose>,
    pub transfer: &'a DeliveryContext<F, TransferPurpose>,
    pub next: &'a DeliveryContext<F, TransferPurpose>,
    pub identity: AdmittedKemIdentity<'a>,
}

pub struct ReplaySuccessReturn<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> {
    pub vectors: Vec<ReplayVector<P, D>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReplayErrorReturn {
    NotTheNextSettlement {
        field: NextSettlementField,
    },
    Mint(MintErrorReturn),
    Transfer {
        settlement: ReplaySettlement,
        error: TransferErrorReturn,
    },
    Verify {
        verdict: ReplayVerdict,
        error: VerifyErrorReturn,
    },
}

pub type ReplayReturn<P, D> = Result<ReplaySuccessReturn<P, D>, ReplayErrorReturn>;

pub type ReplayFn<'a, P, K, A, D> = fn(
    &ReplayDeps<'a, P, K, A, D>,
    ReplayParams,
    ReplayPayload<'_, <D as IDeliveryProofAdapter>::Forms>,
) -> ReplayReturn<P, D>;

#[cfg(any(test, feature = "mocks"))]
pub trait IReplayCaseConsumer {
    type Output;

    fn consume_replay_case<P, H, E, F, K, A, D>(
        &self,
        pairing: &P,
        hash_to_scalar: &H,
        encoder: &E,
        forms: &F,
        kem: &K,
        key_agreement: &A,
        delivery_proof: &D,
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
        D: IDeliveryProofAdapter<Pairing = P, Forms = F>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ReplayCaseDeps<C: IReplayCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ReplayCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ReplayCasePayload<'a> {
    pub canonical: &'a [u8],
    pub root: &'a Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ReplayCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum ReplayCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateKeyAgreement(CreateKeyAgreementErrorReturn),
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type ReplayCaseReturn<O> = Result<ReplayCaseSuccessReturn<O>, ReplayCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunReplayCaseFn<C> = fn(
    &ReplayCaseDeps<C>,
    ReplayCaseParams,
    ReplayCasePayload<'_>,
) -> ReplayCaseReturn<<C as IReplayCaseConsumer>::Output>;
