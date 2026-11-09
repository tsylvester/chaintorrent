#![allow(dead_code, clippy::too_many_arguments)]

use chain::DECLARED_CHAIN_FORMS_SELECTIONS;
use core::fmt::Debug;
use encoding::DECLARED_ENCODING_SELECTIONS;
use envelope::DECLARED_KEY_AGREEMENT_SELECTIONS;
use hash_to_scalar::DECLARED_HASH_TO_SCALAR_SELECTIONS;
use kem::DECLARED_KEM_SELECTIONS;
use pairing::{PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, TARGET_GROUP_ENCODING_IDENTIFIERS};
use proof::DELIVERY_PROOF_CONCRETES;
use random::RANDOM_SOURCE_KINDS;

use super::super::layers::provides::{
    BenchmarkConcreteTableOverrides, CurveTableOverrides, SelectionsTable,
    SelectionsTableOverrides, build_benchmark_concrete_table, build_curve_table,
    build_selections_table,
};
use super::interface::{
    AdmitSelectionsDeps, AdmitSelectionsFn, AdmitSelectionsParams, AdmitSelectionsPayload,
    AdmitSelectionsReturn, AdmitSelectionsSuccessReturn, AdmittedChainForms, AdmittedCurve,
    AdmittedDeliveryProof, AdmittedEncoding, AdmittedHashToScalar, AdmittedKem,
    AdmittedKeyAgreement, AdmittedPairing, AdmittedSelections,
};
use crate::verifier::provides::VERIFIER_CONCRETES;

pub fn declared_name<T: Debug>(value: &T) -> String {
    format!("{:?}", value)
}

#[derive(Default)]
pub struct AdmittedEncodingOverrides {
    pub concrete: Option<encoding::EncodingConcrete>,
    pub identifier: Option<encoding::EncodingIdentifier>,
}

pub fn build_admitted_encoding(overrides: AdmittedEncodingOverrides) -> AdmittedEncoding {
    AdmittedEncoding {
        concrete: overrides
            .concrete
            .unwrap_or(DECLARED_ENCODING_SELECTIONS[0].concrete),
        identifier: overrides
            .identifier
            .unwrap_or(DECLARED_ENCODING_SELECTIONS[0].identifier),
    }
}

#[derive(Default)]
pub struct AdmittedHashToScalarOverrides {
    pub concrete: Option<hash_to_scalar::HashToScalarConcrete>,
    pub identifier: Option<hash_to_scalar::HashToScalarIdentifier>,
}

pub fn build_admitted_hash_to_scalar(
    overrides: AdmittedHashToScalarOverrides,
) -> AdmittedHashToScalar {
    AdmittedHashToScalar {
        concrete: overrides
            .concrete
            .unwrap_or(DECLARED_HASH_TO_SCALAR_SELECTIONS[0].concrete),
        identifier: overrides
            .identifier
            .unwrap_or(DECLARED_HASH_TO_SCALAR_SELECTIONS[0].identifier),
    }
}

#[derive(Default)]
pub struct AdmittedChainFormsOverrides {
    pub concrete: Option<chain::ChainFormsConcrete>,
    pub identifier: Option<chain::ChainFormsIdentifier>,
}

pub fn build_admitted_chain_forms(overrides: AdmittedChainFormsOverrides) -> AdmittedChainForms {
    AdmittedChainForms {
        concrete: overrides
            .concrete
            .unwrap_or(DECLARED_CHAIN_FORMS_SELECTIONS[0].concrete),
        identifier: overrides
            .identifier
            .unwrap_or(DECLARED_CHAIN_FORMS_SELECTIONS[0].identifier),
    }
}

#[derive(Default)]
pub struct AdmittedKemOverrides {
    pub concrete: Option<kem::KemConcrete>,
    pub identifier: Option<kem::KemIdentifier>,
    pub scope: Option<kem::IdentityScope>,
}

pub fn build_admitted_kem(overrides: AdmittedKemOverrides) -> AdmittedKem {
    AdmittedKem {
        concrete: overrides
            .concrete
            .unwrap_or(DECLARED_KEM_SELECTIONS[0].concrete),
        identifier: overrides
            .identifier
            .unwrap_or(DECLARED_KEM_SELECTIONS[0].identifier),
        scope: overrides.scope.unwrap_or(DECLARED_KEM_SELECTIONS[0].scope),
    }
}

