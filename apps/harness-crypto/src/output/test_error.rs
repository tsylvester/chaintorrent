#![allow(clippy::expect_used)]

use core::num::NonZeroU32;
use core::time::Duration;
use pairing::PairingConcrete;
use serde_json::{Value, json};
use std::env::temp_dir;
use std::fs;
use std::path::{Path, PathBuf};

use super::interface::OutputErrorReturn;
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

/// Contract: Destination file → a refused write is
///   `Err(OutputErrorReturn::WriteFile { path, error })`, the error
///   unchanged.
/// Arrange: the block's directory created, and its own path as the
///   destination, a path `std::fs::write` refuses because it names a
///   directory.
/// Act:     `output`, once.
/// Assert:  the result matches `Err(OutputErrorReturn::WriteFile {
///   path, .. })` with `path` equal to the directory's path.
#[test]
fn output_returns_a_refused_file_write_with_its_path() {
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
    let result = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![OutputDestination::File {
                path: directory.clone(),
            }]),
        }),
        payload,
    );
    // Assert
    assert!(matches!(
        &result,
        Err(OutputErrorReturn::WriteFile { path, .. }) if *path == directory
    ));
    // output_returns_a_refused_file_write_with_its_path
}

/// Contract: Destination file → a directory is not created, and the
///   refused write is returned with the path.
/// Arrange: the block's directory removed and not created, and a file path
///   beneath it as the destination.
/// Act:     `output`, once.
/// Assert:  the result matches `Err(OutputErrorReturn::WriteFile {
///   path, .. })` with `path` equal to that file path, and the directory
///   does not exist.
#[test]
fn output_returns_a_refused_write_of_a_file_beneath_a_missing_directory() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
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
    let result = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![OutputDestination::File {
                path: path.to_path_buf(),
            }]),
        }),
        payload,
    );
    // Assert
    assert!(matches!(
        &result,
        Err(OutputErrorReturn::WriteFile { path: refused, .. }) if *refused == file
    ));
    assert!(!directory.exists());
    // output_returns_a_refused_write_of_a_file_beneath_a_missing_directory
}

/// Contract: Ordering → a refused destination returns before any later
///   destination is written.
/// Arrange: the destinations the directory's own path, then a file beneath
///   the directory.
/// Act:     `output`, once.
/// Assert:  the result matches `Err(OutputErrorReturn::WriteFile {
///   path, .. })` with `path` equal to the directory's path, and the later
///   file does not exist.
#[test]
fn output_stops_at_the_first_refused_destination() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let later_file = directory.join("later.json");
    let later: &Path = later_file.as_path();
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
    let result = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![
                OutputDestination::File {
                    path: directory.clone(),
                },
                OutputDestination::File {
                    path: later.to_path_buf(),
                },
            ]),
        }),
        payload,
    );
    // Assert
    assert!(matches!(
        &result,
        Err(OutputErrorReturn::WriteFile { path, .. }) if *path == directory
    ));
    assert!(!later.exists());
    // output_stops_at_the_first_refused_destination
}

/// Contract: Ordering → a destination written before the refusal holds the
///   text.
/// Arrange: the destinations a file beneath the directory, then the
///   directory's own path.
/// Act:     `output`, once.
/// Assert:  the result matches `Err(OutputErrorReturn::WriteFile {
///   path, .. })` with `path` equal to the directory's path, and the file
///   read back equals the text the default outcome renders to.
#[test]
fn output_keeps_the_destinations_written_before_a_refused_one() {
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
    let result = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![
                OutputDestination::File {
                    path: path.to_path_buf(),
                },
                OutputDestination::File {
                    path: directory.clone(),
                },
            ]),
        }),
        payload,
    );
    // Assert
    assert!(matches!(
        &result,
        Err(OutputErrorReturn::WriteFile { path: refused, .. }) if *refused == directory
    ));
    assert_eq!(
        fs::read_to_string(path).expect("the file reads"),
        format!("{expected:#}")
    );
    let _: Value = serde_json::from_str(&format!("{expected:#}")).expect("the literal parses");
    // output_keeps_the_destinations_written_before_a_refused_one
}

/// Contract: Computed → a run's refusal is written like any outcome, and a
///   refused write of it is the write error.
/// Arrange: the outcome
///   `Err(RunErrorReturn::Report(ReportErrorReturn::WriteStandardOutput(
///   std::io::Error::other("refused"))))` and the directory's own path as
///   the destination.
/// Act:     `output`, once.
/// Assert:  the result matches `Err(OutputErrorReturn::WriteFile {
///   path, .. })` with `path` equal to the directory's path.
#[test]
fn output_returns_the_refusal_of_a_write_for_a_refused_run_as_a_write_error() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-output-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let payload = build_output_payload(OutputPayloadOverrides {
        outcome: Some(Err(RunErrorReturn::Report(
            ReportErrorReturn::WriteStandardOutput(std::io::Error::other("refused")),
        ))),
    });
    // Act
    let result = output(
        &build_output_deps(Default::default()),
        build_output_params(OutputParamsOverrides {
            destinations: Some(vec![OutputDestination::File {
                path: directory.clone(),
            }]),
        }),
        payload,
    );
    // Assert
    assert!(matches!(
        &result,
        Err(OutputErrorReturn::WriteFile { path, .. }) if *path == directory
    ));
    // output_returns_the_refusal_of_a_write_for_a_refused_run_as_a_write_error
}
