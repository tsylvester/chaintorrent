#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    BB1_DEPTH_ONE_IDENTITY_TAG, Bb1DepthOneCapsule, Bb1DepthOneDeriveIdentityErrorReturn,
    Bb1DepthOneEncapsulateErrorReturn, Bb1DepthOneIssueErrorReturn, Bb1DepthOneKem,
    Bb1DepthOneKemConstructorParams, Bb1DepthOneParameterSet, Bb1DepthOneParameterSetScope,
    Bb1DepthOneRerandomizeErrorReturn, Bb1DepthOneSetupErrorReturn,
};
use crate::factory::provides::{
    CapsuleComponents, CapsuleComponentsParams, CapsuleComponentsPayload,
    CapsuleFromComponentsParams, CapsuleFromComponentsPayload, CredentialComponentsParams,
    CredentialComponentsPayload, CredentialFromComponentsParams, CredentialFromComponentsPayload,
    DecapsulateParams, DecapsulatePayload, DeriveIdentityErrorReturn, DeriveIdentityParams,
    DeriveIdentityPayload, EncapsulateErrorReturn, EncapsulateParams, EncapsulatePayload,
    ICredentialKemAdapter, IdentityElementComponentsParams, IdentityElementComponentsPayload,
    IdentityScope, IsValidParams, IsValidPayload, IsWellFormedParams, IsWellFormedPayload,
    IssueErrorReturn, IssueParams, IssuePayload, KEM_INTERFACE_VERSION, KemIdentifier,
    MasterScalarComponentsParams, MasterScalarComponentsPayload, MasterScalarFromComponentsParams,
    MasterScalarFromComponentsPayload, ParameterSetComponentsParams, ParameterSetComponentsPayload,
    ParameterSetFromComponentsParams, ParameterSetFromComponentsPayload,
    ParameterSetScopeComponents, RerandomizeErrorReturn, RerandomizeParams, RerandomizePayload,
    SetupErrorReturn, SetupParamsOverrides, SetupPayload, SetupPayloadOverrides, SetupScope,
    build_setup_params, build_setup_payload,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarParamsOverrides, CreateHashToScalarPayload,
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides, CreatePairingPayload,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, ISampleUniformScalar, MsmG1Params,
    MsmG1Payload, MsmG1Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, NegScalarParams,
    NegScalarPayload, PairingConcrete, SampleUniformScalarErrorReturn, build_create_pairing_params,
    create_pairing,
};

fn uniform_draw<P: IPairingArithmetic>(byte: u8) -> Secret<Vec<u8>> {
    build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![byte; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH]),
    })
}

fn short_draw() -> Secret<Vec<u8>> {
    build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0xee; 32]),
    })
}

fn run_probe<C: IPairingConsumer>(consumer: C, concrete: PairingConcrete) -> C::Output {
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(concrete),
        ..Default::default()
    });
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };
    success.output
}

struct EntitlementScopeOutcome {
    issued_is_valid: bool,
    rerandomized_is_valid: bool,
    issued_is_valid_for_other_identity: bool,
    rerandomized_is_valid_for_other_identity: bool,
    issued_is_valid_under_other_set: bool,
    encapsulated: Vec<u8>,
    decapsulated_by_issued: Vec<u8>,
    decapsulated_by_second_issued: Vec<u8>,
    decapsulated_by_rerandomized: Vec<u8>,
    decapsulated_by_other_entitlement: Vec<u8>,
    capsule_is_well_formed: bool,
    swapped_capsule_is_well_formed: bool,
    capsule_is_well_formed_under_other_set: bool,
}

struct EntitlementScopeProbe {
    identity: Vec<u8>,
    other_identity: Vec<u8>,
}

