use super::interface::{
    UnwrapPieceGroupKeyDeps, UnwrapPieceGroupKeyParams, UnwrapPieceGroupKeyPayload,
    UnwrapPieceGroupKeyReturn, UnwrapPieceGroupKeySuccessReturn,
};
use crate::sidecar::wrap::provides::{
    PieceGroupKey, PieceGroupKeyConstructorParamsOverrides, WrappedPieceGroupKey,
    build_piece_group_key,
};
use domain::DerivationContext;
use encoding::IEncoderAdapter;
use kdf::IKeyDerivationAdapter;
use kem::EncapsulatedValue;
use serde_json::{Map, Value};

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyDepsOverrides;

pub fn build_unwrap_piece_group_key_deps<'a, E: IEncoderAdapter>(
    kdf: &'a dyn IKeyDerivationAdapter,
    encoder: &'a E,
    _overrides: UnwrapPieceGroupKeyDepsOverrides,
) -> UnwrapPieceGroupKeyDeps<'a, E> {
    UnwrapPieceGroupKeyDeps { kdf, encoder }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyParamsOverrides;

pub fn build_unwrap_piece_group_key_params(
    _overrides: UnwrapPieceGroupKeyParamsOverrides,
) -> UnwrapPieceGroupKeyParams {
    UnwrapPieceGroupKeyParams
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyParamsCorruptions;

pub fn invalidate_unwrap_piece_group_key_params(
    _corruptions: UnwrapPieceGroupKeyParamsCorruptions,
) -> Value {
    Value::Object(Map::new())
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyPayloadOverrides;

pub fn build_unwrap_piece_group_key_payload<'a>(
    encapsulated: &'a EncapsulatedValue,
    context: &'a DerivationContext,
    wrapped: &'a WrappedPieceGroupKey,
    _overrides: UnwrapPieceGroupKeyPayloadOverrides,
) -> UnwrapPieceGroupKeyPayload<'a> {
    UnwrapPieceGroupKeyPayload {
        encapsulated,
        context,
        wrapped,
    }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeySuccessReturnOverrides {
    pub piece_group_key: Option<PieceGroupKey>,
}

pub fn build_unwrap_piece_group_key_success_return(
    overrides: UnwrapPieceGroupKeySuccessReturnOverrides,
) -> UnwrapPieceGroupKeySuccessReturn {
    UnwrapPieceGroupKeySuccessReturn {
        piece_group_key: overrides.piece_group_key.unwrap_or_else(|| {
            build_piece_group_key(PieceGroupKeyConstructorParamsOverrides::default())
        }),
    }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeySuccessReturnCorruptions {
    pub piece_group_key: Option<Value>,
}

pub fn invalidate_unwrap_piece_group_key_success_return(
    corruptions: UnwrapPieceGroupKeySuccessReturnCorruptions,
) -> Value {
    let success = build_unwrap_piece_group_key_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "piece_group_key".to_string(),
        byte_array(success.piece_group_key.expose()),
    );
    if let Some(piece_group_key) = corruptions.piece_group_key {
        fields.insert("piece_group_key".to_string(), piece_group_key);
    }
    Value::Object(fields)
}

pub fn mock_unwrap_piece_group_key<E: IEncoderAdapter>(
    _deps: &UnwrapPieceGroupKeyDeps<'_, E>,
    _params: UnwrapPieceGroupKeyParams,
    _payload: UnwrapPieceGroupKeyPayload<'_>,
) -> UnwrapPieceGroupKeyReturn {
    Ok(build_unwrap_piece_group_key_success_return(
        Default::default(),
    ))
}
