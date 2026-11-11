#![allow(dead_code, clippy::too_many_arguments)]

use chain::build_create_chain_forms_params;
use encoding::build_create_encoding_params;
use envelope::build_create_key_agreement_params;
use hash_to_scalar::build_create_hash_to_scalar_params;
use kem::build_create_kem_params;
use pairing::build_create_pairing_params;
use proof::{DELIVERY_STATEMENT_VERSION_ONE, build_create_delivery_proof_params};
use random::build_create_random_source_params;

use super::interface::{
    RunCurveOutcome, RunDeps, RunFn, RunParams, RunPayload, RunReturn, RunSuccessReturn,
};
use crate::benchmark::provides::{
    PairingBenchmarkSuccessReturn, build_pairing_benchmark_success_return,
};
use crate::config::provides::{
    AdmittedChainForms, AdmittedCurveOverrides, AdmittedDeliveryProof, AdmittedEncoding,
    AdmittedGenerateOverrides, AdmittedHashToScalar, AdmittedKem, AdmittedKeyAgreement,
    AdmittedPairing, AdmittedSelectionsOverrides, HarnessConfiguration,
    HarnessConfigurationOverrides, build_admitted_curve, build_admitted_generate,
    build_admitted_identities, build_admitted_selections, build_admitted_statements,
    build_harness_configuration,
};
use crate::generate::provides::{EmittedFile, build_create_generate_params, build_emitted_file};
use crate::measure::provides::{DeliveryMeasurement, build_delivery_measurement};
use crate::report::provides::{
    ReportSuccessReturn, ReportText, ReportTextConstructorParamsOverrides,
    build_report_success_return, build_report_text,
};
use crate::verifier::provides::{MockIVerifierAdapterFailureMode, VerifierConcrete};

#[derive(Default)]
pub struct RunDepsOverrides;

pub fn build_run_deps(_overrides: RunDepsOverrides) -> RunDeps {
    RunDeps
}

#[derive(Default)]
pub struct RunParamsOverrides;

pub fn build_run_params(_overrides: RunParamsOverrides) -> RunParams {
    RunParams
}

#[derive(Default)]
pub struct RunPayloadOverrides {
    pub configuration: Option<HarnessConfiguration>,
}

pub fn build_run_payload(overrides: RunPayloadOverrides) -> RunPayload {
    RunPayload {
        configuration: overrides.configuration.unwrap_or_else(|| {
            let random_source = build_create_random_source_params(Default::default());
            let hash_to_scalar = build_create_hash_to_scalar_params(Default::default());
            let encoding = build_create_encoding_params(Default::default());
            let chain_forms = build_create_chain_forms_params(Default::default());
            let kem = build_create_kem_params(Default::default());
            let key_agreement = build_create_key_agreement_params(Default::default());
            let delivery_proof = build_create_delivery_proof_params(Default::default());
            let pairing = build_create_pairing_params(Default::default());
            let generate = build_create_generate_params(Default::default());
            let pairing_selection = AdmittedPairing {
                concrete: pairing.concrete,
                target_group_encoding: pairing.target_group_encoding,
            };
            let curves = vec![
                build_admitted_curve(AdmittedCurveOverrides {
                    pairing: Some(pairing_selection),
                    ..Default::default()
                }),
                build_admitted_curve(AdmittedCurveOverrides {
                    label: Some("curve-b".to_string()),
                    pairing: Some(pairing_selection),
                }),
            ];
            build_harness_configuration(HarnessConfigurationOverrides {
                selections: Some(build_admitted_selections(AdmittedSelectionsOverrides {
                    random_source: Some(random_source.kind),
                    encoding: Some(AdmittedEncoding {
                        concrete: encoding.concrete,
                        identifier: encoding.identifier,
                    }),
                    hash_to_scalar: Some(AdmittedHashToScalar {
                        concrete: hash_to_scalar.concrete,
                        identifier: hash_to_scalar.identifier,
                    }),
                    chain_forms: Some(AdmittedChainForms {
                        concrete: chain_forms.concrete,
                        identifier: chain_forms.identifier,
                    }),
                    kem: Some(AdmittedKem {
                        concrete: kem.concrete,
                        identifier: kem.identifier,
                        scope: kem.scope,
                    }),
                    key_agreement: Some(AdmittedKeyAgreement {
                        concrete: key_agreement.concrete,
                        identifier: key_agreement.identifier,
                        algebra: key_agreement.algebra,
                    }),
                    delivery_proof: Some(AdmittedDeliveryProof {
                        concrete: delivery_proof.concrete,
                        algebra: key_agreement.algebra,
                        statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                    }),
                    curves: Some(curves.clone()),
                    benchmark_concretes: Some(vec![pairing_selection]),
                    precompile_encodings: Some(pairing.supported_encodings),
                    ..Default::default()
                })),
                verifiers: Some(
                    curves
                        .iter()
                        .map(|_| VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure))
                        .collect(),
                ),
                curve_labels: Some(
                    curves
                        .iter()
                        .map(|curve| {
                            build_report_text(ReportTextConstructorParamsOverrides {
                                text: Some(curve.label.clone()),
                            })
                        })
                        .collect(),
                ),
                identities: Some(build_admitted_identities(Default::default())),
                statements: Some(build_admitted_statements(Default::default())),
                generate: Some(build_admitted_generate(AdmittedGenerateOverrides {
                    concrete: Some(generate.concrete),
                    kinds: Some(generate.kinds),
                })),
                ..Default::default()
            })
        }),
    }
}

#[derive(Default)]
pub struct RunCurveOutcomeOverrides {
    pub label: Option<ReportText>,
    pub files: Option<Vec<EmittedFile>>,
    pub measurement: Option<DeliveryMeasurement>,
    pub report: Option<ReportSuccessReturn>,
}

pub fn build_run_curve_outcome(overrides: RunCurveOutcomeOverrides) -> RunCurveOutcome {
    RunCurveOutcome {
        label: overrides
            .label
            .unwrap_or_else(|| build_report_text(Default::default())),
        files: overrides
            .files
            .unwrap_or_else(|| vec![build_emitted_file(Default::default())]),
        measurement: overrides
            .measurement
            .unwrap_or_else(|| build_delivery_measurement(Default::default())),
        report: overrides
            .report
            .unwrap_or_else(|| build_report_success_return(Default::default())),
    }
}

#[derive(Default)]
pub struct RunSuccessReturnOverrides {
    pub benchmarks: Option<Vec<PairingBenchmarkSuccessReturn>>,
    pub curves: Option<Vec<RunCurveOutcome>>,
}

pub fn build_run_success_return(overrides: RunSuccessReturnOverrides) -> RunSuccessReturn {
    RunSuccessReturn {
        benchmarks: overrides
            .benchmarks
            .unwrap_or_else(|| vec![build_pairing_benchmark_success_return(Default::default())]),
        curves: overrides
            .curves
            .unwrap_or_else(|| vec![build_run_curve_outcome(Default::default())]),
    }
}

pub fn mock_run(_deps: &RunDeps, _params: RunParams, _payload: RunPayload) -> RunReturn {
    let _: RunFn = mock_run;
    Ok(build_run_success_return(Default::default()))
}
