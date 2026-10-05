use core::convert::Infallible;
use domain::{AssetIdentityHash, Secret};
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingAdapter, IPairingArithmetic};
use zeroize::Zeroize;

use crate::bb1_depth_one::provides::{
    Bb1DepthOneDeriveIdentityErrorReturn, Bb1DepthOneEncapsulateErrorReturn,
    Bb1DepthOneIssueErrorReturn, Bb1DepthOneKemTryNewErrorReturn,
    Bb1DepthOneRerandomizeErrorReturn, Bb1DepthOneSetupErrorReturn,
};

pub const KEM_INTERFACE_VERSION: u32 = 1;

#[derive(PartialEq, Eq)]
pub enum KemIdentifier {
    Bb1DepthOneV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityScope {
    Entitlement,
    Asset,
}

pub struct KemDeclaration {
    pub identifier: KemIdentifier,
    pub identity_scopes: &'static [IdentityScope],
    pub identity_tag: &'static [u8],
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct EncapsulatedValue {
    pub(crate) bytes: Secret<Vec<u8>>,
}

pub enum SetupScope<'a> {
    Entitlement,
    Asset {
        identity_hash: &'a AssetIdentityHash,
    },
}

pub struct SetupParams<'a> {
    pub scope: SetupScope<'a>,
}

pub struct SetupPayload {
    pub master_uniform: Secret<Vec<u8>>,
    pub u0_uniform: Secret<Vec<u8>>,
    pub u1_uniform: Secret<Vec<u8>>,
}

pub struct SetupSuccessReturn<PS, M> {
    pub parameter_set: PS,
    pub master_scalar: M,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SetupErrorReturn {
    Bb1DepthOne(Bb1DepthOneSetupErrorReturn),
}

pub type SetupReturn<PS, M> = Result<SetupSuccessReturn<PS, M>, SetupErrorReturn>;

pub enum KemIdentity<'a> {
    Entitlement {
        canonical: &'a [u8],
    },
    Asset {
        identity_hash: &'a AssetIdentityHash,
    },
}

pub struct DeriveIdentityParams;

pub struct DeriveIdentityPayload<'a, PS> {
    pub parameter_set: &'a PS,
    pub identity: KemIdentity<'a>,
}

pub struct DeriveIdentitySuccessReturn<IE> {
    pub identity_element: IE,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeriveIdentityErrorReturn {
    Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn),
}

pub type DeriveIdentityReturn<IE> =
    Result<DeriveIdentitySuccessReturn<IE>, DeriveIdentityErrorReturn>;

pub struct IssueParams;

pub struct IssuePayload<'a, PS, M, IE> {
    pub parameter_set: &'a PS,
    pub master_scalar: &'a M,
    pub identity_element: &'a IE,
    pub uniform: Secret<Vec<u8>>,
}

pub struct IssueSuccessReturn<C, S: Zeroize> {
    pub credential: C,
    pub randomness: Secret<S>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IssueErrorReturn {
    Bb1DepthOne(Bb1DepthOneIssueErrorReturn),
}

pub type IssueReturn<C, S> = Result<IssueSuccessReturn<C, S>, IssueErrorReturn>;

pub struct RerandomizeParams;

pub struct RerandomizePayload<'a, PS, IE, C> {
    pub parameter_set: &'a PS,
    pub identity_element: &'a IE,
    pub credential: &'a C,
    pub uniform: Secret<Vec<u8>>,
}

pub struct RerandomizeSuccessReturn<C, S: Zeroize> {
    pub credential: C,
    pub offset: Secret<S>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RerandomizeErrorReturn {
    Bb1DepthOne(Bb1DepthOneRerandomizeErrorReturn),
}

pub type RerandomizeReturn<C, S> = Result<RerandomizeSuccessReturn<C, S>, RerandomizeErrorReturn>;

pub struct IsValidParams;

pub struct IsValidPayload<'a, PS, IE, C> {
    pub parameter_set: &'a PS,
    pub identity_element: &'a IE,
    pub credential: &'a C,
}

