#![allow(clippy::expect_used)]

use core::num::NonZeroU32;
use core::time::Duration;
use serde_json::{Value, json};
use std::env::temp_dir;
use std::path::Path;

use super::interface::{ReportDestination, ReportText};
use super::mock::{
    ReportAdjustmentOverrides, ReportDefaultsOverrides, ReportParamsOverrides,
    ReportPayloadOverrides, ReportTextConstructorParamsOverrides, build_report_adjustment,
    build_report_defaults, build_report_deps, build_report_params, build_report_payload,
    build_report_text, build_report_text_constructor_params,
};
use super::report;
use crate::measure::provides::{
    DeliveryCostsOverrides, DeliveryMeasurementOverrides, DeliveryOperationTimingsOverrides,
    DeliverySizesOverrides, build_delivery_costs, build_delivery_measurement,
    build_delivery_operation_timings, build_delivery_sizes,
};
use crate::verifier::provides::{L1DataFee, VerifyCostOverrides, build_verify_cost};

/// Contract: Render → the document's `curve`, `conditions`, and `iterations`
///   are the label, the conditions text, and the count the payload holds.
/// Arrange: the defaults of this element; no destination.
/// Act:     `report`, once.
/// Assert:  the returned document parsed equals the expected literal.
#[test]
fn report_records_the_curve_the_conditions_and_the_iteration_count() {
    // Arrange
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
    let expected = json!({
        "curve": "curve-a",
        "conditions": "conditions-a",
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
        },
        "defaults_in_force": { "piece_group_size_bytes": 16384, "primary_curve": "curve-b" },
        "adjustment": null
    });
    // Act
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(parsed, expected);
    // report_records_the_curve_the_conditions_and_the_iteration_count
}

/// Contract: Render → a timing is its `Duration`'s `as_secs` and
///   `subsec_nanos`.
/// Arrange: the five timings of this element's defaults, each with distinct
///   seconds and nanoseconds.
/// Act:     `report`, once.
/// Assert:  the parsed document's `timings` equals the expected literal's
///   `timings`.
#[test]
fn report_records_each_timing_as_whole_seconds_and_subsecond_nanoseconds() {
    // Arrange
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
    let expected = json!({
        "curve": "curve-a",
        "conditions": "conditions-a",
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
        },
        "defaults_in_force": { "piece_group_size_bytes": 16384, "primary_curve": "curve-b" },
        "adjustment": null
    });
    // Act
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(parsed["timings"], expected["timings"]);
    // report_records_each_timing_as_whole_seconds_and_subsecond_nanoseconds
}

/// Contract: Render → each of the four sizes is recorded under its own key.
/// Arrange: the four sizes of this element's defaults.
/// Act:     `report`, once.
/// Assert:  the parsed document's `sizes_bytes` equals the expected literal's
///   `sizes_bytes`.
#[test]
fn report_records_each_size_in_bytes() {
    // Arrange
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
    let expected = json!({
        "curve": "curve-a",
        "conditions": "conditions-a",
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
        },
        "defaults_in_force": { "piece_group_size_bytes": 16384, "primary_curve": "curve-b" },
        "adjustment": null
    });
    // Act
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(parsed["sizes_bytes"], expected["sizes_bytes"]);
    // report_records_each_size_in_bytes
}

/// Contract: Render and Fee → each delivery's gas and its charged fee as
///   decimal text of its wei.
/// Arrange: the two costs of this element's defaults.
/// Act:     `report`, once.
/// Assert:  the parsed document's `costs` equals the expected literal's
///   `costs`.
#[test]
fn report_records_the_cost_of_each_delivery_with_its_charged_l1_fee_in_wei() {
    // Arrange
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
    let expected = json!({
        "curve": "curve-a",
        "conditions": "conditions-a",
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
        },
        "defaults_in_force": { "piece_group_size_bytes": 16384, "primary_curve": "curve-b" },
        "adjustment": null
    });
    // Act
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(parsed["costs"], expected["costs"]);
    // report_records_the_cost_of_each_delivery_with_its_charged_l1_fee_in_wei
}

