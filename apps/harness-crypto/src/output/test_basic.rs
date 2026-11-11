#![allow(clippy::expect_used)]

use core::num::NonZeroU32;
use core::time::Duration;
use pairing::PairingConcrete;
use serde_json::{Value, json};
use std::env::temp_dir;
use std::fs;
use std::path::{Path, PathBuf};

use super::mock::{
    OutputParamsOverrides, OutputPayloadOverrides, build_output_deps, build_output_params,
    build_output_payload,
};
use super::output;
use crate::benchmark::provides::{
    PairingBenchmarkSuccessReturnOverrides, PairingOperationTimingsOverrides,
    build_pairing_benchmark_success_return, build_pairing_operation_timings,
};
use crate::config::provides::OutputDestination;
use crate::generate::provides::{EmittedFileOverrides, VectorKind, build_emitted_file};
use crate::measure::provides::{
    DeliveryCostsOverrides, DeliveryMeasurementOverrides, DeliveryOperationTimingsOverrides,
    DeliverySizesOverrides, build_delivery_costs, build_delivery_measurement,
    build_delivery_operation_timings, build_delivery_sizes,
};
use crate::report::provides::{
    ReportErrorReturn, ReportSuccessReturnOverrides, ReportTextConstructorParamsOverrides,
    build_report_success_return, build_report_text,
};
use crate::run::provides::{
    RunCurveOutcomeOverrides, RunErrorReturn, RunSuccessReturnOverrides, build_run_curve_outcome,
    build_run_success_return,
};
use crate::verifier::provides::{L1DataFee, VerifyCostOverrides, build_verify_cost};

