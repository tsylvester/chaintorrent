#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use super::super::layers::provides::{
    OutputDestinationDocumentOverrides, OutputDocumentOverrides, build_output_destination_document,
    build_output_document,
};
use super::admit_output;
use super::interface::*;
use super::mock::{
    AdmitOutputSuccessReturnOverrides, build_admit_output_deps, build_admit_output_params,
    build_admit_output_payload, build_admit_output_success_return,
};

/// Contract: `("standard_output", None)` admits
///   `OutputDestination::StandardOutput`.
/// Arrange: `output` `build_output_document` with no override, the one
///   destination the document builder defaults to.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Ok(build_admit_output_success_return` with
///   `destinations` `Some(vec![OutputDestination::StandardOutput]))`.
#[test]
fn admit_output_maps_the_document_defaults() {
    // Arrange
    let output = build_output_document(Default::default());
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_output_success_return(
            AdmitOutputSuccessReturnOverrides {
                destinations: Some(vec![OutputDestination::StandardOutput]),
            },
        ))
    );
    // admit_output_maps_the_document_defaults
}

/// Contract: each destination is admitted in document order,
///   `("file", Some(path))` admitting `OutputDestination::File { path }`
///   carrying the document's path.
/// Arrange: `output` `build_output_document` with `destinations` a
///   `build_output_destination_document` with `kind` `"file"` and `path`
///   `Some(Some(PathBuf::from("harness-output.txt")))`, then
///   `build_output_destination_document` with no override.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Ok(build_admit_output_success_return` with
///   `destinations` `Some(vec![OutputDestination::File {
///   path: PathBuf::from("harness-output.txt") },
///   OutputDestination::StandardOutput]))`.
#[test]
fn admit_output_maps_each_destination_in_document_order() {
    // Arrange
    let output = build_output_document(OutputDocumentOverrides {
        destinations: Some(vec![
            build_output_destination_document(OutputDestinationDocumentOverrides {
                kind: Some("file".to_string()),
                path: Some(Some(PathBuf::from("harness-output.txt"))),
            }),
            build_output_destination_document(Default::default()),
        ]),
    });
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_output_success_return(
            AdmitOutputSuccessReturnOverrides {
                destinations: Some(vec![
                    OutputDestination::File {
                        path: PathBuf::from("harness-output.txt"),
                    },
                    OutputDestination::StandardOutput,
                ]),
            },
        ))
    );
    // admit_output_maps_each_destination_in_document_order
}

/// Contract: `output.destinations` is empty, a list naming no destination
///   → `Err(AdmitOutputErrorReturn::Empty)`.
/// Arrange: `output` `build_output_document` with `destinations` `Vec::new()`.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitOutputErrorReturn::Empty)`.
#[test]
fn admit_output_refuses_an_empty_destination_list() {
    // Arrange
    let output = build_output_document(OutputDocumentOverrides {
        destinations: Some(Vec::new()),
    });
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(result, Err(AdmitOutputErrorReturn::Empty));
    // admit_output_refuses_an_empty_destination_list
}

/// Contract: a kind text other than `"standard_output"` and `"file"` →
///   `Err(AdmitOutputErrorReturn::UnknownKind { index, text })`, `index` the
///   destination's position.
/// Arrange: `output` `build_output_document` with `destinations`
///   `build_output_destination_document` with no override, then one with
///   `kind` `"screen"`.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitOutputErrorReturn::UnknownKind {
///   index: 1, text: "screen".to_string() })`.
#[test]
fn admit_output_refuses_an_unknown_kind_with_its_index() {
    // Arrange
    let output = build_output_document(OutputDocumentOverrides {
        destinations: Some(vec![
            build_output_destination_document(Default::default()),
            build_output_destination_document(OutputDestinationDocumentOverrides {
                kind: Some("screen".to_string()),
                ..Default::default()
            }),
        ]),
    });
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::UnknownKind {
            index: 1,
            text: "screen".to_string(),
        })
    );
    // admit_output_refuses_an_unknown_kind_with_its_index
}

