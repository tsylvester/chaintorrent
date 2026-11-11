#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    ChainFormsConcrete, ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams,
    ConsumeChainFormsPayload, CreateChainFormsCaseDeps, CreateChainFormsCaseErrorReturn,
    CreateChainFormsCaseParams, CreateChainFormsCasePayload, CreateChainFormsCaseReturn,
    CreateChainFormsCaseSuccessReturn, CreateChainFormsDeps, CreateChainFormsParams,
    CreateChainFormsPayload, CreateChainFormsReturn, CreateChainFormsSuccessReturn,
    IChainFormsConsumer, ICreateChainFormsCaseConsumer,
};
use crate::evm_forms::provides::EvmForms;
use crate::factory::provides::{
    ChainFormsIdentifier, IChainForms, MockIChainForms, MockIChainFormsOverrides,
    build_mock_i_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};

#[derive(Clone)]
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

pub fn build_create_chain_forms_deps<'a, C: IChainFormsConsumer, E: IEncoderAdapter>(
    consumer: C,
    encoder: &'a E,
    _overrides: CreateChainFormsDepsOverrides,
) -> CreateChainFormsDeps<'a, C, E> {
    CreateChainFormsDeps { consumer, encoder }
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
pub struct ConsumeCaseParamsOverrides;

pub fn build_consume_case_params(_overrides: ConsumeCaseParamsOverrides) -> ConsumeCaseParams {
    ConsumeCaseParams
}

#[derive(Default)]
pub struct ConsumeCasePayloadOverrides;

pub fn build_consume_case_payload<'a, E: IEncoderAdapter>(
    encoder: &'a E,
    _overrides: ConsumeCasePayloadOverrides,
) -> ConsumeCasePayload<'a, E> {
    ConsumeCasePayload { encoder }
}

#[derive(Default)]
pub struct CreateChainFormsCaseDepsOverrides;

pub fn build_create_chain_forms_case_deps<K: ICreateChainFormsCaseConsumer>(
    consumer: K,
    _overrides: CreateChainFormsCaseDepsOverrides,
) -> CreateChainFormsCaseDeps<K> {
    CreateChainFormsCaseDeps { consumer }
}

#[derive(Default)]
pub struct CreateChainFormsCaseParamsOverrides {
    pub encoding: Option<CreateEncodingParams>,
}

pub fn build_create_chain_forms_case_params(
    overrides: CreateChainFormsCaseParamsOverrides,
) -> CreateChainFormsCaseParams {
    CreateChainFormsCaseParams {
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
    }
}

#[derive(Default)]
pub struct CreateChainFormsCasePayloadOverrides;

pub fn build_create_chain_forms_case_payload(
    _overrides: CreateChainFormsCasePayloadOverrides,
) -> CreateChainFormsCasePayload {
    CreateChainFormsCasePayload
}

#[derive(Default)]
pub struct CreateChainFormsCaseSuccessReturnOverrides;

pub fn build_create_chain_forms_case_success_return<O>(
    output: O,
    _overrides: CreateChainFormsCaseSuccessReturnOverrides,
) -> CreateChainFormsCaseSuccessReturn<O> {
    CreateChainFormsCaseSuccessReturn { output }
}

struct CaseAtEncoding<'a, K: ICreateChainFormsCaseConsumer> {
    consumer: &'a K,
}

impl<'a, K: ICreateChainFormsCaseConsumer> IEncodingConsumer for CaseAtEncoding<'a, K> {
    type Output = K::Output;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        self.consumer.consume_case(
            build_consume_case_params(Default::default()),
            build_consume_case_payload(&payload.adapter, Default::default()),
        )
    }
}

pub fn create_chain_forms_case<K: ICreateChainFormsCaseConsumer>(
    deps: &CreateChainFormsCaseDeps<K>,
    params: CreateChainFormsCaseParams,
    _payload: CreateChainFormsCasePayload,
) -> CreateChainFormsCaseReturn<K::Output> {
    match create_encoding(
        &build_create_encoding_deps(CaseAtEncoding {
            consumer: &deps.consumer,
        }),
        params.encoding,
        build_create_encoding_payload(Default::default()),
    ) {
        Err(error) => Err(CreateChainFormsCaseErrorReturn::Encoding(error)),
        Ok(success) => Ok(build_create_chain_forms_case_success_return(
            success.output,
            Default::default(),
        )),
    }
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

pub fn mock_create_chain_forms<C: IChainFormsConsumer, E: IEncoderAdapter>(
    deps: &CreateChainFormsDeps<'_, C, E>,
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
