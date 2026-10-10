#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{AssetCoordinate, AssetIdentity, AssetIdentityConstructorParams};

#[derive(Default)]
pub struct AssetIdentityConstructorParamsOverrides {
    pub name: Option<String>,
    pub version: Option<String>,
}

pub fn build_asset_identity_constructor_params(
    overrides: AssetIdentityConstructorParamsOverrides,
) -> AssetIdentityConstructorParams {
    AssetIdentityConstructorParams {
        name: overrides
            .name
            .unwrap_or_else(|| "example-package".to_string()),
        version: overrides.version.unwrap_or_else(|| "1.0.0".to_string()),
    }
}

pub fn build_asset_identity(overrides: AssetIdentityConstructorParamsOverrides) -> AssetIdentity {
    AssetIdentity::try_new(build_asset_identity_constructor_params(overrides))
        .expect("built asset identity constructor params are admitted")
}

pub fn build_asset_coordinate(
    overrides: AssetIdentityConstructorParamsOverrides,
) -> AssetCoordinate {
    build_asset_identity(overrides).coordinate()
}
