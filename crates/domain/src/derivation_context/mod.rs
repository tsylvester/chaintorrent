mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::asset_identity::provides::AssetIdentity;
use crate::deployment_identity::provides::DeploymentIdentity;
use crate::group_index::provides::GroupIndex;
use crate::parameter_set_identifier::provides::ParameterSetIdentifier;
use crate::piece_geometry::provides::PieceGeometry;
use crate::suite_identifier::provides::SuiteIdentifier;
use interface::{
    DerivationContext, DerivationContextConstructorParams, DerivationContextTryNewErrorReturn,
    DerivationContextTryNewReturn,
};

impl DerivationContext {
    pub fn try_new(params: DerivationContextConstructorParams) -> DerivationContextTryNewReturn {
        let group_index = params.group_index.value();
        let group_count = params.geometry.group_count();
        if group_index >= group_count {
            return Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
                group_index,
                group_count,
            });
        }
        Ok(DerivationContext {
            asset: params.asset,
            deployment: params.deployment,
            suite: params.suite,
            parameter_set: params.parameter_set,
            group_index: params.group_index,
            geometry: params.geometry,
        })
    }

    pub fn asset(&self) -> &AssetIdentity {
        &self.asset
    }

    pub fn deployment(&self) -> &DeploymentIdentity {
        &self.deployment
    }

    pub fn suite(&self) -> &SuiteIdentifier {
        &self.suite
    }

    pub fn parameter_set(&self) -> &ParameterSetIdentifier {
        &self.parameter_set
    }

    pub fn group_index(&self) -> &GroupIndex {
        &self.group_index
    }

    pub fn geometry(&self) -> &PieceGeometry {
        &self.geometry
    }
}
