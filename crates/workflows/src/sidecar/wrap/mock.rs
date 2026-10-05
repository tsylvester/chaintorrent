#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    PIECE_GROUP_KEY_LENGTH, PieceGroupKey, WrapPieceGroupKeyDeps, WrapPieceGroupKeyParams,
    WrapPieceGroupKeyPayload, WrapPieceGroupKeyReturn, WrapPieceGroupKeySuccessReturn,
    WrappedPieceGroupKey,
};
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::IEncoderAdapter;

#[derive(Default)]
pub struct PieceGroupKeyOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_piece_group_key(overrides: PieceGroupKeyOverrides) -> PieceGroupKey {
    let bytes = overrides
        .bytes
        .unwrap_or_else(|| vec![0x33; PIECE_GROUP_KEY_LENGTH]);
    let secret = build_secret(SecretConstructorParamsOverrides { value: Some(bytes) });
    let Ok(key) = PieceGroupKey::try_from_secret_bytes(secret) else {
        panic!("the default bytes are the piece-group key length")
    };
    key
}

#[derive(Default)]
pub struct WrappedPieceGroupKeyOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_wrapped_piece_group_key(
    overrides: WrappedPieceGroupKeyOverrides,
) -> WrappedPieceGroupKey {
    let bytes = overrides
        .bytes
        .unwrap_or_else(|| vec![0x5a; PIECE_GROUP_KEY_LENGTH]);
    let Ok(wrapped) = WrappedPieceGroupKey::try_from_bytes(bytes) else {
        panic!("the default bytes are the piece-group key length")
    };
    wrapped
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

pub fn mock_wrap_piece_group_key<E: IEncoderAdapter>(
    _deps: &WrapPieceGroupKeyDeps<'_, E>,
    _params: WrapPieceGroupKeyParams,
    _payload: WrapPieceGroupKeyPayload<'_>,
) -> WrapPieceGroupKeyReturn {
    Ok(build_wrap_piece_group_key_success_return(Default::default()))
}
