#![allow(clippy::expect_used)]

use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashConstructorParamsOverrides,
    build_asset_identity_hash, build_secret,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    CapsuleComponents, CapsuleFromComponentsPayloadOverrides, ConsumeKemParams, ConsumeKemPayload,
    DECLARED_KEM_SELECTIONS, EncapsulatePayloadOverrides, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, SetupErrorReturn, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_capsule_components_params, build_capsule_components_payload,
    build_capsule_from_components_params, build_capsule_from_components_payload,
    build_create_kem_deps, build_create_kem_payload, build_encapsulate_params,
    build_encapsulate_payload, build_is_well_formed_params, build_is_well_formed_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar, MulG2Params,
    MulG2Payload, PAIRING_CONCRETES, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};

use super::interface::MalformedCapsulesErrorReturn;
use super::malformed_capsules;
use super::mock::{
    MalformedCapsulesParamsOverrides, build_malformed_capsules_deps,
    build_malformed_capsules_params, build_malformed_capsules_payload,
};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: own entry
///   `every_record_holds_a_well_formed_capsule_and_an_ill_formed_displacement`;
///   `malformed_capsules` over a case with a count above one, every adapter
///   real and the operating-system source real → every record's capsule is
///   well formed and its displaced capsule is not, in the record's verdicts
///   and under the KEM's own check.
/// Arrange: `count` of `2`; every case of the private surface — every member
///   of `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every selection and scope of
///   `DECLARED_KEM_SELECTIONS`, the identity `b"pkg@1.0.0#1"` under the
///   entitlement scope and `vec![0x44; 32]` under the asset scope.
/// Act:     `malformed_capsules`, once per case, in the KEM consumer's
///   method.
/// Assert:  `vectors` holds `2` records, each with `is_well_formed` `true`
///   and `displaced_is_well_formed` `false`, and `is_well_formed` over
///   `build_is_well_formed_params` and `build_is_well_formed_payload` with
///   the record's parameter set returns `true` for `capsule` and `false`
///   for `displaced`; each factory's result is unpacked by `expect` naming
///   the declared selection and the outermost `Result` `is_ok()`, proving
///   the consumers ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names — `create_random_source` for the operating-system
///   kind, `create_pairing`, `create_hash_to_scalar`, `create_kem`, and
///   every adapter method the function calls, all running for real.
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

