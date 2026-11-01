#![allow(clippy::expect_used)]

use super::provides::{
    ITransferStatementCaseConsumer, TRANSFER_STATEMENT_FIELD_COUNT,
    TransferStatementCaseParamsOverrides, TransferStatementCasePayload,
    TransferStatementCorruptions, TransferStatementDescription,
    TransferStatementFromFieldsErrorReturn, TransferStatementOverrides, build_transfer_statement,
    build_transfer_statement_case_deps, build_transfer_statement_case_params,
    build_transfer_statement_case_payload, build_transfer_statement_description,
    invalidate_transfer_statement, run_transfer_statement_case,
};
use crate::mint_statement::provides::TransferPurpose;
use chain::IChainForms;
use domain::{ParameterSetIdentifierTryNewErrorReturn, SuiteIdentifierTryNewErrorReturn};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldErrorReturn,
    ICanonicalField, IDecoderAdapter, IEncoderAdapter, IEncodingContract, build_from_fields_params,
    build_to_field_params, build_to_fields_params,
};
use hashing::RootTryNewErrorReturn;
use pairing::{
    CreatePairingParamsOverrides, DecodeG1ErrorReturn, DecodeG2ErrorReturn, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides, IPairingArithmetic,
    IPairingReference, MockIPairingAdapterFailureMode, PairingConcrete,
    build_create_pairing_params, build_encode_g1_payload, build_encode_g2_payload,
};

