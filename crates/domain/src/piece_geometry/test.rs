#![allow(clippy::expect_used)]

use super::provides::{
    PieceGeometry, PieceGeometryConstructorParamsOverrides, PieceGeometryTryNewErrorReturn,
    build_piece_geometry, build_piece_geometry_constructor_params,
};

/// Contract: `!params.piece_size.is_power_of_two()` →
///   `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `49152` and the piece-group size with `49152`; `49152` is a multiple of
///   the minimum piece size and above it but not a power of two, so a check of the
///   minimum or of a multiple of it fails, and the sizes agree, so the power-of-two
///   check is the only refusal the params can meet; the extent keeps its valid
///   default.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   PieceSizeNotPowerOfTwo { piece_size: 49152 })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_piece_size_that_is_not_a_power_of_two() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(49152),
        piece_group_size: Some(49152),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 49152 })
    );
}

/// Contract: `!params.piece_size.is_power_of_two()`, zero included →
///   `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `0`; the piece-group size and the extent keep their valid defaults, so a
///   remainder taken by the piece size before the piece size is checked fails.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   PieceSizeNotPowerOfTwo { piece_size: 0 })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_zero_piece_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(0),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 0 })
    );
}

/// Contract: the piece size is a power of two and
///   `params.piece_size < MINIMUM_PIECE_SIZE` →
///   `Err(PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum { piece_size,
///   minimum: MINIMUM_PIECE_SIZE })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `8192`, the power of two just below 16 KiB, and the piece-group size
///   with `8192`, so the sizes agree and the minimum is the only refusal the
///   params can meet; the extent keeps its valid default.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   PieceSizeBelowMinimum { piece_size: 8192, minimum: 16384 })` by `assert_eq!`,
///   the minimum being the 16 KiB the objective states.
#[test]
fn try_new_rejects_a_piece_size_below_the_minimum() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(8192),
        piece_group_size: Some(8192),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum {
            piece_size: 8192,
            minimum: 16384
        })
    );
}

/// Contract: the piece size passes and `params.piece_group_size == 0` →
///   `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece-group
///   size with `0`; the piece size and the extent keep their valid defaults, so a
///   remainder check alone, which reads zero as a multiple, fails.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   ZeroPieceGroupSize)` by `assert_eq!`.
#[test]
fn try_new_rejects_a_zero_piece_group_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_group_size: Some(0),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)
    );
}

/// Contract: the piece size passes, the piece-group size is nonzero, and
///   `params.piece_group_size % params.piece_size != 0` →
///   `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize
///   { piece_group_size, piece_size })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `16384` and the piece-group size with `24576`, a group larger than a
///   piece and one and a half pieces long; the extent keeps its valid default.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 24576,
///   piece_size: 16384 })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_piece_group_size_that_is_not_a_multiple_of_the_piece_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(24576),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize {
                piece_group_size: 24576,
                piece_size: 16384
            }
        )
    );
}

/// Contract: the piece size passes, the piece-group size is nonzero, and
///   `params.piece_group_size % params.piece_size != 0` →
///   `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize
///   { piece_group_size, piece_size })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `32768` and the piece-group size with `16384`, a group smaller than a
///   piece, so a remainder taken with its operands exchanged, which reads zero,
///   fails; the extent keeps its valid default.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 16384,
///   piece_size: 32768 })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_piece_group_smaller_than_a_piece() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(32768),
        piece_group_size: Some(16384),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize {
                piece_group_size: 16384,
                piece_size: 32768
            }
        )
    );
}

/// Contract: both sizes pass and `params.total_extent == 0` →
///   `Err(PieceGeometryTryNewErrorReturn::ZeroTotalExtent)`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the extent with
///   `0`; the sizes keep their valid defaults, so the extent is the only refusal
///   the params can meet.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   ZeroTotalExtent)` by `assert_eq!`.
#[test]
fn try_new_rejects_a_zero_total_extent() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        total_extent: Some(0),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(result, Err(PieceGeometryTryNewErrorReturn::ZeroTotalExtent));
}