/// Contract: own entry `every_record_draw_replays_to_the_values_it_holds`;
///   the record's `draws`, split at `UNIFORM_BYTES_LENGTH` into the uniforms
///   of its calls in draw order and replayed through the case's KEM under
///   the configured identity's setup scope, reproduce the record's
///   parameter set and capsule; the displaced capsule carries the capsule's
///   other components and the parameter set's `g1` as `v`; the capsule's
///   `u` is the second-group generator multiplied by the encapsulation
///   scalar.
/// Arrange: `count` of `1`; every case of the private surface, each run
///   under the entitlement scope and under the asset scope with the
///   identities `vec![0x44; 32]` and `vec![0x45; 32]`, so a replay under
///   another asset identity reports otherwise.
/// Act:     `malformed_capsules`, once per case and identity, in the KEM
///   consumer's method.
/// Assert:  the encoded `hpub` of the replayed parameter set and the
///   encoded members of the replayed capsule's components each equal the
///   record's; the record's `displaced`, read through `capsule_components`,
///   holds the encoded `u`, and under the entitlement scope `w`, of the
///   record's capsule and a `v` whose `encode_g1` bytes equal those of the
///   record's parameter set's `g1`, read through `parameter_set_components`;
///   the `encode_g2` bytes of the record's capsule's `u` equal those of
///   `mul_g2` of `g2_generator` by the scalar `sample_from_uniform_bytes`
///   returns for the fourth uniform; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_values_it_holds() {
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
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
            let record = success.vectors.first().expect("the one record");
            // Assert
            let hash;
            let setup_scope = match payload.scope() {
                IdentityScope::Entitlement => SetupScope::Entitlement,
                IdentityScope::Asset => {
                    hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                        bytes: Some(
                            <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(self.identity)
                                .expect("the configured asset identity is a hash length"),
                        ),
                    });
                    SetupScope::Asset {
                        identity_hash: &hash,
                    }
                }
            };
            let mut uniforms = record
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                .map(|chunk| build_secret(chunk.to_vec(), Default::default()));
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold four uniforms"),
                        ),
                        u0_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold four uniforms"),
                        ),
                        u1_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold four uniforms"),
                        ),
                    }),
                )
                .expect("the replay's setup succeeds");
            let encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold four uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                self.pairing
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
                self.pairing
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
            let components_of = |capsule: &K::Capsule| {
                payload
                    .adapter
                    .capsule_components(
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
            let expected_parameter_set = payload
                .adapter
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            let actual_parameter_set = payload
                .adapter
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
            let expected = encodings_of(&components_of(&encapsulated.capsule));
            let actual = encodings_of(&components_of(&record.capsule));
            assert_eq!(actual, expected);
            let displaced = encodings_of(&components_of(&record.displaced));
            assert_eq!(displaced.0, actual.0);
            assert_eq!(displaced.2, actual.2);
            assert_eq!(displaced.1, encode_g1(&actual_parameter_set.g1));
            let encapsulation_scalar =
                <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            record
                                .draws
                                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                                .nth(3)
                                .expect("the record's draws hold four uniforms")
                                .to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the fourth uniform samples")
                .scalar
                .expose()
                .clone();
            let Ok(g2) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(expected_u) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: encapsulation_scalar,
                },
            );
            assert_eq!(actual.0, encode_g2(&expected_u.product));
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
                    let identities: Vec<Vec<u8>> = match kem_selection.scope {
                        IdentityScope::Entitlement => vec![b"pkg@1.0.0#1".to_vec()],
                        IdentityScope::Asset => vec![
                            vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                            vec![0x45; ASSET_IDENTITY_HASH_LENGTH],
                        ],
                    };
                    for identity in &identities {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    pairing: &payload.adapter,
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
    // every_record_draw_replays_to_the_values_it_holds
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`;
///   the list holds the configured count, each record's `draws` hold its
///   four uniforms each `UNIFORM_BYTES_LENGTH` long, and no two records
///   share `draws`.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES` crossed
///   with the declared hash-to-scalar and KEM selections, the identity of
///   each selection's scope.
/// Act:     `malformed_capsules`, once per case, in the KEM consumer's
///   method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
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
///   `a_capsule_rebuilt_from_its_components_without_displacement_is_well_formed`;
///   each record's capsule, read through `capsule_components` and rebuilt
///   through `capsule_from_components` with no component changed, is well
///   formed under the record's parameter set, and the record's displaced
///   capsule is not.
/// Arrange: `count` of `2`; every case of the private surface; the rebuild
///   over `build_capsule_components_params`,
///   `build_capsule_components_payload`,
///   `build_capsule_from_components_params`, and
///   `build_capsule_from_components_payload` with the components read from
///   the record's `capsule`.
/// Act:     `malformed_capsules`, once per case, in the KEM consumer's
///   method.
/// Assert:  for each record `is_well_formed` over the record's parameter set
///   returns `true` for the rebuild and `false` for `displaced`; each
///   factory's result is unpacked by `expect` naming the declared selection
///   and the outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_capsule_rebuilt_from_its_components_without_displacement_is_well_formed() {
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
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
            for record in &success.vectors {
                let components = payload
                    .adapter
                    .capsule_components(
                        build_capsule_components_params(Default::default()),
                        build_capsule_components_payload(&record.capsule, Default::default()),
                    )
                    .expect("the capsule's components are infallible")
                    .components;
                let rebuilt = payload
                    .adapter
                    .capsule_from_components(
                        build_capsule_from_components_params(Default::default()),
                        build_capsule_from_components_payload(
                            self.pairing,
                            CapsuleFromComponentsPayloadOverrides {
                                components: Some(components),
                            },
                        ),
                    )
                    .expect("the rebuild is infallible")
                    .capsule;
                let Ok(rebuilt_verdict) = payload.adapter.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &record.parameter_set,
                        &rebuilt,
                        Default::default(),
                    ),
                );
                assert!(rebuilt_verdict.is_well_formed);
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
                                pairing: &payload.adapter,
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
    // a_capsule_rebuilt_from_its_components_without_displacement_is_well_formed
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`;
///   a hash-to-scalar refusal reaches the caller inside the function's
///   `Setup` variant under the asset scope, whole, and the entitlement scope
///   hashes nothing.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   with `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `malformed_capsules`, once per case, in the KEM consumer's
///   method.
/// Assert:  under the asset scope the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` whole, `error` the expected literal; under the
///   entitlement scope the result `is_ok()`; each factory's result is
///   unpacked by `expect` naming the declared selection and the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Boundary: `malformed_capsules` and the chain of real functions the
///   private surface names.
/// Mocked:  nothing at the outer edge — the operating-system generator runs
///   real; the hash-to-scalar family's mock concrete is selected by
///   configuration.
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
