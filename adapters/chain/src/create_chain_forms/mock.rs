#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    ChainFormsConcrete, ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsDeps,
    CreateChainFormsParams, CreateChainFormsPayload, CreateChainFormsReturn,
    CreateChainFormsSuccessReturn, IChainFormsConsumer,
};
use crate::factory::provides::{ChainFormsIdentifier, IChainForms};

#[derive(Default)]
pub struct CreateChainFormsParamsOverrides {
    pub concrete: Option<ChainFormsConcrete>,
    pub identifier: Option<ChainFormsIdentifier>,
}

pub fn build_create_chain_forms_params(
    overrides: CreateChainFormsParamsOverrides,
) -> CreateChainFormsParams {
    CreateChainFormsParams {
        concrete: overrides.concrete.unwrap_or(ChainFormsConcrete::Evm),
        identifier: overrides.identifier.unwrap_or(ChainFormsIdentifier::EvmV1),
    }
}

#[derive(Default)]
pub struct CreateChainFormsSuccessReturnOverrides<O> {
    pub output: Option<O>,
}

pub fn build_create_chain_forms_success_return<O: Default>(
    overrides: CreateChainFormsSuccessReturnOverrides<O>,
) -> CreateChainFormsSuccessReturn<O> {
    CreateChainFormsSuccessReturn {
        output: overrides.output.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ConsumeChainFormsPayloadOverrides<F: IChainForms> {
    pub forms: Option<F>,
}

pub fn build_consume_chain_forms_payload<F: IChainForms + Default>(
    overrides: ConsumeChainFormsPayloadOverrides<F>,
) -> ConsumeChainFormsPayload<F> {
    ConsumeChainFormsPayload {
        forms: overrides.forms.unwrap_or_default(),
    }
}

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

pub fn mock_create_chain_forms<C: IChainFormsConsumer>(
    _deps: &CreateChainFormsDeps<C>,
    _params: CreateChainFormsParams,
    _payload: CreateChainFormsPayload,
) -> CreateChainFormsReturn<C::Output>
where
    C::Output: Default,
{
    Ok(build_create_chain_forms_success_return(Default::default()))
}
