#![cfg(feature = "mocks")]

use hashing::{
    COMMITMENT_CONCRETES, COMMITMENT_INTERFACE_VERSION, CommitmentSchemeIdentifier,
    LookupHashingDeclarationParamsOverrides, build_lookup_hashing_declaration_deps,
    build_lookup_hashing_declaration_params, build_lookup_hashing_declaration_payload,
    lookup_hashing_declaration,
};

/// Contract: `lookup_hashing_declaration` is called through the crate's
///   public surface once for each member of `COMMITMENT_CONCRETES` → `Ok`
///   whose declaration's `identifier` is the real
///   `CommitmentSchemeIdentifier::Blake3BaoV1` and whose `interface_version`
///   is `COMMITMENT_INTERFACE_VERSION`.
/// Arrange: for each member of `COMMITMENT_CONCRETES`,
///   `build_lookup_hashing_declaration_params` with the `concrete` override
///   that member; the body is written once and run over the declared set.
/// Act:     `lookup_hashing_declaration(&deps, params, payload)` once per
///   member.
/// Assert:  for each member the success arm binds through the irrefutable
///   pattern `let Ok(success) = result;`, `declaration.identifier` equals
///   `CommitmentSchemeIdentifier::Blake3BaoV1`, and
///   `declaration.interface_version` equals `COMMITMENT_INTERFACE_VERSION`.
/// Boundary: the public call `lookup_hashing_declaration` from the crate
///   root, running the `match` arm of the selected member and that member's
///   interface-held declaration constant.
/// Mocked:  nothing.
#[test]
fn every_declared_member_declares_itself_through_the_public_surface() {
    for member in COMMITMENT_CONCRETES {
        // Arrange
        let deps = build_lookup_hashing_declaration_deps(Default::default());
        let params =
            build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                concrete: Some(*member),
            });
        let payload = build_lookup_hashing_declaration_payload(Default::default());

        // Act
        let result = lookup_hashing_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert_eq!(
            success.declaration.identifier,
            CommitmentSchemeIdentifier::Blake3BaoV1
        );
        assert_eq!(
            success.declaration.interface_version,
            COMMITMENT_INTERFACE_VERSION
        );
    }
}
