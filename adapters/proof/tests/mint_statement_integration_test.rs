#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParamsOverrides, CreateEncodingPayload, DECLARED_ENCODING_SELECTIONS,
    DecodeParams, EncodeParams, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    IEncodingContract, build_create_encoding_deps, build_create_encoding_params,
    build_from_fields_params, build_to_fields_params, create_encoding,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeG1ErrorReturn, DecodeG2ErrorReturn, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides,
    G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload,
    G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_deps, build_create_pairing_params, build_encode_g1_payload,
    build_encode_g2_payload, create_pairing,
};
use proof::{
    MintStatementCorruptions, MintStatementDescription, MintStatementDescriptionConstructorParams,
    MintStatementFromFieldsErrorReturn, MintStatementOverrides, build_mint_statement,
    build_mint_statement_description_constructor_params, invalidate_mint_statement,
};
use std::cell::Cell;

/// Contract: entry `a_mint_statement_survives_the_encoding_round_trip`; given a
///   statement encoded under its description by the received encoder and the
///   bytes decoded under the same description, the decoded statement equals
///   the original field by field.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `create_encoding`, the encoding
///   concrete's constructor, `consume_encoding`, `create_chain_forms`,
///   `consume_chain_forms`, the description's `try_new`, `to_fields`, and
///   `fields_to_value`, the encoder's `encode` and `decode`, and the pairing's
///   `encode_g1`, `encode_g2`, `decode_g1`, and `decode_g2`.
/// Mocked:   none.
/// Arrange:  `statement` from `build_mint_statement(&adapter,
///   Default::default())`, whose values are distinct at every position, so a
///   swap of same-kind fields or a dropped field fails; every selection of
///   `DECLARED_ENCODING_SELECTIONS` over every pairing concrete and every
///   member of `CHAIN_FORMS_CONCRETES`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, the innermost consumer calling the encoder's
///   `encode(EncodeParams { description }, &statement)` and then
///   `decode(DecodeParams { description }, bytes)` with the bytes `encode`
///   returned.
/// Assert:   both calls return `Ok`; the decoded statement's
///   `suite_identifier`, `asset_identity_hash`, `parameter_set_identifier`,
///   `purpose`, and `expiry` equal the original's; each form field equals the
///   original's by `assert_eq!`; and each group element's `encode_g1` or
///   `encode_g2` through the received pairing equals the original's.
#[test]
fn a_mint_statement_survives_the_encoding_round_trip() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'a, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainFormsConsumer<'a, 'b, P, E>
                    where
                        P: IPairingAdapter,
                        E: IEncoderAdapter + IDecoderAdapter,
                    {
                        adapter: &'a P,
                        encoder: &'b E,
                    }

                    impl<'a, 'b, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, 'b, P, E>
                    where
                        P: IPairingAdapter,
                        E: IEncoderAdapter + IDecoderAdapter,
                    {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let params: MintStatementDescriptionConstructorParams<'_, P> =
                                build_mint_statement_description_constructor_params(
                                    self.adapter,
                                    Default::default(),
                                );
                            let Ok(description) = MintStatementDescription::<P, F>::try_new(params);
                            let statement = build_mint_statement::<P, F>(
                                self.adapter,
                                MintStatementOverrides::default(),
                            );

                            let encoded = self
                                .encoder
                                .encode(
                                    EncodeParams {
                                        description: &description,
                                    },
                                    &statement,
                                )
                                .expect("the encoder admits the statement");
                            let decoded = self
                                .encoder
                                .decode(
                                    DecodeParams {
                                        description: &description,
                                    },
                                    &encoded.bytes,
                                )
                                .expect("the decoder admits the encoding")
                                .described;

                            assert_eq!(decoded.suite_identifier, statement.suite_identifier);
                            assert_eq!(decoded.asset_identity_hash, statement.asset_identity_hash);
                            assert_eq!(
                                decoded.parameter_set_identifier,
                                statement.parameter_set_identifier
                            );
                            assert_eq!(decoded.purpose, statement.purpose);
                            assert_eq!(decoded.expiry, statement.expiry);
                            assert_eq!(decoded.chain, statement.chain);
                            assert_eq!(
                                decoded.entitlement_contract,
                                statement.entitlement_contract
                            );
                            assert_eq!(decoded.source_entitlement, statement.source_entitlement);
                            assert_eq!(decoded.target_entitlement, statement.target_entitlement);
                            assert_eq!(decoded.old_interval, statement.old_interval);
                            assert_eq!(decoded.new_interval, statement.new_interval);
                            assert_eq!(decoded.seller, statement.seller);
                            assert_eq!(decoded.buyer, statement.buyer);

                            let encode_g1 = |point: P::G1| -> Vec<u8> {
                                let Ok(encoded) = self.adapter.encode_g1(
                                    EncodeG1Params,
                                    build_encode_g1_payload(
                                        self.adapter,
                                        EncodeG1PayloadOverrides { point: Some(point) },
                                    ),
                                );
                                encoded.bytes.as_ref().to_vec()
                            };
                            let encode_g2 = |point: P::G2| -> Vec<u8> {
                                let Ok(encoded) = self.adapter.encode_g2(
                                    EncodeG2Params,
                                    build_encode_g2_payload(
                                        self.adapter,
                                        EncodeG2PayloadOverrides { point: Some(point) },
                                    ),
                                );
                                encoded.bytes.as_ref().to_vec()
                            };

                            assert_eq!(
                                encode_g1(decoded.buyer_keys.pk1),
                                encode_g1(statement.buyer_keys.pk1)
                            );
                            assert_eq!(
                                encode_g2(decoded.buyer_keys.pk2),
                                encode_g2(statement.buyer_keys.pk2)
                            );
                            assert_eq!(
                                encode_g1(decoded.envelope.c1),
                                encode_g1(statement.envelope.c1)
                            );
                            assert_eq!(
                                encode_g1(decoded.envelope.c2),
                                encode_g1(statement.envelope.c2)
                            );
                            assert_eq!(
                                encode_g2(decoded.envelope.d1),
                                encode_g2(statement.envelope.d1)
                            );
                            assert_eq!(
                                encode_g2(decoded.envelope.d2),
                                encode_g2(statement.envelope.d2)
                            );
                            assert_eq!(
                                encode_g2(decoded.first_messages.hpub),
                                encode_g2(statement.first_messages.hpub)
                            );
                            assert_eq!(
                                encode_g1(decoded.first_messages.c1),
                                encode_g1(statement.first_messages.c1)
                            );
                            assert_eq!(
                                encode_g1(decoded.first_messages.c2),
                                encode_g1(statement.first_messages.c2)
                            );
                            assert_eq!(
                                encode_g2(decoded.first_messages.d1),
                                encode_g2(statement.first_messages.d1)
                            );
                            assert_eq!(
                                encode_g2(decoded.first_messages.d2),
                                encode_g2(statement.first_messages.d2)
                            );
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let consumer = ChainFormsConsumer {
                            adapter: self.adapter,
                            encoder: &payload.adapter,
                        };
                        let deps = build_create_chain_forms_deps(consumer, Default::default());
                        let concrete: ChainFormsConcrete = *concrete;
                        let params =
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(concrete),
                                ..Default::default()
                            });
                        let payload = build_create_chain_forms_payload(Default::default());

                        create_chain_forms(&deps, params, payload)
                            .expect("the factory admits each declared concrete");
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let consumer = EncodingConsumer {
                    adapter: &payload.adapter,
                };
                let deps = build_create_encoding_deps(consumer);
                let params = build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(selection.concrete),
                    identifier: Some(selection.identifier),
                    ..Default::default()
                });

                create_encoding(&deps, params, CreateEncodingPayload)
                    .expect("the factory admits each declared selection");
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        create_pairing(&deps, params, CreatePairingPayload)
            .expect("the factory admits each declared concrete");
    }
}

