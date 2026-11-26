#![cfg(feature = "mocks")]

use kem::{
    AdmitKemCompositionDepsOverrides, AdmitKemCompositionParamsOverrides,
    AdmitKemCompositionSuccessReturn, AdmitKemSelectionParamsOverrides, IDENTITY_SCOPES,
    KEM_CONCRETES, admit_kem_composition, admit_kem_selection, build_admit_kem_composition_deps,
    build_admit_kem_composition_params, build_admit_kem_composition_payload,
    build_admit_kem_selection_deps, build_admit_kem_selection_params, lookup_kem_declaration,
};
use serde_json::Value;

/// Contract: carried entry
///   `every_member_of_the_declared_set_has_a_declaration_that_names_it`;
///   `admit_kem_composition` called with each member of `KEM_CONCRETES` and
///   each member of `IDENTITY_SCOPES` over the real `lookup_kem_declaration` →
///   `Ok(AdmitKemCompositionSuccessReturn { concrete: member, scope })` for
///   each pair, the declaration compared being that member's own.
/// Arrange:  the deps from `build_admit_kem_composition_deps` overriding
///   `lookup_kem_declaration` with the real `lookup_kem_declaration`, the
///   params from `build_admit_kem_composition_params` with `concrete` each
///   member of `KEM_CONCRETES` and `scope` each member of `IDENTITY_SCOPES` in
///   turn, the body written once and run over the declared set and each scope.
/// Act:      `admit_kem_composition` once per pair.
/// Assert:   the result is `Ok`, the property the entry proves for every pair
///   of the declared sets, and its `concrete` and `scope` equal the params'
///   the loop built, which shows no other member or scope was substituted.
/// Boundary: the public call `admit_kem_composition` through the crate root
///   and the `lookup_kem_declaration` it calls.
/// Mocked:   nothing; no outer-edge collaborator is on the route.
#[test]
fn every_member_of_the_declared_set_has_a_declaration_that_names_it() {
    for member in KEM_CONCRETES {
        for scope in IDENTITY_SCOPES {
            // Arrange
            let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
                lookup_kem_declaration: Some(lookup_kem_declaration),
            });
            let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
                concrete: Some(*member),
                scope: Some(*scope),
            });
            let payload = build_admit_kem_composition_payload(Default::default());

            // Act
            let result = admit_kem_composition(&deps, params, payload);

            // Assert
            let success =
                result.expect("a member of the declared set is admitted under each scope");
            assert_eq!(success.concrete, *member);
            assert_eq!(success.scope, *scope);
        }
    }
}
// every_member_of_the_declared_set_has_a_declaration_that_names_it

/// Contract: own entry
///   `a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set`;
///   `admit_kem_selection` called with each member of `KEM_CONCRETES` and
///   `Value::Null`, then `admit_kem_composition` called with the selection it
///   returned and each member of `IDENTITY_SCOPES` →
///   `Ok(AdmitKemCompositionSuccessReturn { concrete: member, scope })` for
///   each pair.
/// Arrange:  the selection params from `build_admit_kem_selection_params` with
///   `concrete` each member of `KEM_CONCRETES` and the payload `Value::Null`,
///   then the composition deps from `build_admit_kem_composition_deps`
///   overriding `lookup_kem_declaration` with the real `lookup_kem_declaration`
///   and the composition params with `concrete` the selection
///   `admit_kem_selection` returned and `scope` each member of
///   `IDENTITY_SCOPES`, the body written once and run over the declared set
///   and each scope.
/// Act:      `admit_kem_selection`, then `admit_kem_composition`, once per
///   pair.
/// Assert:   the composition result is `Ok`, the property the entry proves
///   for every pair of the declared sets, and its `concrete` is the selection
///   `admit_kem_selection` returned and its `scope` the scope the loop built.
/// Boundary: the public calls `admit_kem_selection` and `admit_kem_composition`
///   through the crate root and the `lookup_kem_declaration` the second calls.
/// Mocked:   nothing; no outer-edge collaborator is on the route.
#[test]
fn a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set() {
    for member in KEM_CONCRETES {
        // Arrange
        let selection_deps = build_admit_kem_selection_deps(Default::default());
        let selection_params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
            concrete: Some(*member),
        });

        // Act
        let selection_result = admit_kem_selection(&selection_deps, selection_params, Value::Null);
        let selection = selection_result.expect("a member of the declared set is admitted");

        for scope in IDENTITY_SCOPES {
            // Arrange
            let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
                lookup_kem_declaration: Some(lookup_kem_declaration),
            });
            let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
                concrete: Some(selection.selection),
                scope: Some(*scope),
            });
            let payload = build_admit_kem_composition_payload(Default::default());

            // Act
            let result = admit_kem_composition(&deps, params, payload);

            // Assert
            assert_eq!(
                result,
                Ok(AdmitKemCompositionSuccessReturn {
                    concrete: selection.selection,
                    scope: *scope,
                })
            );
        }
    }
}
// a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set

// Public-surface enumeration: the own entry
//   `a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set`,
//   proven by
//   `a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set`;
//   the carried entry
//   `every_member_of_the_declared_set_has_a_declaration_that_names_it`,
//   proven by `every_member_of_the_declared_set_has_a_declaration_that_names_it`.
