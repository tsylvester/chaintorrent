#![cfg(feature = "mocks")]

use pairing::{
    AdmitPairingSelectionErrorReturn, AdmitPairingSelectionParamsOverrides,
    AdmitPairingSelectionSuccessReturn, AdmitPairingSelectionSuccessReturnOverrides,
    MockIPairingAdapterFailureMode, PAIRING_CONCRETES, PairingConcrete, admit_pairing_selection,
    build_admit_pairing_selection_deps, build_admit_pairing_selection_params,
    build_admit_pairing_selection_success_return,
};
use serde_json::{Map, Value};

/// Contract: entry `admission_returns_each_declared_member_for_a_document_without_a_setting`;
///   each member of `PAIRING_CONCRETES` with `Value::Object` holding no entry →
///   `Ok` whose `selection` is that member.
/// Boundary: the public call `admit_pairing_selection`, the route reading the
///   identifier and the payload.
/// Mocked:   nothing; the route has no outer-edge collaborator, so the test does
///   not prove the mock member.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES`, params built by `build_admit_pairing_selection_params`
///   with the member as the identifier override.
/// Act:      `admit_pairing_selection(&deps, params, document)` per member.
/// Assert:   each result equals `Ok(build_admit_pairing_selection_success_return(...))`
///   built with the member as the selection override.
#[test]
fn admission_returns_each_declared_member_for_a_document_without_a_setting() {
    for member in PAIRING_CONCRETES {
        // Arrange
        let deps = build_admit_pairing_selection_deps(Default::default());
        let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
            identifier: Some(*member),
        });
        let document = Value::Object(Map::new());
        let expected: AdmitPairingSelectionSuccessReturn =
            build_admit_pairing_selection_success_return(
                AdmitPairingSelectionSuccessReturnOverrides {
                    selection: Some(*member),
                },
            );

        // Act
        let result = admit_pairing_selection(&deps, params, document);

        // Assert
        assert_eq!(result, Ok(expected));
    }
}
// admission_returns_each_declared_member_for_a_document_without_a_setting

/// Contract: entry `admission_refuses_a_setting_for_each_declared_member`;
///   each member of `PAIRING_CONCRETES` with a `Value::Object` holding one entry →
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })`
///   naming that member.
/// Boundary: the public call `admit_pairing_selection`, the route reading the
///   identifier and the payload.
/// Mocked:   nothing; the route has no outer-edge collaborator.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES`, with a `Value::Object` holding the key `endpoint`
///   and the value `Value::Bool(true)`.
/// Act:      `admit_pairing_selection(&deps, params, document)` per member.
/// Assert:   each result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied
///   { identifier })` with the member it was called over.
#[test]
fn admission_refuses_a_setting_for_each_declared_member() {
    for member in PAIRING_CONCRETES {
        // Arrange
        let deps = build_admit_pairing_selection_deps(Default::default());
        let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
            identifier: Some(*member),
        });
        let mut entries = Map::new();
        entries.insert("endpoint".to_string(), Value::Bool(true));
        let document = Value::Object(entries);

        // Act
        let result = admit_pairing_selection(&deps, params, document);

        // Assert
        assert_eq!(
            result,
            Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
                identifier: *member,
            })
        );
    }
}
// admission_refuses_a_setting_for_each_declared_member

/// Contract: entry `mock_admission_names_the_identifier_admitted`;
///   `PairingConcrete::Mock(mode)` with no setting → `Ok` whose `selection` is
///   `PairingConcrete::Mock(mode)`, and with a setting → `Err` naming
///   `PairingConcrete::Mock(mode)`.
/// Boundary: the public call `admit_pairing_selection`, the route reading the
///   identifier and the payload.
/// Mocked:   the mock member itself, so the test proves the identifier is
///   carried and no real member's admission.
/// Arrange:  the identifier override
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`,
///   a mode other than `Succeeds`, called once with an empty `Value::Object`
///   and once with a `Value::Object` holding one entry.
/// Act:      `admit_pairing_selection(&deps, params, document)` per document.
/// Assert:   the first result equals `Ok(build_admit_pairing_selection_success_return(...))`
///   with the mock identifier as the selection override, and the second equals
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve) })`.
#[cfg(feature = "mocks")]
#[test]
fn mock_admission_names_the_identifier_admitted() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
    });
    let document = Value::Object(Map::new());
    let expected: AdmitPairingSelectionSuccessReturn =
        build_admit_pairing_selection_success_return(AdmitPairingSelectionSuccessReturnOverrides {
            selection: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
            )),
        });

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(result, Ok(expected));

    // Arrange
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
    });
    let mut entries = Map::new();
    entries.insert("endpoint".to_string(), Value::Bool(true));
    let document = Value::Object(entries);

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
            identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,),
        })
    );
}
// mock_admission_names_the_identifier_admitted

