#![allow(clippy::expect_used)]

use domain::build_secret;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CapsuleComponents, CapsuleFromComponentsPayloadOverrides, DECLARED_KEM_SELECTIONS,
    EncapsulatePayloadOverrides, ICredentialKemAdapter, IdentityScope, SetupErrorReturn,
    SetupParamsOverrides, SetupPayloadOverrides, SetupScope, build_capsule_components_params,
    build_capsule_components_payload, build_capsule_from_components_params,
    build_capsule_from_components_payload, build_encapsulate_params, build_encapsulate_payload,
    build_is_well_formed_params, build_is_well_formed_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, MulG2Params, MulG2Payload, PAIRING_CONCRETES,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::interface::{IMalformedCapsulesCaseConsumer, MalformedCapsulesErrorReturn};
use super::malformed_capsules;
use super::mock::{
    MalformedCapsulesCaseParamsOverrides, MalformedCapsulesCasePayloadOverrides,
    MalformedCapsulesParamsOverrides, MalformedCapsulesPayloadOverrides,
    build_malformed_capsules_case_deps, build_malformed_capsules_case_params,
    build_malformed_capsules_case_payload, build_malformed_capsules_deps,
    build_malformed_capsules_params, build_malformed_capsules_payload, run_malformed_capsules_case,
};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: own entry
///   `every_record_holds_a_well_formed_capsule_and_an_ill_formed_displacement`;
///   `malformed_capsules` over a case with a count above one, every adapter
///   real and the operating-system source real → every record's capsule is
///   well formed and its displaced capsule is not, in the record's verdicts
///   and under the KEM's own check.
/// Arrange: `count` of `2`; every case — every member of
///   `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every selection and scope of
///   `DECLARED_KEM_SELECTIONS`, the case payload's defaults.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  `vectors` holds `2` records, each with `is_well_formed` `true`
///   and `displaced_is_well_formed` `false`, and `is_well_formed` over
///   `build_is_well_formed_params` and `build_is_well_formed_payload` with
///   the record's parameter set returns `true` for `capsule` and `false`
///   for `displaced`; the outermost `Result` `is_ok()` per case, proving
///   the runner's levels ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
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

