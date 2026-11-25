use super::provides::{
    AdmitEncodingSelectionErrorReturn, AdmitEncodingSelectionParamsOverrides,
    AdmitEncodingSelectionSuccessReturn, admit_encoding_selection,
    build_admit_encoding_selection_deps, build_admit_encoding_selection_params,
};
use crate::factory::provides::{
    EncodingConcrete, EncodingConcreteOverrides, build_encoding_concrete,
};
use serde_json::Value;

/// Contract: `admit_encoding_selection`, the `Abi` member, admitted:
///   `params.concrete` is `EncodingConcrete::Abi` and the document is
///   `Value::Null` → `Ok(AdmitEncodingSelectionSuccessReturn { selection:
///   EncodingConcrete::Abi })`.
/// Arrange: the deps from their builder; the params from
///   `build_admit_encoding_selection_params` overriding `concrete` with
///   `build_encoding_concrete(EncodingConcrete::Abi,
///   EncodingConcreteOverrides::default())`; the payload `Value::Null`; the
///   expected selection written as the independent literal
///   `EncodingConcrete::Abi`, which differs from the member the mock arm
///   returns, so an admission that returns the mock selection for the real
///   member fails.
/// Act:     `admit_encoding_selection(&deps, params, payload)`.
/// Assert:  `result` equals `Ok(AdmitEncodingSelectionSuccessReturn {
///   selection: EncodingConcrete::Abi })` by `assert_eq!`.
#[test]
fn admission_returns_the_abi_selection_for_an_absent_document() {
    // Arrange
    let deps = build_admit_encoding_selection_deps(Default::default());
    let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
        concrete: Some(build_encoding_concrete(
            EncodingConcrete::Abi,
            EncodingConcreteOverrides,
        )),
    });
    let payload = Value::Null;

    // Act
    let result = admit_encoding_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitEncodingSelectionSuccessReturn {
            selection: EncodingConcrete::Abi,
        })
    );
}

/// Contract: `admit_encoding_selection`, the `Abi` member, admitted:
///   `params.concrete` is `EncodingConcrete::Abi` and the document is an empty
///   `Value::Object` → `Ok(AdmitEncodingSelectionSuccessReturn { selection:
///   EncodingConcrete::Abi })`.
/// Arrange: the deps from their builder; the params as in the absent-document
///   block; the payload `json!({})`; the empty object differs from
///   `Value::Null`, so an admission that treats only `Value::Null` as absent
///   fails.
/// Act:     `admit_encoding_selection(&deps, params, payload)`.
/// Assert:  `result` equals `Ok(AdmitEncodingSelectionSuccessReturn {
///   selection: EncodingConcrete::Abi })` by `assert_eq!`.
#[test]
fn admission_returns_the_abi_selection_for_an_empty_document() {
    // Arrange
    let deps = build_admit_encoding_selection_deps(Default::default());
    let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
        concrete: Some(build_encoding_concrete(
            EncodingConcrete::Abi,
            EncodingConcreteOverrides,
        )),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_encoding_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitEncodingSelectionSuccessReturn {
            selection: EncodingConcrete::Abi,
        })
    );
}

/// Contract: `admit_encoding_selection`, the `Abi` member, a setting supplied:
///   `params.concrete` is `EncodingConcrete::Abi` and the document is a
///   non-empty `Value::Object` → `Err(AdmitEncodingSelectionErrorReturn::
///   SettingSupplied { concrete: EncodingConcrete::Abi })`.
/// Arrange: the deps from their builder; the params as in the absent-document
///   block; the payload `json!({ "limit": 1 })`, a document that differs from
///   the admitted documents only by carrying one key, so an admission that
///   ignores the document fails.
/// Act:     `admit_encoding_selection(&deps, params, payload)`.
/// Assert:  `result` equals `Err(AdmitEncodingSelectionErrorReturn::
///   SettingSupplied { concrete: EncodingConcrete::Abi })` by `assert_eq!`.
#[test]
fn admission_refuses_a_setting_for_the_abi_member() {
    // Arrange
    let deps = build_admit_encoding_selection_deps(Default::default());
    let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
        concrete: Some(build_encoding_concrete(
            EncodingConcrete::Abi,
            EncodingConcreteOverrides,
        )),
    });
    let payload = serde_json::json!({ "limit": 1 });

    // Act
    let result = admit_encoding_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitEncodingSelectionErrorReturn::SettingSupplied {
            concrete: EncodingConcrete::Abi,
        })
    );
}

