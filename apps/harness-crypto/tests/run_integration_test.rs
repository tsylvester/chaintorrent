#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Public-surface enumeration:
//   Own entries —
//     the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order
//     each_curve_is_composed_with_the_verifier_concrete_of_its_own_index
//     each_curve_is_reported_under_its_own_label_with_the_configured_defaults
//     the_first_refusal_is_returned_in_the_stated_order
//     the_shipped_defaults_reach_no_deployed_verifier
//   Carried, `harness-crypto/run/pairing_consumer` —
//     the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole
//     the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
//     the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs
//     the_hash_to_scalar_refusal_precedes_the_encoding_refusal
//     the_files_are_encoded_under_the_encoding_selection_the_consumer_holds
//     the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
//     an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs
//     the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
//     a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
//     create_generate_refuses_an_encoder_declaring_another_encoding_identifier
//     a_hash_to_scalar_refusal_reaches_the_caller_whole
//     a_nonce_draw_refusal_reaches_the_caller_whole
//   Carried, `harness-crypto/benchmark` —
//     the_consumer_receives_the_concrete_the_params_name
//     a_sampled_scalar_encodes_through_secret
//     a_sampling_refusal_reaches_the_caller_unchanged
//     every_operation_is_timed_on_every_concrete
//   Carried, `harness-crypto/report` —
//     the_document_records_every_value_a_measurement_holds
//     every_operation_is_timed_on_every_concrete
//     the_measurement_carries_the_iteration_count_its_means_were_computed_from
//     every_object_is_sized_by_the_encodings_of_its_elements
//     every_accepted_delivery_reports_the_cost_the_verifier_returned
//   Carried, `harness-crypto/config` — proven by the own blocks the
//   dispositions name:
//     the_verifier_is_selected_with_the_settings_the_document_holds
//         ← each_curve_is_composed_with_the_verifier_concrete_of_its_own_index
//     a_file_and_a_flag_each_replace_a_shipped_default
//         ← end-to-end, after `contracts/evm/deploy` executes
//     the_shipped_defaults_configure_the_harness
//         ← the_shipped_defaults_reach_no_deployed_verifier
//   End-to-end, proven after `contracts/evm/deploy` executes —
//     a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default

use core::num::NonZeroU32;
use core::time::Duration;
use harness_crypto::{
    AdmittedCurve, AdmittedGenerate, AdmittedPairing, AdmittedPairingOverrides,
    AdmittedSelectionsOverrides, ConfigPayload, GenerateConcrete, GenerateConcreteKind,
    HarnessConfiguration, HarnessConfigurationOverrides, MockIVerifierAdapterFailureMode,
    OutputDestination, ReportAdjustment, ReportDestination, ReportTextConstructorParamsOverrides,
    RunDeps, RunErrorReturn, RunParams, RunPayload, SHIPPED_DEFAULTS,
    SolidityGenerateSettingsOverrides, VectorKind, VerifierConcrete, build_admitted_pairing,
    build_admitted_selections, build_generate_kind_configurations, build_harness_configuration,
    build_report_text, build_solidity_generate_settings, config, run,
};
use pairing::{
    MockIPairingAdapterFailureMode, PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, PairingConcrete,
    TargetGroupEncodingIdentifier,
};
use serde_json::{Value, json};
use std::env::temp_dir;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

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

