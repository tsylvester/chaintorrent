#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use kdf::{
    AdmitKdfSelectionErrorReturn, AdmitKdfSelectionParamsOverrides,
    AdmitKdfSelectionPayloadCorruptions, AdmitKdfSelectionPayloadError, KDF_CONCRETES, KdfConcrete,
    KdfConcreteOverrides, MockIKeyDerivationAdapterFailureMode, admit_kdf_selection,
    build_admit_kdf_selection_deps, build_admit_kdf_selection_params, build_kdf_concrete,
    invalidate_admit_kdf_selection_payload,
};
use serde_json::Value;

/// Contract: `admit_kdf_selection` is called through the crate's public
///   surface with the member identifier of each member of `KDF_CONCRETES`
///   and an empty settings document, with no key-derivation member
///   constructed anywhere in the call → `Ok` whose `selection` is that
///   member.
/// Arrange: for each `concrete` in `KDF_CONCRETES`,
///   `build_admit_kdf_selection_params` with the `concrete` override
///   `build_kdf_concrete(concrete, KdfConcreteOverrides::default())`;
///   `build_admit_kdf_selection_deps` with no override; the document from
///   `invalidate_admit_kdf_selection_payload` with no corruption; the body
///   is written once and runs over the declared set, and the call is made
///   from outside the crate with no member and no factory involved.
/// Act:     `admit_kdf_selection(&deps, params, document)` for the case.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an empty document supplies no setting; `success.selection`
///   equals `concrete`.
/// Boundary: the public call `admit_kdf_selection` and the route the own
///   entries state, through the narrowing and the `match`.
/// Mocked:  nothing; the test constructs no member and does not prove the
///   mock member's arm, which a later block proves.
#[test]
fn admit_kdf_selection_returns_the_selection_naming_each_declared_member_through_the_crate_surface()
{
    // Arrange
    for concrete in KDF_CONCRETES {
        let deps = build_admit_kdf_selection_deps(Default::default());
        let params = build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
            concrete: Some(build_kdf_concrete(*concrete, KdfConcreteOverrides)),
        });
        let document = invalidate_admit_kdf_selection_payload(Default::default());

        // Act
        let result = admit_kdf_selection(&deps, params, document);

        // Assert
        let success = result.expect("an empty settings document supplies no setting");
        assert_eq!(success.selection, *concrete);
    }
}

/// Contract: `admit_kdf_selection` is called through the crate's public
///   surface with the member identifier of each member of `KDF_CONCRETES`
///   and the JSON null as the settings document → `Ok` whose `selection`
///   is that member.
/// Arrange: for each `concrete` in `KDF_CONCRETES`,
///   `build_admit_kdf_selection_params` with the `concrete` override
///   `build_kdf_concrete(concrete, KdfConcreteOverrides::default())`;
///   `build_admit_kdf_selection_deps` with no override; the document from
///   `invalidate_admit_kdf_selection_payload` with the `settings`
///   corruption `Some(Value::Null)`, so the document is absent rather than
///   an empty table.
/// Act:     `admit_kdf_selection(&deps, params, document)` for the case.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an absent document supplies no setting; `success.selection`
///   equals `concrete`.
/// Boundary: the public call `admit_kdf_selection` and the route the own
///   entries state, through the narrowing and the `match`.
/// Mocked:  nothing.
#[test]
fn admit_kdf_selection_returns_the_selection_for_an_absent_document_through_the_crate_surface() {
    // Arrange
    for concrete in KDF_CONCRETES {
        let deps = build_admit_kdf_selection_deps(Default::default());
        let params = build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
            concrete: Some(build_kdf_concrete(*concrete, KdfConcreteOverrides)),
        });
        let document =
            invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions {
                settings: Some(Value::Null),
            });

        // Act
        let result = admit_kdf_selection(&deps, params, document);

        // Assert
        let success = result.expect("an absent document supplies no setting");
        assert_eq!(success.selection, *concrete);
    }
}

