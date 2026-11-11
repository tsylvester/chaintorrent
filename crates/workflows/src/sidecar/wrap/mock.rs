#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    IWrapPieceGroupKeyCaseConsumer, PIECE_GROUP_KEY_LENGTH, PieceGroupKey,
    PieceGroupKeyConstructorParams, RunWrapPieceGroupKeyCaseFn, WrapPieceGroupKeyCaseDeps,
    WrapPieceGroupKeyCaseErrorReturn, WrapPieceGroupKeyCaseParams, WrapPieceGroupKeyCasePayload,
    WrapPieceGroupKeyCaseReturn, WrapPieceGroupKeyCaseSuccessReturn, WrapPieceGroupKeyDeps,
    WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload, WrapPieceGroupKeyReturn,
    WrapPieceGroupKeySuccessReturn, WrappedPieceGroupKey, WrappedPieceGroupKeyConstructorParams,
};
use crate::sidecar::derivation_context::provides::DerivationContext;
use domain::{Secret, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use kdf::{
    CreateKeyDerivationParams, IKeyDerivationAdapter, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload, create_key_derivation,
};
use kem::EncapsulatedValue;
use serde_json::{Map, Value};

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

#[derive(Default)]
pub struct WrapPieceGroupKeyDepsOverrides;

pub fn build_wrap_piece_group_key_deps<'a, E: IEncoderAdapter>(
    kdf: &'a dyn IKeyDerivationAdapter,
    encoder: &'a E,
    _overrides: WrapPieceGroupKeyDepsOverrides,
) -> WrapPieceGroupKeyDeps<'a, E> {
    WrapPieceGroupKeyDeps { kdf, encoder }
}

#[derive(Default)]
pub struct WrapPieceGroupKeyParamsOverrides;

pub fn build_wrap_piece_group_key_params(
    _overrides: WrapPieceGroupKeyParamsOverrides,
) -> WrapPieceGroupKeyParams {
    WrapPieceGroupKeyParams
}

#[derive(Default)]
pub struct WrapPieceGroupKeyPayloadOverrides;

pub fn build_wrap_piece_group_key_payload<'a>(
    encapsulated: &'a EncapsulatedValue,
    context: &'a DerivationContext,
    piece_group_key: &'a PieceGroupKey,
    _overrides: WrapPieceGroupKeyPayloadOverrides,
) -> WrapPieceGroupKeyPayload<'a> {
    WrapPieceGroupKeyPayload {
        encapsulated,
        context,
        piece_group_key,
    }
}

#[derive(Default)]
pub struct PieceGroupKeyConstructorParamsOverrides {
    pub secret: Option<Secret<Vec<u8>>>,
}

pub fn build_piece_group_key_constructor_params(
    overrides: PieceGroupKeyConstructorParamsOverrides,
) -> PieceGroupKeyConstructorParams {
    PieceGroupKeyConstructorParams {
        secret: overrides
            .secret
            .unwrap_or_else(|| build_secret(vec![1u8; PIECE_GROUP_KEY_LENGTH], Default::default())),
    }
}

#[derive(Default)]
pub struct PieceGroupKeyConstructorParamsCorruptions {
    pub secret: Option<Value>,
}

pub fn invalidate_piece_group_key_constructor_params(
    corruptions: PieceGroupKeyConstructorParamsCorruptions,
) -> Value {
    let params = build_piece_group_key_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("secret".to_string(), byte_array(params.secret.expose()));
    if let Some(secret) = corruptions.secret {
        fields.insert("secret".to_string(), secret);
    }
    Value::Object(fields)
}

pub fn build_piece_group_key(overrides: PieceGroupKeyConstructorParamsOverrides) -> PieceGroupKey {
    PieceGroupKey::try_new(build_piece_group_key_constructor_params(overrides))
        .expect("built piece-group key constructor params are admitted")
}

#[derive(Default)]
pub struct WrappedPieceGroupKeyConstructorParamsOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_wrapped_piece_group_key_constructor_params(
    overrides: WrappedPieceGroupKeyConstructorParamsOverrides,
) -> WrappedPieceGroupKeyConstructorParams {
    WrappedPieceGroupKeyConstructorParams {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| vec![1u8; PIECE_GROUP_KEY_LENGTH]),
    }
}

#[derive(Default)]
pub struct WrappedPieceGroupKeyConstructorParamsCorruptions {
    pub bytes: Option<Value>,
}

pub fn invalidate_wrapped_piece_group_key_constructor_params(
    corruptions: WrappedPieceGroupKeyConstructorParamsCorruptions,
) -> Value {
    let params = build_wrapped_piece_group_key_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("bytes".to_string(), byte_array(&params.bytes));
    if let Some(bytes) = corruptions.bytes {
        fields.insert("bytes".to_string(), bytes);
    }
    Value::Object(fields)
}

pub fn build_wrapped_piece_group_key(
    overrides: WrappedPieceGroupKeyConstructorParamsOverrides,
) -> WrappedPieceGroupKey {
    WrappedPieceGroupKey::try_new(build_wrapped_piece_group_key_constructor_params(overrides))
        .expect("built wrapped piece-group key constructor params are admitted")
}

#[derive(Default)]
pub struct WrapPieceGroupKeySuccessReturnOverrides {
    pub wrapped: Option<WrappedPieceGroupKey>,
}

