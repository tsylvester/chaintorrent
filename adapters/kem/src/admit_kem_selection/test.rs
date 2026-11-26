#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_kem_selection;
use super::interface::{AdmitKemSelectionErrorReturn, AdmitKemSelectionSuccessReturn};
use super::mock::{
    AdmitKemSelectionParamsOverrides, build_admit_kem_selection_deps,
    build_admit_kem_selection_params,
};
use crate::factory::provides::KemConcrete;
use serde_json::{Map, Value, json};

/// Contract: the real member's admitted branch — `params.concrete` is
///   `KemConcrete::Bb1DepthOne` and the document supplies no setting → `Ok`
///   holding the selection naming it.
/// Arrange: the deps from `build_admit_kem_selection_deps`, the params from
///   `build_admit_kem_selection_params` with `concrete`
///   `KemConcrete::Bb1DepthOne`, and the payload `Value::Object(Map::new())`,
///   the table with no entries, the variation separating it from the refusal
///   blocks below.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Ok(AdmitKemSelectionSuccessReturn { selection: KemConcrete::Bb1DepthOne })`.
#[test]
fn admits_the_bb1_depth_one_member_for_a_table_with_no_entries() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = Value::Object(Map::new());

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitKemSelectionSuccessReturn {
            selection: KemConcrete::Bb1DepthOne,
        })
    );
}
// admits_the_bb1_depth_one_member_for_a_table_with_no_entries

/// Contract: the real member's admitted branch for the other document that
///   supplies no setting — `params.concrete` is `KemConcrete::Bb1DepthOne` and
///   the payload is `Value::Null` → `Ok` holding the selection naming it.
/// Arrange: as `admits_the_bb1_depth_one_member_for_a_table_with_no_entries`
///   with the payload `Value::Null`, a form distinct from the table with no
///   entries, so an admission that treats only an empty table as no setting
///   fails.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Ok(AdmitKemSelectionSuccessReturn { selection: KemConcrete::Bb1DepthOne })`.
#[test]
fn admits_the_bb1_depth_one_member_for_a_null_document() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = Value::Null;

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitKemSelectionSuccessReturn {
            selection: KemConcrete::Bb1DepthOne,
        })
    );
}
// admits_the_bb1_depth_one_member_for_a_null_document

/// Contract: the real member's refused branch — `params.concrete` is
///   `KemConcrete::Bb1DepthOne` and the document supplies a setting → `Err`
///   naming the member.
/// Arrange: as `admits_the_bb1_depth_one_member_for_a_table_with_no_entries`
///   with the payload `json!({ "bound": 1 })`, a table with one entry, against
///   the table with no entries the first block admits.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete:
///   KemConcrete::Bb1DepthOne })`.
#[test]
fn refuses_a_table_with_an_entry_naming_the_bb1_depth_one_member() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = json!({ "bound": 1 });

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
            concrete: KemConcrete::Bb1DepthOne,
        })
    );
}
// refuses_a_table_with_an_entry_naming_the_bb1_depth_one_member

/// Contract: the real member's refused branch for a value that is not a table —
///   `params.concrete` is `KemConcrete::Bb1DepthOne` and the payload is a
///   `Value::Array` → `Err` naming the member.
/// Arrange: as `admits_the_bb1_depth_one_member_for_a_table_with_no_entries`
///   with the payload `Value::Array(Vec::new())`, an array with no element, so
///   a branch that tested emptiness alone and not the table fails.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete:
///   KemConcrete::Bb1DepthOne })`.
#[test]
fn refuses_a_document_that_is_not_a_table_naming_the_bb1_depth_one_member() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = Value::Array(Vec::new());

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
            concrete: KemConcrete::Bb1DepthOne,
        })
    );
}
// refuses_a_document_that_is_not_a_table_naming_the_bb1_depth_one_member

/// Contract: the real member's refused branch for a boolean — `params.concrete`
///   is `KemConcrete::Bb1DepthOne` and the payload is a `Value::Bool` → `Err`
///   naming the member.
/// Arrange: as `admits_the_bb1_depth_one_member_for_a_table_with_no_entries`
///   with the payload `Value::Bool(true)`, a scalar, so an admission that
///   treats a non-table as absent fails.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete:
///   KemConcrete::Bb1DepthOne })`.
#[test]
fn refuses_a_boolean_document_naming_the_bb1_depth_one_member() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = Value::Bool(true);

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
            concrete: KemConcrete::Bb1DepthOne,
        })
    );
}
// refuses_a_boolean_document_naming_the_bb1_depth_one_member

/// Contract: the real member's refused branch for a number — `params.concrete`
///   is `KemConcrete::Bb1DepthOne` and the payload is a `Value::Number` → `Err`
///   naming the member.
/// Arrange: as `admits_the_bb1_depth_one_member_for_a_table_with_no_entries`
///   with the payload `Value::from(7_u64)`, a number, so an admission that
///   treats a non-table as absent fails.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete:
///   KemConcrete::Bb1DepthOne })`.
#[test]
fn refuses_a_number_document_naming_the_bb1_depth_one_member() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = Value::from(7_u64);

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
            concrete: KemConcrete::Bb1DepthOne,
        })
    );
}
// refuses_a_number_document_naming_the_bb1_depth_one_member

/// Contract: the real member's refused branch for a string — `params.concrete`
///   is `KemConcrete::Bb1DepthOne` and the payload is a `Value::String` → `Err`
///   naming the member.
/// Arrange: as `admits_the_bb1_depth_one_member_for_a_table_with_no_entries`
///   with the payload `Value::String(String::from("text"))`, a string, so an
///   admission that treats a non-table as absent fails.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete:
///   KemConcrete::Bb1DepthOne })`.
#[test]
fn refuses_a_string_document_naming_the_bb1_depth_one_member() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = Value::String(String::from("text"));

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
            concrete: KemConcrete::Bb1DepthOne,
        })
    );
}
// refuses_a_string_document_naming_the_bb1_depth_one_member

/// Contract: the mock member's admitted branch — `params.concrete` is
///   `KemConcrete::Mock` and the document supplies no setting → `Ok` holding
///   the selection naming it.
/// Arrange: as `admits_the_bb1_depth_one_member_for_a_table_with_no_entries`
///   with `concrete` `KemConcrete::Mock`.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Ok(AdmitKemSelectionSuccessReturn { selection: KemConcrete::Mock })`,
///   the selection differing from the real member's so a branch that returned
///   one selection for both members fails.
#[test]
fn admits_the_mock_member_for_a_table_with_no_entries() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Mock),
    });
    let payload = Value::Object(Map::new());

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitKemSelectionSuccessReturn {
            selection: KemConcrete::Mock,
        })
    );
}
// admits_the_mock_member_for_a_table_with_no_entries

/// Contract: the mock member's refused branch — `params.concrete` is
///   `KemConcrete::Mock` and the document supplies a setting → `Err` naming
///   the member.
/// Arrange: as `refuses_a_table_with_an_entry_naming_the_bb1_depth_one_member`
///   with `concrete` `KemConcrete::Mock`.
/// Act:     `admit_kem_selection` once.
/// Assert:  the whole result equals
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete:
///   KemConcrete::Mock })`.
#[test]
fn refuses_a_table_with_an_entry_naming_the_mock_member() {
    // Arrange
    let deps = build_admit_kem_selection_deps(Default::default());
    let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
        concrete: Some(KemConcrete::Mock),
    });
    let payload = json!({ "bound": 1 });

    // Act
    let result = admit_kem_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
            concrete: KemConcrete::Mock,
        })
    );
}
// refuses_a_table_with_an_entry_naming_the_mock_member
