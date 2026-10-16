#![allow(clippy::expect_used)]

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use super::interface::*;
use super::mock::*;
use super::rejection_vectors;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, FromFieldsParamsOverrides,
    IEncodingContract, ToFieldsParamsOverrides, build_canonical_fields, build_from_fields_params,
    build_to_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDepsOverrides,
    CreatePairingParamsOverrides, CreatePairingPayloadOverrides,
    G1OutsideSubgroupEncodingErrorReturn, G2OutsideSubgroupEncodingErrorReturn,
    G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, MockIPairingAdapterFailureMode, PairingConcrete,
    ScalarFieldOrderParams, ScalarFieldOrderPayload, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};

/// Contract: `as_str()` is exhaustive, `G1` to `"G1"`.
/// Arrange:  `DecodeTarget::G1`.
/// Act:      `as_str`.
/// Assert:   the text equals `"G1"`.
/// Collaborators: none — the method is a leaf.
#[test]
fn decode_target_g1_is_named_g1() {
    // Arrange
    let target = DecodeTarget::G1;

    // Act
    let text = target.as_str();

    // Assert
    assert_eq!(text, "G1");
    // decode_target_g1_is_named_g1
}

/// Contract: `as_str()` is exhaustive, `G2` to `"G2"`.
/// Arrange:  `DecodeTarget::G2`.
/// Act:      `as_str`.
/// Assert:   the text equals `"G2"`.
/// Collaborators: none — the method is a leaf.
#[test]
fn decode_target_g2_is_named_g2() {
    // Arrange
    let target = DecodeTarget::G2;

    // Act
    let text = target.as_str();

    // Assert
    assert_eq!(text, "G2");
    // decode_target_g2_is_named_g2
}

/// Contract: `as_str()` is exhaustive, `Scalar` to `"Scalar"`.
/// Arrange:  `DecodeTarget::Scalar`.
/// Act:      `as_str`.
/// Assert:   the text equals `"Scalar"`.
/// Collaborators: none — the method is a leaf.
#[test]
fn decode_target_scalar_is_named_scalar() {
    // Arrange
    let target = DecodeTarget::Scalar;

    // Act
    let text = target.as_str();

    // Assert
    assert_eq!(text, "Scalar");
    // decode_target_scalar_is_named_scalar
}

/// Contract: `params.text` equals `G1`'s `as_str()` → `Ok(DecodeTarget::G1)`.
/// Arrange:  `build_decode_target_constructor_params` with `text` `"G1"`.
/// Act:      `DecodeTarget::try_new` over it.
/// Assert:   the result equals `Ok(DecodeTarget::G1)`.
/// Collaborators: none — the constructor is a leaf.
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
    // decode_target_admits_the_name_g1
}

/// Contract: `params.text` equals `G2`'s `as_str()` → `Ok(DecodeTarget::G2)`.
/// Arrange:  `build_decode_target_constructor_params` with `text` `"G2"`.
/// Act:      `DecodeTarget::try_new` over it.
/// Assert:   the result equals `Ok(DecodeTarget::G2)`.
/// Collaborators: none — the constructor is a leaf.
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
    // decode_target_admits_the_name_g2
}

/// Contract: `params.text` equals `Scalar`'s `as_str()` → `Ok(DecodeTarget::Scalar)`.
/// Arrange:  `build_decode_target_constructor_params` with `text` `"Scalar"`.
/// Act:      `DecodeTarget::try_new` over it.
/// Assert:   the result equals `Ok(DecodeTarget::Scalar)`.
/// Collaborators: none — the constructor is a leaf.
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
    // decode_target_admits_the_name_scalar
}

/// Contract: `params.text` equals no variant's `as_str()` →
///   `Err(DecodeTargetTryNewErrorReturn::Unknown { text })`, the text moved.
/// Arrange:  `build_decode_target_constructor_params` with `text` `"G3"`.
/// Act:      `DecodeTarget::try_new` over it.
/// Assert:   the result equals `Err(DecodeTargetTryNewErrorReturn::Unknown {
///   text: "G3".into() })`.
/// Collaborators: none — the constructor is a leaf.
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
    // decode_target_refuses_an_unknown_name
}

