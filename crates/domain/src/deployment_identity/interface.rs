pub const DEPLOYMENT_IDENTITY_LENGTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentIdentity {
    pub(super) bytes: [u8; DEPLOYMENT_IDENTITY_LENGTH],
}

pub struct DeploymentIdentityConstructorParams {
    pub bytes: [u8; DEPLOYMENT_IDENTITY_LENGTH],
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeploymentIdentityTryNewErrorReturn {
    AllZero,
}

pub type DeploymentIdentityTryNewReturn =
    Result<DeploymentIdentity, DeploymentIdentityTryNewErrorReturn>;