/// Contract: Fee → `L1DataFee::NotApplicable` is
///   `{ "state": "not_applicable" }`, never a zero.
/// Arrange: the mint's cost with `L1DataFee::NotApplicable`.
/// Act:     `report`, once.
/// Assert:  the parsed document's `costs.mint.l1_data_fee` equals
///   `json!({ "state": "not_applicable" })`.
#[test]
fn report_records_a_fee_the_chain_does_not_charge_as_its_own_state() {
    // Arrange
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
                    l1_data_fee: Some(L1DataFee::NotApplicable),
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
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(
        parsed["costs"]["mint"]["l1_data_fee"],
        json!({ "state": "not_applicable" })
    );
    // report_records_a_fee_the_chain_does_not_charge_as_its_own_state
}

/// Contract: Render → the piece-group size in bytes and the primary curve's
///   label.
/// Arrange: the defaults of this element.
/// Act:     `report`, once.
/// Assert:  the parsed document's `defaults_in_force` equals the expected
///   literal's `defaults_in_force`.
#[test]
fn report_records_the_defaults_in_force() {
    // Arrange
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
    let expected = json!({
        "curve": "curve-a",
        "conditions": "conditions-a",
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
        },
        "defaults_in_force": { "piece_group_size_bytes": 16384, "primary_curve": "curve-b" },
        "adjustment": null
    });
    // Act
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(parsed["defaults_in_force"], expected["defaults_in_force"]);
    // report_records_the_defaults_in_force
}

/// Contract: Adjustment → no adjustment is the JSON `null`.
/// Arrange: no adjustment.
/// Act:     `report`, once.
/// Assert:  the parsed document's `adjustment` is `Value::Null`.
#[test]
fn report_records_no_adjustment_as_null() {
    // Arrange
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
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(parsed["adjustment"], Value::Null);
    // report_records_no_adjustment_as_null
}

/// Contract: Adjustment → each field of the adjustment is its value, or
///   `null` where it holds none.
/// Arrange: an adjustment of `piece_group_size`
///   `NonZeroU32::MIN.saturating_add(32_767)` and no `primary_curve`.
/// Act:     `report`, once.
/// Assert:  the parsed document's `adjustment` equals
///   `json!({ "piece_group_size_bytes": 32768, "primary_curve": null })`.
#[test]
fn report_records_an_adjustment_that_changes_one_default() {
    // Arrange
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
        adjustment: Some(Some(build_report_adjustment(ReportAdjustmentOverrides {
            piece_group_size: Some(Some(NonZeroU32::MIN.saturating_add(32_767))),
            primary_curve: Some(None),
        }))),
    });
    // Act
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(
        parsed["adjustment"],
        json!({ "piece_group_size_bytes": 32768, "primary_curve": null })
    );
    // report_records_an_adjustment_that_changes_one_default
}

