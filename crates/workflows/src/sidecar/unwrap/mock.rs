#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    UnwrapPieceGroupKeyDeps, UnwrapPieceGroupKeyParams, UnwrapPieceGroupKeyPayload,
    UnwrapPieceGroupKeyReturn, UnwrapPieceGroupKeySuccessReturn,
};
use crate::sidecar::wrap::provides::{
    PIECE_GROUP_KEY_LENGTH, PieceGroupKey, PieceGroupKeyOverrides, build_piece_group_key,
};
use encoding::IEncoderAdapter;

#[derive(Default)]
pub struct UnwrapPieceGroupKeySuccessReturnOverrides {
    pub piece_group_key: Option<PieceGroupKey>,
}

pub fn build_unwrap_piece_group_key_success_return(
    overrides: UnwrapPieceGroupKeySuccessReturnOverrides,
) -> UnwrapPieceGroupKeySuccessReturn {
    UnwrapPieceGroupKeySuccessReturn {
        piece_group_key: overrides.piece_group_key.unwrap_or_else(|| {
            build_piece_group_key(PieceGroupKeyOverrides {
                bytes: Some(vec![0x33; PIECE_GROUP_KEY_LENGTH]),
            })
        }),
    }
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
