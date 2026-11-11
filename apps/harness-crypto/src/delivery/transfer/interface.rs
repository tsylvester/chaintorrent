#[cfg(any(test, feature = "mocks"))]
use chain::CreateChainFormsErrorReturn;
use chain::IChainForms;
use domain::Secret;
#[cfg(any(test, feature = "mocks"))]
use encoding::CreateEncodingErrorReturn;
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
use envelope::{
    EnvelopeCoins, GenerateKeysErrorReturn, IKeyAgreementAdapter, KeyPairComponents,
    PublicKeysFromComponentsErrorReturn, WrapToErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::CreateHashToScalarErrorReturn;
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::CreateKemErrorReturn;
use kem::{ICredentialKemAdapter, RerandomizeErrorReturn};
use pairing::IPairingAdapter;
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, IPairingArithmetic, IPairingReference};
use proof::{
    AlgebraicTransferStatement, DeliveryContext, IDeliveryProofAdapter, ProveTransferErrorReturn,
    TransferPurpose,
};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::delivery::mint::provides::DeliveryHolder;
#[cfg(any(test, feature = "mocks"))]
use crate::delivery::mint::provides::{AdmittedKemIdentity, MintCaseParams};

pub struct TransferWitnesses<P: IPairingAdapter> {
    pub seller_secrets: KeyPairComponents<P::Scalar>,
    pub offset: Secret<P::Scalar>,
    pub coins: EnvelopeCoins<P::Scalar>,
}

pub struct TransferDelivery<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    pub proof: D::Proof,
    pub credential: K::Credential,
    pub witnesses: TransferWitnesses<P>,
    pub buyer: DeliveryHolder<A>,
    pub draws: Vec<u8>,
}

pub struct TransferDeps<
    'a,
    K: ICredentialKemAdapter,
    A: IKeyAgreementAdapter<Pairing = K::Pairing>,
    D: IDeliveryProofAdapter<Pairing = K::Pairing>,
> {
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
}

pub struct TransferParams;

pub struct TransferPayload<'a, K: ICredentialKemAdapter, A: IKeyAgreementAdapter, F: IChainForms> {
    pub context: &'a DeliveryContext<F, TransferPurpose>,
    pub parameter_set: &'a K::ParameterSet,
    pub identity_element: &'a K::IdentityElement,
    pub holder: &'a DeliveryHolder<A>,
}

pub struct TransferSuccessReturn<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub delivery: TransferDelivery<P, K, A, D>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TransferErrorReturn {
    FillBytes(FillBytesErrorReturn),
    GenerateKeys(GenerateKeysErrorReturn),
    PublicKeysFromComponents(PublicKeysFromComponentsErrorReturn),
    Rerandomize(RerandomizeErrorReturn),
    WrapTo(WrapToErrorReturn),
    ProveTransfer(ProveTransferErrorReturn),
}

pub type TransferReturn<P, K, A, D> =
    Result<TransferSuccessReturn<P, K, A, D>, TransferErrorReturn>;

pub type TransferFn<'a, P, K, A, D> = fn(
    &TransferDeps<'a, K, A, D>,
    TransferParams,
    TransferPayload<'_, K, A, <D as IDeliveryProofAdapter>::Forms>,
) -> TransferReturn<P, K, A, D>;

#[cfg(any(test, feature = "mocks"))]
pub trait ITransferCaseConsumer {
    type Output;

    fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
        &self,
        params: ConsumeTransferCaseParams,
        payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        A1: IKeyAgreementAdapter<Pairing = P>,
        A2: IKeyAgreementAdapter<Pairing = P>,
        D1: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        D2: IDeliveryProofAdapter<Pairing = P, Forms = F>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeTransferCaseParams;

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeTransferCasePayload<
    'a,
    P: IPairingArithmetic + IPairingReference,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A1: IKeyAgreementAdapter<Pairing = P>,
    A2: IKeyAgreementAdapter<Pairing = P>,
    D1: IDeliveryProofAdapter<Pairing = P, Forms = F>,
    D2: IDeliveryProofAdapter<Pairing = P, Forms = F>,
> {
    pub pairing: &'a P,
    pub kem: &'a K,
    pub key_agreement: &'a A1,
    pub transfer_key_agreement: &'a A2,
    pub delivery_proof: &'a D1,
    pub transfer_delivery_proof: &'a D2,
    pub mint_source: &'a dyn IRandomSourceAdapter,
    pub transfer_source: &'a dyn IRandomSourceAdapter,
    pub identity: AdmittedKemIdentity<'a>,
}

#[cfg(any(test, feature = "mocks"))]
pub struct TransferCaseDeps<C: ITransferCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct TransferCaseParams {
    pub mint: MintCaseParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct TransferCasePayload<'a> {
    pub canonical: &'a [u8],
    pub identity_hash: &'a Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct TransferCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum TransferCaseErrorReturn {
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
pub type TransferCaseReturn<O> = Result<TransferCaseSuccessReturn<O>, TransferCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunTransferCaseFn<C> = fn(
    &TransferCaseDeps<C>,
    TransferCaseParams,
    TransferCasePayload<'_>,
) -> TransferCaseReturn<<C as ITransferCaseConsumer>::Output>;
