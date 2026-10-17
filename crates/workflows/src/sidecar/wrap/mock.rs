#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    PIECE_GROUP_KEY_LENGTH, PieceGroupKey, PieceGroupKeyConstructorParams, WrapPieceGroupKeyDeps,
    WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload, WrapPieceGroupKeyReturn,
    WrapPieceGroupKeySuccessReturn, WrappedPieceGroupKey, WrappedPieceGroupKeyConstructorParams,
};
use domain::{DerivationContext, Secret, build_secret};
use encoding::IEncoderAdapter;
use kdf::IKeyDerivationAdapter;
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
