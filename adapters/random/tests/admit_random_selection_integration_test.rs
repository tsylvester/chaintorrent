#![cfg(feature = "mocks")]

use random::{
    AdmitRandomSelectionDepsOverrides, AdmitRandomSelectionErrorReturn,
    AdmitRandomSelectionParamsOverrides, MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_KINDS,
    RandomSourceKind, admit_random_selection, build_admit_random_selection_deps,
    build_admit_random_selection_params,
};
use serde_json::{Map, Value};

/// Contract: own entry `every_declared_member_is_admitted_to_the_selection_naming_it` —
///   `admit_random_selection` is called by a caller outside the crate over each
///   identifier of `RANDOM_SOURCE_KINDS`, with a document that is `Value::Null`
///   and with a document that is an empty `Value::Object` → `Ok` whose
///   selection equals the identifier.
/// Boundary: the public `admit_random_selection`; the route is
///   `admit_random_selection`, with no outer-edge collaborator.
/// Mocked:   nothing.
/// Arrange:  each identifier of `RANDOM_SOURCE_KINDS`, so a check that admits
///   only one member's identifier fails; the payloads `Value::Null` and an
///   empty `Value::Object`, the documents that supply no setting, so a check
///   that refuses either fails.
/// Act:      `admit_random_selection` over each identifier and each payload.
/// Assert:   each result equals `Ok` holding the selection that equals the
///   identifier it was called over.
#[test]
fn admit_random_selection_returns_each_declared_member_its_own_selection_for_an_outside_caller() {
    // Arrange
    let deps = build_admit_random_selection_deps(AdmitRandomSelectionDepsOverrides);

    // Act, Assert — over each identifier of the declared set and each document
    // that supplies no setting
    for identifier in RANDOM_SOURCE_KINDS {
        let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
            identifier: Some(*identifier),
        });
        for payload in [Value::Null, Value::Object(Map::new())] {
            let result = admit_random_selection(&deps, params, payload);
            assert_eq!(result.map(|success| success.selection), Ok(*identifier));
        }
    }
}
// admit_random_selection_returns_each_declared_member_its_own_selection_for_an_outside_caller

/// Contract: own entry `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode` —
///   `admit_random_selection` is called by a caller outside the crate under the
///   `mocks` feature over
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   and over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`,
///   with a document that is `Value::Null` and with a document that is an empty
///   `Value::Object` → `Ok` whose selection is the identifier looked at, its
///   failure mode included.
/// Boundary: the public `admit_random_selection`; the route is
///   `admit_random_selection`, with no outer-edge collaborator.
/// Mocked:   nothing, because the mock member is the family's own mock
///   concrete.
/// Arrange:  the identifiers carrying `FillBytesRefused` and `Succeeds`, modes
///   that differ from one another and from the operating-system member, so a
///   branch that returns one fixed mode, or the real member's selection,
///   fails; the payloads `Value::Null` and an empty `Value::Object`.
/// Act:      `admit_random_selection` over each identifier and each payload.
/// Assert:   each result for the `FillBytesRefused` identifier equals `Ok`
///   holding the selection
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`,
///   and each result for the `Succeeds` identifier equals `Ok` holding the
///   selection
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`.
#[test]
fn admit_random_selection_returns_the_mock_member_a_selection_carrying_its_failure_mode_for_an_outside_caller()
 {
    // Arrange
    let deps = build_admit_random_selection_deps(AdmitRandomSelectionDepsOverrides);
    let refused_params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let succeeds_params =
        build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
            identifier: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::Succeeds,
            )),
        });

    // Act
    let refused_null = admit_random_selection(&deps, refused_params, Value::Null);
    let refused_object = admit_random_selection(&deps, refused_params, Value::Object(Map::new()));
    let succeeds_null = admit_random_selection(&deps, succeeds_params, Value::Null);
    let succeeds_object = admit_random_selection(&deps, succeeds_params, Value::Object(Map::new()));

    // Assert
    assert_eq!(
        refused_null.map(|success| success.selection),
        Ok(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused
        ))
    );
    assert_eq!(
        refused_object.map(|success| success.selection),
        Ok(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused
        ))
    );
    assert_eq!(
        succeeds_null.map(|success| success.selection),
        Ok(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds
        ))
    );
    assert_eq!(
        succeeds_object.map(|success| success.selection),
        Ok(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds
        ))
    );
}
// admit_random_selection_returns_the_mock_member_a_selection_carrying_its_failure_mode_for_an_outside_caller

/// Contract: own entry `a_document_supplying_a_setting_is_refused_naming_the_member` —
///   `admit_random_selection` is called by a caller outside the crate over each
///   identifier of `RANDOM_SOURCE_KINDS` and, under the `mocks` feature, over
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`,
///   with a document that is a `Value::Object` holding an entry, with a
///   document that is an empty `Value::Array`, with a document that is a
///   `Value::Bool`, with a document that is a `Value::Number`, and with a
///   document that is a `Value::String` →
///   `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier })`
///   naming the identifier looked at.
/// Boundary: the public `admit_random_selection`; the route is
///   `admit_random_selection`, with no outer-edge collaborator.
/// Mocked:   nothing, because the mock member is the family's own mock
///   concrete.
/// Arrange:  each identifier of `RANDOM_SOURCE_KINDS` and the mock identifier
///   carrying `FillBytesRefused`; the payloads an object holding an entry, an
///   empty array, a boolean, a number, and a string, where the empty array is
///   a value an emptiness check on the document would admit, so a check that
///   refuses only objects, or that omits one of these kinds, fails.
/// Act:      `admit_random_selection` over each identifier and each payload.
/// Assert:   each result equals
///   `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier })`
///   naming the identifier it was called over.
#[test]
fn admit_random_selection_refuses_a_document_supplying_a_setting_naming_the_member_for_an_outside_caller()
 {
    // Arrange
    let deps = build_admit_random_selection_deps(AdmitRandomSelectionDepsOverrides);

    // Act, Assert — over each identifier and each document that supplies a
    // setting
    for identifier in RANDOM_SOURCE_KINDS
        .iter()
        .copied()
        .chain([RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )])
    {
        let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
            identifier: Some(identifier),
        });
        let mut entries = Map::new();
        entries.insert(String::from("unsupported_setting"), Value::Bool(true));
        for payload in [
            Value::Object(entries),
            Value::Array(Vec::new()),
            Value::Bool(true),
            Value::from(7_u64),
            Value::String(String::from("text")),
        ] {
            let result = admit_random_selection(&deps, params, payload);
            assert_eq!(
                result,
                Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier })
            );
        }
    }
}
// admit_random_selection_refuses_a_document_supplying_a_setting_naming_the_member_for_an_outside_caller
