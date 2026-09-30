use crate::factory::provides::CanonicalFieldKind;
use core::convert::Infallible;
use domain::{
    AssetIdentityTryNewErrorReturn, DeploymentIdentityTryNewErrorReturn,
    DerivationContextTryNewErrorReturn, ParameterSetIdentifierTryNewErrorReturn,
    PieceGeometryTryNewErrorReturn, SuiteIdentifierTryNewErrorReturn,
};

pub const DERIVATION_CONTEXT_FIELD_COUNT: usize = 10;

pub const DERIVATION_CONTEXT_FIELD_KINDS: [CanonicalFieldKind; DERIVATION_CONTEXT_FIELD_COUNT] = [
    CanonicalFieldKind::Text,
    CanonicalFieldKind::Text,
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::Unsigned16,
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::Unsigned64,
    CanonicalFieldKind::Unsigned32,
    CanonicalFieldKind::Unsigned32,
    CanonicalFieldKind::Unsigned64,
];

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
    AssetIdentity(AssetIdentityTryNewErrorReturn),
    DeploymentIdentity(DeploymentIdentityTryNewErrorReturn),
    SuiteIdentifier(SuiteIdentifierTryNewErrorReturn),
    ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn),
    PieceGeometry(PieceGeometryTryNewErrorReturn),
    DerivationContext(DerivationContextTryNewErrorReturn),
}
