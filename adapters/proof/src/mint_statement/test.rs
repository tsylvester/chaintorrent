#![allow(clippy::expect_used)]

use super::provides::{
    DeliveryPurpose, DeliveryRelation, MINT_STATEMENT_FIELD_COUNT, MintPurpose,
    MintStatementCorruptions, MintStatementDescription, MintStatementFromFieldsErrorReturn,
    MintStatementOverrides, TransferPurpose, UndeclaredDeliveryPurposeCode, build_mint_statement,
    build_mint_statement_description, invalidate_mint_statement,
};
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsReturn, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    AssetIdentityHashTryNewErrorReturn, ParameterSetIdentifierTryNewErrorReturn,
    SuiteIdentifierTryNewErrorReturn,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldErrorReturn,
    ICanonicalField, IEncodingContract, build_from_fields_params, build_to_field_params,
    build_to_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeG1ErrorReturn, DecodeG2ErrorReturn, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, MockIPairingAdapterFailureMode,
    PairingConcrete, build_create_pairing_deps, build_create_pairing_params,
    build_encode_g1_payload, build_encode_g2_payload, create_pairing,
};

/// Contract: `FIELDS` is the kinds, in index order, `FixedBytes32`,
///   `Unsigned16`, the chain-identifier form's kind, the identity form's
///   kind, `FixedBytes32`, `FixedBytes32`, the entitlement form's kind
///   twice, the interval form's kind twice, `Unsigned16`, the identity
///   form's kind twice, `Bytes` six times, `Unsigned64`, and `Bytes` five
///   times.
/// Arrange: the `P` and `F` the consumers received; the constant is read
///   through the contract trait and no value is built.
/// Act:     `<MintStatementDescription<'_, P, F> as IEncodingContract>::FIELDS`
///   inside the inner consumer's method.
/// Assert:  in place, `assert_eq!` against the kinds stated in the contract
///   as a literal list, the form positions read as
///   `<F::… as ICanonicalField>::KIND`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn mint_statement_description_declares_its_field_kinds_in_transcript_order() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    _ = self.adapter;
                    assert_eq!(
                        <MintStatementDescription<'_, P, F> as IEncodingContract>::FIELDS,
                        [
                            CanonicalFieldKind::FixedBytes32,
                            CanonicalFieldKind::Unsigned16,
                            <F::ChainIdentifier as ICanonicalField>::KIND,
                            <F::Identity as ICanonicalField>::KIND,
                            CanonicalFieldKind::FixedBytes32,
                            CanonicalFieldKind::FixedBytes32,
                            <F::Entitlement as ICanonicalField>::KIND,
                            <F::Entitlement as ICanonicalField>::KIND,
                            <F::Interval as ICanonicalField>::KIND,
                            <F::Interval as ICanonicalField>::KIND,
                            CanonicalFieldKind::Unsigned16,
                            <F::Identity as ICanonicalField>::KIND,
                            <F::Identity as ICanonicalField>::KIND,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Unsigned64,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                            CanonicalFieldKind::Bytes,
                        ]
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: `to_fields` preserves the declared field order and kinds;
///   every field other than a group field and the purpose follows its form
///   and domain conversion.
/// Arrange: `statement` from `build_mint_statement` with `expiry` overridden
///   to `0x0102_0304_0506_0708`, whose form values, hashes, and identifiers
///   are distinct at every position, so a swap of same-kind fields, a
///   dropped field, or a shifted position fails; the description from
///   `build_mint_statement_description` over the received pairing.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method.
/// Assert:  in place, the field list holds `MINT_STATEMENT_FIELD_COUNT`
///   values; `0` is `FixedBytes32([0x22; 32])`, `1` is `Unsigned16(1)`, `4`
///   is `FixedBytes32([0x44; 32])`, `5` is `FixedBytes32([0x33; 32])`, `19`
///   is `Unsigned64(0x0102_0304_0506_0708)`, and `2`, `3`, `6`, `7`, `8`,
///   `9`, `11`, `12` each equal the `to_field` result of the statement's
///   `chain`, `entitlement_contract`, `source_entitlement`,
///   `target_entitlement`, `old_interval`, `new_interval`, `seller`, and
///   `buyer`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_lists_the_non_group_fields_in_transcript_order() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides {
                            expiry: Some(0x0102_0304_0506_0708),
                            ..Default::default()
                        },
                    );
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());

                    let fields = description
                        .to_fields(build_to_fields_params(Default::default()), &statement)
                        .expect("to_fields admits a statement")
                        .fields;

                    assert_eq!(fields.values.len(), MINT_STATEMENT_FIELD_COUNT);
                    assert_eq!(
                        fields.values[0],
                        CanonicalFieldValue::FixedBytes32([0x22; 32])
                    );
                    assert_eq!(fields.values[1], CanonicalFieldValue::Unsigned16(1));
                    assert_eq!(
                        fields.values[4],
                        CanonicalFieldValue::FixedBytes32([0x44; 32])
                    );
                    assert_eq!(
                        fields.values[5],
                        CanonicalFieldValue::FixedBytes32([0x33; 32])
                    );
                    assert_eq!(
                        fields.values[19],
                        CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708)
                    );
                    assert_eq!(
                        fields.values[2],
                        <F::ChainIdentifier as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.chain,
                        )
                        .expect("the chain form renders")
                        .field
                    );
                    assert_eq!(
                        fields.values[3],
                        <F::Identity as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.entitlement_contract,
                        )
                        .expect("the identity form renders")
                        .field
                    );
                    assert_eq!(
                        fields.values[6],
                        <F::Entitlement as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.source_entitlement,
                        )
                        .expect("the entitlement form renders")
                        .field
                    );
                    assert_eq!(
                        fields.values[7],
                        <F::Entitlement as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.target_entitlement,
                        )
                        .expect("the entitlement form renders")
                        .field
                    );
                    assert_eq!(
                        fields.values[8],
                        <F::Interval as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.old_interval,
                        )
                        .expect("the interval form renders")
                        .field
                    );
                    assert_eq!(
                        fields.values[9],
                        <F::Interval as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.new_interval,
                        )
                        .expect("the interval form renders")
                        .field
                    );
                    assert_eq!(
                        fields.values[11],
                        <F::Identity as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.seller,
                        )
                        .expect("the identity form renders")
                        .field
                    );
                    assert_eq!(
                        fields.values[12],
                        <F::Identity as ICanonicalField>::to_field(
                            build_to_field_params(Default::default()),
                            &statement.buyer,
                        )
                        .expect("the identity form renders")
                        .field
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: for each typed key, envelope, and first-message group field
///   `to_fields` calls the selected pairing's matching `encode_g1` or
///   `encode_g2` and copies the encoded wrapper's `as_ref()` bytes into
///   `CanonicalFieldValue::Bytes`.
/// Arrange: `statement` from `build_mint_statement` under defaults, whose
///   group points are distinct at every position, so a swap between
///   positions or between the two groups fails; the description.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method.
/// Assert:  in place, the values at `13`, `15`, `16`, `21`, `22` are `Bytes`
///   of `encode_g1` of `buyer_keys.pk1`, `envelope.c1`, `envelope.c2`,
///   `first_messages.c1`, `first_messages.c2`, and the values at `14`, `17`,
///   `18`, `20`, `23`, `24` are `Bytes` of `encode_g2` of `buyer_keys.pk2`,
///   `envelope.d1`, `envelope.d2`, `first_messages.hpub`,
///   `first_messages.d1`, `first_messages.d2`, each encoding read through
///   the received pairing with params and payload from its official
///   builders; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_flattens_each_group_field_through_the_pairing() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let statement = build_mint_statement::<P, F>(self.adapter, Default::default());
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());

                    let fields = description
                        .to_fields(build_to_fields_params(Default::default()), &statement)
                        .expect("to_fields admits a statement")
                        .fields;

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
                        fields.values[13],
                        CanonicalFieldValue::Bytes(encode_g1(statement.buyer_keys.pk1))
                    );
                    assert_eq!(
                        fields.values[14],
                        CanonicalFieldValue::Bytes(encode_g2(statement.buyer_keys.pk2))
                    );
                    assert_eq!(
                        fields.values[15],
                        CanonicalFieldValue::Bytes(encode_g1(statement.envelope.c1))
                    );
                    assert_eq!(
                        fields.values[16],
                        CanonicalFieldValue::Bytes(encode_g1(statement.envelope.c2))
                    );
                    assert_eq!(
                        fields.values[17],
                        CanonicalFieldValue::Bytes(encode_g2(statement.envelope.d1))
                    );
                    assert_eq!(
                        fields.values[18],
                        CanonicalFieldValue::Bytes(encode_g2(statement.envelope.d2))
                    );
                    assert_eq!(
                        fields.values[20],
                        CanonicalFieldValue::Bytes(encode_g2(statement.first_messages.hpub))
                    );
                    assert_eq!(
                        fields.values[21],
                        CanonicalFieldValue::Bytes(encode_g1(statement.first_messages.c1))
                    );
                    assert_eq!(
                        fields.values[22],
                        CanonicalFieldValue::Bytes(encode_g1(statement.first_messages.c2))
                    );
                    assert_eq!(
                        fields.values[23],
                        CanonicalFieldValue::Bytes(encode_g2(statement.first_messages.d1))
                    );
                    assert_eq!(
                        fields.values[24],
                        CanonicalFieldValue::Bytes(encode_g2(statement.first_messages.d2))
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: `to_fields` uses `purpose.code()` for the `Unsigned16` field.
/// Arrange: `statement` from `build_mint_statement` under defaults, whose
///   `purpose` is `MintPurpose::Mint`, against the replacement block's
///   `MintPurpose::Replacement`; the description.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method; the field list is returned whole, the
///   subject's return naming no received type.
/// Assert:  the value at `10` is `Unsigned16(1)`, the mint code the
///   delivery purposes declare.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_encodes_a_mint_purpose_by_its_code() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<CanonicalFields>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = CanonicalFields;

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let statement = build_mint_statement::<P, F>(self.adapter, Default::default());
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());

                    description
                        .to_fields(build_to_fields_params(Default::default()), &statement)
                        .expect("to_fields admits a statement")
                        .fields
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    let fields = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer")
        .output;
    assert_eq!(fields.values[10], CanonicalFieldValue::Unsigned16(1));
}