/// Contract: Render success, benchmark, curve, and measurement → the text
///   is one JSON document holding the outcome, the benchmark, and the
///   curve's label, files, measurement, and report document.
/// Arrange: the defaults of this element.
/// Act:     `output`, once.
/// Assert:  the result is `Ok` and its `text` parsed equals the expected
///   default document.
#[test]
fn output_renders_a_successful_outcome_as_one_json_document() {
    // Arrange
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    let expected = json!({
        "outcome": "succeeded",
        "benchmarks": [
            {
                "concrete": "Bn254Arkworks",
                "iterations": 3,
                "timings": {
                    "mul_g1": { "seconds": 1, "subsec_nanos": 0 },
                    "mul_g2": { "seconds": 0, "subsec_nanos": 2000000 },
                    "msm_g1": { "seconds": 0, "subsec_nanos": 3 },
                    "msm_g2": { "seconds": 4, "subsec_nanos": 0 },
                    "pairing_product": { "seconds": 0, "subsec_nanos": 5000000 }
                }
            }
        ],
        "curves": [
            {
                "label": "curve-a",
                "files": [ { "kind": "G1Addition", "path": "g1_add.vectors" } ],
                "measurement": {
                    "iterations": 3,
                    "timings": {
                        "prove_mint": { "seconds": 1, "subsec_nanos": 1 },
                        "prove_transfer": { "seconds": 2, "subsec_nanos": 2 },
                        "verify_mint": { "seconds": 3, "subsec_nanos": 3 },
                        "verify_transfer": { "seconds": 4, "subsec_nanos": 4 },
                        "decapsulate": { "seconds": 5, "subsec_nanos": 5 }
                    },
                    "sizes_bytes": {
                        "capsule": 11,
                        "envelope": 22,
                        "mint_proof": 33,
                        "transfer_proof": 44
                    },
                    "costs": {
                        "mint": { "l2_execution_gas": 100, "l1_data_fee": { "state": "charged", "wei": "7" } },
                        "transfer": { "l2_execution_gas": 200, "l1_data_fee": { "state": "charged", "wei": "8" } }
                    }
                },
                "report": "{\n  \"curve\": \"curve-a\"\n}"
            }
        ]
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed, expected);
    // output_renders_a_successful_outcome_as_one_json_document
}

/// Contract: Render benchmark → one object per entry in the order held,
///   each holding its own concrete, iterations, and timings.
/// Arrange: two benchmarks, the default and a second over the concrete
///   `PairingConcrete::Bls12381Arkworks`, the iterations `NonZeroU32::MIN`,
///   and timings distinct from the first's, in that order, and no curve.
/// Act:     `output`, once.
/// Assert:  the parsed document's `benchmarks` equals the array of the two
///   objects in that order and its `curves` is the empty array.
#[test]
fn output_renders_the_benchmarks_in_the_order_the_outcome_holds_them() {
    // Arrange
    let first = build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_pairing_operation_timings(
            PairingOperationTimingsOverrides {
                mul_g1: Some(Duration::from_secs(1)),
                mul_g2: Some(Duration::from_millis(2)),
                msm_g1: Some(Duration::from_nanos(3)),
                msm_g2: Some(Duration::from_secs(4)),
                pairing_product: Some(Duration::from_millis(5)),
            },
        )),
    });
    let second = build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        iterations: Some(NonZeroU32::MIN),
        timings: Some(build_pairing_operation_timings(
            PairingOperationTimingsOverrides {
                mul_g1: Some(Duration::from_secs(6)),
                mul_g2: Some(Duration::from_millis(7)),
                msm_g1: Some(Duration::from_nanos(8)),
                msm_g2: Some(Duration::from_secs(9)),
                pairing_product: Some(Duration::from_millis(10)),
            },
        )),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![first, second]),
            curves: Some(vec![]),
        }))),
    });
    let expected = json!([
        {
            "concrete": "Bn254Arkworks",
            "iterations": 3,
            "timings": {
                "mul_g1": { "seconds": 1, "subsec_nanos": 0 },
                "mul_g2": { "seconds": 0, "subsec_nanos": 2000000 },
                "msm_g1": { "seconds": 0, "subsec_nanos": 3 },
                "msm_g2": { "seconds": 4, "subsec_nanos": 0 },
                "pairing_product": { "seconds": 0, "subsec_nanos": 5000000 }
            }
        },
        {
            "concrete": "Bls12381Arkworks",
            "iterations": 1,
            "timings": {
                "mul_g1": { "seconds": 6, "subsec_nanos": 0 },
                "mul_g2": { "seconds": 0, "subsec_nanos": 7000000 },
                "msm_g1": { "seconds": 0, "subsec_nanos": 8 },
                "msm_g2": { "seconds": 9, "subsec_nanos": 0 },
                "pairing_product": { "seconds": 0, "subsec_nanos": 10000000 }
            }
        }
    ]);
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed["benchmarks"], expected);
    assert_eq!(parsed["curves"], json!([]));
    // output_renders_the_benchmarks_in_the_order_the_outcome_holds_them
}

