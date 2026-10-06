#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    CapsuleComponents, CapsuleComponentsParams, CapsuleComponentsPayload,
    CapsuleComponentsSuccessReturn, CapsuleFromComponentsParams, CapsuleFromComponentsPayload,
    CapsuleFromComponentsSuccessReturn, ConsumeKemParams, ConsumeKemPayload, CreateKemDeps,
    CreateKemParams, CreateKemPayload, CreateKemReturn, CreateKemSuccessReturn,
    CredentialComponents, CredentialComponentsParams, CredentialComponentsPayload,
    CredentialComponentsSuccessReturn, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, CredentialFromComponentsSuccessReturn, DecapsulateParams,
    DecapsulatePayload, DecapsulateSuccessReturn, DeriveIdentityParams, DeriveIdentityPayload,
    DeriveIdentitySuccessReturn, EncapsulateParams, EncapsulatePayload, EncapsulateSuccessReturn,
    EncapsulatedValue, ICredentialKemAdapter, IKemConsumer, IdentityElementComponents,
    IdentityElementComponentsParams, IdentityElementComponentsPayload,
    IdentityElementComponentsSuccessReturn, IdentityScope, IsValidParams, IsValidPayload,
    IsValidSuccessReturn, IsWellFormedParams, IsWellFormedPayload, IsWellFormedSuccessReturn,
    IssueParams, IssuePayload, IssueSuccessReturn, KemConcrete, KemDeclaration, KemIdentifier,
    MasterScalarComponents, MasterScalarComponentsParams, MasterScalarComponentsPayload,
    MasterScalarComponentsSuccessReturn, MasterScalarFromComponentsParams,
    MasterScalarFromComponentsPayload, MasterScalarFromComponentsSuccessReturn,
    ParameterSetComponents, ParameterSetComponentsParams, ParameterSetComponentsPayload,
    ParameterSetComponentsSuccessReturn, ParameterSetFromComponentsParams,
    ParameterSetFromComponentsPayload, ParameterSetFromComponentsSuccessReturn,
    ParameterSetScopeComponents, RerandomizeParams, RerandomizePayload, RerandomizeSuccessReturn,
    SetupParams, SetupPayload, SetupScope, SetupSuccessReturn,
};
use core::marker::PhantomData;
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    ISampleUniformScalar, SampleUniformScalarParams, SampleUniformScalarPayload,
};
use zeroize::Zeroize;

use crate::bb1_depth_one::provides::BB1_DEPTH_ONE_DECLARATION;

#[derive(Default)]
pub struct KemDeclarationOverrides {
    pub identifier: Option<KemIdentifier>,
    pub identity_scopes: Option<&'static [IdentityScope]>,
    pub identity_tag: Option<&'static [u8]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_kem_declaration(overrides: KemDeclarationOverrides) -> KemDeclaration {
    KemDeclaration {
        identifier: overrides
            .identifier
            .unwrap_or(BB1_DEPTH_ONE_DECLARATION.identifier),
        identity_scopes: overrides
            .identity_scopes
            .unwrap_or(BB1_DEPTH_ONE_DECLARATION.identity_scopes),
        identity_tag: overrides
            .identity_tag
            .unwrap_or(BB1_DEPTH_ONE_DECLARATION.identity_tag),
        adapter_version: overrides
            .adapter_version
            .unwrap_or(BB1_DEPTH_ONE_DECLARATION.adapter_version),
        interface_version: overrides
            .interface_version
            .unwrap_or(BB1_DEPTH_ONE_DECLARATION.interface_version),
    }
}

#[derive(Default)]
pub struct EncapsulatedValueOverrides {
    pub bytes: Option<Secret<Vec<u8>>>,
}

pub fn build_encapsulated_value(overrides: EncapsulatedValueOverrides) -> EncapsulatedValue {
    EncapsulatedValue {
        bytes: overrides.bytes.unwrap_or_else(|| {
            let mut bytes = vec![0u8; 384];
            bytes[31] = 1;
            build_secret(SecretConstructorParamsOverrides { value: Some(bytes) })
        }),
    }
}

#[derive(Default)]
pub struct SetupParamsOverrides<'a> {
    pub scope: Option<SetupScope<'a>>,
}

pub fn build_setup_params<'a>(overrides: SetupParamsOverrides<'a>) -> SetupParams<'a> {
    SetupParams {
        scope: overrides.scope.unwrap_or(SetupScope::Entitlement),
    }
}