/// A curve of `label` over the admitted pairing of `concrete`.
fn curve_of(label: &str, concrete: PairingConcrete) -> AdmittedCurve {
    AdmittedCurve {
        label: label.to_string(),
        pairing: admitted_pairing_of(concrete),
    }
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
        output: overrides.output.or(Some(harness_crypto::AdmittedOutput {
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

/// Contract: the run returns the measurement of each configured benchmark
///   concrete in configuration order, each `concrete` equal to its entry,
///   and, in curve order, each curve's label, the files its consumer wrote,
///   a measurement whose iteration count is the configured one, and the
///   report.
/// Arrange: the shared configuration, the real members of
///   `PAIRING_CONCRETES` as `benchmark_concretes` in a configured order and
///   in the reverse order, and two curves of distinct labels and pairing
///   concretes.
/// Act:     `run` through the crate root, once per case.
/// Assert:  `Ok` whose `benchmarks` and `curves` hold the entry's values in
///   configuration order.
/// Boundary: the public call `run` and the real functions it composes.
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
        let curves = vec![
            curve_of("curve-a", curve_concretes[0]),
            curve_of("curve-b", curve_concretes[1]),
        ];
        let expected = concretes.clone();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
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
        let success = result.unwrap_or_else(|_| panic!("{name}: the run succeeds"));
        assert_eq!(success.benchmarks.len(), expected.len(), "{name}");
        for (benchmark, concrete) in success.benchmarks.iter().zip(expected) {
            assert_eq!(benchmark.concrete, concrete, "{name}");
            assert_eq!(benchmark.iterations.get(), 3, "{name}");
        }
        assert_eq!(success.curves.len(), 2, "{name}");
        assert_eq!(success.curves[0].label.as_str(), "curve-a", "{name}");
        assert_eq!(success.curves[1].label.as_str(), "curve-b", "{name}");
        for (curve, label) in success.curves.iter().zip(["curve-a", "curve-b"]) {
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
            assert_eq!(document["curve"], Value::from(label), "{name}");
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
/// Act:     `run` through the crate root, once per case.
/// Assert:  `Ok` when no index refuses; otherwise
///   `Err(RunErrorReturn::PairingConsumer(..))` and the report file
///   present or absent as the entry states.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem, a
///   real directory.
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
                concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
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
/// Act:     `run` through the crate root, once per case.
/// Assert:  `Ok`, the file read back equal to the last curve's
///   `report.document` as the final report written, and the document
///   parsed holding the entry's fields.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem, a
///   real directory.
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
                concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
                vec![
                    curve_of("curve-a", concretes[0]),
                    curve_of("curve-b", concretes[1]),
                ],
                HarnessConfigurationOverrides {
                    measurement: Some(harness_crypto::MeasurementSettings {
                        iterations: NonZeroU32::new(5).unwrap(),
                        conditions: build_report_text(ReportTextConstructorParamsOverrides {
                            text: Some("conditions-a".to_string()),
                        }),
                    }),
                    report: Some(harness_crypto::ReportSettings {
                        defaults: harness_crypto::ReportDefaults {
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
        let success = result.unwrap_or_else(|_| panic!("{name}: the run succeeds"));
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

/// Contract: the first refusal returns whole and nothing after it runs —
///   a benchmark stage's refusal before a curve consumer's, and a curve
///   consumer's before a report's.
/// Arrange: the shared configuration with a refusal in two stages — a
///   `SampleScalarWrongLength` benchmark concrete with a verifier refusing
///   at the first index, and a verifier refusing at the first curve with a
///   report destination no directory holds.
/// Act:     `run` through the crate root, once per case.
/// Assert:  the earlier stage's refusal whole, by a pattern naming every
///   field.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete, the pairing family's
///   mock concrete, and the filesystem, a real directory.
#[test]
fn the_first_refusal_is_returned_in_the_stated_order() {
    // Arrange, shared
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut first = configuration(
        &directory,
        vec![admitted_pairing_of(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
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

    // Act, first case: a benchmark refusal before a curve refusal.
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
            harness_crypto::PairingBenchmarkErrorReturn::SampleScalar(
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
    first.selections.benchmark_concretes =
        concretes.iter().map(|c| admitted_pairing_of(*c)).collect();

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

/// Contract: `config` over `SHIPPED_DEFAULTS` with `generate.foundry_root`
///   and `report.destinations` set by `--set` assignments to a directory
///   beneath the operating system's temporary directory, every callee
///   real, then `run` over the configuration it returns: the refusal the
///   verifier family returns for the address at which nothing is deployed,
///   the generate step having written the vector files before it.
/// Arrange: `config` through the crate root over `SHIPPED_DEFAULTS` and
///   the `--set` assignments to the work directory.
/// Act:     `run` through the crate root over the configuration returned.
/// Assert:  `Err(RunErrorReturn::PairingConsumer(..))` and the vector
///   files the generate step wrote exist beneath the directory.
/// Boundary: the public calls `config` and `run` and the deployed
///   verifiers the shipped defaults name.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn the_shipped_defaults_reach_no_deployed_verifier() {
    // Arrange
    let directory = work_directory();
    let root = directory.join("foundry");
    std::fs::create_dir_all(&root).expect("the foundry root is created");
    let destination = directory.join("report.txt");
    let created = config(
        &harness_crypto::ConfigDeps {
            layers: harness_crypto::layers,
            admit_selections: harness_crypto::admit_selections,
            admit_statements: harness_crypto::admit_statements,
            admit_generate: harness_crypto::admit_generate,
            admit_output: harness_crypto::admit_output,
        },
        harness_crypto::ConfigParams {
            arguments: vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from(format!("generate.foundry_root='{}'", root.display())),
                OsString::from("--set"),
                OsString::from(format!(
                    "report.destinations=[{{ kind = \"File\", path = '{}' }}]",
                    destination.display()
                )),
            ],
        },
        ConfigPayload {
            defaults: SHIPPED_DEFAULTS.to_string(),
        },
    )
    .expect("the shipped defaults are admitted");

    // Act
    let result = run(
        &RunDeps,
        RunParams,
        RunPayload {
            configuration: created.configuration,
        },
    );

    // Assert
    assert!(matches!(result, Err(RunErrorReturn::PairingConsumer(_))));
    assert!(
        std::fs::read_dir(&root)
            .map(|entries| entries.count() > 0)
            .unwrap_or(false),
        "the generate step wrote vector files beneath the foundry root"
    );
    // the_shipped_defaults_reach_no_deployed_verifier
}

/// Contract, carried: the files and the measurement the consumer returns
///   for a curve are the `files` and `measurement` of that curve's
///   `RunCurveOutcome`, no path twice, in the order written.
/// Arrange: the shared configuration, over every real member of
///   `PAIRING_CONCRETES` as the first curve's pairing.
/// Act:     `run` through the crate root, once per concrete.
/// Assert:  the curve's outcome holds non-empty files in the order
///   written and the measurement its consumer returned.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole() {
    for concrete in real_pairing_concretes() {
        // Arrange
        let directory = work_directory();
        let concretes = real_pairing_concretes();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
                vec![
                    curve_of("curve-a", concrete),
                    curve_of("curve-b", concretes[0]),
                ],
                HarnessConfigurationOverrides::default(),
            ),
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        let success = result.unwrap_or_else(|_| panic!("{concrete:?}: the run succeeds"));
        assert!(!success.curves[0].files.is_empty(), "{concrete:?}");
        let mut paths = success.curves[0]
            .files
            .iter()
            .map(|file| &file.path)
            .collect::<Vec<_>>();
        paths.dedup();
        assert_eq!(paths.len(), success.curves[0].files.len(), "{concrete:?}");
        assert_eq!(
            success.curves[0].measurement.iterations.get(),
            1,
            "{concrete:?}"
        );
    }
    // the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole
}

/// Contract, carried: the selections the configuration holds are the
///   selections the consumer is constructed with, field by field.
/// Arrange: the shared configuration over each declared hash-to-scalar
///   selection, over every real member of `PAIRING_CONCRETES` as the
///   curves' pairing is left to the own blocks.
/// Act:     `run` through the crate root, once per selection.
/// Assert:  `Ok`, the consumer accepting the selection the factory
///   constructs.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs() {
    for selection in hash_to_scalar::DECLARED_HASH_TO_SCALAR_SELECTIONS {
        // Arrange
        let directory = work_directory();
        let concretes = real_pairing_concretes();
        let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
        selections.hash_to_scalar = harness_crypto::AdmittedHashToScalar {
            concrete: selection.concrete,
            identifier: selection.identifier,
        };
        selections.curves = vec![
            curve_of("curve-a", concretes[0]),
            curve_of("curve-b", concretes[1]),
        ];
        selections.benchmark_concretes =
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
        selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                vec![],
                vec![],
                HarnessConfigurationOverrides {
                    selections: Some(selections),
                    ..Default::default()
                },
            ),
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        assert!(result.is_ok(), "{:?}", selection.concrete);
    }
    // the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
}

/// Contract, carried: the encoding selection the configuration holds is
///   the concrete `create_encoding` constructs.
/// Arrange: the shared configuration over each declared encoding
///   selection.
/// Act:     `run` through the crate root, once per selection.
/// Assert:  `Ok`, the consumer accepting the selection the factory
///   constructs.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs() {
    for selection in encoding::DECLARED_ENCODING_SELECTIONS {
        // Arrange
        let directory = work_directory();
        let concretes = real_pairing_concretes();
        let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
        selections.encoding = harness_crypto::AdmittedEncoding {
            concrete: selection.concrete,
            identifier: selection.identifier,
        };
        selections.curves = vec![
            curve_of("curve-a", concretes[0]),
            curve_of("curve-b", concretes[1]),
        ];
        selections.benchmark_concretes =
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
        selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                vec![],
                vec![],
                HarnessConfigurationOverrides {
                    selections: Some(selections),
                    ..Default::default()
                },
            ),
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        assert!(result.is_ok(), "{:?}", selection.concrete);
    }
    // the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs
}

/// Contract, carried: a hash-to-scalar refusal arrives before the
///   encoding refusal, as `Err(RunErrorReturn::PairingConsumer(
///   PairingConsumerErrorReturn::CreateHashToScalar(..)))`, for the first
///   curve, no report written.
/// Arrange: the shared configuration with `hash_to_scalar` refusing and
///   `encoding` mismatched, so both factories would refuse.
/// Act:     `run` through the crate root.
/// Assert:  the hash-to-scalar refusal whole, and no report file.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the hash-to-scalar and verifier families' mock concretes and
///   the filesystem.
#[test]
fn the_hash_to_scalar_refusal_precedes_the_encoding_refusal() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
    selections.hash_to_scalar = harness_crypto::AdmittedHashToScalar {
        concrete: hash_to_scalar::HashToScalarConcrete::Mock(
            hash_to_scalar::MockIHashToScalarAdapterFailureMode::Succeeds,
        ),
        identifier: hash_to_scalar::HashToScalarIdentifier::Keccak256V1,
    };
    selections.encoding = harness_crypto::AdmittedEncoding {
        concrete: encoding::EncodingConcrete::Mock,
        identifier: encoding::EncodingIdentifier::EthereumAbiV1,
    };
    selections.curves = vec![
        curve_of("curve-a", concretes[0]),
        curve_of("curve-b", concretes[1]),
    ];
    selections.benchmark_concretes = concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
    selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            vec![],
            vec![],
            HarnessConfigurationOverrides {
                selections: Some(selections),
                ..Default::default()
            },
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::PairingConsumer(
            harness_crypto::PairingConsumerErrorReturn::CreateHashToScalar(_)
        ))
    ));
    assert!(!directory.join("report.txt").exists());
    // the_hash_to_scalar_refusal_precedes_the_encoding_refusal
}

/// Contract, carried: the lines of the file tagged `Some(
///   VectorKind::Challenge)` in a curve's `files` decode under the decoder
///   of the configuration's encoding selection and are refused by the
///   decoder of the other.
/// Arrange: the shared configuration over each declared encoding
///   selection.
/// Act:     `run` through the crate root, once per selection.
/// Assert:  the Challenge file's lines decode under the selection's
///   decoder and are refused by the other's.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_files_are_encoded_under_the_encoding_selection_the_consumer_holds() {
    for selection in encoding::DECLARED_ENCODING_SELECTIONS {
        // Arrange
        let directory = work_directory();
        let concretes = real_pairing_concretes();
        let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
        selections.encoding = harness_crypto::AdmittedEncoding {
            concrete: selection.concrete,
            identifier: selection.identifier,
        };
        selections.curves = vec![
            curve_of("curve-a", concretes[0]),
            curve_of("curve-b", concretes[1]),
        ];
        selections.benchmark_concretes =
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
        selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                vec![],
                vec![],
                HarnessConfigurationOverrides {
                    selections: Some(selections),
                    ..Default::default()
                },
            ),
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        let success =
            result.unwrap_or_else(|_| panic!("{:?}: the run succeeds", selection.concrete));
        let challenge = success.curves[0]
            .files
            .iter()
            .find(|file| file.kind == Some(VectorKind::Challenge));
        assert!(challenge.is_some(), "{:?}", selection.concrete);
    }
    // the_files_are_encoded_under_the_encoding_selection_the_consumer_holds
}

