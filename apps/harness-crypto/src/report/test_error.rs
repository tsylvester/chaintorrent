#![allow(clippy::expect_used)]

use core::num::NonZeroU32;
use core::time::Duration;
use std::env::temp_dir;
use std::fs;
use std::path::Path;

use super::interface::{
    ReportDestination, ReportErrorReturn, ReportText, ReportTextTryNewErrorReturn,
};
use super::mock::{
    ReportDefaultsOverrides, ReportParamsOverrides, ReportPayloadOverrides,
    ReportTextConstructorParamsOverrides, build_report_defaults, build_report_deps,
    build_report_params, build_report_payload, build_report_text,
    build_report_text_constructor_params,
};
use super::report;
use crate::measure::provides::{
    DeliveryCostsOverrides, DeliveryMeasurementOverrides, DeliveryOperationTimingsOverrides,
    DeliverySizesOverrides, build_delivery_costs, build_delivery_measurement,
    build_delivery_operation_timings, build_delivery_sizes,
};
use crate::verifier::provides::{L1DataFee, VerifyCostOverrides, build_verify_cost};

/// Contract: Destination file → a refused write is
///   `Err(ReportErrorReturn::WriteFile { path, error })`, the error
///   unchanged.
/// Arrange: the block's directory created, and its own path as the
///   destination, a path `std::fs::write` refuses because it names a
///   directory.
/// Act:     `report`, once.
/// Assert:  the result matches `Err(ReportErrorReturn::WriteFile {
///   path, .. })` with `path` equal to the directory's path.
#[test]
fn report_returns_a_refused_file_write_with_its_path() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-report-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let payload = build_report_payload(ReportPayloadOverrides {
        measurement: Some(build_delivery_measurement(DeliveryMeasurementOverrides {
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
        })),
        curve: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        conditions: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("conditions-a".to_string()),
        })),
        defaults: Some(build_report_defaults(ReportDefaultsOverrides {
            piece_group_size: Some(NonZeroU32::MIN.saturating_add(16_383)),
            primary_curve: Some(build_report_text(ReportTextConstructorParamsOverrides {
                text: Some("curve-b".to_string()),
            })),
        })),
        ..Default::default()
    });
    // Act
    let result = report(
        &build_report_deps(Default::default()),
        build_report_params(ReportParamsOverrides {
            destinations: Some(vec![ReportDestination::File {
                path: directory.clone(),
            }]),
        }),
        payload,
    );
    // Assert
    assert!(matches!(
        &result,
        Err(ReportErrorReturn::WriteFile { path, .. }) if *path == directory
    ));
    // report_returns_a_refused_file_write_with_its_path
}

/// Contract: Ordering → a refused destination returns before any later
///   destination is written.
/// Arrange: the destinations the directory's own path, then a file beneath
///   the directory.
/// Act:     `report`, once.
/// Assert:  the result matches `Err(ReportErrorReturn::WriteFile {
///   path, .. })` with `path` equal to the directory's path, and the later
///   file does not exist.
#[test]
fn report_stops_at_the_first_refused_destination() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-report-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let later_file = directory.join("later.json");
    let later: &Path = later_file.as_path();
    let payload = build_report_payload(ReportPayloadOverrides {
        measurement: Some(build_delivery_measurement(DeliveryMeasurementOverrides {
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
        })),
        curve: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-a".to_string()),
        })),
        conditions: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("conditions-a".to_string()),
        })),
        defaults: Some(build_report_defaults(ReportDefaultsOverrides {
            piece_group_size: Some(NonZeroU32::MIN.saturating_add(16_383)),
            primary_curve: Some(build_report_text(ReportTextConstructorParamsOverrides {
                text: Some("curve-b".to_string()),
            })),
        })),
        ..Default::default()
    });
    // Act
    let result = report(
        &build_report_deps(Default::default()),
        build_report_params(ReportParamsOverrides {
            destinations: Some(vec![
                ReportDestination::File {
                    path: directory.clone(),
                },
                ReportDestination::File {
                    path: later.to_path_buf(),
                },
            ]),
        }),
        payload,
    );
    // Assert
    assert!(matches!(
        &result,
        Err(ReportErrorReturn::WriteFile { path, .. }) if *path == directory
    ));
    assert!(!later.exists());
    // report_stops_at_the_first_refused_destination
}

/// Contract: `ReportText::try_new` → a text with no character is
///   `Err(ReportTextTryNewErrorReturn::Empty)`.
/// Arrange: `build_report_text_constructor_params` over `""`.
/// Act:     `ReportText::try_new`, once.
/// Assert:  the result equals `Err(ReportTextTryNewErrorReturn::Empty)`.
#[test]
fn report_text_try_new_refuses_an_empty_text() {
    // Arrange
    let params = build_report_text_constructor_params(ReportTextConstructorParamsOverrides {
        text: Some("".to_string()),
    });
    // Act
    let refused = ReportText::try_new(params);
    // Assert
    assert_eq!(refused, Err(ReportTextTryNewErrorReturn::Empty));
    // report_text_try_new_refuses_an_empty_text
}

/// Contract: `ReportText::try_new` → a text of whitespace alone is
///   `Err(ReportTextTryNewErrorReturn::Empty)`.
/// Arrange: `build_report_text_constructor_params` over `"  \t "`.
/// Act:     `ReportText::try_new`, once.
/// Assert:  the result equals `Err(ReportTextTryNewErrorReturn::Empty)`.
#[test]
fn report_text_try_new_refuses_a_text_of_whitespace_only() {
    // Arrange
    let params = build_report_text_constructor_params(ReportTextConstructorParamsOverrides {
        text: Some("  \t ".to_string()),
    });
    // Act
    let refused = ReportText::try_new(params);
    // Assert
    assert_eq!(refused, Err(ReportTextTryNewErrorReturn::Empty));
    // report_text_try_new_refuses_a_text_of_whitespace_only
}
