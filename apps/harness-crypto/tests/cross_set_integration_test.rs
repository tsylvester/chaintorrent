#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::build_secret;
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, IEncoderAdapter,
    build_create_encoding_deps,
    build_create_encoding_payload, build_mock_i_encoding_consumer, create_encoding,
};
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
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides,
    KemIdentity, SetupErrorReturn, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_is_well_formed_params, build_is_well_formed_payload, build_issue_params,
    build_issue_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MockIPairingAdapterFailureMode, PAIRING_CONCRETES,
    PairingConcrete, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, FillBytesParams,
    FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, build_fill_bytes_payload,
};
use workflows::{
    DerivationContext, PIECE_GROUP_KEY_LENGTH, WrapPieceGroupKeyErrorReturn,
    build_derivation_context, build_derivation_context_description,
};

use harness_crypto::{
    AdmittedKemIdentityOverrides, CrossSetCaseParamsOverrides, CrossSetCasePayloadOverrides,
    CrossSetErrorReturn, CrossSetParamsOverrides, CrossSetPayloadOverrides, ICrossSetCaseConsumer,
    VectorCount, VectorCountConstructorParamsOverrides, build_admitted_kem_identity,
    build_cross_set_case_deps, build_cross_set_case_params, build_cross_set_case_payload,
    build_cross_set_deps, build_cross_set_params, build_cross_set_payload, build_vector_count,
    cross_set, run_cross_set_case,
};

/// Contract: own entry `every_record_unwraps_the_drawn_piece_group_key_under_
///   both_sets`, as an outside caller observes it.
/// Arrange: `count` of `2`; every case of the declared sets; the identity
///   bytes `b"pkg@1.0.0#1"` under the entitlement scope and `[0x44; 32]`
///   under the asset scope; the default derivation context.
/// Act:     `cross_set`, once per case, in the case consumer's method.
/// Assert:  `vectors` holds `2` records; for each member `is_valid` and
///   `is_well_formed` return `true`, `decapsulated` equals `encapsulated`,
///   `unwrapped.expose()` equals `piece_group_key.expose()`, and the two
///   members' `encapsulated` and `wrapped` differ; each case's `Result`
///   `is_ok()`.
/// Boundary: the public call `cross_set`, with the route the spec's public
///   surface states.
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
                        let identities: Vec<u8> = match kem_selection.scope {
                            IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                            IdentityScope::Asset => [0x44; 32].to_vec(),
                        };
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
                            build_cross_set_case_payload(CrossSetCasePayloadOverrides {
                                identity: Some(identities),
                                context: Some(build_derivation_context(Default::default())),
                            }),
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

