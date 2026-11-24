use super::provides::{
    AdmitHashingSelectionErrorReturn, AdmitHashingSelectionParamsOverrides,
    AdmitHashingSelectionSuccessReturn, admit_hashing_selection,
    build_admit_hashing_selection_deps, build_admit_hashing_selection_params,
};

use crate::factory::provides::{CommitmentConcrete, MockICommitmentAdapterFailureMode};
use serde_json::Value;

/// Contract: `admit_hashing_selection`, member `Blake3Bao`, document absent:
///   `params.concrete` is `CommitmentConcrete::Blake3Bao` and the document is
///   `Value::Null` → `Ok` whose selection is `CommitmentConcrete::Blake3Bao`.
/// Arrange: `build_admit_hashing_selection_params` with the `concrete` override
///   `CommitmentConcrete::Blake3Bao`; the payload `Value::Null`.
/// Act:     `admit_hashing_selection(&deps, params, payload)`.
/// Assert:  the result equals `Ok` holding the literal selection
///   `CommitmentConcrete::Blake3Bao`.
#[test]
fn admit_returns_the_blake3_bao_selection_for_an_absent_document() {
    // Arrange
    let deps = build_admit_hashing_selection_deps(Default::default());
    let params = build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
        concrete: Some(CommitmentConcrete::Blake3Bao),
    });
    let payload = Value::Null;

    // Act
    let result = admit_hashing_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitHashingSelectionSuccessReturn {
            selection: CommitmentConcrete::Blake3Bao,
        })
    );
}

/// Contract: `admit_hashing_selection`, member `Blake3Bao`, document empty:
///   `params.concrete` is `CommitmentConcrete::Blake3Bao` and the document is a
///   JSON object with no entry → `Ok` whose selection is
///   `CommitmentConcrete::Blake3Bao`.
/// Arrange: the params as above; the payload `Value::Object` over an empty
///   `serde_json::Map`, an object that differs from the supplying document of
///   the refusal blocks only in holding no entry, so an admission that refuses
///   every object fails.
/// Act:     `admit_hashing_selection(&deps, params, payload)`.
/// Assert:  the result equals `Ok` holding the literal selection
///   `CommitmentConcrete::Blake3Bao`.
#[test]
fn admit_returns_the_blake3_bao_selection_for_an_empty_document() {
    // Arrange
    let deps = build_admit_hashing_selection_deps(Default::default());
    let params = build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
        concrete: Some(CommitmentConcrete::Blake3Bao),
    });
    let payload = Value::Object(serde_json::Map::new());

    // Act
    let result = admit_hashing_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitHashingSelectionSuccessReturn {
            selection: CommitmentConcrete::Blake3Bao,
        })
    );
}

/// Contract: `admit_hashing_selection`, member `Mock`, document absent:
///   `params.concrete` is `CommitmentConcrete::Mock(failure_mode)` and the
///   document is `Value::Null` → `Ok` whose selection is
///   `CommitmentConcrete::Mock(failure_mode)`.
/// Arrange: `build_admit_hashing_selection_params` with the `concrete` override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`,
///   a member other than the real member and a mode other than the `NoFailure`
///   the family's mock constant holds, so an admission that returns a fixed
///   member or an unsubstituted mode fails; the payload `Value::Null`.
/// Act:     `admit_hashing_selection(&deps, params, payload)`.
/// Assert:  the result equals `Ok` holding the literal selection
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`.
#[test]
fn admit_returns_the_mock_selection_naming_the_selected_failure_mode() {
    // Arrange
    let deps = build_admit_hashing_selection_deps(Default::default());
    let params = build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable,
        )),
    });
    let payload = Value::Null;

    // Act
    let result = admit_hashing_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitHashingSelectionSuccessReturn {
            selection: CommitmentConcrete::Mock(
                MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable,
            ),
        })
    );
}

/// Contract: `admit_hashing_selection`, member `Blake3Bao`, document supplies a
///   setting: `params.concrete` is `CommitmentConcrete::Blake3Bao` and the
///   document is a JSON object holding an entry →
///   `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete })`.
/// Arrange: the params with the `concrete` override
///   `CommitmentConcrete::Blake3Bao`; the payload
///   `serde_json::json!({ "setting": 1 })`, an object that differs from the
///   empty object of the accepting block only in holding an entry.
/// Act:     `admit_hashing_selection(&deps, params, payload)`.
/// Assert:  the result equals
///   `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete: CommitmentConcrete::Blake3Bao })`.
#[test]
fn admit_refuses_a_supplied_setting_naming_the_blake3_bao_member() {
    // Arrange
    let deps = build_admit_hashing_selection_deps(Default::default());
    let params = build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
        concrete: Some(CommitmentConcrete::Blake3Bao),
    });
    let payload = serde_json::json!({ "setting": 1 });

    // Act
    let result = admit_hashing_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitHashingSelectionErrorReturn::SettingsSupplied {
            concrete: CommitmentConcrete::Blake3Bao,
        })
    );
}

/// Contract: `admit_hashing_selection`, member `Mock`, document supplies a
///   setting: `params.concrete` is `CommitmentConcrete::Mock(failure_mode)` and
///   the document is a JSON object holding an entry →
///   `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete })`
///   naming the mock member with its failure mode.
/// Arrange: `build_admit_hashing_selection_params` with the `concrete` override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::OffsetUnrepresentable)`;
///   the payload `serde_json::json!({ "setting": 1 })`.
/// Act:     `admit_hashing_selection(&deps, params, payload)`.
/// Assert:  the result equals
///   `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete: CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::OffsetUnrepresentable) })`.
#[test]
fn admit_refuses_a_supplied_setting_naming_the_mock_member() {
    // Arrange
    let deps = build_admit_hashing_selection_deps(Default::default());
    let params = build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::OffsetUnrepresentable,
        )),
    });
    let payload = serde_json::json!({ "setting": 1 });

    // Act
    let result = admit_hashing_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitHashingSelectionErrorReturn::SettingsSupplied {
            concrete: CommitmentConcrete::Mock(
                MockICommitmentAdapterFailureMode::OffsetUnrepresentable,
            ),
        })
    );
}

/// Contract: `admit_hashing_selection`, member `Blake3Bao`, document neither
///   absent nor an object: `params.concrete` is `CommitmentConcrete::Blake3Bao`
///   and the document is neither `Value::Null` nor an object →
///   `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete })`.
/// Arrange: the params with the `concrete` override
///   `CommitmentConcrete::Blake3Bao`; the payload `serde_json::json!([1])`, an
///   array, a value that is neither the absent document nor an object.
/// Act:     `admit_hashing_selection(&deps, params, payload)`.
/// Assert:  the result equals
///   `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete: CommitmentConcrete::Blake3Bao })`.
#[test]
fn admit_refuses_a_document_that_is_not_an_object() {
    // Arrange
    let deps = build_admit_hashing_selection_deps(Default::default());
    let params = build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
        concrete: Some(CommitmentConcrete::Blake3Bao),
    });
    let payload = serde_json::json!([1]);

    // Act
    let result = admit_hashing_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitHashingSelectionErrorReturn::SettingsSupplied {
            concrete: CommitmentConcrete::Blake3Bao,
        })
    );
}
