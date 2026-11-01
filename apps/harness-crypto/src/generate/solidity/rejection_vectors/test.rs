#![allow(clippy::expect_used)]

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params, build_to_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    G1OutsideSubgroupEncodingErrorReturn, G2OutsideSubgroupEncodingErrorReturn,
    G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, MockIPairingAdapterFailureMode, PairingConcrete,
    ScalarFieldOrderParams, ScalarFieldOrderPayload, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::interface::{
    DecodeTarget, DecodeTargetTryNewErrorReturn, RejectionVectorDescription,
    RejectionVectorFromFieldsErrorReturn, RejectionVectorsErrorReturn, RejectionVectorsReturn,
};
use super::mock::{
    DecodeTargetConstructorParamsOverrides, RejectionVectorCorruptions, RejectionVectorOverrides,
    build_decode_target_constructor_params, build_rejection_vector,
    build_rejection_vector_description, build_rejection_vector_description_constructor_params,
    build_rejection_vectors_deps, build_rejection_vectors_params, build_rejection_vectors_payload,
    invalidate_rejection_vector,
};
use super::rejection_vectors;

/// Contract: `as_str` on `G1` returns `"G1"`.
/// Arrange: `DecodeTarget::G1`.
/// Act:    `as_str`.
/// Assert: the text equals `"G1"`.
#[test]
fn decode_target_g1_is_named_g1() {
    // Arrange
    let target = DecodeTarget::G1;

    // Act
    let text = target.as_str();

    // Assert
    assert_eq!(text, "G1");
}
// decode_target_g1_is_named_g1

/// Contract: `as_str` on `G2` returns `"G2"`.
/// Arrange: `DecodeTarget::G2`.
/// Act:    `as_str`.
/// Assert: the text equals `"G2"`.
#[test]
fn decode_target_g2_is_named_g2() {
    // Arrange
    let target = DecodeTarget::G2;

    // Act
    let text = target.as_str();

    // Assert
    assert_eq!(text, "G2");
}
// decode_target_g2_is_named_g2

/// Contract: `as_str` on `Scalar` returns `"Scalar"`.
/// Arrange: `DecodeTarget::Scalar`.
/// Act:    `as_str`.
/// Assert: the text equals `"Scalar"`.
#[test]
fn decode_target_scalar_is_named_scalar() {
    // Arrange
    let target = DecodeTarget::Scalar;

    // Act
    let text = target.as_str();

    // Assert
    assert_eq!(text, "Scalar");
}
// decode_target_scalar_is_named_scalar

/// Contract: `params.text` equals the `as_str()` of `G1` → `Ok(DecodeTarget::G1)`.
/// Arrange: `build_decode_target_constructor_params` with `text` `"G1"`.
/// Act:    `DecodeTarget::try_new` over it.
/// Assert: the result equals `Ok(DecodeTarget::G1)`.
#[test]
fn decode_target_admits_the_name_g1() {
    // Arrange
    let params = build_decode_target_constructor_params(DecodeTargetConstructorParamsOverrides {
        text: Some("G1".to_string()),
    });

    // Act
    let result = DecodeTarget::try_new(params);

    // Assert
    assert_eq!(result, Ok(DecodeTarget::G1));
}
// decode_target_admits_the_name_g1

/// Contract: `params.text` equals the `as_str()` of `G2` → `Ok(DecodeTarget::G2)`.
/// Arrange: `build_decode_target_constructor_params` with `text` `"G2"`.
/// Act:    `DecodeTarget::try_new` over it.
/// Assert: the result equals `Ok(DecodeTarget::G2)`.
#[test]
fn decode_target_admits_the_name_g2() {
    // Arrange
    let params = build_decode_target_constructor_params(DecodeTargetConstructorParamsOverrides {
        text: Some("G2".to_string()),
    });

    // Act
    let result = DecodeTarget::try_new(params);

    // Assert
    assert_eq!(result, Ok(DecodeTarget::G2));
}
// decode_target_admits_the_name_g2

/// Contract: `params.text` equals the `as_str()` of `Scalar` → `Ok(DecodeTarget::Scalar)`.
/// Arrange: `build_decode_target_constructor_params` with `text` `"Scalar"`.
/// Act:    `DecodeTarget::try_new` over it.
/// Assert: the result equals `Ok(DecodeTarget::Scalar)`.
#[test]
fn decode_target_admits_the_name_scalar() {
    // Arrange
    let params = build_decode_target_constructor_params(DecodeTargetConstructorParamsOverrides {
        text: Some("Scalar".to_string()),
    });

    // Act
    let result = DecodeTarget::try_new(params);

    // Assert
    assert_eq!(result, Ok(DecodeTarget::Scalar));
}
// decode_target_admits_the_name_scalar