pub fn build_wrap_piece_group_key_success_return(
    overrides: WrapPieceGroupKeySuccessReturnOverrides,
) -> WrapPieceGroupKeySuccessReturn {
    WrapPieceGroupKeySuccessReturn {
        wrapped: overrides
            .wrapped
            .unwrap_or_else(|| build_wrapped_piece_group_key(Default::default())),
    }
}

#[derive(Default)]
pub struct WrapPieceGroupKeySuccessReturnCorruptions {
    pub wrapped: Option<Value>,
}

pub fn invalidate_wrap_piece_group_key_success_return(
    corruptions: WrapPieceGroupKeySuccessReturnCorruptions,
) -> Value {
    let success = build_wrap_piece_group_key_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "wrapped".to_string(),
        byte_array(success.wrapped.as_bytes()),
    );
    if let Some(wrapped) = corruptions.wrapped {
        fields.insert("wrapped".to_string(), wrapped);
    }
    Value::Object(fields)
}

pub fn mock_wrap_piece_group_key<E: IEncoderAdapter>(
    _deps: &WrapPieceGroupKeyDeps<'_, E>,
    _params: WrapPieceGroupKeyParams,
    _payload: WrapPieceGroupKeyPayload<'_>,
) -> WrapPieceGroupKeyReturn {
    Ok(build_wrap_piece_group_key_success_return(Default::default()))
}

#[derive(Default)]
pub struct WrapPieceGroupKeyCaseParamsOverrides {
    pub encoding: Option<CreateEncodingParams>,
    pub key_derivation: Option<CreateKeyDerivationParams>,
}

pub fn build_wrap_piece_group_key_case_params(
    overrides: WrapPieceGroupKeyCaseParamsOverrides,
) -> WrapPieceGroupKeyCaseParams {
    WrapPieceGroupKeyCaseParams {
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        key_derivation: overrides
            .key_derivation
            .unwrap_or_else(|| build_create_key_derivation_params(Default::default())),
    }
}

#[derive(Default)]
pub struct WrapPieceGroupKeyCaseDepsOverrides;

pub fn build_wrap_piece_group_key_case_deps<C: IWrapPieceGroupKeyCaseConsumer>(
    consumer: C,
    _overrides: WrapPieceGroupKeyCaseDepsOverrides,
) -> WrapPieceGroupKeyCaseDeps<C> {
    WrapPieceGroupKeyCaseDeps { consumer }
}

#[derive(Default)]
pub struct WrapPieceGroupKeyCasePayloadOverrides;

pub fn build_wrap_piece_group_key_case_payload<'a>(
    encapsulated: &'a EncapsulatedValue,
    context: &'a DerivationContext,
    _overrides: WrapPieceGroupKeyCasePayloadOverrides,
) -> WrapPieceGroupKeyCasePayload<'a> {
    WrapPieceGroupKeyCasePayload {
        encapsulated,
        context,
    }
}

#[derive(Default)]
pub struct WrapPieceGroupKeyCaseSuccessReturnOverrides;

pub fn build_wrap_piece_group_key_case_success_return<O>(
    output: O,
    _overrides: WrapPieceGroupKeyCaseSuccessReturnOverrides,
) -> WrapPieceGroupKeyCaseSuccessReturn<O> {
    WrapPieceGroupKeyCaseSuccessReturn { output }
}

struct CaseAtEncoding<'a, C: IWrapPieceGroupKeyCaseConsumer> {
    consumer: &'a C,
    key_derivation: CreateKeyDerivationParams,
    case_payload: &'a WrapPieceGroupKeyCasePayload<'a>,
}

impl<C: IWrapPieceGroupKeyCaseConsumer> IEncodingConsumer for CaseAtEncoding<'_, C> {
    type Output = WrapPieceGroupKeyCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let created = match create_key_derivation(
            &build_create_key_derivation_deps(Default::default()),
            self.key_derivation,
            build_create_key_derivation_payload(Default::default()),
        ) {
            Ok(created) => created,
            Err(error) => {
                return Err(WrapPieceGroupKeyCaseErrorReturn::CreateKeyDerivation(error));
            }
        };
        let output = self.consumer.consume_wrap_piece_group_key_case(
            &payload.adapter,
            &*created.adapter,
            self.case_payload,
        );
        Ok(WrapPieceGroupKeyCaseSuccessReturn { output })
    }
}

pub fn run_wrap_piece_group_key_case<C: IWrapPieceGroupKeyCaseConsumer>(
    deps: &WrapPieceGroupKeyCaseDeps<C>,
    params: WrapPieceGroupKeyCaseParams,
    payload: WrapPieceGroupKeyCasePayload<'_>,
) -> WrapPieceGroupKeyCaseReturn<C::Output> {
    let _: RunWrapPieceGroupKeyCaseFn<C> = run_wrap_piece_group_key_case;
    match create_encoding(
        &build_create_encoding_deps(CaseAtEncoding {
            consumer: &deps.consumer,
            key_derivation: params.key_derivation,
            case_payload: &payload,
        }),
        params.encoding,
        build_create_encoding_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(WrapPieceGroupKeyCaseErrorReturn::CreateEncoding(error)),
    }
}