/// Contract: entry
///   `each_second_group_decoder_refuses_the_second_group_reference_encoding`;
///   given a field list whose second-group buyer-key position holds the
///   concrete's second-group reference encoding, `fields_to_value` returns
///   `InvalidG2` at that index with `DecodeG2ErrorReturn::NotInSubgroup`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `g2_outside_subgroup_encoding`,
///   `create_chain_forms`, `consume_chain_forms`, the description's `try_new`
///   and `fields_to_value`, and `decode_g2`.
/// Mocked:   none.
/// Arrange:  the received pairing's `g2_outside_subgroup_encoding` bytes as
///   `outside`, and `valid1` the bytes of `encode_g1` of the points of
///   `build_mint_statement(&adapter, Default::default())`'s `buyer_keys.pk1`;
///   `payload` is `invalidate_mint_statement(&adapter,
///   MintStatementCorruptions { buyer_keys: Some(vec![Bytes(valid1),
///   Bytes(outside)]), ..Default::default() })`, the first-group field valid,
///   so a decoder that checks the curve alone passes the field and fails the
///   entry; every pairing concrete in turn, each with every member of
///   `CHAIN_FORMS_CONCRETES`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, the innermost consumer calling
///   `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:   `assert_eq!(result.err(), Some(
///   MintStatementFromFieldsErrorReturn::InvalidG2 { index: 14,
///   error: DecodeG2ErrorReturn::NotInSubgroup }))`, the whole error.
#[test]
fn each_second_group_decoder_refuses_the_second_group_reference_encoding() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct ChainFormsConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
                outside: Vec<u8>,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for ChainFormsConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let params: MintStatementDescriptionConstructorParams<'_, P> =
                        build_mint_statement_description_constructor_params(
                            self.adapter,
                            Default::default(),
                        );
                    let Ok(description) = MintStatementDescription::<P, F>::try_new(params);
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides::default(),
                    );
                    let Ok(encoded1) = self.adapter.encode_g1(
                        EncodeG1Params,
                        build_encode_g1_payload(
                            self.adapter,
                            EncodeG1PayloadOverrides {
                                point: Some(statement.buyer_keys.pk1),
                            },
                        ),
                    );
                    let fields = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer_keys: Some(vec![
                                CanonicalFieldValue::Bytes(encoded1.bytes.as_ref().to_vec()),
                                CanonicalFieldValue::Bytes(self.outside.clone()),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), fields);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::InvalidG2 {
                            index: 14,
                            error: DecodeG2ErrorReturn::NotInSubgroup
                        })
                    );
                }
            }

            let outside = payload
                .adapter
                .g2_outside_subgroup_encoding(
                    G2OutsideSubgroupEncodingParams,
                    G2OutsideSubgroupEncodingPayload,
                )
                .expect("each declared concrete's second-group reference exists")
                .bytes
                .as_ref()
                .to_vec();

            for concrete in CHAIN_FORMS_CONCRETES {
                let consumer = ChainFormsConsumer {
                    adapter: &payload.adapter,
                    outside: outside.clone(),
                };
                let deps = build_create_chain_forms_deps(consumer, Default::default());
                let concrete: ChainFormsConcrete = *concrete;
                let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                });
                let payload = build_create_chain_forms_payload(Default::default());

                create_chain_forms(&deps, params, payload)
                    .expect("the factory admits each declared concrete");
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        create_pairing(&deps, params, CreatePairingPayload)
            .expect("the factory admits each declared concrete");
    }
}

