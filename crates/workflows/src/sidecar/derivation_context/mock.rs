#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    DerivationContext, DerivationContextConstructorParams, DerivationContextDescription,
    DerivationContextDescriptionConstructorParams,
};
use domain::{
    DeploymentIdentity, GroupIndex, ParameterSetIdentifier, PieceGeometry, SuiteIdentifier,
    build_deployment_identity, build_group_index, build_parameter_set_identifier,
    build_piece_geometry, build_suite_identifier,
};
use encoding::{
    CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides, build_canonical_fields,
};
use hashing::{Root, build_root};

#[derive(Default)]
pub struct DerivationContextConstructorParamsOverrides {
    pub asset: Option<Root>,
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
            .unwrap_or_else(|| build_root(Default::default())),
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

#[derive(Default)]
pub struct DerivationContextConstructorParamsCorruptions {
    pub asset: Option<Vec<CanonicalFieldValue>>,
    pub deployment: Option<Vec<CanonicalFieldValue>>,
    pub suite: Option<Vec<CanonicalFieldValue>>,
    pub parameter_set: Option<Vec<CanonicalFieldValue>>,
    pub group_index: Option<Vec<CanonicalFieldValue>>,
    pub geometry: Option<Vec<CanonicalFieldValue>>,
}

pub fn invalidate_derivation_context_constructor_params(
    corruptions: DerivationContextConstructorParamsCorruptions,
) -> CanonicalFields {
    let params = build_derivation_context_constructor_params(Default::default());
    let asset = corruptions
        .asset
        .unwrap_or_else(|| vec![CanonicalFieldValue::FixedBytes32(*params.asset.as_bytes())]);
    let deployment = corruptions.deployment.unwrap_or_else(|| {
        vec![CanonicalFieldValue::FixedBytes32(
            *params.deployment.as_bytes(),
        )]
    });
    let suite = corruptions.suite.unwrap_or_else(|| {
        vec![
            CanonicalFieldValue::FixedBytes32(*params.suite.identifier()),
            CanonicalFieldValue::Unsigned16(params.suite.version()),
        ]
    });
    let parameter_set = corruptions.parameter_set.unwrap_or_else(|| {
        vec![CanonicalFieldValue::FixedBytes32(
            *params.parameter_set.as_bytes(),
        )]
    });
    let group_index = corruptions
        .group_index
        .unwrap_or_else(|| vec![CanonicalFieldValue::Unsigned64(params.group_index.value())]);
    let geometry = corruptions.geometry.unwrap_or_else(|| {
        vec![
            CanonicalFieldValue::Unsigned32(params.geometry.piece_size()),
            CanonicalFieldValue::Unsigned32(params.geometry.piece_group_size()),
            CanonicalFieldValue::Unsigned64(params.geometry.total_extent()),
        ]
    });
    let values = [
        asset,
        deployment,
        suite,
        parameter_set,
        group_index,
        geometry,
    ]
    .concat();
    build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    })
}

#[derive(Default)]
pub struct DerivationContextDescriptionConstructorParamsOverrides;

pub fn build_derivation_context_description_constructor_params(
    _overrides: DerivationContextDescriptionConstructorParamsOverrides,
) -> DerivationContextDescriptionConstructorParams {
    DerivationContextDescriptionConstructorParams
}

pub fn build_derivation_context_description(
    overrides: DerivationContextDescriptionConstructorParamsOverrides,
) -> DerivationContextDescription {
    let Ok(description) = DerivationContextDescription::try_new(
        build_derivation_context_description_constructor_params(overrides),
    );
    description
}