/// Contract: `params.text` equals no variant's `as_str()` →
///   `Err(DecodeTargetTryNewErrorReturn::Unknown { text })`, the text moved.
/// Arrange: `build_decode_target_constructor_params` with `text` `"G3"`.
/// Act:    `DecodeTarget::try_new` over it.
/// Assert: the result equals `Err(DecodeTargetTryNewErrorReturn::Unknown { text: "G3".into() })`.
#[test]
fn decode_target_refuses_an_unknown_name() {
    // Arrange
    let params = build_decode_target_constructor_params(DecodeTargetConstructorParamsOverrides {
        text: Some("G3".to_string()),
    });

    // Act
    let result = DecodeTarget::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(DecodeTargetTryNewErrorReturn::Unknown { text: "G3".into() })
    );
}
// decode_target_refuses_an_unknown_name

/// Contract: fieldless params → `Ok(RejectionVectorDescription)`.
/// Arrange: `build_rejection_vector_description_constructor_params(Default::default())`.
/// Act:    `RejectionVectorDescription::try_new` over it.
/// Assert: the result is `Ok(RejectionVectorDescription)`.
#[test]
fn rejection_vector_description_is_admitted_from_its_fieldless_params() {
    // Arrange
    let params = build_rejection_vector_description_constructor_params(Default::default());

    // Act
    let result = RejectionVectorDescription::try_new(params);

    // Assert
    assert!(matches!(result, Ok(RejectionVectorDescription)));
}
// rejection_vector_description_is_admitted_from_its_fieldless_params

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose `values` are
///   `CanonicalFieldValue::Text` of `target.as_str()` and `CanonicalFieldValue::Bytes`
///   of a clone of `input`, in that order.
/// Arrange: `build_rejection_vector` with `target` `DecodeTarget::Scalar` and `input`
///   `vec![0x0A, 0x0B]`, and `build_rejection_vector_description(Default::default())`.
/// Act:    `to_fields` over `build_to_fields_params(Default::default())` and the record.
/// Assert: `fields` equals `build_canonical_fields` with `values` `Text("Scalar")` then
///   `Bytes(vec![0x0A, 0x0B])`.
#[test]
fn rejection_vector_description_maps_target_and_input_in_order() {
    // Arrange
    let vector = build_rejection_vector(RejectionVectorOverrides {
        target: Some(DecodeTarget::Scalar),
        input: Some(vec![0x0A, 0x0B]),
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let fields = description
        .to_fields(build_to_fields_params(Default::default()), &vector)
        .expect("the description maps the record")
        .fields;

    // Assert
    assert_eq!(
        fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Text("Scalar".to_string()),
                CanonicalFieldValue::Bytes(vec![0x0A, 0x0B]),
            ]),
        })
    );
}
// rejection_vector_description_maps_target_and_input_in_order

/// Contract: `payload.values` converts into the field array and every value holds the
///   kind `REJECTION_VECTOR_FIELD_KINDS` declares → `Ok(FromFieldsSuccessReturn {
///   described: RejectionVector { target, input } })`, each moved from its field.
/// Arrange: `build_canonical_fields` with `values` `Text("G1")` and `Bytes(vec![0x0C])`,
///   and `build_rejection_vector_description(Default::default())`.
/// Act:    `fields_to_value` over `build_from_fields_params(Default::default())`.
/// Assert: `described` equals `build_rejection_vector` with `DecodeTarget::G1` and
///   `vec![0x0C]`.
#[test]
fn rejection_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("G1".to_string()),
            CanonicalFieldValue::Bytes(vec![0x0C]),
        ]),
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let described = description
        .fields_to_value(build_from_fields_params(Default::default()), payload)
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(
        described,
        build_rejection_vector(RejectionVectorOverrides {
            target: Some(DecodeTarget::G1),
            input: Some(vec![0x0C]),
        })
    );
}
// rejection_vector_description_rebuilds_the_record_from_its_fields

/// Contract: `payload.values` does not convert into the field array →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected: 2, actual: 1 })`.
/// Arrange: `build_canonical_fields` with `values` `Text("G1")` alone.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldCount { expected: 2, actual: 1 }`.
#[test]
fn rejection_vector_description_refuses_too_few_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![CanonicalFieldValue::Text("G1".to_string())]),
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(RejectionVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1,
        })
    );
}
// rejection_vector_description_refuses_too_few_fields

/// Contract: `payload.values` does not convert into the field array →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected: 2, actual: 3 })`.
/// Arrange: `build_canonical_fields` with `values` `Text("G1")`, `Bytes(vec![0x0C])`,
///   and `Bytes(vec![0x0D])`.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldCount { expected: 2, actual: 3 }`.
#[test]
fn rejection_vector_description_refuses_too_many_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("G1".to_string()),
            CanonicalFieldValue::Bytes(vec![0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(RejectionVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 3,
        })
    );
}
// rejection_vector_description_refuses_too_many_fields