/// Contract: the piece size's checks precede the piece-group size's → a piece
///   size that is not a power of two with a zero piece-group size yields
///   `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `49152` and the piece-group size with `0`; the extent keeps its valid
///   default, so a piece-group size check that runs ahead of the piece size
///   check fails.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   PieceSizeNotPowerOfTwo { piece_size: 49152 })` by `assert_eq!`.
#[test]
fn try_new_reports_the_piece_size_before_the_piece_group_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(49152),
        piece_group_size: Some(0),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 49152 })
    );
}

/// Contract: the piece-group size's checks precede the total extent's → a zero
///   piece-group size with a zero extent yields
///   `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece-group
///   size with `0` and the extent with `0`; the piece size keeps its valid
///   default, so an extent check that runs ahead of the piece-group size check
///   fails.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   ZeroPieceGroupSize)` by `assert_eq!`.
#[test]
fn try_new_reports_the_piece_group_size_before_the_total_extent() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_group_size: Some(0),
        total_extent: Some(0),
        ..Default::default()
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)
    );
}

/// Contract: the piece size's checks precede the total extent's → a piece size
///   that is not a power of two with a zero extent yields
///   `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `49152`, the piece-group size with `49152`, and the extent with `0`, so
///   the sizes agree and an extent check that runs ahead of the piece size check
///   fails.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result equals `Err(PieceGeometryTryNewErrorReturn::
///   PieceSizeNotPowerOfTwo { piece_size: 49152 })` by `assert_eq!`.
#[test]
fn try_new_reports_the_piece_size_before_the_total_extent() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(49152),
        piece_group_size: Some(49152),
        total_extent: Some(0),
    });

    // Act
    let result = PieceGeometry::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 49152 })
    );
}

/// Contract: every check passes →
///   `Ok(PieceGeometry { piece_size, piece_group_size, total_extent })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `16384`, the minimum, the piece-group size with `16384`, one piece, and
///   the extent with `1`, the smallest nonzero extent, so a minimum check that
///   refuses the minimum itself, a group check that requires more than one
///   piece, and an extent check that requires more than one byte each fail.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result is `Ok`; the geometry's `piece_size` field equals the
///   literal `16384`, its `piece_group_size` field equals the literal `16384`,
///   and its `total_extent` field equals the literal `1`.
#[test]
fn try_new_admits_the_smallest_geometry() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(16384),
        total_extent: Some(1),
    });

    // Act
    let geometry = PieceGeometry::try_new(params).expect("the smallest geometry is admitted");

    // Assert
    assert_eq!(geometry.piece_size, 16384);
    assert_eq!(geometry.piece_group_size, 16384);
    assert_eq!(geometry.total_extent, 1);
}

/// Contract: every check passes →
///   `Ok(PieceGeometry { piece_size, piece_group_size, total_extent })`.
/// Arrange: `build_piece_geometry_constructor_params` overriding the piece size
///   with `16384` and the piece-group size with `49152`, an odd multiple of the
///   piece size and not a power of two, so a group check that requires a power
///   of two fails; the extent keeps its valid default.
/// Act:     `PieceGeometry::try_new` on the built params.
/// Assert:  the result is `Ok`; the geometry's `piece_size` field equals the
///   literal `16384` and its `piece_group_size` field equals the literal `49152`.
#[test]
fn try_new_admits_a_piece_group_size_that_is_an_odd_multiple_of_the_piece_size() {
    // Arrange
    let params = build_piece_geometry_constructor_params(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(49152),
        ..Default::default()
    });

    // Act
    let geometry = PieceGeometry::try_new(params)
        .expect("a piece-group size that is an odd multiple of the piece size is admitted");

    // Assert
    assert_eq!(geometry.piece_size, 16384);
    assert_eq!(geometry.piece_group_size, 49152);
}

