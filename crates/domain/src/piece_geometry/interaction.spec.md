# piece_geometry interactions

`PieceGeometry` is an owned value type: a fallible constructor `try_new` is its only producer, and the read accessors `piece_size`, `piece_group_size`, `total_extent`, `piece_count`, and `group_count` are its only views. There are no dependency calls; the only decisions are arithmetic checks on the declared sizes and extent.

## `PieceGeometry::try_new`

`PieceGeometry::try_new(params: PieceGeometryConstructorParams) -> PieceGeometryTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| piece size not a power of two | `!params.piece_size.is_power_of_two()`, zero included | the power-of-two check | none | `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })` |
| piece size below the minimum | the piece size is a power of two and `params.piece_size < MINIMUM_PIECE_SIZE` | the comparison | none | `Err(PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum { piece_size, minimum: MINIMUM_PIECE_SIZE })` |
| zero piece-group size | the piece size passes and `params.piece_group_size == 0` | the equality check | none | `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)` |
| piece-group size not a multiple of the piece size | the piece size passes, the piece-group size is nonzero, and `params.piece_group_size % params.piece_size != 0` | the remainder check | none | `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size, piece_size })` |
| zero total extent | both sizes pass and `params.total_extent == 0` | the equality check | none | `Err(PieceGeometryTryNewErrorReturn::ZeroTotalExtent)` |
| admitted | every check passes | none further | none | `Ok(PieceGeometry { piece_size, piece_group_size, total_extent })`, each moved from the params |

Ordering: the piece size's checks precede the piece-group size's, which precede the total extent's; the same params always yield the same outcome.

## `PieceGeometry::piece_size`, `piece_group_size`, `total_extent`

`PieceGeometry::piece_size(&self) -> u32` `PieceGeometry::piece_group_size(&self) -> u32` `PieceGeometry::total_extent(&self) -> u64`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | the held value |

## `PieceGeometry::piece_count`

`PieceGeometry::piece_count(&self) -> u64`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `self.total_extent.div_ceil(u64::from(self.piece_size))`, nonzero because both operands are |

## `PieceGeometry::group_count`

`PieceGeometry::group_count(&self) -> u64`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `self.total_extent.div_ceil(u64::from(self.piece_group_size))`, nonzero because both operands are |

## Invariants

- Every `PieceGeometry` holds a power-of-two piece size of at least 16 KiB, a nonzero piece-group size that is a whole multiple of it, and a nonzero total extent.
- `try_new` is the only producer; `Clone` copies only an already-admitted value, so no admitted form exists outside the constructor's invariants.

