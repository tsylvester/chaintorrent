mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    DEPLOYMENT_IDENTITY_LENGTH, DeploymentIdentity, DeploymentIdentityConstructorParams,
    DeploymentIdentityTryNewErrorReturn, DeploymentIdentityTryNewReturn,
};

impl DeploymentIdentity {
    pub fn try_new(params: DeploymentIdentityConstructorParams) -> DeploymentIdentityTryNewReturn {
        if params.bytes.iter().all(|byte| *byte == 0) {
            return Err(DeploymentIdentityTryNewErrorReturn::AllZero);
        }
        Ok(DeploymentIdentity {
            bytes: params.bytes,
        })
    }

    pub fn as_bytes(&self) -> &[u8; DEPLOYMENT_IDENTITY_LENGTH] {
        &self.bytes
    }
}
