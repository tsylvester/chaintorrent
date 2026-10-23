#![allow(clippy::expect_used)]

use domain::build_secret;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    create_hash_to_scalar, expected_uniform_bytes,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn,
    DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    IssuePayloadOverrides, KemIdentity, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_create_kem_deps, build_create_kem_payload, build_credential_components_params,
    build_credential_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_issue_params, build_issue_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    AddG1Params, AddG1Payload, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};

use super::interface::NonConvertibilityErrorReturn;
use super::mock::{
    NonConvertibilityParamsOverrides, build_non_convertibility_deps,
    build_non_convertibility_params, build_non_convertibility_payload,
};
use super::non_convertibility;
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: own entry
///   `every_record_holds_a_true_verdict_for_its_own_identity_and_a_false_one_for_another`;
///   `non_convertibility` under the entitlement scope over a case with a
///   count above one and two distinct identities, every adapter real and
///   the operating-system source real → every record's verdict against its
///   own identity's element is `true` and against the other's is `false`;
///   with the other identity equal to the configured one, both are `true`.
/// Arrange: `count` of `2`; every case of the private surface — every
///   member of `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every entitlement-scope
///   selection of `DECLARED_KEM_SELECTIONS`, the identity `b"pkg@1.0.0#1"`;
///   each case run with the other identity `b"pkg@2.0.0#7"` and again with
///   the other identity `b"pkg@1.0.0#1"`, the configured identity's bytes.
/// Act:     `non_convertibility`, once per pair of identities, in the KEM
///   consumer's method.
/// Assert:  under the distinct identities `vectors` holds `2` records, each
///   with `is_valid` `true` and `other_is_valid` `false`; under the equal
///   identities each record's `is_valid` and `other_is_valid` are `true`;
///   each factory's result is unpacked by `expect` naming the declared
///   selection and the outermost `Result` `is_ok()`, proving the consumers
///   ran.
/// Boundary: `non_convertibility` and the chain of real functions the
///   private surface names — `create_random_source` for the
///   operating-system kind, `create_pairing`, `create_hash_to_scalar`,
///   `create_kem`, and every adapter method the function calls, all running
///   for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_holds_a_true_verdict_for_its_own_identity_and_a_false_one_for_another() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
        expected_other_is_valid: bool,
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
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.is_valid);
                assert_eq!(record.other_is_valid, self.expected_other_is_valid);
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
                for kem_selection in DECLARED_KEM_SELECTIONS
                    .iter()
                    .filter(|selection| selection.scope == IdentityScope::Entitlement)
                {
                    let identity: &[u8] = b"pkg@1.0.0#1";
                    for (other_identity, expected_other_is_valid) in
                        [(b"pkg@2.0.0#7" as &[u8], false), (identity, true)]
                    {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    subject: &*subject_source.adapter,
                                    count: self.count,
                                    identity,
                                    other_identity,
                                    expected_other_is_valid,
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
    // every_record_holds_a_true_verdict_for_its_own_identity_and_a_false_one_for_another
}

/// Contract: own entry
///   `every_record_draw_replays_to_the_credential_and_verdicts_it_holds`;
///   the record's `draws`, split at `UNIFORM_BYTES_LENGTH` into the uniforms
///   of its calls in draw order and replayed through the case's KEM over
///   the configured identities, reproduce the record's credential; the
///   record's credential is valid under the replayed parameter set for the
///   configured identity's replayed element and not for the other's; its
///   `b` is the second-group generator multiplied by the issuance scalar;
///   the configured identity's element is `u0` added to `u1` multiplied by
///   its scalar; and that scalar is the identity mapping under the declared
///   tag.
/// Arrange: `count` of `1`; every case of the private surface, each run
///   under the identity pairs `b"pkg@1.0.0#1"` with `b"pkg@2.0.0#7"` and
///   `b"pkg@3.0.0#2"` with `b"pkg@4.0.0#9"`, so a replay over an identity
///   other than the configured one reports otherwise.
/// Act:     `non_convertibility`, once per case and identity pair, in the
///   KEM consumer's method.
/// Assert:  the encoded `a` and `b` of the replayed credential, read through
///   `credential_components`, each equal the record's `credential`'s;
///   `is_valid` over `build_is_valid_params` and `build_is_valid_payload`
///   with the replayed parameter set, the configured identity's replayed
///   element, and the record's `credential` returns `true`, and with the
///   other identity's replayed element returns `false`; the `encode_g2`
///   bytes of the record's `credential`'s `b` equal those of `mul_g2` of
///   `g2_generator` by the scalar `sample_from_uniform_bytes` returns for
///   the fourth uniform; the `encode_g1` bytes of the configured identity's
///   replayed element's `element`, read through `identity_element_components`,
///   equal those of `add_g1` of the `u0` and `mul_g1` of the `u1` that
///   `parameter_set_components` reads from the replayed parameter set by
///   that element's `scalar`; for each selection declaring
///   `HashToScalarIdentifier::Keccak256V1`, `encode_scalar` of the
///   configured identity's replayed element's `scalar` equals
///   `encode_scalar` of the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds with
///   `DomainTagConstructorParamsOverrides { bytes: Some(K::DECLARATION.
///   identity_tag.to_vec()) }`, the configured identity bytes, and
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`; each
///   factory's result is unpacked by `expect` naming the declared selection
///   and the outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: `non_convertibility` and the chain of real functions the
///   private surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_credential_and_verdicts_it_holds() {
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
        h2s_identifier: HashToScalarIdentifier,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
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
            let mut uniforms = record
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                .map(|chunk| build_secret(chunk.to_vec(), Default::default()));
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Entitlement),
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
            let other_derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Entitlement {
                                canonical: self.other_identity,
                            }),
                        },
                    ),
                )
                .expect("the replay's other identity derivation succeeds");
            let issued = payload
                .adapter
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
                                    .expect("the record's draws hold four uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's issuance succeeds");
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
            let expected_credential = components_of(&issued.credential);
            let actual_credential = components_of(&record.credential);
            assert_eq!(
                encode_g1(&actual_credential.a),
                encode_g1(&expected_credential.a)
            );
            assert_eq!(
                encode_g2(&actual_credential.b),
                encode_g2(&expected_credential.b)
            );
            let Ok(own_verdict) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &record.credential,
                    Default::default(),
                ),
            );
            assert!(own_verdict.is_valid);
            let Ok(other_verdict) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &other_derived.identity_element,
                    &record.credential,
                    Default::default(),
                ),
            );
            assert!(!other_verdict.is_valid);
            let issuance_scalar = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
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
            let Ok(expected_b) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: issuance_scalar,
                },
            );
            assert_eq!(
                encode_g2(&actual_credential.b),
                encode_g2(&expected_b.product)
            );
            let parameter_set = payload
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
            let identity_element = payload
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
            let Ok(identity_inner) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: parameter_set.u1.clone(),
                    scalar: identity_element.scalar.clone(),
                },
            );
            let Ok(expected_identity_element) = self.pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: parameter_set.u0.clone(),
                    right: identity_inner.product,
                },
            );
            assert_eq!(
                encode_g1(&identity_element.element),
                encode_g1(&expected_identity_element.sum)
            );
            if self.h2s_identifier == HashToScalarIdentifier::Keccak256V1 {
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                });
                let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(
                                &tag,
                                self.identity,
                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            ),
                            Default::default(),
                        ),
                    },
                )
                .expect("the expected uniform bytes sample")
                .scalar
                .expose()
                .clone();
                assert_eq!(
                    encode_scalar(&identity_element.scalar),
                    encode_scalar(&expected)
                );
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
                for kem_selection in DECLARED_KEM_SELECTIONS
                    .iter()
                    .filter(|selection| selection.scope == IdentityScope::Entitlement)
                {
                    for (identity, other_identity) in [
                        (b"pkg@1.0.0#1" as &[u8], b"pkg@2.0.0#7" as &[u8]),
                        (b"pkg@3.0.0#2" as &[u8], b"pkg@4.0.0#9" as &[u8]),
                    ] {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    pairing: &payload.adapter,
                                    subject: &*subject_source.adapter,
                                    count: self.count,
                                    identity,
                                    other_identity,
                                    h2s_identifier: h2s_selection.identifier,
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
    // every_record_draw_replays_to_the_credential_and_verdicts_it_holds
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`;
///   the list holds the configured count, each record's `draws` hold its
///   four uniforms each `UNIFORM_BYTES_LENGTH` long, and no two records
///   share `draws`.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES` crossed
///   with the declared hash-to-scalar selections and every entitlement-scope
///   KEM selection, the identity `b"pkg@1.0.0#1"` and the other identity
///   `b"pkg@2.0.0#7"`.
/// Act:     `non_convertibility`, once per case, in the KEM consumer's
///   method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: `non_convertibility` and the chain of real functions the
///   private surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
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
                for kem_selection in DECLARED_KEM_SELECTIONS
                    .iter()
                    .filter(|selection| selection.scope == IdentityScope::Entitlement)
                {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: b"pkg@1.0.0#1",
                                other_identity: b"pkg@2.0.0#7",
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

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`;
///   a hash-to-scalar refusal reaches the caller inside the function's
///   `DeriveIdentity` variant, whole.
/// Arrange: `count` of `1`; each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   with `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `non_convertibility`, once per case, in the KEM consumer's
///   method.
/// Assert:  the result's `Err` equals
///   `Some(NonConvertibilityErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(error)))` whole; each factory's
///   result is unpacked by `expect` naming the declared selection and the
///   outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: `non_convertibility` and the chain of real functions the
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
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
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
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::HashToScalar(expected)
                ))
            );
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
            for kem_selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| selection.scope == IdentityScope::Entitlement)
            {
                let created = create_kem(
                    &build_create_kem_deps(
                        &payload.adapter,
                        &*hashing.adapter,
                        KemConsumer {
                            subject: &*subject_source.adapter,
                            count: self.count,
                            identity: b"pkg@1.0.0#1",
                            other_identity: b"pkg@2.0.0#7",
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
