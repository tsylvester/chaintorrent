#![allow(dead_code, clippy::too_many_arguments)]

use core::num::NonZeroU32;
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::IPairingAdapter;
use proof::{DeliveryContext, IDeliveryProofAdapter, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;

use super::interface::{VerifierConsumer, VerifierConsumerConstructorParams};
use crate::delivery::provides::AdmittedKemIdentity;

#[derive(Default)]
pub struct VerifierConsumerConstructorParamsOverrides {
    pub iterations: Option<NonZeroU32>,
}

pub fn build_verifier_consumer_constructor_params<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    mint: DeliveryContext<D::Forms, MintPurpose>,
    transfer: DeliveryContext<D::Forms, TransferPurpose>,
    identity: AdmittedKemIdentity<'a>,
    overrides: VerifierConsumerConstructorParamsOverrides,
) -> VerifierConsumerConstructorParams<'a, P, K, A, D> {
    VerifierConsumerConstructorParams {
        pairing,
        kem,
        key_agreement,
        delivery_proof,
        random,
        mint,
        transfer,
        identity,
        iterations: overrides.iterations.unwrap_or(NonZeroU32::MIN),
    }
}

pub fn build_verifier_consumer<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    mint: DeliveryContext<D::Forms, MintPurpose>,
    transfer: DeliveryContext<D::Forms, TransferPurpose>,
    identity: AdmittedKemIdentity<'a>,
    overrides: VerifierConsumerConstructorParamsOverrides,
) -> VerifierConsumer<'a, P, K, A, D> {
    let Ok(consumer) = VerifierConsumer::try_new(build_verifier_consumer_constructor_params(
        pairing,
        kem,
        key_agreement,
        delivery_proof,
        random,
        mint,
        transfer,
        identity,
        overrides,
    ));
    consumer
}
