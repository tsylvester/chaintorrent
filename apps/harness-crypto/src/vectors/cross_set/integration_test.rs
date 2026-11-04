#![allow(clippy::expect_used)]

use domain::build_secret;
use domain::{
    DeploymentIdentityConstructorParamsOverrides, GroupIndexConstructorParamsOverrides,
    ParameterSetIdentifierConstructorParamsOverrides, PieceGeometryConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_deployment_identity, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_suite_identifier,
};
use encoding::{CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, IEncoderAdapter};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarIdentifier, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params, build_domain_tag,
    expected_uniform_bytes,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DeriveKeyErrorReturn,
    IKeyDerivationAdapter, KdfConcrete, MockIKeyDerivationAdapterDeriveKeyErrorReturn,
    MockIKeyDerivationAdapterFailureMode, build_create_key_derivation_params,
};
use kem::{
    CapsuleComponents, DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn,
    DeriveIdentityPayloadOverrides, EncapsulatePayloadOverrides, ICredentialKemAdapter,
    IdentityScope, IssuePayloadOverrides, KemIdentity, SetupErrorReturn, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_capsule_components_params,
    build_capsule_components_payload, build_credential_components_params,
    build_credential_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_encapsulate_params, build_encapsulate_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_is_well_formed_params,
    build_is_well_formed_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeGtParams, EncodeGtPayload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1Payload, PAIRING_CONCRETES, PairingProductParams, PairingProductPayload,
    PairingProductTerm, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, TargetGroupEncodingIdentifier, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    build_fill_bytes_payload,
};
use workflows::{
    DerivationContext, DerivationContextConstructorParamsOverrides, PIECE_GROUP_KEY_LENGTH,
    PieceGroupKeyConstructorParamsOverrides, WrapPieceGroupKeyErrorReturn,
    build_derivation_context, build_piece_group_key, build_wrap_piece_group_key_deps,
    build_wrap_piece_group_key_params, build_wrap_piece_group_key_payload, wrap_piece_group_key,
};

use super::cross_set;
use super::interface::{CrossSetErrorReturn, ICrossSetCaseConsumer};
use super::mock::{
    CrossSetCaseParamsOverrides, CrossSetCasePayloadOverrides, CrossSetParamsOverrides,
    CrossSetPayloadOverrides, build_cross_set_case_deps, build_cross_set_case_params,
    build_cross_set_case_payload, build_cross_set_deps, build_cross_set_params,
    build_cross_set_payload, run_cross_set_case,
};
use crate::delivery::provides::{AdmittedKemIdentityOverrides, build_admitted_kem_identity};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: own entry `every_record_unwraps_the_drawn_piece_group_key_under_
///   both_sets`; `cross_set` over a case with a count above one, every
///   adapter real and the operating-system source real → each member's
///   credential is valid and capsule well formed, its decapsulated value
///   equals its encapsulated value, its unwrapped key equals the record's
///   drawn key, and the two members' encapsulated values and wrapped keys
///   differ.
/// Arrange: `count` of `2`; every case of the private surface — every member
///   of `PAIRING_CONCRETES` crossed with `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   each selection and scope of `DECLARED_KEM_SELECTIONS`,
///   `DECLARED_ENCODING_SELECTIONS`, and `DECLARED_KDF_SELECTIONS`.
/// Act:     `cross_set`, once per case, in the case consumer's method.
/// Assert:  `vectors` holds `2` records; for each member `is_valid` and
///   `is_well_formed` return `true`, `decapsulated` equals `encapsulated`,
///   `unwrapped.expose()` equals `piece_group_key.expose()`, and the two
///   members' `encapsulated` and `wrapped` differ; each case's `Result`
///   `is_ok()`.
/// Boundary: `cross_set` and the chain of real functions the private
///   surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_unwraps_the_drawn_piece_group_key_under_both_sets() {
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
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
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
            // Assert
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                for member in [&record.first, &record.second] {
                    let Ok(valid) = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &member.parameter_set,
                            &member.identity_element,
                            &member.credential,
                            Default::default(),
                        ),
                    );
                    assert!(valid.is_valid);
                    let Ok(well_formed) = kem.is_well_formed(
                        build_is_well_formed_params(Default::default()),
                        build_is_well_formed_payload(
                            &member.parameter_set,
                            &member.capsule,
                            Default::default(),
                        ),
                    );
                    assert!(well_formed.is_well_formed);
                    assert_eq!(
                        member.decapsulated.key_material().expose(),
                        member.encapsulated.key_material().expose()
                    );
                    assert_eq!(member.unwrapped.expose(), record.piece_group_key.expose());
                }
                assert_ne!(
                    record.first.encapsulated.key_material().expose(),
                    record.second.encapsulated.key_material().expose()
                );
                assert_ne!(
                    record.first.wrapped.as_bytes(),
                    record.second.wrapped.as_bytes()
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for kdf_selection in DECLARED_KDF_SELECTIONS {
                        // Act
                        let returned = run_cross_set_case(
                            &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
                            build_cross_set_case_params(CrossSetCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*h2s_selection),
                                kem: Some(*kem_selection),
                                encoding: Some(CreateEncodingParams {
                                    concrete: encoding_selection.concrete,
                                    identifier: encoding_selection.identifier,
                                }),
                                key_derivation: Some(*kdf_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                            }),
                            build_cross_set_case_payload(Default::default()),
                        );
                        // Assert
                        assert!(returned.is_ok());
                    }
                }
            }
        }
    }
    // every_record_unwraps_the_drawn_piece_group_key_under_both_sets
}

