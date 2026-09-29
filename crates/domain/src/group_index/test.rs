#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::GroupIndex;
use super::mock::{GroupIndexConstructorParamsOverrides, build_group_index_constructor_params};

/// Contract: the lowest index is admitted and read back unchanged.
/// Arrange: params built with `value` overridden to `0`, differing from the
///   builder's default.
/// Act:     `GroupIndex::try_new(params)`.
/// Assert:  `index.value()` equals `0`.
#[test]
fn try_new_admits_index_zero() {
    // Arrange
    let params = build_group_index_constructor_params(GroupIndexConstructorParamsOverrides {
        value: Some(0),
    });

    // Act
    let Ok(index) = GroupIndex::try_new(params);

    // Assert
    assert_eq!(index.value(), 0);
}

/// Contract: the full width of the type is admitted and read back unchanged.
/// Arrange: params built with `value` overridden to `u64::MAX`.
/// Act:     `GroupIndex::try_new(params)`.
/// Assert:  `index.value()` equals `18446744073709551615`.
#[test]
fn try_new_admits_the_largest_index() {
    // Arrange
    let params = build_group_index_constructor_params(GroupIndexConstructorParamsOverrides {
        value: Some(u64::MAX),
    });

    // Act
    let Ok(index) = GroupIndex::try_new(params);

    // Assert
    assert_eq!(index.value(), 18446744073709551615);
}
