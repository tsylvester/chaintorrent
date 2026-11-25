#![cfg(feature = "mocks")]

use hash_to_scalar::{
    HASH_TO_SCALAR_CONCRETES, HASH_TO_SCALAR_INTERFACE_VERSION, HashToScalarConcrete,
    LookupHashToScalarDeclarationParamsOverrides, MockIHashToScalarAdapterFailureMode,
    build_hash_to_scalar_concrete, build_lookup_hash_to_scalar_declaration_deps,
    build_lookup_hash_to_scalar_declaration_params,
    build_lookup_hash_to_scalar_declaration_payload, lookup_hash_to_scalar_declaration,
};

/// Contract: entry `every_declared_member_declares_itself_through_the_public_surface`;
///   for each member of `HASH_TO_SCALAR_CONCRETES`, the lookup yields `Ok`
///   whose declaration has `interface_version` equal to
///   `HASH_TO_SCALAR_INTERFACE_VERSION`.
/// Arrange: for each member of `HASH_TO_SCALAR_CONCRETES`,
///   `build_lookup_hash_to_scalar_declaration_params` with the `concrete`
///   override `build_hash_to_scalar_concrete(*member, Default::default())`;
///   `build_lookup_hash_to_scalar_declaration_deps` and
///   `build_lookup_hash_to_scalar_declaration_payload` with no override; the
///   body is written once and names no concrete.
/// Act:     `lookup_hash_to_scalar_declaration(&deps, params, payload)` per
///   member.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.interface_version`
///   equals `HASH_TO_SCALAR_INTERFACE_VERSION`.
/// Boundary: the public call `lookup_hash_to_scalar_declaration`, whose route
///   is the module's function, then the member's interface-held declaration
///   constant through `crate::keccak256::provides`.
/// Mocked:  none.
#[test]
fn every_declared_member_declares_itself_through_the_public_surface() {
    // Arrange
    for member in HASH_TO_SCALAR_CONCRETES {
        let deps = build_lookup_hash_to_scalar_declaration_deps(Default::default());
        let params = build_lookup_hash_to_scalar_declaration_params(
            LookupHashToScalarDeclarationParamsOverrides {
                concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
            },
        );
        let payload = build_lookup_hash_to_scalar_declaration_payload(Default::default());

        // Act
        let result = lookup_hash_to_scalar_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert_eq!(
            success.declaration.interface_version,
            HASH_TO_SCALAR_INTERFACE_VERSION
        );
    }
}

/// Contract: entry `a_mock_member_declares_the_member_selected_through_the_public_surface`;
///   for `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`,
///   the lookup yields `Ok` whose declaration has `concrete` equal to that
///   member and `interface_version` equal to `HASH_TO_SCALAR_INTERFACE_VERSION`.
/// Arrange: `build_lookup_hash_to_scalar_declaration_params` with the
///   `concrete` override
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`,
///   a mode other than the `Succeeds` mode the mock declaration constant
///   names; `build_lookup_hash_to_scalar_declaration_deps` and
///   `build_lookup_hash_to_scalar_declaration_payload` with no override.
/// Act:     `lookup_hash_to_scalar_declaration(&deps, params, payload)`.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.concrete` equals
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`;
///   `success.declaration.interface_version` equals
///   `HASH_TO_SCALAR_INTERFACE_VERSION`.
/// Boundary: the public call `lookup_hash_to_scalar_declaration`, whose route
///   is the module's function, then the mock declaration constant through
///   `crate::factory::provides`.
/// Mocked:  the family's mock member, selected by the params, the official
///   collaborator the `mocks` feature provides.
#[test]
fn a_mock_member_declares_the_member_selected_through_the_public_surface() {
    // Arrange
    let deps = build_lookup_hash_to_scalar_declaration_deps(Default::default());
    let params = build_lookup_hash_to_scalar_declaration_params(
        LookupHashToScalarDeclarationParamsOverrides {
            concrete: Some(HashToScalarConcrete::Mock(
                MockIHashToScalarAdapterFailureMode::SamplingRefused,
            )),
        },
    );
    let payload = build_lookup_hash_to_scalar_declaration_payload(Default::default());

    // Act
    let result = lookup_hash_to_scalar_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(
        success.declaration.concrete,
        HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)
    );
    assert_eq!(
        success.declaration.interface_version,
        HASH_TO_SCALAR_INTERFACE_VERSION
    );
}
