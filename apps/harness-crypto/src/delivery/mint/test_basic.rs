#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{Secret, SecretConstructorParams, build_asset_identity_hash};
use envelope::{
    EnvelopeComponentsParams, EnvelopeComponentsPayload, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
};
use kem::{
    CreateKemParamsOverrides, CredentialComponentsParams, CredentialComponentsPayload,
    DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter,
    IdentityElementComponentsParams, IdentityElementComponentsPayload, IdentityScope, IssueParams,
    IssuePayload, KemIdentity, ParameterSetComponentsParams, ParameterSetComponentsPayload,
    ParameterSetScopeComponents, SetupParams, SetupPayload, SetupScope, build_create_kem_params,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
};
use proof::{IDeliveryProofAdapter, MintPurpose, build_delivery_context};
use random::IRandomSourceAdapter;

use super::interface::AdmittedKemIdentity;
use super::mint;
use super::mock::{
    AdmittedKemIdentityOverrides, IMintCaseConsumer, MintCaseParamsOverrides,
    MintPayloadOverrides, build_admitted_kem_identity, build_mint_case_params, build_mint_deps,
    build_mint_params, build_mint_payload, run_mint_case,
};

/// Contract: Computed → `Ok(MintSuccessReturn { delivery })`, the statement's
///   context the payload's context, its `buyer_keys` the recipient's
///   public-key components, its `envelope` the recipient envelope's
///   components, its `hpub` the parameter set's, and its `identity_element`
///   the identity element's.
/// Arrange: a context from `build_delivery_context` with distinct values at
///   every field.
/// Act:     `mint`.
/// Assert:  the statement's context fields equal the payload context's, field
///   by field; `key_pair_public_keys` over `delivery.recipient.key_pair`
///   equals `delivery.statement.buyer_keys`; `envelope_components` over
///   `delivery.recipient.envelope` equals `delivery.statement.envelope`;
///   `parameter_set_components` over `delivery.parameter_set` holds
///   `delivery.statement.hpub`; `identity_element_components` over
///   `delivery.identity_element` holds `delivery.statement.identity_element`,
///   every element compared through `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn mint_returns_a_statement_over_the_payloads_context_and_the_recipients_keys() {
    // Arrange
    let identity = build_admitted_kem_identity(Default::default());

    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(self.identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            assert_eq!(
                delivery.statement.context.suite_identifier,
                context.suite_identifier
            );
            assert_eq!(delivery.statement.context.chain, context.chain);
            assert_eq!(
                delivery.statement.context.entitlement_contract,
                context.entitlement_contract
            );
            assert_eq!(
                delivery.statement.context.asset_identity_hash,
                context.asset_identity_hash
            );
            assert_eq!(
                delivery.statement.context.parameter_set_identifier,
                context.parameter_set_identifier
            );
            assert_eq!(
                delivery.statement.context.source_entitlement,
                context.source_entitlement
            );
            assert_eq!(
                delivery.statement.context.target_entitlement,
                context.target_entitlement
            );
            assert_eq!(
                delivery.statement.context.old_interval,
                context.old_interval
            );
            assert_eq!(
                delivery.statement.context.new_interval,
                context.new_interval
            );
            assert_eq!(delivery.statement.context.purpose, context.purpose);
            assert_eq!(delivery.statement.context.seller, context.seller);
            assert_eq!(delivery.statement.context.buyer, context.buyer);
            assert_eq!(delivery.statement.context.expiry, context.expiry);
            let Ok(recipient_keys) = key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &delivery.recipient.key_pair,
                },
            );
            let Ok(recipient_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: recipient_keys.components.pk1,
                },
            );
            let Ok(statement_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.buyer_keys.pk1.clone(),
                },
            );
            assert_eq!(recipient_pk1.bytes.as_ref(), statement_pk1.bytes.as_ref());
            let Ok(recipient_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: recipient_keys.components.pk2,
                },
            );
            let Ok(statement_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.buyer_keys.pk2.clone(),
                },
            );
            assert_eq!(recipient_pk2.bytes.as_ref(), statement_pk2.bytes.as_ref());
            let Ok(recipient_envelope) = key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &delivery.recipient.envelope,
                },
            );
            let Ok(recipient_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: recipient_envelope.components.c1,
                },
            );
            let Ok(statement_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.envelope.c1.clone(),
                },
            );
            assert_eq!(recipient_c1.bytes.as_ref(), statement_c1.bytes.as_ref());
            let Ok(recipient_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: recipient_envelope.components.c2,
                },
            );
            let Ok(statement_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.envelope.c2.clone(),
                },
            );
            assert_eq!(recipient_c2.bytes.as_ref(), statement_c2.bytes.as_ref());
            let Ok(recipient_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: recipient_envelope.components.d1,
                },
            );
            let Ok(statement_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.envelope.d1.clone(),
                },
            );
            assert_eq!(recipient_d1.bytes.as_ref(), statement_d1.bytes.as_ref());
            let Ok(recipient_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: recipient_envelope.components.d2,
                },
            );
            let Ok(statement_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.envelope.d2.clone(),
                },
            );
            assert_eq!(recipient_d2.bytes.as_ref(), statement_d2.bytes.as_ref());
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(set_hpub) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: parameter_set.components.hpub,
                },
            );
            let Ok(statement_hpub) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.hpub.clone(),
                },
            );
            assert_eq!(set_hpub.bytes.as_ref(), statement_hpub.bytes.as_ref());
            let Ok(identity_element) = kem.identity_element_components(
                IdentityElementComponentsParams,
                IdentityElementComponentsPayload {
                    identity_element: &delivery.identity_element,
                },
            );
            let Ok(element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: identity_element.components.element,
                },
            );
            let Ok(statement_element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.identity_element.clone(),
                },
            );
            assert_eq!(element.bytes.as_ref(), statement_element.bytes.as_ref());
        }
    }

    run_mint_case(
        build_mint_case_params(Default::default()),
        CaseConsumer { identity },
    );
    // mint_returns_a_statement_over_the_payloads_context_and_the_recipients_keys
}

