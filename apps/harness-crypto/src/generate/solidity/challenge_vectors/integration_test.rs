#![allow(clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::build_secret;
use encoding::{
    CanonicalFieldValue, CreateEncodingParams, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, DecodeParams, ICanonicalField, IDecoderAdapter, IEncoderAdapter,
    IEncodingContract, ToFieldsParams, build_create_encoding_params, build_from_field_params,
};
use hash_to_scalar::{
    CreateHashToScalarParams, CreateHashToScalarParamsOverrides,
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarIdentifier, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params, build_domain_tag,
    expected_uniform_bytes,
};
use pairing::{
    CreatePairingParamsOverrides, DecodeScalarParams, EncodeG1Params, EncodeG1Payload,
    EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayloadOverrides,
    G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, IPairingReference, ISampleUniformScalar, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayloadOverrides, build_create_pairing_params,
    build_encode_scalar_payload, build_sample_uniform_scalar_payload,
};
use proof::{
    DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_REPLACEMENT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, DeliveryProofDeclarationOverrides,
    MintPurpose, MintStatementDescription, MintStatementDescriptionConstructorParams,
    TransferPurpose, TransferStatementDescription, TransferStatementDescriptionConstructorParams,
    build_delivery_context, build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::challenge_vectors;
use super::interface::{
    ChallengeVectorsCaseParams, ChallengeVectorsCasePayload, ChallengeVectorsErrorReturn,
    IChallengeVectorsCaseConsumer,
};
use super::mock::{
    ChallengeVectorsCaseParamsOverrides, ChallengeVectorsCasePayloadOverrides,
    ChallengeVectorsParamsOverrides, build_challenge_vectors_case_deps,
    build_challenge_vectors_case_params, build_challenge_vectors_case_payload,
    build_challenge_vectors_deps, build_challenge_vectors_params, build_challenge_vectors_payload,
    run_challenge_vectors_case,
};
use crate::config::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, build_canonical_delivery_context,
};

