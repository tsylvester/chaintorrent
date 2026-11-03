#![allow(clippy::expect_used)]

use domain::{
    DeploymentIdentityConstructorParamsOverrides, GroupIndexConstructorParamsOverrides,
    ParameterSetIdentifierConstructorParamsOverrides, PieceGeometryConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_deployment_identity, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_suite_identifier,
};
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kdf::IKeyDerivationAdapter;
use kem::{
    CapsuleComponents, CreateKemParamsOverrides, DeriveIdentityPayloadOverrides,
    DeriveIdentitySuccessReturn, EncapsulatePayloadOverrides, EncapsulateSuccessReturn,
    ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides, IssueSuccessReturn, KemIdentity,
    SetupParamsOverrides, SetupPayloadOverrides, SetupScope, SetupSuccessReturn,
    build_capsule_components_params, build_capsule_components_payload, build_create_kem_params,
    build_credential_components_params, build_credential_components_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, build_fill_bytes_payload,
};
use workflows::{
    DerivationContext, DerivationContextConstructorParamsOverrides, PIECE_GROUP_KEY_LENGTH,
    PieceGroupKeyConstructorParamsOverrides, WrapPieceGroupKeySuccessReturn,
    build_derivation_context, build_piece_group_key, build_wrap_piece_group_key_deps,
    build_wrap_piece_group_key_params, build_wrap_piece_group_key_payload, wrap_piece_group_key,
};

use super::cross_set;
use super::interface::{CrossSetMember, ICrossSetCaseConsumer};
use super::mock::{
    CrossSetCaseParamsOverrides, CrossSetCasePayloadOverrides, CrossSetParamsOverrides,
    CrossSetPayloadOverrides,
    build_cross_set_case_deps, build_cross_set_case_params, build_cross_set_case_payload,
    build_cross_set_deps, build_cross_set_params, build_cross_set_payload, run_cross_set_case,
};
use crate::delivery::provides::{AdmittedKemIdentityOverrides, build_admitted_kem_identity};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(CrossSetSuccessReturn { vectors })`, a record
///   for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3`, a count other than the params builder's default,
///   so a list sized from the default reports otherwise; the case's defaults.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn cross_set_emits_the_configured_count_of_records() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(Default::default()),
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_emits_the_configured_count_of_records
}

/// Contract: Draw → the piece-group key's draw has `PIECE_GROUP_KEY_LENGTH`
///   bytes and every other uniform the length the pairing's scalar sampling
///   bound declares, and a record's `draws` are the key's draw and the five
///   uniforms of each member concatenated.
/// Arrange: `count` of `1`.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the record's `draws` hold `32` bytes plus `10` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn cross_set_draws_each_record_the_key_and_the_uniforms_of_its_calls_at_the_declared_lengths() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert_eq!(
                record.draws.len(),
                32 + 10 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(Default::default()),
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_draws_each_record_the_key_and_the_uniforms_of_its_calls_at_the_declared_lengths
}