/// Contract: `to_fields` uses `purpose.code()` for the `Unsigned16` field.
/// Arrange: `statement` from `build_mint_statement` with `purpose`
///   overridden to `MintPurpose::Replacement`, against the mint block's
///   `MintPurpose::Mint`; the description.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method; the field list is returned whole.
/// Assert:  the value at `10` is `Unsigned16(4)`, the replacement code the
///   delivery purposes declare.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_encodes_a_replacement_purpose_by_its_code() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<CanonicalFields>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = CanonicalFields;

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let statement = build_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementOverrides {
                            purpose: Some(MintPurpose::Replacement),
                            ..Default::default()
                        },
                    );
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());

                    description
                        .to_fields(build_to_fields_params(Default::default()), &statement)
                        .expect("to_fields admits a statement")
                        .fields
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    let fields = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer")
        .output;
    assert_eq!(fields.values[10], CanonicalFieldValue::Unsigned16(4));
}

/// Contract: wrong count: `payload.values` that does not convert into
///   `[CanonicalFieldValue; MINT_STATEMENT_FIELD_COUNT]` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::FieldCount { expected:
///   MINT_STATEMENT_FIELD_COUNT, actual })`, `actual` the returned vector's
///   length.
/// Arrange: corruption `expiry: Some(vec![])`, so the list is one value
///   short of the count, against the one-long block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)` inside
///   the inner consumer's method; the result's error names no received
///   type, so the assertion is in place.
/// Assert:  `assert_eq!(result.err(), Some(MintStatementFromFieldsErrorReturn
///   ::FieldCount { expected: MINT_STATEMENT_FIELD_COUNT, actual: 24 }))`;
///   outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_field_list_one_short() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            expiry: Some(vec![]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldCount {
                            expected: MINT_STATEMENT_FIELD_COUNT,
                            actual: 24
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: wrong count: `payload.values` that does not convert into
///   `[CanonicalFieldValue; MINT_STATEMENT_FIELD_COUNT]` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::FieldCount { expected:
///   MINT_STATEMENT_FIELD_COUNT, actual })`, `actual` the returned vector's
///   length.
/// Arrange: corruption `expiry: Some(vec![Unsigned64(1), Unsigned64(2)])`,
///   so the list is one value long, against the one-short block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)` inside
///   the inner consumer's method.
/// Assert:  `FieldCount { expected: MINT_STATEMENT_FIELD_COUNT,
///   actual: 26 }`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_field_list_one_long() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            expiry: Some(vec![
                                CanonicalFieldValue::Unsigned64(1),
                                CanonicalFieldValue::Unsigned64(2),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldCount {
                            expected: MINT_STATEMENT_FIELD_COUNT,
                            actual: 26
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `0` that is not the variant that index's kind in `FIELDS` names is
///   refused as `Err(MintStatementFromFieldsErrorReturn::FieldKind {
///   index, expected })`, `expected` the kind at that index.
/// Arrange: corruption `suite_identifier: Some(vec![Unsigned64(1),
///   Unsigned16(1)])`, the identifier position holding a kind other than
///   its own.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 0, expected: FixedBytes32 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_suite_identifier_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            suite_identifier: Some(vec![
                                CanonicalFieldValue::Unsigned64(1),
                                CanonicalFieldValue::Unsigned16(1),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 0,
                            expected: CanonicalFieldKind::FixedBytes32
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `1` that is not its kind's variant is refused as `FieldKind {
///   index, expected }`.
/// Arrange: corruption `suite_identifier: Some(vec![FixedBytes32([0x22; 32]),
///   FixedBytes32([0x22; 32])])`, the version position holding a kind other
///   than its own, the identifier position valid.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 1, expected: Unsigned16 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_suite_version_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            suite_identifier: Some(vec![
                                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 1,
                            expected: CanonicalFieldKind::Unsigned16
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `4` that is not its kind's variant is refused as `FieldKind {
///   index, expected }`.
/// Arrange: corruption `asset_identity_hash: Some(vec![Unsigned16(1)])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 4, expected: FixedBytes32 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_an_asset_identity_hash_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            asset_identity_hash: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 4,
                            expected: CanonicalFieldKind::FixedBytes32
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `5` that is not its kind's variant is refused as `FieldKind {
///   index, expected }`.
/// Arrange: corruption `parameter_set_identifier: Some(vec![Unsigned16(1)])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 5, expected: FixedBytes32 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_parameter_set_digest_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            parameter_set_identifier: Some(vec![CanonicalFieldValue::Unsigned16(
                                1,
                            )]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 5,
                            expected: CanonicalFieldKind::FixedBytes32
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `10` that is not its kind's variant is refused as `FieldKind {
///   index, expected }`.
/// Arrange: corruption `purpose: Some(vec![FixedBytes32([0x01; 32])])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 10, expected: Unsigned16 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_purpose_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            purpose: Some(vec![CanonicalFieldValue::FixedBytes32([0x01; 32])]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 10,
                            expected: CanonicalFieldKind::Unsigned16
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `13` that is not its kind's variant is refused as `FieldKind {
///   index, expected }`.
/// Arrange: corruption `buyer_keys: Some(vec![Unsigned16(1),
///   Bytes(vec![])])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 13, expected: Bytes }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_buyer_key_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer_keys: Some(vec![
                                CanonicalFieldValue::Unsigned16(1),
                                CanonicalFieldValue::Bytes(vec![]),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 13,
                            expected: CanonicalFieldKind::Bytes
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `19` that is not its kind's variant is refused as `FieldKind {
///   index, expected }`.
/// Arrange: corruption `expiry: Some(vec![Unsigned16(1)])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 19, expected: Unsigned64 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_an_expiry_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            expiry: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 19,
                            expected: CanonicalFieldKind::Unsigned64
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: a directly mapped field of the wrong kind: the value at index
///   `20` that is not its kind's variant is refused as `FieldKind {
///   index, expected }`.
/// Arrange: corruption `first_messages: Some(vec![Unsigned16(1),
///   Bytes(vec![]), Bytes(vec![]), Bytes(vec![]), Bytes(vec![])])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 20, expected: Bytes }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_first_message_field_of_the_wrong_kind() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            first_messages: Some(vec![
                                CanonicalFieldValue::Unsigned16(1),
                                CanonicalFieldValue::Bytes(vec![]),
                                CanonicalFieldValue::Bytes(vec![]),
                                CanonicalFieldValue::Bytes(vec![]),
                                CanonicalFieldValue::Bytes(vec![]),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 20,
                            expected: CanonicalFieldKind::Bytes
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: suite identifier refused: fields `0` and `1` of their kinds
///   whose `SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
///   identifier, version })` returns `Err(error)` are refused as
///   `Err(MintStatementFromFieldsErrorReturn::SuiteIdentifier(error))`, the
///   refusal unchanged.
/// Arrange: corruption `suite_identifier: Some(vec![FixedBytes32([0x00; 32]),
///   Unsigned16(1)])`, the identifier zero and the version valid, against
///   the zero-version block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn
///   ::AllZeroIdentifier)`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_suite_identifiers_all_zero_identifier_refusal() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            suite_identifier: Some(vec![
                                CanonicalFieldValue::FixedBytes32([0x00; 32]),
                                CanonicalFieldValue::Unsigned16(1),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::SuiteIdentifier(
                            SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier
                        ))
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: suite identifier refused: `SuiteIdentifier::try_new` over
///   fields `0` and `1` returning `Err(error)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::SuiteIdentifier(error))`, the
///   refusal unchanged.
/// Arrange: corruption `suite_identifier: Some(vec![FixedBytes32([0x22; 32]),
///   Unsigned16(0)])`, the version zero and the identifier valid, against
///   the all-zero-identifier block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`;
///   outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_suite_identifiers_zero_version_refusal() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            suite_identifier: Some(vec![
                                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                                CanonicalFieldValue::Unsigned16(0),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::SuiteIdentifier(
                            SuiteIdentifierTryNewErrorReturn::ZeroVersion
                        ))
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `2` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::ChainIdentifier { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `chain: Some(vec![Unsigned16(1)])`, a variant no
///   form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `ChainIdentifier { index: 2, error: FromFieldErrorReturn
///   ::WrongKind { expected: <F::ChainIdentifier as ICanonicalField>::KIND }
///   }`, the whole error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_chain_identifiers_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            chain: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::ChainIdentifier {
                            index: 2,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::ChainIdentifier as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `3` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::Identity { index, error })`,
///   the refusal unchanged.
/// Arrange: corruption `entitlement_contract: Some(vec![Unsigned16(1)])`, a
///   variant no form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Identity { index: 3, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Identity as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_entitlement_contracts_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            entitlement_contract: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Identity {
                            index: 3,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Identity as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `6` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::Entitlement { index, error
///   })`, the refusal unchanged.
/// Arrange: corruption `source_entitlement: Some(vec![Unsigned16(1)])`, a
///   variant no form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Entitlement { index: 6, error: FromFieldErrorReturn::WrongKind
///   { expected: <F::Entitlement as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_source_entitlements_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            source_entitlement: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Entitlement {
                            index: 6,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Entitlement as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `7` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::Entitlement { index, error
///   })`, the refusal unchanged.
/// Arrange: corruption `target_entitlement: Some(vec![Unsigned16(1)])`, a
///   variant no form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Entitlement { index: 7, error: FromFieldErrorReturn::WrongKind
///   { expected: <F::Entitlement as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_target_entitlements_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            target_entitlement: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Entitlement {
                            index: 7,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Entitlement as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `8` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::Interval { index, error })`,
///   the refusal unchanged.
/// Arrange: corruption `old_interval: Some(vec![Unsigned16(1)])`, a variant
///   no form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Interval { index: 8, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Interval as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_old_intervals_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            old_interval: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Interval {
                            index: 8,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Interval as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `9` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::Interval { index, error })`,