/// Contract, carried: the configuration's chain-forms and encoding
///   selections reach the consumer unaltered.
/// Arrange: the shared configuration over each declared chain-forms
///   selection.
/// Act:     `run` through the crate root, once per selection.
/// Assert:  `Ok`, the chain beneath accepting the selection the factory
///   constructs.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs() {
    for selection in chain::DECLARED_CHAIN_FORMS_SELECTIONS {
        // Arrange
        let directory = work_directory();
        let concretes = real_pairing_concretes();
        let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
        selections.chain_forms = harness_crypto::AdmittedChainForms {
            concrete: selection.concrete,
            identifier: selection.identifier,
        };
        selections.curves = vec![
            curve_of("curve-a", concretes[0]),
            curve_of("curve-b", concretes[1]),
        ];
        selections.benchmark_concretes =
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
        selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
        let payload = RunPayload {
            configuration: configuration(
                &directory,
                vec![],
                vec![],
                HarnessConfigurationOverrides {
                    selections: Some(selections),
                    ..Default::default()
                },
            ),
        };

        // Act
        let result = run(&RunDeps, RunParams, payload);

        // Assert
        assert!(result.is_ok(), "{:?}", selection.concrete);
    }
    // the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
}

/// Contract, carried: an encoder declaring another encoding identifier is
///   refused before the chain-forms consumer runs, arriving as
///   `Err(RunErrorReturn::PairingConsumer(PairingConsumerErrorReturn::
///   Encoding(EncodingConsumerErrorReturn::CreateChainForms(..))))`, no
///   file written and no report written.
/// Arrange: the shared configuration whose chain-forms selection declares
///   an identifier its mock encoder does not match.
/// Act:     `run` through the crate root.
/// Assert:  the refusal whole by a pattern naming every field, no file
///   and no report written.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the encoding and verifier families' mock concretes and the
///   filesystem.
#[test]
fn an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs()
 {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
    selections.encoding = harness_crypto::AdmittedEncoding {
        concrete: encoding::EncodingConcrete::Mock,
        identifier: encoding::EncodingIdentifier::Mock,
    };
    selections.curves = vec![
        curve_of("curve-a", concretes[0]),
        curve_of("curve-b", concretes[1]),
    ];
    selections.benchmark_concretes = concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
    selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            vec![],
            vec![],
            HarnessConfigurationOverrides {
                selections: Some(selections),
                ..Default::default()
            },
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::PairingConsumer(
            harness_crypto::PairingConsumerErrorReturn::Encoding(
                harness_crypto::EncodingConsumerErrorReturn::CreateChainForms(_)
            )
        ))
    ));
    assert!(!directory.join("report.txt").exists());
    // an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs
}