/// Contract: `FIELDS` is the kinds, in index order, `FixedBytes32`,
///   `Unsigned16`, the chain-identifier form's kind, the identity form's
///   kind, `FixedBytes32`, `FixedBytes32`, the entitlement form's kind
///   twice, the interval form's kind twice, `Unsigned16`, the identity
///   form's kind twice, `Bytes` twelve times, `Unsigned64`, and `Bytes` six
///   times.
/// Arrange: the `P` and `F` the consumers received; the constant is read
///   through the contract trait and no value is built; the mock forms
///   declare `FixedBytes20` for the identity, `Unsigned256` for the
///   entitlement and the chain identifier, and `Unsigned64` for the
///   interval, against the description's own `FixedBytes32`, `Unsigned16`,
///   `Bytes`, and `Unsigned64`, so a description that orders the identity
///   or interval form's kind, or any directly read kind, differently fails.
/// Act:     `<TransferStatementDescription<'_, P, F> as IEncodingContract>
///   ::FIELDS` inside the inner consumer's method.
/// Assert:  in place, `assert_eq!` against the kinds stated in the contract
///   as a literal list, the form positions read as
///   `<F::… as ICanonicalField>::KIND`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn transfer_statement_description_declares_its_field_kinds_in_transcript_order() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            _ = pairing;
            assert_eq!(
                <TransferStatementDescription<'_, P, F> as IEncodingContract>::FIELDS,
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
                    CanonicalFieldKind::Bytes,
                ]
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `to_fields`, encoded: the non-group positions hold the suite
///   identifier's bytes at `0` and its version at `1`, each form's own
///   `to_field` value at `2`, `3`, `6`, `7`, `8`, `9`, `11`, and `12`, the
///   asset identity hash's bytes at `4`, the parameter-set identifier's
///   bytes at `5`, and `Unsigned64(expiry)` at `25`.
/// Arrange: `statement` from `build_transfer_statement` with `purpose`
///   overridden to `TransferPurpose::Grant`, whose form values, hashes, and
///   identifiers are distinct at every position, so a swap of same-kind
///   fields, a dropped field, or a shifted position fails; the expected
///   form values built through each form's own `to_field`; the expected
///   domain values read through `identifier()`, `version()`, and
///   `as_bytes()`; the description from
///   `build_transfer_statement_description` over the received pairing.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method.
/// Assert:  in place, the field list holds `TRANSFER_STATEMENT_FIELD_COUNT`
///   values; `0` is `FixedBytes32` of the identifier, `1` is `Unsigned16`
///   of the version, `4` is `FixedBytes32` of the asset identity hash, `5`
///   is `FixedBytes32` of the parameter-set identifier, `25` is
///   `Unsigned64` of the expiry, and `2`, `3`, `6`, `7`, `8`, `9`, `11`,
///   `12` each equal the `to_field` result of the statement's `chain`,
///   `entitlement_contract`, `source_entitlement`, `target_entitlement`,
///   `old_interval`, `new_interval`, `seller`, and `buyer`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_lists_the_non_group_fields_in_transcript_order() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let statement = build_transfer_statement::<P, F>(
                pairing,
                TransferStatementOverrides {
                    purpose: Some(TransferPurpose::Grant),
                    ..Default::default()
                },
            );
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());

            let fields = description
                .to_fields(build_to_fields_params(Default::default()), &statement)
                .expect("to_fields admits a statement")
                .fields;

            assert_eq!(fields.values.len(), TRANSFER_STATEMENT_FIELD_COUNT);
            assert_eq!(
                fields.values[0],
                CanonicalFieldValue::FixedBytes32(*statement.suite_identifier.identifier())
            );
            assert_eq!(
                fields.values[1],
                CanonicalFieldValue::Unsigned16(statement.suite_identifier.version())
            );
            assert_eq!(
                fields.values[4],
                CanonicalFieldValue::FixedBytes32(*statement.asset_root.as_bytes())
            );
            assert_eq!(
                fields.values[5],
                CanonicalFieldValue::FixedBytes32(*statement.parameter_set_identifier.as_bytes())
            );
            assert_eq!(
                fields.values[25],
                CanonicalFieldValue::Unsigned64(statement.expiry)
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

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `to_fields`, encoded: each typed point is flattened to
///   `CanonicalFieldValue::Bytes` of the encoded wrapper's bytes,
///   `encode_g1` for the first-group points and `encode_g2` for the
///   second-group points, at `13` and `14` for `seller_keys.pk1` and
///   `pk2`, `15` and `16` for `buyer_keys.pk1` and `pk2`, `17` through `20`
///   for `old_envelope.c1`, `c2`, `d1`, and `d2`, `21` through `24` for
///   `new_envelope.c1`, `c2`, `d1`, and `d2`, and `26` through `31` for
///   `first_messages.pk1`, `c1`, `c2`, `pk2`, `d1`, and `d2`.
/// Arrange: `statement` from `build_transfer_statement` with `purpose`
///   overridden to `TransferPurpose::Grant`, its points distinct at every
///   position so that a swap of same-group points fails; the expected bytes
///   built through the received pairing's own `encode_g1` and `encode_g2`
///   over each point of the statement, read through the wrapper's
///   `as_ref()`; the description.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method.
/// Assert:  in place, each of the positions above equals
///   `CanonicalFieldValue::Bytes` of the expected encoding of its point, by
///   `assert_eq!`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_flattens_each_group_field_through_the_pairing() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let statement = build_transfer_statement::<P, F>(
                pairing,
                TransferStatementOverrides {
                    purpose: Some(TransferPurpose::Grant),
                    ..Default::default()
                },
            );
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());

            let fields = description
                .to_fields(build_to_fields_params(Default::default()), &statement)
                .expect("to_fields admits a statement")
                .fields;

            let encode_g1 = |point: P::G1| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides { point: Some(point) },
                    ),
                );
                encoded.bytes.as_ref().to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides { point: Some(point) },
                    ),
                );
                encoded.bytes.as_ref().to_vec()
            };

            assert_eq!(
                fields.values[13],
                CanonicalFieldValue::Bytes(encode_g1(statement.seller_keys.pk1))
            );
            assert_eq!(
                fields.values[14],
                CanonicalFieldValue::Bytes(encode_g2(statement.seller_keys.pk2))
            );
            assert_eq!(
                fields.values[15],
                CanonicalFieldValue::Bytes(encode_g1(statement.buyer_keys.pk1))
            );
            assert_eq!(
                fields.values[16],
                CanonicalFieldValue::Bytes(encode_g2(statement.buyer_keys.pk2))
            );
            assert_eq!(
                fields.values[17],
                CanonicalFieldValue::Bytes(encode_g1(statement.old_envelope.c1))
            );
            assert_eq!(
                fields.values[18],
                CanonicalFieldValue::Bytes(encode_g1(statement.old_envelope.c2))
            );
            assert_eq!(
                fields.values[19],
                CanonicalFieldValue::Bytes(encode_g2(statement.old_envelope.d1))
            );
            assert_eq!(
                fields.values[20],
                CanonicalFieldValue::Bytes(encode_g2(statement.old_envelope.d2))
            );
            assert_eq!(
                fields.values[21],
                CanonicalFieldValue::Bytes(encode_g1(statement.new_envelope.c1))
            );
            assert_eq!(
                fields.values[22],
                CanonicalFieldValue::Bytes(encode_g1(statement.new_envelope.c2))
            );
            assert_eq!(
                fields.values[23],
                CanonicalFieldValue::Bytes(encode_g2(statement.new_envelope.d1))
            );
            assert_eq!(
                fields.values[24],
                CanonicalFieldValue::Bytes(encode_g2(statement.new_envelope.d2))
            );
            assert_eq!(
                fields.values[26],
                CanonicalFieldValue::Bytes(encode_g1(statement.first_messages.pk1))
            );
            assert_eq!(
                fields.values[27],
                CanonicalFieldValue::Bytes(encode_g1(statement.first_messages.c1))
            );
            assert_eq!(
                fields.values[28],
                CanonicalFieldValue::Bytes(encode_g1(statement.first_messages.c2))
            );
            assert_eq!(
                fields.values[29],
                CanonicalFieldValue::Bytes(encode_g2(statement.first_messages.pk2))
            );
            assert_eq!(
                fields.values[30],
                CanonicalFieldValue::Bytes(encode_g2(statement.first_messages.d1))
            );
            assert_eq!(
                fields.values[31],
                CanonicalFieldValue::Bytes(encode_g2(statement.first_messages.d2))
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `to_fields`, encoded: `TransferPurpose::Transfer` is flattened
///   at `10` as `Unsigned16(purpose.code())`, the code `2`.
/// Arrange: `statement` from `build_transfer_statement` with `purpose`
///   overridden to `TransferPurpose::Transfer`; the code is the literal
///   `2`; the description.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method; the field list is returned whole, the
///   subject's return naming no received type.
/// Assert:  the value at `10` is `Unsigned16(2)`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_encodes_a_transfer_purpose_by_its_code() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = CanonicalFields;

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let statement = build_transfer_statement::<P, F>(
                pairing,
                TransferStatementOverrides {
                    purpose: Some(TransferPurpose::Transfer),
                    ..Default::default()
                },
            );
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());

            description
                .to_fields(build_to_fields_params(Default::default()), &statement)
                .expect("to_fields admits a statement")
                .fields
        }
    }

    let fields = run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer")
    .output;
    assert_eq!(fields.values[10], CanonicalFieldValue::Unsigned16(2));
}