/// Contract: Ordering → the draws are the six uniforms master, `u0`, `u1`,
///   issuance, recipient `x`, recipient `y`, concatenated, each consumed by
///   its call.
/// Arrange: the randomness family's mock concrete at its defaults, whose
///   successive draws differ, so a uniform handed to another call forms
///   another object.
/// Act:     `mint`.
/// Assert:  `delivery.draws` is six slices of `UNIFORM_BYTES_LENGTH` bytes;
///   `setup` on the handed KEM under the identity's setup scope over the
///   first, second, and third slices returns a parameter set whose
///   `parameter_set_components` equal those of `delivery.parameter_set`;
///   `issue` under that set and its master scalar, over
///   `delivery.identity_element` and the fourth slice, returns a credential
///   whose `credential_components` equal those of `delivery.credential`;
///   `generate_keys` over the fifth and sixth slices returns a key pair whose
///   `key_pair_public_keys` equal `delivery.statement.buyer_keys`; every
///   element compared through `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn mint_draws_six_uniforms_in_order() {
    // Arrange
    let identity = build_admitted_kem_identity(Default::default());

    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(self.identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(delivery.draws.len(), 6 * length);
            let uniform = |index: usize| {
                let Ok(secret) = Secret::try_new(SecretConstructorParams {
                    value: delivery.draws[index * length..(index + 1) * length].to_vec(),
                });
                secret
            };
            let replayed_setup = kem
                .setup(
                    SetupParams {
                        scope: self.identity.setup_scope,
                    },
                    SetupPayload {
                        master_uniform: uniform(0),
                        u0_uniform: uniform(1),
                        u1_uniform: uniform(2),
                    },
                )
                .expect("the replayed setup succeeds");
            let Ok(replayed_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &replayed_setup.parameter_set,
                },
            );
            let Ok(delivered_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(replayed_g1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: replayed_set.components.g1,
                },
            );
            let Ok(delivered_g1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivered_set.components.g1.clone(),
                },
            );
            assert_eq!(replayed_g1.bytes.as_ref(), delivered_g1.bytes.as_ref());
            let Ok(replayed_u0) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: replayed_set.components.u0,
                },
            );
            let Ok(delivered_u0) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivered_set.components.u0.clone(),
                },
            );
            assert_eq!(replayed_u0.bytes.as_ref(), delivered_u0.bytes.as_ref());
            let Ok(replayed_u1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: replayed_set.components.u1,
                },
            );
            let Ok(delivered_u1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivered_set.components.u1.clone(),
                },
            );
            assert_eq!(replayed_u1.bytes.as_ref(), delivered_u1.bytes.as_ref());
            let Ok(replayed_g2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: replayed_set.components.g2,
                },
            );
            let Ok(delivered_g2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered_set.components.g2.clone(),
                },
            );
            assert_eq!(replayed_g2.bytes.as_ref(), delivered_g2.bytes.as_ref());
            let Ok(replayed_hpub) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: replayed_set.components.hpub,
                },
            );
            let Ok(delivered_hpub) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered_set.components.hpub.clone(),
                },
            );
            assert_eq!(replayed_hpub.bytes.as_ref(), delivered_hpub.bytes.as_ref());
            assert!(matches!(
                replayed_set.components.scope,
                ParameterSetScopeComponents::Entitlement
            ));
            assert!(matches!(
                delivered_set.components.scope,
                ParameterSetScopeComponents::Entitlement
            ));
            let replayed_issued = kem
                .issue(
                    IssueParams,
                    IssuePayload {
                        parameter_set: &replayed_setup.parameter_set,
                        master_scalar: &replayed_setup.master_scalar,
                        identity_element: &delivery.identity_element,
                        uniform: uniform(3),
                    },
                )
                .expect("the replayed issuance succeeds");
            let Ok(replayed_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &replayed_issued.credential,
                },
            );
            let Ok(delivered_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(replayed_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: replayed_credential.components.a,
                },
            );
            let Ok(delivered_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivered_credential.components.a.clone(),
                },
            );
            assert_eq!(replayed_a.bytes.as_ref(), delivered_a.bytes.as_ref());
            let Ok(replayed_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: replayed_credential.components.b,
                },
            );
            let Ok(delivered_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered_credential.components.b.clone(),
                },
            );
            assert_eq!(replayed_b.bytes.as_ref(), delivered_b.bytes.as_ref());
            let replayed_keys = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: uniform(4),
                        y_uniform: uniform(5),
                    },
                )
                .expect("the replayed key generation succeeds");
            let Ok(replayed_public) = key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &replayed_keys.key_pair,
                },
            );
            let Ok(replayed_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: replayed_public.components.pk1,
                },
            );
            let Ok(delivered_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.buyer_keys.pk1.clone(),
                },
            );
            assert_eq!(replayed_pk1.bytes.as_ref(), delivered_pk1.bytes.as_ref());
            let Ok(replayed_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: replayed_public.components.pk2,
                },
            );
            let Ok(delivered_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.buyer_keys.pk2.clone(),
                },
            );
            assert_eq!(replayed_pk2.bytes.as_ref(), delivered_pk2.bytes.as_ref());
        }
    }

    run_mint_case(
        build_mint_case_params(Default::default()),
        CaseConsumer { identity },
    );
    // mint_draws_six_uniforms_in_order
}

