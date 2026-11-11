#![allow(clippy::expect_used)]

use std::path::PathBuf;

use super::super::layers::provides::{DestinationTableOverrides, build_destination_table};
use super::admit_output;
use super::interface::{
    AdmitOutputErrorReturn, DESTINATION_KINDS, DestinationKind, OutputDestination,
};
use super::mock::{
    build_admit_output_deps, build_admit_output_params, build_admit_output_payload, declared_name,
};
use crate::report::provides::ReportDestination;

/// Contract: `every_declared_destination_kind_is_admitted_to_its_own_variant`
///   → `admit_output` over a document whose `output.destinations` and
///   `report.destinations` each hold one entry whose `kind` is the `Debug`
///   text of the member under test returns `Ok` whose `destinations` holds
///   the `OutputDestination` the member names and whose `report_destinations`
///   holds the `ReportDestination` the member names, a file holding the path
///   its own list's entry carried.
/// Arrange: for each member of `DESTINATION_KINDS` a payload from
///   `build_admit_output_payload` whose two lists each hold one entry keyed
///   `declared_name` of the member, its path chosen by an exhaustive `match`
///   on the member (absent for `StandardOutput`, a distinct path in each
///   list for `File`), and the expected `OutputDestination` and
///   `ReportDestination` built by the same `match`.
/// Act:     `admit_output`, once per member.
/// Assert:  the result is `Ok` and each admitted list equals the one expected
///   destination.
/// Boundary: the real `admit_output` and `kind_named` over the real declared
///   set.
/// Mocked:  none.
#[test]
fn every_declared_destination_kind_is_admitted_to_its_own_variant() {
    // Arrange — the deps and params are invariant across every member.
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());

    // Act / Assert — every member of `DESTINATION_KINDS` in turn.
    for member in DESTINATION_KINDS {
        let (output_path, report_path, expected_output, expected_report) = match member {
            DestinationKind::StandardOutput => (
                None,
                None,
                OutputDestination::StandardOutput,
                ReportDestination::StandardOutput,
            ),
            DestinationKind::File => {
                let output_path = PathBuf::from("outcome.txt");
                let report_path = PathBuf::from("report.json");
                (
                    Some(output_path.clone()),
                    Some(report_path.clone()),
                    OutputDestination::File { path: output_path },
                    ReportDestination::File { path: report_path },
                )
            }
        };
        let mut payload = build_admit_output_payload(Default::default());
        payload.output.destinations = vec![build_destination_table(DestinationTableOverrides {
            kind: Some(declared_name(member)),
            path: Some(output_path),
        })];
        payload.report.destinations = vec![build_destination_table(DestinationTableOverrides {
            kind: Some(declared_name(member)),
            path: Some(report_path),
        })];

        let admitted = admit_output(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;

        assert_eq!(admitted.destinations, vec![expected_output]);
        assert_eq!(admitted.report_destinations, vec![expected_report]);
    }
    // every_declared_destination_kind_is_admitted_to_its_own_variant
}

/// Contract: `a_destination_list_keeps_the_order_the_document_holds` →
///   `admit_output` over a document whose two lists each hold every member of
///   `DESTINATION_KINDS`, once in declared order and once in reverse, returns
///   `Ok` whose lists hold the admitted destinations in the order the entries
///   were held, each file with its own path.
/// Arrange: a payload whose two lists each hold every member of
///   `DESTINATION_KINDS` in declared order and a payload whose lists hold
///   them in reverse, each file with a path distinct from every other
///   file's, and the expected destinations built by the exhaustive `match`
///   in the same orders.
/// Act:     `admit_output`, once per payload.
/// Assert:  the result is `Ok` and each admitted list equals the expected
///   destinations in order.
/// Boundary: the real `admit_output` and `kind_named` over the real declared
///   set.
/// Mocked:  none.
#[test]
fn a_destination_list_keeps_the_order_the_document_holds() {
    // Arrange — the deps and params are invariant across every order.
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let declared: Vec<DestinationKind> = DESTINATION_KINDS.to_vec();
    let reversed: Vec<DestinationKind> = DESTINATION_KINDS.iter().rev().copied().collect();

    // Act / Assert — the declared order and the reverse in turn.
    for order in [declared, reversed] {
        let mut payload = build_admit_output_payload(Default::default());
        payload.output.destinations = order
            .iter()
            .enumerate()
            .map(|(index, member)| {
                build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(member)),
                    path: Some(match member {
                        DestinationKind::StandardOutput => None,
                        DestinationKind::File => Some(PathBuf::from(format!("output-{index}"))),
                    }),
                })
            })
            .collect();
        payload.report.destinations = order
            .iter()
            .enumerate()
            .map(|(index, member)| {
                build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(member)),
                    path: Some(match member {
                        DestinationKind::StandardOutput => None,
                        DestinationKind::File => Some(PathBuf::from(format!("report-{index}"))),
                    }),
                })
            })
            .collect();
        let expected_output: Vec<OutputDestination> = order
            .iter()
            .enumerate()
            .map(|(index, member)| match member {
                DestinationKind::StandardOutput => OutputDestination::StandardOutput,
                DestinationKind::File => OutputDestination::File {
                    path: PathBuf::from(format!("output-{index}")),
                },
            })
            .collect();
        let expected_report: Vec<ReportDestination> = order
            .iter()
            .enumerate()
            .map(|(index, member)| match member {
                DestinationKind::StandardOutput => ReportDestination::StandardOutput,
                DestinationKind::File => ReportDestination::File {
                    path: PathBuf::from(format!("report-{index}")),
                },
            })
            .collect();

        let admitted = admit_output(&deps, params, payload)
            .expect("the entries are admitted")
            .admitted;

        assert_eq!(admitted.destinations, expected_output);
        assert_eq!(admitted.report_destinations, expected_report);
    }
    // a_destination_list_keeps_the_order_the_document_holds
}