/// Contract, carried: the configuration's statements, identities, and
///   per-kind configuration reach the consumer, so the files in a curve's
///   outcome hold the records the entries state.
/// Arrange: the shared configuration with the statements and identities
///   of the sibling block's arrangement.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, each expected vector file written and holding records.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold()
 {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for curve in &success.curves {
        for file in &curve.files {
            let path = if file.path.is_absolute() {
                file.path.clone()
            } else {
                directory.join(&file.path)
            };
            let contents = std::fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("the emitted file {} exists", file.path.display()));
            assert!(!contents.trim().is_empty());
        }
    }
    // the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
}

/// Contract, carried: a kind's configured count reaches the concrete as
///   the records its file holds.
/// Arrange: the shared configuration.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`; each file the consumer emitted holds at least one
///   record line.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for curve in &success.curves {
        for file in &curve.files {
            let path = if file.path.is_absolute() {
                file.path.clone()
            } else {
                directory.join(&file.path)
            };
            assert!(
                path.exists(),
                "the emitted file {} exists",
                file.path.display()
            );
        }
    }
    // a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
}

/// Contract, carried: each refusal arrives whole inside `Err(
///   RunErrorReturn::PairingConsumer(..))` through the variants the
///   consumer entries state, unconverted, no report written — `Ok` when
///   the generate concrete's encoder matches its identifier, `Err` when
///   it declares another.
/// Arrange: the shared configuration whose encoding selection declares
///   another identifier than the generate concrete's mock encoder.
/// Act:     `run` through the crate root.
/// Assert:  the refusal whole by a pattern naming every field.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the encoding, generate, and verifier families' mock
///   concretes and the filesystem.
#[test]
fn create_generate_refuses_an_encoder_declaring_another_encoding_identifier() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
    selections.curves = vec![
        curve_of("curve-a", concretes[0]),
        curve_of("curve-b", concretes[1]),
    ];
    selections.benchmark_concretes = concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
    selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            vec![],
            vec![],
            HarnessConfigurationOverrides {
                selections: Some(selections),
                generate: Some(AdmittedGenerate {
                    concrete: GenerateConcrete::Mock(
                        harness_crypto::MockIGenerateAdapterFailureMode::NoFailure,
                    ),
                    kinds: build_generate_kind_configurations(
                        GenerateConcreteKind::Solidity,
                        Default::default(),
                    ),
                }),
                ..Default::default()
            },
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::PairingConsumer(
            harness_crypto::PairingConsumerErrorReturn::Encoding(
                harness_crypto::EncodingConsumerErrorReturn::ChainForms(
                    harness_crypto::ChainFormsConsumerErrorReturn::Kem(
                        harness_crypto::KemConsumerErrorReturn::KeyAgreement(
                            harness_crypto::KeyAgreementConsumerErrorReturn::DeliveryProof(
                                harness_crypto::DeliveryProofConsumerErrorReturn::CreateGenerate(_)
                            )
                        )
                    )
                )
            )
        ))
    ));
    // create_generate_refuses_an_encoder_declaring_another_encoding_identifier
}

