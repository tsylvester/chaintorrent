#![allow(clippy::expect_used)]

use std::path::PathBuf;

use super::super::layers::provides::{
    DestinationTableOverrides, OutputTableOverrides, ReportTableOverrides, build_destination_table,
    build_output_table, build_report_table,
};
use super::admit_output;
use super::interface::{DestinationKind, OutputDestination};
use super::mock::{
    AdmitOutputPayloadOverrides, build_admit_output_deps, build_admit_output_params,
    build_admit_output_payload, build_admitted_output, declared_name,
};
use crate::report::provides::ReportDestination;

/// Contract: Success → a payload whose texts the declared set names admits to
///   the destinations built from the same members.
/// Arrange: `build_admit_output_payload(Default::default())`.
/// Act:     `admit_output`, once.
/// Assert:  the result is `Ok` and its `admitted` equals
///   `build_admitted_output(Default::default())`.
#[test]
fn admit_output_admits_the_default_payload() {
    // Arrange
    let payload = build_admit_output_payload(Default::default());
    let expected = build_admitted_output(Default::default());
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result.expect("the admission succeeds").admitted, expected);
    // admit_output_admits_the_default_payload
}

/// Contract: Output destination and Report destination → the kind
///   `StandardOutput` admits to the variant of that name in each list.
/// Arrange: one entry of the kind
///   `declared_name(&DestinationKind::StandardOutput)` and no path in each
///   list.
/// Act:     `admit_output`, once.
/// Assert:  `admitted.destinations` equals
///   `vec![OutputDestination::StandardOutput]` and
///   `admitted.report_destinations` equals
///   `vec![ReportDestination::StandardOutput]`.
#[test]
fn admit_output_admits_a_standard_output_destination_in_each_list() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::StandardOutput)),
                path: Some(None),
            })]),
        })),
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::StandardOutput)),
                path: Some(None),
            })]),
            ..Default::default()
        })),
    });
    // Act
    let admitted = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(
        admitted.destinations,
        vec![OutputDestination::StandardOutput]
    );
    assert_eq!(
        admitted.report_destinations,
        vec![ReportDestination::StandardOutput]
    );
    // admit_output_admits_a_standard_output_destination_in_each_list
}

/// Contract: Output destination and Report destination → the kind `File`
///   admits to a file holding its own entry's path.
/// Arrange: one entry of the kind `declared_name(&DestinationKind::File)` in
///   each list, the output entry with the path `"outcome.txt"` and the report
///   entry with the path `"report.json"`.
/// Act:     `admit_output`, once.
/// Assert:  `admitted.destinations` equals
///   `vec![OutputDestination::File { path: PathBuf::from("outcome.txt") }]`
///   and `admitted.report_destinations` equals
///   `vec![ReportDestination::File { path: PathBuf::from("report.json") }]`.
#[test]
fn admit_output_admits_a_file_destination_with_the_path_the_document_holds() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::File)),
                path: Some(Some(PathBuf::from("outcome.txt"))),
            })]),
        })),
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::File)),
                path: Some(Some(PathBuf::from("report.json"))),
            })]),
            ..Default::default()
        })),
    });
    // Act
    let admitted = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(
        admitted.destinations,
        vec![OutputDestination::File {
            path: PathBuf::from("outcome.txt")
        }]
    );
    assert_eq!(
        admitted.report_destinations,
        vec![ReportDestination::File {
            path: PathBuf::from("report.json")
        }]
    );
    // admit_output_admits_a_file_destination_with_the_path_the_document_holds
}