#[derive(Default)]
pub struct SetupPayloadOverrides {
    pub master_uniform: Option<Secret<Vec<u8>>>,
    pub u0_uniform: Option<Secret<Vec<u8>>>,
    pub u1_uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_setup_payload(overrides: SetupPayloadOverrides) -> SetupPayload {
    SetupPayload {
        master_uniform: overrides.master_uniform.unwrap_or_else(|| {
            build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x11; 64]),
            })
        }),
        u0_uniform: overrides.u0_uniform.unwrap_or_else(|| {
            build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x22; 64]),
            })
        }),
        u1_uniform: overrides.u1_uniform.unwrap_or_else(|| {
            build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x33; 64]),
            })
        }),
    }
}

fn sample_scalar_secret<P: IPairingAdapter>(fill: u8) -> Secret<P::Scalar> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![fill; P::Scalar::UNIFORM_BYTES_LENGTH]),
            }),
        },
    )
    .unwrap()
    .scalar
}

fn sample_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    sample_scalar_secret::<P>(fill).expose().clone()
}

fn g1_element<P: IPairingAdapter>(pairing: &P, additions: usize) -> P::G1 {
    let generator = pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
        .unwrap()
        .point;
    let mut element = generator.clone();
    for _ in 0..additions {
        element = pairing
            .add_g1(
                AddG1Params,
                AddG1Payload {
                    left: element,
                    right: generator.clone(),
                },
            )
            .unwrap()
            .sum;
    }
    element
}

fn g2_element<P: IPairingAdapter>(pairing: &P, additions: usize) -> P::G2 {
    let generator = pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
        .unwrap()
        .point;
    let mut element = generator.clone();
    for _ in 0..additions {
        element = pairing
            .add_g2(
                AddG2Params,
                AddG2Payload {
                    left: element,
                    right: generator.clone(),
                },
            )
            .unwrap()
            .sum;
    }
    element
}

#[derive(Default)]
pub struct SetupSuccessReturnOverrides<PS, M> {
    pub parameter_set: Option<PS>,
    pub master_scalar: Option<M>,
}

