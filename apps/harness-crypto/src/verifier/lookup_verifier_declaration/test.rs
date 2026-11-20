#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::factory::provides::{VERIFIER_CONCRETES, VerifierConcreteKind};
use super::interface::*;
use super::lookup_verifier_declaration;
use super::mock::*;

/// Contract: each identifier in `VERIFIER_CONCRETES` → `Ok` holding that
///   member's declaration.
/// Arrange: each identifier in `VERIFIER_CONCRETES`, so a branch that
///   returned one member's declaration for every identifier, or a
///   declaration built from the identifier, fails.
/// Act:    `lookup_verifier_declaration` over each identifier.
/// Assert: each result is `Ok`, the declaration's `identifier` equals the
///   identifier looked up, and its `name` is not the empty text.
#[test]
fn every_declared_member_returns_the_declaration_it_owns() {
    // Arrange
    let deps = build_lookup_verifier_declaration_deps(Default::default());

    for &identifier in VERIFIER_CONCRETES {
        let params =
            build_lookup_verifier_declaration_params(LookupVerifierDeclarationParamsOverrides {
                identifier: Some(identifier),
            });
        let payload = build_lookup_verifier_declaration_payload(Default::default());

        // Act
        let result: LookupVerifierDeclarationReturn =
            lookup_verifier_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert_eq!(success.declaration.identifier, identifier);
        assert!(!success.declaration.name.is_empty());
    }
}
// every_declared_member_returns_the_declaration_it_owns

/// Contract: the identifier `VerifierConcreteKind::Mock` → `Ok` holding the
///   mock member's own declaration, which no declared member holds.
/// Arrange: the identifier `Mock` and the first identifier of
///   `VERIFIER_CONCRETES`, so a branch that returned a declared member's
///   declaration for the mock identifier fails.
/// Act:    `lookup_verifier_declaration` over each.
/// Assert: the mock result is `Ok` whose declaration's `identifier` equals
///   `VerifierConcreteKind::Mock` and whose `name` equals `mock`, stated as
///   independent literals, and which is not equal to the declaration the
///   declared member's result holds.
#[test]
fn the_mock_member_returns_the_declaration_it_owns() {
    // Arrange
    let deps = build_lookup_verifier_declaration_deps(Default::default());
    let declared = VERIFIER_CONCRETES[0];

    // Act
    let mock_result: LookupVerifierDeclarationReturn = lookup_verifier_declaration(
        &deps,
        build_lookup_verifier_declaration_params(LookupVerifierDeclarationParamsOverrides {
            identifier: Some(VerifierConcreteKind::Mock),
        }),
        build_lookup_verifier_declaration_payload(Default::default()),
    );
    let declared_result: LookupVerifierDeclarationReturn = lookup_verifier_declaration(
        &deps,
        build_lookup_verifier_declaration_params(LookupVerifierDeclarationParamsOverrides {
            identifier: Some(declared),
        }),
        build_lookup_verifier_declaration_payload(Default::default()),
    );

    // Assert
    let Ok(mock_success) = mock_result;
    assert_eq!(
        mock_success.declaration.identifier,
        VerifierConcreteKind::Mock
    );
    assert_eq!(mock_success.declaration.name, "mock");
    let Ok(declared_success) = declared_result;
    assert_ne!(mock_success.declaration, declared_success.declaration);
}
// the_mock_member_returns_the_declaration_it_owns