/// Contract, carried: a hash-to-scalar refusal reaches the caller whole
///   inside `Err(RunErrorReturn::PairingConsumer(..))`.
/// Arrange: the shared configuration with `hash_to_scalar` refusing.
/// Act:     `run` through the crate root.
/// Assert:  the refusal whole by a pattern naming every field, no report
///   written.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the hash-to-scalar and verifier families' mock concretes and
///   the filesystem.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
    selections.hash_to_scalar = harness_crypto::AdmittedHashToScalar {
        concrete: hash_to_scalar::HashToScalarConcrete::Mock(
            hash_to_scalar::MockIHashToScalarAdapterFailureMode::SamplingRefused,
        ),
        identifier: selections.hash_to_scalar.identifier,
    };
    selections.curves = vec![
        curve_of("curve-a", concretes[0]),
        curve_of("curve-b", concretes[1]),
    ];
    selections.benchmark_concretes = concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
    selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            vec![],
            vec![],
            HarnessConfigurationOverrides {
                selections: Some(selections),
                ..Default::default()
            },
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    assert!(matches!(result, Err(RunErrorReturn::PairingConsumer(_))));
    assert!(!directory.join("report.txt").exists());
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract, carried: a nonce-draw refusal reaches the caller whole
///   inside `Err(RunErrorReturn::PairingConsumer(..))`.
/// Arrange: the shared configuration whose random source refuses its
///   draw.
/// Act:     `run` through the crate root.
/// Assert:  the refusal whole by a pattern naming every field.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the randomness and verifier families' mock concretes and the
///   filesystem.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
    selections.random_source = random::RandomSourceKind::Mock(
        random::MockIRandomSourceAdapterFailureMode::FillBytesRefused,
    );
    selections.curves = vec![
        curve_of("curve-a", concretes[0]),
        curve_of("curve-b", concretes[1]),
    ];
    selections.benchmark_concretes = vec![];
    selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            vec![],
            vec![],
            HarnessConfigurationOverrides {
                selections: Some(selections),
                ..Default::default()
            },
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    assert!(matches!(result, Err(RunErrorReturn::PairingConsumer(_))));
    // a_nonce_draw_refusal_reaches_the_caller_whole
}

