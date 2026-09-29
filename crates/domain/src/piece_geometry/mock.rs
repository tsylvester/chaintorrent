#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{MINIMUM_PIECE_SIZE, PieceGeometry, PieceGeometryConstructorParams};

#[derive(Default)]
pub struct PieceGeometryConstructorParamsOverrides {
    pub piece_size: Option<u32>,
    pub piece_group_size: Option<u32>,
    pub total_extent: Option<u64>,
}

pub fn build_piece_geometry_constructor_params(
    overrides: PieceGeometryConstructorParamsOverrides,
) -> PieceGeometryConstructorParams {
    PieceGeometryConstructorParams {
        piece_size: overrides.piece_size.unwrap_or(MINIMUM_PIECE_SIZE),
        piece_group_size: overrides.piece_group_size.unwrap_or(16384),
        total_extent: overrides.total_extent.unwrap_or(1048576),
    }
}

pub fn build_piece_geometry(overrides: PieceGeometryConstructorParamsOverrides) -> PieceGeometry {
    PieceGeometry::try_new(build_piece_geometry_constructor_params(overrides))
        .expect("built piece geometry constructor params are admitted")
}
