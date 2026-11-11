#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Private-surface enumeration:
//   `a_successful_runs_outcome_is_written_with_every_value_it_holds`
//       proven by the block of the same name.
//   `a_refused_runs_outcome_is_written_as_its_refusal`
//       proven by the block of the same name.

use core::num::NonZeroU32;
use pairing::{
    MockIPairingAdapterFailureMode, PAIRING_CONCRETES, PairingConcrete,
    TargetGroupEncodingIdentifier,
};
use random::RandomSourceKind;
use serde_json::{Value, json};
use std::env::temp_dir;
use std::path::{Path, PathBuf};

use super::mock::{
    OutputParamsOverrides, OutputPayloadOverrides, build_output_deps, build_output_params,
    build_output_payload,
};
use super::output;
use crate::config::provides::{
    AdmitOutputPayloadOverrides, AdmittedCurve, AdmittedGenerate, AdmittedOutput, AdmittedPairing,
    AdmittedPairingOverrides, AdmittedSelectionsOverrides, DESTINATION_KINDS, DestinationKind,
    DestinationTableOverrides, HarnessConfiguration, HarnessConfigurationOverrides,
    OutputDestination, OutputTableOverrides, admit_output, build_admit_output_deps,
    build_admit_output_params, build_admit_output_payload, build_admitted_pairing,
    build_admitted_selections, build_destination_table, build_harness_configuration,
    build_output_table, declared_name,
};
use crate::generate::provides::{
    GenerateConcrete, GenerateConcreteKind, SolidityGenerateSettingsOverrides,
    build_generate_kind_configurations, build_solidity_generate_settings,
};
use crate::report::provides::{
    ReportDestination, ReportTextConstructorParamsOverrides, build_report_text,
};
use crate::run::provides::{RunDeps, RunParams, RunPayload, run};
use crate::verifier::provides::{L1DataFee, MockIVerifierAdapterFailureMode, VerifierConcrete};

/// The real members of `PAIRING_CONCRETES`, in declared order.
fn real_pairing_concretes() -> Vec<PairingConcrete> {
    PAIRING_CONCRETES
        .iter()
        .copied()
        .filter(|concrete| {
            !matches!(
                concrete,
                PairingConcrete::Mock(_) | PairingConcrete::MockShortUniform(_)
            )
        })
        .collect()
}

/// An admitted pairing whose target-group encoding follows its concrete.
fn admitted_pairing_of(concrete: PairingConcrete) -> AdmittedPairing {
    build_admitted_pairing(AdmittedPairingOverrides {
        concrete: Some(concrete),
        target_group_encoding: Some(match concrete {
            PairingConcrete::Bn254Arkworks | PairingConcrete::Bn254Halo2curves => {
                TargetGroupEncodingIdentifier::Bn254V1
            }
            PairingConcrete::Bls12381Arkworks | PairingConcrete::Bls12381Halo2curves => {
                TargetGroupEncodingIdentifier::Bls12381V1
            }
            PairingConcrete::Mock(_) | PairingConcrete::MockShortUniform(_) => {
                TargetGroupEncodingIdentifier::Mock
            }
        }),
    })
}

/// A curve of `label` over the admitted pairing of `concrete`.
fn curve_of(label: &str, concrete: PairingConcrete) -> AdmittedCurve {
    AdmittedCurve {
        label: label.to_string(),
        pairing: admitted_pairing_of(concrete),
    }
}

/// A directory beneath the operating system's temporary directory named
/// `chaintorrent-output-`, the process id, `-`, and the current thread's
/// id as its `Debug` text, removed before the act and created fresh.
fn work_directory() -> PathBuf {
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the work directory is created");
    directory
}

