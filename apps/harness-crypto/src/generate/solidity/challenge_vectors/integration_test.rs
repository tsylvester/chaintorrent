#![allow(clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{AssetIdentityHash, AssetIdentityHashConstructorParams, Secret, build_secret};
use encoding::{
    CanonicalFieldValue, CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, DecodeParams,
    ICanonicalField, IDecoderAdapter, IEncoderAdapter, IEncodingContract, ToFieldsParams,
    build_create_encoding_params, build_from_field_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params, build_domain_tag, expected_uniform_bytes,
};
use pairing::{
    CreatePairingParamsOverrides, DecodeScalarParams, EncodeG1Params, EncodeG1Payload,
    EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    PAIRING_CONCRETES, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_REPLACEMENT, DeliveryContext,
    DeliveryContextOverrides, DeliveryProofDeclarationOverrides, MintPurpose,
    MintStatementDescription, MintStatementDescriptionConstructorParams, TransferPurpose,
    TransferStatementDescription, TransferStatementDescriptionConstructorParams,
    build_delivery_context, build_delivery_proof_declaration,
};
use random::IRandomSourceAdapter;

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::challenge_vectors;
use super::interface::{
    CanonicalDeliveryContext, ChallengeVectorsCasePayload, ChallengeVectorsErrorReturn,
    ChallengeVectorsParams, IChallengeVectorsCaseConsumer,
};
use super::mock::{
    CanonicalDeliveryContextOverrides, ChallengeVectorsCaseParamsOverrides,
    ChallengeVectorsCasePayloadOverrides, ChallengeVectorsDepsOverrides,
    ChallengeVectorsParamsOverrides, ChallengeVectorsPayloadOverrides,
    build_canonical_delivery_context, build_challenge_vectors_case_deps,
    build_challenge_vectors_case_params, build_challenge_vectors_case_payload,
    build_challenge_vectors_deps, build_challenge_vectors_params, build_challenge_vectors_payload,
    run_challenge_vectors_case,
};

const CHALLENGE_TAG: &[u8] = b"challenge vectors integration test domain tag";

fn typed_context<F: IChainForms, Purpose>(
    context: &CanonicalDeliveryContext,
    purpose: Purpose,
) -> DeliveryContext<F, Purpose> {
    build_delivery_context::<F, Purpose>(
        purpose,
        DeliveryContextOverrides::<F> {
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
            asset_identity_hash: Some(
                AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                    bytes: *context.asset_root.as_bytes(),
                })
                .expect("a root's bytes admit an asset identity hash"),
            ),
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
            ..Default::default()
        },
    )
}

