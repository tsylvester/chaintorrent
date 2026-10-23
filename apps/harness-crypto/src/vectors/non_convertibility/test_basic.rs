#![allow(clippy::expect_used)]

use hash_to_scalar::{
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides, CreateKemReturn,
    DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    IssuePayloadOverrides, KemIdentity, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_create_kem_deps, build_create_kem_params, build_create_kem_payload,
    build_credential_components_params, build_credential_components_payload,
    build_derive_identity_params, build_derive_identity_payload, build_issue_params,
    build_issue_payload, build_setup_params, build_setup_payload, create_kem,
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

use super::mock::{
    NonConvertibilityParamsOverrides, build_non_convertibility_deps,
    build_non_convertibility_params, build_non_convertibility_payload,
};
use super::non_convertibility;
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(NonConvertibilitySuccessReturn { vectors })`, a
///   record for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3` through `build_vector_count`, a count other than
///   the params builder's default `2`, so a list sized from the default
///   reports otherwise; the scope `IdentityScope::Entitlement`, the identity
///   `b"pkg@1.0.0#1"`, and the other identity `b"pkg@2.0.0#7"`, the defaults.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`, so a list of
///   another length fails; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn non_convertibility_emits_the_configured_count_of_records() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
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
        other_identity: &'a [u8],
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
                        other_identity: self.other_identity,
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
        other_identity,
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
    // non_convertibility_emits_the_configured_count_of_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are the master, `u0`,
///   `u1`, and issuance uniforms concatenated.
/// Arrange: `count` of `1`; the scope `IdentityScope::Entitlement`, the
///   identity `b"pkg@1.0.0#1"`, and the other identity `b"pkg@2.0.0#7"`,
///   the defaults.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  the record's `draws` hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`, so a draw of another length or count fails; the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn non_convertibility_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
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
        other_identity: &'a [u8],
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
                        other_identity: self.other_identity,
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
        other_identity,
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
    // non_convertibility_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the records in index order, within a record its
///   draws in the order master, `u0`, `u1`, and issuance.
/// Arrange: `count` of `2`; the expected draws four consecutive
///   `fill_bytes` calls of the independent source — a separate adapter of
///   the family's mock concrete at the same defaults as the subject's, whose
///   `fill_bytes` yields, call by call, the bytes the subject's source
///   yields — grouped for each record in turn, so a transposed record or
///   draw reports otherwise; the scope `IdentityScope::Entitlement`, the
///   identity `b"pkg@1.0.0#1"`, and the other identity `b"pkg@2.0.0#7"`,
///   the defaults.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn non_convertibility_records_the_draws_in_the_sequence_the_source_yields() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
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
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
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
        other_identity: &'a [u8],
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
                        other_identity: self.other_identity,
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
        other_identity,
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
    // non_convertibility_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Scope under `Entitlement`, Record, and Record outcome → the
///   record holds the credential `issue` returned over its draws for the
///   configured identity's element, a `true` verdict against that element,
///   and a `false` verdict against the other identity's element.
/// Arrange: `count` of `1`; the scope `IdentityScope::Entitlement`, the
///   identity `b"pkg@1.0.0#1"`, and the other identity `b"pkg@2.0.0#7"`,
///   the defaults; the replay over the independent source's first four
///   draws under `SetupScope::Entitlement` and
///   `KemIdentity::Entitlement { canonical: &identity }`, each replayed call
///   through the family's params and payload builders over the received KEM
///   and the replayed credential read through `credential_components`.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  the encoded `a` and `b` of the record's `credential` each equal
///   the replay's by `assert_eq!`, `is_valid` is `true`, and
///   `other_is_valid` is `false`; the outermost `Result` `is_ok()`, proving
///   the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source and the independent source, each the
///   family's mock concrete at the default selection.
#[test]
fn non_convertibility_records_the_credential_and_verdicts_its_draws_produce() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
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
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
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
            let expected = components_of(&issued.credential);
            let actual = components_of(&record.credential);
            assert_eq!(encode_g1(&actual.a), encode_g1(&expected.a));
            assert_eq!(encode_g2(&actual.b), encode_g2(&expected.b));
            assert!(record.is_valid);
            assert!(!record.other_is_valid);
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        other_identity: &'a [u8],
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
                        other_identity: self.other_identity,
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
        other_identity,
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
    // non_convertibility_records_the_credential_and_verdicts_its_draws_produce
}

/// Contract: Computed, `other_is_valid` → `true` when the other identity
///   bytes equal the configured identity bytes.
/// Arrange: `count` of `1`; the scope `IdentityScope::Entitlement`, the
///   identity `b"pkg@1.0.0#1"`, and the other identity `b"pkg@1.0.0#1"`,
///   the configured identity's bytes, against the default other identity
///   under which the verdict is `false`.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  `is_valid` and `other_is_valid` are each `true`, so a verdict
///   held as a constant reports otherwise; the outermost `Result` `is_ok()`,
///   proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn non_convertibility_records_a_true_verdict_against_the_other_identity_when_it_is_the_configured_one()
 {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@1.0.0#1";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert!(record.is_valid);
            assert!(record.other_is_valid);
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        other_identity: &'a [u8],
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
                        other_identity: self.other_identity,
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
        other_identity,
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
    // non_convertibility_records_a_true_verdict_against_the_other_identity_when_it_is_the_configured_one
}

/// Contract: Scope under `Asset` → `Ok(NonConvertibilitySuccessReturn
///   { vectors })`, `vectors` empty.
/// Arrange: the scope `IdentityScope::Asset` and `count` of `3`, a count
///   that yields three records under the entitlement scope, so a scope
///   deciding after a record reports otherwise; the identity
///   `b"pkg@1.0.0#1"` and the other identity `b"pkg@2.0.0#7"`, the
///   defaults.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  `vectors` is empty by `assert!`; the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn non_convertibility_returns_an_empty_set_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    let scope = IdentityScope::Asset;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert!(success.vectors.is_empty());
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        other_identity: &'a [u8],
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
                        other_identity: self.other_identity,
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
        other_identity,
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
    // non_convertibility_returns_an_empty_set_under_the_asset_scope
}
