#![allow(clippy::expect_used, clippy::panic)]

use super::challenge_vectors;
use super::interface::*;
use super::mock::{
    CanonicalDeliveryContextOverrides, ChallengeVectorsParamsOverrides,
    ChallengeVectorsPayloadOverrides, build_canonical_delivery_context,
    build_challenge_vectors_deps, build_challenge_vectors_params, build_challenge_vectors_payload,
};

use super::super::group_vectors::provides::{
    VectorCount, VectorCountConstructorParamsOverrides, build_vector_count,
};
use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::build_secret;
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, DecodeParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, IEncodingContract, ToFieldsParams,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, build_domain_tag, create_hash_to_scalar,
    expected_uniform_bytes,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeScalarParams, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::{
    DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_REPLACEMENT, DeliveryProofDeclaration,
    DeliveryProofDeclarationOverrides, MintStatementDescription,
    MintStatementDescriptionConstructorParams, TransferStatementDescription,
    TransferStatementDescriptionConstructorParams, build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

/// Contract: entry
///   `challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order`;
///   for a count above one the records are, per index, a mint record then a
///   transfer record, each `purpose` the purpose's code, each `draws` one
///   32-byte scalar encoding per transcript point, and no two records share
///   `draws`.
/// Arrange: `count` of `2`; the purposes `DELIVERY_PURPOSE_REPLACEMENT` and
///   `DELIVERY_PURPOSE_GRANT`, whose codes are not the builder defaults'; the
///   default contexts.
/// Act:     the forms consumer's single call to `challenge_vectors`.
/// Assert:  the result is `Ok`; `records` holds four records whose `purpose`
///   values are `4`, `3`, `4`, and `3` in that order and whose `draws` hold
///   `352`, `576`, `352`, and `576` bytes; no two records' `draws` are equal.
/// Boundary: `challenge_vectors` over the real chain of the private surface.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order() {
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
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });
    let declaration = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"ChainTorrent-v1-challenge-vectors"),
        ..Default::default()
    });
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());

    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
        pairing: &'a P,
        encoder: &'a E,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    self.pairing,
                    self.encoder,
                    self.hashing,
                    self.random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(self.count),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    self.declaration,
                    self.mint,
                    self.transfer,
                    ChallengeVectorsPayloadOverrides {
                        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
                        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(success.records.len(), 4);
            assert_eq!(success.records[0].purpose, 4);
            assert_eq!(success.records[1].purpose, 3);
            assert_eq!(success.records[2].purpose, 4);
            assert_eq!(success.records[3].purpose, 3);
            assert_eq!(success.records[0].draws.len(), 352);
            assert_eq!(success.records[1].draws.len(), 576);
            assert_eq!(success.records[2].draws.len(), 352);
            assert_eq!(success.records[3].draws.len(), 576);
            for (index, record) in success.records.iter().enumerate() {
                for other in success.records.iter().skip(index + 1) {
                    assert_ne!(record.draws, other.draws);
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let Ok(_) = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                            hashing: self.hashing,
                            random: self.random,
                            count: self.count,
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                ) else {
                    panic!("the declared chain-forms concrete is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
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
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
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
        declaration: &declaration,
        mint: &mint,
        transfer: &transfer,
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
    // challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order
}

/// Contract: entry
///   `challenge_vectors_place_each_contexts_configured_values_in_its_transcript`;
///   each record's encoding, decoded under its statement description, holds
///   its context's configured values at the canonical field indices.
/// Arrange: `count` of `1`, the purposes `DELIVERY_PURPOSE_REPLACEMENT` and
///   `DELIVERY_PURPOSE_GRANT`, the mint context with `chain`
///   `Unsigned256([0x21; 32])`, `entitlement_contract`
///   `FixedBytes20([0x31; 20])`, `source_entitlement`
///   `Unsigned256([0x41; 32])`, `target_entitlement`
///   `Unsigned256([0x42; 32])`, `old_interval` `Unsigned64(5)`,
///   `new_interval` `Unsigned64(6)`, `seller` `FixedBytes20([0x61; 20])`,
///   `buyer` `FixedBytes20([0x62; 20])`, and `expiry` `1_800_000_000`, and the
///   transfer context with `chain` `Unsigned256([0x21; 32])`,
///   `entitlement_contract` `FixedBytes20([0x32; 20])`, `source_entitlement`
///   and `target_entitlement` each `Unsigned256([0x43; 32])`, `old_interval`
///   `Unsigned64(6)`, `new_interval` `Unsigned64(7)`, `seller`
///   `FixedBytes20([0x62; 20])`, `buyer` `FixedBytes20([0x63; 20])`, and
///   `expiry` `1_900_000_000`, each value a `CanonicalFieldValue`.
/// Act:     the forms consumer's single call to `challenge_vectors`.
/// Assert:  the result is `Ok`; the mint record's encoding decoded through
///   `IDecoderAdapter::decode` under `MintStatementDescription::<P,
///   F>::try_new` and read through `to_fields` holds at indices `2`, `3`,
///   `6`, `7`, `8`, `9`, `10`, `11`, `12`, and `19` the values
///   `Unsigned256([0x21; 32])`, `FixedBytes20([0x31; 20])`,
///   `Unsigned256([0x41; 32])`, `Unsigned256([0x42; 32])`, `Unsigned64(5)`,
///   `Unsigned64(6)`, `Unsigned16(4)`, `FixedBytes20([0x61; 20])`,
///   `FixedBytes20([0x62; 20])`, and `Unsigned64(1_800_000_000)`; the
///   transfer record's encoding decoded under
///   `TransferStatementDescription::<P, F>::try_new` holds at indices `2`,
///   `3`, `6`, `7`, `8`, `9`, `10`, `11`, `12`, and `25` the values
///   `Unsigned256([0x21; 32])`, `FixedBytes20([0x32; 20])`,
///   `Unsigned256([0x43; 32])` twice, `Unsigned64(6)`, `Unsigned64(7)`,
///   `Unsigned16(3)`, `FixedBytes20([0x62; 20])`,
///   `FixedBytes20([0x63; 20])`, and `Unsigned64(1_900_000_000)`, each
///   literal written in the block.
/// Boundary: `challenge_vectors`, the forms' `from_field` and `to_field`, the
///   description's `to_fields`, and the encoder's `encode` and `decode`.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_place_each_contexts_configured_values_in_its_transcript() {
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
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let declaration = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"ChainTorrent-v1-challenge-vectors"),
        ..Default::default()
    });
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

    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        encoder: &'a E,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter, E: IEncoderAdapter + IDecoderAdapter> IChainFormsConsumer
        for FormsConsumer<'_, P, E>
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    self.pairing,
                    self.encoder,
                    self.hashing,
                    self.random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(self.count),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    self.declaration,
                    self.mint,
                    self.transfer,
                    ChallengeVectorsPayloadOverrides {
                        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
                        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(success.records.len(), 2);
            let Ok(mint_description) = MintStatementDescription::<P, F>::try_new(
                MintStatementDescriptionConstructorParams {
                    pairing: self.pairing,
                },
            );
            let mint_statement = self
                .encoder
                .decode(
                    DecodeParams {
                        description: &mint_description,
                    },
                    &success.records[0].encoding,
                )
                .expect("the mint record's encoding decodes under its description");
            let Ok(mint_fields) =
                mint_description.to_fields(ToFieldsParams, &mint_statement.described);
            assert_eq!(
                mint_fields.fields.values[2],
                CanonicalFieldValue::Unsigned256([0x21; 32])
            );
            assert_eq!(
                mint_fields.fields.values[3],
                CanonicalFieldValue::FixedBytes20([0x31; 20])
            );
            assert_eq!(
                mint_fields.fields.values[6],
                CanonicalFieldValue::Unsigned256([0x41; 32])
            );
            assert_eq!(
                mint_fields.fields.values[7],
                CanonicalFieldValue::Unsigned256([0x42; 32])
            );
            assert_eq!(
                mint_fields.fields.values[8],
                CanonicalFieldValue::Unsigned64(5)
            );
            assert_eq!(
                mint_fields.fields.values[9],
                CanonicalFieldValue::Unsigned64(6)
            );
            assert_eq!(
                mint_fields.fields.values[10],
                CanonicalFieldValue::Unsigned16(4)
            );
            assert_eq!(
                mint_fields.fields.values[11],
                CanonicalFieldValue::FixedBytes20([0x61; 20])
            );
            assert_eq!(
                mint_fields.fields.values[12],
                CanonicalFieldValue::FixedBytes20([0x62; 20])
            );
            assert_eq!(
                mint_fields.fields.values[19],
                CanonicalFieldValue::Unsigned64(1_800_000_000)
            );
            let Ok(transfer_description) = TransferStatementDescription::<P, F>::try_new(
                TransferStatementDescriptionConstructorParams {
                    pairing: self.pairing,
                },
            );
            let transfer_statement = self
                .encoder
                .decode(
                    DecodeParams {
                        description: &transfer_description,
                    },
                    &success.records[1].encoding,
                )
                .expect("the transfer record's encoding decodes under its description");
            let Ok(transfer_fields) =
                transfer_description.to_fields(ToFieldsParams, &transfer_statement.described);
            assert_eq!(
                transfer_fields.fields.values[2],
                CanonicalFieldValue::Unsigned256([0x21; 32])
            );
            assert_eq!(
                transfer_fields.fields.values[3],
                CanonicalFieldValue::FixedBytes20([0x32; 20])
            );
            assert_eq!(
                transfer_fields.fields.values[6],
                CanonicalFieldValue::Unsigned256([0x43; 32])
            );
            assert_eq!(
                transfer_fields.fields.values[7],
                CanonicalFieldValue::Unsigned256([0x43; 32])
            );
            assert_eq!(
                transfer_fields.fields.values[8],
                CanonicalFieldValue::Unsigned64(6)
            );
            assert_eq!(
                transfer_fields.fields.values[9],
                CanonicalFieldValue::Unsigned64(7)
            );
            assert_eq!(
                transfer_fields.fields.values[10],
                CanonicalFieldValue::Unsigned16(3)
            );
            assert_eq!(
                transfer_fields.fields.values[11],
                CanonicalFieldValue::FixedBytes20([0x62; 20])
            );
            assert_eq!(
                transfer_fields.fields.values[12],
                CanonicalFieldValue::FixedBytes20([0x63; 20])
            );
            assert_eq!(
                transfer_fields.fields.values[25],
                CanonicalFieldValue::Unsigned64(1_900_000_000)
            );
        }
    }

    struct EncodingConsumer<'a, P: IPairingAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let Ok(_) = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                            hashing: self.hashing,
                            random: self.random,
                            count: self.count,
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                ) else {
                    panic!("the declared chain-forms concrete is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
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
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
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
        declaration: &declaration,
        mint: &mint,
        transfer: &transfer,
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
    // challenge_vectors_place_each_contexts_configured_values_in_its_transcript
}

/// Contract: entry
///   `challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar`;
///   every transcript point is the generator of its group times the scalar
///   the record's `draws` hold at its position.
/// Arrange: `count` of `1`; the default contexts and purposes; the
///   group-field indices written as literals in the block, `13` through `18`
///   and `20` through `24` for the mint transcript and `13` through `24` and
///   `26` through `31` for the transfer transcript, in the order the
///   descriptions list their group fields.
/// Act:     the forms consumer's single call to `challenge_vectors`.
/// Assert:  the result is `Ok`; for each record, each 32-byte chunk of
///   `draws` in order decodes through `decode_scalar` to a scalar; the
///   generator of the group whose encoding has the length of the transcript
///   value at the chunk's index, the first-group generator where the length
///   is `encode_g1` of `g1_generator`'s and the second-group generator where
///   it is `encode_g2` of `g2_generator`'s, multiplied by that scalar
///   through `mul_g1` or `mul_g2` and encoded through `encode_g1` or
///   `encode_g2`, equals `CanonicalFieldValue::Bytes` of the decoded
///   transcript's value at that index; the chunks and the indices pair one
///   to one.
/// Boundary: `challenge_vectors`, the pairing's `decode_scalar`,
///   `g1_generator`, `g2_generator`, `mul_g1`, `mul_g2`, `encode_g1`, and
///   `encode_g2`, and the encoder's `decode`.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar() {
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
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let declaration = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"ChainTorrent-v1-challenge-vectors"),
        ..Default::default()
    });
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let mint_group_fields: [usize; 11] = [13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24];
    let transfer_group_fields: [usize; 18] = [
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 31,
    ];

    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        encoder: &'a E,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        mint_group_fields: &'a [usize],
        transfer_group_fields: &'a [usize],
    }

    impl<P: IPairingAdapter, E: IEncoderAdapter + IDecoderAdapter> IChainFormsConsumer
        for FormsConsumer<'_, P, E>
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
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
                    point: g1_generator.point.clone(),
                },
            );
            let Ok(reference_g2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: g2_generator.point.clone(),
                },
            );
            let pairing = self.pairing;
            let assert_record = |record: &ChallengeVector,
                                 values: &[CanonicalFieldValue],
                                 group_fields: &[usize]| {
                assert_eq!(record.draws.len(), group_fields.len() * 32);
                for (chunk, field_index) in record
                    .draws
                    .chunks_exact(32)
                    .zip(group_fields.iter().copied())
                {
                    let CanonicalFieldValue::Bytes(point_bytes) = &values[field_index] else {
                        panic!("the group field is a byte encoding")
                    };
                    let decoded = pairing
                        .decode_scalar(DecodeScalarParams, chunk)
                        .expect("the recorded draw decodes as a scalar");
                    if point_bytes.len() == reference_g1.bytes.as_ref().len() {
                        let Ok(product) = pairing.mul_g1(
                            MulG1Params,
                            MulG1Payload {
                                point: g1_generator.point.clone(),
                                scalar: decoded.scalar,
                            },
                        );
                        let Ok(encoded) = pairing.encode_g1(
                            EncodeG1Params,
                            EncodeG1Payload {
                                point: product.product,
                            },
                        );
                        assert_eq!(
                            values[field_index],
                            CanonicalFieldValue::Bytes(encoded.bytes.as_ref().to_vec())
                        );
                    } else if point_bytes.len() == reference_g2.bytes.as_ref().len() {
                        let Ok(product) = pairing.mul_g2(
                            MulG2Params,
                            MulG2Payload {
                                point: g2_generator.point.clone(),
                                scalar: decoded.scalar,
                            },
                        );
                        let Ok(encoded) = pairing.encode_g2(
                            EncodeG2Params,
                            EncodeG2Payload {
                                point: product.product,
                            },
                        );
                        assert_eq!(
                            values[field_index],
                            CanonicalFieldValue::Bytes(encoded.bytes.as_ref().to_vec())
                        );
                    } else {
                        panic!("the group field has a group encoding width")
                    }
                }
            };

            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    self.pairing,
                    self.encoder,
                    self.hashing,
                    self.random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(self.count),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    self.declaration,
                    self.mint,
                    self.transfer,
                    ChallengeVectorsPayloadOverrides {
                        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
                        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(success.records.len(), 2);
            let Ok(mint_description) = MintStatementDescription::<P, F>::try_new(
                MintStatementDescriptionConstructorParams {
                    pairing: self.pairing,
                },
            );
            let mint_statement = self
                .encoder
                .decode(
                    DecodeParams {
                        description: &mint_description,
                    },
                    &success.records[0].encoding,
                )
                .expect("the mint record's encoding decodes under its description");
            let Ok(mint_fields) =
                mint_description.to_fields(ToFieldsParams, &mint_statement.described);
            assert_record(
                &success.records[0],
                &mint_fields.fields.values,
                self.mint_group_fields,
            );
            let Ok(transfer_description) = TransferStatementDescription::<P, F>::try_new(
                TransferStatementDescriptionConstructorParams {
                    pairing: self.pairing,
                },
            );
            let transfer_statement = self
                .encoder
                .decode(
                    DecodeParams {
                        description: &transfer_description,
                    },
                    &success.records[1].encoding,
                )
                .expect("the transfer record's encoding decodes under its description");
            let Ok(transfer_fields) =
                transfer_description.to_fields(ToFieldsParams, &transfer_statement.described);
            assert_record(
                &success.records[1],
                &transfer_fields.fields.values,
                self.transfer_group_fields,
            );
        }
    }

    struct EncodingConsumer<'a, P: IPairingAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        mint_group_fields: &'a [usize],
        transfer_group_fields: &'a [usize],
    }

    impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let Ok(_) = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                            hashing: self.hashing,
                            random: self.random,
                            count: self.count,
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
                            mint_group_fields: self.mint_group_fields,
                            transfer_group_fields: self.transfer_group_fields,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                ) else {
                    panic!("the declared chain-forms concrete is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        mint_group_fields: &'a [usize],
        transfer_group_fields: &'a [usize],
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
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
                            mint_group_fields: self.mint_group_fields,
                            transfer_group_fields: self.transfer_group_fields,
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
        declaration: &declaration,
        mint: &mint,
        transfer: &transfer,
        mint_group_fields: &mint_group_fields,
        transfer_group_fields: &transfer_group_fields,
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
    // challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar
}

/// Contract: entry
///   `challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to`;
///   each record's `challenge` is the 32-byte encoding of the scalar the
///   sampling bound returns for the oracle's uniform bytes over the declared
///   tag and the record's `encoding`.
/// Arrange: `count` of `2`; a `challenge_tag` distinct from every default
///   tag; the default contexts and purposes; the hash-to-scalar selections
///   that declare `HashToScalarIdentifier::Keccak256V1`.
/// Act:     the forms consumer's single call to `challenge_vectors`.
/// Assert:  the result is `Ok`; for each record, the tag from
///   `build_domain_tag` with `DomainTagConstructorParamsOverrides { bytes:
///   Some(...) }` over the declared bytes,
///   `expected_uniform_bytes(&tag, &record.encoding, <P::Scalar as
///   ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)` through `build_secret`
///   into `sample_from_uniform_bytes`, and `encode_scalar` of the sampled
///   scalar, read through `Secret::expose`, equal `record.challenge`, on
///   every member of `PAIRING_CONCRETES`, both curves included.
/// Boundary: `challenge_vectors`, the encoder's `encode`, the adapter's
///   `hash_to_scalar`, `DomainTag::try_new`, `DomainTag::as_bytes`, and the
///   pairing's `sample_from_uniform_bytes` and `encode_scalar`.
/// Mocked:  none; the operating-system generator runs real.
#[test]
fn challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to() {
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
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });
    let declaration = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"ChainTorrent-v1-challenge-vectors"),
        ..Default::default()
    });
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());

    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
        pairing: &'a P,
        encoder: &'a E,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    self.pairing,
                    self.encoder,
                    self.hashing,
                    self.random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(self.count),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    self.declaration,
                    self.mint,
                    self.transfer,
                    ChallengeVectorsPayloadOverrides {
                        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
                        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(success.records.len(), 4);
            for record in &success.records {
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(self.declaration.challenge_tag.to_vec()),
                });
                let uniform = expected_uniform_bytes(
                    &tag,
                    &record.encoding,
                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                );
                let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(uniform, Default::default()),
                    },
                )
                .expect("the oracle's uniform bytes admit a scalar");
                let Ok(encoded) = self.pairing.encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: sampled.scalar.expose().clone(),
                    },
                );
                let challenge = <[u8; 32]>::try_from(encoded.bytes.expose().as_ref())
                    .expect("the sampled scalar's encoding is 32 bytes");
                assert_eq!(record.challenge, challenge);
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let Ok(_) = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                            hashing: self.hashing,
                            random: self.random,
                            count: self.count,
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                ) else {
                    panic!("the declared chain-forms concrete is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for hashing_params in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
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
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
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
        declaration: &declaration,
        mint: &mint,
        transfer: &transfer,
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
    // challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to
}

/// Contract: entry
///   `challenge_vectors_return_the_hash_to_scalar_refusal_whole`; the
///   hash-to-scalar adapter's refusal reaches the caller whole.
/// Arrange: `count` of `1`; the default contexts and purposes; the
///   hash-to-scalar adapter from `build_create_hash_to_scalar_params` with
///   `concrete` `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`, in place of the
///   declared selections.
/// Act:     the forms consumer's single call to `challenge_vectors`.
/// Assert:  `assert_eq!` of the result against `Err(
///   ChallengeVectorsErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected: <P::Scalar as
///   ISampleUniformScalar>::UNIFORM_BYTES_LENGTH, actual: 0 })))`.
/// Boundary: `challenge_vectors` over the real pairing, encoder, forms, and
///   generator, to the hash-to-scalar adapter.
/// Mocked:  the hash-to-scalar family's mock concrete under
///   `SamplingRefused`.
#[test]
fn challenge_vectors_return_the_hash_to_scalar_refusal_whole() {
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
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let declaration = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"ChainTorrent-v1-challenge-vectors"),
        ..Default::default()
    });
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());

    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
        pairing: &'a P,
        encoder: &'a E,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let result = challenge_vectors::<P, E, F>(
                &build_challenge_vectors_deps(
                    self.pairing,
                    self.encoder,
                    self.hashing,
                    self.random,
                    Default::default(),
                ),
                build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
                    count: Some(self.count),
                    ..Default::default()
                }),
                build_challenge_vectors_payload(
                    self.declaration,
                    self.mint,
                    self.transfer,
                    ChallengeVectorsPayloadOverrides {
                        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
                        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
                    },
                ),
            );

            // Assert
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

    struct EncodingConsumer<'a, P: IPairingAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let Ok(_) = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                            hashing: self.hashing,
                            random: self.random,
                            count: self.count,
                            declaration: self.declaration,
                            mint: self.mint,
                            transfer: self.transfer,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                ) else {
                    panic!("the declared chain-forms concrete is created")
                };
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        declaration: &'a DeliveryProofDeclaration,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    identifier: Some(HashToScalarIdentifier::Mock),
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the mock hash-to-scalar adapter is created")
            };
            for encoding_params in DECLARED_ENCODING_SELECTIONS {
                let Ok(_) = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                        hashing: &*hashing.adapter,
                        random: self.random,
                        count: self.count,
                        declaration: self.declaration,
                        mint: self.mint,
                        transfer: self.transfer,
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

    let consumer = PairingConsumer {
        random: &*source,
        count,
        declaration: &declaration,
        mint: &mint,
        transfer: &transfer,
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
    // challenge_vectors_return_the_hash_to_scalar_refusal_whole
}