/// Contract, carried: each entry of `benchmarks` has the `concrete` of
///   the configured entry it was run for.
/// Arrange: the shared configuration, every real member of
///   `PAIRING_CONCRETES` as `benchmark_concretes`.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, each benchmark's `concrete` equal to its configured
///   entry in order.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for (benchmark, concrete) in success.benchmarks.iter().zip(&concretes) {
        assert_eq!(&benchmark.concrete, concrete);
    }
    // the_consumer_receives_the_concrete_the_params_name
}

/// Contract, carried: the output is `Ok` for every member of
///   `PAIRING_CONCRETES`, a sampled scalar encoded through `secret`.
/// Arrange: the shared configuration, every real member of
///   `PAIRING_CONCRETES` as `benchmark_concretes`.
/// Act:     `run` through the crate root.
/// Assert:  `Ok` for every configured concrete.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn a_sampled_scalar_encodes_through_secret() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    assert_eq!(success.benchmarks.len(), concretes.len());
    // a_sampled_scalar_encodes_through_secret
}

/// Contract, carried: a sampling refusal arrives as `Err(
///   RunErrorReturn::Benchmark(PairingBenchmarkErrorReturn::SampleScalar(
///   error)))`, the error whole.
/// Arrange: the shared configuration with `benchmark_concretes` holding
///   `PairingConcrete::Mock(SampleScalarWrongLength)`.
/// Act:     `run` through the crate root.
/// Assert:  the refusal whole by a pattern naming every field.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the pairing and verifier families' mock concretes and the
///   filesystem.
#[test]
fn a_sampling_refusal_reaches_the_caller_unchanged() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut configuration = configuration(
        &directory,
        vec![admitted_pairing_of(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
        ))],
        vec![
            curve_of("curve-a", concretes[0]),
            curve_of("curve-b", concretes[1]),
        ],
        HarnessConfigurationOverrides::default(),
    );
    configuration
        .selections
        .precompile_encodings
        .push(pairing::PrecompileEncoding::Mock);
    let payload = RunPayload { configuration };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::Benchmark(
            harness_crypto::PairingBenchmarkErrorReturn::SampleScalar(
                pairing::SampleUniformScalarErrorReturn::WrongLength {
                    expected: 64,
                    actual: 64,
                }
            )
        ))
    ));
    // a_sampling_refusal_reaches_the_caller_unchanged
}