pub struct IsValidSuccessReturn {
    pub is_valid: bool,
}

pub type IsValidReturn = Result<IsValidSuccessReturn, Infallible>;

pub struct EncapsulateParams;

pub struct EncapsulatePayload<'a, PS> {
    pub parameter_set: &'a PS,
    pub uniform: Secret<Vec<u8>>,
}

pub struct EncapsulateSuccessReturn<CA> {
    pub capsule: CA,
    pub encapsulated: EncapsulatedValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EncapsulateErrorReturn {
    Bb1DepthOne(Bb1DepthOneEncapsulateErrorReturn),
}

pub type EncapsulateReturn<CA> = Result<EncapsulateSuccessReturn<CA>, EncapsulateErrorReturn>;

pub struct IsWellFormedParams;

pub struct IsWellFormedPayload<'a, PS, CA> {
    pub parameter_set: &'a PS,
    pub capsule: &'a CA,
}

pub struct IsWellFormedSuccessReturn {
    pub is_well_formed: bool,
}

pub type IsWellFormedReturn = Result<IsWellFormedSuccessReturn, Infallible>;

pub struct DecapsulateParams;

pub struct DecapsulatePayload<'a, IE, C, CA> {
    pub identity_element: &'a IE,
    pub credential: &'a C,
    pub capsule: &'a CA,
}

pub struct DecapsulateSuccessReturn {
    pub encapsulated: EncapsulatedValue,
}

pub type DecapsulateReturn = Result<DecapsulateSuccessReturn, Infallible>;

pub struct CredentialComponents<G1, G2> {
    pub a: G1,
    pub b: G2,
}

pub enum ParameterSetScopeComponents<G1> {
    Entitlement,
    Asset { identity_element: G1 },
}

pub struct ParameterSetComponents<G1, G2> {
    pub g1: G1,
    pub u0: G1,
    pub u1: G1,
    pub g2: G2,
    pub hpub: G2,
    pub scope: ParameterSetScopeComponents<G1>,
}

pub struct IdentityElementComponents<S, G1> {
    pub scalar: S,
    pub element: G1,
}

pub enum CapsuleComponents<G1, G2> {
    Entitlement { u: G2, v: G1, w: G1 },
    Asset { u: G2, v: G1 },
}

pub struct MasterScalarComponents<S: Zeroize> {
    pub value: Secret<S>,
}

pub struct CredentialComponentsParams;

pub struct CredentialComponentsPayload<'a, C> {
    pub credential: &'a C,
}

pub struct CredentialComponentsSuccessReturn<G1, G2> {
    pub components: CredentialComponents<G1, G2>,
}

pub type CredentialComponentsReturn<G1, G2> =
    Result<CredentialComponentsSuccessReturn<G1, G2>, Infallible>;

pub struct CredentialFromComponentsParams;

pub struct CredentialFromComponentsPayload<G1, G2> {
    pub components: CredentialComponents<G1, G2>,
}

pub struct CredentialFromComponentsSuccessReturn<C> {
    pub credential: C,
}

pub type CredentialFromComponentsReturn<C> =
    Result<CredentialFromComponentsSuccessReturn<C>, Infallible>;

pub struct ParameterSetComponentsParams;

pub struct ParameterSetComponentsPayload<'a, PS> {
    pub parameter_set: &'a PS,
}

pub struct ParameterSetComponentsSuccessReturn<G1, G2> {
    pub components: ParameterSetComponents<G1, G2>,
}

pub type ParameterSetComponentsReturn<G1, G2> =
    Result<ParameterSetComponentsSuccessReturn<G1, G2>, Infallible>;

pub struct ParameterSetFromComponentsParams;

pub struct ParameterSetFromComponentsPayload<G1, G2> {
    pub components: ParameterSetComponents<G1, G2>,
}

pub struct ParameterSetFromComponentsSuccessReturn<PS> {
    pub parameter_set: PS,
}