/// Contract: an unknown kind text is refused whatever its path, the kind
///   decided before the path → `Err(AdmitOutputErrorReturn::UnknownKind {
///   index, text })`.
/// Arrange: `output` `build_output_document` with `destinations` one
///   `build_output_destination_document` with `kind` `"screen"` and `path`
///   `Some(Some(PathBuf::from("harness-output.txt")))`.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitOutputErrorReturn::UnknownKind {
///   index: 0, text: "screen".to_string() })`.
#[test]
fn admit_output_refuses_an_unknown_kind_whatever_its_path() {
    // Arrange
    let output = build_output_document(OutputDocumentOverrides {
        destinations: Some(vec![build_output_destination_document(
            OutputDestinationDocumentOverrides {
                kind: Some("screen".to_string()),
                path: Some(Some(PathBuf::from("harness-output.txt"))),
            },
        )]),
    });
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::UnknownKind {
            index: 0,
            text: "screen".to_string(),
        })
    );
    // admit_output_refuses_an_unknown_kind_whatever_its_path
}

/// Contract: `("file", None)` → `Err(AdmitOutputErrorReturn::MissingPath {
///   index })`.
/// Arrange: `output` `build_output_document` with `destinations`
///   `build_output_destination_document` with no override, then one with
///   `kind` `"file"` and `path` `Some(None)`.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitOutputErrorReturn::MissingPath {
///   index: 1 })`.
#[test]
fn admit_output_refuses_a_file_without_a_path() {
    // Arrange
    let output = build_output_document(OutputDocumentOverrides {
        destinations: Some(vec![
            build_output_destination_document(Default::default()),
            build_output_destination_document(OutputDestinationDocumentOverrides {
                kind: Some("file".to_string()),
                path: Some(None),
            }),
        ]),
    });
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::MissingPath { index: 1 })
    );
    // admit_output_refuses_a_file_without_a_path
}

/// Contract: `("standard_output", Some(_))` →
///   `Err(AdmitOutputErrorReturn::UnexpectedPath { index })`.
/// Arrange: `output` `build_output_document` with `destinations`
///   `build_output_destination_document` with no override, then one with
///   `path` `Some(Some(PathBuf::from("harness-output.txt")))`.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitOutputErrorReturn::UnexpectedPath {
///   index: 1 })`.
#[test]
fn admit_output_refuses_standard_output_with_a_path() {
    // Arrange
    let output = build_output_document(OutputDocumentOverrides {
        destinations: Some(vec![
            build_output_destination_document(Default::default()),
            build_output_destination_document(OutputDestinationDocumentOverrides {
                path: Some(Some(PathBuf::from("harness-output.txt"))),
                ..Default::default()
            }),
        ]),
    });
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::UnexpectedPath { index: 1 })
    );
    // admit_output_refuses_standard_output_with_a_path
}

/// Contract: the destinations are checked in document order and the first
///   refusal returns → `Err(AdmitOutputErrorReturn::UnknownKind { index: 0,
///   text })`.
/// Arrange: `output` `build_output_document` with `destinations` one
///   `build_output_destination_document` with `kind` `"screen"`, then one
///   with `kind` `"file"` and `path` `Some(None)`.
/// Act:    `admit_output(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitOutputErrorReturn::UnknownKind {
///   index: 0, text: "screen".to_string() })`.
#[test]
fn admit_output_returns_the_first_refusal_in_document_order() {
    // Arrange
    let output = build_output_document(OutputDocumentOverrides {
        destinations: Some(vec![
            build_output_destination_document(OutputDestinationDocumentOverrides {
                kind: Some("screen".to_string()),
                ..Default::default()
            }),
            build_output_destination_document(OutputDestinationDocumentOverrides {
                kind: Some("file".to_string()),
                path: Some(None),
            }),
        ]),
    });
    let deps = build_admit_output_deps(Default::default());
    let params = build_admit_output_params(Default::default());
    let payload = build_admit_output_payload(&output, Default::default());

    // Act
    let result = admit_output(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitOutputErrorReturn::UnknownKind {
            index: 0,
            text: "screen".to_string(),
        })
    );
    // admit_output_returns_the_first_refusal_in_document_order
}
