#![cfg(feature = "mocks")]

use chain::{
    ChainMemberIdentifier, DECLARED_CHAIN_MEMBERS, LookupChainDeclarationDepsOverrides,
    LookupChainDeclarationParamsOverrides, LookupChainDeclarationPayloadOverrides,
    build_lookup_chain_declaration_deps, build_lookup_chain_declaration_params,
    build_lookup_chain_declaration_payload, lookup_chain_declaration,
};

/// Contract: the own entry
///   `each_declared_member_is_looked_up_to_the_declaration_it_owns`:
///   `lookup_chain_declaration` called once for each identifier in
///   `DECLARED_CHAIN_MEMBERS` by a caller outside the crate → `Ok`
///   whose declaration's `identifier` is the identifier looked up and
///   whose `name` is a nonempty text.
/// Arrange: each identifier of `DECLARED_CHAIN_MEMBERS`; deps, params,
///   and payload built by `build_lookup_chain_declaration_deps`,
///   `build_lookup_chain_declaration_params`, and
///   `build_lookup_chain_declaration_payload` from the crate's public
///   surface under the `mocks` feature; each member of the declared set
///   is the variation.
/// Act:     `lookup_chain_declaration(&deps, params, payload)` per
///   member.
/// Assert:  for each member the result is `Ok`, the declaration's
///   `identifier` equals the identifier looked up, and its `name` is
///   not the empty text.
/// Boundary: the public `lookup_chain_declaration`; the real chain is
///   `lookup_chain_declaration` and the member's declaration constant
///   it matches.
/// Mocked:  nothing.
#[test]
fn lookup_chain_declaration_returns_each_declared_member_its_own_declaration_for_an_outside_caller()
{
    // Arrange
    let deps = build_lookup_chain_declaration_deps(LookupChainDeclarationDepsOverrides);

    for identifier in DECLARED_CHAIN_MEMBERS.iter().copied() {
        let params = build_lookup_chain_declaration_params(LookupChainDeclarationParamsOverrides {
            identifier: Some(identifier),
        });
        let payload =
            build_lookup_chain_declaration_payload(LookupChainDeclarationPayloadOverrides);

        // Act
        let result = lookup_chain_declaration(&deps, params, payload);

        // Assert
        assert!(result.is_ok());
        let declaration = result.ok().map(|success| success.declaration);
        assert_eq!(
            declaration.map(|declared| declared.identifier),
            Some(identifier)
        );
        assert!(declaration.is_some_and(|declared| !declared.name.is_empty()));
    }
}
// lookup_chain_declaration_returns_each_declared_member_its_own_declaration_for_an_outside_caller

/// Contract: the own entry
///   `the_mock_member_is_looked_up_to_the_declaration_the_mock_concrete_owns`:
///   `lookup_chain_declaration` over `ChainMemberIdentifier::Mock` and
///   over the first identifier of `DECLARED_CHAIN_MEMBERS` by a caller
///   outside the crate under the `mocks` feature → `Ok` whose
///   declaration's `identifier` is `ChainMemberIdentifier::Mock` and
///   which differs from the declaration the declared member returns.
/// Arrange: deps and payload built through their builders; params built
///   by `build_lookup_chain_declaration_params` over
///   `ChainMemberIdentifier::Mock` and over the first identifier of
///   `DECLARED_CHAIN_MEMBERS`; the pairing is the variation.
/// Act:     `lookup_chain_declaration(&deps, params, payload)` over
///   each.
/// Assert:  the mock result is `Ok`, its declaration's `identifier`
///   equals `ChainMemberIdentifier::Mock`, and the mock declaration is
///   not equal to the declaration the declared member's result holds.
/// Boundary: the public `lookup_chain_declaration`; the real chain is
///   `lookup_chain_declaration` and each member's declaration constant
///   it matches.
/// Mocked:  nothing, because the mock member is the family's own mock
///   concrete.
#[test]
fn lookup_chain_declaration_returns_the_mock_member_its_own_declaration_for_an_outside_caller() {
    // Arrange
    let deps = build_lookup_chain_declaration_deps(LookupChainDeclarationDepsOverrides);
    let mock_params =
        build_lookup_chain_declaration_params(LookupChainDeclarationParamsOverrides {
            identifier: Some(ChainMemberIdentifier::Mock),
        });
    let mock_payload =
        build_lookup_chain_declaration_payload(LookupChainDeclarationPayloadOverrides);
    let declared_params =
        build_lookup_chain_declaration_params(LookupChainDeclarationParamsOverrides {
            identifier: Some(DECLARED_CHAIN_MEMBERS[0]),
        });
    let declared_payload =
        build_lookup_chain_declaration_payload(LookupChainDeclarationPayloadOverrides);

    // Act
    let mock_result = lookup_chain_declaration(&deps, mock_params, mock_payload);
    let declared_result = lookup_chain_declaration(&deps, declared_params, declared_payload);

    // Assert
    assert!(mock_result.is_ok());
    assert!(declared_result.is_ok());
    assert_eq!(
        mock_result
            .ok()
            .map(|success| success.declaration.identifier),
        Some(ChainMemberIdentifier::Mock)
    );
    assert_ne!(
        mock_result.ok().map(|success| success.declaration),
        declared_result.ok().map(|success| success.declaration)
    );
}
// lookup_chain_declaration_returns_the_mock_member_its_own_declaration_for_an_outside_caller
