#![allow(clippy::expect_used)]

use hash_to_scalar::IHashToScalarAdapter;
use kem::{
    CapsuleComponents, CreateKemParamsOverrides, EncapsulatePayloadOverrides,
    ICredentialKemAdapter, IdentityScope, SetupParamsOverrides, SetupPayloadOverrides,
    build_capsule_components_params, build_capsule_components_payload, build_create_kem_params,
    build_encapsulate_params, build_encapsulate_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, build_fill_bytes_payload,
};

use super::interface::IMalformedCapsulesCaseConsumer;
use super::malformed_capsules;
use super::mock::{
    MalformedCapsulesCaseParamsOverrides, MalformedCapsulesParamsOverrides,
    MalformedCapsulesPayloadOverrides, build_malformed_capsules_case_deps,
    build_malformed_capsules_case_params, build_malformed_capsules_case_payload,
    build_malformed_capsules_deps, build_malformed_capsules_params,
    build_malformed_capsules_payload, run_malformed_capsules_case,
};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(MalformedCapsulesSuccessReturn { vectors })`, a
///   record for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3`, a count other than the params builder's default,
///   so a list sized from the default reports otherwise.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`, so a list of
///   another length fails; the outermost `Result` `is_ok()`, proving the
///   runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn malformed_capsules_emits_the_configured_count_of_records() {
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
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(Default::default()),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_emits_the_configured_count_of_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are the master, `u0`,
///   `u1`, and encapsulation uniforms concatenated.
/// Arrange: `count` of `1`.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  the record's `draws` hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`, so a draw of another length or count fails; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn malformed_capsules_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

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
            assert_eq!(
                success.vectors.first().expect("the one record").draws.len(),
                4 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(Default::default()),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the records in index order, within a record its
///   draws in the order master, `u0`, `u1`, and encapsulation.
/// Arrange: `count` of `2`; the expected draws four consecutive
///   `fill_bytes` calls of the independent source — a separate adapter of
///   the family's mock concrete at the same defaults as the subject's, whose
///   `fill_bytes` yields, call by call, the bytes the subject's source
///   yields — grouped for each record in turn, so a transposed record or
///   draw reports otherwise.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn malformed_capsules_records_the_draws_in_the_sequence_the_source_yields() {
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
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let mut expected: Vec<Vec<u8>> = Vec::new();
            for _ in 0..self.count.get() {
                let mut draws = Vec::new();
                for _ in 0..4 {
                    let filled = independent
                        .fill_bytes(
                            FillBytesParams,
                            build_fill_bytes_payload(FillBytesPayloadOverrides {
                                length: Some(
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                ),
                            }),
                        )
                        .expect("the independent source fills");
                    draws.extend_from_slice(filled.bytes.expose());
                }
                expected.push(draws);
            }
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
            for (record, expected_draws) in success.vectors.iter().zip(expected.iter()) {
                assert_eq!(&record.draws, expected_draws);
            }
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(Default::default()),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Record under an entitlement identity, Displacement, Verdicts,
///   and Record outcome → setup takes `payload.identity.setup_scope`,
///   `SetupScope::Entitlement`; the record holds the parameter set and
///   capsule its calls returned over its draws, the capsule rebuilt as
///   `Entitlement { u, v: g1, w }`, a `true` verdict for the capsule, and a
///   `false` verdict for the displaced capsule.
/// Arrange: `count` of `1`; the admitted entitlement identity over
///   `b"pkg@1.0.0#1"`; the replay over the independent source's first four
///   draws under that identity's `setup_scope`, each replayed call through
///   the family's params and payload builders over the received KEM and
///   each replayed value read through its components method.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  the encoded `hpub` of the record's parameter set and the
///   encoded `u`, `v`, and `w` of its capsule each equal the replay's by
///   `assert_eq!`; the record's `displaced`, read through
///   `capsule_components`, is the `CapsuleComponents::Entitlement` variant
///   whose encoded `u` and `w` equal the replayed capsule's and whose
///   encoded `v` equals the encoding of the replayed parameter set's `g1`;
///   `is_well_formed` is `true` and `displaced_is_well_formed` is `false`;
///   the outermost `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn malformed_capsules_records_the_values_its_draws_produce_under_the_entitlement_scope() {
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
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let draw = || {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("the replay's setup succeeds");
            let encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
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
                .expect("the parameter set's components are infallible")
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
                encode_g2(&actual_parameter_set.hpub),
                encode_g2(&expected_parameter_set.hpub)
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
            assert_eq!(displaced.1, encode_g1(&expected_parameter_set.g1));
            assert!(record.is_well_formed);
            assert!(!record.displaced_is_well_formed);
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(Default::default()),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_records_the_values_its_draws_produce_under_the_entitlement_scope
}

/// Contract: Record under an asset identity, Displacement, Verdicts, and
///   Record outcome → setup takes `payload.identity.setup_scope`,
///   `SetupScope::Asset { root }`; the record holds the values its calls
///   returned over its draws, the capsule rebuilt as `Asset { u, v: g1 }`,
///   a `true` verdict for the capsule, and a `false` verdict for the
///   displaced capsule.
/// Arrange: `scope` `IdentityScope::Asset`, `count` of `1`; the admitted
///   asset identity over the root `build_root` builds from `[0x44; 32]`,
///   the case payload's default; the replay as the entitlement block
///   builds it, under that identity's `setup_scope`.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  the encoded `hpub` of the record's parameter set and the
///   encoded `u` and `v` of its capsule each equal the replay's by
///   `assert_eq!`; the record's `displaced`, read through
///   `capsule_components`, is the `CapsuleComponents::Asset` variant whose
///   encoded `u` equals the replayed capsule's and whose encoded `v`
///   equals the encoding of the replayed parameter set's `g1`;
///   `is_well_formed` is `true` and `displaced_is_well_formed` is `false`;
///   the outermost `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the KEM's `scope`
///   selection named.
#[test]
fn malformed_capsules_records_the_values_its_draws_produce_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let kem_selection = build_create_kem_params(CreateKemParamsOverrides {
        scope: Some(IdentityScope::Asset),
        ..Default::default()
    });

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
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let draw = || {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("the replay's setup succeeds");
            let encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
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
                .expect("the parameter set's components are infallible")
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
                encode_g2(&actual_parameter_set.hpub),
                encode_g2(&expected_parameter_set.hpub)
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
            assert_eq!(displaced.1, encode_g1(&expected_parameter_set.g1));
            assert!(record.is_well_formed);
            assert!(!record.displaced_is_well_formed);
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
            kem: Some(kem_selection),
            ..Default::default()
        }),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_records_the_values_its_draws_produce_under_the_asset_scope
}