/// Contract: own entry `every_record_draw_replays_to_the_values_it_holds`;
///   the record's `draws`, split at `UNIFORM_BYTES_LENGTH` into the
///   uniforms of its calls in draw order and replayed through the case's
///   KEM under the admitted identity's setup scope, reproduce the record's
///   parameter set and capsule; the displaced capsule carries the
///   capsule's other components and the parameter set's `g1` as `v`; the
///   capsule's `u` is the second-group generator multiplied by the
///   encapsulation scalar.
/// Arrange: `count` of `1`; every case, run under the admitted entitlement
///   identity and under the admitted asset identities over the roots
///   `build_root` builds from `[0x44; 32]` and `[0x45; 32]`, so a replay
///   under another asset identity reports otherwise.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  the encoded `hpub` of the replayed parameter set and the
///   encoded members of the replayed capsule's components each equal the
///   record's; the record's `displaced`, read through `capsule_components`,
///   holds the encoded `u`, and under the entitlement scope `w`, of the
///   record's capsule and a `v` whose `encode_g1` bytes equal those of the
///   record's parameter set's `g1`, read through
///   `parameter_set_components`; the `encode_g2` bytes of the record's
///   capsule's `u` equal those of `mul_g2` of `g2_generator` by the scalar
///   `sample_from_uniform_bytes` returns for the fourth uniform; the
///   outermost `Result` `is_ok()` per case, proving the runner's levels
///   ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_values_it_holds() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
            &self,
            pairing: &P,
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
            let record = success.vectors.first().expect("the one record");
            // Arrange the replay
            let uniform_bytes = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let (master, rest) = record.draws.split_at(uniform_bytes);
            let (u0, rest) = rest.split_at(uniform_bytes);
            let (u1, encapsulation) = rest.split_at(uniform_bytes);
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(build_secret(master.to_vec(), Default::default())),
                        u0_uniform: Some(build_secret(u0.to_vec(), Default::default())),
                        u1_uniform: Some(build_secret(u1.to_vec(), Default::default())),
                    }),
                )
                .expect("the replay's setup succeeds");
            let encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(build_secret(encapsulation.to_vec(), Default::default())),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
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
            let expected_parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the replay's parameter set's components are infallible")
                .components;
            let actual_parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &record.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the record's parameter set's components are infallible")
                .components;
            assert_eq!(
                encode_g2(&expected_parameter_set.hpub),
                encode_g2(&actual_parameter_set.hpub)
            );
            let components_of = |capsule: &K::Capsule| {
                kem.capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(capsule, Default::default()),
                )
                .expect("a capsule's components are infallible")
                .components
            };
            let encodings_of =
                |components: &CapsuleComponents<P::G1, P::G2>| -> (Vec<u8>, Vec<u8>, Option<Vec<u8>>) {
                    match components {
                        CapsuleComponents::Entitlement { u, v, w } => {
                            (encode_g2(u), encode_g1(v), Some(encode_g1(w)))
                        }
                        CapsuleComponents::Asset { u, v } => {
                            (encode_g2(u), encode_g1(v), None)
                        }
                    }
                };
            let expected_capsule = components_of(&encapsulated.capsule);
            let actual_capsule = components_of(&record.capsule);
            let actual_displaced = components_of(&record.displaced);
            assert_eq!(
                core::mem::discriminant(&actual_capsule),
                core::mem::discriminant(&expected_capsule)
            );
            assert_eq!(
                core::mem::discriminant(&actual_displaced),
                core::mem::discriminant(&expected_capsule)
            );
            let expected = encodings_of(&expected_capsule);
            assert_eq!(encodings_of(&actual_capsule), expected);
            let displaced = encodings_of(&actual_displaced);
            assert_eq!(displaced.0, expected.0);
            assert_eq!(displaced.2, expected.2);
            assert_eq!(displaced.1, encode_g1(&actual_parameter_set.g1));
            let encapsulation_scalar =
                <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(encapsulation.to_vec(), Default::default()),
                    },
                )
                .expect("the fourth uniform samples")
                .scalar
                .expose()
                .clone();
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(expected_u) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: encapsulation_scalar,
                },
            );
            assert_eq!(encode_g2(&expected_u.product), expected.0);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                let payloads: Vec<MalformedCapsulesCasePayloadOverrides> = match kem_selection.scope
                {
                    IdentityScope::Entitlement => vec![Default::default()],
                    IdentityScope::Asset => vec![
                        MalformedCapsulesCasePayloadOverrides {
                            root: Some(build_root(RootConstructorParamsOverrides {
                                bytes: Some([0x44; 32]),
                            })),
                            ..Default::default()
                        },
                        MalformedCapsulesCasePayloadOverrides {
                            root: Some(build_root(RootConstructorParamsOverrides {
                                bytes: Some([0x45; 32]),
                            })),
                            ..Default::default()
                        },
                    ],
                };
                for overrides in payloads {
                    // Act
                    let returned = run_malformed_capsules_case(
                        &build_malformed_capsules_case_deps(
                            CaseConsumer { count },
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
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                            },
                        ),
                        build_malformed_capsules_case_payload(overrides),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
    }
    // every_record_draw_replays_to_the_values_it_holds
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`;
///   the list holds the configured count, each record's `draws` hold its
///   four uniforms each `UNIFORM_BYTES_LENGTH` long, and no two records
///   share `draws`.
/// Arrange: `count` of `3`; every case.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; the outermost `Result` `is_ok()` per
///   case, proving the runner's levels ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
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

/// Contract: own entry
///   `a_capsule_rebuilt_from_its_components_without_displacement_is_well_formed`;
///   each record's capsule, read through `capsule_components` and rebuilt
///   through `capsule_from_components` with no component changed, is well
///   formed under the record's parameter set, and the record's displaced
///   capsule is not.
/// Arrange: `count` of `2`; every case; the rebuild over
///   `build_capsule_components_params`, `build_capsule_components_payload`,
///   `build_capsule_from_components_params`, and
///   `build_capsule_from_components_payload` with the components read from
///   the record's `capsule`.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  for each record `is_well_formed` over the record's parameter
///   set returns `true` for the rebuild and `false` for `displaced`; the
///   outermost `Result` `is_ok()` per case, proving the runner's levels
///   ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_capsule_rebuilt_from_its_components_without_displacement_is_well_formed() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
            &self,
            pairing: &P,
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
            for record in &success.vectors {
                let components = kem
                    .capsule_components(
                        build_capsule_components_params(Default::default()),
                        build_capsule_components_payload(&record.capsule, Default::default()),
                    )
                    .expect("the capsule's components are infallible")
                    .components;
                let rebuilt = kem
                    .capsule_from_components(
                        build_capsule_from_components_params(Default::default()),
                        build_capsule_from_components_payload(
                            pairing,
                            CapsuleFromComponentsPayloadOverrides {
                                components: Some(components),
                            },
                        ),
                    )
                    .expect("the rebuild is infallible")
                    .capsule;
                let Ok(rebuilt_verdict) = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &record.parameter_set,
                        &rebuilt,
                        Default::default(),
                    ),
                );
                assert!(rebuilt_verdict.is_well_formed);
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
    // a_capsule_rebuilt_from_its_components_without_displacement_is_well_formed
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`;
///   a hash-to-scalar refusal reaches the caller inside the function's
///   `Setup` variant under the asset scope, whole, and the entitlement
///   scope hashes nothing.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES` and
///   every selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, the
///   case params' `hash_to_scalar` from `build_create_hash_to_scalar_params`
///   overriding `concrete` with `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`; the case
///   payload's defaults; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  under the asset scope the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` whole, `error` the expected literal; under the
///   entitlement scope the result `is_ok()`; the outermost `Result`
///   `is_ok()` per case, proving the runner's levels ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
/// Mocked:  nothing at the outer edge — the operating-system generator runs
///   real; the hash-to-scalar family's mock concrete is selected by
///   configuration.
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
