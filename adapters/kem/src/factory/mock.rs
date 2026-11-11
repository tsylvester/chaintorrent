#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    CapsuleComponents, CapsuleComponentsParams, CapsuleComponentsPayload, CapsuleComponentsReturn,
    CapsuleComponentsSuccessReturn, CapsuleFromComponentsParams, CapsuleFromComponentsPayload,
    CapsuleFromComponentsReturn, CapsuleFromComponentsSuccessReturn, ConsumeKemParams,
    ConsumeKemPayload, CreateKemCaseDeps, CreateKemCaseErrorReturn, CreateKemCaseParams,
    CreateKemCasePayload, CreateKemCaseReturn, CreateKemCaseSuccessReturn, CreateKemDeps,
    CreateKemFn, CreateKemParams, CreateKemPayload, CreateKemReturn, CreateKemSuccessReturn,
    CredentialComponents, CredentialComponentsParams, CredentialComponentsPayload,
    CredentialComponentsReturn, CredentialComponentsSuccessReturn, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, CredentialFromComponentsReturn,
    CredentialFromComponentsSuccessReturn, DecapsulateParams, DecapsulatePayload,
    DecapsulateReturn, DecapsulateSuccessReturn, DeriveIdentityErrorReturn, DeriveIdentityParams,
    DeriveIdentityPayload, DeriveIdentityReturn, DeriveIdentitySuccessReturn,
    EncapsulateErrorReturn, EncapsulateParams, EncapsulatePayload, EncapsulateReturn,
    EncapsulateSuccessReturn, EncapsulatedValue, ICreateKemCaseConsumer, ICredentialKemAdapter,
    IKemConsumer, IdentityElementComponents, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, IdentityElementComponentsReturn,
    IdentityElementComponentsSuccessReturn, IdentityScope, IsValidParams, IsValidPayload,
    IsValidReturn, IsValidSuccessReturn, IsWellFormedParams, IsWellFormedPayload,
    IsWellFormedReturn, IsWellFormedSuccessReturn, IssueErrorReturn, IssueParams, IssuePayload,
    IssueReturn, IssueSuccessReturn, KEM_INTERFACE_VERSION, KemConcrete, KemDeclaration,
    KemIdentifier, KemIdentity, MasterScalarComponents, MasterScalarComponentsParams,
    MasterScalarComponentsPayload, MasterScalarComponentsReturn,
    MasterScalarComponentsSuccessReturn, MasterScalarFromComponentsParams,
    MasterScalarFromComponentsPayload, MasterScalarFromComponentsReturn,
    MasterScalarFromComponentsSuccessReturn, ParameterSetComponents, ParameterSetComponentsParams,
    ParameterSetComponentsPayload, ParameterSetComponentsReturn,
    ParameterSetComponentsSuccessReturn, ParameterSetFromComponentsParams,
    ParameterSetFromComponentsPayload, ParameterSetFromComponentsReturn,
    ParameterSetFromComponentsSuccessReturn, ParameterSetScopeComponents, RerandomizeErrorReturn,
    RerandomizeParams, RerandomizePayload, RerandomizeReturn, RerandomizeSuccessReturn,
    RunCreateKemCaseFn, SetupErrorReturn, SetupParams, SetupPayload, SetupReturn, SetupScope,
    SetupSuccessReturn,
};
use core::convert::Infallible;
use domain::{Secret, SecretConstructorParams, build_secret};
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, DomainTag,
    DomainTagConstructorParamsOverrides, HashToScalarErrorReturn, HashToScalarParams,
    HashToScalarPayload, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    create_hash_to_scalar,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingParams, CreatePairingPayload, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeGtParams, EncodeGtPayload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer,
    IPairingReference, ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1Payload, MsmG1Params,
    MsmG1Payload, MsmG1Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, NegG1Params,
    NegG1Payload, PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductParams,
    PairingProductPayload, PairingProductTerm, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use serde_json::{Map, Value};
use zeroize::Zeroize;

pub(crate) const MOCK_I_CREDENTIAL_KEM_ADAPTER_IDENTITY_TAG: &[u8] =
    b"ChainTorrent-v1-kem-identity-mock";

pub(crate) const MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION: KemDeclaration = KemDeclaration {
    identifier: KemIdentifier::Mock,
    identity_scopes: &[IdentityScope::Entitlement, IdentityScope::Asset],
    identity_tag: MOCK_I_CREDENTIAL_KEM_ADAPTER_IDENTITY_TAG,
    adapter_version: 1,
    interface_version: KEM_INTERFACE_VERSION,
};

pub struct MockICredentialKemAdapterParameterSet<P: IPairingAdapter> {
    pub(crate) g1: P::G1,
    pub(crate) u0: P::G1,
    pub(crate) u1: P::G1,
    pub(crate) g2: P::G2,
    pub(crate) hpub: P::G2,
    pub(crate) scope: ParameterSetScopeComponents<P::G1>,
}

pub struct MockICredentialKemAdapterMasterScalar<P: IPairingAdapter> {
    pub(crate) value: Secret<P::Scalar>,
}

pub struct MockICredentialKemAdapterIdentityElement<P: IPairingAdapter> {
    pub(crate) scalar: P::Scalar,
    pub(crate) element: P::G1,
}

pub struct MockICredentialKemAdapterCredential<P: IPairingAdapter> {
    pub(crate) a: P::G1,
    pub(crate) b: P::G2,
}

pub enum MockICredentialKemAdapterCapsule<P: IPairingAdapter> {
    Entitlement { u: P::G2, v: P::G1, w: P::G1 },
    Asset { u: P::G2, v: P::G1 },
}

pub struct MockICredentialKemAdapter<'a, P: IPairingArithmetic> {
    pub(crate) pairing: &'a P,
    pub(crate) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(crate) tag: DomainTag,
}

pub struct MockICredentialKemAdapterConstructorParams<'a, P: IPairingArithmetic> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub tag: DomainTag,
}

enum MapIdentityFailure {
    HashToScalar(HashToScalarErrorReturn),
    TrivialIdentityElement,
}

impl<'a, P: IPairingArithmetic> MockICredentialKemAdapter<'a, P> {
    pub fn try_new(
        params: MockICredentialKemAdapterConstructorParams<'a, P>,
    ) -> Result<MockICredentialKemAdapter<'a, P>, Infallible> {
        Ok(MockICredentialKemAdapter {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            tag: params.tag,
        })
    }

    fn map_identity(
        &self,
        identity: &KemIdentity<'_>,
        u0: &P::G1,
        u1: &P::G1,
    ) -> Result<(P::Scalar, P::G1), MapIdentityFailure> {
        let message: &[u8] = match identity {
            KemIdentity::Entitlement { canonical } => canonical,
            KemIdentity::Asset { root } => root.as_bytes(),
        };
        let scalar = self
            .hash_to_scalar
            .hash_to_scalar(
                HashToScalarParams { tag: &self.tag },
                HashToScalarPayload { message },
            )
            .map_err(MapIdentityFailure::HashToScalar)?
            .scalar;
        let Ok(scaled) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: u1.clone(),
                scalar: scalar.clone(),
            },
        );
        let Ok(element) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: u0.clone(),
                right: scaled.product,
            },
        );
        let Ok(identity_check) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: element.sum.clone(),
            },
        );
        if identity_check.is_identity {
            return Err(MapIdentityFailure::TrivialIdentityElement);
        }
        Ok((scalar, element.sum))
    }
}

impl<'a, P: IPairingArithmetic> ICredentialKemAdapter for MockICredentialKemAdapter<'a, P> {
    const DECLARATION: KemDeclaration = MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION;

    type Pairing = P;
    type ParameterSet = MockICredentialKemAdapterParameterSet<P>;
    type MasterScalar = MockICredentialKemAdapterMasterScalar<P>;
    type IdentityElement = MockICredentialKemAdapterIdentityElement<P>;
    type Credential = MockICredentialKemAdapterCredential<P>;
    type Capsule = MockICredentialKemAdapterCapsule<P>;

