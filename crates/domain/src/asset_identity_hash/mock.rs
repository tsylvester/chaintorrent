#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParams,
};

#[derive(Default)]
pub struct AssetIdentityHashConstructorParamsOverrides {
    pub bytes: Option<[u8; ASSET_IDENTITY_HASH_LENGTH]>,
}

pub fn build_asset_identity_hash_constructor_params(
    overrides: AssetIdentityHashConstructorParamsOverrides,
) -> AssetIdentityHashConstructorParams {
    AssetIdentityHashConstructorParams {
        bytes: overrides
            .bytes
            .unwrap_or([0x44; ASSET_IDENTITY_HASH_LENGTH]),
    }
}

pub fn build_asset_identity_hash(
    overrides: AssetIdentityHashConstructorParamsOverrides,
) -> AssetIdentityHash {
    AssetIdentityHash::try_new(build_asset_identity_hash_constructor_params(overrides))
        .expect("built asset identity hash constructor params are admitted")
}
