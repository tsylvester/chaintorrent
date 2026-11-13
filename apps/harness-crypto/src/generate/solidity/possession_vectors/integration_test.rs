#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    DECLARED_ENCODING_SELECTIONS, DecodeParams, EncodeParams, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_payload, create_encoding,
};
use envelope::{
    DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter, PossessionComponents,
    PossessionFromComponentsParams, PossessionFromComponentsPayload, PublicKeysComponents,
    PublicKeysFromComponentsErrorReturn, PublicKeysFromComponentsParams,
    PublicKeysFromComponentsPayload,
};
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter};
use pairing::{
    CreatePairingParamsOverrides, DecodeG1Params, DecodeG2Params, DecodeScalarParams,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES,
    build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::super::super::factory::provides::{
    VectorCount, VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::interface::{
    IPossessionVectorsCaseConsumer, PossessionVector, PossessionVectorDescription,
};
use super::mock::{
    PossessionVectorsCaseParamsOverrides, PossessionVectorsParamsOverrides,
    build_possession_vector, build_possession_vector_description,
    build_possession_vectors_case_deps, build_possession_vectors_case_params,
    build_possession_vectors_case_payload, build_possession_vectors_deps,
    build_possession_vectors_params, build_possession_vectors_payload, run_possession_vectors_case,
};
use super::possession_vectors;

/// Contract: own entry `every_run_emits_the_configured_count_of_records_with_distinct_secrets`;
///   `possession_vectors` over a count greater than one, through the adapters the
///   factories hand the consumers, returns `Ok` whose `records` hold the configured
///   count and no two records share an `x` or a `y`.
/// Arrange: every member of the declared sets; `build_possession_vectors_params` with
///   `count` overridden to `build_vector_count` over `3`, a literal greater than one
///   that differs from the builder's default `2`.
/// Act:    `possession_vectors::<P, K>` once per case, inside the case consumer's method.
/// Assert: in place, the result is `Ok`; `records` holds `3`; no two records share an
///   `x` and no two share a `y`.
/// Boundary: the chain of real functions the private surface lists, ending at
///   `possession_vectors`.
/// Mocked:  nothing; the operating system's generator behind `create_random_source`
///   runs real, so the block proves the generator's draws reach the records and does
///   not prove the generator's quality.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_secrets() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    struct Consumer {
        count: VectorCount,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the generation succeeds");
            assert_eq!(success.records.len(), 3);
            for first in 0..success.records.len() {
                for second in (first + 1)..success.records.len() {
                    assert_ne!(success.records[first].x, success.records[second].x);
                    assert_ne!(success.records[first].y, success.records[second].y);
                }
            }
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                    let result = run_possession_vectors_case(
                        &build_possession_vectors_case_deps(Consumer { count }, Default::default()),
                        build_possession_vectors_case_params(
                            PossessionVectorsCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_selection),
                                encoding: Some(CreateEncodingParams {
                                    concrete: encoding_selection.concrete,
                                    identifier: encoding_selection.identifier,
                                }),
                                key_agreement: Some(*key_agreement_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                            },
                        ),
                        build_possession_vectors_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
// every_run_emits_the_configured_count_of_records_with_distinct_secrets

/// Contract: own entry `every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths`;
///   `possession_vectors` over a count greater than one returns `Ok` whose records
///   hold each point in the length `encode_g1` and `encode_g2` give the pairing the
///   consumer holds and each scalar in an encoding `decode_scalar` admits.
/// Arrange: every member of the declared sets; `count` overridden to `3`; the
///   reference widths the lengths of `encode_g1` of `g1_generator` and `encode_g2`
///   of `g2_generator` through the pairing the consumer holds.
/// Act:    `possession_vectors::<P, K>` once per case, inside the case consumer's method.
/// Assert: in place, the result is `Ok`; each record's `pk1` and `r1` have the
///   `encode_g1` reference width and its `pk2` and `r2` the `encode_g2` reference
///   width; `decode_scalar` admits each record's `x`, `y`, `z1`, and `z2` and
///   `decode_g1` and `decode_g2` admit each record's keys and commitments.
/// Boundary: the chain of real functions the private surface lists, ending at
///   `possession_vectors`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    struct Consumer {
        count: VectorCount,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            // Arrange
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator exists")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator exists")
                .point;
            let g1_bytes = pairing
                .encode_g1(EncodeG1Params, EncodeG1Payload { point: g1 })
                .expect("the first-group generator encodes")
                .bytes;
            let g2_bytes = pairing
                .encode_g2(EncodeG2Params, EncodeG2Payload { point: g2 })
                .expect("the second-group generator encodes")
                .bytes;

            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the generation succeeds");
            for record in &success.records {
                assert_eq!(record.pk1.len(), g1_bytes.as_ref().len());
                assert_eq!(record.r1.len(), g1_bytes.as_ref().len());
                assert_eq!(record.pk2.len(), g2_bytes.as_ref().len());
                assert_eq!(record.r2.len(), g2_bytes.as_ref().len());
                pairing
                    .decode_scalar(DecodeScalarParams, &record.x)
                    .expect("x is an admitted scalar");
                pairing
                    .decode_scalar(DecodeScalarParams, &record.y)
                    .expect("y is an admitted scalar");
                pairing
                    .decode_scalar(DecodeScalarParams, &record.z1)
                    .expect("z1 is an admitted scalar");
                pairing
                    .decode_scalar(DecodeScalarParams, &record.z2)
                    .expect("z2 is an admitted scalar");
                pairing
                    .decode_g1(DecodeG1Params, &record.pk1)
                    .expect("pk1 is an admitted first-group point");
                pairing
                    .decode_g1(DecodeG1Params, &record.r1)
                    .expect("r1 is an admitted first-group point");
                pairing
                    .decode_g2(DecodeG2Params, &record.pk2)
                    .expect("pk2 is an admitted second-group point");
                pairing
                    .decode_g2(DecodeG2Params, &record.r2)
                    .expect("r2 is an admitted second-group point");
            }
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                    let result = run_possession_vectors_case(
                        &build_possession_vectors_case_deps(Consumer { count }, Default::default()),
                        build_possession_vectors_case_params(
                            PossessionVectorsCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_selection),
                                encoding: Some(CreateEncodingParams {
                                    concrete: encoding_selection.concrete,
                                    identifier: encoding_selection.identifier,
                                }),
                                key_agreement: Some(*key_agreement_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                            },
                        ),
                        build_possession_vectors_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
// every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths

/// Contract: own entry `every_record_carries_public_keys_that_are_the_generators_times_their_secrets`;
///   `possession_vectors` over a count greater than one returns `Ok` whose records
///   each carry a `pk1` equal to the encoding of the first-group generator times
///   its `x` and a `pk2` equal to the encoding of the second-group generator times
///   its `y`.
/// Arrange: every member of the declared sets; `count` overridden to `3`, so the
///   records differ from each other.
/// Act:    `possession_vectors::<P, K>` once per case, inside the case consumer's method.
/// Assert: in place, for each record `pk1` equals the `encode_g1` bytes of `mul_g1`
///   of the `g1_generator` point by the scalar `decode_scalar` returns for `x`, and
///   `pk2` equals the `encode_g2` bytes of `mul_g2` of the `g2_generator` point by
///   the scalar `decode_scalar` returns for `y`.
/// Boundary: the chain of real functions the private surface lists, ending at
///   `possession_vectors`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn every_record_carries_public_keys_that_are_the_generators_times_their_secrets() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    struct Consumer {
        count: VectorCount,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            // Arrange
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator exists")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator exists")
                .point;

            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the generation succeeds");
            for record in &success.records {
                let x = pairing
                    .decode_scalar(DecodeScalarParams, &record.x)
                    .expect("x is an admitted scalar")
                    .scalar;
                let y = pairing
                    .decode_scalar(DecodeScalarParams, &record.y)
                    .expect("y is an admitted scalar")
                    .scalar;
                let expected_pk1 = pairing
                    .mul_g1(
                        MulG1Params,
                        MulG1Payload {
                            point: g1.clone(),
                            scalar: x,
                        },
                    )
                    .expect("the first-group product exists")
                    .product;
                let expected_pk2 = pairing
                    .mul_g2(
                        MulG2Params,
                        MulG2Payload {
                            point: g2.clone(),
                            scalar: y,
                        },
                    )
                    .expect("the second-group product exists")
                    .product;
                let expected_pk1_bytes = pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: expected_pk1,
                        },
                    )
                    .expect("the first-group key encodes")
                    .bytes;
                let expected_pk2_bytes = pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: expected_pk2,
                        },
                    )
                    .expect("the second-group key encodes")
                    .bytes;
                assert_eq!(record.pk1.as_slice(), expected_pk1_bytes.as_ref());
                assert_eq!(record.pk2.as_slice(), expected_pk2_bytes.as_ref());
            }
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                    let result = run_possession_vectors_case(
                        &build_possession_vectors_case_deps(Consumer { count }, Default::default()),
                        build_possession_vectors_case_params(
                            PossessionVectorsCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_selection),
                                encoding: Some(CreateEncodingParams {
                                    concrete: encoding_selection.concrete,
                                    identifier: encoding_selection.identifier,
                                }),
                                key_agreement: Some(*key_agreement_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                            },
                        ),
                        build_possession_vectors_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