    fn setup(
        &self,
        params: SetupParams<'_>,
        payload: SetupPayload,
    ) -> SetupReturn<Self::ParameterSet, Self::MasterScalar> {
        let master_scalar = sample_scalar::<P::Scalar>(payload.master_uniform)
            .map_err(SetupErrorReturn::MasterScalarSampling)?;
        let u0_scalar =
            sample_scalar::<P::Scalar>(payload.u0_uniform).map_err(SetupErrorReturn::U0Sampling)?;
        let u1_scalar =
            sample_scalar::<P::Scalar>(payload.u1_uniform).map_err(SetupErrorReturn::U1Sampling)?;
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(u0) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: u0_scalar.expose().clone(),
            },
        );
        let Ok(u1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: u1_scalar.expose().clone(),
            },
        );
        let Ok(hpub) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: master_scalar.expose().clone(),
            },
        );
        let scope = match params.scope {
            SetupScope::Entitlement => ParameterSetScopeComponents::Entitlement,
            SetupScope::Asset { root } => {
                match self.map_identity(&KemIdentity::Asset { root }, &u0.product, &u1.product) {
                    Ok((_, element)) => ParameterSetScopeComponents::Asset {
                        identity_element: element,
                    },
                    Err(MapIdentityFailure::HashToScalar(error)) => {
                        return Err(SetupErrorReturn::HashToScalar(error));
                    }
                    Err(MapIdentityFailure::TrivialIdentityElement) => {
                        return Err(SetupErrorReturn::TrivialIdentityElement);
                    }
                }
            }
        };
        Ok(SetupSuccessReturn {
            parameter_set: MockICredentialKemAdapterParameterSet {
                g1: g1.point,
                u0: u0.product,
                u1: u1.product,
                g2: g2.point,
                hpub: hpub.product,
                scope,
            },
            master_scalar: MockICredentialKemAdapterMasterScalar {
                value: master_scalar,
            },
        })
    }

    fn derive_identity(
        &self,
        _params: DeriveIdentityParams,
        payload: DeriveIdentityPayload<'_, Self::ParameterSet>,
    ) -> DeriveIdentityReturn<Self::IdentityElement> {
        let parameter_set = payload.parameter_set;
        match (&parameter_set.scope, &payload.identity) {
            (ParameterSetScopeComponents::Entitlement, KemIdentity::Asset { .. })
            | (ParameterSetScopeComponents::Asset { .. }, KemIdentity::Entitlement { .. }) => {
                return Err(DeriveIdentityErrorReturn::WrongIdentityScope);
            }
            _ => {}
        }
        let (scalar, element) =
            match self.map_identity(&payload.identity, &parameter_set.u0, &parameter_set.u1) {
                Ok(mapped) => mapped,
                Err(MapIdentityFailure::HashToScalar(error)) => {
                    return Err(DeriveIdentityErrorReturn::HashToScalar(error));
                }
                Err(MapIdentityFailure::TrivialIdentityElement) => {
                    return Err(DeriveIdentityErrorReturn::TrivialIdentityElement);
                }
            };
        if let ParameterSetScopeComponents::Asset { identity_element } = &parameter_set.scope {
            let Ok(derived) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: element.clone(),
                },
            );
            let Ok(fixed) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: identity_element.clone(),
                },
            );
            if derived.bytes != fixed.bytes {
                return Err(DeriveIdentityErrorReturn::OutsideAssetScope);
            }
        }
        Ok(DeriveIdentitySuccessReturn {
            identity_element: MockICredentialKemAdapterIdentityElement { scalar, element },
        })
    }

    fn issue(
        &self,
        _params: IssueParams,
        payload: IssuePayload<'_, Self::ParameterSet, Self::MasterScalar, Self::IdentityElement>,
    ) -> IssueReturn<Self::Credential, P::Scalar> {
        let randomness =
            sample_scalar::<P::Scalar>(payload.uniform).map_err(IssueErrorReturn::Sampling)?;
        let r = randomness.expose().clone();
        let Ok(a) = self.pairing.msm_g1(
            MsmG1Params,
            MsmG1Payload {
                terms: vec![
                    MsmG1Term {
                        base: payload.parameter_set.g1.clone(),
                        scalar: payload.master_scalar.value.expose().clone(),
                    },
                    MsmG1Term {
                        base: payload.identity_element.element.clone(),
                        scalar: r.clone(),
                    },
                ],
            },
        );
        let Ok(b) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: payload.parameter_set.g2.clone(),
                scalar: r,
            },
        );
        Ok(IssueSuccessReturn {
            credential: MockICredentialKemAdapterCredential {
                a: a.sum,
                b: b.product,
            },
            randomness,
        })
    }

    fn rerandomize(
        &self,
        _params: RerandomizeParams,
        payload: RerandomizePayload<
            '_,
            Self::ParameterSet,
            Self::IdentityElement,
            Self::Credential,
        >,
    ) -> RerandomizeReturn<Self::Credential, P::Scalar> {
        let offset = sample_scalar::<P::Scalar>(payload.uniform)
            .map_err(RerandomizeErrorReturn::Sampling)?;
        let s = offset.expose().clone();
        let Ok(scaled) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: payload.identity_element.element.clone(),
                scalar: s.clone(),
            },
        );
        let Ok(a) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: payload.credential.a.clone(),
                right: scaled.product,
            },
        );
        let Ok(scaled_base) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: payload.parameter_set.g2.clone(),
                scalar: s,
            },
        );
        let Ok(b) = self.pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: payload.credential.b.clone(),
                right: scaled_base.product,
            },
        );
        Ok(RerandomizeSuccessReturn {
            credential: MockICredentialKemAdapterCredential { a: a.sum, b: b.sum },
            offset,
        })
    }

    fn is_valid(
        &self,
        _params: IsValidParams,
        payload: IsValidPayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>,
    ) -> IsValidReturn {
        let parameter_set = payload.parameter_set;
        let identity_element = payload.identity_element;
        let credential = payload.credential;
        let Ok(neg_g1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: parameter_set.g1.clone(),
            },
        );
        let Ok(neg_element) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: identity_element.element.clone(),
            },
        );
        let Ok(check) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: credential.a.clone(),
                        g2: parameter_set.g2.clone(),
                    },
                    PairingProductTerm {
                        g1: neg_g1.negation,
                        g2: parameter_set.hpub.clone(),
                    },
                    PairingProductTerm {
                        g1: neg_element.negation,
                        g2: credential.b.clone(),
                    },
                ],
            },
        );
        Ok(IsValidSuccessReturn {
            is_valid: check.is_one,
        })
    }

    fn encapsulate(
        &self,
        _params: EncapsulateParams,
        payload: EncapsulatePayload<'_, Self::ParameterSet>,
    ) -> EncapsulateReturn<Self::Capsule> {
        let parameter_set = payload.parameter_set;
        let t = sample_scalar::<P::Scalar>(payload.uniform)
            .map_err(EncapsulateErrorReturn::Sampling)?
            .expose()
            .clone();
        let Ok(u) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: parameter_set.g2.clone(),
                scalar: t.clone(),
            },
        );
        let capsule = match &parameter_set.scope {
            ParameterSetScopeComponents::Entitlement => {
                let Ok(v) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: parameter_set.u0.clone(),
                        scalar: t.clone(),
                    },
                );
                let Ok(w) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: parameter_set.u1.clone(),
                        scalar: t.clone(),
                    },
                );
                MockICredentialKemAdapterCapsule::Entitlement {
                    u: u.product,
                    v: v.product,
                    w: w.product,
                }
            }
            ParameterSetScopeComponents::Asset { identity_element } => {
                let Ok(v) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: identity_element.clone(),
                        scalar: t.clone(),
                    },
                );
                MockICredentialKemAdapterCapsule::Asset {
                    u: u.product,
                    v: v.product,
                }
            }
        };
        let Ok(scaled_base) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: parameter_set.g1.clone(),
                scalar: t,
            },
        );
        let Ok(product) = self.pairing.pairing_product(
            PairingProductParams,
            PairingProductPayload {
                terms: vec![PairingProductTerm {
                    g1: scaled_base.product,
                    g2: parameter_set.hpub.clone(),
                }],
            },
        );
        let Ok(encoded) = self.pairing.encode_gt(
            EncodeGtParams,
            EncodeGtPayload {
                value: product.product,
            },
        );
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: encoded.bytes.expose().as_ref().to_vec(),
        });
        Ok(EncapsulateSuccessReturn {
            capsule,
            encapsulated: EncapsulatedValue { bytes },
        })
    }

    fn is_well_formed(
        &self,
        _params: IsWellFormedParams,
        payload: IsWellFormedPayload<'_, Self::ParameterSet, Self::Capsule>,
    ) -> IsWellFormedReturn {
        let parameter_set = payload.parameter_set;
        let is_well_formed = match (&parameter_set.scope, payload.capsule) {
            (
                ParameterSetScopeComponents::Entitlement,
                MockICredentialKemAdapterCapsule::Entitlement { u, v, w },
            ) => {
                let Ok(neg_u0) = self.pairing.neg_g1(
                    NegG1Params,
                    NegG1Payload {
                        point: parameter_set.u0.clone(),
                    },
                );
                let Ok(first) = self.pairing.pairing_product_is_one(
                    PairingProductIsOneParams,
                    PairingProductIsOnePayload {
                        terms: vec![
                            PairingProductTerm {
                                g1: v.clone(),
                                g2: parameter_set.g2.clone(),
                            },
                            PairingProductTerm {
                                g1: neg_u0.negation,
                                g2: u.clone(),
                            },
                        ],
                    },
                );
                let Ok(neg_u1) = self.pairing.neg_g1(
                    NegG1Params,
                    NegG1Payload {
                        point: parameter_set.u1.clone(),
                    },
                );
                let Ok(second) = self.pairing.pairing_product_is_one(
                    PairingProductIsOneParams,
                    PairingProductIsOnePayload {
                        terms: vec![
                            PairingProductTerm {
                                g1: w.clone(),
                                g2: parameter_set.g2.clone(),
                            },
                            PairingProductTerm {
                                g1: neg_u1.negation,
                                g2: u.clone(),
                            },
                        ],
                    },
                );
                first.is_one && second.is_one
            }
            (
                ParameterSetScopeComponents::Asset { identity_element },
                MockICredentialKemAdapterCapsule::Asset { u, v },
            ) => {
                let Ok(neg_element) = self.pairing.neg_g1(
                    NegG1Params,
                    NegG1Payload {
                        point: identity_element.clone(),
                    },
                );
                let Ok(check) = self.pairing.pairing_product_is_one(
                    PairingProductIsOneParams,
                    PairingProductIsOnePayload {
                        terms: vec![
                            PairingProductTerm {
                                g1: v.clone(),
                                g2: parameter_set.g2.clone(),
                            },
                            PairingProductTerm {
                                g1: neg_element.negation,
                                g2: u.clone(),
                            },
                        ],
                    },
                );
                check.is_one
            }
            _ => false,
        };
        Ok(IsWellFormedSuccessReturn { is_well_formed })
    }

    fn decapsulate(
        &self,
        _params: DecapsulateParams,
        payload: DecapsulatePayload<'_, Self::IdentityElement, Self::Credential, Self::Capsule>,
    ) -> DecapsulateReturn {
        let credential = payload.credential;
        let product_terms = match payload.capsule {
            MockICredentialKemAdapterCapsule::Entitlement { u, v, w } => {
                let Ok(scaled) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: w.clone(),
                        scalar: payload.identity_element.scalar.clone(),
                    },
                );
                let Ok(sum) = self.pairing.add_g1(
                    AddG1Params,
                    AddG1Payload {
                        left: v.clone(),
                        right: scaled.product,
                    },
                );
                let Ok(neg_sum) = self
                    .pairing
                    .neg_g1(NegG1Params, NegG1Payload { point: sum.sum });
                vec![
                    PairingProductTerm {
                        g1: credential.a.clone(),
                        g2: u.clone(),
                    },
                    PairingProductTerm {
                        g1: neg_sum.negation,
                        g2: credential.b.clone(),
                    },
                ]
            }
            MockICredentialKemAdapterCapsule::Asset { u, v } => {
                let Ok(neg_v) = self
                    .pairing
                    .neg_g1(NegG1Params, NegG1Payload { point: v.clone() });
                vec![
                    PairingProductTerm {
                        g1: credential.a.clone(),
                        g2: u.clone(),
                    },
                    PairingProductTerm {
                        g1: neg_v.negation,
                        g2: credential.b.clone(),
                    },
                ]
            }
        };
        let Ok(product) = self.pairing.pairing_product(
            PairingProductParams,
            PairingProductPayload {
                terms: product_terms,
            },
        );
        let Ok(encoded) = self.pairing.encode_gt(
            EncodeGtParams,
            EncodeGtPayload {
                value: product.product,
            },
        );
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: encoded.bytes.expose().as_ref().to_vec(),
        });
        Ok(DecapsulateSuccessReturn {
            encapsulated: EncapsulatedValue { bytes },
        })
    }

    fn credential_components(
        &self,
        _params: CredentialComponentsParams,
        payload: CredentialComponentsPayload<'_, Self::Credential>,
    ) -> CredentialComponentsReturn<P::G1, P::G2> {
        Ok(CredentialComponentsSuccessReturn {
            components: CredentialComponents {
                a: payload.credential.a.clone(),
                b: payload.credential.b.clone(),
            },
        })
    }

    fn credential_from_components(
        &self,
        _params: CredentialFromComponentsParams,
        payload: CredentialFromComponentsPayload<P::G1, P::G2>,
    ) -> CredentialFromComponentsReturn<Self::Credential> {
        Ok(CredentialFromComponentsSuccessReturn {
            credential: MockICredentialKemAdapterCredential {
                a: payload.components.a,
                b: payload.components.b,
            },
        })
    }

    fn parameter_set_components(
        &self,
        _params: ParameterSetComponentsParams,
        payload: ParameterSetComponentsPayload<'_, Self::ParameterSet>,
    ) -> ParameterSetComponentsReturn<P::G1, P::G2> {
        let parameter_set = payload.parameter_set;
        Ok(ParameterSetComponentsSuccessReturn {
            components: ParameterSetComponents {
                g1: parameter_set.g1.clone(),
                u0: parameter_set.u0.clone(),
                u1: parameter_set.u1.clone(),
                g2: parameter_set.g2.clone(),
                hpub: parameter_set.hpub.clone(),
                scope: match &parameter_set.scope {
                    ParameterSetScopeComponents::Entitlement => {
                        ParameterSetScopeComponents::Entitlement
                    }
                    ParameterSetScopeComponents::Asset { identity_element } => {
                        ParameterSetScopeComponents::Asset {
                            identity_element: identity_element.clone(),
                        }
                    }
                },
            },
        })
    }

    fn parameter_set_from_components(
        &self,
        _params: ParameterSetFromComponentsParams,
        payload: ParameterSetFromComponentsPayload<P::G1, P::G2>,
    ) -> ParameterSetFromComponentsReturn<Self::ParameterSet> {
        Ok(ParameterSetFromComponentsSuccessReturn {
            parameter_set: MockICredentialKemAdapterParameterSet {
                g1: payload.components.g1,
                u0: payload.components.u0,
                u1: payload.components.u1,
                g2: payload.components.g2,
                hpub: payload.components.hpub,
                scope: payload.components.scope,
            },
        })
    }

    fn identity_element_components(
        &self,
        _params: IdentityElementComponentsParams,
        payload: IdentityElementComponentsPayload<'_, Self::IdentityElement>,
    ) -> IdentityElementComponentsReturn<P::Scalar, P::G1> {
        Ok(IdentityElementComponentsSuccessReturn {
            components: IdentityElementComponents {
                scalar: payload.identity_element.scalar.clone(),
                element: payload.identity_element.element.clone(),
            },
        })
    }

    fn capsule_components(
        &self,
        _params: CapsuleComponentsParams,
        payload: CapsuleComponentsPayload<'_, Self::Capsule>,
    ) -> CapsuleComponentsReturn<P::G1, P::G2> {
        Ok(CapsuleComponentsSuccessReturn {
            components: match payload.capsule {
                MockICredentialKemAdapterCapsule::Entitlement { u, v, w } => {
                    CapsuleComponents::Entitlement {
                        u: u.clone(),
                        v: v.clone(),
                        w: w.clone(),
                    }
                }
                MockICredentialKemAdapterCapsule::Asset { u, v } => CapsuleComponents::Asset {
                    u: u.clone(),
                    v: v.clone(),
                },
            },
        })
    }

    fn capsule_from_components(
        &self,
        _params: CapsuleFromComponentsParams,
        payload: CapsuleFromComponentsPayload<P::G1, P::G2>,
    ) -> CapsuleFromComponentsReturn<Self::Capsule> {
        Ok(CapsuleFromComponentsSuccessReturn {
            capsule: match payload.components {
                CapsuleComponents::Entitlement { u, v, w } => {
                    MockICredentialKemAdapterCapsule::Entitlement { u, v, w }
                }
                CapsuleComponents::Asset { u, v } => {
                    MockICredentialKemAdapterCapsule::Asset { u, v }
                }
            },
        })
    }

    fn master_scalar_components(
        &self,
        _params: MasterScalarComponentsParams,
        payload: MasterScalarComponentsPayload<'_, Self::MasterScalar>,
    ) -> MasterScalarComponentsReturn<P::Scalar> {
        let Ok(value) = Secret::try_new(SecretConstructorParams {
            value: payload.master_scalar.value.expose().clone(),
        });
        Ok(MasterScalarComponentsSuccessReturn {
            components: MasterScalarComponents { value },
        })
    }

    fn master_scalar_from_components(
        &self,
        _params: MasterScalarFromComponentsParams,
        payload: MasterScalarFromComponentsPayload<P::Scalar>,
    ) -> MasterScalarFromComponentsReturn<Self::MasterScalar> {
        Ok(MasterScalarFromComponentsSuccessReturn {
            master_scalar: MockICredentialKemAdapterMasterScalar {
                value: payload.components.value,
            },
        })
    }
}