/// Contract: entry `challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order`;
///   for a count above one the records are, per index, a mint record then a
///   transfer record, each `purpose` the code of its context's purpose, each
///   `draws` one 32-byte scalar encoding per transcript point, and no two
///   records share `draws`.
/// Arrange: `count` of `2`; the mint and transfer contexts at their default
///   values with the purposes `DELIVERY_PURPOSE_REPLACEMENT` and
///   `DELIVERY_PURPOSE_GRANT`, whose codes are not those of
///   `DELIVERY_PURPOSE_MINT` and `DELIVERY_PURPOSE_TRANSFER`.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES`, `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   `DECLARED_ENCODING_SELECTIONS`, and `CHAIN_FORMS_CONCRETES`.
/// Assert: the result is `Ok`; `records` holds four records whose `purpose`
///   values are `4`, `3`, `4`, and `3` in that order and whose `draws` hold
///   `352`, `576`, `352`, and `576` bytes; no two records' `draws` are equal.
/// Boundary: `challenge_vectors` over the real chain of the private surface.
/// Mocked: none; the operating-system generator runs real.
#[test]
fn challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

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
            P: IPairingArithmetic + IPairingReference + IPairingAdapter,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            let result = challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            );
            let records = result.expect("the function computes the records").records;
            assert_eq!(records.len(), 4);
            assert_eq!(records[0].purpose, 4);
            assert_eq!(records[1].purpose, 3);
            assert_eq!(records[2].purpose, 4);
            assert_eq!(records[3].purpose, 3);
            assert_eq!(records[0].draws.len(), 352);
            assert_eq!(records[1].draws.len(), 576);
            assert_eq!(records[2].draws.len(), 352);
            assert_eq!(records[3].draws.len(), 576);
            for first in 0..records.len() {
                for second in first + 1..records.len() {
                    assert_ne!(records[first].draws, records[second].draws);
                }
            }
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        }),
    };
    let case_payload = build_challenge_vectors_case_payload(ChallengeVectorsCasePayloadOverrides {
        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
        ..Default::default()
    });

    // Act & Assert
    for concrete in PAIRING_CONCRETES.iter() {
        for hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS.iter() {
            for encoding in DECLARED_ENCODING_SELECTIONS.iter() {
                for chain_forms in CHAIN_FORMS_CONCRETES.iter() {
                    let case_params =
                        build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            hash_to_scalar: Some(*hash_to_scalar),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(*chain_forms),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        });
                    let result = run_challenge_vectors_case(
                        &build_challenge_vectors_case_deps(
                            Consumer {
                                params: consumer.params,
                            },
                            Default::default(),
                        ),
                        case_params,
                        build_challenge_vectors_case_payload(
                            ChallengeVectorsCasePayloadOverrides {
                                delivery_proof: None,
                                mint: Some(case_payload.mint.clone()),
                                mint_purpose: Some(case_payload.mint_purpose),
                                transfer: Some(case_payload.transfer.clone()),
                                transfer_purpose: Some(case_payload.transfer_purpose),
                            },
                        ),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
// challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order

/// Contract: entry `challenge_vectors_place_each_contexts_configured_values_in_its_transcript`;
///   each record's encoding, decoded under its statement description, holds
///   its context's configured values at the canonical field indices.
/// Arrange: `count` of `1`, the case payload's purposes
///   `DELIVERY_PURPOSE_REPLACEMENT` and `DELIVERY_PURPOSE_GRANT` and its `mint`
///   and `transfer` the `CanonicalDeliveryContext` values below — the mint
///   context with `chain` `Unsigned256([0x21; 32])`, `entitlement_contract`
///   `FixedBytes20([0x31; 20])`, `source_entitlement` `Unsigned256([0x41; 32])`,
///   `target_entitlement` `Unsigned256([0x42; 32])`, `old_interval`
///   `Unsigned64(5)`, `new_interval` `Unsigned64(6)`, `seller`
///   `FixedBytes20([0x61; 20])`, `buyer` `FixedBytes20([0x62; 20])`, and
///   `expiry` `1_800_000_000`, and the transfer context with `chain`
///   `Unsigned256([0x21; 32])`, `entitlement_contract` `FixedBytes20([0x32; 20])`,
///   `source_entitlement` and `target_entitlement` each `Unsigned256([0x43; 32])`,
///   `old_interval` `Unsigned64(6)`, `new_interval` `Unsigned64(7)`, `seller`
///   `FixedBytes20([0x62; 20])`, `buyer` `FixedBytes20([0x63; 20])`, and
///   `expiry` `1_900_000_000`.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES`, `DECLARED_ENCODING_SELECTIONS`, and
///   `CHAIN_FORMS_CONCRETES`.
/// Assert: the result is `Ok`; the mint record's encoding decoded through
///   `IDecoderAdapter::decode` under `MintStatementDescription::<P, F>` and
///   read through `to_fields` holds at indices `2`, `3`, `6`, `7`, `8`, `9`,
///   `10`, `11`, `12`, and `19` the values `Unsigned256([0x21; 32])`,
///   `FixedBytes20([0x31; 20])`, `Unsigned256([0x41; 32])`,
///   `Unsigned256([0x42; 32])`, `Unsigned64(5)`, `Unsigned64(6)`,
///   `Unsigned16(4)`, `FixedBytes20([0x61; 20])`, `FixedBytes20([0x62; 20])`,
///   and `Unsigned64(1_800_000_000)`; the transfer record's encoding decoded
///   under `TransferStatementDescription::<P, F>` holds at indices `2`, `3`,
///   `6`, `7`, `8`, `9`, `10`, `11`, `12`, and `25` the values
///   `Unsigned256([0x21; 32])`, `FixedBytes20([0x32; 20])`,
///   `Unsigned256([0x43; 32])` twice, `Unsigned64(6)`, `Unsigned64(7)`,
///   `Unsigned16(3)`, `FixedBytes20([0x62; 20])`, `FixedBytes20([0x63; 20])`,
///   and `Unsigned64(1_900_000_000)`.
/// Boundary: `challenge_vectors`, the forms' `from_field` in the arrangement
///   and `to_field`, the description's `to_fields`, and the encoder's `encode`
///   and `decode`.
/// Mocked: none; the operating-system generator runs real.
#[test]
fn challenge_vectors_place_each_contexts_configured_values_in_its_transcript() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

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
            P: IPairingArithmetic + IPairingReference + IPairingAdapter,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            let result = challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            );
            let records = result.expect("the function computes the records").records;
            assert_eq!(records.len(), 2);
            let Ok(mint_description) = MintStatementDescription::<P, F>::try_new(
                MintStatementDescriptionConstructorParams { pairing },
            );
            let mint_transcript = encoder
                .decode(
                    DecodeParams {
                        description: &mint_description,
                    },
                    &records[0].encoding,
                )
                .expect("the encoder decodes what it encoded")
                .described;
            let mint_fields = mint_description
                .to_fields(ToFieldsParams, &mint_transcript)
                .expect("the description maps the transcript")
                .fields;
            assert_eq!(
                mint_fields.values[2],
                CanonicalFieldValue::Unsigned256([0x21; 32])
            );
            assert_eq!(
                mint_fields.values[3],
                CanonicalFieldValue::FixedBytes20([0x31; 20])
            );
            assert_eq!(
                mint_fields.values[6],
                CanonicalFieldValue::Unsigned256([0x41; 32])
            );
            assert_eq!(
                mint_fields.values[7],
                CanonicalFieldValue::Unsigned256([0x42; 32])
            );
            assert_eq!(mint_fields.values[8], CanonicalFieldValue::Unsigned64(5));
            assert_eq!(mint_fields.values[9], CanonicalFieldValue::Unsigned64(6));
            assert_eq!(mint_fields.values[10], CanonicalFieldValue::Unsigned16(4));
            assert_eq!(
                mint_fields.values[11],
                CanonicalFieldValue::FixedBytes20([0x61; 20])
            );
            assert_eq!(
                mint_fields.values[12],
                CanonicalFieldValue::FixedBytes20([0x62; 20])
            );
            assert_eq!(
                mint_fields.values[19],
                CanonicalFieldValue::Unsigned64(1_800_000_000)
            );
            let Ok(transfer_description) = TransferStatementDescription::<P, F>::try_new(
                TransferStatementDescriptionConstructorParams { pairing },
            );
            let transfer_transcript = encoder
                .decode(
                    DecodeParams {
                        description: &transfer_description,
                    },
                    &records[1].encoding,
                )
                .expect("the encoder decodes what it encoded")
                .described;
            let transfer_fields = transfer_description
                .to_fields(ToFieldsParams, &transfer_transcript)
                .expect("the description maps the transcript")
                .fields;
            assert_eq!(
                transfer_fields.values[2],
                CanonicalFieldValue::Unsigned256([0x21; 32])
            );
            assert_eq!(
                transfer_fields.values[3],
                CanonicalFieldValue::FixedBytes20([0x32; 20])
            );
            assert_eq!(
                transfer_fields.values[6],
                CanonicalFieldValue::Unsigned256([0x43; 32])
            );
            assert_eq!(
                transfer_fields.values[7],
                CanonicalFieldValue::Unsigned256([0x43; 32])
            );
            assert_eq!(
                transfer_fields.values[8],
                CanonicalFieldValue::Unsigned64(6)
            );
            assert_eq!(
                transfer_fields.values[9],
                CanonicalFieldValue::Unsigned64(7)
            );
            assert_eq!(
                transfer_fields.values[10],
                CanonicalFieldValue::Unsigned16(3)
            );
            assert_eq!(
                transfer_fields.values[11],
                CanonicalFieldValue::FixedBytes20([0x62; 20])
            );
            assert_eq!(
                transfer_fields.values[12],
                CanonicalFieldValue::FixedBytes20([0x63; 20])
            );
            assert_eq!(
                transfer_fields.values[25],
                CanonicalFieldValue::Unsigned64(1_900_000_000)
            );
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(1),
            })),
            ..Default::default()
        }),
    };
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned256([0x21; 32])),
        entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
        source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x41; 32])),
        target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x42; 32])),
        old_interval: Some(CanonicalFieldValue::Unsigned64(5)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(6)),
        seller: Some(CanonicalFieldValue::FixedBytes20([0x61; 20])),
        buyer: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
        expiry: Some(1_800_000_000),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned256([0x21; 32])),
        entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x32; 20])),
        source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x43; 32])),
        target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x43; 32])),
        old_interval: Some(CanonicalFieldValue::Unsigned64(6)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(7)),
        seller: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
        buyer: Some(CanonicalFieldValue::FixedBytes20([0x63; 20])),
        expiry: Some(1_900_000_000),
        ..Default::default()
    });

    // Act & Assert
    for concrete in PAIRING_CONCRETES.iter() {
        for encoding in DECLARED_ENCODING_SELECTIONS.iter() {
            for chain_forms in CHAIN_FORMS_CONCRETES.iter() {
                let case_params =
                    build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        })),
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding.concrete),
                                identifier: Some(encoding.identifier),
                            },
                        )),
                        chain_forms: Some(build_create_chain_forms_params(
                            CreateChainFormsParamsOverrides {
                                concrete: Some(*chain_forms),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    });
                let result = run_challenge_vectors_case(
                    &build_challenge_vectors_case_deps(
                        Consumer {
                            params: consumer.params,
                        },
                        Default::default(),
                    ),
                    case_params,
                    build_challenge_vectors_case_payload(ChallengeVectorsCasePayloadOverrides {
                        mint: Some(mint.clone()),
                        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
                        transfer: Some(transfer.clone()),
                        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
                        ..Default::default()
                    }),
                );
                assert!(result.is_ok());
            }
        }
    }
}
// challenge_vectors_place_each_contexts_configured_values_in_its_transcript