pub fn build_setup_success_return<PS: Default, M: Default>(
    overrides: SetupSuccessReturnOverrides<PS, M>,
) -> SetupSuccessReturn<PS, M> {
    SetupSuccessReturn {
        parameter_set: overrides.parameter_set.unwrap_or_default(),
        master_scalar: overrides.master_scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct DeriveIdentitySuccessReturnOverrides<IE> {
    pub identity_element: Option<IE>,
}

pub fn build_derive_identity_success_return<IE: Default>(
    overrides: DeriveIdentitySuccessReturnOverrides<IE>,
) -> DeriveIdentitySuccessReturn<IE> {
    DeriveIdentitySuccessReturn {
        identity_element: overrides.identity_element.unwrap_or_default(),
    }
}

pub struct IssueSuccessReturnOverrides<C, S: Zeroize> {
    pub credential: Option<C>,
    pub randomness: Option<Secret<S>>,
}

impl<C, S: Zeroize> Default for IssueSuccessReturnOverrides<C, S> {
    fn default() -> Self {
        Self {
            credential: None,
            randomness: None,
        }
    }
}

pub fn build_issue_success_return<P: IPairingAdapter, C: Default>(
    _pairing: &P,
    overrides: IssueSuccessReturnOverrides<C, P::Scalar>,
) -> IssueSuccessReturn<C, P::Scalar> {
    IssueSuccessReturn {
        credential: overrides.credential.unwrap_or_default(),
        randomness: overrides
            .randomness
            .unwrap_or_else(|| sample_scalar_secret::<P>(0x41)),
    }
}

pub struct RerandomizeSuccessReturnOverrides<C, S: Zeroize> {
    pub credential: Option<C>,
    pub offset: Option<Secret<S>>,
}

impl<C, S: Zeroize> Default for RerandomizeSuccessReturnOverrides<C, S> {
    fn default() -> Self {
        Self {
            credential: None,
            offset: None,
        }
    }
}

pub fn build_rerandomize_success_return<P: IPairingAdapter, C: Default>(
    _pairing: &P,
    overrides: RerandomizeSuccessReturnOverrides<C, P::Scalar>,
) -> RerandomizeSuccessReturn<C, P::Scalar> {
    RerandomizeSuccessReturn {
        credential: overrides.credential.unwrap_or_default(),
        offset: overrides
            .offset
            .unwrap_or_else(|| sample_scalar_secret::<P>(0x42)),
    }
}

#[derive(Default)]
pub struct EncapsulateSuccessReturnOverrides<CA> {
    pub capsule: Option<CA>,
    pub encapsulated: Option<EncapsulatedValue>,
}

pub fn build_encapsulate_success_return<CA: Default>(
    overrides: EncapsulateSuccessReturnOverrides<CA>,
) -> EncapsulateSuccessReturn<CA> {
    EncapsulateSuccessReturn {
        capsule: overrides.capsule.unwrap_or_default(),
        encapsulated: overrides
            .encapsulated
            .unwrap_or_else(|| build_encapsulated_value(Default::default())),
    }
}

pub struct CredentialComponentsOverrides<G1, G2> {
    pub a: Option<G1>,
    pub b: Option<G2>,
}

impl<G1, G2> Default for CredentialComponentsOverrides<G1, G2> {
    fn default() -> Self {
        Self { a: None, b: None }
    }
}

pub fn build_credential_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: CredentialComponentsOverrides<P::G1, P::G2>,
) -> CredentialComponents<P::G1, P::G2> {
    CredentialComponents {
        a: overrides.a.unwrap_or_else(|| g1_element(pairing, 0)),
        b: overrides.b.unwrap_or_else(|| g2_element(pairing, 0)),
    }
}

pub struct ParameterSetComponentsOverrides<G1, G2> {
    pub g1: Option<G1>,
    pub u0: Option<G1>,
    pub u1: Option<G1>,
    pub g2: Option<G2>,
    pub hpub: Option<G2>,
    pub scope: Option<ParameterSetScopeComponents<G1>>,
}

impl<G1, G2> Default for ParameterSetComponentsOverrides<G1, G2> {
    fn default() -> Self {
        Self {
            g1: None,
            u0: None,
            u1: None,
            g2: None,
            hpub: None,
            scope: None,
        }
    }
}

pub fn build_parameter_set_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: ParameterSetComponentsOverrides<P::G1, P::G2>,
) -> ParameterSetComponents<P::G1, P::G2> {
    ParameterSetComponents {
        g1: overrides.g1.unwrap_or_else(|| g1_element(pairing, 0)),
        u0: overrides.u0.unwrap_or_else(|| g1_element(pairing, 1)),
        u1: overrides.u1.unwrap_or_else(|| g1_element(pairing, 2)),
        g2: overrides.g2.unwrap_or_else(|| g2_element(pairing, 0)),
        hpub: overrides.hpub.unwrap_or_else(|| g2_element(pairing, 1)),
        scope: overrides
            .scope
            .unwrap_or(ParameterSetScopeComponents::Entitlement),
    }
}

pub struct IdentityElementComponentsOverrides<S, G1> {
    pub scalar: Option<S>,
    pub element: Option<G1>,
}

impl<S, G1> Default for IdentityElementComponentsOverrides<S, G1> {
    fn default() -> Self {
        Self {
            scalar: None,
            element: None,
        }
    }
}

pub fn build_identity_element_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: IdentityElementComponentsOverrides<P::Scalar, P::G1>,
) -> IdentityElementComponents<P::Scalar, P::G1> {
    IdentityElementComponents {
        scalar: overrides.scalar.unwrap_or_else(|| sample_scalar::<P>(0x43)),
        element: overrides.element.unwrap_or_else(|| g1_element(pairing, 0)),
    }
}

pub struct MasterScalarComponentsOverrides<S: Zeroize> {
    pub value: Option<Secret<S>>,
}

impl<S: Zeroize> Default for MasterScalarComponentsOverrides<S> {
    fn default() -> Self {
        Self { value: None }
    }
}

pub fn build_master_scalar_components<P: IPairingAdapter>(
    _pairing: &P,
    overrides: MasterScalarComponentsOverrides<P::Scalar>,
) -> MasterScalarComponents<P::Scalar> {
    MasterScalarComponents {
        value: overrides
            .value
            .unwrap_or_else(|| sample_scalar_secret::<P>(0x44)),
    }
}

pub struct CredentialComponentsSuccessReturnOverrides<G1, G2> {
    pub components: Option<CredentialComponents<G1, G2>>,
}