fn sample_scalar<S: ISampleUniformScalar>(
    uniform: Secret<Vec<u8>>,
) -> Result<Secret<S>, SampleUniformScalarErrorReturn> {
    S::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload { uniform },
    )
    .map(|sampled| sampled.scalar)
}

fn filled_uniform_bytes<S: ISampleUniformScalar>(value: u8) -> Secret<Vec<u8>> {
    build_secret(vec![value; S::UNIFORM_BYTES_LENGTH], Default::default())
}

fn tail_uniform_bytes<S: ISampleUniformScalar>(value: u8) -> Secret<Vec<u8>> {
    let mut uniform = vec![0u8; S::UNIFORM_BYTES_LENGTH];
    uniform[S::UNIFORM_BYTES_LENGTH - 1] = value;
    build_secret(uniform, Default::default())
}

fn sampled_scalar<S: ISampleUniformScalar>(uniform: Secret<Vec<u8>>) -> Secret<S> {
    sample_scalar(uniform).expect("sampling a default scalar always succeeds")
}

fn default_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    success.point
}

fn default_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    success.point
}

fn g1_sum<P: IPairingAdapter>(pairing: &P, left: P::G1, right: P::G1) -> P::G1 {
    let Ok(success) = pairing.add_g1(AddG1Params, AddG1Payload { left, right });
    success.sum
}