/// Contract: entry
///   `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`;
///   whenever the concrete's `g1_outside_subgroup_encoding` holds bytes, a
///   field list whose first-group buyer-key position holds them is refused by
///   `fields_to_value` as `InvalidG1` at that index with
///   `DecodeG1ErrorReturn::NotInSubgroup`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `g1_outside_subgroup_encoding`,
///   `create_chain_forms`, `consume_chain_forms`, the description's `try_new`
///   and `fields_to_value`, and `decode_g1`.
/// Mocked:   none.
/// Arrange:  the received pairing's `g1_outside_subgroup_encoding` bytes as
///   `outside`, present for a concrete whose group has such a point, and
///   `valid2` the bytes of `encode_g2` of the points of
///   `build_mint_statement(&adapter, Default::default())`'s `buyer_keys.pk2`;
///   for a concrete holding bytes, `payload` is
///   `invalidate_mint_statement(&adapter, MintStatementCorruptions {
///   buyer_keys: Some(vec![Bytes(outside), Bytes(valid2)]),
///   ..Default::default() })`; every pairing concrete in turn, so a concrete
///   with no such point and a concrete with one both run.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, the innermost consumer calling
///   `description.fields_to_value(FromFieldsParams, payload)` for a concrete
///   holding bytes.
/// Assert:   for a concrete holding bytes, `assert_eq!(result.err(), Some(
///   MintStatementFromFieldsErrorReturn::InvalidG1 { index: 13,
///   error: DecodeG1ErrorReturn::NotInSubgroup }))`, the whole error; across
///   the declared set at least one concrete held bytes.
#[test]
fn each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists() {
    // Arrange
    struct PairingConsumer<'a> {
        saw: &'a Cell<bool>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct ChainFormsConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
                outside: Vec<u8>,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for ChainFormsConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let params: MintStatementDescriptionConstructorParams<'_, P> =
                        build_mint_statement_description_constructor_params(
                            self.adapter,
                            Default::default(),
                        );
                    let Ok(description) = MintStatementDescription::<P, F>::try_new(params);
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides::default(),
                    );
                    let Ok(encoded2) = self.adapter.encode_g2(
                        EncodeG2Params,
                        build_encode_g2_payload(
                            self.adapter,
                            EncodeG2PayloadOverrides {
                                point: Some(statement.buyer_keys.pk2),
                            },
                        ),
                    );
                    let fields = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer_keys: Some(vec![
                                CanonicalFieldValue::Bytes(self.outside.clone()),
                                CanonicalFieldValue::Bytes(encoded2.bytes.as_ref().to_vec()),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), fields);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::InvalidG1 {
                            index: 13,
                            error: DecodeG1ErrorReturn::NotInSubgroup
                        })
                    );
                }
            }

            let reference = payload
                .adapter
                .g1_outside_subgroup_encoding(
                    G1OutsideSubgroupEncodingParams,
                    G1OutsideSubgroupEncodingPayload,
                )
                .expect("the first-group reference search either finds or is absent")
                .bytes;

            if let Some(outside) = reference {
                self.saw.set(true);
                let outside = outside.as_ref().to_vec();
                for concrete in CHAIN_FORMS_CONCRETES {
                    let consumer = ChainFormsConsumer {
                        adapter: &payload.adapter,
                        outside: outside.clone(),
                    };
                    let deps = build_create_chain_forms_deps(consumer, Default::default());
                    let concrete: ChainFormsConcrete = *concrete;
                    let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    });
                    let payload = build_create_chain_forms_payload(Default::default());

                    create_chain_forms(&deps, params, payload)
                        .expect("the factory admits each declared concrete");
                }
            }
        }
    }

    let saw = Cell::new(false);
    for concrete in PAIRING_CONCRETES {
        let deps = build_create_pairing_deps(PairingConsumer { saw: &saw }, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        create_pairing(&deps, params, CreatePairingPayload)
            .expect("the factory admits each declared concrete");
    }

    // Assert
    assert!(saw.get(), "at least one declared concrete held bytes");
}

