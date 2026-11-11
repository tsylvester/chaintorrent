use core::convert::Infallible;
use core::num::NonZeroU32;
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::IPairingAdapter;
use proof::{DeliveryContext, IDeliveryProofAdapter, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;

use crate::delivery::provides::AdmittedKemIdentity;
use crate::measure::provides::MeasureReturn;

pub struct VerifierConsumer<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub(super) pairing: &'a P,
    pub(super) kem: &'a K,
    pub(super) key_agreement: &'a A,
    pub(super) delivery_proof: &'a D,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) mint: DeliveryContext<D::Forms, MintPurpose>,
    pub(super) transfer: DeliveryContext<D::Forms, TransferPurpose>,
    pub(super) identity: AdmittedKemIdentity<'a>,
    pub(super) iterations: NonZeroU32,
}

pub struct VerifierConsumerConstructorParams<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub pairing: &'a P,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
    pub mint: DeliveryContext<D::Forms, MintPurpose>,
    pub transfer: DeliveryContext<D::Forms, TransferPurpose>,
    pub identity: AdmittedKemIdentity<'a>,
    pub iterations: NonZeroU32,
}

pub type VerifierConsumerTryNewReturn<'a, P, K, A, D> =
    Result<VerifierConsumer<'a, P, K, A, D>, Infallible>;

pub type VerifierConsumerReturn = MeasureReturn;
