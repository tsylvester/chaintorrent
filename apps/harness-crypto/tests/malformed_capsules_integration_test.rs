#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params,
};
use kem::{
    DECLARED_KEM_SELECTIONS, ICredentialKemAdapter, SetupErrorReturn, SetupScope,
    build_is_well_formed_params, build_is_well_formed_payload,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PAIRING_CONCRETES, PairingConcrete,
    SampleUniformScalarErrorReturn, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, IMalformedCapsulesCaseConsumer, MalformedCapsulesCaseParamsOverrides,
    MalformedCapsulesErrorReturn, MalformedCapsulesParamsOverrides,
    MalformedCapsulesPayloadOverrides, VectorCount, VectorCountConstructorParamsOverrides,
    build_malformed_capsules_case_deps, build_malformed_capsules_case_params,
    build_malformed_capsules_case_payload, build_malformed_capsules_deps,
    build_malformed_capsules_params, build_malformed_capsules_payload, build_vector_count,
    malformed_capsules, run_malformed_capsules_case,
};

/// Contract: own entry
///   `every_record_holds_a_well_formed_capsule_and_an_ill_formed_displacement`,
///   as an outside caller observes it — `malformed_capsules` over a case with
///   a count above one, every adapter real and the operating-system source
///   real → every record's capsule is well formed and its displaced capsule
///   is not, in the record's verdicts and under the KEM's own check.
/// Arrange: `count` of `2` from `build_vector_count` with
///   `VectorCountConstructorParamsOverrides`; every case — every member of
///   `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every selection and scope of
///   `DECLARED_KEM_SELECTIONS`, the identity bytes `b"pkg@1.0.0#1"` under
///   the entitlement scope and `[0x44; 32]` under the asset scope, the case
///   payload's defaults.
/// Act:     `malformed_capsules`, once per case, in the case consumer's
///   method.
/// Assert:  as the private integration test element's block of the same
///   name asserts — `vectors` holds `2` records, each with `is_well_formed`
///   `true` and `displaced_is_well_formed` `false`, and `is_well_formed`
///   over `build_is_well_formed_params` and `build_is_well_formed_payload`
///   with the record's parameter set returns `true` for `capsule` and
///   `false` for `displaced`; each factory's result is unpacked by `expect`
///   naming the declared selection and the outermost `Result` `is_ok()`,
///   proving the consumers ran.
/// Boundary: the public call `malformed_capsules`, with the route
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected concrete's `try_new`, `malformed_capsules`,
///   and every KEM method it and the block call.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_holds_a_well_formed_capsule_and_an_ill_formed_displacement() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.is_well_formed);
                assert!(!record.displaced_is_well_formed);
                let Ok(capsule_verdict) = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &record.parameter_set,
                        &record.capsule,
                        Default::default(),
                    ),
                );
                assert!(capsule_verdict.is_well_formed);
                let Ok(displaced_verdict) = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &record.parameter_set,
                        &record.displaced,
                        Default::default(),
                    ),
                );
                assert!(!displaced_verdict.is_well_formed);
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                // Act
                let returned = run_malformed_capsules_case(
                    &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
                    build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(*h2s_selection),
                        kem: Some(*kem_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                    }),
                    build_malformed_capsules_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // every_record_holds_a_well_formed_capsule_and_an_ill_formed_displacement
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it — the list holds the configured count,
///   each record's `draws` hold its four uniforms each `UNIFORM_BYTES_LENGTH`
///   long, and no two records share `draws`.
/// Arrange: `count` of `3` from `build_vector_count`; every member of
///   `PAIRING_CONCRETES` with each selection and scope of
///   `DECLARED_KEM_SELECTIONS` and every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, the identity of each selection's
///   scope, the case payload's defaults.
/// Act:     `malformed_capsules`, once per case, in the case consumer's
///   method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `malformed_capsules`, with the route of the
///   block above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
            for record in &success.vectors {
                assert_eq!(
                    record.draws.len(),
                    4 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                );
            }
            for left in &success.vectors {
                for right in &success.vectors {
                    if !core::ptr::eq(left, right) {
                        assert_ne!(left.draws, right.draws);
                    }
                }
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                // Act
                let returned = run_malformed_capsules_case(
                    &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
                    build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(*h2s_selection),
                        kem: Some(*kem_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                    }),
                    build_malformed_capsules_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole`, as an
///   outside caller observes it — a `fill_bytes` refusal reaches the caller
///   inside the function's `FillBytes` variant at the master uniform's draw,
///   before any KEM call, whole.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES` and
///   every selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the subject's
///   source `create_random_source` returns for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, and, as the counterfactual, the source for
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `malformed_capsules`, once per source, in the case consumer's
///   method.
/// Assert:  the result under the refusing source equals
///   `Err(MalformedCapsulesErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` whole, and the result under the
///   operating-system source `is_ok()`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `malformed_capsules`, with the route of the
///   blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer<'a> {
        count: VectorCount,
        expected: &'a Option<MalformedCapsulesErrorReturn>,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(returned.err(), *self.expected);
        }
    }

    let refused_expected = Some(MalformedCapsulesErrorReturn::FillBytes(
        FillBytesErrorReturn::MockIRandomSourceAdapter,
    ));
    let admitted_expected: Option<MalformedCapsulesErrorReturn> = None;
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                for (kind, expected) in [
                    (
                        RandomSourceKind::Mock(
                            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                        ),
                        &refused_expected,
                    ),
                    (RandomSourceKind::OperatingSystem, &admitted_expected),
                ] {
                    // Act
                    let returned = run_malformed_capsules_case(
                        &build_malformed_capsules_case_deps(
                            CaseConsumer { count, expected },
                            Default::default(),
                        ),
                        build_malformed_capsules_case_params(
                            MalformedCapsulesCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*h2s_selection),
                                kem: Some(*kem_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides { kind: Some(kind) },
                                )),
                            },
                        ),
                        build_malformed_capsules_case_payload(Default::default()),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
    }
    // a_draw_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_setup_sampling_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it — a scalar-sampling refusal at setup's
