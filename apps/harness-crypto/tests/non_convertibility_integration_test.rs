#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{build_derivation_context, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_mock_i_encoding_consumer, create_encoding,
};
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
    IssuePayloadOverrides, KemIdentity, SetupErrorReturn, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_create_kem_deps, build_create_kem_payload,
    build_credential_components_params, build_credential_components_payload,
    build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    AddG1Params, AddG1Payload, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, FillBytesParams,
    FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};

use harness_crypto::{
    NonConvertibilityErrorReturn, NonConvertibilityParamsOverrides, VectorCount,
    VectorCountConstructorParamsOverrides, build_non_convertibility_deps,
    build_non_convertibility_params, build_non_convertibility_payload, build_vector_count,
    non_convertibility,
};

/// Contract: own entry
///   `every_record_holds_a_true_verdict_for_its_own_identity_and_a_false_one_for_another`,
///   as an outside caller observes it — `non_convertibility` under the
///   entitlement scope over a case with a count above one and two distinct
///   identities, every adapter real and the operating-system source real →
///   every record's verdict against its own identity's element is `true`
///   and against the other's is `false`; with the other identity equal to
///   the configured one, both are `true`.
/// Arrange: `count` of `2` from `build_vector_count` with
///   `VectorCountConstructorParamsOverrides`; every case — every member of
///   `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every entitlement-scope
///   selection of `DECLARED_KEM_SELECTIONS` — the configured identity
///   `b"pkg@1.0.0#1"` with the other identity `b"pkg@2.0.0#7"`, and again
///   with the other identity `b"pkg@1.0.0#1"`.
/// Act:     `non_convertibility`, once per pair of identities, in the KEM
///   consumer's method.
/// Assert:  as the private integration test element's block of the same
///   name asserts — under the distinct identities `vectors` holds `2`
///   records, each with `is_valid` `true` and `other_is_valid` `false`;
///   under the equal identities each record's `is_valid` and
///   `other_is_valid` are `true`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `non_convertibility`, with the route
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected concrete's `try_new`, `non_convertibility`,
///   and every KEM method it calls.
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
                        [(b"pkg@2.0.0#7".as_slice(), false), (identity, true)]
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
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it — the list holds the configured count,
///   each record's `draws` hold its four uniforms each `UNIFORM_BYTES_LENGTH`
///   long, and no two records share `draws`.
/// Arrange: `count` of `3` from `build_vector_count`; every member of
///   `PAIRING_CONCRETES` with each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` and every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, the configured identity
///   `b"pkg@1.0.0#1"` and the other identity `b"pkg@2.0.0#7"`.
/// Act:     `non_convertibility`, once per case, in the KEM consumer's
///   method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   block above.
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

/// Contract: own entry `the_set_is_empty_under_the_asset_scope`, as an
///   outside caller observes it — `non_convertibility` under the asset
///   scope makes no draw, so the refusing source refuses nothing, and
///   returns `Ok` with an empty `vectors`.
/// Arrange: `count` of `3` from `build_vector_count`; each asset-scope
///   selection of `DECLARED_KEM_SELECTIONS` over every member of
///   `PAIRING_CONCRETES`, once with the source for
///   `RandomSourceKind::OperatingSystem` and once with the source for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; and, as the counterfactual, each entitlement-scope
///   selection with the operating-system source, under which the same count
///   returns three records.
/// Act:     `non_convertibility`, once per selection and source, in the KEM
///   consumer's method.
/// Assert:  under each asset-scope selection and each source the result
///   `is_ok()` and its `vectors` is empty; under each entitlement-scope
///   selection `vectors` holds `3` records; each factory's result is
///   unpacked by `expect` naming the declared selection and the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration in the refusing run;
///   the operating-system generator runs real otherwise.
#[test]
fn the_set_is_empty_under_the_asset_scope() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
        expected_len: usize,
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
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), self.expected_len);
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
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let sources: &[(&dyn IRandomSourceAdapter, usize)] = match kem_selection.scope {
                        IdentityScope::Asset => &[
                            (&*subject_source.adapter, 0),
                            (&*refusing_source.adapter, 0),
                        ],
                        IdentityScope::Entitlement => &[(&*subject_source.adapter, 3)],
                    };
                    for (source, expected_len) in sources {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    subject: *source,
                                    count: self.count,
                                    identity: b"pkg@1.0.0#1",
                                    other_identity: b"pkg@2.0.0#7",
                                    expected_len: *expected_len,
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
    // the_set_is_empty_under_the_asset_scope
}

