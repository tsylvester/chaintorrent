#![allow(dead_code, clippy::too_many_arguments)]

use chain::build_create_chain_forms_params;
use core::num::NonZeroU32;
use encoding::build_create_encoding_params;
use envelope::build_create_key_agreement_params;
use hash_to_scalar::build_create_hash_to_scalar_params;
use kem::build_create_kem_params;
use proof::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete, build_create_delivery_proof_params,
};
use random::IRandomSourceAdapter;

use super::interface::{PairingConsumer, PairingConsumerConstructorParams};
use crate::config::provides::{
    AdmittedChainForms, AdmittedEncoding, AdmittedHashToScalar, AdmittedIdentities, AdmittedKem,
    AdmittedKeyAgreement, AdmittedStatements,
};
use crate::generate::provides::{
    GenerateConcrete, VectorKindConfiguration, build_create_generate_params,
};
use crate::verifier::provides::{MockIVerifierAdapterFailureMode, VerifierConcrete};

#[derive(Default)]
pub struct PairingConsumerConstructorParamsOverrides {
    pub hash_to_scalar: Option<AdmittedHashToScalar>,
    pub encoding: Option<AdmittedEncoding>,
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

pub fn build_pairing_consumer_constructor_params<'a>(
    random: &'a dyn IRandomSourceAdapter,
    statements: &'a AdmittedStatements,
    identities: &'a AdmittedIdentities,
    overrides: PairingConsumerConstructorParamsOverrides,
) -> PairingConsumerConstructorParams<'a> {
    PairingConsumerConstructorParams {
        random,
        statements,
        identities,
        hash_to_scalar: overrides.hash_to_scalar.unwrap_or_else(|| {
            let params = build_create_hash_to_scalar_params(Default::default());
            AdmittedHashToScalar {
                concrete: params.concrete,
                identifier: params.identifier,
            }
        }),
        encoding: overrides.encoding.unwrap_or_else(|| {
            let params = build_create_encoding_params(Default::default());
            AdmittedEncoding {
                concrete: params.concrete,
                identifier: params.identifier,
            }
        }),
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

pub fn build_pairing_consumer<'a>(
    random: &'a dyn IRandomSourceAdapter,
    statements: &'a AdmittedStatements,
    identities: &'a AdmittedIdentities,
    overrides: PairingConsumerConstructorParamsOverrides,
) -> PairingConsumer<'a> {
    let Ok(consumer) = PairingConsumer::try_new(build_pairing_consumer_constructor_params(
        random, statements, identities, overrides,
    ));
    consumer
}
