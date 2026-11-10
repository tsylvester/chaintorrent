#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Public-surface enumeration:
//   Own entries —
//     a_successful_runs_outcome_is_written_with_every_value_it_holds
//     a_refused_runs_outcome_is_written_as_its_refusal
//   Carried, `harness-crypto/run` —
//     the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order
//     each_curve_is_reported_under_its_own_label_with_the_configured_defaults
//     the_first_refusal_is_returned_in_the_stated_order
//   Carried, `harness-crypto/config/admit_output` —
//     every_declared_destination_kind_is_admitted_to_its_own_variant
//     a_destination_list_keeps_the_order_the_document_holds

use core::num::NonZeroU32;
use harness_crypto::{
    AdmitOutputPayloadOverrides, AdmittedCurve, AdmittedGenerate, AdmittedOutput, AdmittedPairing,
    AdmittedPairingOverrides, AdmittedSelectionsOverrides, DESTINATION_KINDS, DestinationKind,
    DestinationTableOverrides, GenerateConcrete, GenerateConcreteKind, HarnessConfiguration,
    HarnessConfigurationOverrides, L1DataFee, MeasurementSettings, MockIVerifierAdapterFailureMode,
    OutputDestination, OutputErrorReturn, OutputParamsOverrides, OutputPayloadOverrides,
    OutputTableOverrides, ReportAdjustment, ReportDefaults, ReportDestination, ReportSettings,
    ReportTextConstructorParamsOverrides, RunDeps, RunParams, RunPayload, RunSuccessReturn,
    SolidityGenerateSettingsOverrides, VerifierConcrete, admit_output, build_admit_output_deps,
    build_admit_output_params, build_admit_output_payload, build_admitted_pairing,
    build_admitted_selections, build_destination_table, build_generate_kind_configurations,
    build_harness_configuration, build_output_deps, build_output_params, build_output_payload,
    build_output_table, build_report_text, build_solidity_generate_settings, declared_name, output,
    run,
};
use pairing::{
    MockIPairingAdapterFailureMode, PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, PairingConcrete,
    TargetGroupEncodingIdentifier,
};
use serde_json::{Value, json};
use std::env::temp_dir;
use std::path::{Path, PathBuf};