/// Contract: carried `kem/factory` entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`,
///   restated by the spec — `non_convertibility` hands the payload's
///   identity bytes to `derive_identity` unchanged and issues each record's
///   credential for the element `derive_identity` returns for them, so the
///   record's credential is valid under its replayed parameter set for the
///   element of those bytes, and that element's scalar is the scalar the
///   Keccak-256 identity mapping yields for the bytes the encoding factory
///   returns for the derivation context, under the tag the KEM declares.
/// Arrange: each entitlement-scope selection of `DECLARED_KEM_SELECTIONS`
///   over every member of `PAIRING_CONCRETES`, each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`, and each case of
///   `DECLARED_ENCODING_SELECTIONS` that declares
///   `EncodingIdentifier::EthereumAbiV1`; the configured identity bytes are
///   the bytes `create_encoding` returns for the derivation context
///   `build_derivation_context` returns, under
///   `DerivationContextDescription`, through
///   `build_mock_i_encoding_consumer`; the other identity `b"pkg@2.0.0#7"`;
///   `count` of `1`.
/// Act:     `non_convertibility`, once, in the KEM consumer's method, over
///   `build_non_convertibility_payload` of those bytes.
/// Assert:  replaying the record's `draws` through the case's KEM over those
///   bytes, `is_valid` with the replayed parameter set, the replayed element
///   of those bytes, and the record's `credential` returns `true`, and
///   `encode_scalar` of that element's `scalar`, read through
///   `identity_element_components`, equals `encode_scalar` of the scalar
///   `sample_from_uniform_bytes` returns for `expected_uniform_bytes` over
///   the tag `build_domain_tag` builds from `K::DECLARATION.identity_tag`,
///   those bytes, and `<P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH`; each factory's result is unpacked by `expect`
///   naming the declared selection and the outermost `Result` `is_ok()`,
///   proving the consumers ran.
/// Boundary: the public call `non_convertibility`, with the route the spec
///   states for the entry.
/// Mocked:  the encoding family's official consumer mock, which hands the
///   bytes the encoding returned to the block; the operating-system
///   generator runs real.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
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
            let Ok(verdict) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &record.credential,
                    Default::default(),
                ),
            );
            assert!(verdict.is_valid);
            let actual_identity_element = payload
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
            assert_eq!(
                encode_scalar(&actual_identity_element.scalar),
                encode_scalar(&expected)
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
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
                if h2s_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                    continue;
                }
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    if kem_selection.scope != IdentityScope::Entitlement {
                        continue;
                    }
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                pairing: &payload.adapter,
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: self.identity,
                                other_identity: self.other_identity,
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
    let context = build_derivation_context(Default::default());
    let other_identity: &[u8] = b"pkg@2.0.0#7";
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        if encoding_selection.identifier != EncodingIdentifier::EthereumAbiV1 {
            continue;
        }
        let description = build_derivation_context_description(Default::default());
        let identity = create_encoding(
            &build_create_encoding_deps(build_mock_i_encoding_consumer(
                &description,
                context.clone(),
                Vec::new(),
            )),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        )
        .expect("the declared selection constructs")
        .output
        .encoded
        .expect("the derivation context encodes")
        .bytes;
        for pairing_concrete in PAIRING_CONCRETES {
            // Act
            let created = create_pairing(
                &build_create_pairing_deps(
                    PairingConsumer {
                        count,
                        identity: identity.as_slice(),
                        other_identity,
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
    }
    // every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle
}

/// Contract: own entry
///   `every_record_draw_replays_to_the_credential_and_verdicts_it_holds`,
///   as an outside caller observes it — the record's `draws`, split at
///   `UNIFORM_BYTES_LENGTH` into the uniforms of its calls in draw order and
///   replayed through the case's KEM over the configured identities,
///   reproduce the record's credential; the record's credential is valid
///   under the replayed parameter set for the configured identity's
///   replayed element and not for the other's; its `b` is the second-group
///   generator multiplied by the issuance scalar; the configured identity's
///   element is `u0` added to `u1` multiplied by its scalar; and that
///   scalar is the identity mapping under the declared tag.
/// Arrange: `count` of `1` from `build_vector_count`; every case, each run
///   under the identity pairs `b"pkg@1.0.0#1"` with `b"pkg@2.0.0#7"` and
///   `b"pkg@3.0.0#2"` with `b"pkg@4.0.0#9"`, so a replay over an identity
///   other than the configured one reports otherwise.
/// Act:     `non_convertibility`, once per case and identity pair, in the
///   KEM consumer's method.
/// Assert:  as the private integration test element's block of the same
///   name asserts, through the crate's public surface and the adapter
///   crates' public methods — the encoded `a` and `b` of the replayed
///   credential equal the record's `credential`'s; `is_valid` over the
///   replayed parameter set returns `true` for the record's `credential`
///   with the configured identity's replayed element and `false` with the
///   other's; the credential's `b` encodes to `mul_g2` of `g2_generator` by
///   the fourth uniform's scalar; the configured identity's replayed
///   element's `element` encodes to `add_g1` of `u0` and `mul_g1` of `u1` by
///   its `scalar`; and for each `Keccak256V1` selection the element's
///   `scalar` encodes to the oracle's; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
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
                        (b"pkg@1.0.0#1".as_slice(), b"pkg@2.0.0#7".as_slice()),
                        (b"pkg@3.0.0#2".as_slice(), b"pkg@4.0.0#9".as_slice()),
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

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it — a hash-to-scalar refusal reaches
///   the caller inside the function's `DeriveIdentity` variant, whole.
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
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
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

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole`, as an
///   outside caller observes it — a `fill_bytes` refusal reaches the caller
///   inside the function's `FillBytes` variant at the master uniform's
///   draw, before any KEM call, whole.
/// Arrange: `count` of `1`; each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES` and
///   every selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the subject's
///   source `create_random_source` returns for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, and, as the counterfactual, the source for
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `non_convertibility`, once per source, in the KEM consumer's
///   method.
/// Assert:  the result under the refusing source equals
///   `Err(NonConvertibilityErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` whole, and the result under the
///   operating-system source `is_ok()`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `non_convertibility`, with the route of the
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
            let refused = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.refusing, Default::default()),
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
                refused.err(),
                Some(NonConvertibilityErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))
            );
            // Act
            let admitted = non_convertibility::<K>(
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
                    if kem_selection.scope != IdentityScope::Entitlement {
                        continue;
                    }
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                refusing: &*refusing_source.adapter,
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
///   master uniform reaches the caller inside the function's `Setup`
///   variant as `MasterScalarSampling`, whole.
/// Arrange: `count` of `1`; `create_pairing` over
///   `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`, with each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, and, as the counterfactual, each
///   member of `PAIRING_CONCRETES`; the expected `error` the `Err` that
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes`
///   returns, in the pairing consumer's method, for `UNIFORM_BYTES_LENGTH`
///   bytes `fill_bytes` draws from a further operating-system source.
/// Act:     `non_convertibility`, once per pairing selection, in the KEM
///   consumer's method.
/// Assert:  the result under the refusing selection equals
///   `Err(NonConvertibilityErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(error)))` whole, and the result under each member
///   of `PAIRING_CONCRETES` `is_ok()`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
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
                self.expected.as_ref().map(
                    |SampleUniformScalarErrorReturn::WrongLength { expected, actual }| {
                        NonConvertibilityErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(
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
            let probe_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            // Arrange
            let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: probe_source
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            build_fill_bytes_payload(FillBytesPayloadOverrides {
                                length: Some(
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                ),
                            }),
                        )
                        .expect("the probe source fills")
                        .bytes,
                },
            )
            .err();
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    if kem_selection.scope != IdentityScope::Entitlement {
                        continue;
                    }
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: b"pkg@1.0.0#1",
                                other_identity: b"pkg@2.0.0#7",
                                expected: &expected,
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
    // Act
    let refused = create_pairing(
        &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
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
    // a_setup_sampling_refusal_reaches_the_caller_whole
}
