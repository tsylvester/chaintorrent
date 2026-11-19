#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::lookup_chain_declaration;
use super::mock::*;
use crate::base::provides::BASE_CHAIN_DECLARATION;
use crate::factory::provides::{
    ChainFormsIdentifier, ChainMemberIdentifier, MOCK_I_CHAIN_ADAPTER_DECLARATION,
};

/// Contract: the identifier `ChainMemberIdentifier::Base` → `Ok` holding
///   `BASE_CHAIN_DECLARATION`.
/// Arrange: the identifier `Base`, a member whose declaration differs
///   from the mock concrete's declaration, so a branch that returned the
///   mock concrete's declaration, or a declaration built from the
///   identifier, fails.
/// Act:     `lookup_chain_declaration`.
/// Assert:  the result equals `Ok` holding `BASE_CHAIN_DECLARATION`, the
///   returned declaration differs from `MOCK_I_CHAIN_ADAPTER_DECLARATION`,
///   its `identifier` equals `ChainMemberIdentifier::Base`, its `name`
///   equals `"Base"`, and its `interface_version` equals `1`, each stated
///   as an independent literal.
#[test]
fn the_base_member_returns_the_declaration_its_member_owns() {
    // Arrange
    let deps = build_lookup_chain_declaration_deps(Default::default());
    let params = build_lookup_chain_declaration_params(LookupChainDeclarationParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = build_lookup_chain_declaration_payload(Default::default());

    // Act
    let result = lookup_chain_declaration(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_lookup_chain_declaration_success_return(
            LookupChainDeclarationSuccessReturnOverrides {
                declaration: Some(BASE_CHAIN_DECLARATION),
            }
        ))
    );
    let declaration = result
        .expect("the lookup admits the declared member")
        .declaration;
    assert_ne!(declaration, MOCK_I_CHAIN_ADAPTER_DECLARATION);
    assert_eq!(declaration.identifier, ChainMemberIdentifier::Base);
    assert_eq!(declaration.name, "Base");
    assert_eq!(declaration.interface_version, 1);
}
// the_base_member_returns_the_declaration_its_member_owns

/// Contract: the identifier `ChainMemberIdentifier::Mock` → `Ok` holding
///   `MOCK_I_CHAIN_ADAPTER_DECLARATION`.
/// Arrange: the identifier `Mock`, a member whose declaration differs
///   from the Base member's declaration, so a branch that returned the
///   Base declaration fails.
/// Act:     `lookup_chain_declaration`.
/// Assert:  the result equals `Ok` holding
///   `MOCK_I_CHAIN_ADAPTER_DECLARATION`, the returned declaration differs
///   from `BASE_CHAIN_DECLARATION`, its `identifier` equals
///   `ChainMemberIdentifier::Mock`, and its `forms_identifier` equals
///   `ChainFormsIdentifier::EvmV1`.
#[test]
fn the_mock_member_returns_the_declaration_its_member_owns() {
    // Arrange
    let deps = build_lookup_chain_declaration_deps(Default::default());
    let params = build_lookup_chain_declaration_params(LookupChainDeclarationParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Mock),
    });
    let payload = build_lookup_chain_declaration_payload(Default::default());

    // Act
    let result = lookup_chain_declaration(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_lookup_chain_declaration_success_return(
            LookupChainDeclarationSuccessReturnOverrides {
                declaration: Some(MOCK_I_CHAIN_ADAPTER_DECLARATION),
            }
        ))
    );
    let declaration = result
        .expect("the lookup admits the mock member")
        .declaration;
    assert_ne!(declaration, BASE_CHAIN_DECLARATION);
    assert_eq!(declaration.identifier, ChainMemberIdentifier::Mock);
    assert_eq!(declaration.forms_identifier, ChainFormsIdentifier::EvmV1);
}
// the_mock_member_returns_the_declaration_its_member_owns