/// Contract: `an_unadmissible_destination_list_is_never_admitted` →
///   `admit_output` over a document whose `output.destinations` and then
///   whose `report.destinations`, the other list admissible, is empty, holds
///   an entry whose `kind` is no member's `Debug` text, or holds a `File`
///   entry with no path returns `Err` carrying the list's variant with the
///   entry's index and, for a kind, its text.
/// Arrange: for each list the default payload with that list empty, holding
///   one entry of the kind `"unknown"`, or holding an entry of the kind
///   `declared_name` of each member that takes a path with none.
/// Act:     `admit_output`, once per payload.
/// Assert:  the result equals `Err` of the list's variant carrying the index
///   and, for a kind, the text.
/// Boundary: the real `admit_output` and `kind_named` over the real declared
///   set.
/// Mocked:  none.
#[test]
fn an_unadmissible_destination_list_is_never_admitted() {
    // Arrange — the deps and params are invariant across every payload.
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());

    // The output list's three refusals, the report list the default.
    let mut payload = build_admit_output_payload(Default::default());
    payload.output.destinations = vec![];
    let result = admit_output(&deps, params, payload);
    assert_eq!(result, Err(AdmitOutputErrorReturn::OutputDestinationsEmpty));

    let mut payload = build_admit_output_payload(Default::default());
    payload.output.destinations = vec![build_destination_table(DestinationTableOverrides {
        kind: Some("unknown".to_string()),
        path: Some(None),
    })];
    let result = admit_output(&deps, params, payload);
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::OutputDestinationKind {
            index: 0,
            text: "unknown".to_string()
        })
    );

    for member in DESTINATION_KINDS {
        match member {
            DestinationKind::StandardOutput => {}
            DestinationKind::File => {
                let mut payload = build_admit_output_payload(Default::default());
                payload.output.destinations =
                    vec![build_destination_table(DestinationTableOverrides {
                        kind: Some(declared_name(member)),
                        path: Some(None),
                    })];
                let result = admit_output(&deps, params, payload);
                assert_eq!(
                    result,
                    Err(AdmitOutputErrorReturn::OutputDestinationPath { index: 0 })
                );
            }
        }
    }

    // The report list's three refusals, the output list the default.
    let mut payload = build_admit_output_payload(Default::default());
    payload.report.destinations = vec![];
    let result = admit_output(&deps, params, payload);
    assert_eq!(result, Err(AdmitOutputErrorReturn::ReportDestinationsEmpty));

    let mut payload = build_admit_output_payload(Default::default());
    payload.report.destinations = vec![build_destination_table(DestinationTableOverrides {
        kind: Some("unknown".to_string()),
        path: Some(None),
    })];
    let result = admit_output(&deps, params, payload);
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::ReportDestinationKind {
            index: 0,
            text: "unknown".to_string()
        })
    );

    for member in DESTINATION_KINDS {
        match member {
            DestinationKind::StandardOutput => {}
            DestinationKind::File => {
                let mut payload = build_admit_output_payload(Default::default());
                payload.report.destinations =
                    vec![build_destination_table(DestinationTableOverrides {
                        kind: Some(declared_name(member)),
                        path: Some(None),
                    })];
                let result = admit_output(&deps, params, payload);
                assert_eq!(
                    result,
                    Err(AdmitOutputErrorReturn::ReportDestinationPath { index: 0 })
                );
            }
        }
    }
    // an_unadmissible_destination_list_is_never_admitted
}
