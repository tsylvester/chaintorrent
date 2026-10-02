mod interface;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CapsuleComponents, CapsuleComponentsParams, CapsuleComponentsPayload, CapsuleComponentsReturn,
    CapsuleComponentsSuccessReturn, CapsuleFromComponentsParams, CapsuleFromComponentsPayload,
    CapsuleFromComponentsReturn, CapsuleFromComponentsSuccessReturn, CredentialComponents,
    CredentialComponentsParams, CredentialComponentsPayload, CredentialComponentsReturn,
    CredentialComponentsSuccessReturn, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, CredentialFromComponentsReturn,
    CredentialFromComponentsSuccessReturn, DecapsulateParams, DecapsulatePayload,
    DecapsulateReturn, DecapsulateSuccessReturn, DeriveIdentityErrorReturn, DeriveIdentityParams,
    DeriveIdentityPayload, DeriveIdentityReturn, DeriveIdentitySuccessReturn,
    EncapsulateErrorReturn, EncapsulateParams, EncapsulatePayload, EncapsulateReturn,
    EncapsulateSuccessReturn, EncapsulatedValue, ICredentialKemAdapter, IdentityElementComponents,
    IdentityElementComponentsParams, IdentityElementComponentsPayload,
    IdentityElementComponentsReturn, IdentityElementComponentsSuccessReturn, IdentityScope,
    IsValidParams, IsValidPayload, IsValidReturn, IsValidSuccessReturn, IsWellFormedParams,
    IsWellFormedPayload, IsWellFormedReturn, IsWellFormedSuccessReturn, IssueErrorReturn,
    IssueParams, IssuePayload, IssueReturn, IssueSuccessReturn, KEM_INTERFACE_VERSION,
    KemDeclaration, KemIdentifier, MasterScalarComponents, MasterScalarComponentsParams,
    MasterScalarComponentsPayload, MasterScalarComponentsReturn,
    MasterScalarComponentsSuccessReturn, MasterScalarFromComponentsParams,
    MasterScalarFromComponentsPayload, MasterScalarFromComponentsReturn,
    MasterScalarFromComponentsSuccessReturn, ParameterSetComponents, ParameterSetComponentsParams,
    ParameterSetComponentsPayload, ParameterSetComponentsReturn,
    ParameterSetComponentsSuccessReturn, ParameterSetFromComponentsParams,
    ParameterSetFromComponentsPayload, ParameterSetFromComponentsReturn,
    ParameterSetFromComponentsSuccessReturn, ParameterSetScopeComponents, RerandomizeErrorReturn,
    RerandomizeParams, RerandomizePayload, RerandomizeReturn, RerandomizeSuccessReturn,
    SetupErrorReturn, SetupParams, SetupPayload, SetupReturn, SetupScope, SetupSuccessReturn,
};
use domain::{Secret, SecretConstructorParams};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
};
use interface::{
    BB1_DEPTH_ONE_IDENTITY_TAG, Bb1DepthOneCapsule, Bb1DepthOneCredential,
    Bb1DepthOneDeriveIdentityErrorReturn, Bb1DepthOneEncapsulateErrorReturn,
    Bb1DepthOneIdentityElement, Bb1DepthOneIssueErrorReturn, Bb1DepthOneKem,
    Bb1DepthOneKemConstructorParams, Bb1DepthOneKemTryNewErrorReturn, Bb1DepthOneKemTryNewReturn,
    Bb1DepthOneMasterScalar, Bb1DepthOneParameterSet, Bb1DepthOneParameterSetScope,
    Bb1DepthOneRerandomizeErrorReturn, Bb1DepthOneSetupErrorReturn,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, EncodeG1Params, EncodeG1Payload,
    EncodeGtParams, EncodeGtPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, ISampleUniformScalar, IsIdentityG1Params,
    IsIdentityG1Payload, MsmG1Params, MsmG1Payload, MsmG1Term, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, NegG1Params, NegG1Payload, PairingProductIsOneParams,
    PairingProductIsOnePayload, PairingProductParams, PairingProductPayload, PairingProductTerm,
    SampleUniformScalarParams, SampleUniformScalarPayload,
};