/// Contract: entry `chain_forms_identity_survives_its_field_round_trip`; the
///   identity forms the description's `to_fields` yields as fixed twenty-byte
///   fields are rebuilt by `fields_to_value` into forms equal to the
///   originals.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `create_chain_forms`,
///   `consume_chain_forms`, the description's `try_new`, `to_fields`, and
///   `fields_to_value`, and the identity form's `to_field` and `from_field`.
/// Mocked:   none.
/// Arrange:  `statement` from `build_mint_statement(&adapter,
///   Default::default())`, whose entitlement contract, seller, and buyer are
///   distinct from one another, so a rebuild from a default, from one
///   position's field, or from a swapped position fails; every pairing
///   concrete in turn, each with every member of `CHAIN_FORMS_CONCRETES`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, the innermost consumer calling
///   `description.to_fields(ToFieldsParams, &statement)` and handing its
///   fields to `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:   both calls return `Ok`, and the rebuilt statement's
///   `entitlement_contract`, `seller`, and `buyer` equal the originals' by
///   `assert_eq!`, each in place.
#[test]
fn chain_forms_identity_survives_its_field_round_trip() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct ChainFormsConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for ChainFormsConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let params: MintStatementDescriptionConstructorParams<'_, P> =
                        build_mint_statement_description_constructor_params(
                            self.adapter,
                            Default::default(),
                        );
                    let Ok(description) = MintStatementDescription::<P, F>::try_new(params);
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides::default(),
                    );

                    let Ok(flattened) = description
                        .to_fields(build_to_fields_params(Default::default()), &statement);
                    let rebuilt = description
                        .fields_to_value(
                            build_from_fields_params(Default::default()),
                            flattened.fields,
                        )
                        .expect("the description admits its own rendering")
                        .described;

                    assert_eq!(rebuilt.entitlement_contract, statement.entitlement_contract);
                    assert_eq!(rebuilt.seller, statement.seller);
                    assert_eq!(rebuilt.buyer, statement.buyer);
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let consumer = ChainFormsConsumer {
                    adapter: &payload.adapter,
                };
                let deps = build_create_chain_forms_deps(consumer, Default::default());
                let concrete: ChainFormsConcrete = *concrete;
                let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                });
                let payload = build_create_chain_forms_payload(Default::default());

                create_chain_forms(&deps, params, payload)
                    .expect("the factory admits each declared concrete");
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        create_pairing(&deps, params, CreatePairingPayload)
            .expect("the factory admits each declared concrete");
    }
}