///   the refusal unchanged.
/// Arrange: corruption `new_interval: Some(vec![Unsigned16(1)])`, a variant
///   no form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Interval { index: 9, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Interval as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_new_intervals_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            new_interval: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Interval {
                            index: 9,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Interval as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `11` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::Identity { index, error })`,
///   the refusal unchanged.
/// Arrange: corruption `seller: Some(vec![Unsigned16(1)])`, a variant no
///   form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Identity { index: 11, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Identity as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_sellers_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            seller: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Identity {
                            index: 11,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Identity as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: form refused: the form at index `12` returning `Err(error)`
///   from its `from_field(FromFieldParams, value)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::Identity { index, error })`,
///   the refusal unchanged.
/// Arrange: corruption `buyer: Some(vec![Unsigned16(1)])`, a variant no
///   form declares as its kind.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Identity { index: 12, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Identity as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_buyers_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Identity {
                            index: 12,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Identity as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: asset identity hash refused: field `4` a `FixedBytes32` whose
///   `AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
///   bytes })` returns `Err(error)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::AssetIdentityHash(error))`,
///   the refusal unchanged.
/// Arrange: corruption `asset_identity_hash: Some(vec![FixedBytes32(
///   [0x00; 32])])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `AssetIdentityHash(AssetIdentityHashTryNewErrorReturn
///   ::AllZero)`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_asset_identity_hashs_refusal() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            asset_identity_hash: Some(vec![CanonicalFieldValue::FixedBytes32(
                                [0x00; 32],
                            )]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::AssetIdentityHash(
                            AssetIdentityHashTryNewErrorReturn::AllZero
                        ))
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: parameter-set digest refused: field `5` a `FixedBytes32`
///   whose `ParameterSetIdentifier::try_new(ParameterSetIdentifier
///   ConstructorParams { bytes })` returns `Err(error)` is refused as
///   `Err(MintStatementFromFieldsErrorReturn::ParameterSetIdentifier
///   (error))`, the refusal unchanged.
/// Arrange: corruption `parameter_set_identifier: Some(vec![FixedBytes32(
///   [0x00; 32])])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn
///   ::AllZero)`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_parameter_set_digests_refusal() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            parameter_set_identifier: Some(vec![
                                CanonicalFieldValue::FixedBytes32([0x00; 32]),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::ParameterSetIdentifier(
                            ParameterSetIdentifierTryNewErrorReturn::AllZero
                        ))
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: purpose refused: `MintPurpose::try_from(code)` rejects any
///   code except mint or replacement with `PurposeCode { index: 10, code }`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(2)])`, the transfer
///   code, against the grant and undeclared blocks.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `PurposeCode { index: 10, code: 2 }`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_transfer_purpose_code() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            purpose: Some(vec![CanonicalFieldValue::Unsigned16(2)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::PurposeCode {
                            index: 10,
                            code: 2
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: purpose refused: `MintPurpose::try_from(code)` rejects any
///   code except mint or replacement with `PurposeCode { index: 10, code }`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(3)])`, the grant
///   code, against the transfer and undeclared blocks.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `PurposeCode { index: 10, code: 3 }`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_grant_purpose_code() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            purpose: Some(vec![CanonicalFieldValue::Unsigned16(3)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::PurposeCode {
                            index: 10,
                            code: 3
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: purpose refused: `MintPurpose::try_from(code)` rejects any
///   code except mint or replacement with `PurposeCode { index: 10, code }`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(5)])`, a code no
///   declared purpose carries.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `PurposeCode { index: 10, code: 5 }`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_purpose_code_no_declared_purpose_carries() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            purpose: Some(vec![CanonicalFieldValue::Unsigned16(5)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::PurposeCode {
                            index: 10,
                            code: 5
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: admitted: each group `Bytes` field is decoded in ascending
///   index through the selected pairing's `decode_g1` or `decode_g2`,
///   returning `InvalidG1` or `InvalidG2` with its index and decoder error
///   at the first refusal.
/// Arrange: `valid1` and `valid2` the bytes of the received pairing's
///   `encode_g1` and `encode_g2` of the points of the default statement's
///   `buyer_keys`, params and payload from the pairing's official builders;
///   corruption `buyer_keys: Some(vec![Bytes(valid1 without its last
///   byte), Bytes(valid2)])`, the second group's field valid, against the
///   second-group block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG1 { index: 13, error: DecodeG1ErrorReturn::WrongLength
///   { expected: valid1.len(), actual: valid1.len() - 1 } }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_a_first_group_decoders_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let statement = build_mint_statement::<P, F>(self.adapter, Default::default());
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let Ok(encoded1) = self.adapter.encode_g1(
                        EncodeG1Params,
                        build_encode_g1_payload(
                            self.adapter,
                            EncodeG1PayloadOverrides {
                                point: Some(statement.buyer_keys.pk1),
                            },
                        ),
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
                    let valid1 = encoded1.bytes.as_ref().to_vec();
                    let valid2 = encoded2.bytes.as_ref().to_vec();
                    let short1 = valid1[..valid1.len() - 1].to_vec();
                    let actual1 = short1.len();
                    let expected1 = valid1.len();
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer_keys: Some(vec![
                                CanonicalFieldValue::Bytes(short1),
                                CanonicalFieldValue::Bytes(valid2),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::InvalidG1 {
                            index: 13,
                            error: DecodeG1ErrorReturn::WrongLength {
                                expected: expected1,
                                actual: actual1
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: admitted: each group `Bytes` field is decoded in ascending
///   index through the selected pairing's `decode_g1` or `decode_g2`,
///   returning `InvalidG1` or `InvalidG2` with its index and decoder error
///   at the first refusal.
/// Arrange: `valid1` and `valid2` as the first-group block; corruption
///   `buyer_keys: Some(vec![Bytes(valid1), Bytes(valid2 without its last
///   byte)])`, the first group's field valid, against the first-group
///   block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG2 { index: 14, error: DecodeG2ErrorReturn::WrongLength
///   { expected: valid2.len(), actual: valid2.len() - 1 } }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_a_second_group_decoders_refusal_with_its_index() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let statement = build_mint_statement::<P, F>(self.adapter, Default::default());
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let Ok(encoded1) = self.adapter.encode_g1(
                        EncodeG1Params,
                        build_encode_g1_payload(
                            self.adapter,
                            EncodeG1PayloadOverrides {
                                point: Some(statement.buyer_keys.pk1),
                            },
                        ),
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
                    let valid1 = encoded1.bytes.as_ref().to_vec();
                    let valid2 = encoded2.bytes.as_ref().to_vec();
                    let short2 = valid2[..valid2.len() - 1].to_vec();
                    let actual2 = short2.len();
                    let expected2 = valid2.len();
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer_keys: Some(vec![
                                CanonicalFieldValue::Bytes(valid1),
                                CanonicalFieldValue::Bytes(short2),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::InvalidG2 {
                            index: 14,
                            error: DecodeG2ErrorReturn::WrongLength {
                                expected: expected2,
                                actual: actual2
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: admitted: each group `Bytes` field is decoded in ascending
///   index through the selected pairing's `decode_g1` or `decode_g2`,
///   returning `InvalidG1` or `InvalidG2` with its index and decoder error
///   at the first refusal.
/// Arrange: `valid1` and `valid2` as the first-group block; corruption
///   `buyer_keys: Some(vec![Bytes(valid1 without its last byte),
///   Bytes(valid2 without its last byte)])`, both group fields refused.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG1 { index: 13, error: DecodeG1ErrorReturn::WrongLength
///   { expected: valid1.len(), actual: valid1.len() - 1 } }`, the field at
///   the lower index deciding; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_the_lowest_refused_group_field() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let statement = build_mint_statement::<P, F>(self.adapter, Default::default());
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let Ok(encoded1) = self.adapter.encode_g1(
                        EncodeG1Params,
                        build_encode_g1_payload(
                            self.adapter,
                            EncodeG1PayloadOverrides {
                                point: Some(statement.buyer_keys.pk1),
                            },
                        ),
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
                    let valid1 = encoded1.bytes.as_ref().to_vec();
                    let valid2 = encoded2.bytes.as_ref().to_vec();
                    let short1 = valid1[..valid1.len() - 1].to_vec();
                    let short2 = valid2[..valid2.len() - 1].to_vec();
                    let actual1 = short1.len();
                    let expected1 = valid1.len();
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer_keys: Some(vec![
                                CanonicalFieldValue::Bytes(short1),
                                CanonicalFieldValue::Bytes(short2),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::InvalidG1 {
                            index: 13,
                            error: DecodeG1ErrorReturn::WrongLength {
                                expected: expected1,
                                actual: actual1
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: `MintStatementDescription::try_new(params)` stores
///   `params.pairing`; `fields_to_value` decodes every group `Bytes` field
///   through that pairing's `decode_g1` or `decode_g2`.
/// Arrange: the outer consumer's `params` from
///   `build_create_pairing_params(CreatePairingParamsOverrides { concrete:
///   Some(PairingConcrete::Mock(MockIPairingAdapterFailureMode
///   ::DecodeG1NotOnCurve)), ..Default::default() })`, so the received
///   pairing refuses a first-group encoding as off the curve; `payload` is
///   `invalidate_mint_statement` under no corruptions, whose first-group
///   encodings are the pairing's own and valid; against the blocks over the
///   default selection, where the same payload is admitted, so a
///   description that decodes through any pairing but the one it was built
///   over fails.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG1 { index: 13, error: DecodeG1ErrorReturn::NotOnCurve
///   }`, the whole error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's mock concrete selected by
///   `DecodeG1NotOnCurve` through `create_pairing` and the chain family's
///   mock concrete through `create_chain_forms` under default params.
#[test]
fn fields_to_value_decodes_through_the_pairing_the_description_was_built_over() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions::default(),
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::InvalidG1 {
                            index: 13,
                            error: DecodeG1ErrorReturn::NotOnCurve
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
            )),
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: admitted: after count, kind, domain, and form checks, decode
///   each group `Bytes` field and move decoded points into typed
///   `PublicKeysComponents`, `EnvelopeComponents`, and `MintFirstMessages`,
///   and the relation-specific purpose into `MintStatement<P, F>`; round
///   trip preserves the exact canonical field order and values.
/// Arrange: `payload` is `invalidate_mint_statement` under no corruptions;
///   `expected` is `build_mint_statement` under defaults, whose values are
///   distinct at every position, so a swap of same-kind fields, a dropped
///   field, or a shifted position fails.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  in place, the success arm's `described` equals `expected` field
///   by field: `suite_identifier`, `asset_identity_hash`,
///   `parameter_set_identifier`, `purpose`, and `expiry` by equality; each
///   form field by equality; each group element by equality of the
///   received pairing's `encode_g1` or `encode_g2` of it with that of the
///   corresponding point of `expected`; outside, both factories' `Ok`
///   arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_returns_the_transcript_the_reference_fields_describe() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions::default(),
                    );
                    let expected = build_mint_statement::<P, F>(self.adapter, Default::default());

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    let described = result
                        .expect("the reference fields describe a statement")
                        .described;
                    assert_eq!(described.suite_identifier, expected.suite_identifier);
                    assert_eq!(described.chain, expected.chain);
                    assert_eq!(
                        described.entitlement_contract,
                        expected.entitlement_contract
                    );
                    assert_eq!(described.asset_identity_hash, expected.asset_identity_hash);
                    assert_eq!(
                        described.parameter_set_identifier,
                        expected.parameter_set_identifier
                    );
                    assert_eq!(described.source_entitlement, expected.source_entitlement);
                    assert_eq!(described.target_entitlement, expected.target_entitlement);
                    assert_eq!(described.old_interval, expected.old_interval);
                    assert_eq!(described.new_interval, expected.new_interval);
                    assert_eq!(described.purpose, expected.purpose);
                    assert_eq!(described.seller, expected.seller);
                    assert_eq!(described.buyer, expected.buyer);
                    assert_eq!(described.expiry, expected.expiry);
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
                        encode_g1(described.buyer_keys.pk1),
                        encode_g1(expected.buyer_keys.pk1)
                    );
                    assert_eq!(
                        encode_g2(described.buyer_keys.pk2),
                        encode_g2(expected.buyer_keys.pk2)
                    );
                    assert_eq!(
                        encode_g1(described.envelope.c1),
                        encode_g1(expected.envelope.c1)
                    );
                    assert_eq!(
                        encode_g1(described.envelope.c2),
                        encode_g1(expected.envelope.c2)
                    );
                    assert_eq!(
                        encode_g2(described.envelope.d1),
                        encode_g2(expected.envelope.d1)
                    );
                    assert_eq!(
                        encode_g2(described.envelope.d2),
                        encode_g2(expected.envelope.d2)
                    );
                    assert_eq!(
                        encode_g2(described.first_messages.hpub),
                        encode_g2(expected.first_messages.hpub)
                    );
                    assert_eq!(
                        encode_g1(described.first_messages.c1),
                        encode_g1(expected.first_messages.c1)
                    );
                    assert_eq!(
                        encode_g1(described.first_messages.c2),
                        encode_g1(expected.first_messages.c2)
                    );
                    assert_eq!(
                        encode_g2(described.first_messages.d1),
                        encode_g2(expected.first_messages.d1)
                    );
                    assert_eq!(
                        encode_g2(described.first_messages.d2),
                        encode_g2(expected.first_messages.d2)
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: admitted: after count, kind, domain, and form checks, the
///   relation-specific purpose moves into `MintStatement<P, F>`; the mint
///   code reconstructs `MintPurpose::Mint`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(1)])`, against the
///   replacement block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  in place, the success arm's statement has `purpose` equal to
///   `MintPurpose::Mint`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_admits_a_mint_purpose() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            purpose: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    let described = result
                        .expect("a mint code is an admitted purpose")
                        .described;
                    assert_eq!(described.purpose, MintPurpose::Mint);
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: admitted: after count, kind, domain, and form checks, the
///   relation-specific purpose moves into `MintStatement<P, F>`; the
///   replacement code reconstructs `MintPurpose::Replacement`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(4)])`, against the
///   mint block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  in place, the success arm's statement has `purpose` equal to
///   `MintPurpose::Replacement`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_admits_a_replacement_purpose() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            purpose: Some(vec![CanonicalFieldValue::Unsigned16(4)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    let described = result
                        .expect("a replacement code is an admitted purpose")
                        .described;
                    assert_eq!(described.purpose, MintPurpose::Replacement);
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: ordering: the count precedes every field.
/// Arrange: corruptions `expiry: Some(vec![])` and `suite_identifier:
///   Some(vec![FixedBytes32([0x00; 32]), Unsigned16(1)])`, a list one short
///   that also holds a refused suite identifier.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldCount { expected: MINT_STATEMENT_FIELD_COUNT,
///   actual: 24 }`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_a_wrong_count_before_any_field() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            expiry: Some(vec![]),
                            suite_identifier: Some(vec![
                                CanonicalFieldValue::FixedBytes32([0x00; 32]),
                                CanonicalFieldValue::Unsigned16(1),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldCount {
                            expected: MINT_STATEMENT_FIELD_COUNT,
                            actual: 24
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: ordering: the fields are read in ascending index, each
///   field's kind check, form reconstruction, or domain construction
///   completing before the next field is read; the lowest failing index
///   decides.
/// Arrange: corruptions `seller: Some(vec![Unsigned16(1)])` and `expiry:
///   Some(vec![Unsigned16(1)])`, a form refusal at index `11` and a wrong
///   kind at index `19`, against the kind-first block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Identity { index: 11, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Identity as ICanonicalField>::KIND } }`, the whole
///   error; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_the_lowest_failing_field_when_a_form_refusal_comes_first() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            seller: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            expiry: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::Identity {
                            index: 11,
                            error: FromFieldErrorReturn::WrongKind {
                                expected: <F::Identity as ICanonicalField>::KIND
                            }
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: ordering: the fields are read in ascending index, each
///   field's kind check, form reconstruction, or domain construction
///   completing before the next field is read; the lowest failing index
///   decides.
/// Arrange: corruptions `purpose: Some(vec![FixedBytes32([0x01; 32])])` and
///   `buyer: Some(vec![Unsigned16(1)])`, a wrong kind at index `10` and a
///   form refusal at index `12`, against the form-first block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 10, expected: Unsigned16 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_the_lowest_failing_field_when_a_kind_failure_comes_first() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            purpose: Some(vec![CanonicalFieldValue::FixedBytes32([0x01; 32])]),
                            buyer: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 10,
                            expected: CanonicalFieldKind::Unsigned16
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: ordering: the suite identifier is constructed once field `1`
///   is read, before field `2`.
/// Arrange: corruptions `suite_identifier: Some(vec![FixedBytes32(
///   [0x00; 32]), Unsigned16(1)])` and `chain: Some(vec![Unsigned16(1)])`,
///   a refused suite identifier and a refused chain-identifier form at
///   index `2`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn
///   ::AllZeroIdentifier)`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_constructs_the_suite_identifier_before_reading_a_later_form() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            suite_identifier: Some(vec![
                                CanonicalFieldValue::FixedBytes32([0x00; 32]),
                                CanonicalFieldValue::Unsigned16(1),
                            ]),
                            chain: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::SuiteIdentifier(
                            SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier
                        ))
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}

/// Contract: `DeliveryPurpose::code`: `1` for `Mint`.
/// Arrange: `DeliveryPurpose::Mint`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Mint.code()`.
/// Assert:  the code equals `1u16`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_derives_the_mint_code() {
    let code = DeliveryPurpose::Mint.code();

    assert_eq!(code, 1u16);
}

/// Contract: `DeliveryPurpose::code`: `2` for `Transfer`.
/// Arrange: `DeliveryPurpose::Transfer`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Transfer.code()`.
/// Assert:  the code equals `2u16`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_derives_the_transfer_code() {
    let code = DeliveryPurpose::Transfer.code();

    assert_eq!(code, 2u16);
}

/// Contract: `DeliveryPurpose::code`: `3` for `Grant`.
/// Arrange: `DeliveryPurpose::Grant`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Grant.code()`.
/// Assert:  the code equals `3u16`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_derives_the_grant_code() {
    let code = DeliveryPurpose::Grant.code();

    assert_eq!(code, 3u16);
}

/// Contract: `DeliveryPurpose::code`: `4` for `Replacement`.
/// Arrange: `DeliveryPurpose::Replacement`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Replacement.code()`.
/// Assert:  the code equals `4u16`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_derives_the_replacement_code() {
    let code = DeliveryPurpose::Replacement.code();

    assert_eq!(code, 4u16);
}

/// Contract: `DeliveryPurpose::relation`: `DeliveryRelation::Mint` for
///   `Mint`.
/// Arrange: `DeliveryPurpose::Mint`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Mint.relation()`.
/// Assert:  the relation equals `DeliveryRelation::Mint`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_mint_proves_the_mint_relation() {
    let relation = DeliveryPurpose::Mint.relation();

    assert_eq!(relation, DeliveryRelation::Mint);
}

/// Contract: `DeliveryPurpose::relation`: `DeliveryRelation::Transfer` for
///   `Transfer`.
/// Arrange: `DeliveryPurpose::Transfer`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Transfer.relation()`.
/// Assert:  the relation equals `DeliveryRelation::Transfer`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_transfer_proves_the_transfer_relation() {
    let relation = DeliveryPurpose::Transfer.relation();

    assert_eq!(relation, DeliveryRelation::Transfer);
}

/// Contract: `DeliveryPurpose::relation`: `DeliveryRelation::Transfer` for
///   `Grant`.
/// Arrange: `DeliveryPurpose::Grant`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Grant.relation()`.
/// Assert:  the relation equals `DeliveryRelation::Transfer`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_grant_proves_the_transfer_relation() {
    let relation = DeliveryPurpose::Grant.relation();

    assert_eq!(relation, DeliveryRelation::Transfer);
}

/// Contract: `DeliveryPurpose::relation`: `DeliveryRelation::Mint` for
///   `Replacement`.
/// Arrange: `DeliveryPurpose::Replacement`, against the blocks of the other
///   purposes.
/// Act:     `DeliveryPurpose::Replacement.relation()`.
/// Assert:  the relation equals `DeliveryRelation::Mint`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn delivery_purpose_replacement_proves_the_mint_relation() {
    let relation = DeliveryPurpose::Replacement.relation();

    assert_eq!(relation, DeliveryRelation::Mint);
}

/// Contract: `TransferPurpose::code`: `2` for `Transfer`.
/// Arrange: `TransferPurpose::Transfer`, against the grant block.
/// Act:     `TransferPurpose::Transfer.code()`.
/// Assert:  the code equals `2u16`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn transfer_purpose_derives_the_transfer_code() {
    let code = TransferPurpose::Transfer.code();

    assert_eq!(code, 2u16);
}

/// Contract: `TransferPurpose::code`: `3` for `Grant`.
/// Arrange: `TransferPurpose::Grant`, against the transfer block.
/// Act:     `TransferPurpose::Grant.code()`.
/// Assert:  the code equals `3u16`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn transfer_purpose_derives_the_grant_code() {
    let code = TransferPurpose::Grant.code();

    assert_eq!(code, 3u16);
}

/// Contract: `TransferPurpose::try_from(code)`: `Ok(TransferPurpose
///   ::Transfer)` for `2`.
/// Arrange: the code `2u16`, against the grant and refused-code blocks.
/// Act:     `TransferPurpose::try_from(2u16)`.
/// Assert:  `assert_eq!(result, Ok(TransferPurpose::Transfer))`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn transfer_purpose_admits_the_transfer_code() {
    let result = TransferPurpose::try_from(2u16);

    assert_eq!(result, Ok(TransferPurpose::Transfer));
}

/// Contract: `TransferPurpose::try_from(code)`: `Ok(TransferPurpose
///   ::Grant)` for `3`.
/// Arrange: the code `3u16`, against the transfer and refused-code blocks.
/// Act:     `TransferPurpose::try_from(3u16)`.
/// Assert:  `assert_eq!(result, Ok(TransferPurpose::Grant))`.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn transfer_purpose_admits_the_grant_code() {
    let result = TransferPurpose::try_from(3u16);

    assert_eq!(result, Ok(TransferPurpose::Grant));
}

