#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_hash_to_scalar;
use super::interface::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, HASH_TO_SCALAR_INTERFACE_VERSION,
    HashToScalarConcrete, HashToScalarDeclaration, HashToScalarIdentifier,
};
use super::mock::{CreateHashToScalarParamsOverrides, build_create_hash_to_scalar_params};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingConsumer, PairingConcrete,
    build_create_pairing_params, create_pairing,
};

struct DeclarationProbe;

impl IPairingConsumer for DeclarationProbe {
    type Output = HashToScalarDeclaration;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        _payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let Ok(success) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                concrete: Some(HashToScalarConcrete::Keccak256),
                identifier: Some(HashToScalarIdentifier::Keccak256V1),
            }),
            CreateHashToScalarPayload,
        ) else {
            panic!("the Keccak-256 concrete is admitted for its identifier");
        };
        success.declaration
    }
}

/// Contract: admitted — the named concrete's declared identifier is
///   `params.identifier`, so the Keccak-256 concrete is constructed and
///   returned with its `DECLARATION` for the scalar type of the resolved
///   pairing, and the consumer's output carries that declaration back.
/// Arrange: `build_create_pairing_params` with `concrete:
///   Some(PairingConcrete::Bn254Arkworks)` and `CreatePairingDeps` holding a
///   `DeclarationProbe` consumer, which runs `create_hash_to_scalar` with
///   `concrete: Some(HashToScalarConcrete::Keccak256)` and `identifier:
///   Some(HashToScalarIdentifier::Keccak256V1)` over the pairing's scalar type.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `success.output.identifier` is `HashToScalarIdentifier::Keccak256V1`,
///   `success.output.adapter_version` is `1`, and
///   `success.output.interface_version` is `HASH_TO_SCALAR_INTERFACE_VERSION`.
#[test]
fn create_hash_to_scalar_returns_the_keccak256_concrete_and_its_declaration_for_a_pairing_scalar() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: DeclarationProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(matches!(
        success.output.identifier,
        HashToScalarIdentifier::Keccak256V1
    ));
    assert_eq!(success.output.adapter_version, 1);
    assert_eq!(
        success.output.interface_version,
        HASH_TO_SCALAR_INTERFACE_VERSION
    );
}
