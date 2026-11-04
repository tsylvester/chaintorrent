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
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides, KemIdentity,
    RerandomizePayloadOverrides, SetupErrorReturn, SetupParamsOverrides, SetupPayloadOverrides,
    SetupScope, build_credential_components_params, build_credential_components_payload,
    build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_rerandomize_params, build_rerandomize_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, CreatePairingParamsOverrides,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MockIPairingAdapterFailureMode, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES, PairingConcrete,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, FillBytesParams,
    FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, build_fill_bytes_payload,
};

use harness_crypto::{
    AdmittedKemIdentity, IRerandomizationCaseConsumer, RerandomizationCaseParamsOverrides,
    RerandomizationCasePayloadOverrides, RerandomizationErrorReturn,
    RerandomizationParamsOverrides, RerandomizationPayloadOverrides, VectorCount,
    VectorCountConstructorParamsOverrides, build_rerandomization_case_deps,
    build_rerandomization_case_params, build_rerandomization_case_payload,
    build_rerandomization_deps, build_rerandomization_params, build_rerandomization_payload,
    build_vector_count, rerandomization, run_rerandomization_case,
};

/// Contract: own entry
///   `every_record_holds_true_verdicts_for_its_issued_and_rerandomized_credentials`,
///   as an outside caller observes it — `rerandomization` over a case with a
///   count above one, every adapter real and the operating-system source
///   real → every record's verdicts are `true`, its credentials valid under
///   its parameter set and identity element, and its rerandomized credential
///   distinct from its issued one.
/// Arrange: `count` of `2` from `build_vector_count` with
///   `VectorCountConstructorParamsOverrides`; every case — every member of
///   `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every selection of
///   `DECLARED_KEM_SELECTIONS` — the identity bytes `b"pkg@1.0.0#1"` under
///   the entitlement scope and `[0x44; 32]` under the asset scope.
/// Act:     `rerandomization`, once per case, in the case consumer's method.
/// Assert:  `vectors` holds `2` records, and for each `issued_is_valid` and
///   `rerandomized_is_valid` are `true`, `is_valid` over
///   `build_is_valid_params` and `build_is_valid_payload` with the record's
///   parameter set, identity element, and `issued`, and again with
///   `rerandomized`, returns `true`, and the `encode_g2` bytes of the `b`
///   `credential_components` reads from `rerandomized` differ from those of
///   `issued`; each case's `Result` `is_ok()`, proving the runner's levels
///   ran.
/// Boundary: the public call `rerandomization`, with the route
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected concrete's `try_new`, `rerandomization`, and
///   every KEM method it and the block call.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_holds_true_verdicts_for_its_issued_and_rerandomized_credentials() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
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
            let components_of = |credential: &K::Credential| {
                kem.credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(credential, Default::default()),
                )
                .expect("a credential's components are infallible")
                .components
            };
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.issued_is_valid);
                assert!(record.rerandomized_is_valid);
                let Ok(issued_valid) = kem.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.issued,
                        Default::default(),
                    ),
                );
                assert!(issued_valid.is_valid);
                let Ok(rerandomized_valid) = kem.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.rerandomized,
                        Default::default(),
                    ),
                );
                assert!(rerandomized_valid.is_valid);
                assert_ne!(
                    encode_g2(&components_of(&record.issued).b),
                    encode_g2(&components_of(&record.rerandomized).b)
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                // Act
                let returned = run_rerandomization_case(
                    &build_rerandomization_case_deps(CaseConsumer { count }, Default::default()),
                    build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
                    build_rerandomization_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // every_record_holds_true_verdicts_for_its_issued_and_rerandomized_credentials
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it — the list holds the configured count,
///   each record's `draws` hold its five uniforms each
///   `UNIFORM_BYTES_LENGTH` long, and no two records share `draws`.
/// Arrange: `count` of `3` from `build_vector_count`; every member of
///   `PAIRING_CONCRETES` crossed with the declared hash-to-scalar selections
///   and each selection and scope of `DECLARED_KEM_SELECTIONS`.
/// Act:     `rerandomization`, once per case, in the case consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `5` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each case's `Result` `is_ok()`, proving
///   the runner's levels ran.
/// Boundary: the public call `rerandomization`, with the route of the block
///   above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
            for record in &success.vectors {
                assert_eq!(
                    record.draws.len(),
                    5 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
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
            for kem_selection in DECLARED_KEM_SELECTIONS {
                // Act
                let returned = run_rerandomization_case(
                    &build_rerandomization_case_deps(CaseConsumer { count }, Default::default()),
                    build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
                    build_rerandomization_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: carried `kem/factory` entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`,
///   restated by the spec — the identity element a declaring KEM derives
///   for the entitlement identity whose canonical bytes are the bytes the
///   encoding factory's concrete writes for the described value.
/// Arrange: each entitlement-scope selection of `DECLARED_KEM_SELECTIONS`
///   over every member of `PAIRING_CONCRETES`, each selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, and each selection of
///   `DECLARED_ENCODING_SELECTIONS` declaring
///   `EncodingIdentifier::EthereumAbiV1`; the identity bytes are the bytes
///   `create_encoding` returns for a value of the family's mock description,
///   `MockIEncodingContract` over `build_canonical_fields`, through
///   `build_mock_i_encoding_consumer`; `count` of `1`.
/// Act:     `rerandomization`, once per case and encoding, in the case
///   consumer's method, over `build_rerandomization_payload` overriding
///   `identity` with the admitted entitlement identity the KEM level
///   composed over the canonical bytes `create_encoding` returned.
/// Assert:  `encode_scalar` of the record's identity element's `scalar`,
///   read through `identity_element_components`, equals `encode_scalar` of
///   the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds from
///   `K::DECLARATION.identity_tag`, those bytes, and
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`; each
///   case's `Result` `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `rerandomization`, with the route the spec
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

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
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
            let actual_identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &record.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the record's identity element's components are infallible")
                .components;
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
                    encode_scalar(&actual_identity_element.scalar),
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
            for kem_selection in DECLARED_KEM_SELECTIONS {
                if kem_selection.scope != IdentityScope::Entitlement {
                    continue;
                }
                for canonical in &encodings {
                    // Act
                    let returned = run_rerandomization_case(
                        &build_rerandomization_case_deps(
                            CaseConsumer { count },
                            Default::default(),
                        ),
                        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
                        }),
                        build_rerandomization_case_payload(RerandomizationCasePayloadOverrides {
                            canonical: Some(canonical.clone()),
                            ..Default::default()
                        }),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
    }
    // every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle
}

/// Contract: own entry `every_record_draw_replays_to_the_values_it_holds`,
///   as an outside caller observes it — the record's `draws`, split at
///   `UNIFORM_BYTES_LENGTH` into the uniforms of its calls in draw order and
///   replayed through the case's KEM under the admitted identity's
///   `setup_scope` and `identity`, reproduce the record's values.
/// Arrange: `count` of `1` from `build_vector_count`; every case, each run
///   under the identity bytes `b"pkg@1.0.0#1"` and `b"pkg@2.0.0#7"` under
///   the entitlement scope and `[0x44; 32]` and `[0x45; 32]` under the
///   asset scope.
/// Act:     `rerandomization`, once per case and identity, in the case
///   consumer's method.
/// Assert:  as the private integration block of the same name asserts —
///   the encoded `hpub`, identity element `scalar` and `element`, and
///   credentials' `a` and `b` of the replay each equal the record's; the
///   `encode_g2` bytes of `issued`'s `b` equal those of `mul_g2` of
///   `g2_generator` by the fourth uniform's scalar, and those of
///   `rerandomized`'s `b` equal those of `add_g2` of `issued`'s `b` and
///   `mul_g2` of `g2_generator` by the fifth uniform's scalar; the identity
///   element's `element` encodes to `add_g1` of the parameter set's `u0`
///   and `mul_g1` of its `u1` by the element's `scalar`; under the
///   entitlement scope for each Keccak256V1 case, `encode_scalar` of the
///   element's `scalar` equals `encode_scalar` of the scalar
///   `sample_from_uniform_bytes` returns for `expected_uniform_bytes` over
///   the declared tag, the identity bytes, and `UNIFORM_BYTES_LENGTH`; each
///   case's `Result` `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `rerandomization`, with the route of the
///   blocks above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_values_it_holds() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
        h2s_identifier: HashToScalarIdentifier,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
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
                                .expect("the record's draws hold five uniforms"),
                        ),
                        u0_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold five uniforms"),
                        ),
                        u1_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold five uniforms"),
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
                                    .expect("the record's draws hold five uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's issuance succeeds");
            let rerandomized = kem
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        RerandomizePayloadOverrides {
                            uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold five uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
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
                .expect("the parameter set's components are infallible")
                .components;
            assert_eq!(
                encode_g2(&actual_parameter_set.hpub),
                encode_g2(&expected_parameter_set.hpub)
            );
            let expected_identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let actual_identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &record.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
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
                kem.credential_components(
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
            let expected_rerandomized = components_of(&rerandomized.credential);
            let actual_rerandomized = components_of(&record.rerandomized);
            assert_eq!(
                encode_g1(&actual_rerandomized.a),
                encode_g1(&expected_rerandomized.a)
            );
            assert_eq!(
                encode_g2(&actual_rerandomized.b),
                encode_g2(&expected_rerandomized.b)
            );
            let issuance_scalar = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(
                        record
                            .draws
                            .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                            .nth(3)
                            .expect("the record's draws hold five uniforms")
                            .to_vec(),
                        Default::default(),
                    ),
                },
            )
            .expect("the fourth uniform samples")
            .scalar
            .expose()
            .clone();
            let rerandomization_scalar =
                <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            record
                                .draws
                                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                                .nth(4)
                                .expect("the record's draws hold five uniforms")
                                .to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the fifth uniform samples")
                .scalar
                .expose()
                .clone();
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(expected_issued_b) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: issuance_scalar,
                },
            );
            assert_eq!(
                encode_g2(&actual_issued.b),
                encode_g2(&expected_issued_b.product)
            );
            let Ok(offset_term) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: rerandomization_scalar,
                },
            );
            let Ok(expected_rerandomized_b) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: actual_issued.b.clone(),
                    right: offset_term.product,
                },
            );
            assert_eq!(
                encode_g2(&actual_rerandomized.b),
                encode_g2(&expected_rerandomized_b.sum)
            );
            let Ok(expected_identity_inner) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: actual_parameter_set.u1.clone(),
                    scalar: actual_identity_element.scalar.clone(),
                },
            );
            let Ok(expected_identity_element_point) = pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: actual_parameter_set.u0.clone(),
                    right: expected_identity_inner.product,
                },
            );
            assert_eq!(
                encode_g1(&actual_identity_element.element),
                encode_g1(&expected_identity_element_point.sum)
            );
            if self.h2s_identifier == HashToScalarIdentifier::Keccak256V1 {
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
                        encode_scalar(&actual_identity_element.scalar),
                        encode_scalar(&expected)
                    );
                }
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                // Arrange
                let payloads = match kem_selection.scope {
                    IdentityScope::Entitlement => vec![
                        build_rerandomization_case_payload(RerandomizationCasePayloadOverrides {
                            canonical: Some(b"pkg@1.0.0#1".to_vec()),
                            ..Default::default()
                        }),
                        build_rerandomization_case_payload(RerandomizationCasePayloadOverrides {
                            canonical: Some(b"pkg@2.0.0#7".to_vec()),
                            ..Default::default()
                        }),
                    ],
                    IdentityScope::Asset => vec![
                        build_rerandomization_case_payload(RerandomizationCasePayloadOverrides {
                            root: Some(build_root(RootConstructorParamsOverrides {
                                bytes: Some([0x44; 32]),
                            })),
                            ..Default::default()
                        }),
                        build_rerandomization_case_payload(RerandomizationCasePayloadOverrides {
                            root: Some(build_root(RootConstructorParamsOverrides {
                                bytes: Some([0x45; 32]),
                            })),
                            ..Default::default()
                        }),
                    ],
                };
                for case_payload in payloads {
                    // Act
                    let returned = run_rerandomization_case(
                        &build_rerandomization_case_deps(
                            CaseConsumer {
                                count,
                                h2s_identifier: h2s_selection.identifier,
                            },
                            Default::default(),
                        ),
                        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
                        }),
                        case_payload,
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
    }
    // every_record_draw_replays_to_the_values_it_holds
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it — a hash-to-scalar refusal reaches the
///   caller inside the function's `DeriveIdentity` variant under the
///   entitlement scope and its `Setup` variant under the asset scope,
///   whole.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   with `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `rerandomization`, once per case, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(error)))` under the
///   entitlement scope and `Some(RerandomizationErrorReturn::Setup(
///   SetupErrorReturn::HashToScalar(error)))` under the asset scope, whole;
///   each case's `Result` `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `rerandomization`, with the route of the
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

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(match identity.setup_scope {
                    SetupScope::Entitlement => RerandomizationErrorReturn::DeriveIdentity(
                        DeriveIdentityErrorReturn::HashToScalar(expected),
                    ),
                    SetupScope::Asset { .. } => {
                        RerandomizationErrorReturn::Setup(SetupErrorReturn::HashToScalar(expected))
                    }
                })
            );
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_rerandomization_case(
                &build_rerandomization_case_deps(CaseConsumer { count }, Default::default()),
                build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
                build_rerandomization_case_payload(Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole` — a
///   refusal of the subject's source reaches the caller inside the
///   function's `FillBytes` variant, whole.
/// Arrange: `count` of `1`; each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`;
///   the source `create_random_source` returns for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, and, as the counterfactual, the source for
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `rerandomization`, once per source, in the case consumer's
///   method.
/// Assert:  the result under the refusing source's `Err` equals
///   `Some(RerandomizationErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` whole, and the result under the
///   operating-system source `is_ok()`; each case's `Result` `is_ok()`,
///   proving the runner's levels ran.
/// Boundary: the public call `rerandomization`, with the route of the
///   blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
        refuses: bool,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            if self.refuses {
                assert_eq!(
                    returned.err(),
                    Some(RerandomizationErrorReturn::FillBytes(
                        FillBytesErrorReturn::MockIRandomSourceAdapter
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                if kem_selection.scope != IdentityScope::Entitlement {
                    continue;
                }
                for (kind, refuses) in [
                    (
                        RandomSourceKind::Mock(
                            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                        ),
                        true,
                    ),
                    (RandomSourceKind::OperatingSystem, false),
                ] {
                    // Act
                    let returned = run_rerandomization_case(
                        &build_rerandomization_case_deps(
                            CaseConsumer { count, refuses },
                            Default::default(),
                        ),
                        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
                        }),
                        build_rerandomization_case_payload(Default::default()),
                    );
                    // Assert
                    assert!(returned.is_ok());
                }
            }
        }
    }
    // a_draw_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_setup_sampling_refusal_reaches_the_caller_whole`
///   — a refusal of the pairing scalar's sampling reaches the caller inside
///   the function's `Setup` variant's `MasterScalarSampling`, whole.
/// Arrange: `create_pairing` over `build_create_pairing_params` overriding
///   `concrete` with `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`, with each selection and scope of
///   `DECLARED_KEM_SELECTIONS`, and, as the counterfactual, each member of
///   `PAIRING_CONCRETES`; the expected `error` the `Err` that
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes`
///   returns inside the runner's pairing level, for `UNIFORM_BYTES_LENGTH`
///   bytes `fill_bytes` draws from a further operating-system source;
///   `count` of `1`.
/// Act:     `rerandomization`, once per pairing selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection's `Err` equals
///   `Some(RerandomizationErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(error)))` whole, and the result under each member
///   of `PAIRING_CONCRETES` `is_ok()`; each case's `Result` `is_ok()`,
///   proving the runner's levels ran.
/// Boundary: the public call `rerandomization`, with the route of the
///   blocks above.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
        refuses: bool,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            if self.refuses {
                let uniform = independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the further operating-system source fills")
                    .bytes;
                let error = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload { uniform },
                )
                .err()
                .expect("the refusing scalar refuses the sampling");
                assert_eq!(
                    returned.err(),
                    Some(RerandomizationErrorReturn::Setup(
                        SetupErrorReturn::MasterScalarSampling(error)
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }

    for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act — the refusing selection
            let returned = run_rerandomization_case(
                &build_rerandomization_case_deps(
                    CaseConsumer {
                        count,
                        refuses: true,
                    },
                    Default::default(),
                ),
                build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(PairingConcrete::Mock(
                            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                        )),
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
                build_rerandomization_case_payload(Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
            // Act — the counterfactual members
            for pairing_concrete in PAIRING_CONCRETES {
                let returned = run_rerandomization_case(
                    &build_rerandomization_case_deps(
                        CaseConsumer {
                            count,
                            refuses: false,
                        },
                        Default::default(),
                    ),
                    build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
                    build_rerandomization_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // a_setup_sampling_refusal_reaches_the_caller_whole
}
