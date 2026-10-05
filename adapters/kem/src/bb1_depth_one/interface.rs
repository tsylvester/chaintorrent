use domain::Secret;
use hash_to_scalar::{
    DomainTag, DomainTagTryNewErrorReturn, HashToScalarErrorReturn, IHashToScalarAdapter,
};
use pairing::{IPairingAdapter, IPairingArithmetic, SampleUniformScalarErrorReturn};

use crate::factory::provides::{
    IdentityScope, KEM_INTERFACE_VERSION, KemDeclaration, KemIdentifier,
};

pub const BB1_DEPTH_ONE_IDENTITY_TAG: &[u8] = b"ChainTorrent-v1-kem-identity";

pub(crate) const BB1_DEPTH_ONE_DECLARATION: KemDeclaration = KemDeclaration {
    identifier: KemIdentifier::Bb1DepthOneV1,
    identity_scopes: &[IdentityScope::Entitlement, IdentityScope::Asset],
    identity_tag: BB1_DEPTH_ONE_IDENTITY_TAG,
    adapter_version: 1,
    interface_version: KEM_INTERFACE_VERSION,
};

pub struct Bb1DepthOneKem<'a, P: IPairingArithmetic> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) tag: DomainTag,
}

pub struct Bb1DepthOneKemConstructorParams<'a, P: IPairingArithmetic> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
}

pub enum Bb1DepthOneKemTryNewErrorReturn {
    DomainTag(DomainTagTryNewErrorReturn),
}

pub type Bb1DepthOneKemTryNewReturn<'a, P> =
    Result<Bb1DepthOneKem<'a, P>, Bb1DepthOneKemTryNewErrorReturn>;

pub enum Bb1DepthOneParameterSetScope<G> {
    Entitlement,
    Asset { identity_element: G },
}

pub struct Bb1DepthOneParameterSet<P: IPairingAdapter> {
    pub(super) g1: P::G1,
    pub(super) u0: P::G1,
    pub(super) u1: P::G1,
    pub(super) g2: P::G2,
    pub(super) hpub: P::G2,
    pub(super) scope: Bb1DepthOneParameterSetScope<P::G1>,
}

pub struct Bb1DepthOneMasterScalar<P: IPairingAdapter> {
    pub(super) value: Secret<P::Scalar>,
}

pub struct Bb1DepthOneIdentityElement<P: IPairingAdapter> {
    pub(super) scalar: P::Scalar,
    pub(super) element: P::G1,
}

pub struct Bb1DepthOneCredential<P: IPairingAdapter> {
    pub(super) a: P::G1,
    pub(super) b: P::G2,
}

pub enum Bb1DepthOneCapsule<P: IPairingAdapter> {
    Entitlement { u: P::G2, v: P::G1, w: P::G1 },
    Asset { u: P::G2, v: P::G1 },
}

pub enum Bb1DepthOneSetupErrorReturn {
    MasterScalarSampling(SampleUniformScalarErrorReturn),
    U0Sampling(SampleUniformScalarErrorReturn),
    U1Sampling(SampleUniformScalarErrorReturn),
    HashToScalar(HashToScalarErrorReturn),
    TrivialIdentityElement,
}

pub enum Bb1DepthOneDeriveIdentityErrorReturn {
    WrongIdentityScope,
    HashToScalar(HashToScalarErrorReturn),
    TrivialIdentityElement,
    OutsideAssetScope,
}

pub enum Bb1DepthOneIssueErrorReturn {
    Sampling(SampleUniformScalarErrorReturn),
}

pub enum Bb1DepthOneRerandomizeErrorReturn {
    Sampling(SampleUniformScalarErrorReturn),
}

pub enum Bb1DepthOneEncapsulateErrorReturn {
    Sampling(SampleUniformScalarErrorReturn),
}