pub type ParameterSetFromComponentsReturn<PS> =
    Result<ParameterSetFromComponentsSuccessReturn<PS>, Infallible>;

pub struct IdentityElementComponentsParams;

pub struct IdentityElementComponentsPayload<'a, IE> {
    pub identity_element: &'a IE,
}

pub struct IdentityElementComponentsSuccessReturn<S, G1> {
    pub components: IdentityElementComponents<S, G1>,
}

pub type IdentityElementComponentsReturn<S, G1> =
    Result<IdentityElementComponentsSuccessReturn<S, G1>, Infallible>;

pub struct CapsuleComponentsParams;

pub struct CapsuleComponentsPayload<'a, CA> {
    pub capsule: &'a CA,
}

pub struct CapsuleComponentsSuccessReturn<G1, G2> {
    pub components: CapsuleComponents<G1, G2>,
}

pub type CapsuleComponentsReturn<G1, G2> =
    Result<CapsuleComponentsSuccessReturn<G1, G2>, Infallible>;

pub struct CapsuleFromComponentsParams;

pub struct CapsuleFromComponentsPayload<G1, G2> {
    pub components: CapsuleComponents<G1, G2>,
}

pub struct CapsuleFromComponentsSuccessReturn<CA> {
    pub capsule: CA,
}

pub type CapsuleFromComponentsReturn<CA> =
    Result<CapsuleFromComponentsSuccessReturn<CA>, Infallible>;

pub struct MasterScalarComponentsParams;

pub struct MasterScalarComponentsPayload<'a, M> {
    pub master_scalar: &'a M,
}

pub struct MasterScalarComponentsSuccessReturn<S: Zeroize> {
    pub components: MasterScalarComponents<S>,
}

pub type MasterScalarComponentsReturn<S> =
    Result<MasterScalarComponentsSuccessReturn<S>, Infallible>;

pub struct MasterScalarFromComponentsParams;

pub struct MasterScalarFromComponentsPayload<S: Zeroize> {
    pub components: MasterScalarComponents<S>,
}

pub struct MasterScalarFromComponentsSuccessReturn<M> {
    pub master_scalar: M,
}

pub type MasterScalarFromComponentsReturn<M> =
    Result<MasterScalarFromComponentsSuccessReturn<M>, Infallible>;

pub trait ICredentialKemAdapter {
    const DECLARATION: KemDeclaration;

    type Pairing: IPairingAdapter;
    type ParameterSet;
    type MasterScalar;
    type IdentityElement;
    type Credential;
    type Capsule;

    fn setup(
        &self,
        params: SetupParams<'_>,
        payload: SetupPayload,
    ) -> SetupReturn<Self::ParameterSet, Self::MasterScalar>;

    fn derive_identity(
        &self,
        params: DeriveIdentityParams,
        payload: DeriveIdentityPayload<'_, Self::ParameterSet>,
    ) -> DeriveIdentityReturn<Self::IdentityElement>;

    fn issue(
        &self,
        params: IssueParams,
        payload: IssuePayload<'_, Self::ParameterSet, Self::MasterScalar, Self::IdentityElement>,
    ) -> IssueReturn<Self::Credential, <Self::Pairing as IPairingAdapter>::Scalar>;

    fn rerandomize(
        &self,
        params: RerandomizeParams,
        payload: RerandomizePayload<
            '_,
            Self::ParameterSet,
            Self::IdentityElement,
            Self::Credential,
        >,
    ) -> RerandomizeReturn<Self::Credential, <Self::Pairing as IPairingAdapter>::Scalar>;

    fn is_valid(
        &self,
        params: IsValidParams,
        payload: IsValidPayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>,
    ) -> IsValidReturn;

    fn encapsulate(
        &self,
        params: EncapsulateParams,
        payload: EncapsulatePayload<'_, Self::ParameterSet>,
    ) -> EncapsulateReturn<Self::Capsule>;

    fn is_well_formed(
        &self,
        params: IsWellFormedParams,
        payload: IsWellFormedPayload<'_, Self::ParameterSet, Self::Capsule>,
    ) -> IsWellFormedReturn;