#[derive(Default)]
pub struct AdmittedKeyAgreementOverrides {
    pub concrete: Option<envelope::KeyAgreementConcrete>,
    pub identifier: Option<envelope::KeyAgreementIdentifier>,
    pub algebra: Option<envelope::EnvelopeAlgebra>,
}

pub fn build_admitted_key_agreement(
    overrides: AdmittedKeyAgreementOverrides,
) -> AdmittedKeyAgreement {
    AdmittedKeyAgreement {
        concrete: overrides
            .concrete
            .unwrap_or(DECLARED_KEY_AGREEMENT_SELECTIONS[0].concrete),
        identifier: overrides
            .identifier
            .unwrap_or(DECLARED_KEY_AGREEMENT_SELECTIONS[0].identifier),
        algebra: overrides
            .algebra
            .unwrap_or(DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra),
    }
}

#[derive(Default)]
pub struct AdmittedDeliveryProofOverrides {
    pub concrete: Option<proof::DeliveryProofConcrete>,
    pub algebra: Option<envelope::EnvelopeAlgebra>,
    pub statement_version: Option<u16>,
}

pub fn build_admitted_delivery_proof(
    overrides: AdmittedDeliveryProofOverrides,
) -> AdmittedDeliveryProof {
    AdmittedDeliveryProof {
        concrete: overrides.concrete.unwrap_or(DELIVERY_PROOF_CONCRETES[0]),
        algebra: overrides
            .algebra
            .unwrap_or(DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra),
        statement_version: overrides.statement_version.unwrap_or(1),
    }
}

#[derive(Default)]
pub struct AdmittedPairingOverrides {
    pub concrete: Option<pairing::PairingConcrete>,
    pub target_group_encoding: Option<pairing::TargetGroupEncodingIdentifier>,
}

pub fn build_admitted_pairing(overrides: AdmittedPairingOverrides) -> AdmittedPairing {
    AdmittedPairing {
        concrete: overrides.concrete.unwrap_or(PAIRING_CONCRETES[0]),
        target_group_encoding: overrides
            .target_group_encoding
            .unwrap_or(TARGET_GROUP_ENCODING_IDENTIFIERS[0]),
    }
}

#[derive(Default)]
pub struct AdmittedCurveOverrides {
    pub label: Option<String>,
    pub pairing: Option<AdmittedPairing>,
}