impl IPairingConsumer for EntitlementScopeProbe {
    type Output = EntitlementScopeOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(kem) = Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
        }) else {
            panic!("the declared tag is admitted");
        };

        let Ok(setup) = kem.setup(
            build_setup_params(SetupParamsOverrides::default()),
            build_setup_payload(SetupPayloadOverrides::default()),
        ) else {
            panic!("the entitlement setup succeeds");
        };
        let parameter_set = setup.parameter_set;
        let master_scalar = setup.master_scalar;

        let Ok(other_setup) = kem.setup(
            build_setup_params(SetupParamsOverrides::default()),
            build_setup_payload(SetupPayloadOverrides {
                master_uniform: Some(uniform_draw::<P>(0x44)),
                u0_uniform: Some(uniform_draw::<P>(0x55)),
                u1_uniform: Some(uniform_draw::<P>(0x66)),
            }),
        ) else {
            panic!("the second setup succeeds");
        };
        let other_parameter_set = other_setup.parameter_set;

        let Ok(identity_element) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &parameter_set,
                identity: &self.identity,
            },
        ) else {
            panic!("the identity derives");
        };
        let Ok(other_identity_element) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &parameter_set,
                identity: &self.other_identity,
            },
        ) else {
            panic!("the other identity derives");
        };
        let Ok(other_set_identity_element) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &other_parameter_set,
                identity: &self.identity,
            },
        ) else {
            panic!("the identity derives under the other set");
        };

        let Ok(issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &parameter_set,
                master_scalar: &master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: uniform_draw::<P>(0x77),
            },
        ) else {
            panic!("the first issuance succeeds");
        };
        let Ok(second_issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &parameter_set,
                master_scalar: &master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: uniform_draw::<P>(0x88),
            },
        ) else {
            panic!("the second issuance succeeds");
        };
        let Ok(other_issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &parameter_set,
                master_scalar: &master_scalar,
                identity_element: &other_identity_element.identity_element,
                uniform: uniform_draw::<P>(0x99),
            },
        ) else {
            panic!("the other identity's issuance succeeds");
        };
        let Ok(rerandomized) = kem.rerandomize(
            RerandomizeParams,
            RerandomizePayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                uniform: uniform_draw::<P>(0xaa),
            },
        ) else {
            panic!("the rerandomization succeeds");
        };
        let Ok(encapsulated) = kem.encapsulate(
            EncapsulateParams,
            EncapsulatePayload {
                parameter_set: &parameter_set,
                uniform: uniform_draw::<P>(0xbb),
            },
        ) else {
            panic!("the encapsulation succeeds");
        };

        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
            },
        );
        let issued_is_valid = valid.is_valid;
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &rerandomized.credential,
            },
        );
        let rerandomized_is_valid = valid.is_valid;
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &other_identity_element.identity_element,
                credential: &issued.credential,
            },
        );
        let issued_is_valid_for_other_identity = valid.is_valid;
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &other_identity_element.identity_element,
                credential: &rerandomized.credential,
            },
        );
        let rerandomized_is_valid_for_other_identity = valid.is_valid;
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &other_parameter_set,
                identity_element: &other_set_identity_element.identity_element,
                credential: &issued.credential,
            },
        );
        let issued_is_valid_under_other_set = valid.is_valid;

        let encapsulated_bytes = encapsulated.encapsulated.bytes.expose().clone();
        let Ok(decapsulated) = kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let decapsulated_by_issued = decapsulated.encapsulated.bytes.expose().clone();
        let Ok(decapsulated) = kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &second_issued.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let decapsulated_by_second_issued = decapsulated.encapsulated.bytes.expose().clone();
        let Ok(decapsulated) = kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &rerandomized.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let decapsulated_by_rerandomized = decapsulated.encapsulated.bytes.expose().clone();
        let Ok(decapsulated) = kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &other_identity_element.identity_element,
                credential: &other_issued.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let decapsulated_by_other_entitlement = decapsulated.encapsulated.bytes.expose().clone();

        let Ok(well_formed) = kem.is_well_formed(
            IsWellFormedParams,
            IsWellFormedPayload {
                parameter_set: &parameter_set,
                capsule: &encapsulated.capsule,
            },
        );
        let capsule_is_well_formed = well_formed.is_well_formed;

        let Ok(capsule_components) = kem.capsule_components(
            CapsuleComponentsParams,
            CapsuleComponentsPayload {
                capsule: &encapsulated.capsule,
            },
        );
        let swapped = match capsule_components.components {
            CapsuleComponents::Entitlement { u, v, w } => {
                Bb1DepthOneCapsule::Entitlement { u, v: w, w: v }
            }
            CapsuleComponents::Asset { u, v } => Bb1DepthOneCapsule::Asset { u, v },
        };
        let Ok(well_formed) = kem.is_well_formed(
            IsWellFormedParams,
            IsWellFormedPayload {
                parameter_set: &parameter_set,
                capsule: &swapped,
            },
        );
        let swapped_capsule_is_well_formed = well_formed.is_well_formed;

        let Ok(well_formed) = kem.is_well_formed(
            IsWellFormedParams,
            IsWellFormedPayload {
                parameter_set: &other_parameter_set,
                capsule: &encapsulated.capsule,
            },
        );
        let capsule_is_well_formed_under_other_set = well_formed.is_well_formed;

        EntitlementScopeOutcome {
            issued_is_valid,
            rerandomized_is_valid,
            issued_is_valid_for_other_identity,
            rerandomized_is_valid_for_other_identity,
            issued_is_valid_under_other_set,
            encapsulated: encapsulated_bytes,
            decapsulated_by_issued,
            decapsulated_by_second_issued,
            decapsulated_by_rerandomized,
            decapsulated_by_other_entitlement,
            capsule_is_well_formed,
            swapped_capsule_is_well_formed,
            capsule_is_well_formed_under_other_set,
        }
    }
}

struct AssetScopeOutcome {
    issued_is_valid: bool,
    holder_authored_is_valid: bool,
    capsule_is_asset_form: bool,
    capsule_is_well_formed: bool,
    encapsulated: Vec<u8>,
    decapsulated_by_issued: Vec<u8>,
    decapsulated_by_holder_authored: Vec<u8>,
    other_identity_is_outside_the_scope: bool,
    set_carries_the_derived_identity_element: bool,
}

struct AssetScopeProbe {
    asset_identity: Vec<u8>,
    other_identity: Vec<u8>,
}