/// Contract: Render duration → a duration is its `as_secs` and
///   `subsec_nanos`.
/// Arrange: the defaults of this element, whose benchmark and measurement
///   timings each hold distinct seconds and nanoseconds.
/// Act:     `output`, once.
/// Assert:  the parsed document's `benchmarks[0].timings` and
///   `curves[0].measurement.timings` equal the expected default document's.
#[test]
fn output_renders_each_duration_as_whole_seconds_and_subsecond_nanoseconds() {
    // Arrange
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    let expected = json!({
        "outcome": "succeeded",
        "benchmarks": [
            {
                "concrete": "Bn254Arkworks",
                "iterations": 3,
                "timings": {
                    "mul_g1": { "seconds": 1, "subsec_nanos": 0 },
                    "mul_g2": { "seconds": 0, "subsec_nanos": 2000000 },
                    "msm_g1": { "seconds": 0, "subsec_nanos": 3 },
                    "msm_g2": { "seconds": 4, "subsec_nanos": 0 },
                    "pairing_product": { "seconds": 0, "subsec_nanos": 5000000 }
                }
            }
        ],
        "curves": [
            {
                "label": "curve-a",
                "files": [ { "kind": "G1Addition", "path": "g1_add.vectors" } ],
                "measurement": {
                    "iterations": 3,
                    "timings": {
                        "prove_mint": { "seconds": 1, "subsec_nanos": 1 },
                        "prove_transfer": { "seconds": 2, "subsec_nanos": 2 },
                        "verify_mint": { "seconds": 3, "subsec_nanos": 3 },
                        "verify_transfer": { "seconds": 4, "subsec_nanos": 4 },
                        "decapsulate": { "seconds": 5, "subsec_nanos": 5 }
                    },
                    "sizes_bytes": {
                        "capsule": 11,
                        "envelope": 22,
                        "mint_proof": 33,
                        "transfer_proof": 44
                    },
                    "costs": {
                        "mint": { "l2_execution_gas": 100, "l1_data_fee": { "state": "charged", "wei": "7" } },
                        "transfer": { "l2_execution_gas": 200, "l1_data_fee": { "state": "charged", "wei": "8" } }
                    }
                },
                "report": "{\n  \"curve\": \"curve-a\"\n}"
            }
        ]
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(
        parsed["benchmarks"][0]["timings"],
        expected["benchmarks"][0]["timings"]
    );
    assert_eq!(
        parsed["curves"][0]["measurement"]["timings"],
        expected["curves"][0]["measurement"]["timings"]
    );
    // output_renders_each_duration_as_whole_seconds_and_subsecond_nanoseconds
}

/// Contract: Render curve → one object per entry in the order held, each
///   holding its own label, files, measurement, and report document.
/// Arrange: two curves, the default and a second over the label `"curve-b"`,
///   two files of distinct paths, a measurement of iterations
///   `NonZeroU32::MIN`, and the report document `"second"`, in that order,
///   and no benchmark.
/// Act:     `output`, once.
/// Assert:  the parsed document's `curves` equals the array of the first
///   curve's object and the second curve's object in that order, the second
///   curve's `files` in the order held, and its `benchmarks` is the empty
///   array.
#[test]
fn output_renders_the_curves_in_the_order_the_outcome_holds_them_each_with_its_own_values() {
    // Arrange
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let first = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let second = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-b".to_string()),
        })),
        files: Some(vec![
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::G1Addition)),
                path: Some(PathBuf::from("b_one.vectors")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::G1Addition)),
                path: Some(PathBuf::from("b_two.vectors")),
            }),
        ]),
        measurement: Some(build_delivery_measurement(DeliveryMeasurementOverrides {
            iterations: Some(NonZeroU32::MIN),
            timings: Some(build_delivery_operation_timings(
                DeliveryOperationTimingsOverrides {
                    prove_mint: Some(Duration::new(1, 1)),
                    prove_transfer: Some(Duration::new(2, 2)),
                    verify_mint: Some(Duration::new(3, 3)),
                    verify_transfer: Some(Duration::new(4, 4)),
                    decapsulate: Some(Duration::new(5, 5)),
                },
            )),
            sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
                capsule: Some(11),
                envelope: Some(22),
                mint_proof: Some(33),
                transfer_proof: Some(44),
            })),
            costs: Some(build_delivery_costs(DeliveryCostsOverrides {
                mint: Some(build_verify_cost(VerifyCostOverrides {
                    l2_execution_gas: Some(100),
                    l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
                })),
                transfer: Some(build_verify_cost(VerifyCostOverrides {
                    l2_execution_gas: Some(200),
                    l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
                })),
            })),
        })),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("second".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![]),
            curves: Some(vec![first, second]),
        }))),
    });
    let expected = json!([
        {
            "label": "curve-a",
            "files": [ { "kind": "G1Addition", "path": "g1_add.vectors" } ],
            "measurement": {
                "iterations": 3,
                "timings": {
                    "prove_mint": { "seconds": 1, "subsec_nanos": 1 },
                    "prove_transfer": { "seconds": 2, "subsec_nanos": 2 },
                    "verify_mint": { "seconds": 3, "subsec_nanos": 3 },
                    "verify_transfer": { "seconds": 4, "subsec_nanos": 4 },
                    "decapsulate": { "seconds": 5, "subsec_nanos": 5 }
                },
                "sizes_bytes": {
                    "capsule": 11,
                    "envelope": 22,
                    "mint_proof": 33,
                    "transfer_proof": 44
                },
                "costs": {
                    "mint": { "l2_execution_gas": 100, "l1_data_fee": { "state": "charged", "wei": "7" } },
                    "transfer": { "l2_execution_gas": 200, "l1_data_fee": { "state": "charged", "wei": "8" } }
                }
            },
            "report": "{\n  \"curve\": \"curve-a\"\n}"
        },
        {
            "label": "curve-b",
            "files": [
                { "kind": "G1Addition", "path": "b_one.vectors" },
                { "kind": "G1Addition", "path": "b_two.vectors" }
            ],
            "measurement": {
                "iterations": 1,
                "timings": {
                    "prove_mint": { "seconds": 1, "subsec_nanos": 1 },
                    "prove_transfer": { "seconds": 2, "subsec_nanos": 2 },
                    "verify_mint": { "seconds": 3, "subsec_nanos": 3 },
                    "verify_transfer": { "seconds": 4, "subsec_nanos": 4 },
                    "decapsulate": { "seconds": 5, "subsec_nanos": 5 }
                },
                "sizes_bytes": {
                    "capsule": 11,
                    "envelope": 22,
                    "mint_proof": 33,
                    "transfer_proof": 44
                },
                "costs": {
                    "mint": { "l2_execution_gas": 100, "l1_data_fee": { "state": "charged", "wei": "7" } },
                    "transfer": { "l2_execution_gas": 200, "l1_data_fee": { "state": "charged", "wei": "8" } }
                }
            },
            "report": "second"
        }
    ]);
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed["curves"], expected);
    assert_eq!(parsed["benchmarks"], json!([]));
    // output_renders_the_curves_in_the_order_the_outcome_holds_them_each_with_its_own_values
}

