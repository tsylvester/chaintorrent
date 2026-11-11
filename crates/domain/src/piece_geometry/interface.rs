pub const MINIMUM_PIECE_SIZE: u32 = 16384;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieceGeometry {
    pub(super) piece_size: u32,
    pub(super) piece_group_size: u32,
    pub(super) total_extent: u64,
}

pub struct PieceGeometryConstructorParams {
    pub piece_size: u32,
    pub piece_group_size: u32,
    pub total_extent: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceGeometryTryNewErrorReturn {
    PieceSizeNotPowerOfTwo {
        piece_size: u32,
    },
    PieceSizeBelowMinimum {
        piece_size: u32,
        minimum: u32,
    },
    ZeroPieceGroupSize,
    PieceGroupSizeNotMultipleOfPieceSize {
        piece_group_size: u32,
        piece_size: u32,
    },
    ZeroTotalExtent,
}

pub type PieceGeometryTryNewReturn = Result<PieceGeometry, PieceGeometryTryNewErrorReturn>;
