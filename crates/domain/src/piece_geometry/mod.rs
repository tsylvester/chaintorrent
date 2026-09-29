mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    MINIMUM_PIECE_SIZE, PieceGeometry, PieceGeometryConstructorParams,
    PieceGeometryTryNewErrorReturn, PieceGeometryTryNewReturn,
};

impl PieceGeometry {
    pub fn try_new(params: PieceGeometryConstructorParams) -> PieceGeometryTryNewReturn {
        if !params.piece_size.is_power_of_two() {
            return Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo {
                piece_size: params.piece_size,
            });
        }
        if params.piece_size < MINIMUM_PIECE_SIZE {
            return Err(PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum {
                piece_size: params.piece_size,
                minimum: MINIMUM_PIECE_SIZE,
            });
        }
        if params.piece_group_size == 0 {
            return Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize);
        }
        if !params.piece_group_size.is_multiple_of(params.piece_size) {
            return Err(
                PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize {
                    piece_group_size: params.piece_group_size,
                    piece_size: params.piece_size,
                },
            );
        }
        if params.total_extent == 0 {
            return Err(PieceGeometryTryNewErrorReturn::ZeroTotalExtent);
        }
        Ok(PieceGeometry {
            piece_size: params.piece_size,
            piece_group_size: params.piece_group_size,
            total_extent: params.total_extent,
        })
    }

    pub fn piece_size(&self) -> u32 {
        self.piece_size
    }

    pub fn piece_group_size(&self) -> u32 {
        self.piece_group_size
    }

    pub fn total_extent(&self) -> u64 {
        self.total_extent
    }

    pub fn piece_count(&self) -> u64 {
        self.total_extent.div_ceil(u64::from(self.piece_size))
    }

    pub fn group_count(&self) -> u64 {
        self.total_extent.div_ceil(u64::from(self.piece_group_size))
    }
}
