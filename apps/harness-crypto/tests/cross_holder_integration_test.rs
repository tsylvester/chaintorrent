#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashTryNewErrorReturn, build_derivation_context,
    build_secret,
};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_mock_i_encoding_consumer, create_encoding,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides,
    HashToScalarIdentifier, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_payload,
    build_domain_tag, create_hash_to_scalar, expected_uniform_bytes,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IKemConsumer, IdentityScope, build_create_kem_deps, build_create_kem_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_is_well_formed_params,
    build_is_well_formed_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, EncodeScalarParams,
    EncodeScalarPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};

use harness_crypto::{
    CrossHolderErrorReturn, CrossHolderParamsOverrides, VectorCount,
    VectorCountConstructorParamsOverrides, build_cross_holder_deps, build_cross_holder_params,
    build_cross_holder_payload, build_vector_count, cross_holder,
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
///   `DECLARED_KEM_SELECTIONS` — the identity `b"pkg@1.0.0#1"` under the
///   entitlement scope and `vec![0x44; 32]` under the asset scope.
/// Act:     `cross_holder`, once per case, in the KEM consumer's method.
/// Assert:  `vectors` holds `2` records, and for each `is_valid` over
///   `build_is_valid_params` and `build_is_valid_payload` with the record's
///   parameter set, identity element, and `issued`, and again with
///   `successor`, returns `true`, `is_well_formed` over
///   `build_is_well_formed_params` and `build_is_well_formed_payload` with
///   the record's parameter set and capsule returns `true`, and the
///   `key_material().expose()` bytes of `issued_decapsulated` and of
///   `successor_decapsulated` each equal those of `encapsulated`; each
///   factory's result is unpacked by `expect` naming the declared selection
///   and the outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `cross_holder`, with the route
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected concrete's `try_new`, `cross_holder`, and
///   every KEM method it and the block call.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_recovers_the_encapsulated_value_from_its_issued_and_successor_credentials() {
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
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                let Ok(issued_valid) = payload.adapter.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.issued,
                        Default::default(),
                    ),
                );
                assert!(issued_valid.is_valid);
                let Ok(successor_valid) = payload.adapter.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.successor,
                        Default::default(),
                    ),
                );
                assert!(successor_valid.is_valid);
                let Ok(well_formed) = payload.adapter.is_well_formed(
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
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, the identity of each selection's
///   scope.
/// Act:     `cross_holder`, once per case, in the KEM consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `6` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `cross_holder`, with the route of the block
///   above.
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
///   `an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope`,
///   as an outside caller observes it — an identity that is not a 32-byte
///   array is refused in the function's `AssetIdentityLength` variant with
///   the identity's length, and an all-zero 32-byte identity is refused in
///   its `AssetIdentityHash` variant carrying the domain constructor's
///   `AllZero` unchanged, while a well-formed identity is admitted.
/// Arrange: each asset-scope selection of `DECLARED_KEM_SELECTIONS` over
///   every member of `PAIRING_CONCRETES` and every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the identities `vec![0x44; 31]`,
///   `vec![0x00; 32]`, and `vec![0x44; 32]` in turn, each with the `Err` it
///   must equal or `None` for the admitted identity.
/// Act:     `cross_holder`, once per identity, in the KEM consumer's
///   method.
/// Assert:  the results equal `Err(CrossHolderErrorReturn::
///   AssetIdentityLength { actual: 31 })` and
///   `Err(CrossHolderErrorReturn::AssetIdentityHash
///   (AssetIdentityHashTryNewErrorReturn::AllZero))` whole, and the third
///   `is_ok()`; each factory's result is unpacked by `expect` naming the
///   declared selection and the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Boundary: the public call `cross_holder`, with the route of the blocks
///   above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        expected: &'a Option<CrossHolderErrorReturn>,
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
            // Assert
            assert_eq!(returned.err(), *self.expected);
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
            let cases: [(Vec<u8>, Option<CrossHolderErrorReturn>); 3] = [
                (
                    vec![0x44; ASSET_IDENTITY_HASH_LENGTH - 1],
                    Some(CrossHolderErrorReturn::AssetIdentityLength {
                        actual: ASSET_IDENTITY_HASH_LENGTH - 1,
                    }),
                ),
                (
                    vec![0x00; ASSET_IDENTITY_HASH_LENGTH],
                    Some(CrossHolderErrorReturn::AssetIdentityHash(
                        AssetIdentityHashTryNewErrorReturn::AllZero,
                    )),
                ),
                (vec![0x44; ASSET_IDENTITY_HASH_LENGTH], None),
            ];
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    if kem_selection.scope != IdentityScope::Asset {
                        continue;
                    }
                    for (identity, expected) in &cases {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    subject: &*subject_source.adapter,
                                    count: self.count,
                                    identity: identity.as_slice(),
                                    expected,
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
    // an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope
}

/// Contract: carried `kem/factory` entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`,
///   restated by the spec — the identity element's scalar is the scalar the
///   Keccak-256 identity mapping yields for the identity bytes the encoding
///   factory returns for the derivation context, under the tag the KEM
///   declares.
/// Arrange: each entitlement-scope selection of `DECLARED_KEM_SELECTIONS`
///   over every member of `PAIRING_CONCRETES`, each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`, and each case of
///   `DECLARED_ENCODING_SELECTIONS` that declares
///   `EncodingIdentifier::EthereumAbiV1`; the identity bytes are the bytes
///   `create_encoding` returns for the derivation context
///   `build_derivation_context` returns, under `DerivationContextDescription`,
///   through `build_mock_i_encoding_consumer`; `count` of `1`.
/// Act:     `cross_holder`, once per case, in the KEM consumer's method,
///   over `build_cross_holder_payload` of those bytes.
/// Assert:  `encode_scalar` of the record's identity element's `scalar`,
///   read through `identity_element_components`, equals `encode_scalar` of
///   the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds from
///   `K::DECLARATION.identity_tag`, those bytes, and `<P::Scalar as
///   ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`; each factory's result is
///   unpacked by `expect` naming the declared selection and the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `cross_holder`, with the route the spec states
///   for the entry.
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
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
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
            let record = success.vectors.first().expect("the one record");
            // Assert
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