// every_record_carries_public_keys_that_are_the_generators_times_their_secrets

/// Contract: own entry `every_record_carries_proofs_the_key_agreement_admits`;
///   `possession_vectors` over a count greater than one returns `Ok` whose records
///   each pair keys and proof components that `public_keys_from_components` admits,
///   and a record's keys with another record's possession are refused with
///   `PossessionG1`.
/// Arrange: every member of the declared sets; `count` overridden to `3`, so the
///   proofs of two records differ.
/// Act:    `possession_vectors::<P, K>`, then `public_keys_from_components` over
///   `PublicKeysComponents { pk1, pk2 }` decoded from a record's keys and the
///   possession `possession_from_components` returns for `PossessionComponents {
///   r1, z1, r2, z2 }` decoded from the same record, once per case, inside the case
///   consumer's method.
/// Assert: in place, `Ok` for each record; the first record's keys with the second
///   record's possession return `Err(PublicKeysFromComponentsErrorReturn::
///   PossessionG1)` by `assert_eq!`.
/// Boundary: the chain of real functions the private surface lists, ending at
///   `possession_vectors` and the key agreement's `public_keys_from_components` and
///   `possession_from_components`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn every_record_carries_proofs_the_key_agreement_admits() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    struct Consumer {
        count: VectorCount,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the generation succeeds");
            let mut possessions = Vec::new();
            for record in &success.records {
                let pk1 = pairing
                    .decode_g1(DecodeG1Params, &record.pk1)
                    .expect("pk1 is an admitted first-group point")
                    .point;
                let pk2 = pairing
                    .decode_g2(DecodeG2Params, &record.pk2)
                    .expect("pk2 is an admitted second-group point")
                    .point;
                let r1 = pairing
                    .decode_g1(DecodeG1Params, &record.r1)
                    .expect("r1 is an admitted first-group point")
                    .point;
                let z1 = pairing
                    .decode_scalar(DecodeScalarParams, &record.z1)
                    .expect("z1 is an admitted scalar")
                    .scalar;
                let r2 = pairing
                    .decode_g2(DecodeG2Params, &record.r2)
                    .expect("r2 is an admitted second-group point")
                    .point;
                let z2 = pairing
                    .decode_scalar(DecodeScalarParams, &record.z2)
                    .expect("z2 is an admitted scalar")
                    .scalar;
                let possession = key_agreement
                    .possession_from_components(
                        PossessionFromComponentsParams,
                        PossessionFromComponentsPayload {
                            components: PossessionComponents { r1, z1, r2, z2 },
                        },
                    )
                    .expect("the possession rebuilds")
                    .possession;
                key_agreement
                    .public_keys_from_components(
                        PublicKeysFromComponentsParams,
                        PublicKeysFromComponentsPayload {
                            components: PublicKeysComponents { pk1, pk2 },
                            possession: &possession,
                        },
                    )
                    .expect("the keys are admitted with their own possession");
                possessions.push(possession);
            }
            let pk1 = pairing
                .decode_g1(DecodeG1Params, &success.records[0].pk1)
                .expect("pk1 is an admitted first-group point")
                .point;
            let pk2 = pairing
                .decode_g2(DecodeG2Params, &success.records[0].pk2)
                .expect("pk2 is an admitted second-group point")
                .point;
            let refused = key_agreement.public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents { pk1, pk2 },
                    possession: &possessions[1],
                },
            );
            assert_eq!(
                refused.err(),
                Some(PublicKeysFromComponentsErrorReturn::PossessionG1)
            );
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                    let result = run_possession_vectors_case(
                        &build_possession_vectors_case_deps(Consumer { count }, Default::default()),
                        build_possession_vectors_case_params(
                            PossessionVectorsCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_selection),
                                encoding: Some(CreateEncodingParams {
                                    concrete: encoding_selection.concrete,
                                    identifier: encoding_selection.identifier,
                                }),
                                key_agreement: Some(*key_agreement_selection),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                            },
                        ),
                        build_possession_vectors_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