fn delivery_context_overrides<F: IChainForms>(
    context: &CanonicalDeliveryContext,
) -> DeliveryContextOverrides<F> {
    DeliveryContextOverrides {
        suite_identifier: Some(context.suite_identifier.clone()),
        chain: Some(
            F::ChainIdentifier::from_field(
                build_from_field_params(Default::default()),
                context.chain.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        entitlement_contract: Some(
            F::Identity::from_field(
                build_from_field_params(Default::default()),
                context.entitlement_contract.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        asset_root: Some(context.asset_root),
        parameter_set_identifier: Some(context.parameter_set_identifier.clone()),
        source_entitlement: Some(
            F::Entitlement::from_field(
                build_from_field_params(Default::default()),
                context.source_entitlement.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        target_entitlement: Some(
            F::Entitlement::from_field(
                build_from_field_params(Default::default()),
                context.target_entitlement.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        old_interval: Some(
            F::Interval::from_field(
                build_from_field_params(Default::default()),
                context.old_interval.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        new_interval: Some(
            F::Interval::from_field(
                build_from_field_params(Default::default()),
                context.new_interval.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        seller: Some(
            F::Identity::from_field(
                build_from_field_params(Default::default()),
                context.seller.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        buyer: Some(
            F::Identity::from_field(
                build_from_field_params(Default::default()),
                context.buyer.clone(),
            )
            .expect("a canonical value of the declared kind is admitted")
            .value,
        ),
        expiry: Some(context.expiry),
    }
}

fn case_params(
    pairing_concrete: PairingConcrete,
    hash_to_scalar: &CreateHashToScalarParams,
    encoding_selection: &CreateEncodingParams,
    forms_concrete: ChainFormsConcrete,
) -> ChallengeVectorsCaseParams {
    build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        })),
        hash_to_scalar: Some(build_create_hash_to_scalar_params(
            CreateHashToScalarParamsOverrides {
                concrete: Some(hash_to_scalar.concrete),
                identifier: Some(hash_to_scalar.identifier),
            },
        )),
        encoding: Some(build_create_encoding_params(
            CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            },
        )),
        chain_forms: Some(build_create_chain_forms_params(
            CreateChainFormsParamsOverrides {
                concrete: Some(forms_concrete),
                ..Default::default()
            },
        )),
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            },
        )),
    })
}

fn case_payload(
    mint: CanonicalDeliveryContext,
    mint_purpose: MintPurpose,
    transfer: CanonicalDeliveryContext,
    transfer_purpose: TransferPurpose,
) -> ChallengeVectorsCasePayload {
    build_challenge_vectors_case_payload(ChallengeVectorsCasePayloadOverrides {
        delivery_proof: Some(build_delivery_proof_declaration(
            DeliveryProofDeclarationOverrides {
                challenge_tag: Some(b"test-challenge-vectors-tag"),
                ..Default::default()
            },
        )),
        mint: Some(mint),
        mint_purpose: Some(mint_purpose),
        transfer: Some(transfer),
        transfer_purpose: Some(transfer_purpose),
    })
}

/// Contract: `challenge_vectors` with a `count` above one → per index a mint
///   record then a transfer record, each `purpose` the code of its context's
///   purpose, each `draws` one 32-byte scalar encoding per transcript point,
///   and no two records sharing `draws`.
/// Arrange: `count` of `2`; the mint and transfer contexts at their default
///   values with the purposes `DELIVERY_PURPOSE_REPLACEMENT` and
///   `DELIVERY_PURPOSE_GRANT`, whose codes are not those of
///   `DELIVERY_PURPOSE_MINT` and `DELIVERY_PURPOSE_TRANSFER`.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES` and each declared selection.
/// Assert: the result is `Ok`; `records` holds four records whose `purpose`
///   values are `4`, `3`, `4`, and `3` in that order and whose `draws` hold
///   `352`, `576`, `352`, and `576` bytes; no two records' `draws` are equal.
/// Boundary: `challenge_vectors` over the real chain of the private surface.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order() {
    struct Consumer;

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            // Arrange
            let mint = build_delivery_context::<F, MintPurpose>(
                payload.mint_purpose,
                delivery_context_overrides(&payload.mint),
            );
            let transfer = build_delivery_context::<F, TransferPurpose>(
                payload.transfer_purpose,
                delivery_context_overrides(&payload.transfer),
            );

            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    Default::default(),
                ),
            );

            // Assert
            let records = result.expect("the generation succeeds").records;
            assert_eq!(records.len(), 4);
            let purposes: Vec<u16> = records.iter().map(|record| record.purpose).collect();
            assert_eq!(purposes, vec![4, 3, 4, 3]);
            let lengths: Vec<usize> = records.iter().map(|record| record.draws.len()).collect();
            assert_eq!(lengths, vec![352, 576, 352, 576]);
            for (left, record) in records.iter().enumerate() {
                for other in records.iter().skip(left + 1) {
                    assert_ne!(record.draws, other.draws);
                }
            }
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    assert!(
                        run_challenge_vectors_case(
                            &build_challenge_vectors_case_deps(Consumer, Default::default()),
                            case_params(
                                *pairing_concrete,
                                hash_selection,
                                encoding_selection,
                                *forms_concrete,
                            ),
                            case_payload(
                                build_canonical_delivery_context(Default::default()),
                                DELIVERY_PURPOSE_REPLACEMENT,
                                build_canonical_delivery_context(Default::default()),
                                DELIVERY_PURPOSE_GRANT,
                            ),
                        )
                        .is_ok()
                    );
                }
            }
        }
    }
}
// challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order