/// Contract: Render file → `kind` is the `Debug` text of a `Some` kind and
///   the JSON `null` for `None`, and `path` is the path's text.
/// Arrange: the default curve over two files, one over `build_emitted_file`
///   with the kind `None` and the path `"library.sol"` and one over the
///   kind `Some(VectorKind::G1Addition)`.
/// Act:     `output`, once.
/// Assert:  the parsed document's `curves[0].files` equals
///   `json!([ { "kind": null, "path": "library.sol" },
///   { "kind": "G1Addition", "path": "g1_add.vectors" } ])`.
#[test]
fn output_renders_a_file_with_no_kind_as_null_and_a_file_with_a_kind_as_its_debug_text() {
    // Arrange
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![
            build_emitted_file(EmittedFileOverrides {
                kind: Some(None),
                path: Some(PathBuf::from("library.sol")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::G1Addition)),
                path: Some(PathBuf::from("g1_add.vectors")),
            }),
        ]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(
        parsed["curves"][0]["files"],
        json!([
            { "kind": null, "path": "library.sol" },
            { "kind": "G1Addition", "path": "g1_add.vectors" }
        ])
    );
    // output_renders_a_file_with_no_kind_as_null_and_a_file_with_a_kind_as_its_debug_text
}

/// Contract: Render file → a curve with no file has an empty `files`
///   array.
/// Arrange: the default curve over no file.
/// Act:     `output`, once.
/// Assert:  the parsed document's `curves[0].files` equals `json!([])`.
#[test]
fn output_renders_a_curve_that_wrote_no_file_with_an_empty_files_array() {
    // Arrange
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed["curves"][0]["files"], json!([]));
    // output_renders_a_curve_that_wrote_no_file_with_an_empty_files_array
}

