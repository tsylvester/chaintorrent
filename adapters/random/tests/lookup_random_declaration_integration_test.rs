#![cfg(feature = "mocks")]

use random::{
    LookupRandomDeclarationDepsOverrides, LookupRandomDeclarationParamsOverrides,
    LookupRandomDeclarationPayloadOverrides, MockIRandomSourceAdapterFailureMode,
    RANDOM_SOURCE_INTERFACE_VERSION, RANDOM_SOURCE_KINDS, RandomSourceKind,
    build_lookup_random_declaration_deps, build_lookup_random_declaration_params,
    build_lookup_random_declaration_payload, lookup_random_declaration,
};

/// Contract: own entry `the_operating_system_source_is_looked_up_to_the_declaration_it_owns` —
///   `lookup_random_declaration` is called over `RandomSourceKind::OperatingSystem`
///   by a caller outside the crate → `Ok` whose declaration's `source` is
///   `RandomSourceKind::OperatingSystem`, whose `adapter_version` is the
///   literal 1, and whose `interface_version` is `RANDOM_SOURCE_INTERFACE_VERSION`.
/// Boundary: the public `lookup_random_declaration`; the route is
///   `lookup_random_declaration`, with no outer-edge collaborator.
/// Mocked:   nothing.
/// Arrange:  `build_lookup_random_declaration_params` with the identifier
///   override set to `RandomSourceKind::OperatingSystem`, a member whose
///   declaration names a source that differs from any `Mock` kind, so a lookup
///   that returns the mock member's declaration fails;
///   `build_lookup_random_declaration_deps(Default::default())` and
///   `build_lookup_random_declaration_payload(Default::default())` as its deps
///   and payload.
/// Act:      `lookup_random_declaration` over the identifier.
/// Assert:   the `Ok` arm's declaration is read through the irrefutable
///   pattern; its `source` equals `RandomSourceKind::OperatingSystem`, its
///   `adapter_version` equals the literal 1, and its `interface_version`
///   equals `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn lookup_random_declaration_returns_the_operating_system_source_its_own_declaration_for_an_outside_caller()
 {
    // Arrange
    let deps = build_lookup_random_declaration_deps(LookupRandomDeclarationDepsOverrides);
    let params = build_lookup_random_declaration_params(LookupRandomDeclarationParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = build_lookup_random_declaration_payload(LookupRandomDeclarationPayloadOverrides);

    // Act
    let result = lookup_random_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(
        success.declaration.source,
        RandomSourceKind::OperatingSystem
    );
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(
        success.declaration.interface_version,
        RANDOM_SOURCE_INTERFACE_VERSION
    );
}
// lookup_random_declaration_returns_the_operating_system_source_its_own_declaration_for_an_outside_caller

/// Contract: own entry `the_mock_source_is_looked_up_to_a_declaration_naming_the_identifier_it_was_given` —
///   `lookup_random_declaration` is called by a caller outside the crate under
///   the `mocks` feature over
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   and over the first identifier of `RANDOM_SOURCE_KINDS` → `Ok` whose
///   declaration's `source` is the mock identifier with its failure mode and
///   which differs from the declaration the declared member returns.
/// Boundary: the public `lookup_random_declaration`; the route is
///   `lookup_random_declaration`, with no outer-edge collaborator.
/// Mocked:   nothing, because the mock member is the family's own mock
///   concrete.
/// Arrange:  `build_lookup_random_declaration_params` with the identifier
///   override set to the mock identifier carrying `FillBytesRefused`, a mode
///   that differs from the mode the mock concrete's constant holds, beside the
///   first identifier of `RANDOM_SOURCE_KINDS`, so a branch that returns the
///   constant's own `source`, or the declared member's declaration, for the
///   mock identifier fails.
/// Act:      `lookup_random_declaration` over each identifier.
/// Assert:   each `Ok` arm's declaration is read through the irrefutable
///   pattern; the mock declaration's `source` equals
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   and the mock declaration is not equal to the declaration the declared
///   member's result holds.
#[test]
fn lookup_random_declaration_returns_the_mock_source_a_declaration_naming_its_identifier_for_an_outside_caller()
 {
    // Arrange
    let deps = build_lookup_random_declaration_deps(LookupRandomDeclarationDepsOverrides);
    let mock_identifier =
        RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused);
    let mock_params =
        build_lookup_random_declaration_params(LookupRandomDeclarationParamsOverrides {
            identifier: Some(mock_identifier),
        });
    let declared_params =
        build_lookup_random_declaration_params(LookupRandomDeclarationParamsOverrides {
            identifier: Some(RANDOM_SOURCE_KINDS[0]),
        });

    // Act
    let mock_result = lookup_random_declaration(
        &deps,
        mock_params,
        build_lookup_random_declaration_payload(LookupRandomDeclarationPayloadOverrides),
    );
    let declared_result = lookup_random_declaration(
        &deps,
        declared_params,
        build_lookup_random_declaration_payload(LookupRandomDeclarationPayloadOverrides),
    );

    // Assert
    let Ok(mock_success) = mock_result;
    let Ok(declared_success) = declared_result;
    assert_eq!(mock_success.declaration.source, mock_identifier);
    assert_ne!(mock_success.declaration, declared_success.declaration);
}
// lookup_random_declaration_returns_the_mock_source_a_declaration_naming_its_identifier_for_an_outside_caller