/// Contract: the constructor's one branch → `Ok(RejectionVectorDescription)`.
/// Arrange:  `build_rejection_vector_description_constructor_params(Default::
///   default())`.
/// Act:      `RejectionVectorDescription::try_new` over it.
/// Assert:   the result is `Ok(RejectionVectorDescription)`.
/// Collaborators: none — the constructor is a leaf.
#[test]
fn rejection_vector_description_is_admitted_from_its_fieldless_params() {
    // Arrange
    let params = build_rejection_vector_description_constructor_params(Default::default());

    // Act
    let result = RejectionVectorDescription::try_new(params);

    // Assert
    assert!(matches!(result, Ok(RejectionVectorDescription)));
    // rejection_vector_description_is_admitted_from_its_fieldless_params
}

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose
///   `values` are `CanonicalFieldValue::Text` of `target.as_str()` and
///   `CanonicalFieldValue::Bytes` of a clone of `input`, in that order.
/// Arrange:  `build_rejection_vector` with `target` `DecodeTarget::Scalar`
///   and `input` `vec![0x0A, 0x0B]`,
///   `build_rejection_vector_description(Default::default())`, and
///   `build_to_fields_params(Default::default())`.
/// Act:      `to_fields` over the params and the record.
/// Assert:   `fields` equals `build_canonical_fields` with `values`
///   `CanonicalFieldValue::Text("Scalar".into())` then
///   `CanonicalFieldValue::Bytes(vec![0x0A, 0x0B])`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_maps_target_and_input_in_order() {
    // Arrange
    let record = build_rejection_vector(RejectionVectorOverrides {
        target: Some(DecodeTarget::Scalar),
        input: Some(vec![0x0A, 0x0B]),
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_to_fields_params(Default::default());

    // Act
    let Ok(success) = description.to_fields(params, &record);

    // Assert
    assert_eq!(
        success.fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Text("Scalar".into()),
                CanonicalFieldValue::Bytes(vec![0x0A, 0x0B]),
            ]),
        })
    );
    // rejection_vector_description_maps_target_and_input_in_order
}

/// Contract: `payload.values` converts into the field array and each value is
///   the kind `REJECTION_VECTOR_FIELD_KINDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described: RejectionVector { target, input
///   } })`, each moved from its field.
/// Arrange:  `build_canonical_fields` with `values` `Text("G1")` and
///   `Bytes(vec![0x0C])`, the description from
///   `build_rejection_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   `described` equals `build_rejection_vector` with
///   `DecodeTarget::G1` and `vec![0x0C]`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("G1".into()),
            CanonicalFieldValue::Bytes(vec![0x0C]),
        ]),
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let success = description
        .fields_to_value(params, fields)
        .expect("fields of the declared kinds rebuild the record");

    // Assert
    assert_eq!(
        success.described,
        build_rejection_vector(RejectionVectorOverrides {
            target: Some(DecodeTarget::G1),
            input: Some(vec![0x0C]),
        })
    );
    // rejection_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected:
///   REJECTION_VECTOR_FIELD_COUNT, actual })`.
/// Arrange:  `build_canonical_fields` with `values` `Text("G1")` alone, the
///   description, and `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   the error equals `RejectionVectorFromFieldsErrorReturn::
///   FieldCount { expected: 2, actual: 1 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_refuses_too_few_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![CanonicalFieldValue::Text("G1".into())]),
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a wrong field count is refused");

    // Assert
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1,
        }
    );
    // rejection_vector_description_refuses_too_few_fields
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected:
///   REJECTION_VECTOR_FIELD_COUNT, actual })`.
/// Arrange:  `build_canonical_fields` with `values` `Text("G1")`,
///   `Bytes(vec![0x0C])`, and `Bytes(vec![0x0D])`, the description, and
///   `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   the error equals `RejectionVectorFromFieldsErrorReturn::
///   FieldCount { expected: 2, actual: 3 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_refuses_too_many_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("G1".into()),
            CanonicalFieldValue::Bytes(vec![0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a wrong field count is refused");

    // Assert
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 3,
        }
    );
    // rejection_vector_description_refuses_too_many_fields
}

/// Contract: the lowest index `i` whose value is not the kind
///   `REJECTION_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`, `expected` that kind.
/// Arrange:  `invalidate_rejection_vector` with `input`
///   `CanonicalFieldValue::Text("input".into())`, so index `1` holds a wrong
///   kind, the description, and `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   the error equals `RejectionVectorFromFieldsErrorReturn::
///   FieldKind { index: 1, expected: CanonicalFieldKind::Bytes }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_refuses_a_field_of_the_wrong_kind() {
    // Arrange
    let fields = invalidate_rejection_vector(RejectionVectorCorruptions {
        input: Some(CanonicalFieldValue::Text("input".into())),
        ..Default::default()
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a field of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
        }
    );
    // rejection_vector_description_refuses_a_field_of_the_wrong_kind
}