/// Contract: Render curve → `report` is the document as handed, line feeds
///   included, as one JSON string.
/// Arrange: the default curve with the document `"a\nb\n"`.
/// Act:     `output`, once.
/// Assert:  the parsed document's `curves[0].report` equals
///   `json!("a\nb\n")`.
#[test]
fn output_renders_the_report_document_as_one_string_verbatim() {
    // Arrange
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("a\nb\n".to_string()),
        })),
    });
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed["curves"][0]["report"], json!("a\nb\n"));
    // output_renders_the_report_document_as_one_string_verbatim
}

/// Contract: Render measurement and fee → each size under its own key,
///   each delivery's gas, and each charged fee as the decimal text of its
///   wei.
/// Arrange: the defaults of this element.
/// Act:     `output`, once.
/// Assert:  the parsed document's `curves[0].measurement.sizes_bytes` and
///   `curves[0].measurement.costs` equal the expected default document's.
#[test]
fn output_renders_the_sizes_and_the_costs_of_a_measurement() {
    // Arrange
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    let expected = json!({
        "outcome": "succeeded",
        "benchmarks": [
            {
                "concrete": "Bn254Arkworks",
                "iterations": 3,
                "timings": {
                    "mul_g1": { "seconds": 1, "subsec_nanos": 0 },
                    "mul_g2": { "seconds": 0, "subsec_nanos": 2000000 },
                    "msm_g1": { "seconds": 0, "subsec_nanos": 3 },
                    "msm_g2": { "seconds": 4, "subsec_nanos": 0 },
                    "pairing_product": { "seconds": 0, "subsec_nanos": 5000000 }
                }
            }
        ],
        "curves": [
            {
                "label": "curve-a",
                "files": [ { "kind": "G1Addition", "path": "g1_add.vectors" } ],
                "measurement": {
                    "iterations": 3,
                    "timings": {
                        "prove_mint": { "seconds": 1, "subsec_nanos": 1 },
                        "prove_transfer": { "seconds": 2, "subsec_nanos": 2 },
                        "verify_mint": { "seconds": 3, "subsec_nanos": 3 },
                        "verify_transfer": { "seconds": 4, "subsec_nanos": 4 },
                        "decapsulate": { "seconds": 5, "subsec_nanos": 5 }
                    },
                    "sizes_bytes": {
                        "capsule": 11,
                        "envelope": 22,
                        "mint_proof": 33,
                        "transfer_proof": 44
                    },
                    "costs": {
                        "mint": { "l2_execution_gas": 100, "l1_data_fee": { "state": "charged", "wei": "7" } },
                        "transfer": { "l2_execution_gas": 200, "l1_data_fee": { "state": "charged", "wei": "8" } }
                    }
                },
                "report": "{\n  \"curve\": \"curve-a\"\n}"
            }
        ]
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(
        parsed["curves"][0]["measurement"]["sizes_bytes"],
        expected["curves"][0]["measurement"]["sizes_bytes"]
    );
    assert_eq!(
        parsed["curves"][0]["measurement"]["costs"],
        expected["curves"][0]["measurement"]["costs"]
    );
    // output_renders_the_sizes_and_the_costs_of_a_measurement
}

/// Contract: Render fee → `L1DataFee::NotApplicable` is
///   `{ "state": "not_applicable" }`, never a zero.
/// Arrange: the default curve whose mint cost holds
///   `L1DataFee::NotApplicable`.
/// Act:     `output`, once.
/// Assert:  the parsed document's `curves[0].measurement.costs.mint
///   .l1_data_fee` equals `json!({ "state": "not_applicable" })`.
#[test]
fn output_renders_a_fee_the_chain_does_not_charge_as_its_own_state() {
    // Arrange
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::NotApplicable),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(
        parsed["curves"][0]["measurement"]["costs"]["mint"]["l1_data_fee"],
        json!({ "state": "not_applicable" })
    );
    // output_renders_a_fee_the_chain_does_not_charge_as_its_own_state
}

/// Contract: Render → no value is compared with a bound, clamped, or
///   refused.
/// Arrange: a measurement of the timing `Duration::MAX` for `prove_mint`,
///   the size `usize::MAX` for `capsule`, and a mint cost of gas `u64::MAX`
///   and `L1DataFee::Charged { wei: u128::MAX }`, and a benchmark iteration
///   count of `NonZeroU32::MAX`.
/// Act:     `output`, once.
/// Assert:  the parsed document's `curves[0].measurement.timings
///   .prove_mint` equals `json!({ "seconds": u64::MAX, "subsec_nanos":
///   999999999 })`, `sizes_bytes.capsule` equals `usize::MAX` as a JSON
///   number, `costs.mint` equals `json!({ "l2_execution_gas": u64::MAX,
///   "l1_data_fee": { "state": "charged", "wei":
///   "340282366920938463463374607431768211455" } })`, and
///   `benchmarks[0].iterations` equals `u32::MAX` as a JSON number.
#[test]
fn output_renders_every_value_verbatim_whatever_its_magnitude() {
    // Arrange
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MAX),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::MAX),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(usize::MAX),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(u64::MAX),
                l1_data_fee: Some(L1DataFee::Charged { wei: u128::MAX }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(
        parsed["curves"][0]["measurement"]["timings"]["prove_mint"],
        json!({ "seconds": u64::MAX, "subsec_nanos": 999_999_999 })
    );
    assert_eq!(
        parsed["curves"][0]["measurement"]["sizes_bytes"]["capsule"],
        json!(usize::MAX)
    );
    assert_eq!(
        parsed["curves"][0]["measurement"]["costs"]["mint"],
        json!({
            "l2_execution_gas": u64::MAX,
            "l1_data_fee": {
                "state": "charged",
                "wei": "340282366920938463463374607431768211455"
            }
        })
    );
    assert_eq!(parsed["benchmarks"][0]["iterations"], json!(u32::MAX));
    // output_renders_every_value_verbatim_whatever_its_magnitude
}

