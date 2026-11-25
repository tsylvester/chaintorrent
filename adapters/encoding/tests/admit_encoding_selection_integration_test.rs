#![cfg(feature = "mocks")]

use encoding::{
    AdmitEncodingSelectionErrorReturn, AdmitEncodingSelectionParamsOverrides,
    AdmitEncodingSelectionSuccessReturn, ENCODING_CONCRETES, EncodingConcreteOverrides,
    admit_encoding_selection, build_admit_encoding_selection_deps,
    build_admit_encoding_selection_params, build_encoding_concrete,
};

/// Contract: own entry `declared_member_selection_through_the_public_surface`:
///   for each member of `ENCODING_CONCRETES`, `admit_encoding_selection`
///   through the crate's public surface with a document that supplies no
///   setting → `Ok` of the selection naming that member.
/// Arrange: for each member of `ENCODING_CONCRETES`, the params from
///   `build_admit_encoding_selection_params` overriding `concrete` with
///   `build_encoding_concrete(member, EncodingConcreteOverrides)`;
///   the deps from their builder; the payload `json!({})`; the declared
///   members are the input variation.
/// Act: `admit_encoding_selection(&deps, params, payload)` for each member,
///   imported from the crate root.
/// Assert: for each member, the result equals
///   `Ok(AdmitEncodingSelectionSuccessReturn { selection: member })` by
///   `assert_eq!`, each compared apart.
/// Boundary: the crate root's public call `admit_encoding_selection` and the
///   route from the crate barrel through the admission to the returned
///   selection.
/// Mocked: none; the official builders construct the inputs, and no outer-edge
///   collaborator lies on the route.
#[test]
fn admission_returns_each_declared_member_as_its_selection() {
    for member in ENCODING_CONCRETES {
        // Arrange
        let deps = build_admit_encoding_selection_deps(Default::default());
        let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
            concrete: Some(build_encoding_concrete(*member, EncodingConcreteOverrides)),
        });
        let payload = serde_json::json!({});

        // Act
        let result = admit_encoding_selection(&deps, params, payload);

        // Assert
        assert_eq!(
            result,
            Ok(AdmitEncodingSelectionSuccessReturn { selection: *member })
        );
    }
}

/// Contract: own entry `supplied_setting_refused_through_the_public_surface`:
///   for each member of `ENCODING_CONCRETES`, `admit_encoding_selection`
///   through the crate's public surface with a document that supplies a
///   setting → `Err(AdmitEncodingSelectionErrorReturn::SettingSupplied {
///   concrete })` naming that member.
/// Arrange: for each member of `ENCODING_CONCRETES`, the params as in the
///   admitted block; the deps from their builder; the payload
///   `json!({ "limit": 1 })`; the declared members paired with a non-empty
///   document are the input variation.
/// Act: `admit_encoding_selection(&deps, params, payload)` for each member,
///   imported from the crate root.
/// Assert: for each member, the result equals
///   `Err(AdmitEncodingSelectionErrorReturn::SettingSupplied { concrete:
///   member })` by `assert_eq!`, each compared apart.
/// Boundary: the crate root's public call `admit_encoding_selection` and the
///   route from the crate barrel through the admission to the returned error.
/// Mocked: none; the official builders construct the inputs, and no outer-edge
///   collaborator lies on the route.
#[test]
fn admission_refuses_a_supplied_setting_for_each_declared_member() {
    for member in ENCODING_CONCRETES {
        // Arrange
        let deps = build_admit_encoding_selection_deps(Default::default());
        let params = build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
            concrete: Some(build_encoding_concrete(*member, EncodingConcreteOverrides)),
        });
        let payload = serde_json::json!({ "limit": 1 });

        // Act
        let result = admit_encoding_selection(&deps, params, payload);

        // Assert
        assert_eq!(
            result,
            Err(AdmitEncodingSelectionErrorReturn::SettingSupplied { concrete: *member })
        );
    }
}