fn g2_sum<P: IPairingAdapter>(pairing: &P, left: P::G2, right: P::G2) -> P::G2 {
    let Ok(success) = pairing.add_g2(AddG2Params, AddG2Payload { left, right });
    success.sum
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn encoded_g1_value<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Value {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    byte_array(encoded.bytes.as_ref())
}

fn encoded_g2_value<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Value {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    byte_array(encoded.bytes.as_ref())
}

fn encoded_scalar_value<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> Value {
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: scalar.clone(),
        },
    );
    byte_array(encoded.bytes.expose().as_ref())
}

fn scope_value<G1>(scope: &ParameterSetScopeComponents<G1>) -> Value {
    Value::from(match scope {
        ParameterSetScopeComponents::Entitlement => "Entitlement",
        ParameterSetScopeComponents::Asset { .. } => "Asset",
    })
}

fn credential_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &CredentialComponents<P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert("a".to_string(), encoded_g1_value(pairing, &components.a));
    fields.insert("b".to_string(), encoded_g2_value(pairing, &components.b));
    Value::Object(fields)
}

fn parameter_set_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &ParameterSetComponents<P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert("g1".to_string(), encoded_g1_value(pairing, &components.g1));
    fields.insert("u0".to_string(), encoded_g1_value(pairing, &components.u0));
    fields.insert("u1".to_string(), encoded_g1_value(pairing, &components.u1));
    fields.insert("g2".to_string(), encoded_g2_value(pairing, &components.g2));
    fields.insert(
        "hpub".to_string(),
        encoded_g2_value(pairing, &components.hpub),
    );
    fields.insert("scope".to_string(), scope_value(&components.scope));
    Value::Object(fields)
}

fn identity_element_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &IdentityElementComponents<P::Scalar, P::G1>,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "scalar".to_string(),
        encoded_scalar_value(pairing, &components.scalar),
    );
    fields.insert(
        "element".to_string(),
        encoded_g1_value(pairing, &components.element),
    );
    Value::Object(fields)
}

fn capsule_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &CapsuleComponents<P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    match components {
        CapsuleComponents::Entitlement { u, v, w } => {
            fields.insert("variant".to_string(), Value::from("Entitlement"));
            fields.insert("u".to_string(), encoded_g2_value(pairing, u));
            fields.insert("v".to_string(), encoded_g1_value(pairing, v));
            fields.insert("w".to_string(), encoded_g1_value(pairing, w));
        }
        CapsuleComponents::Asset { u, v } => {
            fields.insert("variant".to_string(), Value::from("Asset"));
            fields.insert("u".to_string(), encoded_g2_value(pairing, u));
            fields.insert("v".to_string(), encoded_g1_value(pairing, v));
        }
    }
    Value::Object(fields)
}

fn master_scalar_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &MasterScalarComponents<P::Scalar>,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "value".to_string(),
        encoded_scalar_value(pairing, components.value.expose()),
    );
    Value::Object(fields)
}

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
            .unwrap_or(MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION.identifier),
        identity_scopes: overrides
            .identity_scopes
            .unwrap_or(MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION.identity_scopes),
        identity_tag: overrides
            .identity_tag
            .unwrap_or(MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION.identity_tag),
        adapter_version: overrides
            .adapter_version
            .unwrap_or(MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION.adapter_version),
        interface_version: overrides
            .interface_version
            .unwrap_or(MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION.interface_version),
    }
}

