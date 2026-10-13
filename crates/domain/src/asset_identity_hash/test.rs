#![allow(clippy::expect_used)]

use super::provides::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParamsOverrides,
    AssetIdentityHashTryNewErrorReturn, build_asset_identity_hash,
    build_asset_identity_hash_constructor_params,
};

/// Contract: every byte of `params.bytes` is `0` →
///   `Err(AssetIdentityHashTryNewErrorReturn::AllZero)`.
/// Arrange: `build_asset_identity_hash_constructor_params` overriding the bytes
///   with the array of `ASSET_IDENTITY_HASH_LENGTH` zero bytes, the value an
///   unassigned `bytes32` slot reads as; the builder's default array is nonzero,
///   so the override establishes the refused value.
/// Act:     `AssetIdentityHash::try_new(params)`.
/// Assert:  the result equals `Err(AssetIdentityHashTryNewErrorReturn::AllZero)`
///   by `assert_eq!`, compared whole.
#[test]
fn try_new_rejects_the_all_zero_hash() {
    // Arrange
    let params =
        build_asset_identity_hash_constructor_params(AssetIdentityHashConstructorParamsOverrides {
            bytes: Some([0; ASSET_IDENTITY_HASH_LENGTH]),
        });

    // Act
    let result = AssetIdentityHash::try_new(params);

    // Assert
    assert_eq!(result, Err(AssetIdentityHashTryNewErrorReturn::AllZero));
}

/// Contract: some byte of `params.bytes` is nonzero →
///   `Ok(AssetIdentityHash { bytes })`, the array moved from the params.
/// Arrange: `build_asset_identity_hash_constructor_params` overriding the bytes
///   with an array of `ASSET_IDENTITY_HASH_LENGTH` zeros whose last offset holds
///   `0x01`, so a check that reads only the leading bytes refuses this value.
/// Act:     `AssetIdentityHash::try_new(params)`.
/// Assert:  the result is the `Ok` arm; the hash's `bytes` field, read directly
///   and never through `as_bytes`, equals the array literal of zeros with `0x01`
///   at the last offset.
#[test]
fn try_new_admits_a_hash_whose_only_nonzero_byte_is_the_last() {
    // Arrange
    let mut bytes = [0; ASSET_IDENTITY_HASH_LENGTH];
    bytes[ASSET_IDENTITY_HASH_LENGTH - 1] = 0x01;
    let params =
        build_asset_identity_hash_constructor_params(AssetIdentityHashConstructorParamsOverrides {
            bytes: Some(bytes),
        });

    // Act
    let hash = AssetIdentityHash::try_new(params)
        .expect("a hash whose only nonzero byte is the last is admitted");

    // Assert
    assert_eq!(
        hash.bytes,
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ]
    );
}

/// Contract: some byte of `params.bytes` is nonzero →
///   `Ok(AssetIdentityHash { bytes })`, the array moved from the params.
/// Arrange: `build_asset_identity_hash_constructor_params` overriding the bytes
///   with an array of `ASSET_IDENTITY_HASH_LENGTH` zeros whose first offset holds
///   `0xFF`, so a check that reads only the trailing bytes refuses this value.
/// Act:     `AssetIdentityHash::try_new(params)`.
/// Assert:  the result is the `Ok` arm; the hash's `bytes` field, read directly
///   and never through `as_bytes`, equals the array literal of zeros with `0xFF`
///   at the first offset.
#[test]
fn try_new_admits_a_hash_whose_only_nonzero_byte_is_the_first() {
    // Arrange
    let mut bytes = [0; ASSET_IDENTITY_HASH_LENGTH];
    bytes[0] = 0xFF;
    let params =
        build_asset_identity_hash_constructor_params(AssetIdentityHashConstructorParamsOverrides {
            bytes: Some(bytes),
        });

    // Act
    let hash = AssetIdentityHash::try_new(params)
        .expect("a hash whose only nonzero byte is the first is admitted");

    // Assert
    assert_eq!(
        hash.bytes,
        [
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
}

/// Contract: `AssetIdentityHash::as_bytes(&self) ->
///   &[u8; ASSET_IDENTITY_HASH_LENGTH]` → a shared reference to the held array,
///   no copy.
/// Arrange: `build_asset_identity_hash` with the builder's defaults; an
///   implementation returning a copy or another array of equal content differs
///   only in address, which the address comparison separates.
/// Act:     `hash.as_bytes()`.
/// Assert:  the returned reference is address-identical to the hash's held
///   `bytes` field under `core::ptr::eq`.
#[test]
fn as_bytes_returns_a_reference_to_the_held_array() {
    // Arrange
    let hash = build_asset_identity_hash(Default::default());

    // Act
    let bytes = hash.as_bytes();

    // Assert
    assert!(core::ptr::eq(bytes, &hash.bytes));
}