/// Contract: Success → each list is admitted in the order of its entries.
/// Arrange: in each list, a file with the path `"first"`, a standard output,
///   and a file with the path `"second"`, in that order.
/// Act:     `admit_output`, once.
/// Assert:  each admitted list equals the three destinations in that order.
#[test]
fn admit_output_keeps_the_destinations_in_the_order_the_document_holds_them() {
    // Arrange
    let entries = vec![
        build_destination_table(DestinationTableOverrides {
            kind: Some(declared_name(&DestinationKind::File)),
            path: Some(Some(PathBuf::from("first"))),
        }),
        build_destination_table(DestinationTableOverrides {
            kind: Some(declared_name(&DestinationKind::StandardOutput)),
            path: Some(None),
        }),
        build_destination_table(DestinationTableOverrides {
            kind: Some(declared_name(&DestinationKind::File)),
            path: Some(Some(PathBuf::from("second"))),
        }),
    ];
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(entries.clone()),
        })),
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(entries),
            ..Default::default()
        })),
    });
    let expected_output = vec![
        OutputDestination::File {
            path: PathBuf::from("first"),
        },
        OutputDestination::StandardOutput,
        OutputDestination::File {
            path: PathBuf::from("second"),
        },
    ];
    let expected_report = vec![
        ReportDestination::File {
            path: PathBuf::from("first"),
        },
        ReportDestination::StandardOutput,
        ReportDestination::File {
            path: PathBuf::from("second"),
        },
    ];
    // Act
    let admitted = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.destinations, expected_output);
    assert_eq!(admitted.report_destinations, expected_report);
    // admit_output_keeps_the_destinations_in_the_order_the_document_holds_them
}

/// Contract: Success → the output list's entries do not decide the report
///   list's.
/// Arrange: `output.destinations` holding a file with the path
///   `"outcome.txt"` and `report.destinations` holding a standard output.
/// Act:     `admit_output`, once.
/// Assert:  `admitted.destinations` equals the file and
///   `admitted.report_destinations` equals
///   `vec![ReportDestination::StandardOutput]`.
#[test]
fn admit_output_admits_the_two_lists_independently() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::File)),
                path: Some(Some(PathBuf::from("outcome.txt"))),
            })]),
        })),
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::StandardOutput)),
                path: Some(None),
            })]),
            ..Default::default()
        })),
    });
    // Act
    let admitted = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(
        admitted.destinations,
        vec![OutputDestination::File {
            path: PathBuf::from("outcome.txt")
        }]
    );
    assert_eq!(
        admitted.report_destinations,
        vec![ReportDestination::StandardOutput]
    );
    // admit_output_admits_the_two_lists_independently
}

/// Contract: Output destination → the path of a standard output entry is
///   unread.
/// Arrange: a standard output entry in each list carrying the path
///   `"ignored"`.
/// Act:     `admit_output`, once.
/// Assert:  the result is `Ok` and each admitted list equals its standard
///   output variant alone.
#[test]
fn admit_output_does_not_read_the_path_of_a_standard_output_destination() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        output: Some(build_output_table(OutputTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::StandardOutput)),
                path: Some(Some(PathBuf::from("ignored"))),
            })]),
        })),
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::StandardOutput)),
                path: Some(Some(PathBuf::from("ignored"))),
            })]),
            ..Default::default()
        })),
    });
    // Act
    let admitted = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(
        admitted.destinations,
        vec![OutputDestination::StandardOutput]
    );
    assert_eq!(
        admitted.report_destinations,
        vec![ReportDestination::StandardOutput]
    );
    // admit_output_does_not_read_the_path_of_a_standard_output_destination
}

/// Contract: Report destination → the report table's other values do not
///   change the admission.
/// Arrange: the default payload with `build_report_table` overridden so that
///   `piece_group_size` and `primary_curve` differ from the defaults.
/// Act:     `admit_output`, once.
/// Assert:  the result is `Ok` and its `admitted` equals
///   `build_admitted_output(Default::default())`.
#[test]
fn admit_output_reads_only_the_destinations_of_the_report_table() {
    // Arrange
    let payload = build_admit_output_payload(AdmitOutputPayloadOverrides {
        report: Some(build_report_table(ReportTableOverrides {
            destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                kind: Some(declared_name(&DestinationKind::StandardOutput)),
                path: Some(None),
            })]),
            piece_group_size: Some(4096),
            primary_curve: Some("curve-b".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let expected = build_admitted_output(Default::default());
    // Act
    let result = admit_output(
        &build_admit_output_deps(Default::default()),
        build_admit_output_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result.expect("the admission succeeds").admitted, expected);
    // admit_output_reads_only_the_destinations_of_the_report_table
}
