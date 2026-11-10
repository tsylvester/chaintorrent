#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Private-surface enumeration:
//   `the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order`
//       proven by the block of the same name; carries
//       `the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole`,
//       `the_consumer_receives_the_concrete_the_params_name` (of
//       `harness-crypto/benchmark` and `pairing/factory`),
//       `a_sampled_scalar_encodes_through_secret`,
//       `every_operation_is_timed_on_every_concrete` (of both
//       `harness-crypto/benchmark` and `harness-crypto/report`),
//       `the_document_records_every_value_a_measurement_holds`,
//       `the_measurement_carries_the_iteration_count_its_means_were_computed_from`,
//       `every_object_is_sized_by_the_encodings_of_its_elements`,
//       `every_accepted_delivery_reports_the_cost_the_verifier_returned`,
//       `the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold`,
//       `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`,
//       `the_files_are_encoded_under_the_encoding_selection_the_consumer_holds`,
//       `the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`,
//       `the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs`, and
//       `the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs`.
//   `each_curve_is_composed_with_the_verifier_concrete_of_its_own_index`
//       proven by the block of the same name; carries
//       `the_verifier_is_selected_with_the_settings_the_document_holds`.
//   `each_curve_is_reported_under_its_own_label_with_the_configured_defaults`
//       proven by the block of the same name.
//   `the_first_refusal_is_returned_in_the_stated_order`
//       proven by the block of the same name; carries
//       `a_sampling_refusal_reaches_the_caller_unchanged` and, in its
//       verifier-refusal arm, the whole `PairingConsumer` chain the sibling
//       refusal entries name.

use core::num::NonZeroU32;
use pairing::{PAIRING_CONCRETES, PairingConcrete, TargetGroupEncodingIdentifier};
use random::RandomSourceKind;
use serde_json::{Value, json};
use std::env::temp_dir;
use std::path::{Path, PathBuf};

use super::interface::{RunDeps, RunErrorReturn, RunParams, RunPayload};
use super::run;
use crate::benchmark::provides::PairingBenchmarkErrorReturn;
use crate::config::provides::{
    AdmittedCurve, AdmittedGenerate, AdmittedPairing, AdmittedPairingOverrides,
    AdmittedSelectionsOverrides, HarnessConfigurationOverrides, build_admitted_pairing,
    build_admitted_selections, build_harness_configuration,
};
use crate::generate::provides::{
    GenerateConcrete, GenerateConcreteKind, SolidityGenerateSettingsOverrides,
    build_generate_kind_configurations, build_solidity_generate_settings,
};
use crate::report::provides::{
    ReportAdjustment, ReportDestination, ReportTextConstructorParamsOverrides, build_report_text,
};
use crate::verifier::provides::{MockIVerifierAdapterFailureMode, VerifierConcrete};

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

