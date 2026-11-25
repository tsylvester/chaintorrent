#![cfg(feature = "mocks")]

use hash_to_scalar::{
    AdmitHashToScalarSelectionDeps, AdmitHashToScalarSelectionErrorReturn,
    AdmitHashToScalarSelectionParamsOverrides, HASH_TO_SCALAR_CONCRETES, HashToScalarConcrete,
    MockIHashToScalarAdapterFailureMode, admit_hash_to_scalar_selection,
    build_admit_hash_to_scalar_selection_deps, build_admit_hash_to_scalar_selection_params,
    build_hash_to_scalar_concrete,
};
use serde_json::{Map, Value};

/// Contract: entry `every_declared_member_is_admitted_to_the_selection_that_names_it`;
///   for each member of `HASH_TO_SCALAR_CONCRETES` and each of `Value::Null`
///   and an empty `Value::Object`, the admission yields `Ok` whose `selection`
///   equals that member, the admitted member identifier returned unchanged.
/// Arrange: `build_admit_hash_to_scalar_selection_deps` with no override, the
///   fieldless deps; `build_admit_hash_to_scalar_selection_params` per member
///   with the `concrete` override
///   `build_hash_to_scalar_concrete(*member, Default::default())`; the body is
///   written once and names no concrete.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, payload)` per
///   member over each of `Value::Null` and `Value::Object(Map::new())`.
/// Assert:  per member and document,
///   `result.ok().map(|success| success.selection)` equals `Some(*member)`.
/// Boundary: the public call `admit_hash_to_scalar_selection` alone, the
///   function calling no collaborator.
/// Mocked:  none.
#[test]
fn every_declared_member_is_admitted_to_the_selection_that_names_it() {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());

    for member in HASH_TO_SCALAR_CONCRETES {
        for payload in [Value::Null, Value::Object(Map::new())] {
            let params = build_admit_hash_to_scalar_selection_params(
                AdmitHashToScalarSelectionParamsOverrides {
                    concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
                },
            );

            // Act
            let result = admit_hash_to_scalar_selection(&deps, params, payload);

            // Assert
            assert_eq!(result.ok().map(|success| success.selection), Some(*member));
        }
    }
}

/// Contract: entry `a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock`;
///   for `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`
///   and an empty `Value::Object`, the admission yields `Ok` whose `selection`
///   equals that member, the selected mock member carrying the mode selected.
/// Arrange: `AdmitHashToScalarSelectionDeps` written directly, the fieldless
///   deps; `build_admit_hash_to_scalar_selection_params` with the `concrete`
///   override `HashToScalarConcrete::Mock(SamplingRefused)`, a mode other than
///   the `Succeeds` mode the mock constant names; payload
///   `Value::Object(Map::new())`.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, payload)`.
/// Assert:  `result.ok().map(|success| success.selection)` equals
///   `Some(HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused))`.
/// Boundary: the public call `admit_hash_to_scalar_selection` alone.
/// Mocked:  the family's mock member, selected by the params, the official
///   collaborator the `mocks` feature provides.
#[test]
fn a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock() {
    // Arrange
    let deps = AdmitHashToScalarSelectionDeps;
    let params =
        build_admit_hash_to_scalar_selection_params(AdmitHashToScalarSelectionParamsOverrides {
            concrete: Some(HashToScalarConcrete::Mock(
                MockIHashToScalarAdapterFailureMode::SamplingRefused,
            )),
        });

    // Act
    let result = admit_hash_to_scalar_selection(&deps, params, Value::Object(Map::new()));

    // Assert
    assert_eq!(
        result.ok().map(|success| success.selection),
        Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused
        ))
    );
}

/// Contract: entry `a_setting_supplied_for_a_member_is_refused_naming_the_member`;
///   for each member of `HASH_TO_SCALAR_CONCRETES` and for the
///   `SamplingRefused` mock member, each of an object holding a key and
///   `Value::from(true)` yields
///   `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete })`
///   with `concrete` that member.
/// Arrange: `build_admit_hash_to_scalar_selection_deps` with no override, the
///   fieldless deps; params per member; payloads a `Value::Object` holding the
///   key `unrecognized_setting` with the value `Value::from(1)` and
///   `Value::from(true)`, each written directly.
/// Act:     `admit_hash_to_scalar_selection(&deps, params, payload)` per
///   member over each payload.
/// Assert:  per member and payload, `result.err()` equals
///   `Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: that member })`.
/// Boundary: the public call `admit_hash_to_scalar_selection`.
/// Mocked:  the family's mock member for the mock arrangement.
#[test]
fn a_setting_supplied_for_a_member_is_refused_naming_the_member() {
    // Arrange
    let deps = build_admit_hash_to_scalar_selection_deps(Default::default());

    for member in HASH_TO_SCALAR_CONCRETES {
        let mut map = Map::new();
        map.insert("unrecognized_setting".to_string(), Value::from(1));
        for payload in [Value::Object(map), Value::from(true)] {
            let params = build_admit_hash_to_scalar_selection_params(
                AdmitHashToScalarSelectionParamsOverrides {
                    concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
                },
            );

            // Act
            let result = admit_hash_to_scalar_selection(&deps, params, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: *member })
            );
        }
    }

    let mut map = Map::new();
    map.insert("unrecognized_setting".to_string(), Value::from(1));
    for payload in [Value::Object(map), Value::from(true)] {
        let params = build_admit_hash_to_scalar_selection_params(
            AdmitHashToScalarSelectionParamsOverrides {
                concrete: Some(HashToScalarConcrete::Mock(
                    MockIHashToScalarAdapterFailureMode::SamplingRefused,
                )),
            },
        );

        // Act
        let result = admit_hash_to_scalar_selection(&deps, params, payload);

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
}
