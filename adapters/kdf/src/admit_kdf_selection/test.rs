#![allow(clippy::expect_used)]

use super::provides::{
    AdmitKdfSelectionErrorReturn, AdmitKdfSelectionParamsOverrides, admit_kdf_selection,
    build_admit_kdf_selection_deps, build_admit_kdf_selection_params,
};
use crate::factory::provides::{KdfConcrete, MockIKeyDerivationAdapterFailureMode};
use serde_json::{Map, Value, json};

/// Contract: `admit_kdf_selection`, real member, admitted: `params.concrete`
///   is `KdfConcrete::Blake3Keyed` and the document is an empty table →
///   `Ok` whose `selection` is `KdfConcrete::Blake3Keyed`.
/// Collaborators: none.
/// Arrange: `build_admit_kdf_selection_deps` with no override;
///   `build_admit_kdf_selection_params` with no override, whose default
///   member is the real `Blake3Keyed`; the document `Value::Object` over an
///   empty `serde_json::Map`, written directly, so the document differs
///   from the documents the refusal blocks supply.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an empty table supplies no setting; `success.selection` matches
///   `KdfConcrete::Blake3Keyed`.
#[test]
fn admit_kdf_selection_returns_the_blake3_keyed_selection_for_an_empty_settings_table() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(Default::default());
    let document = Value::Object(Map::new());

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    let success = result.expect("an empty settings table supplies no setting");
    assert!(matches!(success.selection, KdfConcrete::Blake3Keyed));
}

/// Contract: `admit_kdf_selection`, real member, admitted: `params.concrete`
///   is `KdfConcrete::Blake3Keyed` and the document is the JSON null →
///   `Ok` whose `selection` is `KdfConcrete::Blake3Keyed`.
/// Collaborators: none.
/// Arrange: the built deps and params as in the empty-table block; the
///   document `Value::Null`, written directly, so the document is absent
///   rather than an empty table.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an absent document supplies no setting; `success.selection`
///   matches `KdfConcrete::Blake3Keyed`.
#[test]
fn admit_kdf_selection_returns_the_blake3_keyed_selection_for_an_absent_document() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(Default::default());
    let document = Value::Null;

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    let success = result.expect("an absent document supplies no setting");
    assert!(matches!(success.selection, KdfConcrete::Blake3Keyed));
}

/// Contract: `admit_kdf_selection`, mock member, admitted: `params.concrete`
///   is
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`
///   and the document is an empty table → `Ok` whose `selection` is that
///   selected mock member and not
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::NoFailure)`.
/// Collaborators: none.
/// Arrange: `build_admit_kdf_selection_params` with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the selected failure mode differs from the `NoFailure` member the
///   mock constant names; the built deps and the empty-table document as in
///   the first block.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an empty table supplies no setting; `success.selection` matches
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`.
#[test]
fn admit_kdf_selection_returns_the_selected_mock_selection_carrying_its_failure_mode() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
        concrete: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
    });
    let document = Value::Object(Map::new());

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    let success = result.expect("an empty settings table supplies no setting");
    assert!(matches!(
        success.selection,
        KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)
    ));
}

/// Contract: `admit_kdf_selection`, real member, a setting supplied:
///   `params.concrete` is `KdfConcrete::Blake3Keyed` and the document is a
///   table holding one key →
///   `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete:
///   KdfConcrete::Blake3Keyed })`.
/// Collaborators: none.
/// Arrange: the built deps and params with no override; the document
///   `serde_json::json!({ "a_setting": 1 })`, written directly, so the
///   document supplies a setting the empty-table block's document does
///   not.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete:
///   KdfConcrete::Blake3Keyed })`, the whole error.
#[test]
fn admit_kdf_selection_refuses_a_supplied_setting_naming_the_blake3_keyed_member() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(Default::default());
    let document = json!({ "a_setting": 1 });

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitKdfSelectionErrorReturn::SettingSupplied {
            concrete: KdfConcrete::Blake3Keyed,
        })
    );
}

/// Contract: `admit_kdf_selection`, mock member, a setting supplied:
///   `params.concrete` is
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`
///   and the document is a table holding one key →
///   `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete })` with
///   `concrete` that selected mock member.
/// Collaborators: none.
/// Arrange: `build_admit_kdf_selection_params` with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`;
///   the built deps; the document with the table `{"a_setting": 1}` as the
///   previous block.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete:
///   KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable) })`,
///   the whole error.
#[test]
fn admit_kdf_selection_refuses_a_supplied_setting_naming_the_selected_mock_member() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
        concrete: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
    });
    let document = json!({ "a_setting": 1 });

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

/// Contract: `admit_kdf_selection`, document refused: the document is the
///   JSON number `7`, neither `Value::Null` nor a `Value::Object`, so it
///   supplies a setting →
///   `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete:
///   KdfConcrete::Blake3Keyed })`.
/// Collaborators: none mocked.
/// Arrange: the built deps and params with no override; the document
///   `Value::from(7)`, written directly.
/// Act:     `admit_kdf_selection(&deps, params, document)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete:
///   KdfConcrete::Blake3Keyed })`, the whole error.
#[test]
fn admit_kdf_selection_refuses_a_non_object_document_as_a_supplied_setting() {
    // Arrange
    let deps = build_admit_kdf_selection_deps(Default::default());
    let params = build_admit_kdf_selection_params(Default::default());
    let document = Value::from(7);

    // Act
    let result = admit_kdf_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitKdfSelectionErrorReturn::SettingSupplied {
            concrete: KdfConcrete::Blake3Keyed,
        })
    );
}