/// Contract: Ordering → the records in index order; within a record the
///   key's draw, then the first member's draws in the order master, `u0`,
///   `u1`, issuance, and encapsulation, then the second member's in the
///   same order.
/// Arrange: `count` of `2`; the expected draws from the independent source,
///   one call of `PIECE_GROUP_KEY_LENGTH` and ten of `UNIFORM_BYTES_LENGTH`
///   grouped for each record in turn, so a transposed record, member, or
///   draw reports otherwise.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn cross_set_records_the_draws_in_the_sequence_the_source_yields() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let draw = |length: usize| {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            let mut expected: Vec<Vec<u8>> = Vec::new();
            for _ in 0..self.count.get() {
                let mut draws = draw(PIECE_GROUP_KEY_LENGTH).expose().clone();
                for _ in 0..10 {
                    draws.extend_from_slice(
                        draw(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH).expose(),
                    );
                }
                expected.push(draws);
            }
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), expected.len());
            for (record, draws) in success.vectors.iter().zip(expected.iter()) {
                assert_eq!(&record.draws, draws);
            }
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(Default::default()),
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Piece-group key and Member under an entitlement identity →
///   setup takes `payload.identity.setup_scope`, `SetupScope::Entitlement`,
///   and derivation `payload.identity.identity`,
///   `KemIdentity::Entitlement { canonical }`; the record holds the admitted
///   key and, for each member, the parameter set, identity element,
///   credential, capsule, encapsulated value, wrapped key, decapsulated
///   value, and unwrapped key its calls returned over its draws.
/// Arrange: `count` of `1`; the admitted entitlement identity over
///   `b"pkg@1.0.0#1"`; the replay over the independent source's first
///   eleven draws under that identity's `setup_scope` and `identity`.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the record's `piece_group_key.expose()` equals the replayed
///   key's; for each member the encoded `hpub`, `scalar` and `element`,
///   `a` and `b`, and capsule components each equal the replay's, the
///   `key_material().expose()` bytes of `encapsulated` and `decapsulated`
///   each equal the replay's encapsulated bytes, `wrapped.as_bytes()`
///   equals the replayed wrap's, and `unwrapped.expose()` equals the
///   replayed key's; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn cross_set_records_the_values_its_draws_produce_under_the_entitlement_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let draw = |length: usize| {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            let replay_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
                secret: Some(draw(PIECE_GROUP_KEY_LENGTH)),
            });
            let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let replay_member = || {
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(admitted.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(draw(uniform)),
                            u0_uniform: Some(draw(uniform)),
                            u1_uniform: Some(draw(uniform)),
                        }),
                    )
                    .expect("the replay's setup succeeds");
                let derived = kem
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(admitted.identity),
                            },
                        ),
                    )
                    .expect("the replay's identity derivation succeeds");
                let issued = kem
                    .issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(draw(uniform)),
                            },
                        ),
                    )
                    .expect("the replay's issuance succeeds");
                let encapsulated = kem
                    .encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            EncapsulatePayloadOverrides {
                                uniform: Some(draw(uniform)),
                            },
                        ),
                    )
                    .expect("the replay's encapsulation succeeds");
                let wrapped = wrap_piece_group_key(
                    &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                    build_wrap_piece_group_key_params(Default::default()),
                    build_wrap_piece_group_key_payload(
                        &encapsulated.encapsulated,
                        context,
                        &replay_key,
                        Default::default(),
                    ),
                )
                .expect("the replay's wrap succeeds");
                (setup, derived, issued, encapsulated, wrapped)
            };
            let (first_setup, first_derived, first_issued, first_encapsulated, first_wrapped) =
                replay_member();
            let (second_setup, second_derived, second_issued, second_encapsulated, second_wrapped) =
                replay_member();
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, kdf, encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert_eq!(record.piece_group_key.expose(), replay_key.expose());
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
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: scalar.clone(),
                        },
                    )
                    .expect("the scalar encoding is infallible")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let check_member =
                |member: &CrossSetMember<K>,
                 setup: &SetupSuccessReturn<K::ParameterSet, K::MasterScalar>,
                 derived: &DeriveIdentitySuccessReturn<K::IdentityElement>,
                 issued: &IssueSuccessReturn<K::Credential, P::Scalar>,
                 encapsulated: &EncapsulateSuccessReturn<K::Capsule>,
                 wrapped: &WrapPieceGroupKeySuccessReturn| {
                    let expected_set = kem
                        .parameter_set_components(
                            build_parameter_set_components_params(Default::default()),
                            build_parameter_set_components_payload(
                                &setup.parameter_set,
                                Default::default(),
                            ),
                        )
                        .expect("the parameter set's components are infallible")
                        .components;
                    let actual_set = kem
                        .parameter_set_components(
                            build_parameter_set_components_params(Default::default()),
                            build_parameter_set_components_payload(
                                &member.parameter_set,
                                Default::default(),
                            ),
                        )
                        .expect("the member's parameter set's components are infallible")
                        .components;
                    assert_eq!(encode_g2(&actual_set.hpub), encode_g2(&expected_set.hpub));
                    let expected_element = kem
                        .identity_element_components(
                            build_identity_element_components_params(Default::default()),
                            build_identity_element_components_payload(
                                &derived.identity_element,
                                Default::default(),
                            ),
                        )
                        .expect("the identity element's components are infallible")
                        .components;
                    let actual_element = kem
                        .identity_element_components(
                            build_identity_element_components_params(Default::default()),
                            build_identity_element_components_payload(
                                &member.identity_element,
                                Default::default(),
                            ),
                        )
                        .expect("the member's identity element's components are infallible")
                        .components;
                    assert_eq!(
                        encode_scalar(&actual_element.scalar),
                        encode_scalar(&expected_element.scalar)
                    );
                    assert_eq!(
                        encode_g1(&actual_element.element),
                        encode_g1(&expected_element.element)
                    );
                    let expected_credential = kem
                        .credential_components(
                            build_credential_components_params(Default::default()),
                            build_credential_components_payload(
                                &issued.credential,
                                Default::default(),
                            ),
                        )
                        .expect("the credential's components are infallible")
                        .components;
                    let actual_credential = kem
                        .credential_components(
                            build_credential_components_params(Default::default()),
                            build_credential_components_payload(
                                &member.credential,
                                Default::default(),
                            ),
                        )
                        .expect("the member's credential's components are infallible")
                        .components;
                    assert_eq!(
                        encode_g1(&actual_credential.a),
                        encode_g1(&expected_credential.a)
                    );
                    assert_eq!(
                        encode_g2(&actual_credential.b),
                        encode_g2(&expected_credential.b)
                    );
                    let expected_capsule = kem
                        .capsule_components(
                            build_capsule_components_params(Default::default()),
                            build_capsule_components_payload(
                                &encapsulated.capsule,
                                Default::default(),
                            ),
                        )
                        .expect("the capsule's components are infallible")
                        .components;
                    let actual_capsule = kem
                        .capsule_components(
                            build_capsule_components_params(Default::default()),
                            build_capsule_components_payload(&member.capsule, Default::default()),
                        )
                        .expect("the member's capsule's components are infallible")
                        .components;
                    assert_eq!(
                        core::mem::discriminant(&actual_capsule),
                        core::mem::discriminant(&expected_capsule)
                    );
                    match (actual_capsule, expected_capsule) {
                        (
                            CapsuleComponents::Entitlement {
                                u: actual_u,
                                v: actual_v,
                                w: actual_w,
                            },
                            CapsuleComponents::Entitlement {
                                u: expected_u,
                                v: expected_v,
                                w: expected_w,
                            },
                        ) => {
                            assert_eq!(encode_g2(&actual_u), encode_g2(&expected_u));
                            assert_eq!(encode_g1(&actual_v), encode_g1(&expected_v));
                            assert_eq!(encode_g1(&actual_w), encode_g1(&expected_w));
                        }
                        (
                            CapsuleComponents::Asset {
                                u: actual_u,
                                v: actual_v,
                            },
                            CapsuleComponents::Asset {
                                u: expected_u,
                                v: expected_v,
                            },
                        ) => {
                            assert_eq!(encode_g2(&actual_u), encode_g2(&expected_u));
                            assert_eq!(encode_g1(&actual_v), encode_g1(&expected_v));
                        }
                        _ => {}
                    }
                    assert_eq!(
                        member.encapsulated.key_material().expose(),
                        encapsulated.encapsulated.key_material().expose()
                    );
                    assert_eq!(
                        member.decapsulated.key_material().expose(),
                        encapsulated.encapsulated.key_material().expose()
                    );
                    assert_eq!(member.wrapped.as_bytes(), wrapped.wrapped.as_bytes());
                    assert_eq!(member.unwrapped.expose(), replay_key.expose());
                };
            check_member(
                &record.first,
                &first_setup,
                &first_derived,
                &first_issued,
                &first_encapsulated,
                &first_wrapped,
            );
            check_member(
                &record.second,
                &second_setup,
                &second_derived,
                &second_issued,
                &second_encapsulated,
                &second_wrapped,
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(Default::default()),
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_records_the_values_its_draws_produce_under_the_entitlement_scope
}

