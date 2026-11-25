use super::lookup_encoding_declaration;
use super::mock::{
    LookupEncodingDeclarationDepsOverrides, LookupEncodingDeclarationParamsOverrides,
    LookupEncodingDeclarationPayloadOverrides, build_lookup_encoding_declaration_deps,
    build_lookup_encoding_declaration_params, build_lookup_encoding_declaration_payload,
};
use crate::factory::provides::{
    EncodingConcrete, EncodingConcreteOverrides, EncodingIdentifier, build_encoding_concrete,
};

/// Contract: `lookup_encoding_declaration`, the `Abi` member: `params.concrete` is
///   `EncodingConcrete::Abi` → `Ok(LookupEncodingDeclarationSuccessReturn { declaration })`
///   with the ABI member's typed identity, its versioned encoding, and its adapter and
///   interface versions.
/// Arrange: the deps and payload from their builders; the params from
///   `build_lookup_encoding_declaration_params` overriding `concrete` with
///   `build_encoding_concrete(EncodingConcrete::Abi, EncodingConcreteOverrides::default())`;
///   the expected values written as independent literals, concrete
///   `EncodingConcrete::Abi`, identifier `EncodingIdentifier::EthereumAbiV1`, adapter
///   version `1`, and interface version `1`; the expected `concrete` literal differs from
///   the `concrete` the mock member declares, so a lookup that returns the mock
///   declaration for the real member fails.
/// Act: `lookup_encoding_declaration(&deps, params, payload)`.
/// Assert: `result.ok().map(|success| success.declaration.concrete)` equals
///   `Some(EncodingConcrete::Abi)`, and the declaration's `identifier` equals
///   `EncodingIdentifier::EthereumAbiV1`, its `adapter_version` equals `1`, and its
///   `interface_version` equals `1`, each by `assert_eq!`, each compared apart.
#[test]
fn lookup_returns_the_abi_member_declaration() {
    // Arrange
    let deps = build_lookup_encoding_declaration_deps(LookupEncodingDeclarationDepsOverrides);
    let params =
        build_lookup_encoding_declaration_params(LookupEncodingDeclarationParamsOverrides {
            concrete: Some(build_encoding_concrete(
                EncodingConcrete::Abi,
                EncodingConcreteOverrides::default(),
            )),
        });
    let payload =
        build_lookup_encoding_declaration_payload(LookupEncodingDeclarationPayloadOverrides);

    // Act
    let result = lookup_encoding_declaration(&deps, params, payload);

    // Assert
    let success = result.ok();
    assert_eq!(
        success.as_ref().map(|success| success.declaration.concrete),
        Some(EncodingConcrete::Abi)
    );
    assert_eq!(
        success
            .as_ref()
            .map(|success| success.declaration.identifier),
        Some(EncodingIdentifier::EthereumAbiV1)
    );
    assert_eq!(
        success
            .as_ref()
            .map(|success| success.declaration.adapter_version),
        Some(1)
    );
    assert_eq!(
        success
            .as_ref()
            .map(|success| success.declaration.interface_version),
        Some(1)
    );
}

/// Contract: `lookup_encoding_declaration`, the `Mock` member: `params.concrete` is
///   `EncodingConcrete::Mock` → `Ok(LookupEncodingDeclarationSuccessReturn { declaration })`
///   with the mock member's typed identity.
/// Arrange: the deps and payload from their builders; the params from
///   `build_lookup_encoding_declaration_params` overriding `concrete` with
///   `EncodingConcrete::Mock`; the expected values written as independent literals,
///   concrete `EncodingConcrete::Mock`, identifier `EncodingIdentifier::EthereumAbiV1`,
///   adapter version `1`, and interface version `1`; the expected `concrete` literal
///   differs from the `concrete` the real member declares, so a lookup that returns the
///   real declaration for the mock member fails.
/// Act: `lookup_encoding_declaration(&deps, params, payload)`.
/// Assert: `result.ok().map(|success| success.declaration.concrete)` equals
///   `Some(EncodingConcrete::Mock)`, and the declaration's `identifier` equals
///   `EncodingIdentifier::EthereumAbiV1`, its `adapter_version` equals `1`, and its
///   `interface_version` equals `1`, each by `assert_eq!`, each compared apart.
#[test]
fn lookup_returns_the_mock_member_declaration() {
    // Arrange
    let deps = build_lookup_encoding_declaration_deps(LookupEncodingDeclarationDepsOverrides);
    let params =
        build_lookup_encoding_declaration_params(LookupEncodingDeclarationParamsOverrides {
            concrete: Some(EncodingConcrete::Mock),
        });
    let payload =
        build_lookup_encoding_declaration_payload(LookupEncodingDeclarationPayloadOverrides);

    // Act
    let result = lookup_encoding_declaration(&deps, params, payload);

    // Assert
    let success = result.ok();
    assert_eq!(
        success.as_ref().map(|success| success.declaration.concrete),
        Some(EncodingConcrete::Mock)
    );
    assert_eq!(
        success
            .as_ref()
            .map(|success| success.declaration.identifier),
        Some(EncodingIdentifier::EthereumAbiV1)
    );
    assert_eq!(
        success
            .as_ref()
            .map(|success| success.declaration.adapter_version),
        Some(1)
    );
    assert_eq!(
        success
            .as_ref()
            .map(|success| success.declaration.interface_version),
        Some(1)
    );
}