/// The real members of `PAIRING_CONCRETES`, in declared order.
fn real_pairing_concretes() -> Vec<PairingConcrete> {
    PAIRING_CONCRETES
        .iter()
        .copied()
        .filter(|concrete| !matches!(concrete, PairingConcrete::Mock(_)))
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
            PairingConcrete::Mock(_) => TargetGroupEncodingIdentifier::Mock,
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

/// The configuration the public surface's cases share: every family's
/// first declared real member, two `Mock(NoFailure)` verifiers, the real
/// generate concrete rooted at the work directory, and one file report
/// destination beneath it.
fn configuration(
    directory: &Path,
    benchmark_concretes: Vec<AdmittedPairing>,
    curves: Vec<AdmittedCurve>,
    overrides: HarnessConfigurationOverrides,
) -> HarnessConfiguration {
    build_harness_configuration(HarnessConfigurationOverrides {
        selections: Some(overrides.selections.unwrap_or_else(|| {
            build_admitted_selections(AdmittedSelectionsOverrides {
                random_source: Some(random::RandomSourceKind::OperatingSystem),
                curves: Some(curves),
                benchmark_concretes: Some(benchmark_concretes),
                precompile_encodings: Some(PRECOMPILE_ENCODINGS.to_vec()),
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

/// The destinations `entries` declare — a `DestinationKind` and an
/// optional path each — admitted by the public `admit_output` over a
/// payload whose `output` is `build_output_table`.
fn admit_output_entries(entries: &[(DestinationKind, Option<PathBuf>)]) -> Vec<OutputDestination> {
    admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        build_admit_output_payload(AdmitOutputPayloadOverrides {
            output: Some(build_output_table(OutputTableOverrides {
                destinations: Some(
                    entries
                        .iter()
                        .map(|(kind, path)| {
                            build_destination_table(DestinationTableOverrides {
                                kind: Some(declared_name(kind)),
                                path: Some(path.clone()),
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

/// `count` file destinations beneath `directory`, admitted by the public
/// `admit_output`, and their paths.
fn file_destinations(directory: &Path, count: usize) -> (Vec<OutputDestination>, Vec<PathBuf>) {
    let paths: Vec<PathBuf> = (0..count)
        .map(|index| directory.join(format!("outcome-{index}.json")))
        .collect();
    let destinations = admit_output_entries(
        &paths
            .iter()
            .map(|path| (DestinationKind::File, Some(path.clone())))
            .collect::<Vec<_>>(),
    );
    (destinations, paths)
}

/// The `json!` value an `L1DataFee` renders to.
fn fee_json(fee: L1DataFee) -> Value {
    match fee {
        L1DataFee::Charged { wei } => json!({ "state": "charged", "wei": wei.to_string() }),
        L1DataFee::NotApplicable => json!({ "state": "not_applicable" }),
    }
}

/// The `json!` literal built field by field from the values a successful
/// outcome holds, the document an outside caller expects the text to hold.
fn expected_document(success: &RunSuccessReturn) -> Value {
    json!({
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
    })
}

/// Contract: a successful run's outcome is written with every value it
///   holds — `benchmarks` an object per configured concrete in
///   configuration order and `curves` an object per curve in curve order,
///   the same text reaching every admitted destination.
/// Arrange: the configuration the run entry arranges — the real members
///   of `PAIRING_CONCRETES` as `benchmark_concretes` in a configured order
///   and in the reverse order, two curves of distinct labels and pairing
///   concretes — and file destinations admitted by `admit_output`, one
///   and two.
/// Act:     `run`, then `output`, through the crate root, once per case.
/// Assert:  `Ok`, the `text` parsed equals the `json!` literal built in
///   Arrange field by field from the outcome's values, and each file read
///   back equals the returned text.
/// Boundary: the public call `output` over the `RunReturn` the public call
///   `run` returned and the destinations the public call `admit_output`
///   admitted.
/// Mocked:   the verifier family's mock concrete and the filesystem, a
///   real directory.
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
        let (destinations, paths) = file_destinations(&directory, destination_count);
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
        let expected = expected_document(success);

        // Act
        let returned = output(
            &build_output_deps(Default::default()),
            build_output_params(OutputParamsOverrides {
                destinations: Some(destinations),
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
        for path in &paths {
            assert_eq!(
                std::fs::read_to_string(path)
                    .unwrap_or_else(|_| { panic!("{name}: the file {} reads", path.display()) }),
                returned.text,
                "{name}"
            );
        }
        // a_successful_runs_outcome_is_written_with_every_value_it_holds
    }
}

/// Contract: a refused run's outcome is written as its refusal — the
///   document `{ "outcome": "refused", "refusal": <Debug text> }` reaching
///   every admitted destination.
/// Arrange: the configuration whose benchmark concrete is
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode
///   ::SampleScalarWrongLength)`, the `Debug` text of the refusal the run
///   returned taken before the outcome is handed, and file destinations
///   admitted by `admit_output`, one and two.
/// Act:     `run`, then `output`, through the crate root, once per case.
/// Assert:  `Ok`, the `text` parsed equals
///   `json!({ "outcome": "refused", "refusal": <that text> })`, and each
///   file read back equals the returned text.
/// Boundary: the public call `output` over the `RunReturn` the public call
///   `run` returned and the destinations the public call `admit_output`
///   admitted.
/// Mocked:   the pairing family's mock concrete and the filesystem, a
///   real directory.
#[test]
fn a_refused_runs_outcome_is_written_as_its_refusal() {
    for (name, destination_count) in [("one destination", 1), ("two destinations", 2)] {
        // Arrange
        let directory = work_directory();
        let real = real_pairing_concretes();
        let (destinations, paths) = file_destinations(&directory, destination_count);
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
                destinations: Some(destinations),
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
        for path in &paths {
            assert_eq!(
                std::fs::read_to_string(path)
                    .unwrap_or_else(|_| { panic!("{name}: the file {} reads", path.display()) }),
                returned.text,
                "{name}"
            );
        }
        // a_refused_runs_outcome_is_written_as_its_refusal
    }
}

/// Contract, carried: the document's `benchmarks` array holds an object
///   for each configured benchmark concrete in configuration order, each
///   naming its concrete, and its `curves` array holds an object for each
///   curve in curve order, each holding its own label, its own files, and
///   its own measurement.
/// Arrange: the configuration the block of the same name of
///   `harness-crypto/run`'s public integration test arranges — the real
///   members of `PAIRING_CONCRETES` as `benchmark_concretes` in a
///   configured order and in the reverse order, and two curves of
///   distinct labels and pairing concretes — and one file destination
///   beneath the work directory, admitted by `admit_output`.
/// Act:     `run`, then `output`, through the crate root, once per case.
/// Assert:  `Ok`, the parsed document's `benchmarks` naming the
///   configured concretes in configuration order and its `curves` holding
///   each label with its own files and measurement in curve order.
/// Boundary: the public calls `run` and `output`.
/// Mocked:   the verifier family's mock concrete and the filesystem, a
///   real directory.
#[test]
fn the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order()
 {
    for (name, concretes, curve_concretes) in [
        (
            "declared order",
            real_pairing_concretes(),
            vec![real_pairing_concretes()[0], real_pairing_concretes()[1]],
        ),
        (
            "reverse order",
            real_pairing_concretes().into_iter().rev().collect(),
            vec![real_pairing_concretes()[1], real_pairing_concretes()[0]],
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let (destinations, _) = file_destinations(&directory, 1);
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
                    vec![
                        curve_of("curve-a", curve_concretes[0]),
                        curve_of("curve-b", curve_concretes[1]),
                    ],
                    HarnessConfigurationOverrides {
                        benchmark_iterations: NonZeroU32::new(3),
                        ..Default::default()
                    },
                ),
            },
        );
        outcome
            .as_ref()
            .unwrap_or_else(|_| panic!("{name}: the run over real concretes succeeds"));
        let expected_concretes: Vec<String> = concretes
            .iter()
            .map(|concrete| format!("{concrete:?}"))
            .collect();

        // Act
        let returned = output(
            &build_output_deps(Default::default()),
            build_output_params(OutputParamsOverrides {
                destinations: Some(destinations),
            }),
            build_output_payload(OutputPayloadOverrides {
                outcome: Some(outcome),
            }),
        )
        .unwrap_or_else(|_| panic!("{name}: the writes succeed"));

        // Assert
        let parsed: Value = serde_json::from_str(&returned.text)
            .unwrap_or_else(|_| panic!("{name}: the text parses"));
        let benchmarks = parsed["benchmarks"]
            .as_array()
            .unwrap_or_else(|| panic!("{name}: `benchmarks` is an array"));
        assert_eq!(benchmarks.len(), expected_concretes.len(), "{name}");
        for (benchmark, concrete) in benchmarks.iter().zip(&expected_concretes) {
            assert_eq!(benchmark["concrete"], json!(concrete), "{name}");
        }
        let curves = parsed["curves"]
            .as_array()
            .unwrap_or_else(|| panic!("{name}: `curves` is an array"));
        assert_eq!(curves.len(), 2, "{name}");
        for (curve, label) in curves.iter().zip(["curve-a", "curve-b"]) {
            assert_eq!(curve["label"], json!(label), "{name}");
            assert!(
                curve["files"]
                    .as_array()
                    .is_some_and(|files| !files.is_empty()),
                "{name}: the curve holds its own files"
            );
            assert_eq!(curve["measurement"]["iterations"], json!(1), "{name}");
        }
        // the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order
    }
}

/// Contract, carried: the `report` member of each curve's object holds
///   that curve's report document verbatim as one string, the document
///   the file written to the configured report destination holds.
/// Arrange: the configuration the block of the same name of
///   `harness-crypto/run`'s public integration test arranges — conditions
///   `"conditions-a"`, defaults of a piece-group size of `16384` and the
///   primary curve `"curve-b"`, over an absent adjustment and an
///   adjustment holding a piece-group size alone — and one file
///   destination beneath the work directory, admitted by `admit_output`.
/// Act:     `run`, then `output`, through the crate root, once per case.
/// Assert:  `Ok`, each curve's `report` member equal to that curve's
///   `report.document` and the last curve's equal to the file the
///   configured report destination holds.
/// Boundary: the public calls `run` and `output`.
/// Mocked:   the verifier family's mock concrete and the filesystem, a
///   real directory.
#[test]
fn each_curve_is_reported_under_its_own_label_with_the_configured_defaults() {
    for (name, adjustment) in [
        ("absent adjustment", None),
        (
            "adjustment of a piece-group size alone",
            Some(ReportAdjustment {
                piece_group_size: NonZeroU32::new(32_768),
                primary_curve: None,
            }),
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let real = real_pairing_concretes();
        let (destinations, _) = file_destinations(&directory, 1);
        let outcome = run(
            &RunDeps,
            RunParams,
            RunPayload {
                configuration: configuration(
                    &directory,
                    real.iter()
                        .map(|concrete| admitted_pairing_of(*concrete))
                        .collect(),
                    vec![curve_of("curve-a", real[0]), curve_of("curve-b", real[1])],
                    HarnessConfigurationOverrides {
                        measurement: Some(MeasurementSettings {
                            iterations: NonZeroU32::new(5).unwrap(),
                            conditions: build_report_text(ReportTextConstructorParamsOverrides {
                                text: Some("conditions-a".to_string()),
                            }),
                        }),
                        report: Some(ReportSettings {
                            defaults: ReportDefaults {
                                piece_group_size: NonZeroU32::new(16_384).unwrap(),
                                primary_curve: build_report_text(
                                    ReportTextConstructorParamsOverrides {
                                        text: Some("curve-b".to_string()),
                                    },
                                ),
                            },
                            adjustment,
                        }),
                        ..Default::default()
                    },
                ),
            },
        );
        let success = outcome
            .as_ref()
            .unwrap_or_else(|_| panic!("{name}: the run over real concretes succeeds"));
        let expected_documents: Vec<String> = success
            .curves
            .iter()
            .map(|curve| curve.report.document.clone())
            .collect();

        // Act
        let returned = output(
            &build_output_deps(Default::default()),
            build_output_params(OutputParamsOverrides {
                destinations: Some(destinations),
            }),
            build_output_payload(OutputPayloadOverrides {
                outcome: Some(outcome),
            }),
        )
        .unwrap_or_else(|_| panic!("{name}: the writes succeed"));

        // Assert
        let parsed: Value = serde_json::from_str(&returned.text)
            .unwrap_or_else(|_| panic!("{name}: the text parses"));
        let curves = parsed["curves"]
            .as_array()
            .unwrap_or_else(|| panic!("{name}: `curves` is an array"));
        assert_eq!(curves.len(), expected_documents.len(), "{name}");
        for (curve, document) in curves.iter().zip(&expected_documents) {
            assert_eq!(curve["report"], json!(document), "{name}");
        }
        assert_eq!(
            std::fs::read_to_string(directory.join("report.txt"))
                .unwrap_or_else(|_| panic!("{name}: the report file reads")),
            *expected_documents.last().expect("a last curve exists"),
            "{name}: the configured report destination holds the last curve's document"
        );
        // each_curve_is_reported_under_its_own_label_with_the_configured_defaults
    }
}

/// Contract, carried: the refusal the run returned, whole inside the
///   variant of the stage that refused, is the `refusal` member's `Debug`
///   text — a benchmark stage's refusal rendered in place of a curve
///   consumer's when both would refuse.
/// Arrange: the configuration the block of the same name of
///   `harness-crypto/run`'s public integration test arranges — a
///   `SampleScalarWrongLength` benchmark concrete with a verifier
///   refusing at the first index, then real benchmark concretes with the
///   same verifier and a report destination beneath a missing directory —
///   and one file destination beneath the work directory, admitted by
///   `admit_output`.
/// Act:     `run`, then `output`, through the crate root, once per case.
/// Assert:  `Ok`, the parsed document's `refusal` equal to the `Debug`
///   text of the earlier stage's refusal.
/// Boundary: the public calls `run` and `output`.
/// Mocked:   the verifier family's mock concrete, the pairing family's
///   mock concrete, and the filesystem, a real directory.
#[test]
fn the_first_refusal_is_returned_in_the_stated_order() {
    // Arrange, shared
    let directory = work_directory();
    let real = real_pairing_concretes();
    let (destinations, paths) = file_destinations(&directory, 1);
    let mut first = configuration(
        &directory,
        vec![admitted_pairing_of(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
        ))],
        vec![curve_of("curve-a", real[0]), curve_of("curve-b", real[1])],
        HarnessConfigurationOverrides {
            verifiers: Some(vec![
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused),
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
            ]),
            ..Default::default()
        },
    );
    let refusal = format!(
        "{:?}",
        run(
            &RunDeps,
            RunParams,
            RunPayload {
                configuration: first.clone(),
            },
        )
        .err()
        .expect("the benchmark stage refuses before the curve consumer")
    );

    // Act, first case: a benchmark refusal before a curve refusal.
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(destinations.clone()),
        }),
        build_output_payload(OutputPayloadOverrides {
            outcome: Some(run(
                &RunDeps,
                RunParams,
                RunPayload {
                    configuration: first.clone(),
                },
            )),
        }),
    )
    .expect("the writes succeed");

    // Assert, first case
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed, json!({ "outcome": "refused", "refusal": refusal }));
    assert_eq!(
        std::fs::read_to_string(&paths[0]).expect("the file reads"),
        returned.text
    );

    // Arrange, second case: a curve-consumer refusal before a report
    // refusal.
    let missing = directory.join("missing");
    let _ = std::fs::remove_dir_all(&missing);
    first.output.report_destinations = vec![ReportDestination::File {
        path: missing.join("report.txt"),
    }];
    first.selections.benchmark_concretes = real.iter().map(|c| admitted_pairing_of(*c)).collect();
    let refusal = format!(
        "{:?}",
        run(
            &RunDeps,
            RunParams,
            RunPayload {
                configuration: first.clone(),
            },
        )
        .err()
        .expect("the curve consumer refuses before the report")
    );

    // Act, second case
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(destinations),
        }),
        build_output_payload(OutputPayloadOverrides {
            outcome: Some(run(
                &RunDeps,
                RunParams,
                RunPayload {
                    configuration: first,
                },
            )),
        }),
    )
    .expect("the writes succeed");

    // Assert, second case
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed, json!({ "outcome": "refused", "refusal": refusal }));
    assert_eq!(
        std::fs::read_to_string(&paths[0]).expect("the file reads"),
        returned.text
    );
    // the_first_refusal_is_returned_in_the_stated_order
}

/// Contract, carried: a destination admitted for each member of
///   `DESTINATION_KINDS` is a destination the text reaches — a file
///   holding the path its entry carried and standard output the process's
///   standard output.
/// Arrange: one declared entry per member of `DESTINATION_KINDS`, the
///   destination chosen by an exhaustive `match` on the member — a file
///   path beneath the work directory for the file member — and the
///   default successful outcome.
/// Act:     `admit_output`, then `output`, through the crate root, once
///   per member.
/// Assert:  the admitted destination is the member's variant; a file read
///   back equal to the returned text and standard output returning `Ok`
///   with the same text.
/// Boundary: the public calls `admit_output` and `output`.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn every_declared_destination_kind_is_admitted_to_its_own_variant() {
    for member in DESTINATION_KINDS {
        // Arrange
        let directory = work_directory();
        let file_path = directory.join("outcome.json");
        let entry = match member {
            DestinationKind::StandardOutput => (*member, None),
            DestinationKind::File => (*member, Some(file_path.clone())),
        };
        let destinations = admit_output_entries(&[entry]);
        let payload = build_output_payload(Default::default());
        let success = payload
            .outcome
            .as_ref()
            .expect("the default outcome succeeds");
        let expected = expected_document(success);

        // Act
        let returned = output(
            &build_output_deps(Default::default()),
            build_output_params(OutputParamsOverrides {
                destinations: Some(destinations),
            }),
            payload,
        )
        .expect("the writes succeed");

        // Assert
        let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
        assert_eq!(
            parsed, expected,
            "{member:?}: the text reaches the destination"
        );
        if let DestinationKind::File = member {
            assert_eq!(
                std::fs::read_to_string(&file_path).expect("the file reads"),
                returned.text,
                "{member:?}: the file holds the text"
            );
        }
        // every_declared_destination_kind_is_admitted_to_its_own_variant
    }
}

/// Contract, carried: the destinations are written in the order the
///   admitted list holds them — a refused destination stopping the writes
///   after it and leaving the writes before it.
/// Arrange: an admitted list whose first destination is the path of a
///   directory and a list whose last destination is that path, and the
///   default successful outcome.
/// Act:     `admit_output`, then `output`, through the crate root, once
///   per case.
/// Assert:  `Err(OutputErrorReturn::WriteFile { path, .. })` with `path`
///   equal to the directory's path; the first case leaving the file after
///   it unwritten and the second leaving the file before it holding the
///   returned text.
/// Boundary: the public calls `admit_output` and `output`.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn a_destination_list_keeps_the_order_the_document_holds() {
    for (name, refuse_first) in [("refusal first", true), ("refusal last", false)] {
        // Arrange
        let directory = work_directory();
        let file = directory.join("outcome.json");
        let mut entries = vec![
            (DestinationKind::File, Some(file.clone())),
            (DestinationKind::File, Some(directory.clone())),
        ];
        if refuse_first {
            entries.reverse();
        }
        let destinations = admit_output_entries(&entries);
        let payload = build_output_payload(Default::default());
        let success = payload
            .outcome
            .as_ref()
            .expect("the default outcome succeeds");
        let expected = expected_document(success);

        // Act
        let result = output(
            &build_output_deps(Default::default()),
            build_output_params(OutputParamsOverrides {
                destinations: Some(destinations),
            }),
            payload,
        );

        // Assert
        assert!(
            matches!(
                &result,
                Err(OutputErrorReturn::WriteFile { path, .. }) if *path == directory
            ),
            "{name}: the refused write is returned with its path"
        );
        if refuse_first {
            assert!(!file.exists(), "{name}: the later file is unwritten");
        } else {
            assert_eq!(
                std::fs::read_to_string(&file).expect("the file reads"),
                format!("{expected:#}"),
                "{name}: the earlier file holds the text the outcome renders to"
            );
        }
        // a_destination_list_keeps_the_order_the_document_holds
    }
}