impl IPairingConsumer for AssetScopeProbe {
    type Output = AssetScopeOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(kem) = Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
        }) else {
            panic!("the declared tag is admitted");
        };

        let Ok(setup) = kem.setup(
            build_setup_params(SetupParamsOverrides {
                scope: Some(SetupScope::Asset {
                    identity: &self.asset_identity,
                }),
            }),
            build_setup_payload(SetupPayloadOverrides::default()),
        ) else {
            panic!("the asset setup succeeds");
        };
        let parameter_set = setup.parameter_set;
        let master_scalar = setup.master_scalar;

        let Ok(identity_element) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &parameter_set,
                identity: &self.asset_identity,
            },
        ) else {
            panic!("the asset identity derives");
        };

        let Ok(issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &parameter_set,
                master_scalar: &master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: uniform_draw::<P>(0x77),
            },
        ) else {
            panic!("the issuance succeeds");
        };
        let Ok(holder_authored) = kem.rerandomize(
            RerandomizeParams,
            RerandomizePayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                uniform: uniform_draw::<P>(0x88),
            },
        ) else {
            panic!("the holder's rerandomization succeeds");
        };
        let Ok(encapsulated) = kem.encapsulate(
            EncapsulateParams,
            EncapsulatePayload {
                parameter_set: &parameter_set,
                uniform: uniform_draw::<P>(0x99),
            },
        ) else {
            panic!("the encapsulation succeeds");
        };

        let capsule_is_asset_form =
            matches!(encapsulated.capsule, Bb1DepthOneCapsule::Asset { .. });

        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
            },
        );
        let issued_is_valid = valid.is_valid;
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &holder_authored.credential,
            },
        );
        let holder_authored_is_valid = valid.is_valid;

        let Ok(well_formed) = kem.is_well_formed(
            IsWellFormedParams,
            IsWellFormedPayload {
                parameter_set: &parameter_set,
                capsule: &encapsulated.capsule,
            },
        );
        let capsule_is_well_formed = well_formed.is_well_formed;

        let encapsulated_bytes = encapsulated.encapsulated.bytes.expose().clone();
        let Ok(decapsulated) = kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let decapsulated_by_issued = decapsulated.encapsulated.bytes.expose().clone();
        let Ok(decapsulated) = kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &holder_authored.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let decapsulated_by_holder_authored = decapsulated.encapsulated.bytes.expose().clone();

        let other_identity_is_outside_the_scope = matches!(
            kem.derive_identity(
                DeriveIdentityParams,
                DeriveIdentityPayload {
                    parameter_set: &parameter_set,
                    identity: &self.other_identity,
                },
            ),
            Err(DeriveIdentityErrorReturn::Bb1DepthOne(
                Bb1DepthOneDeriveIdentityErrorReturn::OutsideAssetScope
            ))
        );

        let Ok(set_components) = kem.parameter_set_components(
            ParameterSetComponentsParams,
            ParameterSetComponentsPayload {
                parameter_set: &parameter_set,
            },
        );
        let Ok(identity_components) = kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity_element.identity_element,
            },
        );
        let set_carries_the_derived_identity_element = match set_components.components.scope {
            ParameterSetScopeComponents::Asset { identity_element } => {
                let Ok(encoded_set) = pairing.encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: identity_element,
                    },
                );
                let Ok(encoded_derived) = pairing.encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: identity_components.components.element,
                    },
                );
                encoded_set.bytes == encoded_derived.bytes
            }
            ParameterSetScopeComponents::Entitlement => false,
        };

        AssetScopeOutcome {
            issued_is_valid,
            holder_authored_is_valid,
            capsule_is_asset_form,
            capsule_is_well_formed,
            encapsulated: encapsulated_bytes,
            decapsulated_by_issued,
            decapsulated_by_holder_authored,
            other_identity_is_outside_the_scope,
            set_carries_the_derived_identity_element,
        }
    }
}

struct ComponentsOutcome {
    hpub_is_the_master_scalar_times_g2: bool,
    identity_element_is_u0_plus_i_times_u1: bool,
    identity_scalar_is_the_declared_tags_hash: bool,
    issued_a_is_alpha_g1_plus_r_f: bool,
    issued_b_is_r_g2: bool,
    rerandomized_b_is_b_plus_s_g2: bool,
    rebuilt_credential_is_valid: bool,
    credential_is_valid_under_rebuilt_set: bool,
    rebuilt_capsule_is_well_formed: bool,
    rebuilt_capsule_decapsulates_the_encapsulated_value: bool,
    restored_master_scalar_issues_a_valid_credential: bool,
}

struct ComponentsProbe {
    identity: Vec<u8>,
}