/// Contract: the lowest index `i` whose value is not the kind
///   `REJECTION_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`.
/// Arrange: `invalidate_rejection_vector` with `input`
///   `CanonicalFieldValue::Text("input")`.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldKind { index: 1, expected: CanonicalFieldKind::Bytes }`.
#[test]
fn rejection_vector_description_refuses_a_field_of_the_wrong_kind() {
    // Arrange
    let payload = invalidate_rejection_vector(RejectionVectorCorruptions {
        input: Some(CanonicalFieldValue::Text("input".to_string())),
        ..Default::default()
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(RejectionVectorFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
        })
    );
}
// rejection_vector_description_refuses_a_field_of_the_wrong_kind

/// Contract: the lowest index `i` whose value is not the kind
///   `REJECTION_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`.
/// Arrange: `invalidate_rejection_vector` with `target` `CanonicalFieldValue::Bytes(vec![0x0C])`
///   and `input` `CanonicalFieldValue::Text("input")`, each the other field's kind.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldKind { index: 0, expected: CanonicalFieldKind::Text }`.
#[test]
fn rejection_vector_description_reports_the_lowest_index_of_a_wrong_kind() {
    // Arrange
    let payload = invalidate_rejection_vector(RejectionVectorCorruptions {
        target: Some(CanonicalFieldValue::Bytes(vec![0x0C])),
        input: Some(CanonicalFieldValue::Text("input".to_string())),
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(RejectionVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Text,
        })
    );
}
// rejection_vector_description_reports_the_lowest_index_of_a_wrong_kind

/// Contract: `payload.values` does not convert into the field array →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected, actual })`,
///   decided before the kind check.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0C])`, `Text("input")`,
///   and `Text("G1")`, the first of the wrong kind.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldCount { expected: 2, actual: 3 }`.
#[test]
fn rejection_vector_description_reports_the_count_before_the_kind() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0C]),
            CanonicalFieldValue::Text("input".to_string()),
            CanonicalFieldValue::Text("G1".to_string()),
        ]),
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(RejectionVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 3,
        })
    );
}
// rejection_vector_description_reports_the_count_before_the_kind

/// Contract: the first field's `CanonicalFieldValue::Text` names no variant →
///   `Err(RejectionVectorFromFieldsErrorReturn::Target(error))`, the refusal unchanged.
/// Arrange: `invalidate_rejection_vector` with `target`
///   `CanonicalFieldValue::Text("G3")`.
/// Act:    `fields_to_value`.
/// Assert: the error equals `RejectionVectorFromFieldsErrorReturn::Target(
///   DecodeTargetTryNewErrorReturn::Unknown { text: "G3".into() })`.
#[test]
fn rejection_vector_description_refuses_an_unknown_target() {
    // Arrange
    let payload = invalidate_rejection_vector(RejectionVectorCorruptions {
        target: Some(CanonicalFieldValue::Text("G3".to_string())),
        ..Default::default()
    });
    let description = build_rejection_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(RejectionVectorFromFieldsErrorReturn::Target(
            DecodeTargetTryNewErrorReturn::Unknown { text: "G3".into() }
        ))
    );
}
// rejection_vector_description_refuses_an_unknown_target

/// Contract: each name in `REJECTION_VECTOR_MEMBER_NAMES` that
///   `SolidityMemberName::try_new` admits → `Ok` of the names in order.
/// Arrange: `build_rejection_vector_description(Default::default())`.
/// Act:    `member_names()`.
/// Assert: the result equals `Ok` of the names `target` then `input`, each through
///   `build_solidity_member_name`.
#[test]
fn rejection_vector_description_declares_target_and_input_member_names() {
    // Arrange
    let description = build_rejection_vector_description(Default::default());

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(
        result,
        Ok(vec![
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("target".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("input".to_string()),
            }),
        ])
    );
}
// rejection_vector_description_declares_target_and_input_member_names

/// Contract: the first group's `bytes` is `None` → no `G1` record; outcome `Ok` whose
///   `records` hold the `G2` and `Scalar` records in order.
/// Arrange: the default params, whose mock concrete returns no first-group encoding.
/// Act:    `rejection_vectors` in the consumer's method.
/// Assert: the targets of `records`, in order, equal `[DecodeTarget::G2,
///   DecodeTarget::Scalar]`.
#[test]
fn rejection_vectors_enumerate_the_second_group_and_the_scalar_where_the_pairing_has_no_first_group_encoding()
 {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = RejectionVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer;

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );

    // Assert
    let success = created
        .expect("the factory admits the mock selection")
        .output
        .expect("the enumeration succeeds");
    let targets: Vec<DecodeTarget> = success.records.iter().map(|record| record.target).collect();
    assert_eq!(targets, vec![DecodeTarget::G2, DecodeTarget::Scalar]);
}
// rejection_vectors_enumerate_the_second_group_and_the_scalar_where_the_pairing_has_no_first_group_encoding

