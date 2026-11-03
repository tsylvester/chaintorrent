#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::build_secret;
use envelope::{
    GenerateKeysPayloadOverrides, IKeyAgreementAdapter, build_envelope_components_params,
    build_envelope_components_payload, build_generate_keys_params, build_generate_keys_payload,
    build_key_pair_public_keys_params, build_key_pair_public_keys_payload,
};
use hashing::build_root;
use kem::{
    CreateKemParamsOverrides, DeriveIdentityPayloadOverrides, ICredentialKemAdapter,
    IdentityScope, IssuePayloadOverrides, ParameterSetScopeComponents, SetupParamsOverrides,
    SetupPayloadOverrides, build_create_kem_params, build_credential_components_params,
    build_credential_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params,
    MulG2Payload,
};
use proof::{IDeliveryProofAdapter, MintPurpose, build_delivery_context};
use random::IRandomSourceAdapter;

use super::interface::{AdmittedKemIdentity, IMintCaseConsumer};
use super::mint;
use super::mock::{
    MintCaseParamsOverrides, MintPayloadOverrides, build_mint_case_deps, build_mint_case_params,
    build_mint_case_payload, build_mint_deps, build_mint_params, build_mint_payload, run_mint_case,
};

/// Contract: Computed → `Ok(MintSuccessReturn { delivery })`, the statement's
///   context the payload's context, its `buyer_keys` the recipient's
///   public-key components, its `envelope` the recipient envelope's
///   components, its `hpub` the parameter set's, and its `identity_element`
///   the identity element's.
/// Arrange: a context from `build_delivery_context` with distinct values at
///   every field, so a statement that echoes another context reports
///   otherwise.
/// Act:     `mint`.
/// Assert:  the statement's context fields equal the payload context's, field
///   by field; `key_pair_public_keys` over `delivery.recipient.key_pair`
///   equals `delivery.statement.buyer_keys`; `envelope_components` over
///   `delivery.recipient.envelope` equals `delivery.statement.envelope`;
///   `parameter_set_components` over `delivery.parameter_set` holds
///   `delivery.statement.hpub`; `identity_element_components` over
///   `delivery.identity_element` holds `delivery.statement.identity_element`,
///   every element compared through `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn mint_returns_a_statement_over_the_payloads_context_and_the_recipients_keys() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
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
                        identity: Some(identity),
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
                delivery.statement.context.asset_root,
                context.asset_root
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
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: &P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let recipient_public_keys = key_agreement
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(
                        &delivery.recipient.key_pair,
                        Default::default(),
                    ),
                )
                .expect("the key pair's public keys are infallible")
                .components;
            assert_eq!(
                encode_g1(&recipient_public_keys.pk1),
                encode_g1(&delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&recipient_public_keys.pk2),
                encode_g2(&delivery.statement.buyer_keys.pk2)
            );
            let recipient_envelope = key_agreement
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(
                        &delivery.recipient.envelope,
                        Default::default(),
                    ),
                )
                .expect("the envelope's components are infallible")
                .components;
            assert_eq!(
                encode_g1(&recipient_envelope.c1),
                encode_g1(&delivery.statement.envelope.c1)
            );
            assert_eq!(
                encode_g1(&recipient_envelope.c2),
                encode_g1(&delivery.statement.envelope.c2)
            );
            assert_eq!(
                encode_g2(&recipient_envelope.d1),
                encode_g2(&delivery.statement.envelope.d1)
            );
            assert_eq!(
                encode_g2(&recipient_envelope.d2),
                encode_g2(&delivery.statement.envelope.d2)
            );
            let parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &delivery.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            assert_eq!(
                encode_g2(&parameter_set.hpub),
                encode_g2(&delivery.statement.hpub)
            );
            let identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &delivery.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            assert_eq!(
                encode_g1(&identity_element.element),
                encode_g1(&delivery.statement.identity_element)
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(Default::default()),
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_a_statement_over_the_payloads_context_and_the_recipients_keys
}

