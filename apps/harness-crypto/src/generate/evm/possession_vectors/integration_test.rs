#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::group_vectors::provides::{
    VectorCount, VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::interface::*;
use super::mock::*;
use super::possession_vectors;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_decode_params, build_encode_params, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, DECLARED_KEY_AGREEMENT_SELECTIONS,
    IKeyAgreementAdapter, IKeyAgreementConsumer, PossessionComponents,
    PossessionFromComponentsParams, PossessionFromComponentsPayload, PublicKeysComponents,
    PublicKeysFromComponentsErrorReturn, PublicKeysFromComponentsParams,
    PublicKeysFromComponentsPayload, build_create_key_agreement_deps,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeG1Params, DecodeG2Params, DecodeScalarParams, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

/// Contract: entry `every_run_emits_the_configured_count_of_records_with_distinct_secrets`;
///   `possession_vectors` over a count greater than one → `Ok` whose
///   `records` hold the configured count and no two records share an `x`
///   or a `y`.
/// Arrange: every member of `PAIRING_CONCRETES`, each over every member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS` with every selection of
///   `DECLARED_ENCODING_SELECTIONS` and `DECLARED_HASH_TO_SCALAR_SELECTIONS`;
///   `count` overridden to `build_vector_count` over `3`, which differs
///   from the params builder's default; the random source the operating
///   system supplies.
/// Act:     `possession_vectors::<P, K>` once per case, inside
///   `KeyAgreementConsumer`.
/// Assert:  in place, the result is `Ok`; `records` holds the configured
///   count; no two records share an `x` and no two share a `y`.
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_random_source` for `RandomSourceKind::OperatingSystem`,
///   `create_key_agreement`, the selection's concrete through its
///   `try_new`, and `possession_vectors`, with `Secret` running for real.
/// Mocked:  nothing; the operating system's generator is the outer edge
///   and runs real, so the block proves the generator's draws reach the
///   records and does not prove the generator's quality.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_secrets() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P> {
        type Output = ();

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(
                success.records.len(),
                usize::try_from(self.count.get())
                    .expect("a u32 count fits a usize on every supported target")
            );
            for (index, record) in success.records.iter().enumerate() {
                for other in success.records.iter().skip(index + 1) {
                    assert_ne!(record.x, other.x);
                    assert_ne!(record.y, other.y);
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'a, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS.iter().copied() {
                let Ok(_) = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        &payload.adapter,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            random: self.random,
                            count: self.count,
                        },
                        Default::default(),
                    ),
                    selection,
                    build_create_key_agreement_payload(Default::default()),
                ) else {
                    panic!("the declared key-agreement selection is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for hashing_params in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_params.concrete),
                        identifier: Some(hashing_params.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_params in DECLARED_ENCODING_SELECTIONS {
                    let Ok(_) = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            count: self.count,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_params.concrete),
                            identifier: Some(encoding_params.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    ) else {
                        panic!("the declared encoding selection is created")
                    };
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        count,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let Ok(_) = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        ) else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // every_run_emits_the_configured_count_of_records_with_distinct_secrets
}

/// Contract: entry
///   `every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths`;
///   `possession_vectors` over a count greater than one → `Ok` whose
///   records hold each point in the length `encode_g1` and `encode_g2`
///   give the pairing the consumer holds and each scalar in an encoding
///   `decode_scalar` admits.
/// Arrange: every member of the declared sets, as in
///   `every_run_emits_the_configured_count_of_records_with_distinct_secrets`;
///   `count` overridden to `3`; the reference widths are the lengths of
///   `encode_g1` of `g1_generator` and `encode_g2` of `g2_generator`
///   through the pairing the consumer holds.
/// Act:     `possession_vectors::<P, K>` once per case, inside
///   `KeyAgreementConsumer`.
/// Assert:  in place, the result is `Ok`; each record's `pk1` and `r1`
///   have the `encode_g1` reference width and its `pk2` and `r2` the
///   `encode_g2` reference width; `decode_scalar` admits each record's
///   `x`, `y`, `z1`, and `z2`, and `decode_g1` and `decode_g2` admit each
///   record's keys and commitments.
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_random_source` for `RandomSourceKind::OperatingSystem`,
///   `create_key_agreement`, the selection's concrete through its
///   `try_new`, `possession_vectors`, and the pairing's `g1_generator`,
///   `g2_generator`, `encode_g1`, `encode_g2`, `decode_scalar`,
///   `decode_g1`, and `decode_g2`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P> {
        type Output = ();

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Arrange
            let Ok(g1_generator) = self
                .pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2_generator) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(reference_g1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: g1_generator.point,
                },
            );
            let Ok(reference_g2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: g2_generator.point,
                },
            );

            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(
                success.records.len(),
                usize::try_from(self.count.get())
                    .expect("a u32 count fits a usize on every supported target")
            );
            for record in &success.records {
                assert_eq!(record.pk1.len(), reference_g1.bytes.as_ref().len());
                assert_eq!(record.r1.len(), reference_g1.bytes.as_ref().len());
                assert_eq!(record.pk2.len(), reference_g2.bytes.as_ref().len());
                assert_eq!(record.r2.len(), reference_g2.bytes.as_ref().len());
                let Ok(_) = self.pairing.decode_scalar(DecodeScalarParams, &record.x) else {
                    panic!("the record's x is a scalar encoding the pairing admits")
                };
                let Ok(_) = self.pairing.decode_scalar(DecodeScalarParams, &record.y) else {
                    panic!("the record's y is a scalar encoding the pairing admits")
                };
                let Ok(_) = self.pairing.decode_scalar(DecodeScalarParams, &record.z1) else {
                    panic!("the record's z1 is a scalar encoding the pairing admits")
                };
                let Ok(_) = self.pairing.decode_scalar(DecodeScalarParams, &record.z2) else {
                    panic!("the record's z2 is a scalar encoding the pairing admits")
                };
                let Ok(_) = self.pairing.decode_g1(DecodeG1Params, &record.pk1) else {
                    panic!("the record's pk1 is a first-group encoding the pairing admits")
                };
                let Ok(_) = self.pairing.decode_g1(DecodeG1Params, &record.r1) else {
                    panic!("the record's r1 is a first-group encoding the pairing admits")
                };
                let Ok(_) = self.pairing.decode_g2(DecodeG2Params, &record.pk2) else {
                    panic!("the record's pk2 is a second-group encoding the pairing admits")
                };
                let Ok(_) = self.pairing.decode_g2(DecodeG2Params, &record.r2) else {
                    panic!("the record's r2 is a second-group encoding the pairing admits")
                };
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'a, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS.iter().copied() {
                let Ok(_) = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        &payload.adapter,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            random: self.random,
                            count: self.count,
                        },
                        Default::default(),
                    ),
                    selection,
                    build_create_key_agreement_payload(Default::default()),
                ) else {
                    panic!("the declared key-agreement selection is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for hashing_params in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_params.concrete),
                        identifier: Some(hashing_params.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_params in DECLARED_ENCODING_SELECTIONS {
                    let Ok(_) = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            count: self.count,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_params.concrete),
                            identifier: Some(encoding_params.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    ) else {
                        panic!("the declared encoding selection is created")
                    };
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        count,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let Ok(_) = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        ) else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths
}

/// Contract: entry
///   `every_record_carries_public_keys_that_are_the_generators_times_their_secrets`;
///   `possession_vectors` over a count greater than one → `Ok` whose
///   records carry, for each record, a `pk1` equal to the encoding of the
///   first-group generator times its `x` and a `pk2` equal to the
///   encoding of the second-group generator times its `y`.
/// Arrange: every member of the declared sets, as in
///   `every_run_emits_the_configured_count_of_records_with_distinct_secrets`;
///   `count` overridden to `3`, so records differ from each other.
/// Act:     `possession_vectors::<P, K>` once per case, inside
///   `KeyAgreementConsumer`.
/// Assert:  in place, for each record `pk1` equals the `encode_g1` bytes
///   of `mul_g1` of the `g1_generator` point by the scalar `decode_scalar`
///   returns for `x`, and `pk2` equals the `encode_g2` bytes of `mul_g2`
///   of the `g2_generator` point by the scalar `decode_scalar` returns
///   for `y`, each by `assert_eq!`.
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_random_source` for `RandomSourceKind::OperatingSystem`,
///   `create_key_agreement`, the selection's concrete through its
///   `try_new`, `possession_vectors`, and the pairing's `g1_generator`,
///   `g2_generator`, `mul_g1`, `mul_g2`, `decode_scalar`, `encode_g1`,
///   and `encode_g2`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn every_record_carries_public_keys_that_are_the_generators_times_their_secrets() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P> {
        type Output = ();

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Arrange
            let Ok(g1_generator) = self
                .pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2_generator) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);

            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(
                success.records.len(),
                usize::try_from(self.count.get())
                    .expect("a u32 count fits a usize on every supported target")
            );
            for record in &success.records {
                let Ok(x) = self.pairing.decode_scalar(DecodeScalarParams, &record.x) else {
                    panic!("the record's x is a scalar encoding the pairing admits")
                };
                let Ok(product) = self.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1_generator.point.clone(),
                        scalar: x.scalar,
                    },
                );
                let Ok(encoded) = self.pairing.encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: product.product,
                    },
                );
                assert_eq!(record.pk1.as_slice(), encoded.bytes.as_ref());
                let Ok(y) = self.pairing.decode_scalar(DecodeScalarParams, &record.y) else {
                    panic!("the record's y is a scalar encoding the pairing admits")
                };
                let Ok(product) = self.pairing.mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2_generator.point.clone(),
                        scalar: y.scalar,
                    },
                );
                let Ok(encoded) = self.pairing.encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: product.product,
                    },
                );
                assert_eq!(record.pk2.as_slice(), encoded.bytes.as_ref());
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'a, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS.iter().copied() {
                let Ok(_) = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        &payload.adapter,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            random: self.random,
                            count: self.count,
                        },
                        Default::default(),
                    ),
                    selection,
                    build_create_key_agreement_payload(Default::default()),
                ) else {
                    panic!("the declared key-agreement selection is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for hashing_params in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_params.concrete),
                        identifier: Some(hashing_params.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_params in DECLARED_ENCODING_SELECTIONS {
                    let Ok(_) = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            count: self.count,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_params.concrete),
                            identifier: Some(encoding_params.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    ) else {
                        panic!("the declared encoding selection is created")
                    };
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        count,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let Ok(_) = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        ) else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // every_record_carries_public_keys_that_are_the_generators_times_their_secrets
}

/// Contract: entry `every_record_carries_proofs_the_key_agreement_admits`;
///   `possession_vectors` over a count greater than one → `Ok` whose
///   records each pair keys and proof components that
///   `public_keys_from_components` admits, and a record's keys with
///   another record's possession are refused with `PossessionG1`.
/// Arrange: every member of the declared sets, as in
///   `every_run_emits_the_configured_count_of_records_with_distinct_secrets`;
///   `count` overridden to `3`, so the proofs of two records differ.
/// Act:     `possession_vectors::<P, K>` once per case, then
///   `public_keys_from_components` over `PublicKeysComponents { pk1, pk2 }`
///   decoded from a record's keys and the possession
///   `possession_from_components` returns for `PossessionComponents
///   { r1, z1, r2, z2 }` decoded from the same record, inside
///   `KeyAgreementConsumer`.
/// Assert:  in place, the admission is `Ok` for each record; the first
///   record's keys with the second record's possession return
///   `Err(PublicKeysFromComponentsErrorReturn::PossessionG1)` by
///   `assert_eq!`.
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_random_source` for `RandomSourceKind::OperatingSystem`,
///   `create_key_agreement`, the selection's concrete through its
///   `try_new`, `possession_vectors`, the pairing's `decode_g1`,
///   `decode_g2`, and `decode_scalar`, and the key agreement's
///   `public_keys_from_components` and `possession_from_components`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn every_record_carries_proofs_the_key_agreement_admits() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P> {
        type Output = ();

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            let result = possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: Some(self.count),
                }),
                build_possession_vectors_payload(Default::default()),
            );
            let success = result.expect("the declared selections admit the call");
            assert_eq!(
                success.records.len(),
                usize::try_from(self.count.get())
                    .expect("a u32 count fits a usize on every supported target")
            );
            let mut keys = Vec::new();
            let mut possessions = Vec::new();
            for record in &success.records {
                let Ok(pk1) = self.pairing.decode_g1(DecodeG1Params, &record.pk1) else {
                    panic!("the record's pk1 is a first-group encoding the pairing admits")
                };
                let Ok(pk2) = self.pairing.decode_g2(DecodeG2Params, &record.pk2) else {
                    panic!("the record's pk2 is a second-group encoding the pairing admits")
                };
                let Ok(r1) = self.pairing.decode_g1(DecodeG1Params, &record.r1) else {
                    panic!("the record's r1 is a first-group encoding the pairing admits")
                };
                let Ok(z1) = self.pairing.decode_scalar(DecodeScalarParams, &record.z1) else {
                    panic!("the record's z1 is a scalar encoding the pairing admits")
                };
                let Ok(r2) = self.pairing.decode_g2(DecodeG2Params, &record.r2) else {
                    panic!("the record's r2 is a second-group encoding the pairing admits")
                };
                let Ok(z2) = self.pairing.decode_scalar(DecodeScalarParams, &record.z2) else {
                    panic!("the record's z2 is a scalar encoding the pairing admits")
                };
                let Ok(possession) = payload.adapter.possession_from_components(
                    PossessionFromComponentsParams,
                    PossessionFromComponentsPayload {
                        components: PossessionComponents {
                            r1: r1.point,
                            z1: z1.scalar,
                            r2: r2.point,
                            z2: z2.scalar,
                        },
                    },
                );
                keys.push((pk1.point, pk2.point));
                possessions.push(possession.possession);
            }

            // Assert
            for ((pk1, pk2), possession) in keys.iter().zip(possessions.iter()) {
                let Ok(_) = payload.adapter.public_keys_from_components(
                    PublicKeysFromComponentsParams,
                    PublicKeysFromComponentsPayload {
                        components: PublicKeysComponents {
                            pk1: pk1.clone(),
                            pk2: pk2.clone(),
                        },
                        possession,
                    },
                ) else {
                    panic!("each record's keys admit the record's own possession")
                };
            }
            let cross = payload.adapter.public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: keys[0].0.clone(),
                        pk2: keys[0].1.clone(),
                    },
                    possession: &possessions[1],
                },
            );
            let Err(error) = cross else {
                panic!("another record's possession is refused under this record's keys")
            };
            assert_eq!(error, PublicKeysFromComponentsErrorReturn::PossessionG1);
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a, P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'a, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS.iter().copied() {
                let Ok(_) = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        &payload.adapter,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            random: self.random,
                            count: self.count,
                        },
                        Default::default(),
                    ),
                    selection,
                    build_create_key_agreement_payload(Default::default()),
                ) else {
                    panic!("the declared key-agreement selection is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for hashing_params in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_params.concrete),
                        identifier: Some(hashing_params.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_params in DECLARED_ENCODING_SELECTIONS {
                    let Ok(_) = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            count: self.count,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_params.concrete),
                            identifier: Some(encoding_params.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    ) else {
                        panic!("the declared encoding selection is created")
                    };
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        count,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let Ok(_) = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        ) else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // every_record_carries_proofs_the_key_agreement_admits
}

/// Contract: entry
///   `every_record_survives_the_canonical_encoding_under_the_description`;
///   a record encoded under `PossessionVectorDescription` through the
///   encoder `create_encoding` hands the consumer, and the bytes decoded
///   under the same description, returns the record.
/// Arrange: every member of `DECLARED_ENCODING_SELECTIONS`;
///   `build_possession_vector(Default::default())`, whose fields differ
///   from each other; `build_possession_vector_description(Default::default())`.
/// Act:     `encode` over the description and the record, then `decode`
///   of its bytes under the same description, per member, inside
///   `EncodingConsumer`.
/// Assert:  in place, `encode` returns `Ok` and `decode` returns `Ok`
///   whose `described` equals `build_possession_vector(Default::default())`
///   by `assert_eq!`.
/// Boundary: `create_encoding`, the selected concrete's `encode` and
///   `decode`, and `PossessionVectorDescription`'s `try_new`, `to_fields`,
///   and `fields_to_value`.
/// Mocked:  nothing; the chain has no outer edge.
#[test]
fn every_record_survives_the_canonical_encoding_under_the_description() {
    // Arrange
    let record = build_possession_vector(Default::default());
    let description = build_possession_vector_description(Default::default());

    struct EncodingConsumer<'a> {
        description: &'a PossessionVectorDescription,
        record: &'a PossessionVector,
    }

    impl<'a> IEncodingConsumer for &EncodingConsumer<'a> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            // Act
            let encoded = payload
                .adapter
                .encode(build_encode_params(self.description), self.record);
            let Ok(encoded) = encoded else {
                panic!("the description emits only the kinds the encodings declare")
            };
            let decoded = payload
                .adapter
                .decode(build_decode_params(self.description), &encoded.bytes);

            // Assert
            let decoded = decoded.expect("the encoded bytes decode under the description");
            assert_eq!(
                decoded.described,
                build_possession_vector(Default::default())
            );
        }
    }

    let consumer = EncodingConsumer {
        description: &description,
        record: &record,
    };

    // Act & Assert per encoding selection
    for selection in DECLARED_ENCODING_SELECTIONS {
        let Ok(_) = create_encoding(
            &build_create_encoding_deps(&consumer),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(selection.concrete),
                identifier: Some(selection.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        ) else {
            panic!("the declared encoding selection is created")
        };
    }
    // every_record_survives_the_canonical_encoding_under_the_description
}