/// Contract: `challenge_vectors` over configured contexts → each record's
///   `encoding`, decoded under its statement description and read through
///   `to_fields`, holds the context's configured values at the canonical field
///   indices.
/// Arrange: `count` of `1`, the purposes `DELIVERY_PURPOSE_REPLACEMENT` and
///   `DELIVERY_PURPOSE_GRANT`, and `mint` and `transfer` the
///   `CanonicalDeliveryContext` values below.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES` and each declared selection.
/// Assert: the result is `Ok`; the mint record's decoded transcript holds at
///   indices `2`, `3`, `6`, `7`, `8`, `9`, `10`, `11`, `12`, and `19` the
///   arranged values; the transfer record's holds its own at `2`, `3`, `6`,
///   `7`, `8`, `9`, `10`, `11`, `12`, and `25`.
/// Boundary: `challenge_vectors`, the forms' `from_field` in the arrangement
///   and `to_field`, the description's `to_fields`, and the encoder's `encode`
///   and `decode`.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_place_each_contexts_configured_values_in_its_transcript() {
    struct Consumer;

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            // Arrange
            let mint = build_delivery_context::<F, MintPurpose>(
                payload.mint_purpose,
                delivery_context_overrides(&payload.mint),
            );
            let transfer = build_delivery_context::<F, TransferPurpose>(
                payload.transfer_purpose,
                delivery_context_overrides(&payload.transfer),
            );

            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    Default::default(),
                ),
            );

            // Assert
            let records = result.expect("the generation succeeds").records;
            assert_eq!(records.len(), 2);
            let Ok(mint_description) = MintStatementDescription::<P, F>::try_new(
                MintStatementDescriptionConstructorParams { pairing },
            );
            let mint_fields = mint_description
                .to_fields(
                    ToFieldsParams,
                    &encoder
                        .decode(
                            DecodeParams {
                                description: &mint_description,
                            },
                            &records[0].encoding,
                        )
                        .expect("the mint encoding decodes")
                        .described,
                )
                .expect("the mint transcript maps")
                .fields;
            let mint_expected: [(usize, CanonicalFieldValue); 10] = [
                (2, CanonicalFieldValue::Unsigned256([0x21; 32])),
                (3, CanonicalFieldValue::FixedBytes20([0x31; 20])),
                (6, CanonicalFieldValue::Unsigned256([0x41; 32])),
                (7, CanonicalFieldValue::Unsigned256([0x42; 32])),
                (8, CanonicalFieldValue::Unsigned64(5)),
                (9, CanonicalFieldValue::Unsigned64(6)),
                (10, CanonicalFieldValue::Unsigned16(4)),
                (11, CanonicalFieldValue::FixedBytes20([0x61; 20])),
                (12, CanonicalFieldValue::FixedBytes20([0x62; 20])),
                (19, CanonicalFieldValue::Unsigned64(1_800_000_000)),
            ];
            for (index, value) in mint_expected {
                assert_eq!(mint_fields.values[index], value);
            }
            let Ok(transfer_description) = TransferStatementDescription::<P, F>::try_new(
                TransferStatementDescriptionConstructorParams { pairing },
            );
            let transfer_fields = transfer_description
                .to_fields(
                    ToFieldsParams,
                    &encoder
                        .decode(
                            DecodeParams {
                                description: &transfer_description,
                            },
                            &records[1].encoding,
                        )
                        .expect("the transfer encoding decodes")
                        .described,
                )
                .expect("the transfer transcript maps")
                .fields;
            let transfer_expected: [(usize, CanonicalFieldValue); 10] = [
                (2, CanonicalFieldValue::Unsigned256([0x21; 32])),
                (3, CanonicalFieldValue::FixedBytes20([0x32; 20])),
                (6, CanonicalFieldValue::Unsigned256([0x43; 32])),
                (7, CanonicalFieldValue::Unsigned256([0x43; 32])),
                (8, CanonicalFieldValue::Unsigned64(6)),
                (9, CanonicalFieldValue::Unsigned64(7)),
                (10, CanonicalFieldValue::Unsigned16(3)),
                (11, CanonicalFieldValue::FixedBytes20([0x62; 20])),
                (12, CanonicalFieldValue::FixedBytes20([0x63; 20])),
                (25, CanonicalFieldValue::Unsigned64(1_900_000_000)),
            ];
            for (index, value) in transfer_expected {
                assert_eq!(transfer_fields.values[index], value);
            }
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    assert!(
                        run_challenge_vectors_case(
                            &build_challenge_vectors_case_deps(Consumer, Default::default()),
                            case_params(
                                *pairing_concrete,
                                hash_selection,
                                encoding_selection,
                                *forms_concrete,
                            ),
                            case_payload(
                                build_canonical_delivery_context(
                                    CanonicalDeliveryContextOverrides {
                                        chain: Some(CanonicalFieldValue::Unsigned256([0x21; 32])),
                                        entitlement_contract: Some(
                                            CanonicalFieldValue::FixedBytes20([0x31; 20]),
                                        ),
                                        source_entitlement: Some(CanonicalFieldValue::Unsigned256(
                                            [0x41; 32]
                                        ),),
                                        target_entitlement: Some(CanonicalFieldValue::Unsigned256(
                                            [0x42; 32]
                                        ),),
                                        old_interval: Some(CanonicalFieldValue::Unsigned64(5)),
                                        new_interval: Some(CanonicalFieldValue::Unsigned64(6)),
                                        seller: Some(
                                            CanonicalFieldValue::FixedBytes20([0x61; 20],)
                                        ),
                                        buyer: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
                                        expiry: Some(1_800_000_000),
                                        ..Default::default()
                                    },
                                ),
                                DELIVERY_PURPOSE_REPLACEMENT,
                                build_canonical_delivery_context(
                                    CanonicalDeliveryContextOverrides {
                                        chain: Some(CanonicalFieldValue::Unsigned256([0x21; 32])),
                                        entitlement_contract: Some(
                                            CanonicalFieldValue::FixedBytes20([0x32; 20]),
                                        ),
                                        source_entitlement: Some(CanonicalFieldValue::Unsigned256(
                                            [0x43; 32]
                                        ),),
                                        target_entitlement: Some(CanonicalFieldValue::Unsigned256(
                                            [0x43; 32]
                                        ),),
                                        old_interval: Some(CanonicalFieldValue::Unsigned64(6)),
                                        new_interval: Some(CanonicalFieldValue::Unsigned64(7)),
                                        seller: Some(
                                            CanonicalFieldValue::FixedBytes20([0x62; 20],)
                                        ),
                                        buyer: Some(CanonicalFieldValue::FixedBytes20([0x63; 20])),
                                        expiry: Some(1_900_000_000),
                                        ..Default::default()
                                    },
                                ),
                                DELIVERY_PURPOSE_GRANT,
                            ),
                        )
                        .is_ok()
                    );
                }
            }
        }
    }
}
// challenge_vectors_place_each_contexts_configured_values_in_its_transcript

