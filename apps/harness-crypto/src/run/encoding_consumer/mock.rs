#![allow(dead_code, clippy::too_many_arguments)]

use chain::build_create_chain_forms_params;
use core::num::NonZeroU32;
use envelope::build_create_key_agreement_params;
use hash_to_scalar::IHashToScalarAdapter;
use kem::build_create_kem_params;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete, build_create_delivery_proof_params,
};
use random::IRandomSourceAdapter;

use super::interface::{EncodingConsumer, EncodingConsumerConstructorParams};
use crate::config::provides::{
    AdmittedChainForms, AdmittedIdentities, AdmittedKem, AdmittedKeyAgreement, AdmittedStatements,
};
use crate::generate::provides::{
    GenerateConcrete, VectorKindConfiguration, build_create_generate_params,
};
use crate::verifier::provides::{MockIVerifierAdapterFailureMode, VerifierConcrete};

#[derive(Default)]
pub struct EncodingConsumerConstructorParamsOverrides {
    pub chain_forms: Option<AdmittedChainForms>,
    pub kem: Option<AdmittedKem>,
    pub key_agreement: Option<AdmittedKeyAgreement>,
    pub delivery_proof: Option<DeliveryProofConcrete>,
    pub generate: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
    pub verifier: Option<VerifierConcrete>,
    pub statement_version: Option<u16>,
    pub iterations: Option<NonZeroU32>,
}

pub fn build_encoding_consumer_constructor_params<'a, P: IPairingArithmetic + IPairingReference>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    statements: &'a AdmittedStatements,
    identities: &'a AdmittedIdentities,
    overrides: EncodingConsumerConstructorParamsOverrides,
) -> EncodingConsumerConstructorParams<'a, P> {
    EncodingConsumerConstructorParams {
        pairing,
        hash_to_scalar,
        random,
        statements,
        identities,
        chain_forms: overrides.chain_forms.unwrap_or_else(|| {
            let params = build_create_chain_forms_params(Default::default());
            AdmittedChainForms {
                concrete: params.concrete,
                identifier: params.identifier,
            }
        }),
        kem: overrides.kem.unwrap_or_else(|| {
            let params = build_create_kem_params(Default::default());
            AdmittedKem {
                concrete: params.concrete,
                identifier: params.identifier,
                scope: params.scope,
            }
        }),
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

pub fn build_encoding_consumer<'a, P: IPairingArithmetic + IPairingReference>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    statements: &'a AdmittedStatements,
    identities: &'a AdmittedIdentities,
    overrides: EncodingConsumerConstructorParamsOverrides,
) -> EncodingConsumer<'a, P> {
    let Ok(consumer) = EncodingConsumer::try_new(build_encoding_consumer_constructor_params(
        pairing,
        hash_to_scalar,
        random,
        statements,
        identities,
        overrides,
    ));
    consumer
}