impl<'a, P: IPairingArithmetic> Bb1DepthOneKem<'a, P> {
    pub const DECLARATION: KemDeclaration = KemDeclaration {
        identifier: KemIdentifier::Bb1DepthOneV1,
        identity_scopes: &[IdentityScope::Entitlement, IdentityScope::Asset],
        identity_tag: BB1_DEPTH_ONE_IDENTITY_TAG,
        adapter_version: 1,
        interface_version: KEM_INTERFACE_VERSION,
    };

    pub fn try_new(
        params: Bb1DepthOneKemConstructorParams<'a, P>,
    ) -> Bb1DepthOneKemTryNewReturn<'a, P> {
        let tag = match DomainTag::try_new(DomainTagConstructorParams {
            bytes: BB1_DEPTH_ONE_IDENTITY_TAG.to_vec(),
        }) {
            Ok(tag) => tag,
            Err(error) => {
                return Err(Bb1DepthOneKemTryNewErrorReturn::DomainTag(error));
            }
        };
        Ok(Bb1DepthOneKem {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            tag,
        })
    }
}

impl<'a, P: IPairingArithmetic> ICredentialKemAdapter for Bb1DepthOneKem<'a, P> {
    type Pairing = P;
    type ParameterSet = Bb1DepthOneParameterSet<P>;
    type MasterScalar = Bb1DepthOneMasterScalar<P>;
    type IdentityElement = Bb1DepthOneIdentityElement<P>;
    type Credential = Bb1DepthOneCredential<P>;
    type Capsule = Bb1DepthOneCapsule<P>;