/// Contract: `admit_encoding_selection`, the `Abi` member, a setting supplied:
///   `params.concrete` is `EncodingConcrete::Abi` and the document is neither
///   `Value::Null` nor an `Value::Object` →
///   `Err(AdmitEncodingSelectionErrorReturn::SettingSupplied { concrete:
///   EncodingConcrete::Abi })`.
/// Arrange: the deps from their builder; the params as in the absent-document
///   block; the payload `json!("limit")`, a scalar document, so an admission
///   that refuses only objects fails.
/// Act:     `admit_encoding_selection(&deps, params, payload)`.
/// Assert:  `result` equals `Err(AdmitEncodingSelectionErrorReturn::
///   SettingSupplied { concrete: EncodingConcrete::Abi })` by `assert_eq!`.
#[test]
fn admission_refuses_a_document_that_is_not_an_object_for_the_abi_member() {
    // Arrange
    let deps = build_admit_encoding_selection_deps(Default::default());
    let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
        concrete: Some(build_encoding_concrete(
            EncodingConcrete::Abi,
            EncodingConcreteOverrides,
        )),
    });
    let payload = serde_json::json!("limit");

    // Act
    let result = admit_encoding_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitEncodingSelectionErrorReturn::SettingSupplied {
            concrete: EncodingConcrete::Abi,
        })
    );
}

/// Contract: `admit_encoding_selection`, the `Mock` member, admitted:
///   `params.concrete` is `EncodingConcrete::Mock` and the document is
///   `Value::Null` → `Ok(AdmitEncodingSelectionSuccessReturn { selection:
///   EncodingConcrete::Mock })`.
/// Arrange: the deps from their builder; the params from
///   `build_admit_encoding_selection_params` overriding `concrete` with
///   `EncodingConcrete::Mock`; the payload `Value::Null`; the expected
///   selection written as the independent literal `EncodingConcrete::Mock`,
///   which differs from the member the real arm returns, so an admission that
///   returns the real selection for the mock member fails.
/// Act:     `admit_encoding_selection(&deps, params, payload)`.
/// Assert:  `result` equals `Ok(AdmitEncodingSelectionSuccessReturn {
///   selection: EncodingConcrete::Mock })` by `assert_eq!`.
#[test]
fn admission_returns_the_mock_selection_for_an_absent_document() {
    // Arrange
    let deps = build_admit_encoding_selection_deps(Default::default());
    let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
        concrete: Some(EncodingConcrete::Mock),
    });
    let payload = Value::Null;

    // Act
    let result = admit_encoding_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitEncodingSelectionSuccessReturn {
            selection: EncodingConcrete::Mock,
        })
    );
}

/// Contract: `admit_encoding_selection`, the `Mock` member, a setting
///   supplied: `params.concrete` is `EncodingConcrete::Mock` and the document
///   is a non-empty `Value::Object` → `Err(AdmitEncodingSelectionErrorReturn::
///   SettingSupplied { concrete: EncodingConcrete::Mock })`.
/// Arrange: the deps from their builder; the params as in the mock
///   absent-document block; the payload `json!({ "limit": 1 })`; the expected
///   error names the mock member, which differs from the real member, so an
///   error that names the wrong member fails.
/// Act:     `admit_encoding_selection(&deps, params, payload)`.
/// Assert:  `result` equals `Err(AdmitEncodingSelectionErrorReturn::
///   SettingSupplied { concrete: EncodingConcrete::Mock })` by `assert_eq!`.
#[test]
fn admission_refuses_a_setting_for_the_mock_member() {
    // Arrange
    let deps = build_admit_encoding_selection_deps(Default::default());
    let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
        concrete: Some(EncodingConcrete::Mock),
    });
    let payload = serde_json::json!({ "limit": 1 });

    // Act
    let result = admit_encoding_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitEncodingSelectionErrorReturn::SettingSupplied {
            concrete: EncodingConcrete::Mock,
        })
    );
}