/// Contract: own entry `every_record_draw_replays_to_the_values_it_holds`;
///   the record's `draws`, split into the key's draw of `PIECE_GROUP_KEY_
///   LENGTH` bytes and uniforms of `UNIFORM_BYTES_LENGTH` in draw order and
///   replayed through the case's adapters under the admitted identity's
///   `setup_scope` and `identity` and over the configured context,
///   reproduce the record's key and each member's values and wrapped key;
///   each member's encapsulated value is the pairing's target-group
///   encoding of the product over its encapsulation scalar and its `hpub`,
///   and each member's identity element's scalar is the identity mapping
///   under the declared tag.
/// Arrange: `count` of `1`; every case, each run under two identity byte
///   strings of its scope and two derivation contexts differing in every
///   component.
/// Act:     `cross_set`, once per case, identity, and context, in the case
///   consumer's method.
/// Assert:  the replayed key's `expose()` equals the record's; each member's
///   encoded components equal the replay's, its `encapsulated` bytes equal
///   the replay's and `encode_gt` of `pairing_product` over `PairingProduct
///   Term { g1: mul_g1 of g1_generator by the encapsulation scalar, g2: the
///   member's hpub }`, and the replayed wrap's `as_bytes()` equals the
///   member's; the Keccak-256 oracle as the entry states; each case's
///   `Result` `is_ok()`.
/// Boundary: `cross_set` and the chain of real functions the private
///   surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_values_it_holds() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let context_default = build_derivation_context(Default::default());
    let context_other = build_derivation_context(DerivationContextConstructorParamsOverrides {
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
        h2s_identifier: HashToScalarIdentifier,
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
            let key_secret = build_secret(
                record.draws[..PIECE_GROUP_KEY_LENGTH].to_vec(),
                Default::default(),
            );
            let replay_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
                secret: Some(key_secret),
            });
            assert_eq!(record.piece_group_key.expose(), replay_key.expose());
            let uniform_length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let mut uniforms = record.draws[PIECE_GROUP_KEY_LENGTH..]
                .chunks(uniform_length)
                .map(|chunk| build_secret(chunk.to_vec(), Default::default()));
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
            for member in [&record.first, &record.second] {
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(admitted.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold ten uniforms"),
                            ),
                            u0_uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold ten uniforms"),
                            ),
                            u1_uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold ten uniforms"),
                            ),
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
                                uniform: Some(
                                    uniforms
                                        .next()
                                        .expect("the record's draws hold ten uniforms"),
                                ),
                            },
                        ),
                    )
                    .expect("the replay's issuance succeeds");
                let encapsulation_uniform = uniforms
                    .next()
                    .expect("the record's draws hold ten uniforms");
                let encapsulation_scalar =
                    <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        SampleUniformScalarPayload {
                            uniform: build_secret(
                                encapsulation_uniform.expose().clone(),
                                Default::default(),
                            ),
                        },
                    )
                    .expect("the scalar sampling succeeds")
                    .scalar;
                let encapsulated = kem
                    .encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            EncapsulatePayloadOverrides {
                                uniform: Some(encapsulation_uniform),
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
                        build_credential_components_payload(&issued.credential, Default::default()),
                    )
                    .expect("the credential's components are infallible")
                    .components;
                let actual_credential = kem
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(&member.credential, Default::default()),
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
                        build_capsule_components_payload(&encapsulated.capsule, Default::default()),
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
                assert_eq!(member.wrapped.as_bytes(), wrapped.wrapped.as_bytes());
                let generator = pairing
                    .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                    .expect("the first-group generator is infallible")
                    .point;
                let scaled = pairing
                    .mul_g1(
                        MulG1Params,
                        MulG1Payload {
                            point: generator,
                            scalar: encapsulation_scalar.expose().clone(),
                        },
                    )
                    .expect("the first-group multiplication is infallible")
                    .product;
                let product = pairing
                    .pairing_product(
                        PairingProductParams,
                        PairingProductPayload {
                            terms: vec![PairingProductTerm {
                                g1: scaled,
                                g2: actual_set.hpub,
                            }],
                        },
                    )
                    .expect("the pairing product is infallible")
                    .product;
                let encoded = pairing
                    .encode_gt(EncodeGtParams, EncodeGtPayload { value: product })
                    .expect("the target-group encoding is infallible")
                    .bytes;
                assert_eq!(
                    member.encapsulated.key_material().expose().as_slice(),
                    encoded.expose().as_ref()
                );
                if self.h2s_identifier == HashToScalarIdentifier::Keccak256V1
                    && scope == IdentityScope::Entitlement
                {
                    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                        bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                    });
                    let expected_scalar =
                        <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                            SampleUniformScalarParams,
                            SampleUniformScalarPayload {
                                uniform: build_secret(
                                    expected_uniform_bytes(
                                        &tag,
                                        identity,
                                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                    ),
                                    Default::default(),
                                ),
                            },
                        )
                        .expect("the scalar sampling succeeds")
                        .scalar;
                    assert_eq!(
                        encode_scalar(&actual_element.scalar),
                        encode_scalar(expected_scalar.expose())
                    );
                }
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for kdf_selection in DECLARED_KDF_SELECTIONS {
                        let identities: Vec<Vec<u8>> = match kem_selection.scope {
                            IdentityScope::Entitlement => {
                                vec![b"pkg@1.0.0#1".to_vec(), b"pkg@2.0.0#7".to_vec()]
                            }
                            IdentityScope::Asset => {
                                vec![[0x44; 32].to_vec(), [0x45; 32].to_vec()]
                            }
                        };
                        for identity in &identities {
                            for context in [&context_default, &context_other] {
                                // Act
                                let returned = run_cross_set_case(
                                    &build_cross_set_case_deps(
                                        CaseConsumer {
                                            count,
                                            h2s_identifier: h2s_selection.identifier,
                                        },
                                        Default::default(),
                                    ),
                                    build_cross_set_case_params(CrossSetCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(*h2s_selection),
                                        kem: Some(*kem_selection),
                                        encoding: Some(CreateEncodingParams {
                                            concrete: encoding_selection.concrete,
                                            identifier: encoding_selection.identifier,
                                        }),
                                        key_derivation: Some(*kdf_selection),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_cross_set_case_payload(CrossSetCasePayloadOverrides {
                                        identity: Some(identity.clone()),
                                        context: Some(context.clone()),
                                    }),
                                );
                                // Assert
                                assert!(returned.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
    // every_record_draw_replays_to_the_values_it_holds
}

/// Contract: own entry `every_run_emits_the_configured_count_of_records_
///   with_distinct_draws`; the list holds the configured count, each
///   record's `draws` hold the key's draw and its ten uniforms at their
///   lengths, and no two records share `draws`.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES`.
/// Act:     `cross_set`, once per member, in the case consumer's method.
/// Assert:  `vectors` holds `3` records, each record's `draws` hold `32`
///   bytes plus `10` times `UNIFORM_BYTES_LENGTH` bytes, and no two
///   records share `draws`; each case's `Result` `is_ok()`.
/// Boundary: `cross_set` and the chain of real functions the private
///   surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
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
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
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
            // Assert
            assert_eq!(success.vectors.len(), 3);
            for (index, record) in success.vectors.iter().enumerate() {
                assert_eq!(
                    record.draws.len(),
                    PIECE_GROUP_KEY_LENGTH
                        + 10 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                );
                for other in success.vectors.iter().skip(index + 1) {
                    assert_ne!(record.draws, other.draws);
                }
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let returned = run_cross_set_case(
            &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
            build_cross_set_case_params(CrossSetCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    },
                )),
                ..Default::default()
            }),
            build_cross_set_case_payload(Default::default()),
        );
        // Assert
        assert!(returned.is_ok());
    }
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: own entry `a_key_derivation_refusal_reaches_the_caller_whole`;
///   a key-derivation refusal reaches the caller inside the function's
///   `Wrap` variant, whole, from the first member's wrap.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES` and
///   each case of `DECLARED_ENCODING_SELECTIONS`, the key-derivation
///   adapter selected under `KeyMaterialLengthUnrepresentable`; the
///   expected `length` the length of `key_material().expose()` of the
///   value an `encapsulate` returns, which the case consumer runs on the
///   received KEM under a parameter set it sets up over draws from the
///   independent operating-system source.
/// Act:     `cross_set`, once per case, in the case consumer's method.
/// Assert:  the result equals the fully nested refusal whole; each case's
///   `Result` `is_ok()`.
/// Boundary: `cross_set` and the chain of real functions the private
///   surface names, through the wrap's encoding.
/// Mocked:  nothing at the outer edge; the key-derivation family's mock
///   concrete is selected by configuration.
#[test]
fn a_key_derivation_refusal_reaches_the_caller_whole() {
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
            let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            draw(PIECE_GROUP_KEY_LENGTH);
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
            let length = encapsulated.encapsulated.key_material().expose().len();
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
            // Assert
            assert_eq!(
                returned.err().expect("the subject's call fails"),
                CrossSetErrorReturn::Wrap(
                    WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::Mock(
                        MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                            length,
                        }
                    ))
                )
            );
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                // Act
                let returned = run_cross_set_case(
                    &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
                    build_cross_set_case_params(CrossSetCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        encoding: Some(CreateEncodingParams {
                                    concrete: encoding_selection.concrete,
                                    identifier: encoding_selection.identifier,
                                }),
                        key_derivation: Some(build_create_key_derivation_params(
                            CreateKeyDerivationParamsOverrides {
                                concrete: Some(KdfConcrete::Mock(
                                    MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
                                )),
                                ..Default::default()
                            },
                        )),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                        ..Default::default()
                    }),
                    build_cross_set_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // a_key_derivation_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`;