/// Contract: `TransferPurpose::try_from(code)`: `Err(Undeclared
///   DeliveryPurposeCode { code })` for every code other than `2` and `3`,
///   the mint relation's codes included.
/// Arrange: the code `1u16`, a code the mint relation carries, against the
///   admitted blocks.
/// Act:     `TransferPurpose::try_from(1u16)`.
/// Assert:  `assert_eq!(result, Err(UndeclaredDeliveryPurposeCode
///   { code: 1 }))`, the whole error.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn transfer_purpose_refuses_a_mint_relation_code() {
    let result = TransferPurpose::try_from(1u16);

    assert_eq!(result, Err(UndeclaredDeliveryPurposeCode { code: 1 }));
}

/// Contract: `TransferPurpose::try_from(code)`: `Err(Undeclared
///   DeliveryPurposeCode { code })` for every code other than `2` and `3`.
/// Arrange: the code `5u16`, against the admitted blocks.
/// Act:     `TransferPurpose::try_from(5u16)`.
/// Assert:  `assert_eq!(result, Err(UndeclaredDeliveryPurposeCode
///   { code: 5 }))`, the whole error.
/// Collaborators: none; the block calls the purpose's own method and
///   receives no family type.
#[test]
fn transfer_purpose_refuses_a_code_no_declared_purpose_carries() {
    let result = TransferPurpose::try_from(5u16);

    assert_eq!(result, Err(UndeclaredDeliveryPurposeCode { code: 5 }));
}

