use super::admit_hash_to_scalar_selection;

use super::interface::AdmitHashToScalarSelectionErrorReturn;
use super::mock::{
    AdmitHashToScalarSelectionParamsOverrides, build_admit_hash_to_scalar_selection_deps,
    build_admit_hash_to_scalar_selection_params,
};

use crate::factory::provides::{HashToScalarConcrete, MockIHashToScalarAdapterFailureMode};

use serde_json::{Map, Value};

/// Contract: `admit_hash_to_scalar_selection`, real member, null document:
///   `params.concrete` is `HashToScalarConcrete::Keccak256` and the payload is
///   `Value::Null` → `Ok` whose `selection` is the `concrete` of the
///   declaration the lookup returned.
/// Collaborators: `lookup_hash_to_scalar_declaration` is mocked; the mock
///   returns the declaration naming
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::Succeeds)`.
/// Arrange: `build_admit_hash_to_scalar_selection_deps` with no override;
///   `build_admit_hash_to_scalar_selection_params` with the `concrete`
///   override `HashToScalarConcrete::Keccak256`, a member the default mock
///   declaration does not name, so the selection read from the declaration is
///   distinguishable from the identifier given.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, Value::Null)`.
/// Assert:  `result.ok().map(|success| success.selection)` equals
///   `Some(HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::Succeeds))`.
#[test]
fn admission_returns_the_selection_the_declaration_names_for_a_null_document_for_the_real_member() {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());
    let params =
        build_admit_hash_to_scalar_selection_params(AdmitHashToScalarSelectionParamsOverrides {
            concrete: Some(HashToScalarConcrete::Keccak256),
        });

    // Act
    let result = admit_hash_to_scalar_selection(&deps, params, Value::Null);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.selection),
        Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::Succeeds
        ))
    );
}

/// Contract: `admit_hash_to_scalar_selection`, real member, empty object
///   document: `params.concrete` is `HashToScalarConcrete::Keccak256` and the
///   payload is an empty `Value::Object` → `Ok` whose `selection` is the
///   `concrete` of the declaration the lookup returned.
/// Collaborators: `lookup_hash_to_scalar_declaration` is mocked; the mock
///   returns the declaration naming
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::Succeeds)`.
/// Arrange: the same deps and params as the null-document block; payload
///   `Value::Object(Map::new())`.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, payload)`.
/// Assert:  `result.ok().map(|success| success.selection)` equals
///   `Some(HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::Succeeds))`.
#[test]
fn admission_returns_the_selection_the_declaration_names_for_an_empty_object_document_for_the_real_member()
 {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());
    let params =
        build_admit_hash_to_scalar_selection_params(AdmitHashToScalarSelectionParamsOverrides {
            concrete: Some(HashToScalarConcrete::Keccak256),
        });

    // Act
    let result = admit_hash_to_scalar_selection(&deps, params, Value::Object(Map::new()));

    // Assert
    assert_eq!(
        result.ok().map(|success| success.selection),
        Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::Succeeds
        ))
    );
}

/// Contract: `admit_hash_to_scalar_selection`, real member, object document
///   carrying a key: `params.concrete` is `HashToScalarConcrete::Keccak256`
///   and the payload is a `Value::Object` holding a key →
///   `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: HashToScalarConcrete::Keccak256 })`.
/// Collaborators: none read; the refusal precedes the lookup call.
/// Arrange: `build_admit_hash_to_scalar_selection_deps` with no override;
///   `build_admit_hash_to_scalar_selection_params` with the `concrete`
///   override `HashToScalarConcrete::Keccak256`; payload a `Value::Object`
///   from a `Map` holding the key `unrecognized_setting` with the value
///   `Value::from(1)`, against the empty object and null documents the
///   admitted blocks use.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: HashToScalarConcrete::Keccak256 })`.
#[test]
fn admission_refuses_an_object_document_carrying_a_key_for_the_real_member() {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());
    let params =
        build_admit_hash_to_scalar_selection_params(AdmitHashToScalarSelectionParamsOverrides {
            concrete: Some(HashToScalarConcrete::Keccak256),
        });
    let mut map = Map::new();
    map.insert("unrecognized_setting".to_string(), Value::from(1));

    // Act
    let result = admit_hash_to_scalar_selection(&deps, params, Value::Object(map));

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied {
            concrete: HashToScalarConcrete::Keccak256
        })
    );
}

