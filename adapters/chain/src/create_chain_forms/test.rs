#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_chain_forms;
use super::interface::{
    ChainFormsConcrete, ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsDeps,
    CreateChainFormsPayload, IChainFormsConsumer,
};
use super::mock::{CreateChainFormsParamsOverrides, build_create_chain_forms_params};
use crate::factory::provides::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ChainFormsIdentifier, IChainForms,
};

struct DeclarationProbe;

impl IChainFormsConsumer for DeclarationProbe {
    type Output = ChainFormsDeclaration;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        F::DECLARATION
    }
}

/// Contract: the EVM forms concrete, admitted for the EVM forms identifier,
///   reaches the consumer with its declaration.
/// Arrange: `build_create_chain_forms_params` with `concrete:
///   Some(ChainFormsConcrete::Evm)` and `identifier:
///   Some(ChainFormsIdentifier::EvmV1)`, and `CreateChainFormsDeps {
///   consumer: DeclarationProbe }`.
/// Act:     `create_chain_forms(&deps, params, CreateChainFormsPayload)`.
/// Assert:  `success.output.identifier` equals `ChainFormsIdentifier::EvmV1`,
///   `success.output.adapter_version` equals `1`, and
///   `success.output.interface_version` equals `CHAIN_FORMS_INTERFACE_VERSION`.
#[test]
fn create_chain_forms_hands_the_consumer_the_evm_forms_and_their_declaration() {
    // Arrange
    let deps = CreateChainFormsDeps {
        consumer: DeclarationProbe,
    };
    let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
        concrete: Some(ChainFormsConcrete::Evm),
        identifier: Some(ChainFormsIdentifier::EvmV1),
    });

    // Act
    let Ok(success) = create_chain_forms(&deps, params, CreateChainFormsPayload) else {
        panic!("the EVM forms concrete is admitted for the EVM forms identifier")
    };

    // Assert
    assert_eq!(success.output.identifier, ChainFormsIdentifier::EvmV1);
    assert_eq!(success.output.adapter_version, 1);
    assert_eq!(
        success.output.interface_version,
        CHAIN_FORMS_INTERFACE_VERSION
    );
}