/// Contract: the `G2` record's `input` is the second-group candidate's bytes.
/// Arrange: the default params, with a consumer whose `Output` pairs the bytes of the
///   received pairing's `g2_outside_subgroup_encoding`, built in its Arrange, with the
///   return; the mock concrete's second-group candidate and scalar field order are
///   different byte strings, so a record carrying the other's bytes fails.
/// Act:    `rejection_vectors` in the consumer's method.
/// Assert: the `input` of the record whose target is `DecodeTarget::G2` equals those
///   bytes.
#[test]
fn rejection_vectors_hold_the_second_group_candidate_as_the_g2_record_input() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (Vec<u8>, RejectionVectorsReturn);

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let candidate = payload
                .adapter
                .g2_outside_subgroup_encoding(
                    G2OutsideSubgroupEncodingParams,
                    G2OutsideSubgroupEncodingPayload,
                )
                .expect("the search succeeds")
                .bytes
                .as_ref()
                .to_vec();
            let result = rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            );
            (candidate, result)
        }
    }

    let consumer = Consumer;

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );

    // Assert
    let (candidate, result) = created
        .expect("the factory admits the mock selection")
        .output;
    let success = result.expect("the enumeration succeeds");
    let record = success
        .records
        .iter()
        .find(|record| record.target == DecodeTarget::G2)
        .expect("a G2 record exists");
    assert_eq!(record.input, candidate);
}
// rejection_vectors_hold_the_second_group_candidate_as_the_g2_record_input

/// Contract: the `Scalar` record's `input` is the scalar field order's bytes.
/// Arrange: the default params, with a consumer whose `Output` pairs the bytes of the
///   received pairing's `scalar_field_order`, built in its Arrange, with the return.
/// Act:    `rejection_vectors` in the consumer's method.
/// Assert: the `input` of the record whose target is `DecodeTarget::Scalar` equals
///   those bytes.
#[test]
fn rejection_vectors_hold_the_scalar_field_order_as_the_scalar_record_input() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (Vec<u8>, RejectionVectorsReturn);

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let order = payload
                .adapter
                .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)
                .expect("the order is infallible")
                .bytes;
            let result = rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            );
            (order, result)
        }
    }

    let consumer = Consumer;

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );

    // Assert
    let (order, result) = created
        .expect("the factory admits the mock selection")
        .output;
    let success = result.expect("the enumeration succeeds");
    let record = success
        .records
        .iter()
        .find(|record| record.target == DecodeTarget::Scalar)
        .expect("a Scalar record exists");
    assert_eq!(record.input, order);
}
// rejection_vectors_hold_the_scalar_field_order_as_the_scalar_record_input

/// Contract: the first-group candidate search returns `Err(error)` →
///   `Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(error))`, the error
///   unchanged.
/// Arrange: params naming
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted)`.
/// Act:    `rejection_vectors` in the consumer's method.
/// Assert: the result equals `Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(
///   G1OutsideSubgroupEncodingErrorReturn::SearchExhausted))`.
#[test]
fn rejection_vectors_return_the_first_group_refusal_unchanged() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = RejectionVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer;

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted,
            )),
            ..Default::default()
        }),
        build_create_pairing_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("the factory admits the mock selection")
        .output;
    assert_eq!(
        result,
        Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(
            G1OutsideSubgroupEncodingErrorReturn::SearchExhausted
        ))
    );
}
// rejection_vectors_return_the_first_group_refusal_unchanged

/// Contract: the second-group candidate search returns `Err(error)` →
///   `Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(error))`, the error
///   unchanged.
/// Arrange: params naming
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted)`.
/// Act:    `rejection_vectors` in the consumer's method.
/// Assert: the result equals `Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(
///   G2OutsideSubgroupEncodingErrorReturn::SearchExhausted))`.
#[test]
fn rejection_vectors_return_the_second_group_refusal_unchanged() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = RejectionVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer;

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted,
            )),
            ..Default::default()
        }),
        build_create_pairing_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("the factory admits the mock selection")
        .output;
    assert_eq!(
        result,
        Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(
            G2OutsideSubgroupEncodingErrorReturn::SearchExhausted
        ))
    );
}
// rejection_vectors_return_the_second_group_refusal_unchanged