/// Contract: `admit_hash_to_scalar_selection`, real member, non-object
///   document: `params.concrete` is `HashToScalarConcrete::Keccak256` and the
///   payload is neither `Value::Null` nor an object →
///   `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: HashToScalarConcrete::Keccak256 })`.
/// Collaborators: none read; the refusal precedes the lookup call.
/// Arrange: `build_admit_hash_to_scalar_selection_deps` with no override;
///   `build_admit_hash_to_scalar_selection_params` with the `concrete`
///   override `HashToScalarConcrete::Keccak256`; payload `Value::from(true)`.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: HashToScalarConcrete::Keccak256 })`.
#[test]
fn admission_refuses_a_non_object_document_for_the_real_member() {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());
    let params =
        build_admit_hash_to_scalar_selection_params(AdmitHashToScalarSelectionParamsOverrides {
            concrete: Some(HashToScalarConcrete::Keccak256),
        });

    // Act
    let result = admit_hash_to_scalar_selection(&deps, params, Value::from(true));

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied {
            concrete: HashToScalarConcrete::Keccak256
        })
    );
}

/// Contract: `admit_hash_to_scalar_selection`, mock member, null document:
///   `params.concrete` is `HashToScalarConcrete::Mock(mode)` and the payload is
///   `Value::Null` → `Ok` whose `selection` is the `concrete` of the
///   declaration the lookup returned.
/// Collaborators: `lookup_hash_to_scalar_declaration` is mocked; the mock
///   returns the declaration naming
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::Succeeds)`.
/// Arrange: `build_admit_hash_to_scalar_selection_deps` with no override;
///   `build_admit_hash_to_scalar_selection_params` with the `concrete`
///   override
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`,
///   a mode other than the `Succeeds` mode the default lookup's declaration
///   names.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, Value::Null)`.
/// Assert:  `result.ok().map(|success| success.selection)` equals
///   `Some(HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::Succeeds))`.
#[test]
fn admission_returns_the_selection_the_declaration_names_for_a_null_document_for_a_mock_member() {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());
    let params =
        build_admit_hash_to_scalar_selection_params(AdmitHashToScalarSelectionParamsOverrides {
            concrete: Some(HashToScalarConcrete::Mock(
                MockIHashToScalarAdapterFailureMode::SamplingRefused,
            )),
        });

    // Act
    let result = admit_hash_to_scalar_selection(&deps, params, Value::Null);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.selection),
        Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::Succeeds
        ))
    );
}

/// Contract: `admit_hash_to_scalar_selection`, mock member, object document
///   carrying a key: `params.concrete` is `HashToScalarConcrete::Mock(mode)`
///   and the payload is a `Value::Object` holding a key →
///   `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: HashToScalarConcrete::Mock(mode) })`.
/// Collaborators: none read; the refusal precedes the lookup call.
/// Arrange: `build_admit_hash_to_scalar_selection_deps` with no override;
///   `build_admit_hash_to_scalar_selection_params` with the `concrete`
///   override
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`;
///   payload a `Value::Object` holding the key `unrecognized_setting` with the
///   value `Value::from(1)`.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused) })`.
#[test]
fn admission_refuses_an_object_document_carrying_a_key_for_a_mock_member() {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());
    let params =
        build_admit_hash_to_scalar_selection_params(AdmitHashToScalarSelectionParamsOverrides {
            concrete: Some(HashToScalarConcrete::Mock(
                MockIHashToScalarAdapterFailureMode::SamplingRefused,
            )),
        });
    let mut map = Map::new();
    map.insert("unrecognized_setting".to_string(), Value::from(1));

    // Act
    let result = admit_hash_to_scalar_selection(&deps, params, Value::Object(map));

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied {
            concrete: HashToScalarConcrete::Mock(
                MockIHashToScalarAdapterFailureMode::SamplingRefused
            )
        })
    );
}