/// Contract: `PieceGeometry::piece_size(&self) -> u32` → the held piece size.
/// Arrange: `build_piece_geometry` overriding the piece size with `32768`, the
///   piece-group size with `131072`, and the extent with `0x0102_0304_0506_0708`,
///   three values that differ from one another, so an accessor that returns
///   another field fails.
/// Act:     `geometry.piece_size()`.
/// Assert:  the returned `u32` equals the literal `32768` by `assert_eq!`.
#[test]
fn piece_size_returns_the_held_piece_size() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(32768),
        piece_group_size: Some(131072),
        total_extent: Some(0x0102_0304_0506_0708),
    });

    // Act
    let piece_size = geometry.piece_size();

    // Assert
    assert_eq!(piece_size, 32768);
}

/// Contract: `PieceGeometry::piece_group_size(&self) -> u32` → the held
///   piece-group size.
/// Arrange: `build_piece_geometry` overriding the piece size with `32768`, the
///   piece-group size with `131072`, and the extent with `0x0102_0304_0506_0708`,
///   three values that differ from one another, so an accessor that returns
///   another field fails.
/// Act:     `geometry.piece_group_size()`.
/// Assert:  the returned `u32` equals the literal `131072` by `assert_eq!`.
#[test]
fn piece_group_size_returns_the_held_piece_group_size() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(32768),
        piece_group_size: Some(131072),
        total_extent: Some(0x0102_0304_0506_0708),
    });

    // Act
    let piece_group_size = geometry.piece_group_size();

    // Assert
    assert_eq!(piece_group_size, 131072);
}

/// Contract: `PieceGeometry::total_extent(&self) -> u64` → the held total
///   extent.
/// Arrange: `build_piece_geometry` overriding the piece size with `32768`, the
///   piece-group size with `131072`, and the extent with `0x0102_0304_0506_0708`,
///   a value whose eight bytes all differ, so a narrowed or byte-swapped return
///   fails.
/// Act:     `geometry.total_extent()`.
/// Assert:  the returned `u64` equals the literal `0x0102_0304_0506_0708` by
///   `assert_eq!`.
#[test]
fn total_extent_returns_the_held_total_extent() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(32768),
        piece_group_size: Some(131072),
        total_extent: Some(0x0102_0304_0506_0708),
    });

    // Act
    let total_extent = geometry.total_extent();

    // Assert
    assert_eq!(total_extent, 0x0102_0304_0506_0708);
}

/// Contract: `PieceGeometry::piece_count(&self) -> u64` →
///   `self.total_extent.div_ceil(u64::from(self.piece_size))`.
/// Arrange: `build_piece_geometry` overriding the piece size with `16384`, the
///   piece-group size with `65536`, and the extent with `16385`, one byte past a
///   whole piece, so a count that rounds down, and a count taken by the
///   piece-group size, fail.
/// Act:     `geometry.piece_count()`.
/// Assert:  the returned `u64` equals the literal `2`, the pieces `16385` bytes
///   need at `16384` bytes each, by `assert_eq!`.
#[test]
fn piece_count_rounds_up_a_partial_piece() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(65536),
        total_extent: Some(16385),
    });

    // Act
    let piece_count = geometry.piece_count();

    // Assert
    assert_eq!(piece_count, 2);
}

/// Contract: `PieceGeometry::group_count(&self) -> u64` →
///   `self.total_extent.div_ceil(u64::from(self.piece_group_size))`.
/// Arrange: `build_piece_geometry` overriding the piece size with `16384`, the
///   piece-group size with `65536`, and the extent with `65537`, one byte past a
///   whole group, so a count that rounds down, and a count taken by the piece
///   size, fail.
/// Act:     `geometry.group_count()`.
/// Assert:  the returned `u64` equals the literal `2`, the groups `65537` bytes
///   need at `65536` bytes each, by `assert_eq!`.
#[test]
fn group_count_rounds_up_a_partial_group() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(65536),
        total_extent: Some(65537),
    });

    // Act
    let group_count = geometry.group_count();

    // Assert
    assert_eq!(group_count, 2);
}