/// Contract: Render success → an outcome with no entry is the outcome with
///   empty arrays.
/// Arrange: `Ok(build_run_success_return)` over no benchmark and no curve.
/// Act:     `output`, once.
/// Assert:  the parsed document equals
///   `json!({ "outcome": "succeeded", "benchmarks": [], "curves": [] })`.
#[test]
fn output_renders_a_successful_outcome_that_holds_no_benchmark_and_no_curve() {
    // Arrange
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![]),
            curves: Some(vec![]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(
        parsed,
        json!({ "outcome": "succeeded", "benchmarks": [], "curves": [] })
    );
    // output_renders_a_successful_outcome_that_holds_no_benchmark_and_no_curve
}

/// Contract: Render refusal → `outcome` is `"refused"` and `refusal` is the
///   whole `Debug` text of the refusal.
/// Arrange: the outcome
///   `Err(RunErrorReturn::Report(ReportErrorReturn::WriteStandardOutput(
///   std::io::Error::other("refused"))))` and, for the expected document,
///   the `Debug` text of a second value constructed the same way.
/// Act:     `output`, once.
/// Assert:  the result is `Ok` and its `text` parsed equals
///   `json!({ "outcome": "refused", "refusal": <that text> })`.
#[test]
fn output_renders_a_refusal_as_the_debug_text_of_the_refusal() {
    // Arrange
    let expected_refusal = format!(
        "{:?}",
        RunErrorReturn::Report(ReportErrorReturn::WriteStandardOutput(
            std::io::Error::other("refused")
        ))
    );
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Err(RunErrorReturn::Report(
            ReportErrorReturn::WriteStandardOutput(std::io::Error::other("refused")),
        ))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(
        parsed,
        json!({ "outcome": "refused", "refusal": expected_refusal })
    );
    // output_renders_a_refusal_as_the_debug_text_of_the_refusal
}

