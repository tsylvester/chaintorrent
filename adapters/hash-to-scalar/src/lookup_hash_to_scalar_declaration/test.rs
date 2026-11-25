use super::lookup_hash_to_scalar_declaration;

use super::mock::{
    LookupHashToScalarDeclarationParamsOverrides, build_lookup_hash_to_scalar_declaration_deps,
    build_lookup_hash_to_scalar_declaration_params,
    build_lookup_hash_to_scalar_declaration_payload,
};

use crate::factory::provides::{
    HASH_TO_SCALAR_INTERFACE_VERSION, HashToScalarConcrete, HashToScalarIdentifier,
    MockIHashToScalarAdapterFailureMode,
};

/// Contract: `lookup_hash_to_scalar_declaration`, real member: `params.concrete`
///   is `HashToScalarConcrete::Keccak256` → `Ok` whose declaration is
///   `Keccak256HashToScalar::DECLARATION`: `concrete` the `Keccak256` member,
///   `identifier` `HashToScalarIdentifier::Keccak256V1`, `adapter_version` `1`,
///   `interface_version` `HASH_TO_SCALAR_INTERFACE_VERSION`.
/// Collaborators: none; `Keccak256HashToScalar::DECLARATION` is read for real.
/// Arrange: `build_lookup_hash_to_scalar_declaration_deps` and
///   `build_lookup_hash_to_scalar_declaration_payload` with no override;
///   `build_lookup_hash_to_scalar_declaration_params` with the `concrete`
///   override `HashToScalarConcrete::Keccak256`, the member the default mock
///   member differs from.
/// Act:     `lookup_hash_to_scalar_declaration(&deps, params, payload)`.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.concrete` equals
///   `HashToScalarConcrete::Keccak256`; `success.declaration.identifier`
///   matches `HashToScalarIdentifier::Keccak256V1`;
///   `success.declaration.adapter_version` equals the literal `1`;
///   `success.declaration.interface_version` equals
///   `HASH_TO_SCALAR_INTERFACE_VERSION`.
#[test]
fn lookup_returns_the_keccak256_declaration_for_the_keccak256_member() {
    // Arrange
    let deps = build_lookup_hash_to_scalar_declaration_deps(Default::default());
    let params = build_lookup_hash_to_scalar_declaration_params(
        LookupHashToScalarDeclarationParamsOverrides {
            concrete: Some(HashToScalarConcrete::Keccak256),
        },
    );
    let payload = build_lookup_hash_to_scalar_declaration_payload(Default::default());

    // Act
    let result = lookup_hash_to_scalar_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(
        success.declaration.concrete,
        HashToScalarConcrete::Keccak256
    );
    assert!(matches!(
        success.declaration.identifier,
        HashToScalarIdentifier::Keccak256V1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(
        success.declaration.interface_version,
        HASH_TO_SCALAR_INTERFACE_VERSION
    );
}

/// Contract: `lookup_hash_to_scalar_declaration`, mock member: `params.concrete`
///   is `HashToScalarConcrete::Mock(mode)` → `Ok` whose declaration carries
///   `concrete` equal to the selected `Mock(mode)`, the real
///   `HashToScalarIdentifier::Keccak256V1`, `adapter_version` `1`, and
///   `interface_version` `HASH_TO_SCALAR_INTERFACE_VERSION`.
/// Collaborators: none; `MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION` is read
///   for real.
/// Arrange: `build_lookup_hash_to_scalar_declaration_deps` and
///   `build_lookup_hash_to_scalar_declaration_payload` with no override;
///   `build_lookup_hash_to_scalar_declaration_params` with the `concrete`
///   override
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`,
///   a mode other than the `Succeeds` mode
///   `MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION` names, so the substitution is
///   observable.
/// Act:     `lookup_hash_to_scalar_declaration(&deps, params, payload)`.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.concrete` equals
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`;
///   `success.declaration.identifier` matches
///   `HashToScalarIdentifier::Keccak256V1`;
///   `success.declaration.adapter_version` equals the literal `1`;
///   `success.declaration.interface_version` equals
///   `HASH_TO_SCALAR_INTERFACE_VERSION`.
#[test]
fn lookup_returns_the_mock_declaration_carrying_the_selected_mock_identifier() {
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
    assert!(matches!(
        success.declaration.identifier,
        HashToScalarIdentifier::Keccak256V1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(
        success.declaration.interface_version,
        HASH_TO_SCALAR_INTERFACE_VERSION
    );
}