#[derive(Default)]
pub struct KemDeclarationCorruptions {
    pub identifier: Option<Value>,
    pub identity_scopes: Option<Value>,
    pub identity_tag: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_kem_declaration(corruptions: KemDeclarationCorruptions) -> Value {
    let declaration = build_kem_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "identifier".to_string(),
        Value::from(match declaration.identifier {
            KemIdentifier::Bb1DepthOneV1 => "Bb1DepthOneV1",
            KemIdentifier::Mock => "Mock",
        }),
    );
    fields.insert(
        "identity_scopes".to_string(),
        Value::Array(
            declaration
                .identity_scopes
                .iter()
                .map(|scope| {
                    Value::from(match scope {
                        IdentityScope::Entitlement => "Entitlement",
                        IdentityScope::Asset => "Asset",
                    })
                })
                .collect(),
        ),
    );
    fields.insert(
        "identity_tag".to_string(),
        byte_array(declaration.identity_tag),
    );
    fields.insert(
        "adapter_version".to_string(),
        Value::from(declaration.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(declaration.interface_version),
    );
    if let Some(identifier) = corruptions.identifier {
        fields.insert("identifier".to_string(), identifier);
    }
    if let Some(identity_scopes) = corruptions.identity_scopes {
        fields.insert("identity_scopes".to_string(), identity_scopes);
    }
    if let Some(identity_tag) = corruptions.identity_tag {
        fields.insert("identity_tag".to_string(), identity_tag);
    }
    if let Some(adapter_version) = corruptions.adapter_version {
        fields.insert("adapter_version".to_string(), adapter_version);
    }
    if let Some(interface_version) = corruptions.interface_version {
        fields.insert("interface_version".to_string(), interface_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EncapsulatedValueOverrides {
    pub key_material: Option<Secret<Vec<u8>>>,
}

pub fn build_encapsulated_value(overrides: EncapsulatedValueOverrides) -> EncapsulatedValue {
    EncapsulatedValue {
        bytes: overrides
            .key_material
            .unwrap_or_else(|| build_secret(Vec::new(), Default::default())),
    }
}

#[derive(Default)]
pub struct EncapsulatedValueCorruptions {
    pub key_material: Option<Value>,
}

pub fn invalidate_encapsulated_value(corruptions: EncapsulatedValueCorruptions) -> Value {
    let encapsulated = build_encapsulated_value(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "key_material".to_string(),
        byte_array(encapsulated.bytes.expose()),
    );
    if let Some(key_material) = corruptions.key_material {
        fields.insert("key_material".to_string(), key_material);
    }
    Value::Object(fields)
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
pub struct SetupParamsCorruptions {
    pub scope: Option<Value>,
}

pub fn invalidate_setup_params(corruptions: SetupParamsCorruptions) -> Value {
    let params = build_setup_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "scope".to_string(),
        Value::from(match params.scope {
            SetupScope::Entitlement => "Entitlement",
            SetupScope::Asset { .. } => "Asset",
        }),
    );
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct SetupPayloadOverrides {
    pub master_uniform: Option<Secret<Vec<u8>>>,
    pub u0_uniform: Option<Secret<Vec<u8>>>,
    pub u1_uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_setup_payload<S: ISampleUniformScalar>(
    overrides: SetupPayloadOverrides,
) -> SetupPayload {
    SetupPayload {
        master_uniform: overrides
            .master_uniform
            .unwrap_or_else(|| filled_uniform_bytes::<S>(1)),
        u0_uniform: overrides
            .u0_uniform
            .unwrap_or_else(|| filled_uniform_bytes::<S>(2)),
        u1_uniform: overrides
            .u1_uniform
            .unwrap_or_else(|| filled_uniform_bytes::<S>(3)),
    }
}

#[derive(Default)]
pub struct SetupPayloadCorruptions {
    pub master_uniform: Option<Value>,
    pub u0_uniform: Option<Value>,
    pub u1_uniform: Option<Value>,
}

pub fn invalidate_setup_payload<S: ISampleUniformScalar>(
    corruptions: SetupPayloadCorruptions,
) -> Value {
    let payload = build_setup_payload::<S>(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "master_uniform".to_string(),
        byte_array(payload.master_uniform.expose()),
    );
    fields.insert(
        "u0_uniform".to_string(),
        byte_array(payload.u0_uniform.expose()),
    );
    fields.insert(
        "u1_uniform".to_string(),
        byte_array(payload.u1_uniform.expose()),
    );
    if let Some(master_uniform) = corruptions.master_uniform {
        fields.insert("master_uniform".to_string(), master_uniform);
    }
    if let Some(u0_uniform) = corruptions.u0_uniform {
        fields.insert("u0_uniform".to_string(), u0_uniform);
    }
    if let Some(u1_uniform) = corruptions.u1_uniform {
        fields.insert("u1_uniform".to_string(), u1_uniform);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct SetupSuccessReturnOverrides;

pub fn build_setup_success_return<PS, M>(
    parameter_set: PS,
    master_scalar: M,
    _overrides: SetupSuccessReturnOverrides,
) -> SetupSuccessReturn<PS, M> {
    SetupSuccessReturn {
        parameter_set,
        master_scalar,
    }
}

#[derive(Default)]
pub struct DeriveIdentityParamsOverrides;

pub fn build_derive_identity_params(
    _overrides: DeriveIdentityParamsOverrides,
) -> DeriveIdentityParams {
    DeriveIdentityParams
}

#[derive(Default)]
pub struct DeriveIdentityPayloadOverrides<'a> {
    pub identity: Option<KemIdentity<'a>>,
}

pub fn build_derive_identity_payload<'a, PS>(
    parameter_set: &'a PS,
    overrides: DeriveIdentityPayloadOverrides<'a>,
) -> DeriveIdentityPayload<'a, PS> {
    DeriveIdentityPayload {
        parameter_set,
        identity: overrides.identity.unwrap_or(KemIdentity::Entitlement {
            canonical: b"entitlement-one",
        }),
    }
}

#[derive(Default)]
pub struct DeriveIdentitySuccessReturnOverrides;

pub fn build_derive_identity_success_return<IE>(
    identity_element: IE,
    _overrides: DeriveIdentitySuccessReturnOverrides,
) -> DeriveIdentitySuccessReturn<IE> {
    DeriveIdentitySuccessReturn { identity_element }
}

#[derive(Default)]
pub struct IssueParamsOverrides;

pub fn build_issue_params(_overrides: IssueParamsOverrides) -> IssueParams {
    IssueParams
}

#[derive(Default)]
pub struct IssuePayloadOverrides {
    pub uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_issue_payload<'a, S: ISampleUniformScalar, PS, M, IE>(
    parameter_set: &'a PS,
    master_scalar: &'a M,
    identity_element: &'a IE,
    overrides: IssuePayloadOverrides,
) -> IssuePayload<'a, PS, M, IE> {
    IssuePayload {
        parameter_set,
        master_scalar,
        identity_element,
        uniform: overrides
            .uniform
            .unwrap_or_else(|| filled_uniform_bytes::<S>(4)),
    }
}

pub struct IssueSuccessReturnOverrides<S: Zeroize> {
    pub randomness: Option<Secret<S>>,
}

impl<S: Zeroize> Default for IssueSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { randomness: None }
    }
}

pub fn build_issue_success_return<C, S: ISampleUniformScalar + Zeroize>(
    credential: C,
    overrides: IssueSuccessReturnOverrides<S>,
) -> IssueSuccessReturn<C, S> {
    IssueSuccessReturn {
        credential,
        randomness: overrides
            .randomness
            .unwrap_or_else(|| sampled_scalar::<S>(tail_uniform_bytes::<S>(1))),
    }
}

#[derive(Default)]
pub struct RerandomizeParamsOverrides;

pub fn build_rerandomize_params(_overrides: RerandomizeParamsOverrides) -> RerandomizeParams {
    RerandomizeParams
}

#[derive(Default)]
pub struct RerandomizePayloadOverrides {
    pub uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_rerandomize_payload<'a, S: ISampleUniformScalar, PS, IE, C>(
    parameter_set: &'a PS,
    identity_element: &'a IE,
    credential: &'a C,
    overrides: RerandomizePayloadOverrides,
) -> RerandomizePayload<'a, PS, IE, C> {
    RerandomizePayload {
        parameter_set,
        identity_element,
        credential,
        uniform: overrides
            .uniform
            .unwrap_or_else(|| filled_uniform_bytes::<S>(5)),
    }
}

pub struct RerandomizeSuccessReturnOverrides<S: Zeroize> {
    pub offset: Option<Secret<S>>,
}

impl<S: Zeroize> Default for RerandomizeSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { offset: None }
    }
}

pub fn build_rerandomize_success_return<C, S: ISampleUniformScalar + Zeroize>(
    credential: C,
    overrides: RerandomizeSuccessReturnOverrides<S>,
) -> RerandomizeSuccessReturn<C, S> {
    RerandomizeSuccessReturn {
        credential,
        offset: overrides
            .offset
            .unwrap_or_else(|| sampled_scalar::<S>(tail_uniform_bytes::<S>(1))),
    }
}

#[derive(Default)]
pub struct IsValidParamsOverrides;

pub fn build_is_valid_params(_overrides: IsValidParamsOverrides) -> IsValidParams {
    IsValidParams
}

#[derive(Default)]
pub struct IsValidPayloadOverrides;

pub fn build_is_valid_payload<'a, PS, IE, C>(
    parameter_set: &'a PS,
    identity_element: &'a IE,
    credential: &'a C,
    _overrides: IsValidPayloadOverrides,
) -> IsValidPayload<'a, PS, IE, C> {
    IsValidPayload {
        parameter_set,
        identity_element,
        credential,
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
        is_valid: overrides.is_valid.unwrap_or(true),
    }
}

#[derive(Default)]
pub struct IsValidSuccessReturnCorruptions {
    pub is_valid: Option<Value>,
}