impl IPairingConsumer for ComponentsProbe {
    type Output = ComponentsOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(kem) = Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
        }) else {
            panic!("the declared tag is admitted");
        };

        let Ok(setup) = kem.setup(
            build_setup_params(SetupParamsOverrides::default()),
            build_setup_payload(SetupPayloadOverrides::default()),
        ) else {
            panic!("the setup succeeds");
        };
        let parameter_set = setup.parameter_set;
        let master_scalar = setup.master_scalar;

        let Ok(identity_element) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &parameter_set,
                identity: &self.identity,
            },
        ) else {
            panic!("the identity derives");
        };
        let Ok(issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &parameter_set,
                master_scalar: &master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: uniform_draw::<P>(0x77),
            },
        ) else {
            panic!("the issuance succeeds");
        };
        let Ok(rerandomized) = kem.rerandomize(
            RerandomizeParams,
            RerandomizePayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                uniform: uniform_draw::<P>(0x88),
            },
        ) else {
            panic!("the rerandomization succeeds");
        };
        let Ok(encapsulated) = kem.encapsulate(
            EncapsulateParams,
            EncapsulatePayload {
                parameter_set: &parameter_set,
                uniform: uniform_draw::<P>(0x99),
            },
        ) else {
            panic!("the encapsulation succeeds");
        };

        let Ok(credential_components) = kem.credential_components(
            CredentialComponentsParams,
            CredentialComponentsPayload {
                credential: &issued.credential,
            },
        );
        let Ok(rerandomized_components) = kem.credential_components(
            CredentialComponentsParams,
            CredentialComponentsPayload {
                credential: &rerandomized.credential,
            },
        );
        let Ok(set_components) = kem.parameter_set_components(
            ParameterSetComponentsParams,
            ParameterSetComponentsPayload {
                parameter_set: &parameter_set,
            },
        );
        let Ok(identity_components) = kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity_element.identity_element,
            },
        );
        let Ok(master_components) = kem.master_scalar_components(
            MasterScalarComponentsParams,
            MasterScalarComponentsPayload {
                master_scalar: &master_scalar,
            },
        );

        let alpha = master_components.components.value.expose().clone();
        let r = issued.randomness.expose().clone();
        let s = rerandomized.offset.expose().clone();
        let i_scalar = identity_components.components.scalar.clone();

        let Ok(expected_hpub) = pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: set_components.components.g2.clone(),
                scalar: alpha.clone(),
            },
        );
        let Ok(actual_hpub) = pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: set_components.components.hpub.clone(),
            },
        );
        let Ok(expected_hpub_encoded) = pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: expected_hpub.product,
            },
        );
        let hpub_is_the_master_scalar_times_g2 = actual_hpub.bytes == expected_hpub_encoded.bytes;

        let Ok(i_times_u1) = pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: set_components.components.u1.clone(),
                scalar: i_scalar.clone(),
            },
        );
        let Ok(expected_element) = pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: set_components.components.u0.clone(),
                right: i_times_u1.product,
            },
        );
        let Ok(expected_element_encoded) = pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: expected_element.sum,
            },
        );
        let Ok(actual_element_encoded) = pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: identity_components.components.element.clone(),
            },
        );
        let identity_element_is_u0_plus_i_times_u1 =
            actual_element_encoded.bytes == expected_element_encoded.bytes;

        let Ok(declared_tag) = DomainTag::try_new(DomainTagConstructorParams {
            bytes: Bb1DepthOneKem::<'_, P>::DECLARATION.identity_tag.to_vec(),
        }) else {
            panic!("the declared tag is a domain tag");
        };
        let Ok(mapped) = hash.adapter.hash_to_scalar(
            HashToScalarParams { tag: &declared_tag },
            HashToScalarPayload {
                message: &self.identity,
            },
        ) else {
            panic!("the declared-tag mapping succeeds");
        };
        let Ok(expected_i) = pairing.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: mapped.scalar,
            },
        );
        let Ok(actual_i) =
            pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: i_scalar });
        let identity_scalar_is_the_declared_tags_hash =
            expected_i.bytes.expose() == actual_i.bytes.expose();

        let Ok(expected_a) = pairing.msm_g1(
            MsmG1Params,
            MsmG1Payload {
                terms: vec![
                    MsmG1Term {
                        base: set_components.components.g1.clone(),
                        scalar: alpha,
                    },
                    MsmG1Term {
                        base: identity_components.components.element.clone(),
                        scalar: r.clone(),
                    },
                ],
            },
        );
        let Ok(expected_a_encoded) = pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: expected_a.sum,
            },
        );
        let Ok(actual_a_encoded) = pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: credential_components.components.a.clone(),
            },
        );
        let issued_a_is_alpha_g1_plus_r_f = actual_a_encoded.bytes == expected_a_encoded.bytes;

        let Ok(expected_b) = pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: set_components.components.g2.clone(),
                scalar: r.clone(),
            },
        );
        let Ok(expected_b_encoded) = pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: expected_b.product,
            },
        );
        let Ok(actual_b_encoded) = pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: credential_components.components.b.clone(),
            },
        );
        let issued_b_is_r_g2 = actual_b_encoded.bytes == expected_b_encoded.bytes;

        let Ok(s_times_g2) = pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: set_components.components.g2.clone(),
                scalar: s,
            },
        );
        let Ok(expected_b) = pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: credential_components.components.b.clone(),
                right: s_times_g2.product,
            },
        );
        let Ok(expected_b_encoded) = pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: expected_b.sum,
            },
        );
        let Ok(actual_b_encoded) = pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: rerandomized_components.components.b,
            },
        );
        let rerandomized_b_is_b_plus_s_g2 = actual_b_encoded.bytes == expected_b_encoded.bytes;

        let Ok(rebuilt_credential) = kem.credential_from_components(
            CredentialFromComponentsParams,
            CredentialFromComponentsPayload {
                components: credential_components.components,
            },
        );
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &rebuilt_credential.credential,
            },
        );
        let rebuilt_credential_is_valid = valid.is_valid;

        let Ok(set_components_again) = kem.parameter_set_components(
            ParameterSetComponentsParams,
            ParameterSetComponentsPayload {
                parameter_set: &parameter_set,
            },
        );
        let Ok(rebuilt_set) = kem.parameter_set_from_components(
            ParameterSetFromComponentsParams,
            ParameterSetFromComponentsPayload {
                components: set_components_again.components,
            },
        );
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &rebuilt_set.parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
            },
        );
        let credential_is_valid_under_rebuilt_set = valid.is_valid;

        let Ok(capsule_components) = kem.capsule_components(
            CapsuleComponentsParams,
            CapsuleComponentsPayload {
                capsule: &encapsulated.capsule,
            },
        );
        let Ok(rebuilt_capsule) = kem.capsule_from_components(
            CapsuleFromComponentsParams,
            CapsuleFromComponentsPayload {
                components: capsule_components.components,
            },
        );
        let Ok(well_formed) = kem.is_well_formed(
            IsWellFormedParams,
            IsWellFormedPayload {
                parameter_set: &parameter_set,
                capsule: &rebuilt_capsule.capsule,
            },
        );
        let rebuilt_capsule_is_well_formed = well_formed.is_well_formed;
        let Ok(decapsulated) = kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                capsule: &rebuilt_capsule.capsule,
            },
        );
        let rebuilt_capsule_decapsulates_the_encapsulated_value =
            decapsulated.encapsulated.bytes.expose() == encapsulated.encapsulated.bytes.expose();

        let Ok(master_components_again) = kem.master_scalar_components(
            MasterScalarComponentsParams,
            MasterScalarComponentsPayload {
                master_scalar: &master_scalar,
            },
        );
        let Ok(restored_master) = kem.master_scalar_from_components(
            MasterScalarFromComponentsParams,
            MasterScalarFromComponentsPayload {
                components: master_components_again.components,
            },
        );
        let Ok(restored_issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &parameter_set,
                master_scalar: &restored_master.master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: uniform_draw::<P>(0xaa),
            },
        ) else {
            panic!("issuance under the restored master scalar succeeds");
        };
        let Ok(valid) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &restored_issued.credential,
            },
        );
        let restored_master_scalar_issues_a_valid_credential = valid.is_valid;

        ComponentsOutcome {
            hpub_is_the_master_scalar_times_g2,
            identity_element_is_u0_plus_i_times_u1,
            identity_scalar_is_the_declared_tags_hash,
            issued_a_is_alpha_g1_plus_r_f,
            issued_b_is_r_g2,
            rerandomized_b_is_b_plus_s_g2,
            rebuilt_credential_is_valid,
            credential_is_valid_under_rebuilt_set,
            rebuilt_capsule_is_well_formed,
            rebuilt_capsule_decapsulates_the_encapsulated_value,
            restored_master_scalar_issues_a_valid_credential,
        }
    }
}