/// Contract: `PieceGeometry::piece_count(&self) -> u64` →
///   `self.total_extent.div_ceil(u64::from(self.piece_size))`.
/// Arrange: `build_piece_geometry` overriding the piece size with `16384`, the
///   piece-group size with `65536`, and the extent with `49152`, a whole number
///   of pieces, so a count that adds one past the quotient, and a count taken by
///   the piece-group size, fail.
/// Act:     `geometry.piece_count()`.
/// Assert:  the returned `u64` equals the literal `3`, the pieces `49152` bytes
///   fill at `16384` bytes each, by `assert_eq!`.
#[test]
fn piece_count_does_not_round_up_an_exact_extent() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(65536),
        total_extent: Some(49152),
    });

    // Act
    let piece_count = geometry.piece_count();

    // Assert
    assert_eq!(piece_count, 3);
}

/// Contract: `PieceGeometry::group_count(&self) -> u64` →
///   `self.total_extent.div_ceil(u64::from(self.piece_group_size))`.
/// Arrange: `build_piece_geometry` overriding the piece size with `16384`, the
///   piece-group size with `65536`, and the extent with `131072`, a whole number
///   of groups, so a count that adds one past the quotient, and a count taken by
///   the piece size, fail.
/// Act:     `geometry.group_count()`.
/// Assert:  the returned `u64` equals the literal `2`, the groups `131072` bytes
///   fill at `65536` bytes each, by `assert_eq!`.
#[test]
fn group_count_does_not_round_up_an_exact_extent() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(65536),
        total_extent: Some(131072),
    });

    // Act
    let group_count = geometry.group_count();

    // Assert
    assert_eq!(group_count, 2);
}

/// Contract: `PieceGeometry::piece_count(&self) -> u64` →
///   `self.total_extent.div_ceil(u64::from(self.piece_size))`.
/// Arrange: `build_piece_geometry` overriding the piece size with `16384`, the
///   piece-group size with `65536`, and the extent with `u64::MAX`, so a count
///   that adds the divisor before dividing overflows.
/// Act:     `geometry.piece_count()`.
/// Assert:  the returned `u64` equals the literal `1 << 50`, the quotient of
///   `2^64` by the piece size `2^14`, which rounding up `u64::MAX`, one below
///   `2^64`, reaches, by `assert_eq!`.
#[test]
fn piece_count_holds_at_the_largest_extent() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(65536),
        total_extent: Some(u64::MAX),
    });

    // Act
    let piece_count = geometry.piece_count();

    // Assert
    assert_eq!(piece_count, 1 << 50);
}

/// Contract: `PieceGeometry::group_count(&self) -> u64` →
///   `self.total_extent.div_ceil(u64::from(self.piece_group_size))`.
/// Arrange: `build_piece_geometry` overriding the piece size with `16384`, the
///   piece-group size with `65536`, and the extent with `u64::MAX`, so a count
///   that adds the divisor before dividing overflows.
/// Act:     `geometry.group_count()`.
/// Assert:  the returned `u64` equals the literal `1 << 48`, the quotient of
///   `2^64` by the piece-group size `2^16`, which rounding up `u64::MAX`, one
///   below `2^64`, reaches, by `assert_eq!`.
#[test]
fn group_count_holds_at_the_largest_extent() {
    // Arrange
    let geometry = build_piece_geometry(PieceGeometryConstructorParamsOverrides {
        piece_size: Some(16384),
        piece_group_size: Some(65536),
        total_extent: Some(u64::MAX),
    });

    // Act
    let group_count = geometry.group_count();

    // Assert
    assert_eq!(group_count, 1 << 48);
}
