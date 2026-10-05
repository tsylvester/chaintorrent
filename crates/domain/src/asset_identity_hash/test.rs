#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{AssetIdentityHash, AssetIdentityHashTryNewErrorReturn};
use super::mock::{
    AssetIdentityHashConstructorParamsOverrides, build_asset_identity_hash_constructor_params,
};

/// Contract: any nonzero candidate key is admitted and read back unchanged,
///   the last byte included.
/// Arrange: params built with `bytes` holding 31 zero bytes followed by `0x01`.
/// Act:     `AssetIdentityHash::try_new(params)`.
/// Assert:  `hash.as_bytes()` equals an array of 31 zero bytes followed by
///   `0x01`.
#[test]
fn try_new_admits_a_hash_whose_only_nonzero_byte_is_the_last() {
    // Arrange
    let mut bytes = [0u8; 32];
    bytes[31] = 0x01;
    let params =
        build_asset_identity_hash_constructor_params(AssetIdentityHashConstructorParamsOverrides {
            bytes: Some(bytes),
        });

    // Act
    let Ok(hash) = AssetIdentityHash::try_new(params) else {
        panic!("a hash with a nonzero byte is admitted")
    };

    // Assert
    let mut expected = [0u8; 32];
    expected[31] = 0x01;
    assert_eq!(hash.as_bytes(), &expected);
}

/// Contract: the zero check reads every byte, the first included.
/// Arrange: params built with `bytes` holding `0x80` followed by 31 zero
///   bytes.
/// Act:     `AssetIdentityHash::try_new(params)`.
/// Assert:  `hash.as_bytes()` equals an array of `0x80` followed by 31 zero
///   bytes.
#[test]
fn try_new_admits_a_hash_whose_only_nonzero_byte_is_the_first() {
    // Arrange
    let mut bytes = [0u8; 32];
    bytes[0] = 0x80;
    let params =
        build_asset_identity_hash_constructor_params(AssetIdentityHashConstructorParamsOverrides {
            bytes: Some(bytes),
        });

    // Act
    let Ok(hash) = AssetIdentityHash::try_new(params) else {
        panic!("a hash with a nonzero byte is admitted")
    };

    // Assert
    let mut expected = [0u8; 32];
    expected[0] = 0x80;
    assert_eq!(hash.as_bytes(), &expected);
}

/// Contract: the all-zero value, what an unassigned `bytes32` slot reads as,
///   names no asset and is refused.
/// Arrange: params built with `bytes: Some([0u8; 32])`.
/// Act:     `AssetIdentityHash::try_new(params)`.
/// Assert:  `error` equals `AssetIdentityHashTryNewErrorReturn::AllZero`.
#[test]
fn try_new_rejects_the_all_zero_hash() {
    // Arrange
    let params =
        build_asset_identity_hash_constructor_params(AssetIdentityHashConstructorParamsOverrides {
            bytes: Some([0u8; 32]),
        });

    // Act
    let Err(error) = AssetIdentityHash::try_new(params) else {
        panic!("the all-zero hash is refused")
    };

    // Assert
    assert_eq!(error, AssetIdentityHashTryNewErrorReturn::AllZero);
}
