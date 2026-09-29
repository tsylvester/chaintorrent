#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    DEPLOYMENT_IDENTITY_LENGTH, DeploymentIdentity, DeploymentIdentityConstructorParams,
};

#[derive(Default)]
pub struct DeploymentIdentityConstructorParamsOverrides {
    pub bytes: Option<[u8; DEPLOYMENT_IDENTITY_LENGTH]>,
}

pub fn build_deployment_identity_constructor_params(
    overrides: DeploymentIdentityConstructorParamsOverrides,
) -> DeploymentIdentityConstructorParams {
    DeploymentIdentityConstructorParams {
        bytes: overrides
            .bytes
            .unwrap_or([0x11; DEPLOYMENT_IDENTITY_LENGTH]),
    }
}

pub fn build_deployment_identity(
    overrides: DeploymentIdentityConstructorParamsOverrides,
) -> DeploymentIdentity {
    DeploymentIdentity::try_new(build_deployment_identity_constructor_params(overrides))
        .expect("built deployment identity constructor params are admitted")
}
