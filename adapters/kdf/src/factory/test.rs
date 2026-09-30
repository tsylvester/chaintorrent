#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_key_derivation;
use super::interface::{
    CreateKeyDerivationDeps, CreateKeyDerivationPayload, KDF_INTERFACE_VERSION, KdfConcrete,
    KdfIdentifier,
};
use super::mock::{CreateKeyDerivationParamsOverrides, build_create_key_derivation_params};

/// Contract: the BLAKE3 concrete, admitted for its identifier, is returned
///   with its declaration.
/// Arrange: `build_create_key_derivation_params` with `concrete:
///   Some(KdfConcrete::Blake3Keyed)` and `identifier:
///   Some(KdfIdentifier::Blake3KeyedV1)`.
/// Act:     `create_key_derivation(&CreateKeyDerivationDeps, params,
///   CreateKeyDerivationPayload)`.
/// Assert:  `success.declaration.identifier` is `KdfIdentifier::Blake3KeyedV1`,
///   `success.declaration.adapter_version` is `1`, and
///   `success.declaration.interface_version` is `KDF_INTERFACE_VERSION`.
#[test]
fn create_key_derivation_returns_the_blake3_keyed_concrete_and_its_declaration() {
    // Arrange
    let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        concrete: Some(KdfConcrete::Blake3Keyed),
        identifier: Some(KdfIdentifier::Blake3KeyedV1),
    });

    // Act
    let Ok(success) =
        create_key_derivation(&CreateKeyDerivationDeps, params, CreateKeyDerivationPayload)
    else {
        panic!("the BLAKE3 concrete is admitted for its identifier")
    };

    // Assert
    assert!(matches!(
        success.declaration.identifier,
        KdfIdentifier::Blake3KeyedV1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, KDF_INTERFACE_VERSION);
}