    fn decapsulate(
        &self,
        params: DecapsulateParams,
        payload: DecapsulatePayload<'_, Self::IdentityElement, Self::Credential, Self::Capsule>,
    ) -> DecapsulateReturn;

    fn credential_components(
        &self,
        params: CredentialComponentsParams,
        payload: CredentialComponentsPayload<'_, Self::Credential>,
    ) -> CredentialComponentsReturn<
        <Self::Pairing as IPairingAdapter>::G1,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    fn credential_from_components(
        &self,
        params: CredentialFromComponentsParams,
        payload: CredentialFromComponentsPayload<
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> CredentialFromComponentsReturn<Self::Credential>;

    fn parameter_set_components(
        &self,
        params: ParameterSetComponentsParams,
        payload: ParameterSetComponentsPayload<'_, Self::ParameterSet>,
    ) -> ParameterSetComponentsReturn<
        <Self::Pairing as IPairingAdapter>::G1,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    fn parameter_set_from_components(
        &self,
        params: ParameterSetFromComponentsParams,
        payload: ParameterSetFromComponentsPayload<
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> ParameterSetFromComponentsReturn<Self::ParameterSet>;

    fn identity_element_components(
        &self,
        params: IdentityElementComponentsParams,
        payload: IdentityElementComponentsPayload<'_, Self::IdentityElement>,
    ) -> IdentityElementComponentsReturn<
        <Self::Pairing as IPairingAdapter>::Scalar,
        <Self::Pairing as IPairingAdapter>::G1,
    >;

    fn capsule_components(
        &self,
        params: CapsuleComponentsParams,
        payload: CapsuleComponentsPayload<'_, Self::Capsule>,
    ) -> CapsuleComponentsReturn<
        <Self::Pairing as IPairingAdapter>::G1,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    fn capsule_from_components(
        &self,
        params: CapsuleFromComponentsParams,
        payload: CapsuleFromComponentsPayload<
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> CapsuleFromComponentsReturn<Self::Capsule>;

    fn master_scalar_components(
        &self,
        params: MasterScalarComponentsParams,
        payload: MasterScalarComponentsPayload<'_, Self::MasterScalar>,
    ) -> MasterScalarComponentsReturn<<Self::Pairing as IPairingAdapter>::Scalar>;

    fn master_scalar_from_components(
        &self,
        params: MasterScalarFromComponentsParams,
        payload: MasterScalarFromComponentsPayload<<Self::Pairing as IPairingAdapter>::Scalar>,
    ) -> MasterScalarFromComponentsReturn<Self::MasterScalar>;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KemConcrete {
    Bb1DepthOne,
}

pub trait IKemConsumer<P: IPairingAdapter> {
    type Output;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output;
}

pub struct ConsumeKemParams;

pub struct ConsumeKemPayload<K: ICredentialKemAdapter> {
    pub adapter: K,
    pub(super) scope: IdentityScope,
}

impl<K: ICredentialKemAdapter> ConsumeKemPayload<K> {
    pub fn scope(&self) -> IdentityScope {
        self.scope
    }
}

pub struct CreateKemDeps<'a, P: IPairingArithmetic, C> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub consumer: C,
}

pub struct CreateKemParams {
    pub concrete: KemConcrete,
    pub identifier: KemIdentifier,
    pub scope: IdentityScope,
}

pub struct CreateKemPayload;

pub struct CreateKemSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateKemErrorReturn {
    UnsupportedKemIdentifier,
    UnsupportedIdentityScope,
    Bb1DepthOne(Bb1DepthOneKemTryNewErrorReturn),
}

pub type CreateKemReturn<O> = Result<CreateKemSuccessReturn<O>, CreateKemErrorReturn>;

pub type CreateKemFn<'a, P, C> = fn(
    &CreateKemDeps<'a, P, C>,
    CreateKemParams,
    CreateKemPayload,
) -> CreateKemReturn<<C as IKemConsumer<P>>::Output>;
