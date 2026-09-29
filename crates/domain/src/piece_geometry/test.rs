#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{PieceGeometry, PieceGeometryTryNewErrorReturn};
use super::mock::{
    PieceGeometryConstructorParamsOverrides, build_piece_geometry_constructor_params,
};

/// Contract: the minimum piece size, a group of one piece, and a one-byte
///   extent are admitted, read back unchanged, and imply one piece and one
///   group.
/// Arrange: params built with `piece_size: Some(16384)`,
///   `piece_group_size: Some(16384)`, and `total_extent: Some(1)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `geometry.piece_size()` equals `16384`,
///   `geometry.piece_group_size()` equals `16384`,
///   `geometry.total_extent()` equals `1`, `geometry.piece_count()` equals
///   `1`, and `geometry.group_count()` equals `1`.
#[test]
fn try_new_admits_the_smallest_geometry() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(16384),
        total_extent: Some(1),
    });

    // Act
    let Ok(geometry) = PieceGeometry::try_new(params) else {
        panic!("the smallest geometry is admitted")
    };

    // Assert
    assert_eq!(geometry.piece_size(), 16384);
    assert_eq!(geometry.piece_group_size(), 16384);
    assert_eq!(geometry.total_extent(), 1);
    assert_eq!(geometry.piece_count(), 1);
    assert_eq!(geometry.group_count(), 1);
}

/// Contract: a trailing partial piece and a trailing partial group are each
///   counted.
/// Arrange: params built with `piece_size: Some(16384)`,
///   `piece_group_size: Some(32768)`, and `total_extent: Some(49153)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `geometry.piece_count()` equals `4` and `geometry.group_count()`
///   equals `2`.
#[test]
fn piece_count_and_group_count_round_up_a_partial_piece_and_group() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(32768),
        total_extent: Some(49153),
    });

    // Act
    let Ok(geometry) = PieceGeometry::try_new(params) else {
        panic!("a geometry with a partial piece and group is admitted")
    };

    // Assert
    assert_eq!(geometry.piece_count(), 4);
    assert_eq!(geometry.group_count(), 2);
}

/// Contract: an extent that is a whole number of groups adds no partial
///   piece or group.
/// Arrange: params built with `piece_size: Some(16384)`,
///   `piece_group_size: Some(32768)`, and `total_extent: Some(65536)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `geometry.piece_count()` equals `4` and `geometry.group_count()`
///   equals `2`.
#[test]
fn piece_count_and_group_count_do_not_round_up_an_exact_extent() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(32768),
        total_extent: Some(65536),
    });

    // Act
    let Ok(geometry) = PieceGeometry::try_new(params) else {
        panic!("a whole-number-of-groups geometry is admitted")
    };

    // Assert
    assert_eq!(geometry.piece_count(), 4);
    assert_eq!(geometry.group_count(), 2);
}

/// Contract: the counts are computed without overflow across the full width
///   of the extent.
/// Arrange: params built with `piece_size: Some(2147483648)`,
///   `piece_group_size: Some(2147483648)`, and `total_extent: Some(u64::MAX)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `geometry.piece_count()` equals `8589934592` and
///   `geometry.group_count()` equals `8589934592`.
#[test]
fn piece_count_and_group_count_hold_at_the_largest_extent() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(2147483648),
        piece_group_size: Some(2147483648),
        total_extent: Some(u64::MAX),
    });

    // Act
    let Ok(geometry) = PieceGeometry::try_new(params) else {
        panic!("a geometry at the largest extent is admitted")
    };

    // Assert
    assert_eq!(geometry.piece_count(), 8589934592);
    assert_eq!(geometry.group_count(), 8589934592);
}

/// Contract: a piece size that is not a power of two is refused.
/// Arrange: params built with `piece_size: Some(24576)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals
///   `PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 24576 }`.
#[test]
fn try_new_rejects_a_piece_size_that_is_not_a_power_of_two() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(24576),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("a non-power-of-two piece size is refused")
    };

    // Assert
    assert_eq!(
        error,
        PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 24576 }
    );
}

/// Contract: a zero piece size is not a power of two and is refused.
/// Arrange: params built with `piece_size: Some(0)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals
///   `PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 0 }`.
#[test]
fn try_new_rejects_a_zero_piece_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(0),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("a zero piece size is refused")
    };

    // Assert
    assert_eq!(
        error,
        PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 0 }
    );
}

/// Contract: a power-of-two piece size below 16 KiB is refused.
/// Arrange: params built with `piece_size: Some(8192)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals
///   `PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum { piece_size: 8192, minimum: 16384 }`.
#[test]
fn try_new_rejects_a_piece_size_below_the_minimum() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(8192),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("a piece size below the minimum is refused")
    };

    // Assert
    assert_eq!(
        error,
        PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum {
            piece_size: 8192,
            minimum: 16384
        }
    );
}

/// Contract: a zero piece-group size is refused.
/// Arrange: params built with `piece_group_size: Some(0)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals `PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize`.
#[test]
fn try_new_rejects_a_zero_piece_group_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_group_size: Some(0),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("a zero piece-group size is refused")
    };

    // Assert
    assert_eq!(error, PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize);
}

/// Contract: a nonzero piece-group size that is not a whole multiple of the
///   piece size is refused.
/// Arrange: params built with `piece_size: Some(32768)` and
///   `piece_group_size: Some(49152)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals
///   `PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 49152, piece_size: 32768 }`.
#[test]
fn try_new_rejects_a_piece_group_size_that_is_not_a_multiple_of_the_piece_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(32768),
        piece_group_size: Some(49152),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("a piece-group size that is not a multiple of the piece size is refused")
    };

    // Assert
    assert_eq!(
        error,
        PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize {
            piece_group_size: 49152,
            piece_size: 32768
        }
    );
}

/// Contract: a group holds whole pieces, so a group smaller than one piece
///   is refused.
/// Arrange: params built with `piece_size: Some(32768)` and
///   `piece_group_size: Some(16384)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals
///   `PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 16384, piece_size: 32768 }`.
#[test]
fn try_new_rejects_a_piece_group_smaller_than_a_piece() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(32768),
        piece_group_size: Some(16384),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("a piece group smaller than a piece is refused")
    };

    // Assert
    assert_eq!(
        error,
        PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize {
            piece_group_size: 16384,
            piece_size: 32768
        }
    );
}

/// Contract: a zero total extent is refused, so every admitted geometry has
///   at least one piece and one group.
/// Arrange: params built with `total_extent: Some(0)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals `PieceGeometryTryNewErrorReturn::ZeroTotalExtent`.
#[test]
fn try_new_rejects_a_zero_total_extent() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        total_extent: Some(0),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("a zero total extent is refused")
    };

    // Assert
    assert_eq!(error, PieceGeometryTryNewErrorReturn::ZeroTotalExtent);
}

/// Contract: when several fields fail, the earliest check in the fixed
///   order is returned.
/// Arrange: params built with `piece_size: Some(0)` and
///   `total_extent: Some(0)`.
/// Act:     `PieceGeometry::try_new(params)`.
/// Assert:  `error` equals
///   `PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 0 }`.
#[test]
fn try_new_reports_the_piece_size_before_the_total_extent() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(0),
        total_extent: Some(0),
        ..Default::default()
    });

    // Act
    let Err(error) = PieceGeometry::try_new(params) else {
        panic!("failing params are refused")
    };

    // Assert
    assert_eq!(
        error,
        PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 0 }
    );
}
