#![allow(dead_code, clippy::too_many_arguments)]

use chain::IChainForms;
use core::num::NonZeroU32;
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryContext, DeliveryProofConcrete, MintPurpose,
    TransferPurpose, build_create_delivery_proof_params,
};
use random::IRandomSourceAdapter;

use super::interface::{KeyAgreementConsumer, KeyAgreementConsumerConstructorParams};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    GenerateConcrete, VectorKindConfiguration, build_create_generate_params,
};
use crate::verifier::provides::{MockIVerifierAdapterFailureMode, VerifierConcrete};

#[derive(Default)]
pub struct KeyAgreementConsumerConstructorParamsOverrides {
    pub delivery_proof: Option<DeliveryProofConcrete>,
    pub generate: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
    pub verifier: Option<VerifierConcrete>,
    pub statement_version: Option<u16>,
    pub iterations: Option<NonZeroU32>,
}

pub fn build_key_agreement_consumer_constructor_params<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: Vec<AdmittedKemIdentity<'a>>,
    proof_identity: AdmittedKemIdentity<'a>,
    overrides: KeyAgreementConsumerConstructorParamsOverrides,
) -> KeyAgreementConsumerConstructorParams<'a, P, E, K, F> {
    KeyAgreementConsumerConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        mint,
        transfer,
        identities,
        proof_identity,
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default()).concrete),
        generate: overrides
            .generate
            .unwrap_or_else(|| build_create_generate_params(Default::default()).concrete),
        kinds: overrides
            .kinds
            .unwrap_or_else(|| build_create_generate_params(Default::default()).kinds),
        verifier: overrides.verifier.unwrap_or(VerifierConcrete::Mock(
            MockIVerifierAdapterFailureMode::NoFailure,
        )),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
        iterations: overrides.iterations.unwrap_or(NonZeroU32::MIN),
    }
}

pub fn build_key_agreement_consumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: Vec<AdmittedKemIdentity<'a>>,
    proof_identity: AdmittedKemIdentity<'a>,
    overrides: KeyAgreementConsumerConstructorParamsOverrides,
) -> KeyAgreementConsumer<'a, P, E, K, F> {
    let Ok(consumer) =
        KeyAgreementConsumer::try_new(build_key_agreement_consumer_constructor_params(
            pairing,
            hash_to_scalar,
            encoder,
            random,
            kem,
            mint,
            transfer,
            identities,
            proof_identity,
            overrides,
        ));
    consumer
}
