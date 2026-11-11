#![allow(clippy::expect_used)]

use super::super::layers::provides::{
    DestinationTableOverrides, OutputTableOverrides, ReportTableOverrides, build_destination_table,
    build_output_table, build_report_table,
};
use super::admit_output;
use super::interface::{AdmitOutputErrorReturn, DestinationKind};
use super::mock::{
    AdmitOutputPayloadOverrides, build_admit_output_deps, build_admit_output_params,
    build_admit_output_payload, declared_name,
};

/// Contract: Output list empty → `Err(AdmitOutputErrorReturn::OutputDestinationsEmpty)`.
/// Arrange: the default payload with `output.destinations` empty.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::OutputDestinationsEmpty)`.
#[test]
fn admit_output_refuses_an_empty_output_list() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![]),
        })),
        ..Default::default()
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitOutputErrorReturn::OutputDestinationsEmpty));
    // admit_output_refuses_an_empty_output_list
}

/// Contract: Output destination kind →
///   `Err(AdmitOutputErrorReturn::OutputDestinationKind { index, text })`.
/// Arrange: `output.destinations` holding a standard output entry and then an
///   entry of the kind `"unknown"`.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::OutputDestinationKind
///   { index: 1, text: "unknown".to_string() })`.
#[test]
fn admit_output_refuses_an_unknown_output_destination_kind_with_its_index() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![
                build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(&DestinationKind::StandardOutput)),
                    path: Some(None),
                }),
                build_destination_table(DestinationTableOverrides {
                    kind: Some("unknown".to_string()),
                    path: Some(None),
                }),
            ]),
        })),
        ..Default::default()
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::OutputDestinationKind {
            index: 1,
            text: "unknown".to_string()
        })
    );
    // admit_output_refuses_an_unknown_output_destination_kind_with_its_index
}

/// Contract: the naming rule → a text equals a member's `Debug` text whole and
///   case-sensitively.
/// Arrange: `output.destinations` holding one entry whose kind is the
///   lowercase of `declared_name(&DestinationKind::StandardOutput)`.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::OutputDestinationKind
///   { index: 0, text })` with `text` the lowercase text.
#[test]
fn admit_output_refuses_an_output_destination_kind_that_differs_only_in_case() {
    // Arrange
    let text = declared_name(&DestinationKind::StandardOutput).to_lowercase();
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(text.clone()),
                path: Some(None),
            })]),
        })),
        ..Default::default()
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::OutputDestinationKind { index: 0, text })
    );
    // admit_output_refuses_an_output_destination_kind_that_differs_only_in_case
}

/// Contract: Output destination →
///   `Err(AdmitOutputErrorReturn::OutputDestinationPath { index })`.
/// Arrange: `output.destinations` holding a standard output entry and then an
///   entry of the kind `declared_name(&DestinationKind::File)` with no path.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::OutputDestinationPath
///   { index: 1 })`.
#[test]
fn admit_output_refuses_an_output_file_destination_with_no_path() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![
                build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(&DestinationKind::StandardOutput)),
                    path: Some(None),
                }),
                build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(&DestinationKind::File)),
                    path: Some(None),
                }),
            ]),
        })),
        ..Default::default()
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::OutputDestinationPath { index: 1 })
    );
    // admit_output_refuses_an_output_file_destination_with_no_path
}

/// Contract: Report list empty → `Err(AdmitOutputErrorReturn::ReportDestinationsEmpty)`.
/// Arrange: the default payload with `report.destinations` empty.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::ReportDestinationsEmpty)`.
#[test]
fn admit_output_refuses_an_empty_report_list() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![]),
            ..Default::default()
        })),
        ..Default::default()
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitOutputErrorReturn::ReportDestinationsEmpty));
    // admit_output_refuses_an_empty_report_list
}

/// Contract: Report destination kind →
///   `Err(AdmitOutputErrorReturn::ReportDestinationKind { index, text })`.
/// Arrange: `report.destinations` holding a standard output entry and then an
///   entry of the kind `"unknown"`.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::ReportDestinationKind
///   { index: 1, text: "unknown".to_string() })`.
#[test]
fn admit_output_refuses_an_unknown_report_destination_kind_with_its_index() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![
                build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(&DestinationKind::StandardOutput)),
                    path: Some(None),
                }),
                build_destination_table(DestinationTableOverrides {
                    kind: Some("unknown".to_string()),
                    path: Some(None),
                }),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::ReportDestinationKind {
            index: 1,
            text: "unknown".to_string()
        })
    );
    // admit_output_refuses_an_unknown_report_destination_kind_with_its_index
}

/// Contract: Report destination →
///   `Err(AdmitOutputErrorReturn::ReportDestinationPath { index })`.
/// Arrange: `report.destinations` holding an entry of the kind
///   `declared_name(&DestinationKind::File)` with no path.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::ReportDestinationPath
///   { index: 0 })`.
#[test]
fn admit_output_refuses_a_report_file_destination_with_no_path() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::File)),
                path: Some(None),
            })]),
            ..Default::default()
        })),
        ..Default::default()
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::ReportDestinationPath { index: 0 })
    );
    // admit_output_refuses_a_report_file_destination_with_no_path
}

/// Contract: Ordering → the output list is admitted before the report list,
///   and the first refused value returns with later values unread.
/// Arrange: the default payload with `output.destinations` holding an entry of
///   the kind `"unknown"` and `report.destinations` empty.
/// Act:     `admit_output`, once.
/// Assert:  the result equals `Err(AdmitOutputErrorReturn::OutputDestinationKind
///   { index: 0, text: "unknown".to_string() })`.
#[test]
fn admit_output_returns_the_first_refusal_in_the_stated_order() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some("unknown".to_string()),
                path: Some(None),
            })]),
        })),
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![]),
            ..Default::default()
        })),
    });
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::OutputDestinationKind {
            index: 0,
            text: "unknown".to_string()
        })
    );
    // admit_output_returns_the_first_refusal_in_the_stated_order
}