pub fn invalidate_is_valid_success_return(corruptions: IsValidSuccessReturnCorruptions) -> Value {
    let success = build_is_valid_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert("is_valid".to_string(), Value::from(success.is_valid));
    if let Some(is_valid) = corruptions.is_valid {
        fields.insert("is_valid".to_string(), is_valid);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EncapsulateParamsOverrides;

pub fn build_encapsulate_params(_overrides: EncapsulateParamsOverrides) -> EncapsulateParams {
    EncapsulateParams
}

#[derive(Default)]
pub struct EncapsulatePayloadOverrides {
    pub uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_encapsulate_payload<'a, S: ISampleUniformScalar, PS>(
    parameter_set: &'a PS,
    overrides: EncapsulatePayloadOverrides,
) -> EncapsulatePayload<'a, PS> {
    EncapsulatePayload {
        parameter_set,
        uniform: overrides
            .uniform
            .unwrap_or_else(|| filled_uniform_bytes::<S>(6)),
    }
}

#[derive(Default)]
pub struct EncapsulateSuccessReturnOverrides {
    pub encapsulated: Option<EncapsulatedValue>,
}

pub fn build_encapsulate_success_return<CA>(
    capsule: CA,
    overrides: EncapsulateSuccessReturnOverrides,
) -> EncapsulateSuccessReturn<CA> {
    EncapsulateSuccessReturn {
        capsule,
        encapsulated: overrides
            .encapsulated
            .unwrap_or_else(|| build_encapsulated_value(Default::default())),
    }
}

#[derive(Default)]
pub struct IsWellFormedParamsOverrides;

pub fn build_is_well_formed_params(_overrides: IsWellFormedParamsOverrides) -> IsWellFormedParams {
    IsWellFormedParams
}

#[derive(Default)]
pub struct IsWellFormedPayloadOverrides;

pub fn build_is_well_formed_payload<'a, PS, CA>(
    parameter_set: &'a PS,
    capsule: &'a CA,
    _overrides: IsWellFormedPayloadOverrides,
) -> IsWellFormedPayload<'a, PS, CA> {
    IsWellFormedPayload {
        parameter_set,
        capsule,
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
        is_well_formed: overrides.is_well_formed.unwrap_or(true),
    }
}

#[derive(Default)]
pub struct IsWellFormedSuccessReturnCorruptions {
    pub is_well_formed: Option<Value>,
}

pub fn invalidate_is_well_formed_success_return(
    corruptions: IsWellFormedSuccessReturnCorruptions,
) -> Value {
    let success = build_is_well_formed_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "is_well_formed".to_string(),
        Value::from(success.is_well_formed),
    );
    if let Some(is_well_formed) = corruptions.is_well_formed {
        fields.insert("is_well_formed".to_string(), is_well_formed);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct DecapsulateParamsOverrides;

pub fn build_decapsulate_params(_overrides: DecapsulateParamsOverrides) -> DecapsulateParams {
    DecapsulateParams
}

#[derive(Default)]
pub struct DecapsulatePayloadOverrides;

pub fn build_decapsulate_payload<'a, IE, C, CA>(
    identity_element: &'a IE,
    credential: &'a C,
    capsule: &'a CA,
    _overrides: DecapsulatePayloadOverrides,
) -> DecapsulatePayload<'a, IE, C, CA> {
    DecapsulatePayload {
        identity_element,
        credential,
        capsule,
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

#[derive(Default)]
pub struct DecapsulateSuccessReturnCorruptions {
    pub encapsulated: Option<Value>,
}

pub fn invalidate_decapsulate_success_return(
    corruptions: DecapsulateSuccessReturnCorruptions,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "encapsulated".to_string(),
        invalidate_encapsulated_value(Default::default()),
    );
    if let Some(encapsulated) = corruptions.encapsulated {
        fields.insert("encapsulated".to_string(), encapsulated);
    }
    Value::Object(fields)
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
        a: overrides.a.unwrap_or_else(|| default_g1(pairing)),
        b: overrides.b.unwrap_or_else(|| default_g2(pairing)),
    }
}

#[derive(Default)]
pub struct CredentialComponentsCorruptions {
    pub a: Option<Value>,
    pub b: Option<Value>,
}

pub fn invalidate_credential_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: CredentialComponentsCorruptions,
) -> Value {
    let components = build_credential_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("a".to_string(), encoded_g1_value(pairing, &components.a));
    fields.insert("b".to_string(), encoded_g2_value(pairing, &components.b));
    if let Some(a) = corruptions.a {
        fields.insert("a".to_string(), a);
    }
    if let Some(b) = corruptions.b {
        fields.insert("b".to_string(), b);
    }
    Value::Object(fields)
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
    let g1 = overrides.g1.unwrap_or_else(|| default_g1(pairing));
    let g2 = overrides.g2.unwrap_or_else(|| default_g2(pairing));
    let u0 = overrides
        .u0
        .unwrap_or_else(|| g1_sum(pairing, g1.clone(), g1.clone()));
    let u1 = overrides
        .u1
        .unwrap_or_else(|| g1_sum(pairing, u0.clone(), g1.clone()));
    let hpub = overrides
        .hpub
        .unwrap_or_else(|| g2_sum(pairing, g2.clone(), g2.clone()));
    ParameterSetComponents {
        g1,
        u0,
        u1,
        g2,
        hpub,
        scope: overrides
            .scope
            .unwrap_or(ParameterSetScopeComponents::Entitlement),
    }
}

#[derive(Default)]
pub struct ParameterSetComponentsCorruptions {
    pub g1: Option<Value>,
    pub u0: Option<Value>,
    pub u1: Option<Value>,
    pub g2: Option<Value>,
    pub hpub: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_parameter_set_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: ParameterSetComponentsCorruptions,
) -> Value {
    let components = build_parameter_set_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("g1".to_string(), encoded_g1_value(pairing, &components.g1));
    fields.insert("u0".to_string(), encoded_g1_value(pairing, &components.u0));
    fields.insert("u1".to_string(), encoded_g1_value(pairing, &components.u1));
    fields.insert("g2".to_string(), encoded_g2_value(pairing, &components.g2));
    fields.insert(
        "hpub".to_string(),
        encoded_g2_value(pairing, &components.hpub),
    );
    fields.insert("scope".to_string(), scope_value(&components.scope));
    if let Some(g1) = corruptions.g1 {
        fields.insert("g1".to_string(), g1);
    }
    if let Some(u0) = corruptions.u0 {
        fields.insert("u0".to_string(), u0);
    }
    if let Some(u1) = corruptions.u1 {
        fields.insert("u1".to_string(), u1);
    }
    if let Some(g2) = corruptions.g2 {
        fields.insert("g2".to_string(), g2);
    }
    if let Some(hpub) = corruptions.hpub {
        fields.insert("hpub".to_string(), hpub);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
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
        scalar: overrides.scalar.unwrap_or_else(|| {
            sampled_scalar::<P::Scalar>(filled_uniform_bytes::<P::Scalar>(1))
                .expose()
                .clone()
        }),
        element: overrides.element.unwrap_or_else(|| default_g1(pairing)),
    }
}

#[derive(Default)]
pub struct IdentityElementComponentsCorruptions {
    pub scalar: Option<Value>,
    pub element: Option<Value>,
}

pub fn invalidate_identity_element_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: IdentityElementComponentsCorruptions,
) -> Value {
    let components = build_identity_element_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "scalar".to_string(),
        encoded_scalar_value(pairing, &components.scalar),
    );
    fields.insert(
        "element".to_string(),
        encoded_g1_value(pairing, &components.element),
    );
    if let Some(scalar) = corruptions.scalar {
        fields.insert("scalar".to_string(), scalar);
    }
    if let Some(element) = corruptions.element {
        fields.insert("element".to_string(), element);
    }
    Value::Object(fields)
}

pub struct MasterScalarComponentsOverrides<S: Zeroize> {
    pub value: Option<Secret<S>>,
}

impl<S: Zeroize> Default for MasterScalarComponentsOverrides<S> {
    fn default() -> Self {
        Self { value: None }
    }
}