impl<G1, G2> Default for CredentialComponentsSuccessReturnOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_credential_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: CredentialComponentsSuccessReturnOverrides<P::G1, P::G2>,
) -> CredentialComponentsSuccessReturn<P::G1, P::G2> {
    CredentialComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_credential_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct CredentialFromComponentsSuccessReturnOverrides<C> {
    pub credential: Option<C>,
}

pub fn build_credential_from_components_success_return<C: Default>(
    overrides: CredentialFromComponentsSuccessReturnOverrides<C>,
) -> CredentialFromComponentsSuccessReturn<C> {
    CredentialFromComponentsSuccessReturn {
        credential: overrides.credential.unwrap_or_default(),
    }
}

pub struct ParameterSetComponentsSuccessReturnOverrides<G1, G2> {
    pub components: Option<ParameterSetComponents<G1, G2>>,
}

impl<G1, G2> Default for ParameterSetComponentsSuccessReturnOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_parameter_set_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: ParameterSetComponentsSuccessReturnOverrides<P::G1, P::G2>,
) -> ParameterSetComponentsSuccessReturn<P::G1, P::G2> {
    ParameterSetComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_parameter_set_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct ParameterSetFromComponentsSuccessReturnOverrides<PS> {
    pub parameter_set: Option<PS>,
}

pub fn build_parameter_set_from_components_success_return<PS: Default>(
    overrides: ParameterSetFromComponentsSuccessReturnOverrides<PS>,
) -> ParameterSetFromComponentsSuccessReturn<PS> {
    ParameterSetFromComponentsSuccessReturn {
        parameter_set: overrides.parameter_set.unwrap_or_default(),
    }
}

pub struct IdentityElementComponentsSuccessReturnOverrides<S, G1> {
    pub components: Option<IdentityElementComponents<S, G1>>,
}

impl<S, G1> Default for IdentityElementComponentsSuccessReturnOverrides<S, G1> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_identity_element_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: IdentityElementComponentsSuccessReturnOverrides<P::Scalar, P::G1>,
) -> IdentityElementComponentsSuccessReturn<P::Scalar, P::G1> {
    IdentityElementComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_identity_element_components(pairing, Default::default())),
    }
}

pub struct CapsuleComponentsSuccessReturnOverrides<G1, G2> {
    pub components: Option<CapsuleComponents<G1, G2>>,
}

impl<G1, G2> Default for CapsuleComponentsSuccessReturnOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_capsule_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: CapsuleComponentsSuccessReturnOverrides<P::G1, P::G2>,
) -> CapsuleComponentsSuccessReturn<P::G1, P::G2> {
    CapsuleComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| CapsuleComponents::Entitlement {
                u: g2_element(pairing, 0),
                v: g1_element(pairing, 0),
                w: g1_element(pairing, 1),
            }),
    }
}

#[derive(Default)]
pub struct CapsuleFromComponentsSuccessReturnOverrides<CA> {
    pub capsule: Option<CA>,
}

pub fn build_capsule_from_components_success_return<CA: Default>(
    overrides: CapsuleFromComponentsSuccessReturnOverrides<CA>,
) -> CapsuleFromComponentsSuccessReturn<CA> {
    CapsuleFromComponentsSuccessReturn {
        capsule: overrides.capsule.unwrap_or_default(),
    }
}

pub struct MasterScalarComponentsSuccessReturnOverrides<S: Zeroize> {
    pub components: Option<MasterScalarComponents<S>>,
}