/// Contract: entry `chain_forms_entitlement_survives_its_field_round_trip`;
///   the entitlement forms the description's `to_fields` yields as 256-bit
///   fields are rebuilt by `fields_to_value` into forms equal to the
///   originals.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `create_chain_forms`,
///   `consume_chain_forms`, the description's `try_new`, `to_fields`, and
///   `fields_to_value`, and the entitlement form's `to_field` and
///   `from_field`.
/// Mocked:   none.
/// Arrange:  `statement` from `build_mint_statement(&adapter,
///   Default::default())`, whose source and target entitlements differ, so a
///   rebuild from a default, from one position's field, or from a swapped
///   position fails; every pairing concrete in turn, each with every member
///   of `CHAIN_FORMS_CONCRETES`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, the innermost consumer calling
///   `description.to_fields(ToFieldsParams, &statement)` and handing its
///   fields to `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:   both calls return `Ok`, and the rebuilt statement's
///   `source_entitlement` and `target_entitlement` equal the originals' by
///   `assert_eq!`, each in place.
#[test]
fn chain_forms_entitlement_survives_its_field_round_trip() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct ChainFormsConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for ChainFormsConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let params: MintStatementDescriptionConstructorParams<'_, P> =
                        build_mint_statement_description_constructor_params(
                            self.adapter,
                            Default::default(),
                        );
                    let Ok(description) = MintStatementDescription::<P, F>::try_new(params);
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides::default(),
                    );

                    let Ok(flattened) = description
                        .to_fields(build_to_fields_params(Default::default()), &statement);
                    let rebuilt = description
                        .fields_to_value(
                            build_from_fields_params(Default::default()),
                            flattened.fields,
                        )
                        .expect("the description admits its own rendering")
                        .described;

                    assert_eq!(rebuilt.source_entitlement, statement.source_entitlement);
                    assert_eq!(rebuilt.target_entitlement, statement.target_entitlement);
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let consumer = ChainFormsConsumer {
                    adapter: &payload.adapter,
                };
                let deps = build_create_chain_forms_deps(consumer, Default::default());
                let concrete: ChainFormsConcrete = *concrete;
                let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                });
                let payload = build_create_chain_forms_payload(Default::default());

                create_chain_forms(&deps, params, payload)
                    .expect("the factory admits each declared concrete");
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        create_pairing(&deps, params, CreatePairingPayload)
            .expect("the factory admits each declared concrete");
    }
}

