#![allow(dead_code, clippy::too_many_arguments)]

use chain::IChainForms;
use core::num::NonZeroU32;
use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DELIVERY_STATEMENT_VERSION_ONE, DeliveryContext, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;

use super::interface::{DeliveryProofConsumer, DeliveryProofConsumerConstructorParams};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    GenerateConcrete, VectorKindConfiguration, build_create_generate_params,
};
use crate::verifier::provides::{MockIVerifierAdapterFailureMode, VerifierConcrete};

#[derive(Default)]
pub struct DeliveryProofConsumerConstructorParamsOverrides {
    pub generate: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
    pub verifier: Option<VerifierConcrete>,
    pub statement_version: Option<u16>,
    pub iterations: Option<NonZeroU32>,
}

pub fn build_delivery_proof_consumer_constructor_params<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    F: IChainForms,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: Vec<AdmittedKemIdentity<'a>>,
    proof_identity: AdmittedKemIdentity<'a>,
    overrides: DeliveryProofConsumerConstructorParamsOverrides,
) -> DeliveryProofConsumerConstructorParams<'a, P, E, K, A, F> {
    DeliveryProofConsumerConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        mint,
        transfer,
        identities,
        proof_identity,
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

pub fn build_delivery_proof_consumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    F: IChainForms,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: Vec<AdmittedKemIdentity<'a>>,
    proof_identity: AdmittedKemIdentity<'a>,
    overrides: DeliveryProofConsumerConstructorParamsOverrides,
) -> DeliveryProofConsumer<'a, P, E, K, A, F> {
    let Ok(consumer) =
        DeliveryProofConsumer::try_new(build_delivery_proof_consumer_constructor_params(
            pairing,
            hash_to_scalar,
            encoder,
            random,
            kem,
            key_agreement,
            mint,
            transfer,
            identities,
            proof_identity,
            overrides,
        ));
    consumer
}
