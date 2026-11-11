#![allow(dead_code, clippy::too_many_arguments)]

use chain::IChainForms;
use core::num::NonZeroU32;
use encoding::IEncoderAdapter;
use envelope::build_create_key_agreement_params;
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryContext, DeliveryProofConcrete, MintPurpose,
    TransferPurpose, build_create_delivery_proof_params,
};
use random::IRandomSourceAdapter;

use super::interface::{KemConsumer, KemConsumerConstructorParams};
use crate::config::provides::{AdmittedIdentities, AdmittedKeyAgreement};
use crate::generate::provides::{
    GenerateConcrete, VectorKindConfiguration, build_create_generate_params,
};
use crate::verifier::provides::{MockIVerifierAdapterFailureMode, VerifierConcrete};

#[derive(Default)]
pub struct KemConsumerConstructorParamsOverrides {
    pub key_agreement: Option<AdmittedKeyAgreement>,
    pub delivery_proof: Option<DeliveryProofConcrete>,
    pub generate: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
    pub verifier: Option<VerifierConcrete>,
    pub statement_version: Option<u16>,
    pub iterations: Option<NonZeroU32>,
}

pub fn build_kem_consumer_constructor_params<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: &'a AdmittedIdentities,
    overrides: KemConsumerConstructorParamsOverrides,
) -> KemConsumerConstructorParams<'a, P, E, F> {
    KemConsumerConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        mint,
        transfer,
        identities,
        key_agreement: overrides.key_agreement.unwrap_or_else(|| {
            let params = build_create_key_agreement_params(Default::default());
            AdmittedKeyAgreement {
                concrete: params.concrete,
                identifier: params.identifier,
                algebra: params.algebra,
            }
        }),
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

pub fn build_kem_consumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: &'a AdmittedIdentities,
    overrides: KemConsumerConstructorParamsOverrides,
) -> KemConsumer<'a, P, E, F> {
    let Ok(consumer) = KemConsumer::try_new(build_kem_consumer_constructor_params(
        pairing,
        hash_to_scalar,
        encoder,
        random,
        mint,
        transfer,
        identities,
        overrides,
    ));
    consumer
}
