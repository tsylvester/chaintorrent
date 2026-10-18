use crate::factory::provides::{ChainFormsIdentifier, IChainForms};
use core::convert::Infallible;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ChainFormsConcrete {
    Evm,
    #[cfg(any(test, feature = "mocks"))]
    Mock,
}

pub const CHAIN_FORMS_CONCRETES: &[ChainFormsConcrete] = &[ChainFormsConcrete::Evm];

pub trait IChainFormsConsumer {
    type Output;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output;
}

pub struct ConsumeChainFormsParams;

pub struct ConsumeChainFormsPayload<F: IChainForms> {
    pub forms: F,
}

pub struct CreateChainFormsDeps<C> {
    pub consumer: C,
}

pub struct CreateChainFormsParams {
    pub concrete: ChainFormsConcrete,
    pub identifier: ChainFormsIdentifier,
}

pub struct CreateChainFormsPayload;

pub struct CreateChainFormsSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateChainFormsErrorReturn {
    UnsupportedChainFormsIdentifier,
    Evm(Infallible),
}

pub type CreateChainFormsReturn<O> =
    Result<CreateChainFormsSuccessReturn<O>, CreateChainFormsErrorReturn>;

pub type CreateChainFormsFn<C> = fn(
    &CreateChainFormsDeps<C>,
    CreateChainFormsParams,
    CreateChainFormsPayload,
) -> CreateChainFormsReturn<<C as IChainFormsConsumer>::Output>;