struct TrivialIdentityProbe {
    identity: Vec<u8>,
}

impl IPairingConsumer for TrivialIdentityProbe {
    type Output = bool;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(tag) = DomainTag::try_new(DomainTagConstructorParams {
            bytes: BB1_DEPTH_ONE_IDENTITY_TAG.to_vec(),
        }) else {
            panic!("the tag is a domain tag");
        };
        let Ok(mapped) = hash.adapter.hash_to_scalar(
            HashToScalarParams { tag: &tag },
            HashToScalarPayload {
                message: &self.identity,
            },
        ) else {
            panic!("the identity maps");
        };
        let i_scalar = mapped.scalar;

        let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(neg_i) = pairing.neg_scalar(
            NegScalarParams,
            NegScalarPayload {
                scalar: i_scalar.clone(),
            },
        );
        let Ok(u0) = pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: neg_i.negation,
            },
        );
        let Ok(hpub) = pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: i_scalar,
            },
        );
        let parameter_set = Bb1DepthOneParameterSet {
            g1: g1.point.clone(),
            u0: u0.product,
            u1: g1.point,
            g2: g2.point,
            hpub: hpub.product,
            scope: Bb1DepthOneParameterSetScope::Entitlement,
        };

        let Ok(kem) = Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
        }) else {
            panic!("the declared tag is admitted");
        };
        matches!(
            kem.derive_identity(
                DeriveIdentityParams,
                DeriveIdentityPayload {
                    parameter_set: &parameter_set,
                    identity: &self.identity,
                },
            ),
            Err(DeriveIdentityErrorReturn::Bb1DepthOne(
                Bb1DepthOneDeriveIdentityErrorReturn::TrivialIdentityElement
            ))
        )
    }
}

struct SamplingOutcome {
    setup_master_refused: bool,
    setup_base_refused: bool,
    issue_refused: bool,
    rerandomize_refused: bool,
    encapsulate_refused: bool,
}

struct SamplingProbe;

impl IPairingConsumer for SamplingProbe {
    type Output = SamplingOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(kem) = Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
        }) else {
            panic!("the declared tag is admitted");
        };

        let setup_master_refused = matches!(
            kem.setup(
                build_setup_params(SetupParamsOverrides::default()),
                SetupPayload {
                    master_uniform: short_draw(),
                    u0_uniform: uniform_draw::<P>(0x22),
                    u1_uniform: uniform_draw::<P>(0x33),
                },
            ),
            Err(SetupErrorReturn::Bb1DepthOne(
                Bb1DepthOneSetupErrorReturn::MasterScalarSampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: 64,
                        actual: 32,
                    }
                )
            ))
        );
        let setup_base_refused = matches!(
            kem.setup(
                build_setup_params(SetupParamsOverrides::default()),
                SetupPayload {
                    master_uniform: uniform_draw::<P>(0x11),
                    u0_uniform: short_draw(),
                    u1_uniform: uniform_draw::<P>(0x33),
                },
            ),
            Err(SetupErrorReturn::Bb1DepthOne(
                Bb1DepthOneSetupErrorReturn::U0Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: 64,
                        actual: 32,
                    }
                )
            ))
        );

        let Ok(setup) = kem.setup(
            build_setup_params(SetupParamsOverrides::default()),
            build_setup_payload(SetupPayloadOverrides::default()),
        ) else {
            panic!("the setup succeeds");
        };
        let parameter_set = setup.parameter_set;
        let master_scalar = setup.master_scalar;
        let Ok(identity_element) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &parameter_set,
                identity: b"entitlement-one",
            },
        ) else {
            panic!("the identity derives");
        };
        let Ok(issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &parameter_set,
                master_scalar: &master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: uniform_draw::<P>(0x77),
            },
        ) else {
            panic!("the issuance succeeds");
        };

        let issue_refused = matches!(
            kem.issue(
                IssueParams,
                IssuePayload {
                    parameter_set: &parameter_set,
                    master_scalar: &master_scalar,
                    identity_element: &identity_element.identity_element,
                    uniform: short_draw(),
                },
            ),
            Err(IssueErrorReturn::Bb1DepthOne(
                Bb1DepthOneIssueErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: 64,
                        actual: 32,
                    }
                )
            ))
        );
        let rerandomize_refused = matches!(
            kem.rerandomize(
                RerandomizeParams,
                RerandomizePayload {
                    parameter_set: &parameter_set,
                    identity_element: &identity_element.identity_element,
                    credential: &issued.credential,
                    uniform: short_draw(),
                },
            ),
            Err(RerandomizeErrorReturn::Bb1DepthOne(
                Bb1DepthOneRerandomizeErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: 64,
                        actual: 32,
                    }
                )
            ))
        );
        let encapsulate_refused = matches!(
            kem.encapsulate(
                EncapsulateParams,
                EncapsulatePayload {
                    parameter_set: &parameter_set,
                    uniform: short_draw(),
                },
            ),
            Err(EncapsulateErrorReturn::Bb1DepthOne(
                Bb1DepthOneEncapsulateErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: 64,
                        actual: 32,
                    }
                )
            ))
        );

        SamplingOutcome {
            setup_master_refused,
            setup_base_refused,
            issue_refused,
            rerandomize_refused,
            encapsulate_refused,
        }
    }
}