pub fn build_master_scalar_components<S: ISampleUniformScalar + Zeroize>(
    overrides: MasterScalarComponentsOverrides<S>,
) -> MasterScalarComponents<S> {
    MasterScalarComponents {
        value: overrides
            .value
            .unwrap_or_else(|| sampled_scalar::<S>(filled_uniform_bytes::<S>(2))),
    }
}

#[derive(Default)]
pub struct MasterScalarComponentsCorruptions {
    pub value: Option<Value>,
}

pub fn invalidate_master_scalar_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MasterScalarComponentsCorruptions,
) -> Value {
    let components = build_master_scalar_components::<P::Scalar>(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "value".to_string(),
        encoded_scalar_value(pairing, components.value.expose()),
    );
    if let Some(value) = corruptions.value {
        fields.insert("value".to_string(), value);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CredentialComponentsParamsOverrides;

pub fn build_credential_components_params(
    _overrides: CredentialComponentsParamsOverrides,
) -> CredentialComponentsParams {
    CredentialComponentsParams
}

#[derive(Default)]
pub struct CredentialComponentsPayloadOverrides;

pub fn build_credential_components_payload<'a, C>(
    credential: &'a C,
    _overrides: CredentialComponentsPayloadOverrides,
) -> CredentialComponentsPayload<'a, C> {
    CredentialComponentsPayload { credential }
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
pub struct CredentialComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_credential_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: CredentialComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_credential_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        credential_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CredentialFromComponentsParamsOverrides;

pub fn build_credential_from_components_params(
    _overrides: CredentialFromComponentsParamsOverrides,
) -> CredentialFromComponentsParams {
    CredentialFromComponentsParams
}

pub struct CredentialFromComponentsPayloadOverrides<G1, G2> {
    pub components: Option<CredentialComponents<G1, G2>>,
}

impl<G1, G2> Default for CredentialFromComponentsPayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_credential_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: CredentialFromComponentsPayloadOverrides<P::G1, P::G2>,
) -> CredentialFromComponentsPayload<P::G1, P::G2> {
    CredentialFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| build_credential_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct CredentialFromComponentsPayloadCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_credential_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: CredentialFromComponentsPayloadCorruptions,
) -> Value {
    let payload = build_credential_from_components_payload(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        credential_components_value(pairing, &payload.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CredentialFromComponentsSuccessReturnOverrides;

pub fn build_credential_from_components_success_return<C>(
    credential: C,
    _overrides: CredentialFromComponentsSuccessReturnOverrides,
) -> CredentialFromComponentsSuccessReturn<C> {
    CredentialFromComponentsSuccessReturn { credential }
}

#[derive(Default)]
pub struct ParameterSetComponentsParamsOverrides;

pub fn build_parameter_set_components_params(
    _overrides: ParameterSetComponentsParamsOverrides,
) -> ParameterSetComponentsParams {
    ParameterSetComponentsParams
}

#[derive(Default)]
pub struct ParameterSetComponentsPayloadOverrides;

pub fn build_parameter_set_components_payload<'a, PS>(
    parameter_set: &'a PS,
    _overrides: ParameterSetComponentsPayloadOverrides,
) -> ParameterSetComponentsPayload<'a, PS> {
    ParameterSetComponentsPayload { parameter_set }
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
pub struct ParameterSetComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_parameter_set_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: ParameterSetComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_parameter_set_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        parameter_set_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ParameterSetFromComponentsParamsOverrides;

pub fn build_parameter_set_from_components_params(
    _overrides: ParameterSetFromComponentsParamsOverrides,
) -> ParameterSetFromComponentsParams {
    ParameterSetFromComponentsParams
}

pub struct ParameterSetFromComponentsPayloadOverrides<G1, G2> {
    pub components: Option<ParameterSetComponents<G1, G2>>,
}

impl<G1, G2> Default for ParameterSetFromComponentsPayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_parameter_set_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: ParameterSetFromComponentsPayloadOverrides<P::G1, P::G2>,
) -> ParameterSetFromComponentsPayload<P::G1, P::G2> {
    ParameterSetFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| build_parameter_set_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct ParameterSetFromComponentsPayloadCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_parameter_set_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: ParameterSetFromComponentsPayloadCorruptions,
) -> Value {
    let payload = build_parameter_set_from_components_payload(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        parameter_set_components_value(pairing, &payload.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ParameterSetFromComponentsSuccessReturnOverrides;

pub fn build_parameter_set_from_components_success_return<PS>(
    parameter_set: PS,
    _overrides: ParameterSetFromComponentsSuccessReturnOverrides,
) -> ParameterSetFromComponentsSuccessReturn<PS> {
    ParameterSetFromComponentsSuccessReturn { parameter_set }
}

#[derive(Default)]
pub struct IdentityElementComponentsParamsOverrides;

pub fn build_identity_element_components_params(
    _overrides: IdentityElementComponentsParamsOverrides,
) -> IdentityElementComponentsParams {
    IdentityElementComponentsParams
}

#[derive(Default)]
pub struct IdentityElementComponentsPayloadOverrides;

pub fn build_identity_element_components_payload<'a, IE>(
    identity_element: &'a IE,
    _overrides: IdentityElementComponentsPayloadOverrides,
) -> IdentityElementComponentsPayload<'a, IE> {
    IdentityElementComponentsPayload { identity_element }
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

#[derive(Default)]
pub struct IdentityElementComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_identity_element_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: IdentityElementComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_identity_element_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        identity_element_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CapsuleComponentsParamsOverrides;

pub fn build_capsule_components_params(
    _overrides: CapsuleComponentsParamsOverrides,
) -> CapsuleComponentsParams {
    CapsuleComponentsParams
}

#[derive(Default)]
pub struct CapsuleComponentsPayloadOverrides;

pub fn build_capsule_components_payload<'a, CA>(
    capsule: &'a CA,
    _overrides: CapsuleComponentsPayloadOverrides,
) -> CapsuleComponentsPayload<'a, CA> {
    CapsuleComponentsPayload { capsule }
}

fn default_capsule_components<P: IPairingAdapter>(pairing: &P) -> CapsuleComponents<P::G1, P::G2> {
    CapsuleComponents::Entitlement {
        u: default_g2(pairing),
        v: default_g1(pairing),
        w: default_g1(pairing),
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
            .unwrap_or_else(|| default_capsule_components(pairing)),
    }
}

#[derive(Default)]
pub struct CapsuleComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_capsule_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: CapsuleComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_capsule_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        capsule_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CapsuleFromComponentsParamsOverrides;

pub fn build_capsule_from_components_params(
    _overrides: CapsuleFromComponentsParamsOverrides,
) -> CapsuleFromComponentsParams {
    CapsuleFromComponentsParams
}

pub struct CapsuleFromComponentsPayloadOverrides<G1, G2> {
    pub components: Option<CapsuleComponents<G1, G2>>,
}

impl<G1, G2> Default for CapsuleFromComponentsPayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_capsule_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: CapsuleFromComponentsPayloadOverrides<P::G1, P::G2>,
) -> CapsuleFromComponentsPayload<P::G1, P::G2> {
    CapsuleFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| default_capsule_components(pairing)),
    }
}

#[derive(Default)]
pub struct CapsuleFromComponentsPayloadCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_capsule_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: CapsuleFromComponentsPayloadCorruptions,
) -> Value {
    let payload = build_capsule_from_components_payload(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        capsule_components_value(pairing, &payload.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CapsuleFromComponentsSuccessReturnOverrides;

pub fn build_capsule_from_components_success_return<CA>(
    capsule: CA,
    _overrides: CapsuleFromComponentsSuccessReturnOverrides,
) -> CapsuleFromComponentsSuccessReturn<CA> {
    CapsuleFromComponentsSuccessReturn { capsule }
}

#[derive(Default)]
pub struct MasterScalarComponentsParamsOverrides;

pub fn build_master_scalar_components_params(
    _overrides: MasterScalarComponentsParamsOverrides,
) -> MasterScalarComponentsParams {
    MasterScalarComponentsParams
}

#[derive(Default)]
pub struct MasterScalarComponentsPayloadOverrides;

pub fn build_master_scalar_components_payload<'a, M>(
    master_scalar: &'a M,
    _overrides: MasterScalarComponentsPayloadOverrides,
) -> MasterScalarComponentsPayload<'a, M> {
    MasterScalarComponentsPayload { master_scalar }
}

pub struct MasterScalarComponentsSuccessReturnOverrides<S: Zeroize> {
    pub components: Option<MasterScalarComponents<S>>,
}

impl<S: Zeroize> Default for MasterScalarComponentsSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_master_scalar_components_success_return<S: ISampleUniformScalar + Zeroize>(
    overrides: MasterScalarComponentsSuccessReturnOverrides<S>,
) -> MasterScalarComponentsSuccessReturn<S> {
    MasterScalarComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_master_scalar_components(Default::default())),
    }
}