/// The configuration the private surface's cases share, built as
/// `harness-crypto/run`'s `integration_test.rs` builds it for the own
/// entry `the_run_returns_a_measurement_per_benchmark_concrete_and_an_
/// outcome_per_curve_in_configuration_order`: every family's first
/// declared real member, the given benchmark concretes and curves, two
/// `Mock(NoFailure)` verifiers, the generate concrete rooted at the work
/// directory, and one file report destination beneath it.
fn configuration(
    directory: &Path,
    benchmark_concretes: Vec<AdmittedPairing>,
    curves: Vec<AdmittedCurve>,
    overrides: HarnessConfigurationOverrides,
) -> HarnessConfiguration {
    build_harness_configuration(HarnessConfigurationOverrides {
        selections: Some(overrides.selections.unwrap_or_else(|| {
            build_admitted_selections(AdmittedSelectionsOverrides {
                random_source: Some(RandomSourceKind::OperatingSystem),
                curves: Some(curves),
                benchmark_concretes: Some(benchmark_concretes),
                precompile_encodings: Some(pairing::PRECOMPILE_ENCODINGS.to_vec()),
                ..Default::default()
            })
        })),
        verifiers: overrides.verifiers.or(Some(vec![
            VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
            VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
        ])),
        curve_labels: overrides.curve_labels.or(Some(vec![
            build_report_text(ReportTextConstructorParamsOverrides {
                text: Some("curve-a".to_string()),
            }),
            build_report_text(ReportTextConstructorParamsOverrides {
                text: Some("curve-b".to_string()),
            }),
        ])),
        generate: overrides.generate.or(Some(AdmittedGenerate {
            concrete: GenerateConcrete::Solidity {
                settings: Box::new(build_solidity_generate_settings(
                    SolidityGenerateSettingsOverrides {
                        foundry_root: Some(directory.to_path_buf()),
                        ..Default::default()
                    },
                )),
            },
            kinds: build_generate_kind_configurations(
                GenerateConcreteKind::Solidity,
                Default::default(),
            ),
        })),
        output: overrides.output.or(Some(AdmittedOutput {
            destinations: vec![OutputDestination::StandardOutput],
            report_destinations: vec![ReportDestination::File {
                path: directory.join("report.txt"),
            }],
        })),
        benchmark_iterations: overrides.benchmark_iterations,
        measurement: overrides.measurement,
        report: overrides.report,
        ..overrides
    })
}

/// `count` file destinations beneath `directory`, admitted by the real
/// `admit_output` over a payload whose `output` is `build_output_table`
/// over entries of the kind `declared_name` of the file member of
/// `DESTINATION_KINDS` and the paths.
fn admitted_destinations(directory: &Path, count: usize) -> Vec<OutputDestination> {
    let file_kind = DESTINATION_KINDS
        .iter()
        .find(|kind| matches!(kind, DestinationKind::File))
        .expect("DESTINATION_KINDS holds the file member");
    admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        build_admit_output_payload(AdmitOutputPayloadOverrides {
            output: Some(build_output_table(OutputTableOverrides {
                destinations: Some(
                    (0..count)
                        .map(|index| {
                            build_destination_table(DestinationTableOverrides {
                                kind: Some(declared_name(file_kind)),
                                path: Some(Some(directory.join(format!("outcome-{index}.json")))),
                            })
                        })
                        .collect(),
                ),
            })),
            ..Default::default()
        }),
    )
    .expect("the destinations admit")
    .admitted
    .destinations
}

/// The `json!` value the measurement's `l1_data_fee` renders to.
fn fee_json(fee: L1DataFee) -> Value {
    match fee {
        L1DataFee::Charged { wei } => json!({ "state": "charged", "wei": wei.to_string() }),
        L1DataFee::NotApplicable => json!({ "state": "not_applicable" }),
    }
}

