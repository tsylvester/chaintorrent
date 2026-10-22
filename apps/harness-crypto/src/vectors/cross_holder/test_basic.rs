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
    CreateKemReturn, DeriveIdentityPayloadOverrides, EncapsulatePayloadOverrides,
    ICredentialKemAdapter, IKemConsumer, IdentityScope, IssuePayloadOverrides, KemIdentity,
    RerandomizePayloadOverrides, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_capsule_components_params, build_capsule_components_payload, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, build_credential_components_params,
    build_credential_components_payload, build_decapsulate_params, build_decapsulate_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_rerandomize_params, build_rerandomize_payload, build_setup_params, build_setup_payload,
    create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, ISampleUniformScalar, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};

use super::cross_holder;
use super::mock::{
    CrossHolderParamsOverrides, build_cross_holder_deps, build_cross_holder_params,
    build_cross_holder_payload,
};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(CrossHolderSuccessReturn { vectors })`, a record
///   for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3` through `build_vector_count`, a count other than
///   the params builder's default `2`, so a list sized from the default
///   reports otherwise; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults.
/// Act:     `cross_holder`, once, in the KEM consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`, so a list of
///   another length fails; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn cross_holder_emits_the_configured_count_of_records() {
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
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
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
    // cross_holder_emits_the_configured_count_of_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are the master, `u0`,
///   `u1`, issuance, rerandomization, and encapsulation uniforms
///   concatenated.
/// Arrange: `count` of `1`; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults.
/// Act:     `cross_holder`, once, in the KEM consumer's method.
/// Assert:  the record's `draws` hold `6` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`, so a draw of another length or count fails; the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn cross_holder_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
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
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(
                success.vectors.first().expect("the one record").draws.len(),
                6 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
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
    // cross_holder_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the records in index order, within a record its
///   draws in the order master, `u0`, `u1`, issuance, rerandomization, and
///   encapsulation.
/// Arrange: `count` of `2`; the expected draws six consecutive
///   `fill_bytes` calls of the independent source — a separate adapter of
///   the family's mock concrete at the same defaults as the subject's, whose
///   `fill_bytes` yields, call by call, the bytes the subject's source
///   yields — grouped for each record in turn, so a transposed record or
///   draw reports otherwise; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults.
/// Act:     `cross_holder`, once, in the KEM consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn cross_holder_records_the_draws_in_the_sequence_the_source_yields() {
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
                for _ in 0..6 {
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
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
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
    // cross_holder_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Identity under `Entitlement` and Record → the identity is
///   `KemIdentity::Entitlement { canonical: payload.identity }` and the
///   setup scope `SetupScope::Entitlement`; the record holds the parameter
///   set, identity element, issued credential, successor, capsule,
///   encapsulated value, and both decapsulated values its calls returned
///   over its draws.
/// Arrange: `count` of `1`; the scope `IdentityScope::Entitlement` and the
///   identity `b"pkg@1.0.0#1"`, the defaults; the replay over the
///   independent source's first six draws under `SetupScope::Entitlement`
///   and `KemIdentity::Entitlement { canonical: &identity }`, each replayed
///   call through the family's params and payload builders over the received
///   KEM and each replayed value read through its components method.
/// Act:     `cross_holder`, once, in the KEM consumer's method.
/// Assert:  the encoded `hpub` of the record's parameter set, the encoded
///   `scalar` and `element` of its identity element, the encoded `a` and `b`
///   of `issued` and of `successor`, and the encoded members of its
///   capsule's components each equal the replay's by `assert_eq!`, and the
///   `key_material().expose()` bytes of `encapsulated`, `issued_decapsulated`,
///   and `successor_decapsulated` each equal the replay's; the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn cross_holder_records_the_values_its_draws_produce_under_the_entitlement_scope() {
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
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Entitlement {
                                canonical: self.identity,
                            }),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's issuance succeeds");
            let rerandomized = payload
                .adapter
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        RerandomizePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
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
            let issued_decapsulated = payload
                .adapter
                .decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &derived.identity_element,
                        &issued.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                )
                .expect("the replay's decapsulation is infallible");
            let successor_decapsulated = payload
                .adapter
                .decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &derived.identity_element,
                        &rerandomized.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                )
                .expect("the replay's decapsulation is infallible");
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
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
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                self.pairing
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
            let expected_identity_element = payload
                .adapter
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let actual_identity_element = payload
                .adapter
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &record.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the record's identity element's components are infallible")
                .components;
            assert_eq!(
                encode_scalar(&actual_identity_element.scalar),
                encode_scalar(&expected_identity_element.scalar)
            );
            assert_eq!(
                encode_g1(&actual_identity_element.element),
                encode_g1(&expected_identity_element.element)
            );
            let components_of = |credential: &K::Credential| {
                payload
                    .adapter
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(credential, Default::default()),
                    )
                    .expect("a credential's components are infallible")
                    .components
            };
            let expected_issued = components_of(&issued.credential);
            let actual_issued = components_of(&record.issued);
            assert_eq!(encode_g1(&actual_issued.a), encode_g1(&expected_issued.a));
            assert_eq!(encode_g2(&actual_issued.b), encode_g2(&expected_issued.b));
            let expected_successor = components_of(&rerandomized.credential);
            let actual_successor = components_of(&record.successor);
            assert_eq!(
                encode_g1(&actual_successor.a),
                encode_g1(&expected_successor.a)
            );
            assert_eq!(
                encode_g2(&actual_successor.b),
                encode_g2(&expected_successor.b)
            );
            let expected_capsule = payload
                .adapter
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&encapsulated.capsule, Default::default()),
                )
                .expect("the capsule's components are infallible")
                .components;
            let actual_capsule = payload
                .adapter
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&record.capsule, Default::default()),
                )
                .expect("the record's capsule's components are infallible")
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
                record.encapsulated.key_material().expose(),
                encapsulated.encapsulated.key_material().expose()
            );
            assert_eq!(
                record.issued_decapsulated.key_material().expose(),
                issued_decapsulated.encapsulated.key_material().expose()
            );
            assert_eq!(
                record.successor_decapsulated.key_material().expose(),
                successor_decapsulated.encapsulated.key_material().expose()
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
    // cross_holder_records_the_values_its_draws_produce_under_the_entitlement_scope
}