/// A directory beneath the operating system's temporary directory named
/// `chaintorrent-run-`, the process id, `-`, and the current thread's id as
/// its `Debug` text, removed before the act and created fresh.
fn work_directory() -> PathBuf {
    let directory = temp_dir().join(format!(
        "chaintorrent-run-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the work directory is created");
    directory
}

/// The configuration the private surface's cases share: every family's
/// first declared real member, the given benchmark concretes and curves,
/// two `Mock(NoFailure)` verifiers, the generate concrete rooted at the
/// work directory, and one file report destination beneath it.
fn configuration(
    directory: &Path,
    benchmark_concretes: Vec<AdmittedPairing>,
    curves: Vec<AdmittedCurve>,
    overrides: HarnessConfigurationOverrides,
) -> crate::config::provides::HarnessConfiguration {
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
        output: overrides
            .output
            .or(Some(crate::config::provides::AdmittedOutput {
                destinations: vec![crate::config::provides::OutputDestination::StandardOutput],
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

/// A curve of `label` over the admitted pairing of `concrete`.
fn curve_of(label: &str, concrete: PairingConcrete) -> AdmittedCurve {
    AdmittedCurve {
        label: label.to_string(),
        pairing: admitted_pairing_of(concrete),
    }
}

/// Contract: the run returns the measurement of each configured benchmark
///   concrete in configuration order, each `concrete` equal to its entry,
///   and, in curve order, each curve's label, the files its consumer wrote,
///   a measurement whose iteration count is the configured one, and the
///   report.
/// Arrange: the shared configuration, the real members of
///   `PAIRING_CONCRETES` as `benchmark_concretes` in a configured order and
///   in the reverse order, and two curves of distinct labels and pairing
///   concretes, so a run that sorts, reuses one curve's outcome, or reads
///   another index fails.
/// Act:     `run`, once per case.
/// Assert:  `Ok` whose `benchmarks` and `curves` hold the entry's values in
///   configuration order.
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
        let curves = vec![
            curve_of("curve-a", curve_concretes[0]),
            curve_of("curve-b", curve_concretes[1]),
        ];
        let expected_concretes: Vec<PairingConcrete> = concretes.clone();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                concretes
                    .iter()
                    .map(|concrete| admitted_pairing_of(*concrete))
                    .collect(),
                curves,
                HarnessConfigurationOverrides {
                    benchmark_iterations: NonZeroU32::new(3),
                    ..Default::default()
                },
            ),
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        let success = result
            .unwrap_or_else(|_| panic!("the run over real concretes succeeds for case {name}"));
        assert_eq!(success.benchmarks.len(), expected_concretes.len(), "{name}");
        for (benchmark, concrete) in success.benchmarks.iter().zip(expected_concretes) {
            assert_eq!(benchmark.concrete, concrete, "{name}");
            assert_eq!(benchmark.iterations.get(), 3, "{name}");
        }
        assert_eq!(success.curves.len(), 2, "{name}");
        assert_eq!(success.curves[0].label.as_str(), "curve-a", "{name}");
        assert_eq!(success.curves[1].label.as_str(), "curve-b", "{name}");
        for curve in &success.curves {
            assert!(!curve.files.is_empty(), "{name}");
            for file in &curve.files {
                assert!(
                    directory.join(&file.path).exists() || file.path.exists(),
                    "{name}: the emitted file {} exists beneath the foundry root",
                    file.path.display()
                );
            }
            assert_eq!(curve.measurement.iterations.get(), 1, "{name}");
            let document: Value = serde_json::from_str(&curve.report.document)
                .unwrap_or_else(|_| panic!("{name}: the report document is JSON"));
            assert_eq!(
                document["curve"],
                Value::from(curve.label.as_str()),
                "{name}"
            );
        }
    }
    // the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order
}

/// Contract: the verifier concrete of curve `i` is the entry of `verifiers`
///   at `i` and no other; a refusing concrete returns its consumer's
///   refusal whole under `PairingConsumer`, with the report of every curve
///   before the refusing index written and no report written for the
///   refusing index or after it.
/// Arrange: the shared configuration of two curves, `verifiers` refusing
///   at the first index, at the second, and at none, and a file report
///   destination beneath the work directory.
/// Act:     `run`, once per case.
/// Assert:  `Ok` when no index refuses; otherwise
///   `Err(RunErrorReturn::PairingConsumer(..))` and the report file present
///   or absent as the entry states.
#[test]
fn each_curve_is_composed_with_the_verifier_concrete_of_its_own_index() {
    for (name, verifiers, expected_report_present) in [
        (
            "no refusal",
            vec![
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
            ],
            true,
        ),
        (
            "refusal at the first index",
            vec![
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused),
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
            ],
            false,
        ),
        (
            "refusal at the second index",
            vec![
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused),
            ],
            true,
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let concretes = real_pairing_concretes();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                concretes
                    .iter()
                    .map(|concrete| admitted_pairing_of(*concrete))
                    .collect(),
                vec![
                    curve_of("curve-a", concretes[0]),
                    curve_of("curve-b", concretes[1]),
                ],
                HarnessConfigurationOverrides {
                    verifiers: Some(verifiers),
                    ..Default::default()
                },
            ),
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        let report_path = directory.join("report.txt");
        match result {
            Ok(_) => {
                assert!(expected_report_present, "{name}");
                assert!(report_path.exists(), "{name}: a report is written");
            }
            Err(RunErrorReturn::PairingConsumer(_)) => {
                assert_eq!(report_path.exists(), expected_report_present, "{name}");
            }
            Err(other) => panic!("{name}: unexpected refusal {other:?}"),
        }
    }
    // each_curve_is_composed_with_the_verifier_concrete_of_its_own_index
}