///   master uniform reaches the caller inside the function's `Setup` variant
///   as `MasterScalarSampling`, whole.
/// Arrange: `count` of `1`; `create_pairing` over
///   `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`, with each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, and, as the counterfactual, each
///   member of `PAIRING_CONCRETES`; the expected `error` the literal
///   `SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual:
///   16 }`.
/// Act:     `malformed_capsules`, once per pairing selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection equals
///   `Err(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected: 16, actual: 16 })))` whole, and the result under each
///   member of `PAIRING_CONCRETES` `is_ok()`; each factory's result is
///   unpacked by `expect` naming the declared selection and the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `malformed_capsules`, with the route of the
///   blocks above.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer<'a> {
        count: VectorCount,
        expected: &'a Option<SampleUniformScalarErrorReturn>,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                self.expected.as_ref().map(
                    |SampleUniformScalarErrorReturn::WrongLength { expected, actual }| {
                        MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: *expected,
                                actual: *actual,
                            },
                        ))
                    },
                )
            );
        }
    }

    let refused_expected = Some(SampleUniformScalarErrorReturn::WrongLength {
        expected: 16,
        actual: 16,
    });
    for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_malformed_capsules_case(
                &build_malformed_capsules_case_deps(
                    CaseConsumer {
                        count,
                        expected: &refused_expected,
                    },
                    Default::default(),
                ),
                build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(PairingConcrete::Mock(
                            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                        )),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(*h2s_selection),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                }),
                build_malformed_capsules_case_payload(Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    let admitted_expected: Option<SampleUniformScalarErrorReturn> = None;
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                // Act
                let returned = run_malformed_capsules_case(
                    &build_malformed_capsules_case_deps(
                        CaseConsumer {
                            count,
                            expected: &admitted_expected,
                        },
                        Default::default(),
                    ),
                    build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(*h2s_selection),
                        kem: Some(*kem_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                    }),
                    build_malformed_capsules_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // a_setup_sampling_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it — a hash-to-scalar refusal reaches the
///   caller inside the function's `Setup` variant under the asset scope,
///   whole, and the entitlement scope hashes nothing.
/// Arrange: `count` of `1` from `build_vector_count`; each selection and
///   scope of `DECLARED_KEM_SELECTIONS` over every member of
///   `PAIRING_CONCRETES`, the hash-to-scalar adapter from
///   `create_hash_to_scalar` over `build_create_hash_to_scalar_params`
///   overriding `concrete` with `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`, the identity
///   bytes `[0x44; 32]` under the asset scope and `b"pkg@1.0.0#1"` under
///   the entitlement scope, the case payload's defaults; the expected
///   `error` the literal `HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength
///   { expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
///   actual: 0 })`.
/// Act:     `malformed_capsules`, once per selection and scope, in the case
///   consumer's method.
/// Assert:  under the asset scope the result equals
///   `Err(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` whole, `error` the expected literal; under the
///   entitlement scope the result `is_ok()`; each factory's result is
///   unpacked by `expect` naming the declared selection and the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `malformed_capsules`, with the route of the
///   blocks above.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let hash_to_scalar = build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
        concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        ..Default::default()
    });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            match identity.setup_scope {
                SetupScope::Entitlement => assert!(returned.is_ok()),
                SetupScope::Asset { .. } => assert_eq!(
                    returned.err(),
                    Some(MalformedCapsulesErrorReturn::Setup(
                        SetupErrorReturn::HashToScalar(expected)
                    ))
                ),
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_malformed_capsules_case(
                &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
                build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(hash_to_scalar),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                }),
                build_malformed_capsules_case_payload(Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}
