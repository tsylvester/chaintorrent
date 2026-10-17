#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    ChainFormsConcrete, ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsDeps,
    CreateChainFormsParams, CreateChainFormsPayload, CreateChainFormsReturn,
    CreateChainFormsSuccessReturn, IChainFormsConsumer,
};
use crate::evm_forms::provides::EvmForms;
use crate::factory::provides::{
    ChainFormsIdentifier, IChainForms, MockIChainForms, MockIChainFormsOverrides,
    build_mock_i_chain_forms,
};

pub struct MockIChainFormsConsumer;

impl IChainFormsConsumer for MockIChainFormsConsumer {
    type Output = ();

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
    }
}

#[derive(Default)]
pub struct ConsumeChainFormsParamsOverrides;

pub fn build_consume_chain_forms_params(
    _overrides: ConsumeChainFormsParamsOverrides,
) -> ConsumeChainFormsParams {
    ConsumeChainFormsParams
}

#[derive(Default)]
pub struct CreateChainFormsPayloadOverrides;

pub fn build_create_chain_forms_payload(
    _overrides: CreateChainFormsPayloadOverrides,
) -> CreateChainFormsPayload {
    CreateChainFormsPayload
}

#[derive(Default)]
pub struct ConsumeChainFormsPayloadOverrides;

pub fn build_consume_chain_forms_payload<F: IChainForms>(
    forms: F,
    _overrides: ConsumeChainFormsPayloadOverrides,
) -> ConsumeChainFormsPayload<F> {
    ConsumeChainFormsPayload { forms }
}

#[derive(Default)]
pub struct CreateChainFormsDepsOverrides;

pub fn build_create_chain_forms_deps<C: IChainFormsConsumer>(
    consumer: C,
    _overrides: CreateChainFormsDepsOverrides,
) -> CreateChainFormsDeps<C> {
    CreateChainFormsDeps { consumer }
}

#[derive(Default)]
pub struct CreateChainFormsSuccessReturnOverrides;

pub fn build_create_chain_forms_success_return<O>(
    output: O,
    _overrides: CreateChainFormsSuccessReturnOverrides,
) -> CreateChainFormsSuccessReturn<O> {
    CreateChainFormsSuccessReturn { output }
}

#[derive(Default)]
pub struct CreateChainFormsParamsOverrides {
    pub concrete: Option<ChainFormsConcrete>,
    pub identifier: Option<ChainFormsIdentifier>,
}

pub fn build_create_chain_forms_params(
    overrides: CreateChainFormsParamsOverrides,
) -> CreateChainFormsParams {
    let concrete = overrides.concrete.unwrap_or(ChainFormsConcrete::Mock);
    let identifier = overrides.identifier.unwrap_or(match concrete {
        ChainFormsConcrete::Evm => EvmForms::DECLARATION.identifier,
        ChainFormsConcrete::Mock => MockIChainForms::DECLARATION.identifier,
    });
    CreateChainFormsParams {
        concrete,
        identifier,
    }
}

pub fn mock_create_chain_forms<C: IChainFormsConsumer>(
    deps: &CreateChainFormsDeps<C>,
    _params: CreateChainFormsParams,
    _payload: CreateChainFormsPayload,
) -> CreateChainFormsReturn<C::Output> {
    let forms = build_mock_i_chain_forms(MockIChainFormsOverrides);
    let output = deps.consumer.consume_chain_forms(
        build_consume_chain_forms_params(Default::default()),
        build_consume_chain_forms_payload(forms, Default::default()),
    );
    Ok(build_create_chain_forms_success_return(
        output,
        Default::default(),
    ))
}