/// Contract: No destination → the text is returned and nothing is written.
/// Arrange: the block's directory created and no destination.
/// Act:     `output`, once.
/// Assert:  the result is `Ok` with a non-empty text, and the directory
///   holds no entry.
#[test]
fn output_with_no_destination_writes_nothing_and_returns_the_text() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    assert!(!returned.text.is_empty());
    assert!(
        fs::read_dir(&directory)
            .expect("the directory reads")
            .next()
            .is_none()
    );
    // output_with_no_destination_writes_nothing_and_returns_the_text
}

/// Contract: Destination file → the file at the path holds the returned
///   text.
/// Arrange: the block's directory and one `OutputDestination::File`
///   beneath it.
/// Act:     `output`, once.
/// Assert:  `std::fs::read_to_string` of the path equals the returned text,
///   and that text parses as JSON.
#[test]
fn output_writes_the_text_to_a_file_destination() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("outcome.json");
    let path: &Path = file.as_path();
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![OutputDestination::File {
                path: path.to_path_buf(),
            }]),
        }),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    assert_eq!(
        fs::read_to_string(path).expect("the file reads"),
        returned.text
    );
    let _: Value = serde_json::from_str(&returned.text).expect("the text parses");
    // output_writes_the_text_to_a_file_destination
}

/// Contract: Destination file → every destination receives the same text.
/// Arrange: two `OutputDestination::File` destinations of distinct paths
///   beneath the block's directory.
/// Act:     `output`, once.
/// Assert:  both files read back equal the returned text.
#[test]
fn output_writes_the_same_text_to_each_destination() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let first_file = directory.join("first.json");
    let first: &Path = first_file.as_path();
    let second_file = directory.join("second.json");
    let second: &Path = second_file.as_path();
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![
                OutputDestination::File {
                    path: first.to_path_buf(),
                },
                OutputDestination::File {
                    path: second.to_path_buf(),
                },
            ]),
        }),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    assert_eq!(
        fs::read_to_string(first).expect("the first file reads"),
        returned.text
    );
    assert_eq!(
        fs::read_to_string(second).expect("the second file reads"),
        returned.text
    );
    // output_writes_the_same_text_to_each_destination
}

/// Contract: Destination file → a file already holding text holds the
///   returned text alone afterward.
/// Arrange: the block's directory, a file beneath it written with the text
///   `"previous"`, and that path as the destination.
/// Act:     `output`, once.
/// Assert:  the file read back equals the returned text.
#[test]
fn output_replaces_the_content_of_an_existing_file() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("outcome.json");
    let path: &Path = file.as_path();
    fs::write(path, "previous").expect("the prior content writes");
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![OutputDestination::File {
                path: path.to_path_buf(),
            }]),
        }),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    assert_eq!(
        fs::read_to_string(path).expect("the file reads"),
        returned.text
    );
    // output_replaces_the_content_of_an_existing_file
}