struct DeclarationOutcome {
    identifier: KemIdentifier,
    identity_scopes: Vec<IdentityScope>,
    identity_tag: Vec<u8>,
    adapter_version: u32,
    interface_version: u32,
}

struct DeclarationProbe;

impl IPairingConsumer for DeclarationProbe {
    type Output = DeclarationOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        _payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let declaration = Bb1DepthOneKem::<'_, P>::DECLARATION;
        DeclarationOutcome {
            identifier: declaration.identifier,
            identity_scopes: declaration.identity_scopes.to_vec(),
            identity_tag: declaration.identity_tag.to_vec(),
            adapter_version: declaration.adapter_version,
            interface_version: declaration.interface_version,
        }
    }
}

/// Contract: a credential issued under the master scalar passes the public
///   validity check.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `issued_is_valid`.
#[test]
fn an_issued_credential_is_valid_for_its_identity() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.issued_is_valid);
}

/// Contract: a seller's rerandomization yields a valid credential for the same
///   identity (CR-08).
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `rerandomized_is_valid`.
#[test]
fn a_rerandomized_credential_is_valid_without_the_master_scalar() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rerandomized_is_valid);
}

/// Contract: a credential issued or rerandomized for one entitlement does not
///   validate for another (CR-08 non-convertibility).
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `issued_is_valid_for_other_identity` and
///   `rerandomized_is_valid_for_other_identity` are both `false`.
#[test]
fn a_credential_is_invalid_for_another_entitlement() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(!outcome.issued_is_valid_for_other_identity);
    assert!(!outcome.rerandomized_is_valid_for_other_identity);
}

/// Contract: a credential issued under one parameter set does not validate
///   under another.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `issued_is_valid_under_other_set` is `false`.
#[test]
fn a_credential_is_invalid_under_another_parameter_set() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(!outcome.issued_is_valid_under_other_set);
}

/// Contract: two issuances and a rerandomization for one identity all recover
///   `K` (CR-08 cross-holder agreement).
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `decapsulated_by_issued`, `decapsulated_by_second_issued`, and
///   `decapsulated_by_rerandomized` each equal `encapsulated`.
#[test]
fn every_credential_for_an_identity_decapsulates_the_encapsulated_value() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(outcome.decapsulated_by_issued, outcome.encapsulated);
    assert_eq!(outcome.decapsulated_by_second_issued, outcome.encapsulated);
    assert_eq!(outcome.decapsulated_by_rerandomized, outcome.encapsulated);
}

/// Contract: under the entitlement scope one capsule serves every holder of
///   every entitlement under the parameter set.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `decapsulated_by_other_entitlement` equals `encapsulated`.
#[test]
fn another_entitlements_credential_decapsulates_the_same_capsule_to_the_same_value() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.decapsulated_by_other_entitlement,
        outcome.encapsulated
    );
}

/// Contract: a capsule produced by `encapsulate` is well formed under its
///   parameter set.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `capsule_is_well_formed`.
#[test]
fn a_capsule_is_well_formed_under_its_parameter_set() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.capsule_is_well_formed);
}

/// Contract: a malformed capsule is refused (CR-08; EC-06).
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `swapped_capsule_is_well_formed` is `false`.
#[test]
fn a_capsule_with_its_identity_bases_exchanged_is_not_well_formed() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(!outcome.swapped_capsule_is_well_formed);
}

/// Contract: a capsule is not well formed under another parameter set.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `capsule_is_well_formed_under_other_set` is `false`.
#[test]
fn a_capsule_is_not_well_formed_under_another_parameter_set() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(!outcome.capsule_is_well_formed_under_other_set);
}

/// Contract: the construction holds on the BN254 arkworks concrete.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two on
///   `PairingConcrete::Bn254Arkworks`.
/// Act:     `create_pairing`.
/// Assert:  `issued_is_valid`, `rerandomized_is_valid`, and the three
///   decapsulations each equal `encapsulated`.
#[test]
fn every_credential_decapsulates_the_encapsulated_value_on_bn254_arkworks() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    assert!(outcome.issued_is_valid);
    assert!(outcome.rerandomized_is_valid);
    assert_eq!(outcome.decapsulated_by_issued, outcome.encapsulated);
    assert_eq!(outcome.decapsulated_by_rerandomized, outcome.encapsulated);
    assert_eq!(
        outcome.decapsulated_by_other_entitlement,
        outcome.encapsulated
    );
}