/// Contract: Identity under `Asset` and Record → `AssetIdentityHash::try_new`
///   admits the 32 bytes, the identity is
///   `KemIdentity::Asset { identity_hash: &hash }` and the setup scope
///   `SetupScope::Asset { identity_hash: &hash }`; the record holds the
///   values its calls returned over its draws.
/// Arrange: the scope `IdentityScope::Asset`, the identity
///   `vec![0x44; 32]`, `count` of `1`; the replay as the entitlement block
///   builds it, over the hash `build_asset_identity_hash` builds from the
///   bytes `[0x44; 32]`, `SetupScope::Asset`, and `KemIdentity::Asset`.
/// Act:     `cross_holder`, once, in the KEM consumer's method.
/// Assert:  as the entitlement block, against this replay; the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn cross_holder_records_the_values_its_draws_produce_under_the_asset_scope() {
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
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Asset {
                                identity_hash: self.identity_hash,
                            }),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's issuance succeeds");
            let rerandomized = payload
                .adapter
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        RerandomizePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
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
            let issued_decapsulated = payload
                .adapter
                .decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &derived.identity_element,
                        &issued.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                )
                .expect("the replay's decapsulation is infallible");
            let successor_decapsulated = payload
                .adapter
                .decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &derived.identity_element,
                        &rerandomized.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                )
                .expect("the replay's decapsulation is infallible");
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
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
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                self.pairing
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
            let expected_identity_element = payload
                .adapter
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let actual_identity_element = payload
                .adapter
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &record.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the record's identity element's components are infallible")
                .components;
            assert_eq!(
                encode_scalar(&actual_identity_element.scalar),
                encode_scalar(&expected_identity_element.scalar)
            );
            assert_eq!(
                encode_g1(&actual_identity_element.element),
                encode_g1(&expected_identity_element.element)
            );
            let components_of = |credential: &K::Credential| {
                payload
                    .adapter
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(credential, Default::default()),
                    )
                    .expect("a credential's components are infallible")
                    .components
            };
            let expected_issued = components_of(&issued.credential);
            let actual_issued = components_of(&record.issued);
            assert_eq!(encode_g1(&actual_issued.a), encode_g1(&expected_issued.a));
            assert_eq!(encode_g2(&actual_issued.b), encode_g2(&expected_issued.b));
            let expected_successor = components_of(&rerandomized.credential);
            let actual_successor = components_of(&record.successor);
            assert_eq!(
                encode_g1(&actual_successor.a),
                encode_g1(&expected_successor.a)
            );
            assert_eq!(
                encode_g2(&actual_successor.b),
                encode_g2(&expected_successor.b)
            );
            let expected_capsule = payload
                .adapter
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&encapsulated.capsule, Default::default()),
                )
                .expect("the capsule's components are infallible")
                .components;
            let actual_capsule = payload
                .adapter
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&record.capsule, Default::default()),
                )
                .expect("the record's capsule's components are infallible")
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
                record.encapsulated.key_material().expose(),
                encapsulated.encapsulated.key_material().expose()
            );
            assert_eq!(
                record.issued_decapsulated.key_material().expose(),
                issued_decapsulated.encapsulated.key_material().expose()
            );
            assert_eq!(
                record.successor_decapsulated.key_material().expose(),
                successor_decapsulated.encapsulated.key_material().expose()
            );
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
    // cross_holder_records_the_values_its_draws_produce_under_the_asset_scope
}
