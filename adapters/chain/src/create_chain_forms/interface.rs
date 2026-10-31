use crate::factory::provides::{ChainFormsIdentifier, IChainForms};
use core::convert::Infallible;
use encoding::IEncoderAdapter;
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

pub struct CreateChainFormsDeps<'a, C, E: IEncoderAdapter> {
    pub consumer: C,
    pub encoder: &'a E,
}

#[derive(Clone, Copy)]
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
    UnsupportedEncodingIdentifier,
    Evm(Infallible),
}

pub type CreateChainFormsReturn<O> =
    Result<CreateChainFormsSuccessReturn<O>, CreateChainFormsErrorReturn>;

pub type CreateChainFormsFn<C, E> =
    fn(
        &CreateChainFormsDeps<'_, C, E>,
        CreateChainFormsParams,
        CreateChainFormsPayload,
    ) -> CreateChainFormsReturn<<C as IChainFormsConsumer>::Output>;

#[cfg(any(test, feature = "mocks"))]
pub trait ICreateChainFormsCaseConsumer {
    type Output;

    fn consume_case<E: IEncoderAdapter>(
        &self,
        params: ConsumeCaseParams,
        payload: ConsumeCasePayload<'_, E>,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeCaseParams;

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeCasePayload<'a, E: IEncoderAdapter> {
    pub encoder: &'a E,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateChainFormsCaseDeps<K: ICreateChainFormsCaseConsumer> {
    pub consumer: K,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateChainFormsCaseParams {
    pub encoding: CreateEncodingParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateChainFormsCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct CreateChainFormsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum CreateChainFormsCaseErrorReturn {
    Encoding(CreateEncodingErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type CreateChainFormsCaseReturn<O> =
    Result<CreateChainFormsCaseSuccessReturn<O>, CreateChainFormsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type CreateChainFormsCaseFn<K> =
    fn(
        &CreateChainFormsCaseDeps<K>,
        CreateChainFormsCaseParams,
        CreateChainFormsCasePayload,
    ) -> CreateChainFormsCaseReturn<<K as ICreateChainFormsCaseConsumer>::Output>;