/// Contract: Setup and Derivation → `setup` receives the identity's setup
///   scope and `derive_identity` its identity.
/// Arrange: an asset identity hash from `build_asset_identity_hash`;
///   `build_admitted_kem_identity` overriding `setup_scope` with
///   `SetupScope::Asset` and `identity` with `KemIdentity::Asset`, each over
///   that hash, against the builder's default entitlement scope; the KEM
///   selection under `IdentityScope::Asset` through
///   `CreateKemParamsOverrides`.
/// Act:     `mint`, then `derive_identity` on the handed KEM over
///   `&delivery.parameter_set` and `KemIdentity::Asset` of that hash.
/// Assert:  `parameter_set_components` over `delivery.parameter_set` holds a
///   `ParameterSetScopeComponents::Asset` scope; the `element` of
///   `identity_element_components` over the derived identity element and
///   over `delivery.identity_element` encode equal through `encode_g1`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn mint_hands_the_admitted_identity_to_setup_and_derivation() {
    // Arrange
    let hash = build_asset_identity_hash(Default::default());
    let identity = build_admitted_kem_identity(AdmittedKemIdentityOverrides {
        setup_scope: Some(SetupScope::Asset {
            identity_hash: &hash,
        }),
        identity: Some(KemIdentity::Asset {
            identity_hash: &hash,
        }),
    });

    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(self.identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            let derived = kem
                .derive_identity(
                    DeriveIdentityParams,
                    DeriveIdentityPayload {
                        parameter_set: &delivery.parameter_set,
                        identity: self.identity.identity,
                    },
                )
                .expect("the admitted asset identity derives");
            // Assert
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            assert!(matches!(
                parameter_set.components.scope,
                ParameterSetScopeComponents::Asset { .. }
            ));
            let Ok(derived_element) = kem.identity_element_components(
                IdentityElementComponentsParams,
                IdentityElementComponentsPayload {
                    identity_element: &derived.identity_element,
                },
            );
            let Ok(derived_encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: derived_element.components.element,
                },
            );
            let Ok(delivered_element) = kem.identity_element_components(
                IdentityElementComponentsParams,
                IdentityElementComponentsPayload {
                    identity_element: &delivery.identity_element,
                },
            );
            let Ok(delivered_encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivered_element.components.element,
                },
            );
            assert_eq!(
                derived_encoded.bytes.as_ref(),
                delivered_encoded.bytes.as_ref()
            );
        }
    }

    run_mint_case(
        build_mint_case_params(MintCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        CaseConsumer { identity },
    );
    // mint_hands_the_admitted_identity_to_setup_and_derivation
}