/// Contract: Render → no value is compared with a bound, clamped, or
///   refused.
/// Arrange: a measurement of the timing `Duration::MAX` for `prove_mint`,
///   the size `usize::MAX` for `capsule`, and a mint cost of gas `u64::MAX`
///   and `L1DataFee::Charged { wei: u128::MAX }`.
/// Act:     `report`, once.
/// Assert:  the parsed document's `timings.prove_mint` equals
///   `json!({ "seconds": u64::MAX, "subsec_nanos": 999999999 })`,
///   `sizes_bytes.capsule` equals `usize::MAX` as a JSON number, and
///   `costs.mint` equals `json!({ "l2_execution_gas": u64::MAX,
///   "l1_data_fee": { "state": "charged", "wei":
///   "340282366920938463463374607431768211455" } })`.
#[test]
fn report_records_every_value_verbatim_whatever_its_magnitude() {
    // Arrange
    let payload = build_report_payload(ReportPayloadOverrides {
        measurement: Some(build_delivery_measurement(DeliveryMeasurementOverrides {
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
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(
        parsed["timings"]["prove_mint"],
        json!({ "seconds": u64::MAX, "subsec_nanos": 999_999_999 })
    );
    assert_eq!(parsed["sizes_bytes"]["capsule"], json!(usize::MAX));
    assert_eq!(
        parsed["costs"]["mint"],
        json!({
            "l2_execution_gas": u64::MAX,
            "l1_data_fee": {
                "state": "charged",
                "wei": "340282366920938463463374607431768211455"
            }
        })
    );
    // report_records_every_value_verbatim_whatever_its_magnitude
}

/// Contract: Destination file → the file at the path holds the returned
///   document.
/// Arrange: the block's directory and one `ReportDestination::File` beneath
///   it.
/// Act:     `report`, once.
/// Assert:  `std::fs::read_to_string` of the path equals the returned
///   document.
#[test]
fn report_writes_the_document_to_a_file_destination() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-report-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("report.json");
    let path: &Path = file.as_path();
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
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(ReportParamsOverrides {
            destinations: Some(vec![ReportDestination::File {
                path: path.to_path_buf(),
            }]),
        }),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    assert_eq!(
        std::fs::read_to_string(path).expect("the file reads"),
        returned.document
    );
    // report_writes_the_document_to_a_file_destination
}

/// Contract: Destination file → every destination receives the same text.
/// Arrange: two `ReportDestination::File` destinations beneath the block's
///   directory.
/// Act:     `report`, once.
/// Assert:  both files read back equal the returned document.
#[test]
fn report_writes_the_same_document_to_each_destination_in_order() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-report-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the directory is created");
    let first_file = directory.join("first.json");
    let first: &Path = first_file.as_path();
    let second_file = directory.join("second.json");
    let second: &Path = second_file.as_path();
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
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(ReportParamsOverrides {
            destinations: Some(vec![
                ReportDestination::File {
                    path: first.to_path_buf(),
                },
                ReportDestination::File {
                    path: second.to_path_buf(),
                },
            ]),
        }),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    assert_eq!(
        std::fs::read_to_string(first).expect("the first file reads"),
        returned.document
    );
    assert_eq!(
        std::fs::read_to_string(second).expect("the second file reads"),
        returned.document
    );
    // report_writes_the_same_document_to_each_destination_in_order
}

/// Contract: No destination → the document is returned and nothing is
///   written.
/// Arrange: the block's directory created and no destination.
/// Act:     `report`, once.
/// Assert:  the result is `Ok` with a non-empty document, and the directory
///   holds no entry.
#[test]
fn report_with_no_destination_writes_nothing_and_returns_the_document() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-report-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the directory is created");
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
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(Default::default()),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    assert!(!returned.document.is_empty());
    let mut entries = std::fs::read_dir(&directory).expect("the directory reads");
    assert!(entries.next().is_none());
    // report_with_no_destination_writes_nothing_and_returns_the_document
}

/// Contract: Destination standard output → the write succeeds and the
///   document is returned.
/// Arrange: one `ReportDestination::StandardOutput`.
/// Act:     `report`, once.
/// Assert:  the result is `Ok` and the returned document parsed equals the
///   expected literal.
#[test]
fn report_returns_the_document_when_it_writes_to_standard_output() {
    // Arrange
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
    let expected = json!({
        "curve": "curve-a",
        "conditions": "conditions-a",
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
        },
        "defaults_in_force": { "piece_group_size_bytes": 16384, "primary_curve": "curve-b" },
        "adjustment": null
    });
    // Act
    let returned = report(
        &build_report_deps(Default::default()),
        build_report_params(ReportParamsOverrides {
            destinations: Some(vec![ReportDestination::StandardOutput]),
        }),
        payload,
    )
    .expect("the subject's call succeeds");
    // Assert
    let parsed: Value = serde_json::from_str(&returned.document).expect("the document parses");
    assert_eq!(parsed, expected);
    // report_returns_the_document_when_it_writes_to_standard_output
}

/// Contract: `ReportText::try_new` → `Ok(ReportText)` holding the text moved
///   from the params.
/// Arrange: `build_report_text_constructor_params` over `"curve-a"`.
/// Act:     `ReportText::try_new`, once.
/// Assert:  the result is `Ok` and `as_str()` of the value equals
///   `"curve-a"`.
#[test]
fn report_text_try_new_admits_a_text_with_a_visible_character() {
    // Arrange
    let params = build_report_text_constructor_params(ReportTextConstructorParamsOverrides {
        text: Some("curve-a".to_string()),
    });
    // Act
    let admitted = ReportText::try_new(params);
    // Assert
    assert_eq!(
        admitted.expect("a visible text is admitted").as_str(),
        "curve-a"
    );
    // report_text_try_new_admits_a_text_with_a_visible_character
}