/// Contract: `challenge_vectors` → every transcript point is the generator of
///   its group times the scalar the record's `draws` hold at its position.
/// Arrange: `count` of `1`; the contexts at their default values with
///   `DELIVERY_PURPOSE_MINT` and `DELIVERY_PURPOSE_TRANSFER`; the group-field
///   indices written as literals, `13` through `18` and `20` through `24` for
///   the mint transcript and `13` through `24` and `26` through `31` for the
///   transfer transcript, in the order the descriptions list their group
///   fields.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES` and each declared selection.
/// Assert: the result is `Ok`; for each record, `record.draws.len()` equals
///   `group_fields.len() * 32`, and each chunk of
///   `record.draws.as_chunks::<32>().0` in order, decoded through
///   `decode_scalar`, multiplied onto the generator of the group whose
///   encoding has the length of the transcript value at the chunk's index, and
///   encoded, equals `CanonicalFieldValue::Bytes` of the transcript's value at
///   that index.
/// Boundary: `challenge_vectors`, the pairing's `decode_scalar`,
///   `g1_generator`, `g2_generator`, `mul_g1`, `mul_g2`, `encode_g1`, and
///   `encode_g2`, and the encoder's `decode`.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar() {
    const MINT_GROUP_FIELDS: [usize; 11] = [13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24];
    const TRANSFER_GROUP_FIELDS: [usize; 18] = [
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 31,
    ];

    struct Consumer;

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            // Arrange
            let mint = build_delivery_context::<F, MintPurpose>(
                payload.mint_purpose,
                delivery_context_overrides(&payload.mint),
            );
            let transfer = build_delivery_context::<F, TransferPurpose>(
                payload.transfer_purpose,
                delivery_context_overrides(&payload.transfer),
            );

            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    Default::default(),
                ),
            );

            // Assert
            let records = result.expect("the generation succeeds").records;
            assert_eq!(records.len(), 2);
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(encoded_g1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: g1.point.clone(),
                },
            );
            let Ok(encoded_g2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: g2.point.clone(),
                },
            );
            for (position, record) in records.iter().enumerate() {
                let group_fields: &[usize] = if position % 2 == 0 {
                    &MINT_GROUP_FIELDS
                } else {
                    &TRANSFER_GROUP_FIELDS
                };
                assert_eq!(record.draws.len(), group_fields.len() * 32);
                let fields = if position % 2 == 0 {
                    let Ok(description) = MintStatementDescription::<P, F>::try_new(
                        MintStatementDescriptionConstructorParams { pairing },
                    );
                    let described = encoder
                        .decode(
                            DecodeParams {
                                description: &description,
                            },
                            &record.encoding,
                        )
                        .expect("the mint encoding decodes")
                        .described;
                    description
                        .to_fields(ToFieldsParams, &described)
                        .expect("the mint transcript maps")
                        .fields
                } else {
                    let Ok(description) = TransferStatementDescription::<P, F>::try_new(
                        TransferStatementDescriptionConstructorParams { pairing },
                    );
                    let described = encoder
                        .decode(
                            DecodeParams {
                                description: &description,
                            },
                            &record.encoding,
                        )
                        .expect("the transfer encoding decodes")
                        .described;
                    description
                        .to_fields(ToFieldsParams, &described)
                        .expect("the transfer transcript maps")
                        .fields
                };
                for (index, chunk) in record.draws.as_chunks::<32>().0.iter().enumerate() {
                    let field_index = group_fields[index];
                    let scalar = pairing
                        .decode_scalar(DecodeScalarParams, chunk.as_slice())
                        .expect("a drawn scalar decodes")
                        .scalar;
                    let value = &fields.values[field_index];
                    let expected = match value {
                        CanonicalFieldValue::Bytes(bytes)
                            if bytes.len() == encoded_g1.bytes.as_ref().len() =>
                        {
                            let Ok(product) = pairing.mul_g1(
                                MulG1Params,
                                MulG1Payload {
                                    point: g1.point.clone(),
                                    scalar,
                                },
                            );
                            let Ok(encoded) = pairing.encode_g1(
                                EncodeG1Params,
                                EncodeG1Payload {
                                    point: product.product,
                                },
                            );
                            encoded.bytes.as_ref().to_vec()
                        }
                        CanonicalFieldValue::Bytes(bytes)
                            if bytes.len() == encoded_g2.bytes.as_ref().len() =>
                        {
                            let Ok(product) = pairing.mul_g2(
                                MulG2Params,
                                MulG2Payload {
                                    point: g2.point.clone(),
                                    scalar,
                                },
                            );
                            let Ok(encoded) = pairing.encode_g2(
                                EncodeG2Params,
                                EncodeG2Payload {
                                    point: product.product,
                                },
                            );
                            encoded.bytes.as_ref().to_vec()
                        }
                        _ => panic!(
                            "a transcript group field is a bytes value of a group encoding's width"
                        ),
                    };
                    assert_eq!(*value, CanonicalFieldValue::Bytes(expected));
                }
            }
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    assert!(
                        run_challenge_vectors_case(
                            &build_challenge_vectors_case_deps(Consumer, Default::default()),
                            case_params(
                                *pairing_concrete,
                                hash_selection,
                                encoding_selection,
                                *forms_concrete,
                            ),
                            case_payload(
                                build_canonical_delivery_context(Default::default()),
                                DELIVERY_PURPOSE_MINT,
                                build_canonical_delivery_context(Default::default()),
                                DELIVERY_PURPOSE_TRANSFER,
                            ),
                        )
                        .is_ok()
                    );
                }
            }
        }
    }
}
// challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar

/// Contract: `challenge_vectors` → each record's `challenge` is the 32-byte
///   encoding of the scalar the sampling bound returns for the oracle's
///   uniform bytes over the declared tag and the record's `encoding`.
/// Arrange: `count` of `2`; a `challenge_tag` distinct from every default tag;
///   the contexts at their default values with `DELIVERY_PURPOSE_MINT` and
///   `DELIVERY_PURPOSE_TRANSFER`; the hash-to-scalar selections that declare
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES` and each declared selection.
/// Assert: the result is `Ok`; for each record, the tag from `build_domain_tag`
///   over the declared bytes, `expected_uniform_bytes` over the tag and the
///   record's `encoding` through `build_secret` into
///   `sample_from_uniform_bytes`, and `encode_scalar` of the sampled scalar,
///   read through `Secret::expose`, equal `record.challenge`.
/// Boundary: `challenge_vectors`, the encoder's `encode`, the adapter's
///   `hash_to_scalar`, `DomainTag::try_new`, `DomainTag::as_bytes`, and the
///   pairing's `sample_from_uniform_bytes` and `encode_scalar`.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to() {
    struct Consumer;

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            // Arrange
            let mint = build_delivery_context::<F, MintPurpose>(
                payload.mint_purpose,
                delivery_context_overrides(&payload.mint),
            );
            let transfer = build_delivery_context::<F, TransferPurpose>(
                payload.transfer_purpose,
                delivery_context_overrides(&payload.transfer),
            );

            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    Default::default(),
                ),
            );

            // Assert
            let records = result.expect("the generation succeeds").records;
            assert_eq!(records.len(), 4);
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(payload.delivery_proof.challenge_tag.to_vec()),
            });
            for record in &records {
                let uniform = expected_uniform_bytes(
                    &tag,
                    &record.encoding,
                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                );
                let scalar = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                        uniform: Some(build_secret(uniform, Default::default())),
                    }),
                )
                .expect("the reference scalar samples")
                .scalar
                .expose()
                .clone();
                let encoded = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        build_encode_scalar_payload(
                            pairing,
                            EncodeScalarPayloadOverrides {
                                scalar: Some(scalar),
                            },
                        ),
                    )
                    .expect("the reference scalar encodes");
                assert_eq!(record.challenge.as_slice(), encoded.bytes.expose().as_ref());
            }
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    assert!(
                        run_challenge_vectors_case(
                            &build_challenge_vectors_case_deps(Consumer, Default::default()),
                            case_params(
                                *pairing_concrete,
                                hash_selection,
                                encoding_selection,
                                *forms_concrete,
                            ),
                            case_payload(
                                build_canonical_delivery_context(Default::default()),
                                DELIVERY_PURPOSE_MINT,
                                build_canonical_delivery_context(Default::default()),
                                DELIVERY_PURPOSE_TRANSFER,
                            ),
                        )
                        .is_ok()
                    );
                }
            }
        }
    }
}
// challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to

/// Contract: `challenge_vectors` under a hash-to-scalar adapter that refuses
///   the sampling → `Err(ChallengeVectorsErrorReturn::HashToScalar(error))`,
///   the adapter's error unchanged.
/// Arrange: `count` of `1`; the contexts at their default values with
///   `DELIVERY_PURPOSE_MINT` and `DELIVERY_PURPOSE_TRANSFER`; the case params'
///   `hash_to_scalar` selection `build_create_hash_to_scalar_params` with
///   `concrete` `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`, in place of the
///   declared selections.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES`, each encoding selection, and each forms
///   concrete.
/// Assert: `assert_eq!` of the result against `Err(ChallengeVectorsErrorReturn
///   ::HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
///   actual: 0 })))`.
/// Boundary: `challenge_vectors` over the real pairing, encoder, forms, and
///   generator, to the hash-to-scalar adapter.
/// Mocked:  the hash-to-scalar family's mock concrete under `SamplingRefused`.
#[test]
fn challenge_vectors_return_the_hash_to_scalar_refusal_whole() {
    struct Consumer;

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            // Arrange
            let mint = build_delivery_context::<F, MintPurpose>(
                payload.mint_purpose,
                delivery_context_overrides(&payload.mint),
            );
            let transfer = build_delivery_context::<F, TransferPurpose>(
                payload.transfer_purpose,
                delivery_context_overrides(&payload.transfer),
            );

            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                result,
                Err(ChallengeVectorsErrorReturn::HashToScalar(
                    HashToScalarErrorReturn::Sampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            actual: 0
                        }
                    )
                ))
            );
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_selection in DECLARED_ENCODING_SELECTIONS {
            for forms_concrete in CHAIN_FORMS_CONCRETES {
                assert!(
                    run_challenge_vectors_case(
                        &build_challenge_vectors_case_deps(Consumer, Default::default()),
                        case_params(
                            *pairing_concrete,
                            &build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(HashToScalarConcrete::Mock(
                                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                                    )),
                                    ..Default::default()
                                },
                            ),
                            encoding_selection,
                            *forms_concrete,
                        ),
                        case_payload(
                            build_canonical_delivery_context(Default::default()),
                            DELIVERY_PURPOSE_MINT,
                            build_canonical_delivery_context(Default::default()),
                            DELIVERY_PURPOSE_TRANSFER,
                        ),
                    )
                    .is_ok()
                );
            }
        }
    }
}
// challenge_vectors_return_the_hash_to_scalar_refusal_whole