/// Contract: Destination standard output → the write succeeds and the text
///   is returned.
/// Arrange: one `OutputDestination::StandardOutput`.
/// Act:     `output`, once.
/// Assert:  the result is `Ok` and its `text` parsed equals the expected
///   default document.
#[test]
fn output_returns_the_text_when_it_writes_to_standard_output() {
    // Arrange
    let benchmark =
        build_pairing_benchmark_success_return(PairingBenchmarkSuccessReturnOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            iterations: Some(NonZeroU32::MIN.saturating_add(2)),
            timings: Some(build_pairing_operation_timings(
                PairingOperationTimingsOverrides {
                    mul_g1: Some(Duration::from_secs(1)),
                    mul_g2: Some(Duration::from_millis(2)),
                    msm_g1: Some(Duration::from_nanos(3)),
                    msm_g2: Some(Duration::from_secs(4)),
                    pairing_product: Some(Duration::from_millis(5)),
                },
            )),
        });
    let measurement = build_delivery_measurement(DeliveryMeasurementOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        timings: Some(build_delivery_operation_timings(
            DeliveryOperationTimingsOverrides {
                prove_mint: Some(Duration::new(1, 1)),
                prove_transfer: Some(Duration::new(2, 2)),
                verify_mint: Some(Duration::new(3, 3)),
                verify_transfer: Some(Duration::new(4, 4)),
                decapsulate: Some(Duration::new(5, 5)),
            },
        )),
        sizes: Some(build_delivery_sizes(DeliverySizesOverrides {
            capsule: Some(11),
            envelope: Some(22),
            mint_proof: Some(33),
            transfer_proof: Some(44),
        })),
        costs: Some(build_delivery_costs(DeliveryCostsOverrides {
            mint: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(100),
                l1_data_fee: Some(L1DataFee::Charged { wei: 7 }),
            })),
            transfer: Some(build_verify_cost(VerifyCostOverrides {
                l2_execution_gas: Some(200),
                l1_data_fee: Some(L1DataFee::Charged { wei: 8 }),
            })),
        })),
    });
    let curve = build_run_curve_outcome(RunCurveOutcomeOverrides {
        label: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        files: Some(vec![build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("g1_add.vectors")),
        })]),
        measurement: Some(measurement),
        report: Some(build_report_success_return(ReportSuccessReturnOverrides {
            document: Some("{\n  \"curve\": \"curve-a\"\n}".to_string()),
        })),
    });
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Ok(build_run_success_return(RunSuccessReturnOverrides {
            benchmarks: Some(vec![benchmark]),
            curves: Some(vec![curve]),
        }))),
    });
    let expected = json!({
        "outcome": "succeeded",
        "benchmarks": [
            {
                "concrete": "Bn254Arkworks",
                "iterations": 3,
                "timings": {
                    "mul_g1": { "seconds": 1, "subsec_nanos": 0 },
                    "mul_g2": { "seconds": 0, "subsec_nanos": 2000000 },
                    "msm_g1": { "seconds": 0, "subsec_nanos": 3 },
                    "msm_g2": { "seconds": 4, "subsec_nanos": 0 },
                    "pairing_product": { "seconds": 0, "subsec_nanos": 5000000 }
                }
            }
        ],
        "curves": [
            {
                "label": "curve-a",
                "files": [ { "kind": "G1Addition", "path": "g1_add.vectors" } ],
                "measurement": {
                    "iterations": 3,
                    "timings": {
                        "prove_mint": { "seconds": 1, "subsec_nanos": 1 },
                        "prove_transfer": { "seconds": 2, "subsec_nanos": 2 },
                        "verify_mint": { "seconds": 3, "subsec_nanos": 3 },
                        "verify_transfer": { "seconds": 4, "subsec_nanos": 4 },
                        "decapsulate": { "seconds": 5, "subsec_nanos": 5 }
                    },
                    "sizes_bytes": {
                        "capsule": 11,
                        "envelope": 22,
                        "mint_proof": 33,
                        "transfer_proof": 44
                    },
                    "costs": {
                        "mint": { "l2_execution_gas": 100, "l1_data_fee": { "state": "charged", "wei": "7" } },
                        "transfer": { "l2_execution_gas": 200, "l1_data_fee": { "state": "charged", "wei": "8" } }
                    }
                },
                "report": "{\n  \"curve\": \"curve-a\"\n}"
            }
        ]
    });
    // Act
    let returned = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![OutputDestination::StandardOutput]),
        }),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.text).expect("the text parses");
    assert_eq!(parsed, expected);
    // output_returns_the_text_when_it_writes_to_standard_output
}