pub fn build_admitted_curve(overrides: AdmittedCurveOverrides) -> AdmittedCurve {
    AdmittedCurve {
        label: overrides.label.unwrap_or_else(|| "curve-a".to_string()),
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_admitted_pairing(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmittedSelectionsOverrides {
    pub random_source: Option<random::RandomSourceKind>,
    pub encoding: Option<AdmittedEncoding>,
    pub hash_to_scalar: Option<AdmittedHashToScalar>,
    pub chain_forms: Option<AdmittedChainForms>,
    pub kem: Option<AdmittedKem>,
    pub key_agreement: Option<AdmittedKeyAgreement>,
    pub delivery_proof: Option<AdmittedDeliveryProof>,
    pub verifier: Option<crate::verifier::provides::VerifierConcreteKind>,
    pub curves: Option<Vec<AdmittedCurve>>,
    pub benchmark_concretes: Option<Vec<AdmittedPairing>>,
    pub precompile_encodings: Option<Vec<pairing::PrecompileEncoding>>,
}

pub fn build_admitted_selections(overrides: AdmittedSelectionsOverrides) -> AdmittedSelections {
    AdmittedSelections {
        random_source: overrides.random_source.unwrap_or(RANDOM_SOURCE_KINDS[0]),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_admitted_encoding(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_admitted_hash_to_scalar(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_admitted_chain_forms(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_admitted_kem(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_admitted_key_agreement(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_admitted_delivery_proof(Default::default())),
        verifier: overrides.verifier.unwrap_or(VERIFIER_CONCRETES[0]),
        curves: overrides.curves.unwrap_or_else(|| {
            vec![
                build_admitted_curve(Default::default()),
                build_admitted_curve(AdmittedCurveOverrides {
                    label: Some("curve-b".to_string()),
                    ..Default::default()
                }),
            ]
        }),
        benchmark_concretes: overrides
            .benchmark_concretes
            .unwrap_or_else(|| vec![build_admitted_pairing(Default::default())]),
        precompile_encodings: overrides
            .precompile_encodings
            .unwrap_or_else(|| vec![PRECOMPILE_ENCODINGS[0]]),
    }
}

#[derive(Default)]
pub struct AdmitSelectionsDepsOverrides;

pub fn build_admit_selections_deps(
    _overrides: AdmitSelectionsDepsOverrides,
) -> AdmitSelectionsDeps {
    AdmitSelectionsDeps
}

#[derive(Default)]
pub struct AdmitSelectionsParamsOverrides;

pub fn build_admit_selections_params(
    _overrides: AdmitSelectionsParamsOverrides,
) -> AdmitSelectionsParams {
    AdmitSelectionsParams
}

#[derive(Default)]
pub struct AdmitSelectionsPayloadOverrides {
    pub selections: Option<SelectionsTable>,
    pub precompile_encodings: Option<Vec<String>>,
}

pub fn build_admit_selections_payload(
    overrides: AdmitSelectionsPayloadOverrides,
) -> AdmitSelectionsPayload {
    AdmitSelectionsPayload {
        selections: overrides.selections.unwrap_or_else(|| {
            build_selections_table(SelectionsTableOverrides {
                random_source: Some(declared_name(&RANDOM_SOURCE_KINDS[0])),
                encoding: Some(declared_name(&DECLARED_ENCODING_SELECTIONS[0].concrete)),
                hash_to_scalar: Some(declared_name(
                    &DECLARED_HASH_TO_SCALAR_SELECTIONS[0].concrete,
                )),
                chain_forms: Some(declared_name(&DECLARED_CHAIN_FORMS_SELECTIONS[0].concrete)),
                kem: Some(declared_name(&DECLARED_KEM_SELECTIONS[0].concrete)),
                key_agreement: Some(declared_name(
                    &DECLARED_KEY_AGREEMENT_SELECTIONS[0].concrete,
                )),
                delivery_proof: Some(declared_name(&DELIVERY_PROOF_CONCRETES[0])),
                verifier: Some(declared_name(&VERIFIER_CONCRETES[0])),
                identity_scope: Some(declared_name(&DECLARED_KEM_SELECTIONS[0].scope)),
                envelope_algebra: Some(declared_name(
                    &DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
                )),
                delivery_statement_version: Some(1),
                curves: Some(vec![
                    build_curve_table(CurveTableOverrides {
                        label: Some("curve-a".to_string()),
                        pairing_concrete: Some(declared_name(&PAIRING_CONCRETES[0])),
                        target_group_encoding: Some(declared_name(
                            &TARGET_GROUP_ENCODING_IDENTIFIERS[0],
                        )),
                    }),
                    build_curve_table(CurveTableOverrides {
                        label: Some("curve-b".to_string()),
                        pairing_concrete: Some(declared_name(&PAIRING_CONCRETES[0])),
                        target_group_encoding: Some(declared_name(
                            &TARGET_GROUP_ENCODING_IDENTIFIERS[0],
                        )),
                    }),
                ]),
                benchmark_concretes: Some(vec![build_benchmark_concrete_table(
                    BenchmarkConcreteTableOverrides {
                        pairing_concrete: Some(declared_name(&PAIRING_CONCRETES[0])),
                        target_group_encoding: Some(declared_name(
                            &TARGET_GROUP_ENCODING_IDENTIFIERS[0],
                        )),
                    },
                )]),
            })
        }),
        precompile_encodings: overrides
            .precompile_encodings
            .unwrap_or_else(|| vec![declared_name(&PRECOMPILE_ENCODINGS[0])]),
    }
}

#[derive(Default)]
pub struct AdmitSelectionsSuccessReturnOverrides {
    pub admitted: Option<AdmittedSelections>,
}

pub fn build_admit_selections_success_return(
    overrides: AdmitSelectionsSuccessReturnOverrides,
) -> AdmitSelectionsSuccessReturn {
    AdmitSelectionsSuccessReturn {
        admitted: overrides
            .admitted
            .unwrap_or_else(|| build_admitted_selections(Default::default())),
    }
}

pub fn mock_admit_selections(
    _deps: &AdmitSelectionsDeps,
    _params: AdmitSelectionsParams,
    _payload: AdmitSelectionsPayload,
) -> AdmitSelectionsReturn {
    let _: AdmitSelectionsFn = mock_admit_selections;
    Ok(build_admit_selections_success_return(Default::default()))
}
