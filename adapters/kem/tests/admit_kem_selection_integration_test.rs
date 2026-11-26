#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use kem::{
    AdmitKemSelectionErrorReturn, AdmitKemSelectionParamsOverrides, AdmitKemSelectionSuccessReturn,
    KEM_CONCRETES, LookupKemDeclarationParamsOverrides, admit_kem_selection,
    build_admit_kem_selection_deps, build_admit_kem_selection_params,
    build_lookup_kem_declaration_deps, build_lookup_kem_declaration_params,
    build_lookup_kem_declaration_payload, lookup_kem_declaration,
};
use serde_json::{Map, Value, json};

/// Contract: entry
///   `every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads`;
///   each member of `KEM_CONCRETES` with a document that supplies no setting →
///   `Ok` holding a selection equal to the member, and `lookup_kem_declaration`
///   called with that selection → `Ok` holding a declaration whose `concrete`
///   equals the member.
/// Boundary: the public calls `admit_kem_selection` and `lookup_kem_declaration`
///   through the crate root.
/// Mocked:   nothing; no outer-edge collaborator is on the route.
/// Arrange:  the params from `build_admit_kem_selection_params` with `concrete`
///   each member of `KEM_CONCRETES` in turn and the payload `Value::Null` and
///   `Value::Object(Map::new())` in turn, the body written once and run over
///   the declared set and each document.
/// Act:      `admit_kem_selection`, then `lookup_kem_declaration` with
///   `concrete` the selection it returned.
/// Assert:   the admission result equals
///   `Ok(AdmitKemSelectionSuccessReturn { selection: member })` and the
///   declaration's `concrete` equals the member.
#[test]
fn every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads() {
    for member in KEM_CONCRETES {
        for document in [Value::Null, Value::Object(Map::new())] {
            // Arrange
            let deps = build_admit_kem_selection_deps(Default::default());
            let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                concrete: Some(*member),
            });

            // Act
            let result = admit_kem_selection(&deps, params, document);

            // Assert
            assert_eq!(
                result,
                Ok(AdmitKemSelectionSuccessReturn { selection: *member })
            );
            let admitted = result.expect("the admission is Ok");

            // Act
            let looked_up = lookup_kem_declaration(
                &build_lookup_kem_declaration_deps(Default::default()),
                build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                    concrete: Some(admitted.selection),
                }),
                build_lookup_kem_declaration_payload(Default::default()),
            );

            // Assert
            let Ok(success) = looked_up;
            assert_eq!(success.declaration.concrete, *member);
        }
    }
}
// every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads

/// Contract: entry
///   `a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set`;
///   each member of `KEM_CONCRETES` with a document that supplies a setting →
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete })` naming
///   that member.
/// Boundary: the public call `admit_kem_selection` through the crate root.
/// Mocked:   nothing; no outer-edge collaborator is on the route.
/// Arrange:  the params as the entry above for each member of `KEM_CONCRETES`
///   and the payload `json!({ "bound": 1 })`, `Value::Array(Vec::new())`,
///   `Value::Bool(true)`, `Value::from(7_u64)`, and
///   `Value::String(String::from("text"))` in turn, the body written once and
///   run over the declared set and each document.
/// Act:      `admit_kem_selection`.
/// Assert:   the whole result equals
///   `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete: member })`.
#[test]
fn a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set() {
    for member in KEM_CONCRETES {
        for document in [
            json!({ "bound": 1 }),
            Value::Array(Vec::new()),
            Value::Bool(true),
            Value::from(7_u64),
            Value::String(String::from("text")),
        ] {
            // Arrange
            let deps = build_admit_kem_selection_deps(Default::default());
            let params = build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                concrete: Some(*member),
            });

            // Act
            let result = admit_kem_selection(&deps, params, document);

            // Assert
            assert_eq!(
                result,
                Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete: *member })
            );
        }
    }
}
// a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set

// Public-surface enumeration: the own entries
//   `every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads`,
//   proven by `every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads`,
//   and `a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set`,
//   proven by `a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set`.
