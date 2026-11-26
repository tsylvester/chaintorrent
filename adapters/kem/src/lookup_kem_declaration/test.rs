#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::lookup_kem_declaration;
use super::mock::{
    LookupKemDeclarationParamsOverrides, build_lookup_kem_declaration_deps,
    build_lookup_kem_declaration_params, build_lookup_kem_declaration_payload,
};
use crate::factory::provides::{IdentityScope, KemConcrete, KemIdentifier};

/// Contract: `lookup_kem_declaration`, real member: `params.concrete` is
///   `KemConcrete::Bb1DepthOne` → `Ok` holding the declaration
///   `kem/bb1_depth_one` declares.
/// Arrange: the deps and payload from their builders and the params from
///   `build_lookup_kem_declaration_params` with `concrete`
///   `KemConcrete::Bb1DepthOne`, the variation separating it from the mock
///   member.
/// Act:     `lookup_kem_declaration` once.
/// Assert:  the declaration's `concrete` is `KemConcrete::Bb1DepthOne`,
///   `identifier` is `KemIdentifier::Bb1DepthOneV1`, `identity_scopes` equals
///   `[IdentityScope::Entitlement, IdentityScope::Asset]`, `identity_tag`
///   equals the bytes of `ChainTorrent-v1-kem-identity`, `adapter_version` is
///   `1`, and `interface_version` is `1`.
#[test]
fn returns_the_bb1_depth_one_declaration_for_the_bb1_depth_one_member() {
    // Arrange
    let deps = build_lookup_kem_declaration_deps(Default::default());
    let params = build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
    });
    let payload = build_lookup_kem_declaration_payload(Default::default());

    // Act
    let result = lookup_kem_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.declaration.concrete, KemConcrete::Bb1DepthOne);
    assert_eq!(success.declaration.identifier, KemIdentifier::Bb1DepthOneV1);
    assert_eq!(
        success.declaration.identity_scopes,
        [IdentityScope::Entitlement, IdentityScope::Asset].as_slice()
    );
    assert_eq!(
        success.declaration.identity_tag,
        b"ChainTorrent-v1-kem-identity".as_slice()
    );
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}
// returns_the_bb1_depth_one_declaration_for_the_bb1_depth_one_member

/// Contract: `lookup_kem_declaration`, mock member, under
///   `#[cfg(any(test, feature = "mocks"))]`: `params.concrete` is
///   `KemConcrete::Mock` → `Ok` holding the mock declaration unchanged.
/// Arrange: as
///   `returns_the_bb1_depth_one_declaration_for_the_bb1_depth_one_member`,
///   with `concrete` `KemConcrete::Mock`.
/// Act:     `lookup_kem_declaration` once.
/// Assert:  the declaration's `concrete` is `KemConcrete::Mock`, `identifier`
///   is `KemIdentifier::Bb1DepthOneV1`, `identity_scopes` equals
///   `[IdentityScope::Entitlement, IdentityScope::Asset]`, `identity_tag`
///   equals the bytes of `ChainTorrent-v1-kem-identity-mock`,
///   `adapter_version` is `1`, and `interface_version` is `1`, the tag
///   differing from the real member's so a branch that returned one
///   declaration for both members fails.
#[test]
fn returns_the_mock_declaration_for_the_mock_member() {
    // Arrange
    let deps = build_lookup_kem_declaration_deps(Default::default());
    let params = build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
        concrete: Some(KemConcrete::Mock),
    });
    let payload = build_lookup_kem_declaration_payload(Default::default());

    // Act
    let result = lookup_kem_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.declaration.concrete, KemConcrete::Mock);
    assert_eq!(success.declaration.identifier, KemIdentifier::Bb1DepthOneV1);
    assert_eq!(
        success.declaration.identity_scopes,
        [IdentityScope::Entitlement, IdentityScope::Asset].as_slice()
    );
    assert_eq!(
        success.declaration.identity_tag,
        b"ChainTorrent-v1-kem-identity-mock".as_slice()
    );
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}
// returns_the_mock_declaration_for_the_mock_member