/// Contract: the construction holds on the BN254 halo2curves concrete.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two on
///   `PairingConcrete::Bn254Halo2curves`.
/// Act:     `create_pairing`.
/// Assert:  `issued_is_valid`, `rerandomized_is_valid`, and the three
///   decapsulations each equal `encapsulated`.
#[test]
fn every_credential_decapsulates_the_encapsulated_value_on_bn254_halo2curves() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bn254Halo2curves);

    // Assert
    assert!(outcome.issued_is_valid);
    assert!(outcome.rerandomized_is_valid);
    assert_eq!(outcome.decapsulated_by_issued, outcome.encapsulated);
    assert_eq!(outcome.decapsulated_by_rerandomized, outcome.encapsulated);
    assert_eq!(
        outcome.decapsulated_by_other_entitlement,
        outcome.encapsulated
    );
}

/// Contract: the construction holds on the BLS12-381 halo2curves concrete.
/// Arrange: `EntitlementScopeProbe` over entitlements one and two on
///   `PairingConcrete::Bls12381Halo2curves`.
/// Act:     `create_pairing`.
/// Assert:  `issued_is_valid`, `rerandomized_is_valid`, and the three
///   decapsulations each equal `encapsulated`.
#[test]
fn every_credential_decapsulates_the_encapsulated_value_on_bls12_381_halo2curves() {
    // Arrange
    let probe = EntitlementScopeProbe {
        identity: b"entitlement-one".to_vec(),
        other_identity: b"entitlement-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Halo2curves);

    // Assert
    assert!(outcome.issued_is_valid);
    assert!(outcome.rerandomized_is_valid);
    assert_eq!(outcome.decapsulated_by_issued, outcome.encapsulated);
    assert_eq!(outcome.decapsulated_by_rerandomized, outcome.encapsulated);
    assert_eq!(
        outcome.decapsulated_by_other_entitlement,
        outcome.encapsulated
    );
}

/// Contract: the asset scope encapsulates to `(U, V)`.
/// Arrange: `AssetScopeProbe` over assets one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `capsule_is_asset_form`.
#[test]
fn an_asset_scope_capsule_has_two_elements() {
    // Arrange
    let probe = AssetScopeProbe {
        asset_identity: b"asset-one".to_vec(),
        other_identity: b"asset-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.capsule_is_asset_form);
}

/// Contract: an asset-scope capsule is well formed under its parameter set.
/// Arrange: `AssetScopeProbe` over assets one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `capsule_is_well_formed`.
#[test]
fn an_asset_scope_capsule_is_well_formed_under_its_parameter_set() {
    // Arrange
    let probe = AssetScopeProbe {
        asset_identity: b"asset-one".to_vec(),
        other_identity: b"asset-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.capsule_is_well_formed);
}

/// Contract: under the asset scope a holder's rerandomization of its own
///   credential is a credential for a new entitlement of the asset (CD-08).
/// Arrange: `AssetScopeProbe` over assets one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `issued_is_valid` and `holder_authored_is_valid`.
#[test]
fn a_holder_authors_a_valid_asset_scope_credential_without_the_master_scalar() {
    // Arrange
    let probe = AssetScopeProbe {
        asset_identity: b"asset-one".to_vec(),
        other_identity: b"asset-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.issued_is_valid);
    assert!(outcome.holder_authored_is_valid);
}

/// Contract: the issued and the holder-authored asset-scope credentials each
///   recover `K`.
/// Arrange: `AssetScopeProbe` over assets one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `decapsulated_by_issued` and `decapsulated_by_holder_authored`
///   each equal `encapsulated`.
#[test]
fn every_asset_scope_credential_decapsulates_the_encapsulated_value() {
    // Arrange
    let probe = AssetScopeProbe {
        asset_identity: b"asset-one".to_vec(),
        other_identity: b"asset-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(outcome.decapsulated_by_issued, outcome.encapsulated);
    assert_eq!(
        outcome.decapsulated_by_holder_authored,
        outcome.encapsulated
    );
}

/// Contract: an identity other than the asset's is outside the scope.
/// Arrange: `AssetScopeProbe` over assets one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `other_identity_is_outside_the_scope`.
#[test]
fn derive_identity_refuses_an_identity_outside_the_asset_scope() {
    // Arrange
    let probe = AssetScopeProbe {
        asset_identity: b"asset-one".to_vec(),
        other_identity: b"asset-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.other_identity_is_outside_the_scope);
}

/// Contract: the asset scope's fixed `F` is the element its asset identity
///   derives.
/// Arrange: `AssetScopeProbe` over assets one and two.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `set_carries_the_derived_identity_element`.
#[test]
fn an_asset_scope_parameter_set_carries_the_derived_identity_element() {
    // Arrange
    let probe = AssetScopeProbe {
        asset_identity: b"asset-one".to_vec(),
        other_identity: b"asset-two".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.set_carries_the_derived_identity_element);
}

/// Contract: an identity whose element is the identity is refused before any
///   issuance (CR-08; LC-08).
/// Arrange: `TrivialIdentityProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  the output is `true`.
#[test]
fn derive_identity_refuses_a_trivial_identity_element() {
    // Arrange
    let probe = TrivialIdentityProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let refused = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(refused);
}

/// Contract: the components the delivery proof states hold `hpub = α·g2` for
///   the `α` the mint proof takes as its witness.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `hpub_is_the_master_scalar_times_g2`.
#[test]
fn the_parameter_set_components_bind_hpub_to_the_master_scalar() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.hpub_is_the_master_scalar_times_g2);
}

/// Contract: the identity element's components are the mapped scalar and its
///   element.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `identity_element_is_u0_plus_i_times_u1`.
#[test]
fn the_identity_element_components_are_the_mapped_scalar_and_its_element() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.identity_element_is_u0_plus_i_times_u1);
}

/// Contract: the scalar the identity mapping yields is the hash-to-scalar
///   mapping of the identity bytes under the tag the declaration carries, so a
///   contract hashing under the mirrored tag recomputes it (CR-11).
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `identity_scalar_is_the_declared_tags_hash`.
#[test]
fn the_identity_mapping_hashes_under_the_declared_tag() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.identity_scalar_is_the_declared_tags_hash);
}