// every_record_carries_proofs_the_key_agreement_admits

/// Contract: own entry `every_record_survives_the_canonical_encoding_under_the_description`;
///   a record encoded under `PossessionVectorDescription` through the encoder
///   `create_encoding` hands the consumer, and the bytes decoded under the same
///   description, returns the record.
/// Arrange: every member of `DECLARED_ENCODING_SELECTIONS`;
///   `build_possession_vector(Default::default())`, whose fields differ from each
///   other; `build_possession_vector_description(Default::default())`; a
///   block-local `IEncodingConsumer` whose `consume_encoding` calls the encoder's
///   `encode` over the description and the record, then `decode` of the bytes, and
///   asserts in place.
/// Act:    `encode`, then `decode` of its bytes under the same description, per member.
/// Assert: in place, `encode` returns `Ok`; `decode` returns `Ok` whose `described`
///   equals `build_possession_vector(Default::default())`.
/// Boundary: `create_encoding`, the selected concrete's `encode` and `decode`, and
///   `PossessionVectorDescription`'s `try_new`, `to_fields`, and `fields_to_value`.
/// Mocked:  nothing; the chain has no outer edge.
#[test]
fn every_record_survives_the_canonical_encoding_under_the_description() {
    // Arrange
    let record = build_possession_vector(Default::default());
    let description = build_possession_vector_description(Default::default());
    struct Consumer<'a> {
        record: &'a PossessionVector,
        description: &'a PossessionVectorDescription,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            // Act
            let encoded = payload
                .adapter
                .encode(
                    EncodeParams {
                        description: self.description,
                    },
                    self.record,
                )
                .expect("the record encodes");
            let decoded = payload
                .adapter
                .decode(
                    DecodeParams {
                        description: self.description,
                    },
                    &encoded.bytes,
                )
                .expect("the bytes decode");

            // Assert
            assert_eq!(
                decoded.described,
                build_possession_vector(Default::default())
            );
        }
    }

    // Act
    for selection in DECLARED_ENCODING_SELECTIONS {
        let result = create_encoding(
            &build_create_encoding_deps(Consumer {
                record: &record,
                description: &description,
            }),
            CreateEncodingParams {
                concrete: selection.concrete,
                identifier: selection.identifier,
            },
            build_create_encoding_payload(Default::default()),
        );
        assert!(result.is_ok());
    }
}
// every_record_survives_the_canonical_encoding_under_the_description