/// Contract: the lowest index `i` whose value is not the kind
///   `REJECTION_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`, `expected` that kind.
/// Arrange:  `invalidate_rejection_vector` with `target`
///   `CanonicalFieldValue::Bytes(vec![0x0C])` and `input`
///   `CanonicalFieldValue::Text("input".into())`, each the other field's
///   kind, so the indexes `0` and `1` both hold a wrong kind, the
///   description, and `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   the error equals `RejectionVectorFromFieldsErrorReturn::
///   FieldKind { index: 0, expected: CanonicalFieldKind::Text }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_reports_the_lowest_index_of_a_wrong_kind() {
    // Arrange
    let fields = invalidate_rejection_vector(RejectionVectorCorruptions {
        target: Some(CanonicalFieldValue::Bytes(vec![0x0C])),
        input: Some(CanonicalFieldValue::Text("input".into())),
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a field of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Text,
        }
    );
    // rejection_vector_description_reports_the_lowest_index_of_a_wrong_kind
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected:
///   REJECTION_VECTOR_FIELD_COUNT, actual })`, counted before any kind is
///   checked.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0C])`,
///   `Text("input")`, and `Text("G1")`, the first of the wrong kind, the
///   description, and `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   the error equals `RejectionVectorFromFieldsErrorReturn::
///   FieldCount { expected: 2, actual: 3 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_reports_the_count_before_the_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0C]),
            CanonicalFieldValue::Text("input".into()),
            CanonicalFieldValue::Text("G1".into()),
        ]),
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a wrong field count is refused");

    // Assert
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 3,
        }
    );
    // rejection_vector_description_reports_the_count_before_the_kind
}

/// Contract: the first field's text names no variant →
///   `Err(RejectionVectorFromFieldsErrorReturn::Target(error))`, the
///   `DecodeTarget::try_new` refusal unchanged.
/// Arrange:  `invalidate_rejection_vector` with `target`
///   `CanonicalFieldValue::Text("G3".into())`, the description, and
///   `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   the error equals `RejectionVectorFromFieldsErrorReturn::Target(
///   DecodeTargetTryNewErrorReturn::Unknown { text: "G3".into() })`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_refuses_an_unknown_target() {
    // Arrange
    let fields = invalidate_rejection_vector(RejectionVectorCorruptions {
        target: Some(CanonicalFieldValue::Text("G3".into())),
        ..Default::default()
    });
    let description = build_rejection_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("an unknown target name is refused");

    // Assert
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::Target(DecodeTargetTryNewErrorReturn::Unknown {
            text: "G3".into()
        })
    );
    // rejection_vector_description_refuses_an_unknown_target
}

/// Contract: each name `SolidityMemberName::try_new` admits is paired with the
///   description's `FIELDS` kind at its index and
///   `SolidityRecordMembers::try_new` admits the members → `Ok` of the
///   members.
/// Arrange:  `build_rejection_vector_description(Default::default())`.
/// Act:      `members()`.
/// Assert:   the result equals `Ok` of `build_solidity_record_members` over
///   the members `target` of kind `CanonicalFieldKind::Text` and `input` of
///   kind `CanonicalFieldKind::Bytes`, each member through
///   `build_solidity_record_member` with its name from
///   `build_solidity_member_name`.
/// Collaborators: none — the description is a leaf.
#[test]
fn rejection_vector_description_declares_target_and_input_members() {
    // Arrange
    let description = build_rejection_vector_description(Default::default());
    let expected = build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
        members: Some(vec![
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("target".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Text),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("input".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
        ]),
    });

    // Act
    let result = description.members();

    // Assert
    assert_eq!(result, Ok(expected));
    // rejection_vector_description_declares_target_and_input_members
}

/// Contract: `bytes` `None` for the first group skips that record → the
///   records are the second group's candidate and the scalar field's order,
///   in that order.
/// Arrange:  the default params, whose mock concrete returns no first-group
///   encoding.
/// Act:      `rejection_vectors` in the consumer.
/// Assert:   the targets of `records`, in order, equal `[DecodeTarget::G2,
///   DecodeTarget::Scalar]`.
/// Collaborators: the pairing's mock concrete in its succeeding mode through
///   `create_pairing`.
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
            // Act
            rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    let records = success
        .output
        .expect("the mock pairing's searches succeed")
        .records;
    let targets = records
        .iter()
        .map(|record| record.target)
        .collect::<Vec<DecodeTarget>>();
    assert_eq!(targets, vec![DecodeTarget::G2, DecodeTarget::Scalar]);
    // rejection_vectors_enumerate_the_second_group_and_the_scalar_where_the_pairing_has_no_first_group_encoding
}

