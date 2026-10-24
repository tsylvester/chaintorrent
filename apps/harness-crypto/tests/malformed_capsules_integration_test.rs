#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashTryNewErrorReturn};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IKemConsumer, IdentityScope, SetupErrorReturn, build_create_kem_deps, build_create_kem_payload,
    build_is_well_formed_params, build_is_well_formed_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, MockIPairingAdapterFailureMode,
    PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};

use harness_crypto::{
    MalformedCapsulesErrorReturn, MalformedCapsulesParamsOverrides, VectorCount,
    VectorCountConstructorParamsOverrides, build_malformed_capsules_deps,
    build_malformed_capsules_params, build_malformed_capsules_payload, build_vector_count,
    malformed_capsules,
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
///   `DECLARED_KEM_SELECTIONS` — the identity `b"pkg@1.0.0#1"` under the
///   entitlement scope and `vec![0x44; 32]` under the asset scope.
/// Act:     `malformed_capsules`, once per case, in the KEM consumer's
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
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.is_well_formed);
                assert!(!record.displaced_is_well_formed);
                let Ok(capsule_verdict) = payload.adapter.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &record.parameter_set,
                        &record.capsule,
                        Default::default(),
                    ),
                );
                assert!(capsule_verdict.is_well_formed);
                let Ok(displaced_verdict) = payload.adapter.is_well_formed(
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

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let identity: Vec<u8> = match kem_selection.scope {
                        IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                        IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                    };
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: identity.as_slice(),
                            },
                            Default::default(),
                        ),
                        *kem_selection,
                        build_create_kem_payload(Default::default()),
                    );
                    created.expect("the declared selection constructs");
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
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
///   scope.
/// Act:     `malformed_capsules`, once per case, in the KEM consumer's
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
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
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

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let identity: Vec<u8> = match kem_selection.scope {
                        IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                        IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                    };
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: identity.as_slice(),
                            },
                            Default::default(),
                        ),
                        *kem_selection,
                        build_create_kem_payload(Default::default()),
                    );
                    created.expect("the declared selection constructs");
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
    }
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: own entry
///   `an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope`,
///   as an outside caller observes it — an identity that is not a 32-byte
///   array is refused in the function's `AssetIdentityLength` variant with
///   the identity's length, and an all-zero 32-byte identity is refused in
///   its `AssetIdentityHash` variant carrying the domain constructor's
///   `AllZero` unchanged, while a well-formed identity is admitted and the
///   entitlement scope reads no identity at all.
/// Arrange: each asset-scope selection of `DECLARED_KEM_SELECTIONS` over
///   every member of `PAIRING_CONCRETES` and every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the identities `vec![0x44; 31]`,
///   `vec![0x00; 32]`, and `vec![0x44; 32]` in turn, each with the `Err` it
///   must equal or `None` for the admitted identity; and each
///   entitlement-scope selection over `vec![0x44; 31]`.
/// Act:     `malformed_capsules`, once per selection and identity, in the
///   KEM consumer's method.
/// Assert:  under the asset scope the results equal
///   `Err(MalformedCapsulesErrorReturn::AssetIdentityLength { actual: 31 })`
///   and `Err(MalformedCapsulesErrorReturn::AssetIdentityHash(
///   AssetIdentityHashTryNewErrorReturn::AllZero))` whole, and the third
///   `is_ok()`; under the entitlement scope the result `is_ok()`; each
///   factory's result is unpacked by `expect` naming the declared selection
///   and the outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `malformed_capsules`, with the route of the
///   blocks above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        expected: &'a Option<MalformedCapsulesErrorReturn>,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(returned.err(), *self.expected);
        }
    }

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            let asset_cases: [(Vec<u8>, Option<MalformedCapsulesErrorReturn>); 3] = [
                (
                    vec![0x44; ASSET_IDENTITY_HASH_LENGTH - 1],
                    Some(MalformedCapsulesErrorReturn::AssetIdentityLength {
                        actual: ASSET_IDENTITY_HASH_LENGTH - 1,
                    }),
                ),
                (
                    vec![0x00; ASSET_IDENTITY_HASH_LENGTH],
                    Some(MalformedCapsulesErrorReturn::AssetIdentityHash(
                        AssetIdentityHashTryNewErrorReturn::AllZero,
                    )),
                ),
                (vec![0x44; ASSET_IDENTITY_HASH_LENGTH], None),
            ];
            let entitlement_cases: [(Vec<u8>, Option<MalformedCapsulesErrorReturn>); 1] =
                [(vec![0x44; ASSET_IDENTITY_HASH_LENGTH - 1], None)];
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let cases: &[(Vec<u8>, Option<MalformedCapsulesErrorReturn>)] =
                        match kem_selection.scope {
                            IdentityScope::Entitlement => &entitlement_cases,
                            IdentityScope::Asset => &asset_cases,
                        };
                    for (identity, expected) in cases {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    subject: &*subject_source.adapter,
                                    count: self.count,
                                    identity: identity.as_slice(),
                                    expected,
                                },
                                Default::default(),
                            ),
                            *kem_selection,
                            build_create_kem_payload(Default::default()),
                        );
                        created.expect("the declared selection constructs");
                    }
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
    }
    // an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope
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
/// Act:     `malformed_capsules`, once per source, in the KEM consumer's
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
    struct KemConsumer<'a> {
        refusing: &'a dyn IRandomSourceAdapter,
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let refused = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.refusing, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                refused.err(),
                Some(MalformedCapsulesErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))
            );
            // Act
            let admitted = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert!(admitted.is_ok());
        }
    }

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let refusing_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the configured selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let identity: Vec<u8> = match kem_selection.scope {
                        IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                        IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                    };
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                refusing: &*refusing_source.adapter,
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: identity.as_slice(),
                            },
                            Default::default(),
                        ),
                        *kem_selection,
                        build_create_kem_payload(Default::default()),
                    );
                    created.expect("the declared selection constructs");
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
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
/// Act:     `malformed_capsules`, once per pairing selection, in the KEM
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
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        expected: &'a Option<SampleUniformScalarErrorReturn>,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
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

    struct PairingConsumer<'a> {
        count: VectorCount,
        expected: &'a Option<SampleUniformScalarErrorReturn>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let identity: Vec<u8> = match kem_selection.scope {
                        IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                        IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                    };
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: identity.as_slice(),
                                expected: self.expected,
                            },
                            Default::default(),
                        ),
                        *kem_selection,
                        build_create_kem_payload(Default::default()),
                    );
                    created.expect("the declared selection constructs");
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let refused_expected = Some(SampleUniformScalarErrorReturn::WrongLength {
        expected: 16,
        actual: 16,
    });
    let admitted_expected: Option<SampleUniformScalarErrorReturn> = None;
    // Act
    let refused = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                count,
                expected: &refused_expected,
            },
            Default::default(),
        ),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::SampleScalarWrongLength,
            )),
            ..Default::default()
        }),
        build_create_pairing_payload(Default::default()),
    );
    // Assert
    assert!(refused.is_ok());
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(
                PairingConsumer {
                    count,
                    expected: &admitted_expected,
                },
                Default::default(),
            ),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
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
///   `vec![0x44; 32]` under the asset scope and `b"pkg@1.0.0#1"` under the
///   entitlement scope; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `malformed_capsules`, once per selection and scope, in the KEM
///   consumer's method.
/// Assert:  under the asset scope the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
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
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            match payload.scope() {
                IdentityScope::Entitlement => assert!(returned.is_ok()),
                IdentityScope::Asset => assert_eq!(
                    returned.err(),
                    Some(MalformedCapsulesErrorReturn::Setup(
                        SetupErrorReturn::HashToScalar(expected)
                    ))
                ),
            }
        }
    }

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the configured selection constructs");
            for kem_selection in DECLARED_KEM_SELECTIONS {
                let identity: Vec<u8> = match kem_selection.scope {
                    IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                    IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                };
                let created = create_kem(
                    &build_create_kem_deps(
                        &payload.adapter,
                        &*hashing.adapter,
                        KemConsumer {
                            subject: &*subject_source.adapter,
                            count: self.count,
                            identity: identity.as_slice(),
                        },
                        Default::default(),
                    ),
                    *kem_selection,
                    build_create_kem_payload(Default::default()),
                );
                created.expect("the declared selection constructs");
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}