/// Contract: entry `challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar`;
///   every transcript point is the generator of its group times the scalar the
///   record's `draws` hold at its position.
/// Arrange: `count` of `1`; the contexts at their default values with
///   `DELIVERY_PURPOSE_MINT` and `DELIVERY_PURPOSE_TRANSFER`; the group-field
///   indices `13` through `18` and `20` through `24` for the mint transcript
///   and `13` through `24` and `26` through `31` for the transfer transcript.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES`.
/// Assert: the result is `Ok`; for each record, `record.draws.len()` equals
///   `group_fields.len() * 32`, and each chunk of `record.draws.as_chunks::<32>().0`
///   in order, passed to `decode_scalar` as `chunk.as_slice()`, decodes to a
///   scalar; the generator of the group whose encoding has the length of the
///   transcript value at the chunk's index, multiplied by that scalar and
///   encoded, equals `CanonicalFieldValue::Bytes` of the decoded transcript's
///   value at that index.
/// Boundary: `challenge_vectors`, the pairing's `decode_scalar`,
///   `g1_generator`, `g2_generator`, `mul_g1`, `mul_g2`, `encode_g1`, and
///   `encode_g2`, and the encoder's `decode`.
/// Mocked: none; the operating-system generator runs real.
#[test]
fn challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

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
            P: IPairingArithmetic + IPairingReference + IPairingAdapter,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            let result = challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            );
            let records = result.expect("the function computes the records").records;
            assert_eq!(records.len(), 2);
            let Ok(g1_generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2_generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let g1_length = pairing
                .encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: g1_generator.point.clone(),
                    },
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len();
            let mint_indices: &[usize] = &[13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24];
            let transfer_indices: &[usize] = &[
                13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 31,
            ];
            for (record, indices) in records.iter().zip([mint_indices, transfer_indices]) {
                assert_eq!(record.draws.len(), indices.len() * 32);
                let description_is_mint = indices.len() == 11;
                let values = if description_is_mint {
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
                        .expect("the encoder decodes what it encoded")
                        .described;
                    description
                        .to_fields(ToFieldsParams, &described)
                        .expect("the description maps the transcript")
                        .fields
                        .values
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
                        .expect("the encoder decodes what it encoded")
                        .described;
                    description
                        .to_fields(ToFieldsParams, &described)
                        .expect("the description maps the transcript")
                        .fields
                        .values
                };
                for (position, chunk) in record.draws.as_chunks::<32>().0.iter().enumerate() {
                    let index = indices[position];
                    let scalar = pairing
                        .decode_scalar(DecodeScalarParams, chunk.as_slice())
                        .expect("each recorded scalar decodes")
                        .scalar;
                    let CanonicalFieldValue::Bytes(point_bytes) = &values[index] else {
                        panic!("the transcript's group fields are byte encodings");
                    };
                    if point_bytes.len() == g1_length {
                        let product = pairing
                            .mul_g1(
                                MulG1Params,
                                MulG1Payload {
                                    point: g1_generator.point.clone(),
                                    scalar,
                                },
                            )
                            .expect("the generator multiplies")
                            .product;
                        let encoded = pairing
                            .encode_g1(EncodeG1Params, EncodeG1Payload { point: product })
                            .expect("the point encodes")
                            .bytes;
                        assert_eq!(encoded.as_ref(), point_bytes.as_slice());
                    } else {
                        let product = pairing
                            .mul_g2(
                                MulG2Params,
                                MulG2Payload {
                                    point: g2_generator.point.clone(),
                                    scalar,
                                },
                            )
                            .expect("the generator multiplies")
                            .product;
                        let encoded = pairing
                            .encode_g2(EncodeG2Params, EncodeG2Payload { point: product })
                            .expect("the point encodes")
                            .bytes;
                        assert_eq!(encoded.as_ref(), point_bytes.as_slice());
                    }
                }
            }
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(1),
            })),
            ..Default::default()
        }),
    };

    // Act & Assert
    for concrete in PAIRING_CONCRETES.iter() {
        for encoding in DECLARED_ENCODING_SELECTIONS.iter() {
            for chain_forms in CHAIN_FORMS_CONCRETES.iter() {
                let case_params =
                    build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        })),
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding.concrete),
                                identifier: Some(encoding.identifier),
                            },
                        )),
                        chain_forms: Some(build_create_chain_forms_params(
                            CreateChainFormsParamsOverrides {
                                concrete: Some(*chain_forms),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    });
                let result = run_challenge_vectors_case(
                    &build_challenge_vectors_case_deps(
                        Consumer {
                            params: consumer.params,
                        },
                        Default::default(),
                    ),
                    case_params,
                    build_challenge_vectors_case_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}
// challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar

/// Contract: entry `challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to`;
///   each record's `challenge` is the 32-byte encoding of the scalar the
///   sampling bound returns for the oracle's uniform bytes over the declared
///   tag and the record's `encoding`.
/// Arrange: `count` of `2`; a `challenge_tag` distinct from every default tag;
///   the contexts at their default values with `DELIVERY_PURPOSE_MINT` and
///   `DELIVERY_PURPOSE_TRANSFER`; the hash-to-scalar selections that declare
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES` and each Keccak256V1 selection.
/// Assert: the result is `Ok`; for each record, `expected_uniform_bytes` over
///   the declared tag and the record's `encoding`, run through
///   `sample_from_uniform_bytes` and `encode_scalar`, equals `record.challenge`.
/// Boundary: `challenge_vectors`, the encoder's `encode`, the adapter's
///   `hash_to_scalar`, `DomainTag::try_new`, `DomainTag::as_bytes`, and the
///   pairing's `sample_from_uniform_bytes` and `encode_scalar`.
/// Mocked: none; the operating-system generator runs real.
#[test]
fn challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

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
            P: IPairingArithmetic + IPairingReference + IPairingAdapter,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            let result = challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            );
            let records = result.expect("the function computes the records").records;
            assert_eq!(records.len(), 4);
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(CHALLENGE_TAG.to_vec()),
            });
            for record in &records {
                let uniform: Secret<Vec<u8>> = build_secret(
                    expected_uniform_bytes(
                        &tag,
                        &record.encoding,
                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    ),
                    Default::default(),
                );
                let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload { uniform },
                )
                .expect("the oracle's uniform bytes sample")
                .scalar;
                let encoded = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: sampled.expose().clone(),
                        },
                    )
                    .expect("the scalar encodes")
                    .bytes;
                assert_eq!(record.challenge.as_slice(), encoded.expose().as_ref());
            }
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        }),
    };
    let case_payload = build_challenge_vectors_case_payload(ChallengeVectorsCasePayloadOverrides {
        delivery_proof: Some(build_delivery_proof_declaration(
            DeliveryProofDeclarationOverrides {
                challenge_tag: Some(CHALLENGE_TAG),
                ..Default::default()
            },
        )),
        ..Default::default()
    });

    // Act & Assert
    for concrete in PAIRING_CONCRETES.iter() {
        for hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS.iter() {
            if hash_to_scalar.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding in DECLARED_ENCODING_SELECTIONS.iter() {
                for chain_forms in CHAIN_FORMS_CONCRETES.iter() {
                    let case_params =
                        build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            hash_to_scalar: Some(*hash_to_scalar),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(*chain_forms),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        });
                    let result = run_challenge_vectors_case(
                        &build_challenge_vectors_case_deps(
                            Consumer {
                                params: consumer.params,
                            },
                            Default::default(),
                        ),
                        case_params,
                        build_challenge_vectors_case_payload(
                            ChallengeVectorsCasePayloadOverrides {
                                delivery_proof: Some(build_delivery_proof_declaration(
                                    DeliveryProofDeclarationOverrides {
                                        challenge_tag: Some(CHALLENGE_TAG),
                                        ..Default::default()
                                    },
                                )),
                                mint: Some(case_payload.mint.clone()),
                                mint_purpose: Some(case_payload.mint_purpose),
                                transfer: Some(case_payload.transfer.clone()),
                                transfer_purpose: Some(case_payload.transfer_purpose),
                            },
                        ),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
// challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to

/// Contract: entry `challenge_vectors_return_the_hash_to_scalar_refusal_whole`;
///   the hash-to-scalar adapter's refusal reaches the caller whole.
/// Arrange: `count` of `1`; the contexts at their default values with
///   `DELIVERY_PURPOSE_MINT` and `DELIVERY_PURPOSE_TRANSFER`; the case params'
///   `hash_to_scalar` selection `build_create_hash_to_scalar_params` with
///   `concrete` `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`, in place of the declared selections.
/// Act:    the case consumer's single call to `challenge_vectors`, once per
///   member of `PAIRING_CONCRETES`.
/// Assert: `assert_eq!` of the result against
///   `Err(ChallengeVectorsErrorReturn::HashToScalar(HashToScalarErrorReturn::
///   Sampling(SampleUniformScalarErrorReturn::WrongLength {
///   expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
///   actual: 0 })))`.
/// Boundary: `challenge_vectors` over the real pairing, encoder, forms, and
///   generator, to the hash-to-scalar adapter.
/// Mocked: the hash-to-scalar family's mock concrete under `SamplingRefused`.
#[test]
fn challenge_vectors_return_the_hash_to_scalar_refusal_whole() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

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
            P: IPairingArithmetic + IPairingReference + IPairingAdapter,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            let result = challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            );
            assert_eq!(
                result,
                Err(ChallengeVectorsErrorReturn::HashToScalar(
                    HashToScalarErrorReturn::Sampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            actual: 0,
                        }
                    )
                ))
            );
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(1),
            })),
            ..Default::default()
        }),
    };

    // Act & Assert
    for concrete in PAIRING_CONCRETES.iter() {
        for encoding in DECLARED_ENCODING_SELECTIONS.iter() {
            for chain_forms in CHAIN_FORMS_CONCRETES.iter() {
                let case_params =
                    build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*concrete),
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
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding.concrete),
                                identifier: Some(encoding.identifier),
                            },
                        )),
                        chain_forms: Some(build_create_chain_forms_params(
                            CreateChainFormsParamsOverrides {
                                concrete: Some(*chain_forms),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    });
                let result = run_challenge_vectors_case(
                    &build_challenge_vectors_case_deps(
                        Consumer {
                            params: consumer.params,
                        },
                        Default::default(),
                    ),
                    case_params,
                    build_challenge_vectors_case_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}
// challenge_vectors_return_the_hash_to_scalar_refusal_whole
