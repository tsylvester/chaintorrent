#![allow(clippy::panic)]

use super::provides::{
    GroupIndex, GroupIndexConstructorParamsOverrides, build_group_index,
    build_group_index_constructor_params,
};

/// Contract: any params → `Ok(GroupIndex { value })` holding `params.value`.
/// Arrange: `build_group_index_constructor_params` overriding the value with `0`, the
///   lowest `u64` and not the builder's default, so an implementation that rewrites
///   zero fails.
/// Act:     `GroupIndex::try_new` on the built params.
/// Assert:  the index's `value` field equals the literal `0`.
#[test]
fn try_new_admits_the_lowest_index() {
    // Arrange
    let params = build_group_index_constructor_params(GroupIndexConstructorParamsOverrides {
        value: Some(0),
    });

    // Act
    let Ok(index) = GroupIndex::try_new(params);

    // Assert
    assert_eq!(index.value, 0);
}

/// Contract: any params → `Ok(GroupIndex { value })` holding `params.value`.
/// Arrange: `build_group_index_constructor_params` overriding the value with
///   `u64::MAX`, the highest `u64`, so an implementation that clamps or narrows the
///   value fails.
/// Act:     `GroupIndex::try_new` on the built params.
/// Assert:  the index's `value` field equals the literal `u64::MAX`.
#[test]
fn try_new_admits_the_highest_index() {
    // Arrange
    let params = build_group_index_constructor_params(GroupIndexConstructorParamsOverrides {
        value: Some(u64::MAX),
    });

    // Act
    let Ok(index) = GroupIndex::try_new(params);

    // Assert
    assert_eq!(index.value, u64::MAX);
}

/// Contract: `GroupIndex::value(&self) -> u64` → the held index.
/// Arrange: `build_group_index` overriding the value with `0x0102_0304_0506_0708`, a
///   value whose eight bytes all differ, so a byte-swapped or truncated return fails.
/// Act:     `index.value()`.
/// Assert:  the returned `u64` equals the literal `0x0102_0304_0506_0708` by
///   `assert_eq!`.
#[test]
fn value_returns_the_held_index() {
    // Arrange
    let index = build_group_index(GroupIndexConstructorParamsOverrides {
        value: Some(0x0102_0304_0506_0708),
    });

    // Act
    let value = index.value();

    // Assert
    assert_eq!(value, 0x0102_0304_0506_0708);
}