/// Contract: entry `admission_returns_each_declared_member_for_a_null_document`;
///   each member of `PAIRING_CONCRETES` with `Value::Null` → `Ok` whose
///   `selection` is that member.
/// Boundary: the public call `admit_pairing_selection`, the route reading the
///   identifier and the payload.
/// Mocked:   nothing; the route has no outer-edge collaborator.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES`, with `Value::Null`, the absent document, which
///   differs from the empty object the first block uses.
/// Act:      `admit_pairing_selection(&deps, params, document)` per member.
/// Assert:   each result equals `Ok(build_admit_pairing_selection_success_return(...))`
///   built with the member as the selection override.
#[test]
fn admission_returns_each_declared_member_for_a_null_document() {
    for member in PAIRING_CONCRETES {
        // Arrange
        let deps = build_admit_pairing_selection_deps(Default::default());
        let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
            identifier: Some(*member),
        });
        let document = Value::Null;
        let expected: AdmitPairingSelectionSuccessReturn =
            build_admit_pairing_selection_success_return(
                AdmitPairingSelectionSuccessReturnOverrides {
                    selection: Some(*member),
                },
            );

        // Act
        let result = admit_pairing_selection(&deps, params, document);

        // Assert
        assert_eq!(result, Ok(expected));
    }
}
// admission_returns_each_declared_member_for_a_null_document

/// Contract: entry `admission_refuses_each_non_object_document_for_each_declared_member`;
///   each member of `PAIRING_CONCRETES` with `Value::Bool(false)`, a
///   `Value::Number` holding zero, a `Value::String` holding the empty text,
///   and an empty `Value::Array` →
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })`
///   naming that member.
/// Boundary: the public call `admit_pairing_selection`, the route reading the
///   identifier and the payload.
/// Mocked:   nothing; the route has no outer-edge collaborator.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES` and, within each, over the four documents named in
///   the contract, each falsy or empty, so an admission that treats a falsy
///   or empty document as no setting fails.
/// Act:      `admit_pairing_selection(&deps, params, document)` per member and
///   document.
/// Assert:   each result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied
///   { identifier })` with the member it was called over.
#[test]
fn admission_refuses_each_non_object_document_for_each_declared_member() {
    for member in PAIRING_CONCRETES {
        for document in [
            Value::Bool(false),
            Value::Number(0.into()),
            Value::String(String::new()),
            Value::Array(Vec::new()),
        ] {
            // Arrange
            let deps = build_admit_pairing_selection_deps(Default::default());
            let params =
                build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
                    identifier: Some(*member),
                });

            // Act
            let result = admit_pairing_selection(&deps, params, document);

            // Assert
            assert_eq!(
                result,
                Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
                    identifier: *member,
                })
            );
        }
    }
}
// admission_refuses_each_non_object_document_for_each_declared_member

// Public-surface enumeration: the own entries
//   `admission_returns_each_declared_member_for_a_document_without_a_setting`,
//   proven by `admission_returns_each_declared_member_for_a_document_without_a_setting`,
//   `admission_returns_each_declared_member_for_a_null_document`,
//   proven by `admission_returns_each_declared_member_for_a_null_document`,
//   `admission_refuses_a_setting_for_each_declared_member`,
//   proven by `admission_refuses_a_setting_for_each_declared_member`,
//   `admission_refuses_each_non_object_document_for_each_declared_member`,
//   proven by `admission_refuses_each_non_object_document_for_each_declared_member`,
//   and `mock_admission_names_the_identifier_admitted`,
//   proven by `mock_admission_names_the_identifier_admitted`.