/// Contract: Ordering → the draws are the six uniforms master, `u0`, `u1`,
///   issuance, recipient `x`, recipient `y`, concatenated, each consumed by
///   its call.
/// Arrange: the randomness family's mock concrete at its defaults, whose
///   successive draws differ, so a uniform handed to another call forms
///   another object.
/// Act:     `mint`, then `setup`, `issue`, and `generate_keys` replayed on
///   the handed adapters over the draw slices.
/// Assert:  `delivery.draws` is six slices of `UNIFORM_BYTES_LENGTH` bytes;
///   `setup` on the handed KEM under the identity's setup scope over the
///   first, second, and third slices as `master_uniform`, `u0_uniform`, and
///   `u1_uniform` returns a parameter set whose `parameter_set_components`
///   equal those of `delivery.parameter_set`; `issue` under that set and its
///   master scalar, over `delivery.identity_element` and the fourth slice,
///   returns a credential whose `credential_components` equal those of
///   `delivery.credential`; `generate_keys` over the fifth and sixth slices
///   as `x_uniform` and `y_uniform` returns a key pair whose
///   `key_pair_public_keys` equal `delivery.statement.buyer_keys`; every
///   element compared through `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn mint_draws_six_uniforms_in_order() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
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
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(delivery.draws.len(), 6 * length);
            let uniform_at = |index: usize| {
                build_secret(
                    delivery.draws[index * length..(index + 1) * length].to_vec(),
                    Default::default(),
                )
            };
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: &P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let replayed_setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(uniform_at(0)),
                        u0_uniform: Some(uniform_at(1)),
                        u1_uniform: Some(uniform_at(2)),
                    }),
                )
                .expect("the replay's setup succeeds");
            let replayed_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &replayed_setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the replayed set's components are infallible")
                .components;
            let delivered_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &delivery.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the delivery's set's components are infallible")
                .components;
            assert_eq!(encode_g1(&replayed_set.g1), encode_g1(&delivered_set.g1));
            assert_eq!(encode_g1(&replayed_set.u0), encode_g1(&delivered_set.u0));
            assert_eq!(encode_g1(&replayed_set.u1), encode_g1(&delivered_set.u1));
            assert_eq!(encode_g2(&replayed_set.g2), encode_g2(&delivered_set.g2));
            assert_eq!(
                encode_g2(&replayed_set.hpub),
                encode_g2(&delivered_set.hpub)
            );
            assert_eq!(
                std::mem::discriminant(&replayed_set.scope),
                std::mem::discriminant(&delivered_set.scope)
            );
            if let (
                ParameterSetScopeComponents::Asset {
                    identity_element: replayed_element,
                },
                ParameterSetScopeComponents::Asset {
                    identity_element: delivered_element,
                },
            ) = (&replayed_set.scope, &delivered_set.scope)
            {
                assert_eq!(encode_g1(replayed_element), encode_g1(delivered_element));
            }
            let replayed_issue = kem
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &replayed_setup.parameter_set,
                        &replayed_setup.master_scalar,
                        &delivery.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(uniform_at(3)),
                        },
                    ),
                )
                .expect("the replay's issuance succeeds");
            let replayed_credential = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(
                        &replayed_issue.credential,
                        Default::default(),
                    ),
                )
                .expect("the replayed credential's components are infallible")
                .components;
            let delivered_credential = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(
                        &delivery.credential,
                        Default::default(),
                    ),
                )
                .expect("the delivery's credential's components are infallible")
                .components;
            assert_eq!(
                encode_g1(&replayed_credential.a),
                encode_g1(&delivered_credential.a)
            );
            assert_eq!(
                encode_g2(&replayed_credential.b),
                encode_g2(&delivered_credential.b)
            );
            let replayed_keys = key_agreement
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(uniform_at(4)),
                            y_uniform: Some(uniform_at(5)),
                        },
                    ),
                )
                .expect("the replay's key generation succeeds");
            let replayed_public_keys = key_agreement
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(
                        &replayed_keys.key_pair,
                        Default::default(),
                    ),
                )
                .expect("the replayed key pair's public keys are infallible")
                .components;
            assert_eq!(
                encode_g1(&replayed_public_keys.pk1),
                encode_g1(&delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&replayed_public_keys.pk2),
                encode_g2(&delivery.statement.buyer_keys.pk2)
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(Default::default()),
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_draws_six_uniforms_in_order
}

/// Contract: Setup and Derivation → `setup` receives the identity's setup
///   scope and `derive_identity` its identity.
/// Arrange: an asset root from `build_root(Default::default())` handed to
///   `build_mint_case_payload` as `identity_hash`; the KEM selection under
///   `IdentityScope::Asset` through `CreateKemParamsOverrides`, so the runner
///   produces the identity `SetupScope::Asset` and `KemIdentity::Asset` form,
///   each over that hash.
/// Act:     `mint`, then `derive_identity` on the handed KEM over
///   `&delivery.parameter_set` and `KemIdentity::Asset` of that hash.
/// Assert:  `parameter_set_components` over `delivery.parameter_set` holds a
///   `ParameterSetScopeComponents::Asset` scope; the `element` of
///   `identity_element_components` over the derived identity element and
///   over `delivery.identity_element` encode equal through `encode_g1`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn mint_hands_the_admitted_identity_to_setup_and_derivation() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
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
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &delivery.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(identity.identity),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            // Assert
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &delivery.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            assert!(matches!(
                parameter_set.scope,
                ParameterSetScopeComponents::Asset { .. }
            ));
            let derived_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the derived identity element's components are infallible")
                .components;
            let delivered_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &delivery.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the delivery's identity element's components are infallible")
                .components;
            assert_eq!(
                encode_g1(&derived_element.element),
                encode_g1(&delivered_element.element)
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(MintCaseParamsOverrides {
                kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(IdentityScope::Asset),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
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
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn mint_returns_the_witnesses_the_objects_were_formed_with() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
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
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: &P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &delivery.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            let Ok(expected_hpub) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: delivery.witnesses.master_scalar.value.expose().clone(),
                },
            );
            assert_eq!(
                encode_g2(&parameter_set.hpub),
                encode_g2(&expected_hpub.product)
            );
            let credential = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&delivery.credential, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            let Ok(expected_b) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: delivery.witnesses.credential_randomness.expose().clone(),
                },
            );
            assert_eq!(
                encode_g2(&credential.b),
                encode_g2(&expected_b.product)
            );
            let envelope = key_agreement
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(
                        &delivery.recipient.envelope,
                        Default::default(),
                    ),
                )
                .expect("the envelope's components are infallible")
                .components;
            let Ok(expected_c1) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: delivery.witnesses.coins.rho.expose().clone(),
                },
            );
            assert_eq!(
                encode_g1(&envelope.c1),
                encode_g1(&expected_c1.product)
            );
            let Ok(expected_d1) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.coins.sigma.expose().clone(),
                },
            );
            assert_eq!(
                encode_g2(&envelope.d1),
                encode_g2(&expected_d1.product)
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(Default::default()),
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_the_witnesses_the_objects_were_formed_with
}