/// Contract: entry `chain_forms_interval_survives_its_field_round_trip`; the
///   interval forms the description's `to_fields` yields as 64-bit fields are
///   rebuilt by `fields_to_value` into forms equal to the originals.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `create_chain_forms`,
///   `consume_chain_forms`, the description's `try_new`, `to_fields`, and
///   `fields_to_value`, and the interval form's `to_field` and `from_field`.
/// Mocked:   none.
/// Arrange:  `statement` from `build_mint_statement(&adapter,
///   Default::default())`, whose old and new interval counters differ, so a
///   rebuild from a default, from one position's field, or from a swapped
///   position fails; every pairing concrete in turn, each with every member
///   of `CHAIN_FORMS_CONCRETES`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, the innermost consumer calling
///   `description.to_fields(ToFieldsParams, &statement)` and handing its
///   fields to `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:   both calls return `Ok`, and the rebuilt statement's
///   `old_interval` and `new_interval` equal the originals' by `assert_eq!`,
///   each in place.
#[test]
fn chain_forms_interval_survives_its_field_round_trip() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct ChainFormsConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for ChainFormsConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let params: MintStatementDescriptionConstructorParams<'_, P> =
                        build_mint_statement_description_constructor_params(
                            self.adapter,
                            Default::default(),
                        );
                    let Ok(description) = MintStatementDescription::<P, F>::try_new(params);
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides::default(),
                    );

                    let Ok(flattened) = description
                        .to_fields(build_to_fields_params(Default::default()), &statement);
                    let rebuilt = description
                        .fields_to_value(
                            build_from_fields_params(Default::default()),
                            flattened.fields,
                        )
                        .expect("the description admits its own rendering")
                        .described;

                    assert_eq!(rebuilt.old_interval, statement.old_interval);
                    assert_eq!(rebuilt.new_interval, statement.new_interval);
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let consumer = ChainFormsConsumer {
                    adapter: &payload.adapter,
                };
                let deps = build_create_chain_forms_deps(consumer, Default::default());
                let concrete: ChainFormsConcrete = *concrete;
                let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                });
                let payload = build_create_chain_forms_payload(Default::default());

                create_chain_forms(&deps, params, payload)
                    .expect("the factory admits each declared concrete");
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        create_pairing(&deps, params, CreatePairingPayload)
            .expect("the factory admits each declared concrete");
    }
}

/// Contract: entry
///   `chain_forms_chain_identifier_survives_its_field_round_trip`; the
///   chain-identifier form the description's `to_fields` yields as a 256-bit
///   field is rebuilt by `fields_to_value` into a form equal to the original.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `create_chain_forms`,
///   `consume_chain_forms`, the description's `try_new`, `to_fields`, and
///   `fields_to_value`, and the chain-identifier form's `to_field` and
///   `from_field`.
/// Mocked:   none.
/// Arrange:  `statement` from `build_mint_statement(&adapter,
///   Default::default())`, whose chain identifier differs from every
///   entitlement, so a rebuild from a default or from an entitlement's field
///   fails; every pairing concrete in turn, each with every member of
///   `CHAIN_FORMS_CONCRETES`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, the innermost consumer calling
///   `description.to_fields(ToFieldsParams, &statement)` and handing its
///   fields to `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:   both calls return `Ok`, and the rebuilt statement's `chain`
///   equals the original's by `assert_eq!`, in place.
#[test]
fn chain_forms_chain_identifier_survives_its_field_round_trip() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct ChainFormsConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for ChainFormsConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let params: MintStatementDescriptionConstructorParams<'_, P> =
                        build_mint_statement_description_constructor_params(
                            self.adapter,
                            Default::default(),
                        );
                    let Ok(description) = MintStatementDescription::<P, F>::try_new(params);
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides::default(),
                    );

                    let Ok(flattened) = description
                        .to_fields(build_to_fields_params(Default::default()), &statement);
                    let rebuilt = description
                        .fields_to_value(
                            build_from_fields_params(Default::default()),
                            flattened.fields,
                        )
                        .expect("the description admits its own rendering")
                        .described;

                    assert_eq!(rebuilt.chain, statement.chain);
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let consumer = ChainFormsConsumer {
                    adapter: &payload.adapter,
                };
                let deps = build_create_chain_forms_deps(consumer, Default::default());
                let concrete: ChainFormsConcrete = *concrete;
                let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                });
                let payload = build_create_chain_forms_payload(Default::default());

                create_chain_forms(&deps, params, payload)
                    .expect("the factory admits each declared concrete");
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        create_pairing(&deps, params, CreatePairingPayload)
            .expect("the factory admits each declared concrete");
    }
}