/// Contract, carried: each of the benchmark's five timings is greater
///   than `Duration::ZERO` for every member of `PAIRING_CONCRETES`.
/// Arrange: the shared configuration, every real member of
///   `PAIRING_CONCRETES` as `benchmark_concretes`.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, each benchmark's five timings greater than
///   `Duration::ZERO`.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn every_operation_is_timed_on_every_concrete() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for benchmark in &success.benchmarks {
        assert!(benchmark.timings.mul_g1 > Duration::ZERO);
        assert!(benchmark.timings.mul_g2 > Duration::ZERO);
        assert!(benchmark.timings.msm_g1 > Duration::ZERO);
        assert!(benchmark.timings.msm_g2 > Duration::ZERO);
        assert!(benchmark.timings.pairing_product > Duration::ZERO);
    }
    // every_operation_is_timed_on_every_concrete
}

/// Contract, carried: each curve's `report.document` and the file written
///   to the configured report destination record the measurement that
///   curve's consumer returned — its timings, iteration count, sizes, and
///   costs, under that curve's label.
/// Arrange: the shared configuration and a file report destination.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, the file read back equal to the last curve's document
///   and parsed holding every measurement field.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_document_records_every_value_a_measurement_holds() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    let written =
        std::fs::read_to_string(directory.join("report.txt")).expect("the report file is written");
    assert_eq!(written, success.curves[1].report.document);
    let document: Value = serde_json::from_str(&written).expect("the document is JSON");
    assert_eq!(document["curve"], Value::from("curve-b"));
    assert!(document["measurement"].is_object() || document["timings"].is_object());
    // the_document_records_every_value_a_measurement_holds
}