/// Contract: Piece-group key and Member under an asset identity → setup
///   takes `payload.identity.setup_scope`, `SetupScope::Asset { root }`,
///   and derivation `payload.identity.identity`, `KemIdentity::Asset
///   { root }`; the record holds the values its calls returned over its
///   draws.
/// Arrange: `scope` `IdentityScope::Asset`, the identity bytes `[0x44; 32]`,
///   `count` of `1`; the admitted asset identity the case consumer builds
///   over the root `build_root` builds from those bytes; the replay as the
///   entitlement block builds it, under that identity's `setup_scope` and
///   `identity`.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  as the entitlement block, against this replay; the outermost
///   `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the KEM's scope, which
///   this block names; the independent source, a separate adapter at the
///   same defaults as the subject's.
#[test]
fn cross_set_records_the_values_its_draws_produce_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let draw = |length: usize| {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            let replay_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
                secret: Some(draw(PIECE_GROUP_KEY_LENGTH)),
            });
            let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let replay_member = || {
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(admitted.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(draw(uniform)),
                            u0_uniform: Some(draw(uniform)),
                            u1_uniform: Some(draw(uniform)),
                        }),
                    )
                    .expect("the replay's setup succeeds");
                let derived = kem
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(admitted.identity),
                            },
                        ),
                    )
                    .expect("the replay's identity derivation succeeds");
                let issued = kem
                    .issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(draw(uniform)),
                            },
                        ),
                    )
                    .expect("the replay's issuance succeeds");
                let encapsulated = kem
                    .encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            EncapsulatePayloadOverrides {
                                uniform: Some(draw(uniform)),
                            },
                        ),
                    )
                    .expect("the replay's encapsulation succeeds");
                let wrapped = wrap_piece_group_key(
                    &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                    build_wrap_piece_group_key_params(Default::default()),
                    build_wrap_piece_group_key_payload(
                        &encapsulated.encapsulated,
                        context,
                        &replay_key,
                        Default::default(),
                    ),
                )
                .expect("the replay's wrap succeeds");
                (setup, derived, issued, encapsulated, wrapped)
            };
            let (first_setup, first_derived, first_issued, first_encapsulated, first_wrapped) =
                replay_member();
            let (second_setup, second_derived, second_issued, second_encapsulated, second_wrapped) =
                replay_member();
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, kdf, encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert_eq!(record.piece_group_key.expose(), replay_key.expose());
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
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: scalar.clone(),
                        },
                    )
                    .expect("the scalar encoding is infallible")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let check_member =
                |member: &CrossSetMember<K>,
                 setup: &SetupSuccessReturn<K::ParameterSet, K::MasterScalar>,
                 derived: &DeriveIdentitySuccessReturn<K::IdentityElement>,
                 issued: &IssueSuccessReturn<K::Credential, P::Scalar>,
                 encapsulated: &EncapsulateSuccessReturn<K::Capsule>,
                 wrapped: &WrapPieceGroupKeySuccessReturn| {
                    let expected_set = kem
                        .parameter_set_components(
                            build_parameter_set_components_params(Default::default()),
                            build_parameter_set_components_payload(
                                &setup.parameter_set,
                                Default::default(),
                            ),
                        )
                        .expect("the parameter set's components are infallible")
                        .components;
                    let actual_set = kem
                        .parameter_set_components(
                            build_parameter_set_components_params(Default::default()),
                            build_parameter_set_components_payload(
                                &member.parameter_set,
                                Default::default(),
                            ),
                        )
                        .expect("the member's parameter set's components are infallible")
                        .components;
                    assert_eq!(encode_g2(&actual_set.hpub), encode_g2(&expected_set.hpub));
                    let expected_element = kem
                        .identity_element_components(
                            build_identity_element_components_params(Default::default()),
                            build_identity_element_components_payload(
                                &derived.identity_element,
                                Default::default(),
                            ),
                        )
                        .expect("the identity element's components are infallible")
                        .components;
                    let actual_element = kem
                        .identity_element_components(
                            build_identity_element_components_params(Default::default()),
                            build_identity_element_components_payload(
                                &member.identity_element,
                                Default::default(),
                            ),
                        )
                        .expect("the member's identity element's components are infallible")
                        .components;
                    assert_eq!(
                        encode_scalar(&actual_element.scalar),
                        encode_scalar(&expected_element.scalar)
                    );
                    assert_eq!(
                        encode_g1(&actual_element.element),
                        encode_g1(&expected_element.element)
                    );
                    let expected_credential = kem
                        .credential_components(
                            build_credential_components_params(Default::default()),
                            build_credential_components_payload(
                                &issued.credential,
                                Default::default(),
                            ),
                        )
                        .expect("the credential's components are infallible")
                        .components;
                    let actual_credential = kem
                        .credential_components(
                            build_credential_components_params(Default::default()),
                            build_credential_components_payload(
                                &member.credential,
                                Default::default(),
                            ),
                        )
                        .expect("the member's credential's components are infallible")
                        .components;
                    assert_eq!(
                        encode_g1(&actual_credential.a),
                        encode_g1(&expected_credential.a)
                    );
                    assert_eq!(
                        encode_g2(&actual_credential.b),
                        encode_g2(&expected_credential.b)
                    );
                    let expected_capsule = kem
                        .capsule_components(
                            build_capsule_components_params(Default::default()),
                            build_capsule_components_payload(
                                &encapsulated.capsule,
                                Default::default(),
                            ),
                        )
                        .expect("the capsule's components are infallible")
                        .components;
                    let actual_capsule = kem
                        .capsule_components(
                            build_capsule_components_params(Default::default()),
                            build_capsule_components_payload(&member.capsule, Default::default()),
                        )
                        .expect("the member's capsule's components are infallible")
                        .components;
                    assert_eq!(
                        core::mem::discriminant(&actual_capsule),
                        core::mem::discriminant(&expected_capsule)
                    );
                    match (actual_capsule, expected_capsule) {
                        (
                            CapsuleComponents::Entitlement {
                                u: actual_u,
                                v: actual_v,
                                w: actual_w,
                            },
                            CapsuleComponents::Entitlement {
                                u: expected_u,
                                v: expected_v,
                                w: expected_w,
                            },
                        ) => {
                            assert_eq!(encode_g2(&actual_u), encode_g2(&expected_u));
                            assert_eq!(encode_g1(&actual_v), encode_g1(&expected_v));
                            assert_eq!(encode_g1(&actual_w), encode_g1(&expected_w));
                        }
                        (
                            CapsuleComponents::Asset {
                                u: actual_u,
                                v: actual_v,
                            },
                            CapsuleComponents::Asset {
                                u: expected_u,
                                v: expected_v,
                            },
                        ) => {
                            assert_eq!(encode_g2(&actual_u), encode_g2(&expected_u));
                            assert_eq!(encode_g1(&actual_v), encode_g1(&expected_v));
                        }
                        _ => {}
                    }
                    assert_eq!(
                        member.encapsulated.key_material().expose(),
                        encapsulated.encapsulated.key_material().expose()
                    );
                    assert_eq!(
                        member.decapsulated.key_material().expose(),
                        encapsulated.encapsulated.key_material().expose()
                    );
                    assert_eq!(member.wrapped.as_bytes(), wrapped.wrapped.as_bytes());
                    assert_eq!(member.unwrapped.expose(), replay_key.expose());
                };
            check_member(
                &record.first,
                &first_setup,
                &first_derived,
                &first_issued,
                &first_encapsulated,
                &first_wrapped,
            );
            check_member(
                &record.second,
                &second_setup,
                &second_derived,
                &second_issued,
                &second_encapsulated,
                &second_wrapped,
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(CrossSetCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_cross_set_case_payload(CrossSetCasePayloadOverrides {
            identity: Some(vec![0x44; 32]),
            ..Default::default()
        }),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_records_the_values_its_draws_produce_under_the_asset_scope
}

/// Contract: Member → the wrap and the unwrap run over `payload.context`.
/// Arrange: `count` of `1`; the context `build_derivation_context` with
///   `DerivationContextConstructorParamsOverrides` setting every component
///   to values its builder returns over overrides that differ from their
///   defaults; the replay as the entitlement block builds it over that
///   context, and a second replay of the wrap over
///   `build_derivation_context(Default::default())`, so a wrap over a
///   fixed or default context reports otherwise.
/// Act:     `cross_set`, once, in the case consumer's method, over the
///   overridden context.
/// Assert:  each member's `wrapped.as_bytes()` equals the replay's over
///   the overridden context and differs from the replay's over the default
///   context, and each member's `unwrapped.expose()` equals the record's
///   `piece_group_key.expose()`; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn cross_set_wraps_under_the_configured_context() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        asset: Some(build_root(RootConstructorParamsOverrides {
            bytes: Some([0x55; 32]),
        })),
        deployment: Some(build_deployment_identity(
            DeploymentIdentityConstructorParamsOverrides {
                bytes: Some([0x66; 32]),
            },
        )),
        suite: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x77; 32]),
                version: Some(2),
            },
        )),
        parameter_set: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x88; 32]),
            },
        )),
        group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
            value: Some(1),
        })),
        geometry: Some(build_piece_geometry(
            PieceGeometryConstructorParamsOverrides {
                piece_group_size: Some(1),
                total_extent: Some(2097152),
                ..Default::default()
            },
        )),
    });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let draw = |length: usize| {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            let replay_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
                secret: Some(draw(PIECE_GROUP_KEY_LENGTH)),
            });
            let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let default_context = build_derivation_context(Default::default());
            let replay_member = || {
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(admitted.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(draw(uniform)),
                            u0_uniform: Some(draw(uniform)),
                            u1_uniform: Some(draw(uniform)),
                        }),
                    )
                    .expect("the replay's setup succeeds");
                let derived = kem
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(admitted.identity),
                            },
                        ),
                    )
                    .expect("the replay's identity derivation succeeds");
                let _issued = kem
                    .issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(draw(uniform)),
                            },
                        ),
                    )
                    .expect("the replay's issuance succeeds");
                let encapsulated = kem
                    .encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            EncapsulatePayloadOverrides {
                                uniform: Some(draw(uniform)),
                            },
                        ),
                    )
                    .expect("the replay's encapsulation succeeds");
                let wrapped = wrap_piece_group_key(
                    &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                    build_wrap_piece_group_key_params(Default::default()),
                    build_wrap_piece_group_key_payload(
                        &encapsulated.encapsulated,
                        context,
                        &replay_key,
                        Default::default(),
                    ),
                )
                .expect("the replay's wrap succeeds");
                let wrapped_default = wrap_piece_group_key(
                    &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                    build_wrap_piece_group_key_params(Default::default()),
                    build_wrap_piece_group_key_payload(
                        &encapsulated.encapsulated,
                        &default_context,
                        &replay_key,
                        Default::default(),
                    ),
                )
                .expect("the default-context wrap succeeds");
                (wrapped, wrapped_default)
            };
            let (first_wrapped, first_wrapped_default) = replay_member();
            let (second_wrapped, second_wrapped_default) = replay_member();
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, kdf, encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert_eq!(
                record.first.wrapped.as_bytes(),
                first_wrapped.wrapped.as_bytes()
            );
            assert_eq!(
                record.second.wrapped.as_bytes(),
                second_wrapped.wrapped.as_bytes()
            );
            assert_ne!(
                record.first.wrapped.as_bytes(),
                first_wrapped_default.wrapped.as_bytes()
            );
            assert_ne!(
                record.second.wrapped.as_bytes(),
                second_wrapped_default.wrapped.as_bytes()
            );
            assert_eq!(
                record.first.unwrapped.expose(),
                record.piece_group_key.expose()
            );
            assert_eq!(
                record.second.unwrapped.expose(),
                record.piece_group_key.expose()
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(Default::default()),
        build_cross_set_case_payload(CrossSetCasePayloadOverrides {
            context: Some(context),
            ..Default::default()
        }),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_wraps_under_the_configured_context
}
