#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_encoding;
use super::interface::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload,
    ENCODING_INTERFACE_VERSION, EncodingConcrete, EncodingDeclaration, EncodingIdentifier,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
};
use super::mock::{CreateEncodingParamsOverrides, build_create_encoding_params};

struct DeclarationProbe;

impl IEncodingConsumer for DeclarationProbe {
    type Output = EncodingDeclaration;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        payload.declaration
    }
}

/// Contract: the ABI concrete, admitted for the Ethereum ABI identifier,
///   reaches the consumer with its declaration.
/// Arrange: `build_create_encoding_params` with `concrete:
///   Some(EncodingConcrete::Abi)` and `identifier:
///   Some(EncodingIdentifier::EthereumAbiV1)`, and `CreateEncodingDeps {
///   consumer: DeclarationProbe }`.
/// Act:     `create_encoding(&deps, params, CreateEncodingPayload)`.
/// Assert:  `success.output.identifier` matches
///   `EncodingIdentifier::EthereumAbiV1`, `success.output.adapter_version`
///   equals `1`, and `success.output.interface_version` equals
///   `ENCODING_INTERFACE_VERSION`.
#[test]
fn create_encoding_hands_the_consumer_the_abi_concrete_and_its_declaration() {
    // Arrange
    let deps = CreateEncodingDeps {
        consumer: DeclarationProbe,
    };
    let params = build_create_encoding_params(CreateEncodingParamsOverrides {
        concrete: Some(EncodingConcrete::Abi),
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
    });

    // Act
    let Ok(success) = create_encoding(&deps, params, CreateEncodingPayload) else {
        panic!("the ABI concrete is admitted for the Ethereum ABI identifier")
    };

    // Assert
    assert!(matches!(
        success.output.identifier,
        EncodingIdentifier::EthereumAbiV1
    ));
    assert_eq!(success.output.adapter_version, 1);
    assert_eq!(success.output.interface_version, ENCODING_INTERFACE_VERSION);
}