/// Contract: the returned `r` is the mint proof's witness for the issued
///   `(A, B)`.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `issued_a_is_alpha_g1_plus_r_f` and `issued_b_is_r_g2`.
#[test]
fn issuance_returns_the_randomness_its_credential_was_formed_with() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.issued_a_is_alpha_g1_plus_r_f);
    assert!(outcome.issued_b_is_r_g2);
}

/// Contract: the returned `s` is the transfer proof's witness for the new
///   credential.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `rerandomized_b_is_b_plus_s_g2`.
#[test]
fn rerandomization_returns_the_offset_it_applied() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rerandomized_b_is_b_plus_s_g2);
}

/// Contract: the credential engine's rebuild of a decrypted `(A, B)` is the
///   credential.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `rebuilt_credential_is_valid`.
#[test]
fn a_credential_rebuilt_from_its_components_is_valid() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rebuilt_credential_is_valid);
}

/// Contract: a parameter set rebuilt from its components validates its
///   credentials.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `credential_is_valid_under_rebuilt_set`.
#[test]
fn a_parameter_set_rebuilt_from_its_components_validates_its_credentials() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.credential_is_valid_under_rebuilt_set);
}

/// Contract: a capsule rebuilt from its components is well formed and
///   decapsulates to `encapsulate`'s bytes.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `rebuilt_capsule_is_well_formed` and
///   `rebuilt_capsule_decapsulates_the_encapsulated_value`.
#[test]
fn a_capsule_rebuilt_from_its_components_is_well_formed_and_decapsulates() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rebuilt_capsule_is_well_formed);
    assert!(outcome.rebuilt_capsule_decapsulates_the_encapsulated_value);
}

/// Contract: custody's restore of `α` is the master scalar.
/// Arrange: `ComponentsProbe` over entitlement one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `restored_master_scalar_issues_a_valid_credential`.
#[test]
fn a_master_scalar_restored_from_its_components_issues_valid_credentials() {
    // Arrange
    let probe = ComponentsProbe {
        identity: b"entitlement-one".to_vec(),
    };

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.restored_master_scalar_issues_a_valid_credential);
}

/// Contract: a sampling refusal returns in the method's own variant unchanged.
/// Arrange: `SamplingProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `setup_master_refused`.
#[test]
fn setup_refuses_a_master_draw_of_the_wrong_length() {
    // Arrange
    let probe = SamplingProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.setup_master_refused);
}

/// Contract: a sampling refusal returns in the method's own variant unchanged.
/// Arrange: `SamplingProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `setup_base_refused`.
#[test]
fn setup_refuses_an_identity_base_draw_of_the_wrong_length() {
    // Arrange
    let probe = SamplingProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.setup_base_refused);
}

/// Contract: a sampling refusal returns in the method's own variant unchanged.
/// Arrange: `SamplingProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `issue_refused`.
#[test]
fn issue_refuses_a_draw_of_the_wrong_length() {
    // Arrange
    let probe = SamplingProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.issue_refused);
}

/// Contract: a sampling refusal returns in the method's own variant unchanged.
/// Arrange: `SamplingProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `rerandomize_refused`.
#[test]
fn rerandomize_refuses_a_draw_of_the_wrong_length() {
    // Arrange
    let probe = SamplingProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rerandomize_refused);
}

/// Contract: a sampling refusal returns in the method's own variant unchanged.
/// Arrange: `SamplingProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `encapsulate_refused`.
#[test]
fn encapsulate_refuses_a_draw_of_the_wrong_length() {
    // Arrange
    let probe = SamplingProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.encapsulate_refused);
}

/// Contract: the concrete's declaration names the KEM identifier, both
///   identity scopes, its adapter version, and the interface version it
///   implements (CD-08).
/// Arrange: `DeclarationProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`, reading
///   `Bb1DepthOneKem::<'_, P>::DECLARATION` inside the probe.
/// Assert:  `identifier` matches `KemIdentifier::Bb1DepthOneV1`,
///   `identity_scopes` equals `[IdentityScope::Entitlement, IdentityScope::Asset]`
///   as a slice, `identity_tag` equals `b"ChainTorrent-v1-kem-identity"`,
///   `adapter_version` equals `1`, and `interface_version` equals
///   `KEM_INTERFACE_VERSION`.
#[test]
fn bb1_depth_one_kem_declares_its_identifier_scopes_and_versions() {
    // Arrange
    let probe = DeclarationProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(matches!(outcome.identifier, KemIdentifier::Bb1DepthOneV1));
    assert_eq!(
        outcome.identity_scopes,
        vec![IdentityScope::Entitlement, IdentityScope::Asset]
    );
    assert_eq!(
        outcome.identity_tag,
        b"ChainTorrent-v1-kem-identity".to_vec()
    );
    assert_eq!(outcome.adapter_version, 1);
    assert_eq!(outcome.interface_version, KEM_INTERFACE_VERSION);
}
