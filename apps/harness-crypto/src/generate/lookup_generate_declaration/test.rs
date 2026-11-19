#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::EncodingIdentifier;

use super::super::factory::provides::{
    GENERATE_INTERFACE_VERSION, GenerateConcreteKind, VectorKind,
};
use super::lookup_generate_declaration;
use super::mock::*;

/// Contract: the identifier `GenerateConcreteKind::Solidity` → `Ok` holding
///   the Solidity member's declaration.
/// Arrange: the identifier `Solidity`, whose member declares the name,
///   encoding identifier, and identifier that differ from the mock member's,
///   so a branch that returned the mock member's declaration fails on the
///   identifier, the name, and the encoding identifier, and a branch that
///   returned `build_generate_declaration`'s defaults, which declare no
///   kind, fails on the first kind.
/// Act:    `lookup_generate_declaration`.
/// Assert: the returned declaration's `identifier` equals
///   `GenerateConcreteKind::Solidity`, its `name` equals `"Solidity"`, its
///   `encoding_identifier` equals `EncodingIdentifier::EthereumAbiV1`, its
///   `contract_suite` equals `"evm"`, its first kind's `kind` equals
///   `VectorKind::G1Addition`, and its `interface_version` equals
///   `GENERATE_INTERFACE_VERSION`, each by `assert_eq!`.
#[test]
fn the_solidity_member_returns_the_declaration_its_member_owns() {
    // Arrange
    let deps = build_lookup_generate_declaration_deps(Default::default());
    let params =
        build_lookup_generate_declaration_params(LookupGenerateDeclarationParamsOverrides {
            identifier: Some(GenerateConcreteKind::Solidity),
        });
    let payload = build_lookup_generate_declaration_payload(Default::default());

    // Act
    let result = lookup_generate_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(
        success.declaration.identifier,
        GenerateConcreteKind::Solidity
    );
    assert_eq!(success.declaration.name, "Solidity");
    assert_eq!(
        success.declaration.encoding_identifier,
        EncodingIdentifier::EthereumAbiV1
    );
    assert_eq!(success.declaration.contract_suite, "evm");
    assert_eq!(success.declaration.kinds[0].kind, VectorKind::G1Addition);
    assert_eq!(
        success.declaration.interface_version,
        GENERATE_INTERFACE_VERSION
    );
}
// the_solidity_member_returns_the_declaration_its_member_owns

/// Contract: the identifier `GenerateConcreteKind::Mock` → `Ok` holding the
///   mock member's declaration.
/// Arrange: the identifier `Mock`, whose member declares the name, encoding
///   identifier, and identifier that differ from the Solidity member's, so a
///   branch that returned the Solidity member's declaration fails.
/// Act:    `lookup_generate_declaration`.
/// Assert: the returned declaration's `identifier` equals
///   `GenerateConcreteKind::Mock`, its `name` equals `"Mock"`, its
///   `encoding_identifier` equals `EncodingIdentifier::Mock`, its
///   `contract_suite` equals `"evm"`, its first kind's `kind` equals
///   `VectorKind::G1Addition`, and its `interface_version` equals
///   `GENERATE_INTERFACE_VERSION`, each by `assert_eq!`.
#[test]
fn the_mock_member_returns_the_declaration_its_member_owns() {
    // Arrange
    let deps = build_lookup_generate_declaration_deps(Default::default());
    let params =
        build_lookup_generate_declaration_params(LookupGenerateDeclarationParamsOverrides {
            identifier: Some(GenerateConcreteKind::Mock),
        });
    let payload = build_lookup_generate_declaration_payload(Default::default());

    // Act
    let result = lookup_generate_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.declaration.identifier, GenerateConcreteKind::Mock);
    assert_eq!(success.declaration.name, "Mock");
    assert_eq!(
        success.declaration.encoding_identifier,
        EncodingIdentifier::Mock
    );
    assert_eq!(success.declaration.contract_suite, "evm");
    assert_eq!(success.declaration.kinds[0].kind, VectorKind::G1Addition);
    assert_eq!(
        success.declaration.interface_version,
        GENERATE_INTERFACE_VERSION
    );
}
// the_mock_member_returns_the_declaration_its_member_owns
