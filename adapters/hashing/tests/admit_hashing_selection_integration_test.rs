#![cfg(feature = "mocks")]

use hashing::{
    AdmitHashingSelectionErrorReturn, AdmitHashingSelectionParamsOverrides,
    AdmitHashingSelectionSuccessReturn, COMMITMENT_CONCRETES, admit_hashing_selection,
    build_admit_hashing_selection_deps, build_admit_hashing_selection_params,
};
use serde_json::Value;

/// Contract: `admit_hashing_selection` is called through the crate's public
///   surface once for each member of `COMMITMENT_CONCRETES`, first with a
///   document that is `Value::Null` and then with a document that is a JSON
///   object holding an entry → the first call returns `Ok` whose `selection`
///   equals the member the params named, and the second returns
///   `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete })`
///   with `concrete` equal to that member.
/// Arrange: for each member of `COMMITMENT_CONCRETES`,
///   `build_admit_hashing_selection_params` with the `concrete` override that
///   member; the documents `Value::Null` and
///   `serde_json::json!({ "setting": 1 })`; the body is written once and run
///   over the declared set.
/// Act:     `admit_hashing_selection(&deps, params, payload)` over each
///   document, once per member.
/// Assert:  for each member the call with `Value::Null` returns `Ok` whose
///   selection equals that member, and the call with the supplying document
///   returns `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied)`
///   naming that member.
/// Boundary: the public call `admit_hashing_selection` from the crate root,
///   running the `match` arm of the selected member.
/// Mocked:  nothing.
#[test]
fn every_declared_member_is_selected_through_the_public_surface() {
    for member in COMMITMENT_CONCRETES {
        // Arrange
        let deps = build_admit_hashing_selection_deps(Default::default());

        // Act
        let absent = admit_hashing_selection(
            &deps,
            build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
                concrete: Some(*member),
            }),
            Value::Null,
        );
        let supplied = admit_hashing_selection(
            &deps,
            build_admit_hashing_selection_params(AdmitHashingSelectionParamsOverrides {
                concrete: Some(*member),
            }),
            serde_json::json!({ "setting": 1 }),
        );

        // Assert
        assert_eq!(
            absent,
            Ok(AdmitHashingSelectionSuccessReturn { selection: *member })
        );
        assert_eq!(
            supplied,
            Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete: *member })
        );
    }
}