/// Contract: the second group's candidate is the `input` of the record whose
///   `target` is `DecodeTarget::G2`, unchanged.
/// Arrange:  the default params; the consumer pairs the bytes of the
///   received pairing's `g2_outside_subgroup_encoding` with the return. The
///   mock concrete's second-group candidate and scalar field order are
///   different byte strings, so a record carrying the other's bytes fails.
/// Act:      `rejection_vectors` in the consumer.
/// Assert:   the `input` of the record whose target is `DecodeTarget::G2`
///   equals those bytes.
/// Collaborators: the pairing's mock concrete in its succeeding mode through
///   `create_pairing`.
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
            // Arrange
            let expected = payload
                .adapter
                .g2_outside_subgroup_encoding(
                    G2OutsideSubgroupEncodingParams,
                    G2OutsideSubgroupEncodingPayload,
                )
                .expect("the mock pairing's second-group search succeeds")
                .bytes
                .as_ref()
                .to_vec();

            // Act
            let result = rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            );

            (expected, result)
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    let (expected, result) = success.output;
    let records = result.expect("the mock pairing's searches succeed").records;
    let record = records
        .iter()
        .find(|record| record.target == DecodeTarget::G2)
        .expect("the second-group record is emitted");
    assert_eq!(record.input, expected);
    // rejection_vectors_hold_the_second_group_candidate_as_the_g2_record_input
}

/// Contract: the scalar field order is the `input` of the record whose
///   `target` is `DecodeTarget::Scalar`, unchanged.
/// Arrange:  the default params; the consumer pairs the bytes of the
///   received pairing's `scalar_field_order` with the return.
/// Act:      `rejection_vectors` in the consumer.
/// Assert:   the `input` of the record whose target is `DecodeTarget::Scalar`
///   equals those bytes.
/// Collaborators: the pairing's mock concrete in its succeeding mode through
///   `create_pairing`.
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
            // Arrange
            let Ok(order) = payload
                .adapter
                .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
            let expected = order.bytes;

            // Act
            let result = rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            );

            (expected, result)
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    let (expected, result) = success.output;
    let records = result.expect("the mock pairing's searches succeed").records;
    let record = records
        .iter()
        .find(|record| record.target == DecodeTarget::Scalar)
        .expect("the scalar record is emitted");
    assert_eq!(record.input, expected);
    // rejection_vectors_hold_the_scalar_field_order_as_the_scalar_record_input
}

/// Contract: `g1_outside_subgroup_encoding` fails →
///   `Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(error))`,
///   the error unchanged.
/// Arrange:  params naming `PairingConcrete::Mock(MockIPairingAdapterFailure
///   Mode::G1OutsideSubgroupSearchExhausted)`.
/// Act:      `rejection_vectors` in the consumer.
/// Assert:   the result equals `Err(RejectionVectorsErrorReturn::
///   G1OutsideSubgroupEncoding(G1OutsideSubgroupEncodingErrorReturn::
///   SearchExhausted))`.
/// Collaborators: the pairing's mock concrete in its
///   `G1OutsideSubgroupSearchExhausted` mode through `create_pairing`.
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
            // Act
            rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted,
            )),
            ..Default::default()
        }),
        build_create_pairing_payload(Default::default()),
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    assert_eq!(
        success.output,
        Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(
            G1OutsideSubgroupEncodingErrorReturn::SearchExhausted
        ))
    );
    // rejection_vectors_return_the_first_group_refusal_unchanged
}

/// Contract: `g2_outside_subgroup_encoding` fails →
///   `Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(error))`,
///   the error unchanged.
/// Arrange:  params naming `PairingConcrete::Mock(MockIPairingAdapterFailure
///   Mode::G2OutsideSubgroupSearchExhausted)`.
/// Act:      `rejection_vectors` in the consumer.
/// Assert:   the result equals `Err(RejectionVectorsErrorReturn::
///   G2OutsideSubgroupEncoding(G2OutsideSubgroupEncodingErrorReturn::
///   SearchExhausted))`.
/// Collaborators: the pairing's mock concrete in its
///   `G2OutsideSubgroupSearchExhausted` mode through `create_pairing`.
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
            // Act
            rejection_vectors::<P>(
                &build_rejection_vectors_deps(&payload.adapter, Default::default()),
                build_rejection_vectors_params(Default::default()),
                build_rejection_vectors_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted,
            )),
            ..Default::default()
        }),
        build_create_pairing_payload(Default::default()),
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    assert_eq!(
        success.output,
        Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(
            G2OutsideSubgroupEncodingErrorReturn::SearchExhausted
        ))
    );
    // rejection_vectors_return_the_second_group_refusal_unchanged
}