#[derive(Default)]
pub struct MasterScalarComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_master_scalar_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MasterScalarComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_master_scalar_components_success_return::<P::Scalar>(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        master_scalar_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MasterScalarFromComponentsParamsOverrides;

pub fn build_master_scalar_from_components_params(
    _overrides: MasterScalarFromComponentsParamsOverrides,
) -> MasterScalarFromComponentsParams {
    MasterScalarFromComponentsParams
}

pub struct MasterScalarFromComponentsPayloadOverrides<S: Zeroize> {
    pub components: Option<MasterScalarComponents<S>>,
}

impl<S: Zeroize> Default for MasterScalarFromComponentsPayloadOverrides<S> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_master_scalar_from_components_payload<S: ISampleUniformScalar + Zeroize>(
    overrides: MasterScalarFromComponentsPayloadOverrides<S>,
) -> MasterScalarFromComponentsPayload<S> {
    MasterScalarFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| build_master_scalar_components(Default::default())),
    }
}

#[derive(Default)]
pub struct MasterScalarFromComponentsPayloadCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_master_scalar_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MasterScalarFromComponentsPayloadCorruptions,
) -> Value {
    let payload = build_master_scalar_from_components_payload::<P::Scalar>(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        master_scalar_components_value(pairing, &payload.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MasterScalarFromComponentsSuccessReturnOverrides;

pub fn build_master_scalar_from_components_success_return<M>(
    master_scalar: M,
    _overrides: MasterScalarFromComponentsSuccessReturnOverrides,
) -> MasterScalarFromComponentsSuccessReturn<M> {
    MasterScalarFromComponentsSuccessReturn { master_scalar }
}

#[derive(Default)]
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

#[derive(Default)]
pub struct ConsumeKemParamsOverrides;

pub fn build_consume_kem_params(_overrides: ConsumeKemParamsOverrides) -> ConsumeKemParams {
    ConsumeKemParams
}

#[derive(Default)]
pub struct ConsumeKemPayloadOverrides {
    pub scope: Option<IdentityScope>,
}

pub fn build_consume_kem_payload<K: ICredentialKemAdapter>(
    adapter: K,
    overrides: ConsumeKemPayloadOverrides,
) -> ConsumeKemPayload<K> {
    ConsumeKemPayload {
        adapter,
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

#[derive(Default)]
pub struct CreateKemDepsOverrides;

pub fn build_create_kem_deps<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    consumer: C,
    _overrides: CreateKemDepsOverrides,
) -> CreateKemDeps<'a, P, C> {
    CreateKemDeps {
        pairing,
        hash_to_scalar,
        consumer,
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
        concrete: overrides.concrete.unwrap_or(KemConcrete::Mock),
        identifier: overrides
            .identifier
            .unwrap_or(MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION.identifier),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

#[derive(Default)]
pub struct CreateKemParamsCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_create_kem_params(corruptions: CreateKemParamsCorruptions) -> Value {
    let params = build_create_kem_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        Value::from(match params.concrete {
            KemConcrete::Bb1DepthOne => "Bb1DepthOne",
            KemConcrete::Mock => "Mock",
        }),
    );
    fields.insert(
        "identifier".to_string(),
        Value::from(match params.identifier {
            KemIdentifier::Bb1DepthOneV1 => "Bb1DepthOneV1",
            KemIdentifier::Mock => "Mock",
        }),
    );
    fields.insert(
        "scope".to_string(),
        Value::from(match params.scope {
            IdentityScope::Entitlement => "Entitlement",
            IdentityScope::Asset => "Asset",
        }),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(identifier) = corruptions.identifier {
        fields.insert("identifier".to_string(), identifier);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CreateKemPayloadOverrides;

pub fn build_create_kem_payload(_overrides: CreateKemPayloadOverrides) -> CreateKemPayload {
    CreateKemPayload
}

#[derive(Default)]
pub struct CreateKemSuccessReturnOverrides;

pub fn build_create_kem_success_return<O>(
    output: O,
    _overrides: CreateKemSuccessReturnOverrides,
) -> CreateKemSuccessReturn<O> {
    CreateKemSuccessReturn { output }
}

pub fn mock_create_kem<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(
    deps: &CreateKemDeps<'a, P, C>,
    _params: CreateKemParams,
    _payload: CreateKemPayload,
) -> CreateKemReturn<C::Output> {
    let _: CreateKemFn<'a, P, C> = mock_create_kem;
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(MOCK_I_CREDENTIAL_KEM_ADAPTER_IDENTITY_TAG.to_vec()),
    });
    let Ok(adapter) =
        MockICredentialKemAdapter::try_new(MockICredentialKemAdapterConstructorParams {
            pairing: deps.pairing,
            hash_to_scalar: deps.hash_to_scalar,
            tag,
        });
    let output = deps.consumer.consume_kem(
        build_consume_kem_params(Default::default()),
        build_consume_kem_payload(adapter, Default::default()),
    );
    Ok(build_create_kem_success_return(output, Default::default()))
}

#[derive(Default)]
pub struct CreateKemCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
}

pub fn build_create_kem_case_params(
    overrides: CreateKemCaseParamsOverrides,
) -> CreateKemCaseParams {
    CreateKemCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
    }
}

#[derive(Default)]
pub struct CreateKemCaseDepsOverrides;

pub fn build_create_kem_case_deps<C: ICreateKemCaseConsumer>(
    consumer: C,
    _overrides: CreateKemCaseDepsOverrides,
) -> CreateKemCaseDeps<C> {
    CreateKemCaseDeps { consumer }
}

#[derive(Default)]
pub struct CreateKemCasePayloadOverrides;

pub fn build_create_kem_case_payload(
    _overrides: CreateKemCasePayloadOverrides,
) -> CreateKemCasePayload {
    CreateKemCasePayload
}

#[derive(Default)]
pub struct CreateKemCaseSuccessReturnOverrides;

pub fn build_create_kem_case_success_return<O>(
    output: O,
    _overrides: CreateKemCaseSuccessReturnOverrides,
) -> CreateKemCaseSuccessReturn<O> {
    CreateKemCaseSuccessReturn { output }
}

struct CaseAtPairing<'a, C: ICreateKemCaseConsumer> {
    consumer: &'a C,
    case_payload: &'a CreateKemCasePayload,
    hash_to_scalar_params: CreateHashToScalarParams,
}

impl<C: ICreateKemCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = CreateKemCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hashed: Result<_, CreateHashToScalarErrorReturn> = create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            self.hash_to_scalar_params,
            build_create_hash_to_scalar_payload(Default::default()),
        );
        let hashed = match hashed {
            Ok(hashed) => hashed,
            Err(error) => {
                return Err(CreateKemCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let output = self.consumer.consume_create_kem_case(
            &payload.adapter,
            &*hashed.adapter,
            self.case_payload,
        );
        Ok(CreateKemCaseSuccessReturn { output })
    }
}

pub fn run_create_kem_case<C: ICreateKemCaseConsumer>(
    deps: &CreateKemCaseDeps<C>,
    params: CreateKemCaseParams,
    payload: CreateKemCasePayload,
) -> CreateKemCaseReturn<C::Output> {
    let _: RunCreateKemCaseFn<C> = run_create_kem_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                consumer: &deps.consumer,
                case_payload: &payload,
                hash_to_scalar_params: params.hash_to_scalar,
            },
            Default::default(),
        ),
        params.pairing,
        CreatePairingPayload,
    ) {
        Ok(success) => success.output,
        Err(error) => Err(CreateKemCaseErrorReturn::CreatePairing(error)),
    }
}