/// Contract: ordering: group decoding begins only after the count, kind,
///   domain, and form checks pass.
/// Arrange: corruptions `buyer_keys: Some(vec![Bytes(vec![0; 1]),
///   baseline.values[14].clone()])`, a decoding refusal at `13`, and
///   `first_messages` holding `Unsigned16(1)` followed by the clones of
///   `baseline.values[21]` through `baseline.values[24]`, a kind failure at
///   `20`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 20, expected: CanonicalFieldKind::Bytes }`,
///   not `InvalidG1`; outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_decodes_the_group_fields_only_after_every_other_check() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingAdapter> {
                adapter: &'a P,
            }

            impl<'a, P: IPairingAdapter> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    let description: MintStatementDescription<'_, P, F> =
                        build_mint_statement_description(self.adapter, Default::default());
                    let baseline =
                        invalidate_mint_statement::<P, F>(self.adapter, Default::default());
                    let payload = invalidate_mint_statement::<P, F>(
                        self.adapter,
                        MintStatementCorruptions {
                            buyer_keys: Some(vec![
                                CanonicalFieldValue::Bytes(vec![0; 1]),
                                baseline.values[14].clone(),
                            ]),
                            first_messages: Some(vec![
                                CanonicalFieldValue::Unsigned16(1),
                                baseline.values[21].clone(),
                                baseline.values[22].clone(),
                                baseline.values[23].clone(),
                                baseline.values[24].clone(),
                            ]),
                            ..Default::default()
                        },
                    );

                    let result = description
                        .fields_to_value(build_from_fields_params(Default::default()), payload);

                    assert_eq!(
                        result.err(),
                        Some(MintStatementFromFieldsErrorReturn::FieldKind {
                            index: 20,
                            expected: CanonicalFieldKind::Bytes
                        })
                    );
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        adapter: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the chain-forms factory ran the consumer");
}
