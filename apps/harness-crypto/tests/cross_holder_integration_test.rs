#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::build_secret;
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, MockIEncodingContract,
    build_canonical_fields, build_create_encoding_deps, build_create_encoding_payload,
    build_mock_i_encoding_consumer, create_encoding,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides,
    HashToScalarIdentifier, IHashToScalarAdapter, build_domain_tag, expected_uniform_bytes,
};
use kem::{
    DECLARED_KEM_SELECTIONS, ICredentialKemAdapter, IdentityScope, KemIdentity,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_is_well_formed_params,
    build_is_well_formed_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, CrossHolderCaseParamsOverrides, CrossHolderCasePayloadOverrides,
    CrossHolderParamsOverrides, CrossHolderPayloadOverrides, ICrossHolderCaseConsumer, VectorCount,
    VectorCountConstructorParamsOverrides, build_cross_holder_case_deps,
    build_cross_holder_case_params, build_cross_holder_case_payload, build_cross_holder_deps,
    build_cross_holder_params, build_cross_holder_payload, build_vector_count, cross_holder,
    run_cross_holder_case,
};

/// Contract: own entry
///   `every_record_recovers_the_encapsulated_value_from_its_issued_and_successor_credentials`,
///   as an outside caller observes it — `cross_holder` over a case with a
///   count above one, every adapter real and the operating-system source
///   real → every record's credentials are valid, its capsule well formed,
///   and both decapsulated values equal the encapsulated value.
/// Arrange: `count` of `2` from `build_vector_count` with
///   `VectorCountConstructorParamsOverrides`; every case — every member of
///   `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every selection of
///   `DECLARED_KEM_SELECTIONS` — the identity bytes `b"pkg@1.0.0#1"` under
///   the entitlement scope and `[0x44; 32]` under the asset scope.
/// Act:     `cross_holder`, once per case, in the case consumer's method.
/// Assert:  `vectors` holds `2` records, and for each `is_valid` over
///   `build_is_valid_params` and `build_is_valid_payload` with the record's
///   parameter set, identity element, and `issued`, and again with
///   `successor`, returns `true`, `is_well_formed` over
///   `build_is_well_formed_params` and `build_is_well_formed_payload` with
///   the record's parameter set and capsule returns `true`, and the
///   `key_material().expose()` bytes of `issued_decapsulated` and of
///   `successor_decapsulated` each equal those of `encapsulated`; each
///   case's `Result` `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `cross_holder`, with the route
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected concrete's `try_new`, `cross_holder`, and
///   every KEM method it and the block call.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_recovers_the_encapsulated_value_from_its_issued_and_successor_credentials() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
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
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
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
                let Ok(successor_valid) = kem.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.successor,
                        Default::default(),
                    ),
                );
                assert!(successor_valid.is_valid);
                let Ok(well_formed) = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &record.parameter_set,
                        &record.capsule,
                        Default::default(),
                    ),
                );
                assert!(well_formed.is_well_formed);
                assert_eq!(
                    record.issued_decapsulated.key_material().expose(),
                    record.encapsulated.key_material().expose()
                );
                assert_eq!(
                    record.successor_decapsulated.key_material().expose(),
                    record.encapsulated.key_material().expose()
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                // Act
                let returned = run_cross_holder_case(
                    &build_cross_holder_case_deps(CaseConsumer { count }, Default::default()),
                    build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
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
                    build_cross_holder_case_payload(Default::default()),
                );
                // Assert
                assert!(returned.is_ok());
            }
        }
    }
    // every_record_recovers_the_encapsulated_value_from_its_issued_and_successor_credentials
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it — the list holds the configured count,
///   each record's `draws` hold its six uniforms each `UNIFORM_BYTES_LENGTH`
///   long, and no two records share `draws`.
/// Arrange: `count` of `3` from `build_vector_count`; every member of
///   `PAIRING_CONCRETES` with each selection and scope of
///   `DECLARED_KEM_SELECTIONS` and every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, the admitted identity of each
///   selection's scope.
/// Act:     `cross_holder`, once per case, in the case consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `6` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each case's `Result` `is_ok()`, proving
///   the runner's levels ran.
/// Boundary: the public call `cross_holder`, with the route of the block
///   above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
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
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
            for record in &success.vectors {
                assert_eq!(
                    record.draws.len(),
                    6 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
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
                let returned = run_cross_holder_case(
                    &build_cross_holder_case_deps(CaseConsumer { count }, Default::default()),
                    build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
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
                    build_cross_holder_case_payload(Default::default()),
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
///   restated by the spec — `cross_holder` hands the payload's admitted
///   identity to `derive_identity` unchanged, so each record's identity
///   element is the one `derive_identity` returns for it, and for the
///   entitlement identity whose canonical bytes are the bytes
///   `create_encoding` returns for a value of the family's mock description
///   its `scalar` encodes to the scalar sampled under the declared tag.
/// Arrange: `count` of `1`; each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   each selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, and each selection of
///   `DECLARED_ENCODING_SELECTIONS` declaring `EncodingIdentifier::
///   EthereumAbiV1`; the canonical identity bytes are the bytes
///   `create_encoding` returns for `MockIEncodingContract` over
///   `build_canonical_fields`, through `build_mock_i_encoding_consumer`.
/// Act:     `cross_holder`, once per case, in the case consumer's method.
/// Assert:  `encode_scalar` of the record's identity element's `scalar`,
///   read through `identity_element_components`, equals `encode_scalar` of
///   the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds from
///   `K::DECLARATION.identity_tag`, those bytes, and
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`; each
///   case's `Result` `is_ok()`, proving the runner's levels ran.
/// Boundary: the public call `cross_holder`, with the route the spec states
///   for the entry.
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

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
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
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
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
                    let returned = run_cross_holder_case(
                        &build_cross_holder_case_deps(CaseConsumer { count }, Default::default()),
                        build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
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
                        build_cross_holder_case_payload(CrossHolderCasePayloadOverrides {
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