/// Contract: own entry `every_run_emits_the_configured_count_of_records_
///   with_distinct_draws`, as an outside caller observes it.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`.
/// Act:     `cross_set`, once per case, in the case consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `32` bytes
///   plus `10` times `UNIFORM_BYTES_LENGTH` bytes, and no two records
///   share `draws`; each case's `Result` `is_ok()`.
/// Boundary: the public call `cross_set`, with the route of the block above.
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
                    32 + 10 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                );
                for other in success.vectors.iter().skip(index + 1) {
                    assert_ne!(record.draws, other.draws);
                }
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
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole`.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`;
///   the refusing source under `FillBytesRefused`, and, as the
///   counterfactual, the operating-system source.
/// Act:     `cross_set`, once per source, in the case consumer's method.
/// Assert:  the result under the refusing source equals
///   `Err(CrossSetErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` whole, and the result under the
///   operating-system source `is_ok()`; each case's `Result` `is_ok()`.
/// Boundary: the public call `cross_set`, with the route of the blocks
///   above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
        expect_refusal: bool,
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
            // Assert
            if self.expect_refusal {
                assert_eq!(
                    returned.err().expect("the subject's call fails"),
                    CrossSetErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for (kind, expect_refusal) in [
                (
                    RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                    true,
                ),
                (RandomSourceKind::OperatingSystem, false),
            ] {
                // Act
                let returned = run_cross_set_case(
                    &build_cross_set_case_deps(
                        CaseConsumer {
                            count,
                            expect_refusal,
                        },
                        Default::default(),
                    ),
                    build_cross_set_case_params(CrossSetCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides { kind: Some(kind) },
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
    // a_draw_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_setup_sampling_refusal_reaches_the_caller_whole`,
///   and the carried `kem/factory` entry `every_concrete_returns_a_sampling_
///   refusal_whole` in the half the spec restates.
/// Arrange: `count` of `1`; `create_pairing` over the mock concrete under
///   `SampleScalarWrongLength`, with each selection and scope of
///   `DECLARED_KEM_SELECTIONS`, and, as the counterfactual, each member of
///   `PAIRING_CONCRETES`; the expected `error` the literal `WrongLength
///   { expected: 16, actual: 16 }`.
/// Act:     `cross_set`, once per pairing selection, in the case consumer's
///   method.
/// Assert:  the result under the refusing selection equals
///   `Err(CrossSetErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16,
///   actual: 16 })))` whole, and the result under each member of
///   `PAIRING_CONCRETES` `is_ok()`; each case's `Result` `is_ok()`.
/// Boundary: the public call `cross_set`, with the route the spec states
///   for the carried entry.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
        expect_refusal: bool,
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
            // Assert
            if self.expect_refusal {
                assert_eq!(
                    returned.err().expect("the subject's call fails"),
                    CrossSetErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: 16,
                            actual: 16,
                        }
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }

    for kem_selection in DECLARED_KEM_SELECTIONS {
        // Act — the refusing pairing
        let returned = run_cross_set_case(
            &build_cross_set_case_deps(
                CaseConsumer {
                    count,
                    expect_refusal: true,
                },
                Default::default(),
            ),
            build_cross_set_case_params(CrossSetCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                })),
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
        for pairing_concrete in PAIRING_CONCRETES {
            // Act — the counterfactual
            let returned = run_cross_set_case(
                &build_cross_set_case_deps(
                    CaseConsumer {
                        count,
                        expect_refusal: false,
                    },
                    Default::default(),
                ),
                build_cross_set_case_params(CrossSetCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
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
    // a_setup_sampling_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   the hash-to-scalar adapter selected under `SamplingRefused`, the
///   identity bytes `b"pkg@1.0.0#1"` under the entitlement scope and
///   `[0x44; 32]` under the asset scope; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `cross_set`, once per selection and scope, in the case
///   consumer's method.
/// Assert:  the result equals the expected variant whole under the
///   received scope; each case's `Result` `is_ok()`.
/// Boundary: the public call `cross_set`, with the route of the blocks
///   above.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
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
            let identity_bytes: Vec<u8> = match kem_selection.scope {
                IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                IdentityScope::Asset => [0x44; 32].to_vec(),
            };
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
                build_cross_set_case_payload(CrossSetCasePayloadOverrides {
                    identity: Some(identity_bytes),
                    ..Default::default()
                }),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_key_derivation_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES` and
///   each case of `DECLARED_ENCODING_SELECTIONS`, the key-derivation
///   adapter selected under `KeyMaterialLengthUnrepresentable`; the
///   expected `length` the length of `key_material().expose()` of the
///   value an `encapsulate` returns, which the case consumer runs on the
///   received KEM under a parameter set it sets up over draws from a
///   second operating-system source, before the subject's call.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the result equals the fully nested refusal whole; each case's
///   `Result` `is_ok()`.
/// Boundary: the public call `cross_set`, with the route of the blocks
///   above, through the wrap's encoding.
/// Mocked:  the key-derivation family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
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

/// Contract: carried `kem/factory` entry `every_concrete_declaring_the_
///   keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_
///   to_the_oracle`, restated by the spec — `cross_set` hands the payload's
///   admitted identity to `derive_identity` unchanged for each member, so
///   each member's identity element is the one `derive_identity` returns
///   for it under its set, and for the entitlement identity whose
///   canonical bytes are the bytes `create_encoding` returns for the
///   derivation context under `DerivationContextDescription` its `scalar`
///   encodes to the scalar sampled under the declared tag.
/// Arrange: `count` of `1`; each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   each case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`, each case of
///   `DECLARED_ENCODING_SELECTIONS` that declares `EncodingIdentifier::
///   EthereumAbiV1`, and each case of `DECLARED_KDF_SELECTIONS`; the
///   identity bytes are the bytes `create_encoding` returns for the
///   derivation context `build_derivation_context(Default::default())`
///   returns, through `build_mock_i_encoding_consumer`; the context that
///   same derivation context.
/// Act:     `cross_set`, once per case and encoding, in the case
///   consumer's method.
/// Assert:  for each member, `encode_scalar` of its identity element's
///   `scalar`, read through `identity_element_components`, equals
///   `encode_scalar` of the scalar `sample_from_uniform_bytes` returns
///   for `expected_uniform_bytes` over the tag `build_domain_tag` builds
///   from `K::DECLARATION.identity_tag`, those bytes, and `UNIFORM_BYTES_
///   LENGTH`; each case's `Result` `is_ok()`.
/// Boundary: the public call `cross_set`, with the route the spec states
///   for the entry.
/// Mocked:  the encoding family's official consumer mock, which hands the
///   bytes the encoding returned to the block.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let context = build_derivation_context(Default::default());
    let description = build_derivation_context_description(Default::default());
    let mut encodings: Vec<Vec<u8>> = Vec::new();
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        if encoding_selection.identifier == EncodingIdentifier::EthereumAbiV1 {
            let consumer =
                build_mock_i_encoding_consumer(&description, context.clone(), Vec::new());
            let outcome = create_encoding(
                &build_create_encoding_deps(consumer),
                *encoding_selection,
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding succeeds");
            encodings.push(outcome.output);
        }
    }

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
                let element = kem
                    .identity_element_components(
                        build_identity_element_components_params(Default::default()),
                        build_identity_element_components_payload(
                            &member.identity_element,
                            Default::default(),
                        ),
                    )
                    .expect("the member's identity element's components are infallible")
                    .components;
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                });
                let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
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
                    encode_scalar(&element.scalar),
                    encode_scalar(expected.expose())
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if h2s_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                if encoding_selection.identifier != EncodingIdentifier::EthereumAbiV1 {
                    continue;
                }
                for kdf_selection in DECLARED_KDF_SELECTIONS {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        if kem_selection.scope != IdentityScope::Entitlement {
                            continue;
                        }
                        for canonical in &encodings {
                            // Act
                            let returned = run_cross_set_case(
                                &build_cross_set_case_deps(
                                    CaseConsumer { count },
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
                                    identity: Some(canonical.clone()),
                                    context: Some(context),
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
    // every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle
}