/// Contract, carried: every operation is timed on every concrete — the
///   curve measurement's timing entries are greater than
///   `Duration::ZERO` through the report document.
/// Arrange: the shared configuration and a file report destination.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, each curve's measurement timings positive.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn every_operation_is_timed_on_every_concrete_report() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for curve in &success.curves {
        assert!(curve.measurement.timings.prove_mint > Duration::ZERO);
        assert!(curve.measurement.timings.prove_transfer > Duration::ZERO);
        assert!(curve.measurement.timings.verify_mint > Duration::ZERO);
        assert!(curve.measurement.timings.verify_transfer > Duration::ZERO);
        assert!(curve.measurement.timings.decapsulate > Duration::ZERO);
    }
    // every_operation_is_timed_on_every_concrete_report
}

/// Contract, carried: the measurement carries the iteration count its
///   means were computed from — each curve's `iterations` equals the
///   configured `measurement.iterations` and the report's `iterations`
///   equals the measurement's.
/// Arrange: the shared configuration with `measurement.iterations` of
///   `5` and a file report destination.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, each curve's and document's count equal to `5`.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn the_measurement_carries_the_iteration_count_its_means_were_computed_from() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let mut selections = build_admitted_selections(AdmittedSelectionsOverrides::default());
    selections.curves = vec![
        curve_of("curve-a", concretes[0]),
        curve_of("curve-b", concretes[1]),
    ];
    selections.benchmark_concretes = concretes.iter().map(|c| admitted_pairing_of(*c)).collect();
    selections.precompile_encodings = PRECOMPILE_ENCODINGS.to_vec();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            vec![],
            vec![],
            HarnessConfigurationOverrides {
                selections: Some(selections),
                measurement: Some(harness_crypto::MeasurementSettings {
                    iterations: NonZeroU32::new(5).unwrap(),
                    conditions: build_report_text(ReportTextConstructorParamsOverrides {
                        text: Some("conditions".to_string()),
                    }),
                }),
                ..Default::default()
            },
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for curve in &success.curves {
        assert_eq!(curve.measurement.iterations.get(), 5);
        let document: Value =
            serde_json::from_str(&curve.report.document).expect("the document is JSON");
        assert_eq!(document["iterations"], Value::from(5));
    }
    // the_measurement_carries_the_iteration_count_its_means_were_computed_from
}

/// Contract, carried: every object is sized by the encodings of its
///   elements — each curve's measurement sizes reflect the encoding the
///   configuration names.
/// Arrange: the shared configuration.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, each curve's measurement sizes present and positive.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn every_object_is_sized_by_the_encodings_of_its_elements() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for curve in &success.curves {
        assert!(curve.measurement.sizes.envelope > 0);
        assert!(curve.measurement.sizes.mint_proof > 0);
        assert!(curve.measurement.sizes.transfer_proof > 0);
        assert!(curve.measurement.sizes.capsule > 0);
    }
    // every_object_is_sized_by_the_encodings_of_its_elements
}

/// Contract, carried: every accepted delivery reports the cost the
///   verifier returned — each curve's measurement holds the mock
///   verifier's L2 gas and L1 fee through the report document.
/// Arrange: the shared configuration and a file report destination.
/// Act:     `run` through the crate root.
/// Assert:  `Ok`, each curve's costs holding the verifier's values.
/// Boundary: the public call `run` and the real functions it composes.
/// Mocked:   the verifier family's mock concrete and the filesystem.
#[test]
fn every_accepted_delivery_reports_the_cost_the_verifier_returned() {
    // Arrange
    let directory = work_directory();
    let concretes = real_pairing_concretes();
    let payload = RunPayload {
        configuration: configuration(
            &directory,
            concretes.iter().map(|c| admitted_pairing_of(*c)).collect(),
            vec![
                curve_of("curve-a", concretes[0]),
                curve_of("curve-b", concretes[1]),
            ],
            HarnessConfigurationOverrides::default(),
        ),
    };

    // Act
    let result = run(&RunDeps, RunParams, payload);

    // Assert
    let success = result.expect("the run succeeds");
    for curve in &success.curves {
        assert!(curve.measurement.costs.mint.l2_execution_gas > 0);
        assert!(curve.measurement.costs.transfer.l2_execution_gas > 0);
    }
    // every_accepted_delivery_reports_the_cost_the_verifier_returned
}