/// Contract: a successful run's outcome is written with every value it
///   holds — the `benchmarks` array holds an object per configured
///   concrete in configuration order and `curves` an object per curve in
///   curve order, each holding its own label, files, measurement, and
///   report document, and the same text reaches every admitted
///   destination.
/// Arrange: the configuration the run entry arranges — the real members
///   of `PAIRING_CONCRETES` as `benchmark_concretes` in a configured order
///   and in the reverse order, two curves of distinct labels and pairing
///   concretes — and file destinations beneath the work directory, one
///   and two.
/// Act:     `run` through the crate's functions, then `output`, once per
///   case.
/// Assert:  the result is `Ok`, its `text` parsed equals the `json!`
///   literal built in Arrange from the outcome's values field by field,
///   and each file read back equals the returned text.
/// Boundary: the real `admit_output`, `run`, `render_outcome`, and
///   `output`.
/// Mocked:  the verifier family's mock concrete under `NoFailure`.
#[test]
fn a_successful_runs_outcome_is_written_with_every_value_it_holds() {
    for (name, concretes, destination_count) in [
        (
            "declared order, one destination",
            real_pairing_concretes(),
            1,
        ),
        (
            "declared order, two destinations",
            real_pairing_concretes(),
            2,
        ),
        (
            "reverse order, one destination",
            real_pairing_concretes().into_iter().rev().collect(),
            1,
        ),
        (
            "reverse order, two destinations",
            real_pairing_concretes().into_iter().rev().collect(),
            2,
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let real = real_pairing_concretes();
        let destinations = admitted_destinations(&directory, destination_count);
        let outcome = run(
            &RunDeps,
            RunParams,
            RunPayload {
                configuration: configuration(
                    &directory,
                    concretes
                        .iter()
                        .map(|concrete| admitted_pairing_of(*concrete))
                        .collect(),
                    vec![curve_of("curve-a", real[0]), curve_of("curve-b", real[1])],
                    HarnessConfigurationOverrides {
                        benchmark_iterations: NonZeroU32::new(3),
                        ..Default::default()
                    },
                ),
            },
        );
        let success = outcome
            .as_ref()
            .unwrap_or_else(|_| panic!("{name}: the run over real concretes succeeds"));
        let expected = json!({
            "outcome": "succeeded",
            "benchmarks": success
                .benchmarks
                .iter()
                .map(|benchmark| {
                    let timings = &benchmark.timings;
                    json!({
                        "concrete": format!("{:?}", benchmark.concrete),
                        "iterations": benchmark.iterations.get(),
                        "timings": {
                            "mul_g1": { "seconds": timings.mul_g1.as_secs(), "subsec_nanos": timings.mul_g1.subsec_nanos() },
                            "mul_g2": { "seconds": timings.mul_g2.as_secs(), "subsec_nanos": timings.mul_g2.subsec_nanos() },
                            "msm_g1": { "seconds": timings.msm_g1.as_secs(), "subsec_nanos": timings.msm_g1.subsec_nanos() },
                            "msm_g2": { "seconds": timings.msm_g2.as_secs(), "subsec_nanos": timings.msm_g2.subsec_nanos() },
                            "pairing_product": { "seconds": timings.pairing_product.as_secs(), "subsec_nanos": timings.pairing_product.subsec_nanos() }
                        }
                    })
                })
                .collect::<Vec<Value>>(),
            "curves": success
                .curves
                .iter()
                .map(|curve| {
                    let measurement = &curve.measurement;
                    let timings = &measurement.timings;
                    let sizes = &measurement.sizes;
                    let costs = &measurement.costs;
                    json!({
                        "label": curve.label.as_str(),
                        "files": curve
                            .files
                            .iter()
                            .map(|file| {
                                json!({
                                    "kind": match file.kind {
                                        Some(kind) => Value::from(format!("{kind:?}")),
                                        None => Value::Null,
                                    },
                                    "path": file.path.to_string_lossy().into_owned()
                                })
                            })
                            .collect::<Vec<Value>>(),
                        "measurement": {
                            "iterations": measurement.iterations.get(),
                            "timings": {
                                "prove_mint": { "seconds": timings.prove_mint.as_secs(), "subsec_nanos": timings.prove_mint.subsec_nanos() },
                                "prove_transfer": { "seconds": timings.prove_transfer.as_secs(), "subsec_nanos": timings.prove_transfer.subsec_nanos() },
                                "verify_mint": { "seconds": timings.verify_mint.as_secs(), "subsec_nanos": timings.verify_mint.subsec_nanos() },
                                "verify_transfer": { "seconds": timings.verify_transfer.as_secs(), "subsec_nanos": timings.verify_transfer.subsec_nanos() },
                                "decapsulate": { "seconds": timings.decapsulate.as_secs(), "subsec_nanos": timings.decapsulate.subsec_nanos() }
                            },
                            "sizes_bytes": {
                                "capsule": sizes.capsule,
                                "envelope": sizes.envelope,
                                "mint_proof": sizes.mint_proof,
                                "transfer_proof": sizes.transfer_proof
                            },
                            "costs": {
                                "mint": {
                                    "l2_execution_gas": costs.mint.l2_execution_gas,
                                    "l1_data_fee": fee_json(costs.mint.l1_data_fee)
                                },
                                "transfer": {
                                    "l2_execution_gas": costs.transfer.l2_execution_gas,
                                    "l1_data_fee": fee_json(costs.transfer.l1_data_fee)
                                }
                            }
                        },
                        "report": curve.report.document.as_str()
                    })
                })
                .collect::<Vec<Value>>()
        });

        // Act
        let returned = output(
            &build_output_deps(Default::default()),
            build_output_params(OutputParamsOverrides {
                destinations: Some(destinations.clone()),
            }),
            build_output_payload(OutputPayloadOverrides {
                outcome: Some(outcome),
            }),
        )
        .unwrap_or_else(|_| panic!("{name}: the writes succeed"));

        // Assert
        let parsed: Value = serde_json::from_str(&returned.text)
            .unwrap_or_else(|_| panic!("{name}: the text parses"));
        assert_eq!(parsed, expected, "{name}");
        for destination in &destinations {
            match destination {
                OutputDestination::File { path } => {
                    assert_eq!(
                        std::fs::read_to_string(path).unwrap_or_else(|_| {
                            panic!("{name}: the file {} reads", path.display())
                        }),
                        returned.text,
                        "{name}"
                    );
                }
                OutputDestination::StandardOutput => {
                    panic!("{name}: a file destination was expected")
                }
            }
        }
        // a_successful_runs_outcome_is_written_with_every_value_it_holds
    }
}

/// Contract: a refused run's outcome is written as its refusal — the
///   document is `{ "outcome": "refused", "refusal": <Debug text> }` and
///   the same text reaches every admitted destination.
/// Arrange: the configuration whose benchmark concrete is
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode
///   ::SampleScalarWrongLength)`, the `Debug` text of the refusal the run
///   returned taken before the outcome is handed, and file destinations
///   beneath the work directory, one and two.
/// Act:     `run` through the crate's functions, then `output`, once per
///   case.
/// Assert:  the result is `Ok`, its `text` parsed equals
///   `json!({ "outcome": "refused", "refusal": <that text> })`, and each
///   file read back equals the returned text.
/// Boundary: the real `admit_output`, `run`, `render_outcome`, and
///   `output`.
/// Mocked:  the pairing family's mock concrete.
#[test]
fn a_refused_runs_outcome_is_written_as_its_refusal() {
    for (name, destination_count) in [("one destination", 1), ("two destinations", 2)] {
        // Arrange
        let directory = work_directory();
        let real = real_pairing_concretes();
        let destinations = admitted_destinations(&directory, destination_count);
        let outcome = run(
            &RunDeps,
            RunParams,
            RunPayload {
                configuration: configuration(
                    &directory,
                    vec![admitted_pairing_of(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    ))],
                    vec![curve_of("curve-a", real[0]), curve_of("curve-b", real[1])],
                    HarnessConfigurationOverrides::default(),
                ),
            },
        );
        let refusal = format!(
            "{:?}",
            outcome
                .as_ref()
                .err()
                .unwrap_or_else(|| panic!("{name}: the run refuses"))
        );

        // Act
        let returned = output(
            &build_output_deps(Default::default()),
            build_output_params(OutputParamsOverrides {
                destinations: Some(destinations.clone()),
            }),
            build_output_payload(OutputPayloadOverrides {
                outcome: Some(outcome),
            }),
        )
        .unwrap_or_else(|_| panic!("{name}: the writes succeed"));

        // Assert
        let parsed: Value = serde_json::from_str(&returned.text)
            .unwrap_or_else(|_| panic!("{name}: the text parses"));
        assert_eq!(
            parsed,
            json!({ "outcome": "refused", "refusal": refusal }),
            "{name}"
        );
        for destination in &destinations {
            match destination {
                OutputDestination::File { path } => {
                    assert_eq!(
                        std::fs::read_to_string(path).unwrap_or_else(|_| {
                            panic!("{name}: the file {} reads", path.display())
                        }),
                        returned.text,
                        "{name}"
                    );
                }
                OutputDestination::StandardOutput => {
                    panic!("{name}: a file destination was expected")
                }
            }
        }
        // a_refused_runs_outcome_is_written_as_its_refusal
    }
}