/// Contract: Witnesses → the witnesses are copies of the master scalar, the
///   issuance randomness, and the wrap's coins the delivery's parameter set,
///   credential, and envelope were formed with.
/// Arrange: every family's mock concrete at its defaults, each answering its
///   family's contract entries, so each relation below holds for the values
///   the delivery's objects were formed with and for no other.
/// Act:     `mint`.
/// Assert:  through the handed pairing, every element encoded through
///   `encode_g1` or `encode_g2`: the `hpub` of `parameter_set_components`
///   over `delivery.parameter_set` equals `mul_g2` of `g2_generator` by
///   `delivery.witnesses.master_scalar.value`; the `b` of
///   `credential_components` over `delivery.credential` equals `mul_g2` of
///   `g2_generator` by `delivery.witnesses.credential_randomness`; the `c1`
///   of `envelope_components` over `delivery.recipient.envelope` equals
///   `mul_g1` of `g1_generator` by `delivery.witnesses.coins.rho`, and its
///   `d1` equals `mul_g2` of `g2_generator` by
///   `delivery.witnesses.coins.sigma`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn mint_returns_the_witnesses_the_objects_were_formed_with() {
    // Arrange
    let identity = build_admitted_kem_identity(Default::default());

    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(self.identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(delivered_hpub) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: parameter_set.components.hpub,
                },
            );
            let Ok(witnessed_hpub) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: delivery.witnesses.master_scalar.value.expose().clone(),
                },
            );
            let Ok(witnessed_hpub) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: witnessed_hpub.product,
                },
            );
            assert_eq!(
                delivered_hpub.bytes.as_ref(),
                witnessed_hpub.bytes.as_ref()
            );
            let Ok(credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(delivered_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: credential.components.b,
                },
            );
            let Ok(witnessed_b) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: delivery.witnesses.credential_randomness.expose().clone(),
                },
            );
            let Ok(witnessed_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: witnessed_b.product,
                },
            );
            assert_eq!(delivered_b.bytes.as_ref(), witnessed_b.bytes.as_ref());
            let Ok(envelope) = key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &delivery.recipient.envelope,
                },
            );
            let Ok(delivered_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: envelope.components.c1,
                },
            );
            let Ok(witnessed_c1) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: delivery.witnesses.coins.rho.expose().clone(),
                },
            );
            let Ok(witnessed_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: witnessed_c1.product,
                },
            );
            assert_eq!(delivered_c1.bytes.as_ref(), witnessed_c1.bytes.as_ref());
            let Ok(delivered_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: envelope.components.d1,
                },
            );
            let Ok(witnessed_d1) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.coins.sigma.expose().clone(),
                },
            );
            let Ok(witnessed_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: witnessed_d1.product,
                },
            );
            assert_eq!(delivered_d1.bytes.as_ref(), witnessed_d1.bytes.as_ref());
        }
    }

    run_mint_case(
        build_mint_case_params(Default::default()),
        CaseConsumer { identity },
    );
    // mint_returns_the_witnesses_the_objects_were_formed_with
}
