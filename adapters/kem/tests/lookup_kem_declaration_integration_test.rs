#![cfg(feature = "mocks")]

use kem::{
    IDENTITY_SCOPES, KEM_CONCRETES, KEM_IDENTIFIERS, LookupKemDeclarationParamsOverrides,
    build_lookup_kem_declaration_deps, build_lookup_kem_declaration_params,
    build_lookup_kem_declaration_payload, lookup_kem_declaration,
};

/// Contract: own entry
///   `every_member_of_the_declared_set_has_a_declaration_that_names_it`;
///   `lookup_kem_declaration` called with each member of `KEM_CONCRETES` →
///   `Ok` holding a declaration whose `concrete` equals the member, whose
///   `identifier` is a member of `KEM_IDENTIFIERS`, and whose
///   `identity_scopes` is non-empty with each scope a member of
///   `IDENTITY_SCOPES`.
/// Arrange:  the params from `build_lookup_kem_declaration_params` with
///   `concrete` each member of `KEM_CONCRETES` in turn, the body written once
///   and run over the declared set.
/// Act:      `lookup_kem_declaration` once per member.
/// Assert:   each result binds through the irrefutable `let Ok(success) = …;`;
///   the declaration's `concrete` equals the member, its `identifier` is
///   contained in `KEM_IDENTIFIERS`, and its `identity_scopes` is non-empty
///   with each scope contained in `IDENTITY_SCOPES`.
/// Boundary: the public call `lookup_kem_declaration` through the crate root
///   and the constant it reads.
/// Mocked:   nothing; no outer-edge collaborator is on the route.
#[test]
fn every_member_of_the_declared_set_has_a_declaration_that_names_it() {
    for member in KEM_CONCRETES {
        // Arrange
        let deps = build_lookup_kem_declaration_deps(Default::default());
        let params = build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
            concrete: Some(*member),
        });
        let payload = build_lookup_kem_declaration_payload(Default::default());

        // Act
        let result = lookup_kem_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert_eq!(success.declaration.concrete, *member);
        assert!(KEM_IDENTIFIERS.contains(&success.declaration.identifier));
        assert!(!success.declaration.identity_scopes.is_empty());
        assert!(
            success
                .declaration
                .identity_scopes
                .iter()
                .all(|scope| IDENTITY_SCOPES.contains(scope))
        );
    }
}
// every_member_of_the_declared_set_has_a_declaration_that_names_it

// Public-surface enumeration: the own entry
//   `every_member_of_the_declared_set_has_a_declaration_that_names_it`,
//   proven by `every_member_of_the_declared_set_has_a_declaration_that_names_it`.
