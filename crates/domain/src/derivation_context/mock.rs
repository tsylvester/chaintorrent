#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{DerivationContext, DerivationContextConstructorParams};
use crate::asset_identity::provides::{AssetIdentity, build_asset_identity};
use crate::deployment_identity::provides::{DeploymentIdentity, build_deployment_identity};
use crate::group_index::provides::{GroupIndex, build_group_index};
use crate::parameter_set_identifier::provides::{
    ParameterSetIdentifier, build_parameter_set_identifier,
};
use crate::piece_geometry::provides::{PieceGeometry, build_piece_geometry};
use crate::suite_identifier::provides::{SuiteIdentifier, build_suite_identifier};

#[derive(Default)]
pub struct DerivationContextConstructorParamsOverrides {
    pub asset: Option<AssetIdentity>,
    pub deployment: Option<DeploymentIdentity>,
    pub suite: Option<SuiteIdentifier>,
    pub parameter_set: Option<ParameterSetIdentifier>,
    pub group_index: Option<GroupIndex>,
    pub geometry: Option<PieceGeometry>,
}

pub fn build_derivation_context_constructor_params(
    overrides: DerivationContextConstructorParamsOverrides,
) -> DerivationContextConstructorParams {
    DerivationContextConstructorParams {
        asset: overrides
            .asset
            .unwrap_or_else(|| build_asset_identity(Default::default())),
        deployment: overrides
            .deployment
            .unwrap_or_else(|| build_deployment_identity(Default::default())),
        suite: overrides
            .suite
            .unwrap_or_else(|| build_suite_identifier(Default::default())),
        parameter_set: overrides
            .parameter_set
            .unwrap_or_else(|| build_parameter_set_identifier(Default::default())),
        group_index: overrides
            .group_index
            .unwrap_or_else(|| build_group_index(Default::default())),
        geometry: overrides
            .geometry
            .unwrap_or_else(|| build_piece_geometry(Default::default())),
    }
}

pub fn build_derivation_context(
    overrides: DerivationContextConstructorParamsOverrides,
) -> DerivationContext {
    DerivationContext::try_new(build_derivation_context_constructor_params(overrides))
        .expect("built derivation context constructor params are admitted")
}