///   a hash-to-scalar refusal reaches the caller inside the function's
///   `DeriveIdentity` variant under the entitlement scope and its `Setup`
///   variant under the asset scope, whole.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   the hash-to-scalar adapter selected under `SamplingRefused`; the
///   expected `error` the literal `WrongLength { expected: UNIFORM_BYTES_
///   LENGTH, actual: 0 }`.
/// Act:     `cross_set`, once per case, in the case consumer's method.
/// Assert:  the result equals the expected variant whole under the
///   received scope; each case's `Result` `is_ok()`.
/// Boundary: `cross_set` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing at the outer edge; the hash-to-scalar family's mock
///   concrete is selected by configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
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
            // Arrange
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
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
            let error = returned.err().expect("the subject's call fails");
            // Assert
            match scope {
                IdentityScope::Entitlement => assert_eq!(
                    error,
                    CrossSetErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(
                        expected
                    ))
                ),
                IdentityScope::Asset => assert_eq!(
                    error,
                    CrossSetErrorReturn::Setup(SetupErrorReturn::HashToScalar(expected))
                ),
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_cross_set_case(
                &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
                build_cross_set_case_params(CrossSetCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        hash_to_scalar::CreateHashToScalarParamsOverrides {
                            concrete: Some(HashToScalarConcrete::Mock(
                                MockIHashToScalarAdapterFailureMode::SamplingRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_cross_set_case_payload(Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: own entry `records_under_pairings_sharing_a_target_group_
///   encoding_hold_the_same_encapsulated_bytes`; identical draws under two
///   pairing concretes whose declared target-group encoding identifiers
///   are equal yield records whose members' `encapsulated` bytes are
///   equal.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES`, each with
///   its own fresh source from `create_random_source` at the defaults of
///   `build_create_random_source_params`, the randomness family's mock
///   concrete, so every run draws the same key and uniforms; each
///   selection of `DECLARED_KEM_SELECTIONS` under one scope it declares,
///   with a case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_KDF_SELECTIONS`; the case consumer returns the pairing's
///   declared target-group encoding identifier with the `key_material()`
///   bytes of `first.encapsulated` and `second.encapsulated`, collected
///   across the members.
/// Act:     `cross_set`, once per member, in the case consumer's method.
/// Assert:  any two members whose identifiers are equal hold equal bytes
///   for `first` and equal bytes for `second`; each case's `Result`
///   `is_ok()`.
/// Boundary: `cross_set` and the chain of real functions the private
///   surface names.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn records_under_pairings_sharing_a_target_group_encoding_hold_the_same_encapsulated_bytes() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = (TargetGroupEncodingIdentifier, Vec<u8>, Vec<u8>);

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
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
            (
                P::DECLARATION.target_group_encoding,
                record.first.encapsulated.key_material().expose().clone(),
                record.second.encapsulated.key_material().expose().clone(),
            )
        }
    }

    let encoding_selection = DECLARED_ENCODING_SELECTIONS
        .first()
        .expect("the declared encoding selections hold one case");
    let kdf_selection = DECLARED_KDF_SELECTIONS
        .first()
        .expect("the declared key-derivation selections hold one case");
    let mut collected: Vec<(TargetGroupEncodingIdentifier, Vec<u8>, Vec<u8>)> = Vec::new();
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_cross_set_case(
                &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
                build_cross_set_case_params(CrossSetCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    encoding: Some(CreateEncodingParams {
                        concrete: encoding_selection.concrete,
                        identifier: encoding_selection.identifier,
                    }),
                    key_derivation: Some(*kdf_selection),
                    random_source: Some(build_create_random_source_params(Default::default())),
                    ..Default::default()
                }),
                build_cross_set_case_payload(Default::default()),
            );
            // Assert
            collected.push(returned.expect("the case succeeds").output);
        }
    }
    for (index, (identifier, first, second)) in collected.iter().enumerate() {
        for (other_identifier, other_first, other_second) in collected.iter().skip(index + 1) {
            if identifier == other_identifier {
                assert_eq!(first, other_first);
                assert_eq!(second, other_second);
            }
        }
    }
    // records_under_pairings_sharing_a_target_group_encoding_hold_the_same_encapsulated_bytes
}