/// Contract: `admit_kdf_selection` is called through the crate's public
///   surface with the member identifier of each member of `KDF_CONCRETES`
///   and a settings document that is a table holding one key →
///   `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete })` with
///   `concrete` that member.
/// Arrange: for each `concrete` in `KDF_CONCRETES`,
///   `build_admit_kdf_selection_params` with the `concrete` override
///   `build_kdf_concrete(concrete, KdfConcreteOverrides::default())`;
///   `build_admit_kdf_selection_deps` with no override; the document from
///   `invalidate_admit_kdf_selection_payload` with the `settings`
///   corruption the table `{"a_setting": 1}`, so the document supplies a
///   setting the empty-table block's document does not.
/// Act:     `admit_kdf_selection(&deps, params, document)` for the case.
/// Assert:  `result.err()` equals
///   `Some(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete })`,
///   the whole error.
/// Boundary: the public call `admit_kdf_selection` and the route the own
///   entries state.
/// Mocked:  nothing.
#[test]
fn admit_kdf_selection_refuses_a_supplied_setting_naming_each_declared_member_through_the_crate_surface()
 {
    // Arrange
    for concrete in KDF_CONCRETES {
        let deps = build_admit_kdf_selection_deps(Default::default());
        let params = build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
            concrete: Some(build_kdf_concrete(*concrete, KdfConcreteOverrides)),
        });
        let document =
            invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions {
                settings: Some(serde_json::json!({ "a_setting": 1 })),
            });

        // Act
        let result = admit_kdf_selection(&deps, params, document);

        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitKdfSelectionErrorReturn::SettingSupplied {
                concrete: *concrete,
            })
        );
    }
}

/// Contract: `admit_kdf_selection` is called through the crate's public
///   surface under the `mocks` feature with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`
///   and a settings document that is a table holding one key →
///   `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete })` with
///   `concrete` that selected mock member.
/// Arrange: `build_admit_kdf_selection_params` with the `concrete`
///   override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the selected failure mode differs from `NoFailure`;
///   `build_admit_kdf_selection_deps` with no override; the document with
///   the table `{"a_setting": 1}`.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete:
///   KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable) })`,
///   the whole error.
/// Boundary: the public call `admit_kdf_selection` and the route the own
///   entries state.
/// Mocked:  nothing; the mock member is the subject's own input.
#[test]
fn admit_kdf_selection_refuses_a_supplied_setting_naming_the_selected_mock_member_through_the_crate_surface()
 {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
        concrete: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
    });
    let document = invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions {
        settings: Some(serde_json::json!({ "a_setting": 1 })),
    });

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitKdfSelectionErrorReturn::SettingSupplied {
            concrete: KdfConcrete::Mock(
                MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable
            ),
        })
    );
}

/// Contract: `admit_kdf_selection` is called through the crate's public
///   surface with a settings document that is the JSON number `7` →
///   `Err(AdmitKdfSelectionErrorReturn::Document(AdmitKdfSelectionPayloadError::NotATable))`.
/// Arrange: `build_admit_kdf_selection_params` and
///   `build_admit_kdf_selection_deps` with no override; the document from
///   `invalidate_admit_kdf_selection_payload` with the `settings`
///   corruption `Some(Value::from(7))`.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitKdfSelectionErrorReturn::Document(AdmitKdfSelectionPayloadError::NotATable))`,
///   the whole error.
/// Boundary: the public call `admit_kdf_selection` and the route the own
///   entries state.
/// Mocked:  nothing.
#[test]
fn admit_kdf_selection_refuses_a_document_that_is_not_a_table_through_the_crate_surface() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(Default::default());
    let document = invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions {
        settings: Some(Value::from(7)),
    });

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitKdfSelectionErrorReturn::Document(
            AdmitKdfSelectionPayloadError::NotATable
        ))
    );
}

/// Contract: `admit_kdf_selection` is called through the crate's public
///   surface under the `mocks` feature with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`
///   and an empty settings document → `Ok` whose `selection` is that
///   selected mock member.
/// Arrange: `build_admit_kdf_selection_params` with the `concrete`
///   override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the selected failure mode differs from the `NoFailure` member the
///   mock constant names; `build_admit_kdf_selection_deps` with no
///   override; the document with no corruption.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an empty document supplies no setting; `success.selection`
///   equals
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`.
/// Boundary: the public call `admit_kdf_selection` and the route the own
///   entries state, through the narrowing and the `match`.
/// Mocked:  nothing; the mock member is the subject's own input.
#[test]
fn admit_kdf_selection_carries_the_selected_mock_member_through_the_crate_surface() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
        concrete: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
    });
    let document = invalidate_admit_kdf_selection_payload(Default::default());

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    let success = result.expect("an empty settings document supplies no setting");
    assert_eq!(
        success.selection,
        KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)
    );
}
