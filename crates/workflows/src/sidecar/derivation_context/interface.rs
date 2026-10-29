use core::convert::Infallible;
use domain::{
    DeploymentIdentity, DeploymentIdentityTryNewErrorReturn, GroupIndex, ParameterSetIdentifier,
    ParameterSetIdentifierTryNewErrorReturn, PieceGeometry, PieceGeometryTryNewErrorReturn,
    SuiteIdentifier, SuiteIdentifierTryNewErrorReturn,
};
use encoding::CanonicalFieldKind;
use hashing::{Root, RootTryNewErrorReturn};

pub const DERIVATION_CONTEXT_FIELD_COUNT: usize = 9;

pub const DERIVATION_CONTEXT_FIELD_KINDS: [CanonicalFieldKind; DERIVATION_CONTEXT_FIELD_COUNT] = [
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::Unsigned16,
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::Unsigned64,
    CanonicalFieldKind::Unsigned32,
    CanonicalFieldKind::Unsigned32,
    CanonicalFieldKind::Unsigned64,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivationContext {
    pub(super) asset: Root,
    pub(super) deployment: DeploymentIdentity,
    pub(super) suite: SuiteIdentifier,
    pub(super) parameter_set: ParameterSetIdentifier,
    pub(super) group_index: GroupIndex,
    pub(super) geometry: PieceGeometry,
}

pub struct DerivationContextConstructorParams {
    pub asset: Root,
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

pub struct DerivationContextDescription;

pub struct DerivationContextDescriptionConstructorParams;

pub type DerivationContextDescriptionTryNewReturn =
    Result<DerivationContextDescription, Infallible>;

#[derive(Debug, PartialEq, Eq)]
pub enum DerivationContextFromFieldsErrorReturn {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
    Asset(RootTryNewErrorReturn),
    Deployment(DeploymentIdentityTryNewErrorReturn),
    Suite(SuiteIdentifierTryNewErrorReturn),
    ParameterSet(ParameterSetIdentifierTryNewErrorReturn),
    Geometry(PieceGeometryTryNewErrorReturn),
    Context(DerivationContextTryNewErrorReturn),
}
