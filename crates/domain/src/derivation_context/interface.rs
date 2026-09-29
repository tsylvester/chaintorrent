use crate::asset_identity::provides::AssetIdentity;
use crate::deployment_identity::provides::DeploymentIdentity;
use crate::group_index::provides::GroupIndex;
use crate::parameter_set_identifier::provides::ParameterSetIdentifier;
use crate::piece_geometry::provides::PieceGeometry;
use crate::suite_identifier::provides::SuiteIdentifier;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivationContext {
    pub(super) asset: AssetIdentity,
    pub(super) deployment: DeploymentIdentity,
    pub(super) suite: SuiteIdentifier,
    pub(super) parameter_set: ParameterSetIdentifier,
    pub(super) group_index: GroupIndex,
    pub(super) geometry: PieceGeometry,
}

pub struct DerivationContextConstructorParams {
    pub asset: AssetIdentity,
    pub deployment: DeploymentIdentity,
    pub suite: SuiteIdentifier,
    pub parameter_set: ParameterSetIdentifier,
    pub group_index: GroupIndex,
    pub geometry: PieceGeometry,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DerivationContextTryNewErrorReturn {
    GroupIndexOutOfRange { group_index: u64, group_count: u64 },
}

pub type DerivationContextTryNewReturn =
    Result<DerivationContext, DerivationContextTryNewErrorReturn>;