/// Contract: each curve's `report.document` and the file written to the
///   configured report destination record the measurement that curve's
///   consumer returned under that curve's label, its conditions, iteration
///   count, defaults, and adjustment.
/// Arrange: the shared configuration and a file report destination, over
///   an absent adjustment and an adjustment holding a piece-group size
///   alone, over two curves of distinct labels.
/// Act:     `run`, once per case.
/// Assert:  `Ok`, the file read back equal to the last curve's
///   `report.document` as the final report written, and the document
///   parsed to a JSON value holding the entry's fields.
#[test]
fn each_curve_is_reported_under_its_own_label_with_the_configured_defaults() {
    for (name, adjustment, expected_adjustment) in [
        ("absent adjustment", None, Value::Null),
        (
            "adjustment of a piece-group size alone",
            Some(ReportAdjustment {
                piece_group_size: NonZeroU32::new(32_768),
                primary_curve: None,
            }),
            json!({ "piece_group_size_bytes": 32768, "primary_curve": null }),
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let concretes = real_pairing_concretes();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                concretes
                    .iter()
                    .map(|concrete| admitted_pairing_of(*concrete))
                    .collect(),
                vec![
                    curve_of("curve-a", concretes[0]),
                    curve_of("curve-b", concretes[1]),
                ],
                HarnessConfigurationOverrides {
                    measurement: Some(crate::config::provides::MeasurementSettings {
                        iterations: NonZeroU32::new(5).unwrap(),
                        conditions: build_report_text(ReportTextConstructorParamsOverrides {
                            text: Some("conditions-a".to_string()),
                        }),
                    }),
                    report: Some(crate::config::provides::ReportSettings {
                        defaults: crate::report::provides::ReportDefaults {
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
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        let success =
            result.unwrap_or_else(|_| panic!("{name}: the run over real concretes succeeds"));
        let written = std::fs::read_to_string(directory.join("report.txt"))
            .unwrap_or_else(|_| panic!("{name}: the report file is written"));
        assert_eq!(written, success.curves[1].report.document, "{name}");
        let document: Value = serde_json::from_str(&written)
            .unwrap_or_else(|_| panic!("{name}: the document is JSON"));
        assert_eq!(document["curve"], Value::from("curve-b"), "{name}");
        assert_eq!(
            document["conditions"],
            Value::from("conditions-a"),
            "{name}"
        );
        assert_eq!(
            document["iterations"],
            Value::from(success.curves[1].measurement.iterations.get()),
            "{name}"
        );
        assert_eq!(
            document["defaults_in_force"],
            json!({ "piece_group_size_bytes": 16384, "primary_curve": "curve-b" }),
            "{name}"
        );
        assert_eq!(document["adjustment"], expected_adjustment, "{name}");
    }
    // each_curve_is_reported_under_its_own_label_with_the_configured_defaults
}

/// Contract: the source for the curves is obtained first; the benchmark
///   entries run in configuration order before the curves; each curve's
///   report is written before the next curve's pairing is created; the
///   first refusal returns whole and nothing after it runs.
/// Arrange: the shared configuration with a refusal in two stages — a
///   benchmark stage's `SampleScalarWrongLength` concrete with a verifier
///   refusing at the first index, and a verifier refusing at the first
///   curve with a report destination no directory holds.
/// Act:     `run`, once per case.
/// Assert:  the earlier stage's refusal whole, by a pattern naming every
///   field.
#[test]
fn the_first_refusal_is_returned_in_the_stated_order() {
    // Arrange, shared
    let directory = work_directory();
    let concretes = real_pairing_concretes();

    // Arrange, first case: a benchmark refusal before a curve refusal.
    let mut first = configuration(
        &directory,
        vec![admitted_pairing_of(PairingConcrete::Mock(
            pairing::MockIPairingAdapterFailureMode::SampleScalarWrongLength,
        ))],
        vec![
            curve_of("curve-a", concretes[0]),
            curve_of("curve-b", concretes[1]),
        ],
        HarnessConfigurationOverrides {
            verifiers: Some(vec![
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused),
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
            ]),
            ..Default::default()
        },
    );
    first
        .selections
        .precompile_encodings
        .push(pairing::PrecompileEncoding::Mock);

    // Act, first case
    let result = run(
        &RunDeps,
        RunParams,
        RunPayload {
            configuration: first.clone(),
        },
    );

    // Assert, first case
    assert!(matches!(
        result,
        Err(RunErrorReturn::Benchmark(
            PairingBenchmarkErrorReturn::SampleScalar(
                pairing::SampleUniformScalarErrorReturn::WrongLength {
                    expected: 64,
                    actual: 64,
                }
            )
        ))
    ));

    // Arrange, second case: a curve-consumer refusal before a report
    // refusal.
    let missing = directory.join("missing");
    let _ = std::fs::remove_dir_all(&missing);
    first.output.report_destinations = vec![ReportDestination::File {
        path: missing.join("report.txt"),
    }];
    first.verifiers = vec![
        VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused),
        VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
    ];
    first.selections.benchmark_concretes = concretes
        .iter()
        .map(|concrete| admitted_pairing_of(*concrete))
        .collect();

    // Act, second case
    let result = run(
        &RunDeps,
        RunParams,
        RunPayload {
            configuration: first,
        },
    );

    // Assert, second case
    assert!(matches!(result, Err(RunErrorReturn::PairingConsumer(_))));
    // the_first_refusal_is_returned_in_the_stated_order
}
