#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::build_secret;
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, MockIEncodingContract,
    build_canonical_fields, build_create_encoding_deps, build_create_encoding_payload,
    build_mock_i_encoding_consumer, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params, build_domain_tag, expected_uniform_bytes,
};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides, KemIdentity, SetupErrorReturn,
    SetupParamsOverrides, SetupPayloadOverrides, build_credential_components_params,
    build_credential_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_issue_params, build_issue_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    AddG1Params, AddG1Payload, CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload,
    EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, FillBytesParams,
    FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, build_fill_bytes_payload,
};

use harness_crypto::{
    AdmittedKemIdentity, INonConvertibilityCaseConsumer, NonConvertibilityCaseParamsOverrides,
    NonConvertibilityCasePayloadOverrides, NonConvertibilityErrorReturn,
    NonConvertibilityParamsOverrides, NonConvertibilityPayloadOverrides, VectorCount,
    VectorCountConstructorParamsOverrides, build_non_convertibility_case_deps,
    build_non_convertibility_case_params, build_non_convertibility_case_payload,
    build_non_convertibility_deps, build_non_convertibility_params,
    build_non_convertibility_payload, build_vector_count, non_convertibility,
    run_non_convertibility_case,
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
///   `VectorCountConstructorParamsOverrides`; each entitlement-scope case,
///   the configured identity `b"pkg@1.0.0#1"` with the other identity
///   `b"pkg@2.0.0#7"`, and again with the other identity `b"pkg@1.0.0#1"`.
/// Act:     `non_convertibility`, once per pair of identities, in the case
///   consumer's method.
/// Assert:  under the distinct identities `vectors` holds `2` records, each
///   with `is_valid` `true` and `other_is_valid` `false`; under the equal
///   identities each record's `is_valid` and `other_is_valid` are `true`;
///   each case's `Result` `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `non_convertibility`, with the route
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected concrete's `try_new`, `non_convertibility`,
///   and every KEM method it calls.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_holds_a_true_verdict_for_its_own_identity_and_a_false_one_for_another() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
        expect_other: bool,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.is_valid);
                assert_eq!(record.other_is_valid, self.expect_other);
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| selection.scope == IdentityScope::Entitlement)
            {
                for (other_canonical, expect_other) in
                    [(b"pkg@2.0.0#7".as_slice(), false), (b"pkg@1.0.0#1", true)]
                {
                    // Act
                    let returned = run_non_convertibility_case(
                        &build_non_convertibility_case_deps(
                            CaseConsumer {
                                count,
                                expect_other,
                            },
                            Default::default(),
                        ),
                        build_non_convertibility_case_params(
                            NonConvertibilityCaseParamsOverrides {
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
                        build_non_convertibility_case_payload(
                            NonConvertibilityCasePayloadOverrides {
                                other_canonical: Some(other_canonical.to_vec()),
                                ..Default::default()
                            },
                        ),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
    }
    // every_record_holds_a_true_verdict_for_its_own_identity_and_a_false_one_for_another
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it — the list holds the configured
///   count, each record's `draws` hold its four uniforms each
///   `UNIFORM_BYTES_LENGTH` long, and no two records share `draws`.
/// Arrange: `count` of `3` from `build_vector_count`; every member of
///   `PAIRING_CONCRETES` with each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS`.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each case's `Result` `is_ok()`, proving
///   the runner's levels ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   block above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
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
            for (index, left) in success.vectors.iter().enumerate() {
                for right in success.vectors.iter().skip(index + 1) {
                    assert_ne!(left.draws, right.draws);
                }
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| selection.scope == IdentityScope::Entitlement)
            {
                // Act
                let returned = run_non_convertibility_case(
                    &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
                    build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
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
                    build_non_convertibility_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: own entry `the_set_is_empty_under_the_asset_scope` —
///   `non_convertibility` over admitted identities whose `setup_scope` is
///   `SetupScope::Asset` returns `Ok` with an empty `vectors`, and no draw
///   is made, so a refusing source is not reached.
/// Arrange: `count` of `3` from `build_vector_count`; each asset-scope
///   selection of `DECLARED_KEM_SELECTIONS` over every member of
///   `PAIRING_CONCRETES`, once with the source for
///   `RandomSourceKind::OperatingSystem` and once with the source for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; and, as the counterfactual, each
///   entitlement-scope selection with the operating-system source.
/// Act:     `non_convertibility`, once per selection and source, in the
///   case consumer's method.
/// Assert:  under each asset-scope selection and each source the result
///   `is_ok()` and its `vectors` is empty; under each entitlement-scope
///   selection `vectors` holds `3` records; each case's `Result` `is_ok()`,
///   proving the runner's levels ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration in the refusing run;
///   the operating-system generator runs real otherwise.
#[test]
fn the_set_is_empty_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
        expected_len: usize,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), self.expected_len);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| selection.scope == IdentityScope::Asset)
            {
                for kind in [
                    RandomSourceKind::OperatingSystem,
                    RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                ] {
                    // Act
                    let returned = run_non_convertibility_case(
                        &build_non_convertibility_case_deps(
                            CaseConsumer {
                                count,
                                expected_len: 0,
                            },
                            Default::default(),
                        ),
                        build_non_convertibility_case_params(
                            NonConvertibilityCaseParamsOverrides {
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
                        build_non_convertibility_case_payload(Default::default()),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
            for kem_selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| selection.scope == IdentityScope::Entitlement)
            {
                // Act
                let returned = run_non_convertibility_case(
                    &build_non_convertibility_case_deps(
                        CaseConsumer {
                            count,
                            expected_len: 3,
                        },
                        Default::default(),
                    ),
                    build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
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
                    build_non_convertibility_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // the_set_is_empty_under_the_asset_scope
}

/// Contract: carried `kem/factory` entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`,
///   restated by the spec — `non_convertibility` hands the payload's
///   identity bytes to `derive_identity` unchanged and issues each record's
///   credential for the element `derive_identity` returns for them, so the
///   record's credential is valid under its replayed parameter set for the
///   element of those bytes, and that element's scalar is the identity
///   mapping under the declared tag.
/// Arrange: each entitlement-scope selection of `DECLARED_KEM_SELECTIONS`
///   over every member of `PAIRING_CONCRETES`, each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`, and each case of
///   `DECLARED_ENCODING_SELECTIONS` that declares
///   `EncodingIdentifier::EthereumAbiV1`; the configured identity bytes are
///   the bytes `create_encoding` returns for a value of the family's mock
///   description, `MockIEncodingContract` over `build_canonical_fields`,
///   through `build_mock_i_encoding_consumer`, and the other identity
///   `b"pkg@2.0.0#7"`; `count` of `1`.
/// Act:     `non_convertibility`, once, in the case consumer's method, over
///   `build_non_convertibility_payload` overriding `identity` with the
///   admitted entitlement identity whose canonical bytes are those bytes
///   and `other_identity` with the admitted entitlement identity over
///   `b"pkg@2.0.0#7"`.
/// Assert:  replaying the record's `draws` through the case's KEM over
///   those bytes, `is_valid` with the replayed parameter set, the replayed
///   element of those bytes, and the record's `credential` returns `true`,
///   and `encode_scalar` of that element's `scalar`, read through
///   `identity_element_components`, equals `encode_scalar` of the scalar
///   `sample_from_uniform_bytes` returns for `expected_uniform_bytes` over
///   the tag `build_domain_tag` builds from `K::DECLARATION.identity_tag`,
///   those bytes, and
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`; each
///   case's `Result` `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `non_convertibility`, with the route the spec
///   states for the entry.
/// Mocked:  the encoding family's official consumer mock, which hands the
///   bytes the encoding returned to the block.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let description = MockIEncodingContract;
    let mut encodings: Vec<Vec<u8>> = Vec::new();
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        if encoding_selection.identifier == EncodingIdentifier::EthereumAbiV1 {
            let consumer = build_mock_i_encoding_consumer(
                &description,
                build_canonical_fields(Default::default()),
                Vec::new(),
            );
            let outcome = create_encoding(
                &build_create_encoding_deps(consumer),
                CreateEncodingParams {
                    concrete: encoding_selection.concrete,
                    identifier: encoding_selection.identifier,
                },
                build_create_encoding_payload(Default::default()),
            )
            .expect("the declared encoding selection constructs");
            encodings.push(
                outcome
                    .output
                    .encoded
                    .expect("the real encoder encodes the description's value")
                    .bytes,
            );
        }
    }

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            let mut uniforms = record
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                .map(|chunk| build_secret(chunk.to_vec(), Default::default()));
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
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
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(identity.identity),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let Ok(verdict) = kem.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &record.credential,
                    Default::default(),
                ),
            );
            assert!(verdict.is_valid);
            let replayed_identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                pairing
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
            if let KemIdentity::Entitlement { canonical } = identity.identity {
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                });
                let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(
                                &tag,
                                canonical,
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
                    encode_scalar(&replayed_identity_element.scalar),
                    encode_scalar(&expected)
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if h2s_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for kem_selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| selection.scope == IdentityScope::Entitlement)
            {
                for canonical in &encodings {
                    // Act
                    let returned = run_non_convertibility_case(
                        &build_non_convertibility_case_deps(
                            CaseConsumer { count },
                            Default::default(),
                        ),
                        build_non_convertibility_case_params(
                            NonConvertibilityCaseParamsOverrides {
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
                        build_non_convertibility_case_payload(
                            NonConvertibilityCasePayloadOverrides {
                                canonical: Some(canonical.clone()),
                                ..Default::default()
                            },
                        ),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
    }
    // every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle
}

/// Contract: own entry
///   `every_record_draw_replays_to_the_credential_and_verdicts_it_holds`,
///   as an outside caller observes it — the record's `draws`, split at
///   `UNIFORM_BYTES_LENGTH` into the uniforms of its calls in draw order
///   and replayed through the case's KEM under the two admitted identities,
///   reproduce the record's credential and both verdicts.
/// Arrange: `count` of `1` from `build_vector_count`; each
///   entitlement-scope case, each run under the identity pairs
///   `b"pkg@1.0.0#1"` with `b"pkg@2.0.0#7"` and `b"pkg@3.0.0#2"` with
///   `b"pkg@4.0.0#9"`.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the encoded `a` and `b` of the replayed credential each equal
///   the record's `credential`'s; `is_valid` with the replayed parameter
///   set, the configured identity's replayed element, and the record's
///   `credential` returns `true`, and with the other identity's replayed
///   element returns `false`; the record's `credential`'s `b` encodes to
///   `mul_g2` of `g2_generator` by the scalar of the fourth uniform; the
///   configured identity's replayed element's `element` encodes to `add_g1`
///   of the replayed `u0` and `mul_g1` of the replayed `u1` by that
///   element's `scalar`, whose `encode_scalar` equals the oracle for each
///   `HashToScalarIdentifier::Keccak256V1` case; each case's `Result`
///   `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_credential_and_verdicts_it_holds() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
        h2s_identifier: HashToScalarIdentifier,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            let mut uniforms = record
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                .map(|chunk| build_secret(chunk.to_vec(), Default::default()));
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
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
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(identity.identity),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let other_derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(other_identity.identity),
                        },
                    ),
                )
                .expect("the replay's other identity derivation succeeds");
            let issued = kem
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
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                pairing
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
                kem.credential_components(
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
            let Ok(own_verdict) = kem.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &record.credential,
                    Default::default(),
                ),
            );
            assert!(own_verdict.is_valid);
            let Ok(other_verdict) = kem.is_valid(
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
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(expected_b) = pairing.mul_g2(
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
            let replayed_parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            let replayed_identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let Ok(expected_identity_inner) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: replayed_parameter_set.u1.clone(),
                    scalar: replayed_identity_element.scalar.clone(),
                },
            );
            let Ok(expected_identity_element_point) = pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: replayed_parameter_set.u0.clone(),
                    right: expected_identity_inner.product,
                },
            );
            assert_eq!(
                encode_g1(&replayed_identity_element.element),
                encode_g1(&expected_identity_element_point.sum)
            );
            if self.h2s_identifier == HashToScalarIdentifier::Keccak256V1
                && let KemIdentity::Entitlement { canonical } = identity.identity
            {
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                });
                let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(
                                &tag,
                                canonical,
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
                    encode_scalar(&replayed_identity_element.scalar),
                    encode_scalar(&expected)
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| selection.scope == IdentityScope::Entitlement)
            {
                for (canonical, other_canonical) in [
                    (b"pkg@1.0.0#1".as_slice(), b"pkg@2.0.0#7".as_slice()),
                    (b"pkg@3.0.0#2".as_slice(), b"pkg@4.0.0#9".as_slice()),
                ] {
                    // Act
                    let returned = run_non_convertibility_case(
                        &build_non_convertibility_case_deps(
                            CaseConsumer {
                                count,
                                h2s_identifier: h2s_selection.identifier,
                            },
                            Default::default(),
                        ),
                        build_non_convertibility_case_params(
                            NonConvertibilityCaseParamsOverrides {
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
                        build_non_convertibility_case_payload(
                            NonConvertibilityCasePayloadOverrides {
                                canonical: Some(canonical.to_vec()),
                                other_canonical: Some(other_canonical.to_vec()),
                                ..Default::default()
                            },
                        ),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
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
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the result equals `Err(NonConvertibilityErrorReturn::
///   DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))`
///   whole; each case's `Result` `is_ok()`, proving the runner's levels
///   ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
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

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS
            .iter()
            .filter(|selection| selection.scope == IdentityScope::Entitlement)
        {
            // Act
            let returned = run_non_convertibility_case(
                &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
                build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(HashToScalarConcrete::Mock(
                                MockIHashToScalarAdapterFailureMode::SamplingRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                }),
                build_non_convertibility_case_payload(Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole` — a
///   `fill_bytes` refusal reaches the caller inside the function's
///   `FillBytes` variant, whole.
/// Arrange: each entitlement-scope selection of `DECLARED_KEM_SELECTIONS`
///   over every member of `PAIRING_CONCRETES`; the source
///   `create_random_source` returns for `build_create_random_source_params`
///   overriding `kind` with `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, and, as the
///   counterfactual, the source for `RandomSourceKind::OperatingSystem`.
/// Act:     `non_convertibility`, once per source, in the case consumer's
///   method.
/// Assert:  the result under the refusing source equals `Err(
///   NonConvertibilityErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` whole, and the result under the
///   operating-system source `is_ok()`; each case's `Result` `is_ok()`,
///   proving the runner's levels ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))
            );
        }
    }

    struct CounterfactualConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CounterfactualConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS
            .iter()
            .filter(|selection| selection.scope == IdentityScope::Entitlement)
        {
            for kind in [
                RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                RandomSourceKind::OperatingSystem,
            ] {
                // Act
                let returned = match kind {
                    RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    ) => run_non_convertibility_case(
                        &build_non_convertibility_case_deps(
                            CaseConsumer { count },
                            Default::default(),
                        ),
                        build_non_convertibility_case_params(
                            NonConvertibilityCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                kem: Some(*kem_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides { kind: Some(kind) },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_non_convertibility_case_payload(Default::default()),
                    ),
                    _ => run_non_convertibility_case(
                        &build_non_convertibility_case_deps(
                            CounterfactualConsumer { count },
                            Default::default(),
                        ),
                        build_non_convertibility_case_params(
                            NonConvertibilityCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                kem: Some(*kem_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides { kind: Some(kind) },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_non_convertibility_case_payload(Default::default()),
                    ),
                };
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // a_draw_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_setup_sampling_refusal_reaches_the_caller_whole`
///   — a scalar-sampling refusal in `setup` reaches the caller inside the
///   function's `Setup` variant, whole.
/// Arrange: `create_pairing` over `build_create_pairing_params` overriding
///   `concrete` with `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`, with each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS`, and, as the counterfactual, each member of
///   `PAIRING_CONCRETES`; the expected `error` the `Err` that
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes`
///   returns inside the runner's pairing level, for `UNIFORM_BYTES_LENGTH`
///   bytes `fill_bytes` draws from a further operating-system source.
/// Act:     `non_convertibility`, once per pairing selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection equals `Err(
///   NonConvertibilityErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(error)))` whole, and the result under each member
///   of `PAIRING_CONCRETES` `is_ok()`; each case's `Result` `is_ok()`,
///   proving the runner's levels ran.
/// Boundary: the public call `non_convertibility`, with the route of the
///   blocks above.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let uniform = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .expect("the independent source fills")
                .bytes;
            let error = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .err()
            .expect("the refusing scalar refuses the sampling");
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(error)
                ))
            );
        }
    }

    struct CounterfactualConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CounterfactualConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }

    for kem_selection in DECLARED_KEM_SELECTIONS
        .iter()
        .filter(|selection| selection.scope == IdentityScope::Entitlement)
    {
        // Act
        let returned = run_non_convertibility_case(
            &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
            build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                })),
                kem: Some(*kem_selection),
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    },
                )),
                ..Default::default()
            }),
            build_non_convertibility_case_payload(Default::default()),
        );
        // Assert
        assert!(returned.is_ok());
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS
            .iter()
            .filter(|selection| selection.scope == IdentityScope::Entitlement)
        {
            // Act
            let returned = run_non_convertibility_case(
                &build_non_convertibility_case_deps(
                    CounterfactualConsumer { count },
                    Default::default(),
                ),
                build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_non_convertibility_case_payload(Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_setup_sampling_refusal_reaches_the_caller_whole
}