/// Contract: `to_fields`, encoded: `TransferPurpose::Grant` is flattened at
///   `10` as `Unsigned16(purpose.code())`, the code `3`.
/// Arrange: `statement` from `build_transfer_statement` with `purpose`
///   overridden to `TransferPurpose::Grant`; the code is the literal `3`,
///   which differs from the transfer code so that a flattening that ignores
///   the purpose fails; the description.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` inside the
///   inner consumer's method; the field list is returned whole.
/// Assert:  the value at `10` is `Unsigned16(3)`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn to_fields_encodes_a_grant_purpose_by_its_code() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = CanonicalFields;

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let statement = build_transfer_statement::<P, F>(
                pairing,
                TransferStatementOverrides {
                    purpose: Some(TransferPurpose::Grant),
                    ..Default::default()
                },
            );
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());

            description
                .to_fields(build_to_fields_params(Default::default()), &statement)
                .expect("to_fields admits a statement")
                .fields
        }
    }

    let fields = run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer")
    .output;
    assert_eq!(fields.values[10], CanonicalFieldValue::Unsigned16(3));
}

/// Contract: `fields_to_value`, wrong count: `payload.values` that does not
///   convert into `[CanonicalFieldValue; TRANSFER_STATEMENT_FIELD_COUNT]`
///   is refused as `Err(TransferStatementFromFieldsErrorReturn::FieldCount {
///   expected: TRANSFER_STATEMENT_FIELD_COUNT, actual })`, `actual` the
///   returned vector's length.
/// Arrange: corruption `expiry: Some(vec![])`, so the list is one value
///   short of the count, against the one-long block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)` inside
///   the inner consumer's method; the result's error names no received
///   type, so the assertion is in place.
/// Assert:  `assert_eq!(result.err(), Some(TransferStatementFromFields
///   ErrorReturn::FieldCount { expected: TRANSFER_STATEMENT_FIELD_COUNT,
///   actual: 31 }))`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_field_list_one_short() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    expiry: Some(vec![]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldCount {
                    expected: TRANSFER_STATEMENT_FIELD_COUNT,
                    actual: 31
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong count: `payload.values` that does not
///   convert into `[CanonicalFieldValue; TRANSFER_STATEMENT_FIELD_COUNT]`
///   is refused as `Err(TransferStatementFromFieldsErrorReturn::FieldCount {
///   expected: TRANSFER_STATEMENT_FIELD_COUNT, actual })`, `actual` the
///   returned vector's length.
/// Arrange: corruption `expiry: Some(vec![Unsigned64(1), Unsigned64(2)])`,
///   so the list is one value long, against the one-short block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)` inside
///   the inner consumer's method.
/// Assert:  `FieldCount { expected: TRANSFER_STATEMENT_FIELD_COUNT,
///   actual: 33 }`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_field_list_one_long() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    expiry: Some(vec![
                        CanonicalFieldValue::Unsigned64(1),
                        CanonicalFieldValue::Unsigned64(2),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldCount {
                    expected: TRANSFER_STATEMENT_FIELD_COUNT,
                    actual: 33
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: ordering: the count precedes every field; a list of the wrong
///   count whose field `0` is also of the wrong kind returns `FieldCount`.
/// Arrange: corruption `suite_identifier: Some(vec![Unsigned16(1)])`, so
///   the list holds one value fewer than the count and its value at `0` is
///   not `FixedBytes32`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldCount { expected: TRANSFER_STATEMENT_FIELD_COUNT,
///   actual: 31 }`, not `FieldKind`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_a_wrong_count_before_any_field() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    suite_identifier: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldCount {
                    expected: TRANSFER_STATEMENT_FIELD_COUNT,
                    actual: 31
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `0` that is not `FixedBytes32` is refused as `Err(FieldKind {
///   index: 0, expected: CanonicalFieldKind::FixedBytes32 })`.
/// Arrange: corruption `suite_identifier: Some(vec![Unsigned16(1),
///   baseline.values[1].clone()])`, the identifier position holding a kind
///   other than its own, the count intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 0, expected: FixedBytes32 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_suite_identifier_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions::default(),
            );
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    suite_identifier: Some(vec![
                        CanonicalFieldValue::Unsigned16(1),
                        baseline.values[1].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 0,
                    expected: CanonicalFieldKind::FixedBytes32
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `1` that is not `Unsigned16` is refused as `Err(FieldKind {
///   index: 1, expected: CanonicalFieldKind::Unsigned16 })`.
/// Arrange: corruption `suite_identifier: Some(vec![baseline.values[0]
///   .clone(), FixedBytes32([1; 32])])`, the version position holding a
///   kind other than its own, the identifier position valid, the count
///   intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 1, expected: Unsigned16 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_suite_version_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions::default(),
            );
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    suite_identifier: Some(vec![
                        baseline.values[0].clone(),
                        CanonicalFieldValue::FixedBytes32([1; 32]),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 1,
                    expected: CanonicalFieldKind::Unsigned16
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `4` that is not `FixedBytes32` is refused as `Err(FieldKind {
///   index: 4, expected: CanonicalFieldKind::FixedBytes32 })`.
/// Arrange: corruption `asset_root: Some(vec![Unsigned16(1)])`,
///   the count intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 4, expected: FixedBytes32 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_an_asset_root_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    asset_root: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 4,
                    expected: CanonicalFieldKind::FixedBytes32
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `5` that is not `FixedBytes32` is refused as `Err(FieldKind {
///   index: 5, expected: CanonicalFieldKind::FixedBytes32 })`.
/// Arrange: corruption `parameter_set_identifier: Some(vec![Unsigned16(1)
///   ])`, the count intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 5, expected: FixedBytes32 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_parameter_set_digest_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    parameter_set_identifier: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 5,
                    expected: CanonicalFieldKind::FixedBytes32
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `10` that is not `Unsigned16` is refused as `Err(FieldKind {
///   index: 10, expected: CanonicalFieldKind::Unsigned16 })`.
/// Arrange: corruption `purpose: Some(vec![FixedBytes32([1; 32])])`, the
///   count intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 10, expected: Unsigned16 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_purpose_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    purpose: Some(vec![CanonicalFieldValue::FixedBytes32([1; 32])]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 10,
                    expected: CanonicalFieldKind::Unsigned16
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `13` that is not `Bytes` is refused as `Err(FieldKind {
///   index: 13, expected: CanonicalFieldKind::Bytes })`.
/// Arrange: corruption `seller_keys: Some(vec![Unsigned16(1),
///   baseline.values[14].clone()])`, the first seller-key position holding
///   a kind other than its own, the second keeping its baseline value, the
///   count intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 13, expected: Bytes }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_seller_key_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions::default(),
            );
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    seller_keys: Some(vec![
                        CanonicalFieldValue::Unsigned16(1),
                        baseline.values[14].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 13,
                    expected: CanonicalFieldKind::Bytes
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `25` that is not `Unsigned64` is refused as `Err(FieldKind {
///   index: 25, expected: CanonicalFieldKind::Unsigned64 })`.
/// Arrange: corruption `expiry: Some(vec![Unsigned16(1)])`, the count
///   intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 25, expected: Unsigned64 }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_an_expiry_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    expiry: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 25,
                    expected: CanonicalFieldKind::Unsigned64
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, wrong kind, directly mapped field: the
///   value at `26` that is not `Bytes` is refused as `Err(FieldKind {
///   index: 26, expected: CanonicalFieldKind::Bytes })`.
/// Arrange: corruption `first_messages: Some(vec![Unsigned16(1),
///   baseline.values[27].clone(), …, baseline.values[31].clone()])`, the
///   first first-message position holding a kind other than its own, the
///   remaining positions keeping their baseline values, the count intact.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 26, expected: Bytes }`; outside, both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_first_message_field_of_the_wrong_kind() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions::default(),
            );
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    first_messages: Some(vec![
                        CanonicalFieldValue::Unsigned16(1),
                        baseline.values[27].clone(),
                        baseline.values[28].clone(),
                        baseline.values[29].clone(),
                        baseline.values[30].clone(),
                        baseline.values[31].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 26,
                    expected: CanonicalFieldKind::Bytes
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, suite identifier refused: fields `0` and
///   `1` of their kinds whose `SuiteIdentifier::try_new` returns
///   `Err(error)` are refused as
///   `Err(TransferStatementFromFieldsErrorReturn::SuiteIdentifier(error))`,
///   the refusal unchanged.
/// Arrange: corruption `suite_identifier: Some(vec![FixedBytes32([0; 32]),
///   baseline.values[1].clone()])`, the identifier zero and the version
///   valid, against the zero-version block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn
///   ::AllZeroIdentifier)`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_suite_identifiers_all_zero_identifier_refusal() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions::default(),
            );
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    suite_identifier: Some(vec![
                        CanonicalFieldValue::FixedBytes32([0; 32]),
                        baseline.values[1].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::SuiteIdentifier(
                    SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier
                ))
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, suite identifier refused: fields `0` and
///   `1` of their kinds whose `SuiteIdentifier::try_new` returns
///   `Err(error)` are refused as
///   `Err(TransferStatementFromFieldsErrorReturn::SuiteIdentifier(error))`,
///   the refusal unchanged.
/// Arrange: corruption `suite_identifier: Some(vec![baseline.values[0]
///   .clone(), Unsigned16(0)])`, the version zero and the identifier valid,
///   against the all-zero-identifier block.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`;
///   outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_suite_identifiers_zero_version_refusal() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions::default(),
            );
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    suite_identifier: Some(vec![
                        baseline.values[0].clone(),
                        CanonicalFieldValue::Unsigned16(0),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::SuiteIdentifier(
                    SuiteIdentifierTryNewErrorReturn::ZeroVersion
                ))
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, asset identity hash refused: field `4` a
///   `FixedBytes32` whose `Root::try_new` returns `Err(error)`
///   is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::AssetIdentityHash
///   (error))`, the refusal unchanged.
/// Arrange: corruption `asset_root: Some(vec![FixedBytes32(
///   [0; 32])])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `AssetIdentityHash(RootTryNewErrorReturn
///   ::AllZero)`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_asset_roots_refusal() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    asset_root: Some(vec![CanonicalFieldValue::FixedBytes32([0; 32])]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::AssetRoot(
                    RootTryNewErrorReturn::AllZero
                ))
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, parameter-set digest refused: field `5` a
///   `FixedBytes32` whose `ParameterSetIdentifier::try_new` returns
///   `Err(error)` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::ParameterSetIdentifier
///   (error))`, the refusal unchanged.
/// Arrange: corruption `parameter_set_identifier: Some(vec![FixedBytes32(
///   [0; 32])])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn
///   ::AllZero)`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_parameter_set_digests_refusal() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    parameter_set_identifier: Some(vec![CanonicalFieldValue::FixedBytes32(
                        [0; 32],
                    )]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(
                    TransferStatementFromFieldsErrorReturn::ParameterSetIdentifier(
                        ParameterSetIdentifierTryNewErrorReturn::AllZero
                    )
                )
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the chain-identifier form at
///   `2` returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::ChainIdentifier { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `chain: Some(vec![Unsigned16(1)])`, a value of a
///   kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `ChainIdentifier { index: 2, error: FromFieldErrorReturn
///   ::WrongKind { expected: <F::ChainIdentifier as ICanonicalField>::KIND
///   } }`, the whole error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_chain_identifiers_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    chain: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::ChainIdentifier {
                    index: 2,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::ChainIdentifier as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the identity form at `3`
///   returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::Identity { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `entitlement_contract: Some(vec![Unsigned16(1)])`, a
///   value of a kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Identity { index: 3, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Identity as ICanonicalField>::KIND } }`, the whole
///   error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_entitlement_contracts_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    entitlement_contract: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::Identity {
                    index: 3,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::Identity as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the entitlement form at `6`
///   returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::Entitlement { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `source_entitlement: Some(vec![Unsigned16(1)])`, a
///   value of a kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Entitlement { index: 6, error: FromFieldErrorReturn::WrongKind
///   { expected: <F::Entitlement as ICanonicalField>::KIND } }`, the whole
///   error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_source_entitlements_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    source_entitlement: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::Entitlement {
                    index: 6,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::Entitlement as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the entitlement form at `7`
///   returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::Entitlement { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `target_entitlement: Some(vec![Unsigned16(1)])`, a
///   value of a kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Entitlement { index: 7, error: FromFieldErrorReturn::WrongKind
///   { expected: <F::Entitlement as ICanonicalField>::KIND } }`, the whole
///   error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_target_entitlements_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    target_entitlement: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::Entitlement {
                    index: 7,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::Entitlement as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the interval form at `8`
///   returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::Interval { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `old_interval: Some(vec![Unsigned16(1)])`, a value
///   of a kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Interval { index: 8, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Interval as ICanonicalField>::KIND } }`, the whole
///   error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_old_intervals_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    old_interval: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::Interval {
                    index: 8,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::Interval as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the interval form at `9`
///   returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::Interval { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `new_interval: Some(vec![Unsigned16(1)])`, a value
///   of a kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Interval { index: 9, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Interval as ICanonicalField>::KIND } }`, the whole
///   error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_new_intervals_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    new_interval: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::Interval {
                    index: 9,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::Interval as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the identity form at `11`
///   returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::Identity { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `seller: Some(vec![Unsigned16(1)])`, a value of a
///   kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Identity { index: 11, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Identity as ICanonicalField>::KIND } }`, the whole
///   error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_sellers_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    seller: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::Identity {
                    index: 11,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::Identity as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, form refused: the identity form at `12`
///   returning `Err(error)` from its `from_field` is refused as
///   `Err(TransferStatementFromFieldsErrorReturn::Identity { index,
///   error })`, the refusal unchanged.
/// Arrange: corruption `buyer: Some(vec![Unsigned16(1)])`, a value of a
///   kind no form declares.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `Identity { index: 12, error: FromFieldErrorReturn::WrongKind {
///   expected: <F::Identity as ICanonicalField>::KIND } }`, the whole
///   error; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_the_buyers_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    buyer: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::Identity {
                    index: 12,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::Identity as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, purpose refused:
///   `TransferPurpose::try_from(code)` rejects any code except transfer or
///   grant → `Err(PurposeCode { index: 10, code })`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(1)])`, the mint code,
///   against the replacement and undeclared blocks.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `PurposeCode { index: 10, code: 1 }`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_mint_purpose_code() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    purpose: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 1 })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, purpose refused:
///   `TransferPurpose::try_from(code)` rejects any code except transfer or
///   grant → `Err(PurposeCode { index: 10, code })`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(4)])`, the
///   replacement code.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `PurposeCode { index: 10, code: 4 }`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_replacement_purpose_code() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    purpose: Some(vec![CanonicalFieldValue::Unsigned16(4)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 4 })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, purpose refused:
///   `TransferPurpose::try_from(code)` rejects any code except transfer or
///   grant → `Err(PurposeCode { index: 10, code })`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(5)])`, a code no
///   purpose carries.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `PurposeCode { index: 10, code: 5 }`; outside, both factories'
///   `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_rejects_a_purpose_code_no_declared_purpose_carries() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    purpose: Some(vec![CanonicalFieldValue::Unsigned16(5)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 5 })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, purpose admitted: the code `2`
///   reconstructs `TransferPurpose::Transfer`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(2)])`, the
///   baseline's other values.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  in place, the success arm's `described.purpose` is
///   `TransferPurpose::Transfer`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_admits_a_transfer_purpose() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    purpose: Some(vec![CanonicalFieldValue::Unsigned16(2)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            let described = result
                .expect("a transfer code is an admitted purpose")
                .described;
            assert_eq!(described.purpose, TransferPurpose::Transfer);
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, purpose admitted: the code `3`
///   reconstructs `TransferPurpose::Grant`.
/// Arrange: corruption `purpose: Some(vec![Unsigned16(3)])`, the
///   baseline's other values, so a reconstruction that returns the
///   transfer purpose fails.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  in place, the success arm's `described.purpose` is
///   `TransferPurpose::Grant`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_admits_a_grant_purpose() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    purpose: Some(vec![CanonicalFieldValue::Unsigned16(3)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            let described = result
                .expect("a grant code is an admitted purpose")
                .described;
            assert_eq!(described.purpose, TransferPurpose::Grant);
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, invalid group encoding: a first-group
///   `Bytes` field that refuses to decode is refused as `Err(InvalidG1 {
///   index, error })`, the decoder's refusal unchanged.
/// Arrange: corruption `buyer_keys: Some(vec![Bytes(vec![0; 1]),
///   baseline.values[16].clone()])`, an encoding of the wrong length at
///   `15`; `expected` the length of the `Bytes` at `15` in `baseline`, the
///   received pairing's own encoding of a first-group point.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG1 { index: 15, error: DecodeG1ErrorReturn::WrongLength
///   { expected, actual: 1 } }`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_a_first_group_decoders_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(pairing, Default::default());
            let CanonicalFieldValue::Bytes(encoded) = &baseline.values[15] else {
                panic!("the baseline first-group field is Bytes")
            };
            let expected = encoded.len();
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    buyer_keys: Some(vec![
                        CanonicalFieldValue::Bytes(vec![0; 1]),
                        baseline.values[16].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::InvalidG1 {
                    index: 15,
                    error: DecodeG1ErrorReturn::WrongLength {
                        expected,
                        actual: 1
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, invalid group encoding: a second-group
///   `Bytes` field that refuses to decode is refused as `Err(InvalidG2 {
///   index, error })`, the decoder's refusal unchanged.
/// Arrange: corruption `new_envelope` holding the clones of
///   `baseline.values[21]` and `baseline.values[22]`, then `Bytes(vec![0;
///   1])`, then the clone of `baseline.values[24]`, an encoding of the
///   wrong length at `23`; `expected` the length of the `Bytes` at `23` in
///   `baseline`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG2 { index: 23, error: DecodeG2ErrorReturn::WrongLength
///   { expected, actual: 1 } }`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_carries_a_second_group_decoders_refusal_with_its_index() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(pairing, Default::default());
            let CanonicalFieldValue::Bytes(encoded) = &baseline.values[23] else {
                panic!("the baseline second-group field is Bytes")
            };
            let expected = encoded.len();
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    new_envelope: Some(vec![
                        baseline.values[21].clone(),
                        baseline.values[22].clone(),
                        CanonicalFieldValue::Bytes(vec![0; 1]),
                        baseline.values[24].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::InvalidG2 {
                    index: 23,
                    error: DecodeG2ErrorReturn::WrongLength {
                        expected,
                        actual: 1
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: invalid group encoding: the group fields decode in ascending
///   index and the first refusal decides.
/// Arrange: corruptions `seller_keys: Some(vec![baseline.values[13]
///   .clone(), Bytes(vec![0; 1])])` and `buyer_keys: Some(vec![Bytes(vec![0;
///   1]), baseline.values[16].clone()])`, a second-group refusal at `14`
///   below a first-group refusal at `15`, so a decoder that runs every
///   first-group field before any second-group field returns the higher
///   index; `expected` the length of the `Bytes` at `14` in `baseline`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG2 { index: 14, error: DecodeG2ErrorReturn::WrongLength
///   { expected, actual: 1 } }`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_the_lowest_refused_group_field() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(pairing, Default::default());
            let CanonicalFieldValue::Bytes(encoded) = &baseline.values[14] else {
                panic!("the baseline second-group field is Bytes")
            };
            let expected = encoded.len();
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    seller_keys: Some(vec![
                        baseline.values[13].clone(),
                        CanonicalFieldValue::Bytes(vec![0; 1]),
                    ]),
                    buyer_keys: Some(vec![
                        CanonicalFieldValue::Bytes(vec![0; 1]),
                        baseline.values[16].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::InvalidG2 {
                    index: 14,
                    error: DecodeG2ErrorReturn::WrongLength {
                        expected,
                        actual: 1
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: invalid group encoding: the first-group field at `13` decodes
///   through the pairing the description holds, whose `decode_g1` refuses
///   with `NotOnCurve`.
/// Arrange: the pairing factory's params select the mock concrete's
///   `DecodeG1NotOnCurve` failure mode; the payload is `baseline`, every
///   encoding of its right length, so only the pairing's failure mode can
///   refuse.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `InvalidG1 { index: 13, error: DecodeG1ErrorReturn::NotOnCurve
///   }`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's mock concrete under a
///   `CreatePairingParamsOverrides` failure mode and the chain family's
///   mock concrete through `create_chain_forms` under the default params.
#[test]
fn fields_to_value_decodes_through_the_pairing_the_description_was_built_over() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(pairing, Default::default());

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::InvalidG1 {
                    index: 13,
                    error: DecodeG1ErrorReturn::NotOnCurve
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(TransferStatementCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: ordering: group decoding begins only after the count, kind,
///   domain, and form checks pass.
/// Arrange: corruptions `seller_keys: Some(vec![Bytes(vec![0; 1]),
///   baseline.values[14].clone()])`, a decoding refusal at `13`, and
///   `first_messages` holding `Unsigned16(1)` followed by the clones of
///   `baseline.values[27]` through `baseline.values[31]`, a kind failure at
///   `26`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 26, expected: CanonicalFieldKind::Bytes }`,
///   not `InvalidG1`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_decodes_the_group_fields_only_after_every_other_check() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    seller_keys: Some(vec![
                        CanonicalFieldValue::Bytes(vec![0; 1]),
                        baseline.values[14].clone(),
                    ]),
                    first_messages: Some(vec![
                        CanonicalFieldValue::Unsigned16(1),
                        baseline.values[27].clone(),
                        baseline.values[28].clone(),
                        baseline.values[29].clone(),
                        baseline.values[30].clone(),
                        baseline.values[31].clone(),
                    ]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 26,
                    expected: CanonicalFieldKind::Bytes
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: ordering: the fields are read in ascending index and the
///   lowest failing index decides; a form refusal at `2` precedes a kind
///   failure at `4`.
/// Arrange: corruptions `chain: Some(vec![Unsigned16(1)])` and
///   `asset_root: Some(vec![Unsigned16(1)])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `ChainIdentifier { index: 2, error:
///   FromFieldErrorReturn::WrongKind { expected: <F::ChainIdentifier as
///   ICanonicalField>::KIND } }`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_the_lowest_failing_field_when_a_form_refusal_comes_first() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    chain: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    asset_root: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::ChainIdentifier {
                    index: 2,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: <F::ChainIdentifier as ICanonicalField>::KIND
                    }
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: ordering: the fields are read in ascending index and the
///   lowest failing index decides; a kind failure at `1` precedes a form
///   refusal at `3`.
/// Arrange: corruptions `suite_identifier: Some(vec![baseline.values[0]
///   .clone(), FixedBytes32([1; 32])])` and `entitlement_contract:
///   Some(vec![Unsigned16(1)])`.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `FieldKind { index: 1, expected:
///   CanonicalFieldKind::Unsigned16 }`; outside, both factories' `Ok`
///   arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_reports_the_lowest_failing_field_when_a_kind_failure_comes_first() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    suite_identifier: Some(vec![
                        baseline.values[0].clone(),
                        CanonicalFieldValue::FixedBytes32([1; 32]),
                    ]),
                    entitlement_contract: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::FieldKind {
                    index: 1,
                    expected: CanonicalFieldKind::Unsigned16
                })
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: ordering: the suite identifier is constructed once field `1`
///   is read, before a later form is reconstructed.
/// Arrange: corruptions `suite_identifier: Some(vec![FixedBytes32([0; 32]),
///   baseline.values[1].clone()])` and `chain:
///   Some(vec![Unsigned16(1)])`, a suite-identifier refusal below a
///   chain-identifier form refusal.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn::
///   AllZeroIdentifier)`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_constructs_the_suite_identifier_before_reading_a_later_form() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let baseline = invalidate_transfer_statement::<P, F>(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    suite_identifier: Some(vec![
                        CanonicalFieldValue::FixedBytes32([0; 32]),
                        baseline.values[1].clone(),
                    ]),
                    chain: Some(vec![CanonicalFieldValue::Unsigned16(1)]),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            assert_eq!(
                result.err(),
                Some(TransferStatementFromFieldsErrorReturn::SuiteIdentifier(
                    SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier
                ))
            );
        }
    }

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `fields_to_value`, admitted: every check passes and the
///   decoded points move into the typed key, envelope, and first-message
///   bundles and the relation-specific purpose into `TransferStatement<P,
///   F>` → `Ok(FromFieldsSuccessReturn { described })`.
/// Arrange: the payload `invalidate_transfer_statement` under the
///   corruption `purpose: Some(vec![Unsigned16(3)])`; `expected` is
///   `build_transfer_statement` under `TransferStatementOverrides {
///   purpose: Some(TransferPurpose::Grant), ..Default::default() }`, its
///   values distinct at every position so that a reconstruction that swaps
///   same-kind fields fails.
/// Act:     `description.fields_to_value(FromFieldsParams, payload)`.
/// Assert:  in place, the success arm's `described` holds `expected`'s
///   suite identifier, asset identity hash, parameter-set identifier, each
///   form, and expiry by equality, the purpose `TransferPurpose::Grant`,
///   and every point of the seller and buyer keys, the old and new
///   envelopes, and the first messages encoding through the received
///   pairing's `encode_g1` or `encode_g2` to the encoding of `expected`'s
///   point at that position; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn fields_to_value_returns_the_transcript_the_reference_fields_describe() {
    struct Consumer;

    impl ITransferStatementCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_statement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            _encoder: &E,
            _forms: &F,
            _payload: &TransferStatementCasePayload,
        ) -> Self::Output {
            let description: TransferStatementDescription<'_, P, F> =
                build_transfer_statement_description(pairing, Default::default());
            let payload = invalidate_transfer_statement::<P, F>(
                pairing,
                TransferStatementCorruptions {
                    purpose: Some(vec![CanonicalFieldValue::Unsigned16(3)]),
                    ..Default::default()
                },
            );
            let expected = build_transfer_statement::<P, F>(
                pairing,
                TransferStatementOverrides {
                    purpose: Some(TransferPurpose::Grant),
                    ..Default::default()
                },
            );

            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), payload);

            let described = result
                .expect("the reference fields describe a statement")
                .described;
            assert_eq!(described.suite_identifier, expected.suite_identifier);
            assert_eq!(described.chain, expected.chain);
            assert_eq!(
                described.entitlement_contract,
                expected.entitlement_contract
            );
            assert_eq!(described.asset_root, expected.asset_root);
            assert_eq!(
                described.parameter_set_identifier,
                expected.parameter_set_identifier
            );
            assert_eq!(described.source_entitlement, expected.source_entitlement);
            assert_eq!(described.target_entitlement, expected.target_entitlement);
            assert_eq!(described.old_interval, expected.old_interval);
            assert_eq!(described.new_interval, expected.new_interval);
            assert_eq!(described.purpose, TransferPurpose::Grant);
            assert_eq!(described.seller, expected.seller);
            assert_eq!(described.buyer, expected.buyer);
            assert_eq!(described.expiry, expected.expiry);
            let encode_g1 = |point: P::G1| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides { point: Some(point) },
                    ),
                );
                encoded.bytes.as_ref().to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides { point: Some(point) },
                    ),
                );
                encoded.bytes.as_ref().to_vec()
            };
            assert_eq!(
                encode_g1(described.seller_keys.pk1),
                encode_g1(expected.seller_keys.pk1)
            );
            assert_eq!(
                encode_g2(described.seller_keys.pk2),
                encode_g2(expected.seller_keys.pk2)
            );
            assert_eq!(
                encode_g1(described.buyer_keys.pk1),
                encode_g1(expected.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(described.buyer_keys.pk2),
                encode_g2(expected.buyer_keys.pk2)
            );
            assert_eq!(
                encode_g1(described.old_envelope.c1),
                encode_g1(expected.old_envelope.c1)
            );
            assert_eq!(
                encode_g1(described.old_envelope.c2),
                encode_g1(expected.old_envelope.c2)
            );
            assert_eq!(
                encode_g2(described.old_envelope.d1),
                encode_g2(expected.old_envelope.d1)
            );
            assert_eq!(
                encode_g2(described.old_envelope.d2),
                encode_g2(expected.old_envelope.d2)
            );
            assert_eq!(
                encode_g1(described.new_envelope.c1),
                encode_g1(expected.new_envelope.c1)
            );
            assert_eq!(
                encode_g1(described.new_envelope.c2),
                encode_g1(expected.new_envelope.c2)
            );
            assert_eq!(
                encode_g2(described.new_envelope.d1),
                encode_g2(expected.new_envelope.d1)
            );
            assert_eq!(
                encode_g2(described.new_envelope.d2),
                encode_g2(expected.new_envelope.d2)
            );
            assert_eq!(
                encode_g1(described.first_messages.pk1),
                encode_g1(expected.first_messages.pk1)
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
                encode_g2(described.first_messages.pk2),
                encode_g2(expected.first_messages.pk2)
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

    run_transfer_statement_case(
        &build_transfer_statement_case_deps(Consumer, Default::default()),
        build_transfer_statement_case_params(Default::default()),
        build_transfer_statement_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}