impl<S: Zeroize> Default for MasterScalarComponentsSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_master_scalar_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: MasterScalarComponentsSuccessReturnOverrides<P::Scalar>,
) -> MasterScalarComponentsSuccessReturn<P::Scalar> {
    MasterScalarComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_master_scalar_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct MasterScalarFromComponentsSuccessReturnOverrides<M> {
    pub master_scalar: Option<M>,
}

pub fn build_master_scalar_from_components_success_return<M: Default>(
    overrides: MasterScalarFromComponentsSuccessReturnOverrides<M>,
) -> MasterScalarFromComponentsSuccessReturn<M> {
    MasterScalarFromComponentsSuccessReturn {
        master_scalar: overrides.master_scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct IsValidSuccessReturnOverrides {
    pub is_valid: Option<bool>,
}

pub fn build_is_valid_success_return(
    overrides: IsValidSuccessReturnOverrides,
) -> IsValidSuccessReturn {
    IsValidSuccessReturn {
        is_valid: overrides.is_valid.unwrap_or(false),
    }
}

#[derive(Default)]
pub struct IsWellFormedSuccessReturnOverrides {
    pub is_well_formed: Option<bool>,
}

pub fn build_is_well_formed_success_return(
    overrides: IsWellFormedSuccessReturnOverrides,
) -> IsWellFormedSuccessReturn {
    IsWellFormedSuccessReturn {
        is_well_formed: overrides.is_well_formed.unwrap_or(false),
    }
}

#[derive(Default)]
pub struct DecapsulateSuccessReturnOverrides {
    pub encapsulated: Option<EncapsulatedValue>,
}

pub fn build_decapsulate_success_return(
    overrides: DecapsulateSuccessReturnOverrides,
) -> DecapsulateSuccessReturn {
    DecapsulateSuccessReturn {
        encapsulated: overrides
            .encapsulated
            .unwrap_or_else(|| build_encapsulated_value(Default::default())),
    }
}

pub struct MockICredentialKemAdapter<'a, P, PS, M, IE, C, CA> {
    pub pairing: &'a P,
    pub parameter_set: PhantomData<PS>,
    pub master_scalar: PhantomData<M>,
    pub identity_element: PhantomData<IE>,
    pub credential: PhantomData<C>,
    pub capsule: PhantomData<CA>,
}

impl<'a, P, PS, M, IE, C, CA> ICredentialKemAdapter
    for MockICredentialKemAdapter<'a, P, PS, M, IE, C, CA>
where
    P: IPairingAdapter,
    PS: Default,
    M: Default,
    IE: Default,
    C: Default,
    CA: Default,
{
    const DECLARATION: KemDeclaration = BB1_DEPTH_ONE_DECLARATION;

    type Pairing = P;
    type ParameterSet = PS;
    type MasterScalar = M;
    type IdentityElement = IE;
    type Credential = C;
    type Capsule = CA;

    fn setup(
        &self,
        _params: SetupParams<'_>,
        _payload: SetupPayload,
    ) -> super::interface::SetupReturn<Self::ParameterSet, Self::MasterScalar> {
        Ok(build_setup_success_return(Default::default()))
    }

    fn derive_identity(
        &self,
        _params: DeriveIdentityParams,
        _payload: DeriveIdentityPayload<'_, Self::ParameterSet>,
    ) -> super::interface::DeriveIdentityReturn<Self::IdentityElement> {
        Ok(build_derive_identity_success_return(Default::default()))
    }

    fn issue(
        &self,
        _params: IssueParams,
        _payload: IssuePayload<'_, Self::ParameterSet, Self::MasterScalar, Self::IdentityElement>,
    ) -> super::interface::IssueReturn<Self::Credential, P::Scalar> {
        Ok(build_issue_success_return(self.pairing, Default::default()))
    }

    fn rerandomize(
        &self,
        _params: RerandomizeParams,
        _payload: RerandomizePayload<
            '_,
            Self::ParameterSet,
            Self::IdentityElement,
            Self::Credential,
        >,
    ) -> super::interface::RerandomizeReturn<Self::Credential, P::Scalar> {
        Ok(build_rerandomize_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn is_valid(
        &self,
        _params: IsValidParams,
        _payload: IsValidPayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>,
    ) -> super::interface::IsValidReturn {
        Ok(build_is_valid_success_return(Default::default()))
    }

    fn encapsulate(
        &self,
        _params: EncapsulateParams,
        _payload: EncapsulatePayload<'_, Self::ParameterSet>,
    ) -> super::interface::EncapsulateReturn<Self::Capsule> {
        Ok(build_encapsulate_success_return(Default::default()))
    }

    fn is_well_formed(
        &self,
        _params: IsWellFormedParams,
        _payload: IsWellFormedPayload<'_, Self::ParameterSet, Self::Capsule>,
    ) -> super::interface::IsWellFormedReturn {
        Ok(build_is_well_formed_success_return(Default::default()))
    }

    fn decapsulate(
        &self,
        _params: DecapsulateParams,
        _payload: DecapsulatePayload<'_, Self::IdentityElement, Self::Credential, Self::Capsule>,
    ) -> super::interface::DecapsulateReturn {
        Ok(build_decapsulate_success_return(Default::default()))
    }

    fn credential_components(
        &self,
        _params: CredentialComponentsParams,
        _payload: CredentialComponentsPayload<'_, Self::Credential>,
    ) -> super::interface::CredentialComponentsReturn<P::G1, P::G2> {
        Ok(build_credential_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn credential_from_components(
        &self,
        _params: CredentialFromComponentsParams,
        _payload: CredentialFromComponentsPayload<P::G1, P::G2>,
    ) -> super::interface::CredentialFromComponentsReturn<Self::Credential> {
        Ok(build_credential_from_components_success_return(
            Default::default(),
        ))
    }

    fn parameter_set_components(
        &self,
        _params: ParameterSetComponentsParams,
        _payload: ParameterSetComponentsPayload<'_, Self::ParameterSet>,
    ) -> super::interface::ParameterSetComponentsReturn<P::G1, P::G2> {
        Ok(build_parameter_set_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn parameter_set_from_components(
        &self,
        _params: ParameterSetFromComponentsParams,
        _payload: ParameterSetFromComponentsPayload<P::G1, P::G2>,
    ) -> super::interface::ParameterSetFromComponentsReturn<Self::ParameterSet> {
        Ok(build_parameter_set_from_components_success_return(
            Default::default(),
        ))
    }

    fn identity_element_components(
        &self,
        _params: IdentityElementComponentsParams,
        _payload: IdentityElementComponentsPayload<'_, Self::IdentityElement>,
    ) -> super::interface::IdentityElementComponentsReturn<P::Scalar, P::G1> {
        Ok(build_identity_element_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn capsule_components(
        &self,
        _params: CapsuleComponentsParams,
        _payload: CapsuleComponentsPayload<'_, Self::Capsule>,
    ) -> super::interface::CapsuleComponentsReturn<P::G1, P::G2> {
        Ok(build_capsule_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn capsule_from_components(
        &self,
        _params: CapsuleFromComponentsParams,
        _payload: CapsuleFromComponentsPayload<P::G1, P::G2>,
    ) -> super::interface::CapsuleFromComponentsReturn<Self::Capsule> {
        Ok(build_capsule_from_components_success_return(
            Default::default(),
        ))
    }

    fn master_scalar_components(
        &self,
        _params: MasterScalarComponentsParams,
        _payload: MasterScalarComponentsPayload<'_, Self::MasterScalar>,
    ) -> super::interface::MasterScalarComponentsReturn<P::Scalar> {
        Ok(build_master_scalar_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn master_scalar_from_components(
        &self,
        _params: MasterScalarFromComponentsParams,
        _payload: MasterScalarFromComponentsPayload<P::Scalar>,
    ) -> super::interface::MasterScalarFromComponentsReturn<Self::MasterScalar> {
        Ok(build_master_scalar_from_components_success_return(
            Default::default(),
        ))
    }
}

#[derive(Default)]
pub struct CreateKemParamsOverrides {
    pub concrete: Option<KemConcrete>,
    pub identifier: Option<KemIdentifier>,
    pub scope: Option<IdentityScope>,
}

pub fn build_create_kem_params(overrides: CreateKemParamsOverrides) -> CreateKemParams {
    CreateKemParams {
        concrete: overrides.concrete.unwrap_or(KemConcrete::Bb1DepthOne),
        identifier: overrides.identifier.unwrap_or(KemIdentifier::Bb1DepthOneV1),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

#[derive(Default)]
pub struct CreateKemSuccessReturnOverrides<O> {
    pub output: Option<O>,
}

pub fn build_create_kem_success_return<O: Default>(
    overrides: CreateKemSuccessReturnOverrides<O>,
) -> CreateKemSuccessReturn<O> {
    CreateKemSuccessReturn {
        output: overrides.output.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ConsumeKemPayloadOverrides<K> {
    pub adapter: Option<K>,
    pub scope: Option<IdentityScope>,
}

pub fn build_consume_kem_payload<K: ICredentialKemAdapter + Default>(
    overrides: ConsumeKemPayloadOverrides<K>,
) -> ConsumeKemPayload<K> {
    ConsumeKemPayload {
        adapter: overrides.adapter.unwrap_or_default(),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

pub struct MockIKemConsumer;

impl<P: IPairingAdapter> IKemConsumer<P> for MockIKemConsumer {
    type Output = ();

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        _payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
    }
}

pub fn mock_create_kem<'a, P, C>(
    _deps: &CreateKemDeps<'a, P, C>,
    _params: CreateKemParams,
    _payload: CreateKemPayload,
) -> CreateKemReturn<C::Output>
where
    P: IPairingArithmetic,
    C: IKemConsumer<P>,
    C::Output: Default,
{
    Ok(build_create_kem_success_return(Default::default()))
}