    fn setup(
        &self,
        params: SetupParams<'_>,
        payload: SetupPayload,
    ) -> SetupReturn<Self::ParameterSet, Self::MasterScalar> {
        let master_scalar = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.master_uniform,
            },
        ) {
            Ok(success) => success.scalar,
            Err(error) => {
                return Err(SetupErrorReturn::Bb1DepthOne(
                    Bb1DepthOneSetupErrorReturn::MasterScalarSampling(error),
                ));
            }
        };
        let a = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.u0_uniform,
            },
        ) {
            Ok(success) => success.scalar,
            Err(error) => {
                return Err(SetupErrorReturn::Bb1DepthOne(
                    Bb1DepthOneSetupErrorReturn::U0Sampling(error),
                ));
            }
        };
        let b = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.u1_uniform,
            },
        ) {
            Ok(success) => success.scalar,
            Err(error) => {
                return Err(SetupErrorReturn::Bb1DepthOne(
                    Bb1DepthOneSetupErrorReturn::U1Sampling(error),
                ));
            }
        };
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
                scalar: a.expose().clone(),
            },
        );
        let Ok(u1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: b.expose().clone(),
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
            SetupScope::Entitlement => Bb1DepthOneParameterSetScope::Entitlement,
            SetupScope::Asset { identity } => {
                let mapped = match self.hash_to_scalar.hash_to_scalar(
                    HashToScalarParams { tag: &self.tag },
                    HashToScalarPayload { message: identity },
                ) {
                    Ok(mapped) => mapped,
                    Err(error) => {
                        return Err(SetupErrorReturn::Bb1DepthOne(
                            Bb1DepthOneSetupErrorReturn::HashToScalar(error),
                        ));
                    }
                };
                let Ok(i_times_u1) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: u1.product.clone(),
                        scalar: mapped.scalar,
                    },
                );
                let Ok(element) = self.pairing.add_g1(
                    AddG1Params,
                    AddG1Payload {
                        left: u0.product.clone(),
                        right: i_times_u1.product,
                    },
                );
                let Ok(trivial) = self.pairing.is_identity_g1(
                    IsIdentityG1Params,
                    IsIdentityG1Payload {
                        point: element.sum.clone(),
                    },
                );
                if trivial.is_identity {
                    return Err(SetupErrorReturn::Bb1DepthOne(
                        Bb1DepthOneSetupErrorReturn::TrivialIdentityElement,
                    ));
                }
                Bb1DepthOneParameterSetScope::Asset {
                    identity_element: element.sum,
                }
            }
        };
        Ok(SetupSuccessReturn {
            parameter_set: Bb1DepthOneParameterSet {
                g1: g1.point,
                u0: u0.product,
                u1: u1.product,
                g2: g2.point,
                hpub: hpub.product,
                scope,
            },
            master_scalar: Bb1DepthOneMasterScalar {
                value: master_scalar,
            },
        })
    }

    fn derive_identity(
        &self,
        _params: DeriveIdentityParams,
        payload: DeriveIdentityPayload<'_, Self::ParameterSet>,
    ) -> DeriveIdentityReturn<Self::IdentityElement> {
        let mapped = match self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams { tag: &self.tag },
            HashToScalarPayload {
                message: payload.identity,
            },
        ) {
            Ok(mapped) => mapped,
            Err(error) => {
                return Err(DeriveIdentityErrorReturn::Bb1DepthOne(
                    Bb1DepthOneDeriveIdentityErrorReturn::HashToScalar(error),
                ));
            }
        };
        let Ok(i_times_u1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: payload.parameter_set.u1.clone(),
                scalar: mapped.scalar.clone(),
            },
        );
        let Ok(element) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: payload.parameter_set.u0.clone(),
                right: i_times_u1.product,
            },
        );
        let Ok(trivial) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: element.sum.clone(),
            },
        );
        if trivial.is_identity {
            return Err(DeriveIdentityErrorReturn::Bb1DepthOne(
                Bb1DepthOneDeriveIdentityErrorReturn::TrivialIdentityElement,
            ));
        }
        if let Bb1DepthOneParameterSetScope::Asset { identity_element } =
            &payload.parameter_set.scope
        {
            let Ok(derived) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: element.sum.clone(),
                },
            );
            let Ok(fixed) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: identity_element.clone(),
                },
            );
            if derived.bytes != fixed.bytes {
                return Err(DeriveIdentityErrorReturn::Bb1DepthOne(
                    Bb1DepthOneDeriveIdentityErrorReturn::OutsideAssetScope,
                ));
            }
        }
        Ok(DeriveIdentitySuccessReturn {
            identity_element: Bb1DepthOneIdentityElement {
                scalar: mapped.scalar,
                element: element.sum,
            },
        })
    }

    fn issue(
        &self,
        _params: IssueParams,
        payload: IssuePayload<'_, Self::ParameterSet, Self::MasterScalar, Self::IdentityElement>,
    ) -> IssueReturn<Self::Credential, P::Scalar> {
        let randomness = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.uniform,
            },
        ) {
            Ok(success) => success.scalar,
            Err(error) => {
                return Err(IssueErrorReturn::Bb1DepthOne(
                    Bb1DepthOneIssueErrorReturn::Sampling(error),
                ));
            }
        };
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
                        scalar: randomness.expose().clone(),
                    },
                ],
            },
        );
        let Ok(b) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: payload.parameter_set.g2.clone(),
                scalar: randomness.expose().clone(),
            },
        );
        Ok(IssueSuccessReturn {
            credential: Bb1DepthOneCredential {
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
        let offset = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.uniform,
            },
        ) {
            Ok(success) => success.scalar,
            Err(error) => {
                return Err(RerandomizeErrorReturn::Bb1DepthOne(
                    Bb1DepthOneRerandomizeErrorReturn::Sampling(error),
                ));
            }
        };
        let Ok(s_times_f) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: payload.identity_element.element.clone(),
                scalar: offset.expose().clone(),
            },
        );
        let Ok(a) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: payload.credential.a.clone(),
                right: s_times_f.product,
            },
        );
        let Ok(s_times_g2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: payload.parameter_set.g2.clone(),
                scalar: offset.expose().clone(),
            },
        );
        let Ok(b) = self.pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: payload.credential.b.clone(),
                right: s_times_g2.product,
            },
        );
        Ok(RerandomizeSuccessReturn {
            credential: Bb1DepthOneCredential { a: a.sum, b: b.sum },
            offset,
        })
    }

    fn is_valid(
        &self,
        _params: IsValidParams,
        payload: IsValidPayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>,
    ) -> IsValidReturn {
        let Ok(neg_g1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: payload.parameter_set.g1.clone(),
            },
        );
        let Ok(neg_f) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: payload.identity_element.element.clone(),
            },
        );
        let Ok(check) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: payload.credential.a.clone(),
                        g2: payload.parameter_set.g2.clone(),
                    },
                    PairingProductTerm {
                        g1: neg_g1.negation,
                        g2: payload.parameter_set.hpub.clone(),
                    },
                    PairingProductTerm {
                        g1: neg_f.negation,
                        g2: payload.credential.b.clone(),
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
        let t = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.uniform,
            },
        ) {
            Ok(success) => success.scalar,
            Err(error) => {
                return Err(EncapsulateErrorReturn::Bb1DepthOne(
                    Bb1DepthOneEncapsulateErrorReturn::Sampling(error),
                ));
            }
        };
        let Ok(u) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: payload.parameter_set.g2.clone(),
                scalar: t.expose().clone(),
            },
        );
        let capsule = match &payload.parameter_set.scope {
            Bb1DepthOneParameterSetScope::Entitlement => {
                let Ok(v) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: payload.parameter_set.u0.clone(),
                        scalar: t.expose().clone(),
                    },
                );
                let Ok(w) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: payload.parameter_set.u1.clone(),
                        scalar: t.expose().clone(),
                    },
                );
                Bb1DepthOneCapsule::Entitlement {
                    u: u.product,
                    v: v.product,
                    w: w.product,
                }
            }
            Bb1DepthOneParameterSetScope::Asset { identity_element } => {
                let Ok(v) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: identity_element.clone(),
                        scalar: t.expose().clone(),
                    },
                );
                Bb1DepthOneCapsule::Asset {
                    u: u.product,
                    v: v.product,
                }
            }
        };
        let Ok(t_times_g1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: payload.parameter_set.g1.clone(),
                scalar: t.expose().clone(),
            },
        );
        let Ok(k) = self.pairing.pairing_product(
            PairingProductParams,
            PairingProductPayload {
                terms: vec![PairingProductTerm {
                    g1: t_times_g1.product,
                    g2: payload.parameter_set.hpub.clone(),
                }],
            },
        );
        let Ok(encoded) = self
            .pairing
            .encode_gt(EncodeGtParams, EncodeGtPayload { value: k.product });
        Ok(EncapsulateSuccessReturn {
            capsule,
            encapsulated: EncapsulatedValue {
                bytes: encoded.bytes,
            },
        })
    }

    fn is_well_formed(
        &self,
        _params: IsWellFormedParams,
        payload: IsWellFormedPayload<'_, Self::ParameterSet, Self::Capsule>,
    ) -> IsWellFormedReturn {
        let is_well_formed = match (&payload.parameter_set.scope, payload.capsule) {
            (
                Bb1DepthOneParameterSetScope::Entitlement,
                Bb1DepthOneCapsule::Entitlement { u, v, w },
            ) => {
                let Ok(neg_u0) = self.pairing.neg_g1(
                    NegG1Params,
                    NegG1Payload {
                        point: payload.parameter_set.u0.clone(),
                    },
                );
                let Ok(v_check) = self.pairing.pairing_product_is_one(
                    PairingProductIsOneParams,
                    PairingProductIsOnePayload {
                        terms: vec![
                            PairingProductTerm {
                                g1: v.clone(),
                                g2: payload.parameter_set.g2.clone(),
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
                        point: payload.parameter_set.u1.clone(),
                    },
                );
                let Ok(w_check) = self.pairing.pairing_product_is_one(
                    PairingProductIsOneParams,
                    PairingProductIsOnePayload {
                        terms: vec![
                            PairingProductTerm {
                                g1: w.clone(),
                                g2: payload.parameter_set.g2.clone(),
                            },
                            PairingProductTerm {
                                g1: neg_u1.negation,
                                g2: u.clone(),
                            },
                        ],
                    },
                );
                v_check.is_one && w_check.is_one
            }
            (
                Bb1DepthOneParameterSetScope::Asset { identity_element },
                Bb1DepthOneCapsule::Asset { u, v },
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
                                g2: payload.parameter_set.g2.clone(),
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
        let value = match payload.capsule {
            Bb1DepthOneCapsule::Entitlement { u, v, w } => {
                let Ok(i_times_w) = self.pairing.mul_g1(
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
                        right: i_times_w.product,
                    },
                );
                let Ok(negation) = self
                    .pairing
                    .neg_g1(NegG1Params, NegG1Payload { point: sum.sum });
                self.pairing.pairing_product(
                    PairingProductParams,
                    PairingProductPayload {
                        terms: vec![
                            PairingProductTerm {
                                g1: payload.credential.a.clone(),
                                g2: u.clone(),
                            },
                            PairingProductTerm {
                                g1: negation.negation,
                                g2: payload.credential.b.clone(),
                            },
                        ],
                    },
                )
            }
            Bb1DepthOneCapsule::Asset { u, v } => {
                let Ok(neg_v) = self
                    .pairing
                    .neg_g1(NegG1Params, NegG1Payload { point: v.clone() });
                self.pairing.pairing_product(
                    PairingProductParams,
                    PairingProductPayload {
                        terms: vec![
                            PairingProductTerm {
                                g1: payload.credential.a.clone(),
                                g2: u.clone(),
                            },
                            PairingProductTerm {
                                g1: neg_v.negation,
                                g2: payload.credential.b.clone(),
                            },
                        ],
                    },
                )
            }
        };
        let Ok(value) = value;
        let Ok(encoded) = self.pairing.encode_gt(
            EncodeGtParams,
            EncodeGtPayload {
                value: value.product,
            },
        );
        Ok(DecapsulateSuccessReturn {
            encapsulated: EncapsulatedValue {
                bytes: encoded.bytes,
            },
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
            credential: Bb1DepthOneCredential {
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
        Ok(ParameterSetComponentsSuccessReturn {
            components: ParameterSetComponents {
                g1: payload.parameter_set.g1.clone(),
                u0: payload.parameter_set.u0.clone(),
                u1: payload.parameter_set.u1.clone(),
                g2: payload.parameter_set.g2.clone(),
                hpub: payload.parameter_set.hpub.clone(),
                scope: match &payload.parameter_set.scope {
                    Bb1DepthOneParameterSetScope::Entitlement => {
                        ParameterSetScopeComponents::Entitlement
                    }
                    Bb1DepthOneParameterSetScope::Asset { identity_element } => {
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
            parameter_set: Bb1DepthOneParameterSet {
                g1: payload.components.g1,
                u0: payload.components.u0,
                u1: payload.components.u1,
                g2: payload.components.g2,
                hpub: payload.components.hpub,
                scope: match payload.components.scope {
                    ParameterSetScopeComponents::Entitlement => {
                        Bb1DepthOneParameterSetScope::Entitlement
                    }
                    ParameterSetScopeComponents::Asset { identity_element } => {
                        Bb1DepthOneParameterSetScope::Asset { identity_element }
                    }
                },
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
                Bb1DepthOneCapsule::Entitlement { u, v, w } => CapsuleComponents::Entitlement {
                    u: u.clone(),
                    v: v.clone(),
                    w: w.clone(),
                },
                Bb1DepthOneCapsule::Asset { u, v } => CapsuleComponents::Asset {
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
                    Bb1DepthOneCapsule::Entitlement { u, v, w }
                }
                CapsuleComponents::Asset { u, v } => Bb1DepthOneCapsule::Asset { u, v },
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
            master_scalar: Bb1DepthOneMasterScalar {
                value: payload.components.value,
            },
        })
    }
}
