#![allow(clippy::expect_used)]

use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParamsOverrides,
    build_asset_identity_hash,
};
use hash_to_scalar::{
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    CapsuleComponents, ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides,
    CreateKemReturn, EncapsulatePayloadOverrides, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_capsule_components_params, build_capsule_components_payload, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, build_encapsulate_params,
    build_encapsulate_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};

use super::malformed_capsules;
use super::mock::{
    MalformedCapsulesParamsOverrides, build_malformed_capsules_deps,
    build_malformed_capsules_params, build_malformed_capsules_payload,
};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(MalformedCapsulesSuccessReturn { vectors })`, a
///   record for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3` through `build_vector_count`, a count other than
///   the params builder's default `2`, so a list sized from the default
///   reports otherwise; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`, so a list of
///   another length fails; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn malformed_capsules_emits_the_configured_count_of_records() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";

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
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        count: self.count,
                        identity: self.identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // malformed_capsules_emits_the_configured_count_of_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are the master, `u0`,
///   `u1`, and encapsulation uniforms concatenated.
/// Arrange: `count` of `1`; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the record's `draws` hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`, so a draw of another length or count fails; the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn malformed_capsules_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";

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
            assert_eq!(
                success.vectors.first().expect("the one record").draws.len(),
                4 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        count: self.count,
                        identity: self.identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // malformed_capsules_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the records in index order, within a record its
///   draws in the order master, `u0`, `u1`, and encapsulation.
/// Arrange: `count` of `2`; the expected draws four consecutive
///   `fill_bytes` calls of the independent source — a separate adapter of
///   the family's mock concrete at the same defaults as the subject's, whose
///   `fill_bytes` yields, call by call, the bytes the subject's source
///   yields — grouped for each record in turn, so a transposed record or
///   draw reports otherwise; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn malformed_capsules_records_the_draws_in_the_sequence_the_source_yields() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
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
            let mut expected: Vec<Vec<u8>> = Vec::new();
            for _ in 0..self.count.get() {
                let mut draws = Vec::new();
                for _ in 0..4 {
                    let filled = self
                        .independent
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
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            for (record, expected_draws) in success.vectors.iter().zip(expected.iter()) {
                assert_eq!(&record.draws, expected_draws);
            }
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            let independent_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        independent: &*independent_source.adapter,
                        count: self.count,
                        identity: self.identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // malformed_capsules_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Scope under `Entitlement`, Record, Displacement, Verdicts, and
///   Record outcome → the record holds the parameter set and capsule its
///   calls returned over its draws, the capsule rebuilt as
///   `Entitlement { u, v: g1, w }`, a `true` verdict for the capsule, and a
///   `false` verdict for the displaced capsule.
/// Arrange: `count` of `1`; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults; the replay over the
///   independent source's first four draws under `SetupScope::Entitlement`,
///   each replayed call through the family's params and payload builders
///   over the received KEM and each replayed value read through its
///   components method.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the encoded `hpub` of the record's parameter set and the
///   encoded `u`, `v`, and `w` of its capsule each equal the replay's by
///   `assert_eq!`; the record's `displaced`, read through
///   `capsule_components`, is the `CapsuleComponents::Entitlement` variant
///   whose encoded `u` and `w` equal the replayed capsule's and whose
///   encoded `v` equals the encoding of the replayed parameter set's `g1`;
///   `is_well_formed` is `true` and `displaced_is_well_formed` is `false`;
///   the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn malformed_capsules_records_the_values_its_draws_produce_under_the_entitlement_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
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
            // Arrange
            let draw = || {
                self.independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Entitlement),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
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
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
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

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            let independent_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        subject: &*subject_source.adapter,
                        independent: &*independent_source.adapter,
                        count: self.count,
                        identity: self.identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // malformed_capsules_records_the_values_its_draws_produce_under_the_entitlement_scope
}

/// Contract: Scope under `Asset`, Record, Displacement, Verdicts, and
///   Record outcome → `AssetIdentityHash::try_new` admits the 32 bytes and
///   the setup scope is `SetupScope::Asset { identity_hash: &hash }`; the
///   record holds the values its calls returned over its draws, the capsule
///   rebuilt as `Asset { u, v: g1 }`, a `true` verdict for the capsule, and
///   a `false` verdict for the displaced capsule.
/// Arrange: the scope `IdentityScope::Asset`, the identity
///   `vec![0x44; 32]`, `count` of `1`; the replay as the entitlement block
///   builds it, under `SetupScope::Asset` over the hash
///   `build_asset_identity_hash` builds from the bytes `[0x44; 32]`.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the encoded `hpub` of the record's parameter set and the
///   encoded `u` and `v` of its capsule each equal the replay's by
///   `assert_eq!`; the record's `displaced`, read through
///   `capsule_components`, is the `CapsuleComponents::Asset` variant whose
///   encoded `u` equals the replayed capsule's and whose encoded `v` equals
///   the encoding of the replayed parameter set's `g1`; `is_well_formed` is
///   `true` and `displaced_is_well_formed` is `false`; the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn malformed_capsules_records_the_values_its_draws_produce_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let scope = IdentityScope::Asset;
    let identity: Vec<u8> = vec![0x44; ASSET_IDENTITY_HASH_LENGTH];
    let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
        bytes: Some([0x44; ASSET_IDENTITY_HASH_LENGTH]),
    });

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        identity_hash: &'a AssetIdentityHash,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let draw = || {
                self.independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Asset {
                            identity_hash: self.identity_hash,
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
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
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
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

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        identity_hash: &'a AssetIdentityHash,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            let independent_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        subject: &*subject_source.adapter,
                        independent: &*independent_source.adapter,
                        count: self.count,
                        identity: self.identity,
                        identity_hash: self.identity_hash,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity: &identity,
        identity_hash: &hash,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // malformed_capsules_records_the_values_its_draws_produce_under_the_asset_scope
}

/// Contract: Scope under `Entitlement` → `payload.identity` is not read, so
///   identity bytes the asset scope refuses yield records.
/// Arrange: `count` of `1` and the identity `vec![0x00; 31]`, bytes the
///   asset scope refuses by length, under the default entitlement scope.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the subject's return `is_ok()` and its `vectors` holds `1`
///   record by `assert_eq!`, so a call that read the identity reports
///   otherwise; the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn malformed_capsules_does_not_read_the_identity_under_the_entitlement_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let scope = IdentityScope::Entitlement;
    let identity: Vec<u8> = vec![0x00; 31];

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
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 1);
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        count: self.count,
                        identity: self.identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity: &identity,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // malformed_capsules_does_not_read_the_identity_under_the_entitlement_scope
}
