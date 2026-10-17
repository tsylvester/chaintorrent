#![allow(clippy::expect_used)]

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params, build_to_fields_params,
};

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use super::interface::{ChallengeVectorDescription, ChallengeVectorFromFieldsErrorReturn};
use super::mock::{
    ChallengeVectorOverrides, build_challenge_vector, build_challenge_vector_description,
    build_challenge_vector_description_constructor_params,
};

/// Contract: `ChallengeVectorDescription::try_new` over its constructor
///   params returns `Ok(ChallengeVectorDescription)`.
/// Arrange:  `build_challenge_vector_description_constructor_params` at its
///   defaults.
/// Act:      `ChallengeVectorDescription::try_new` over the params.
/// Assert:   `result.is_ok()`.
/// Collaborators: none — the description is a leaf.
#[test]
fn challenge_vector_description_is_constructed_from_its_params() {
    // Arrange
    let params = build_challenge_vector_description_constructor_params(Default::default());

    // Act
    let result = ChallengeVectorDescription::try_new(params);

    // Assert
    assert!(result.is_ok());
    // challenge_vector_description_is_constructed_from_its_params
}

/// Contract: `to_fields` returns, in member order,
///   `CanonicalFieldValue::Unsigned16` of `purpose`,
///   `CanonicalFieldValue::Bytes` of `draws` and of `encoding`, and
///   `CanonicalFieldValue::Unsigned256` of `challenge`.
/// Arrange:  `build_challenge_vector` with `purpose` `2`,
///   `draws` `vec![0x0A]`, `encoding` `vec![0x0B, 0x0C]`, and `challenge`
///   `[0x0D; 32]`, every value different from every other and from the
///   builder default; the description from
///   `build_challenge_vector_description(Default::default())`.
/// Act:      `to_fields` on the description over a reference to the record.
/// Assert:   `success.fields.values` equals the four literals, in that
///   order.
/// Collaborators: none — the description is a leaf.
#[test]
fn challenge_vector_description_maps_purpose_draws_encoding_and_challenge_in_member_order() {
    // Arrange
    let vector = build_challenge_vector(ChallengeVectorOverrides {
        purpose: Some(2),
        draws: Some(vec![0x0A]),
        encoding: Some(vec![0x0B, 0x0C]),
        challenge: Some([0x0D; 32]),
    });
    let description = build_challenge_vector_description(Default::default());
    let params = build_to_fields_params(Default::default());

    // Act
    let success = description
        .to_fields(params, &vector)
        .expect("the description maps every record");

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Unsigned16(2),
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ]
    );
    // challenge_vector_description_maps_purpose_draws_encoding_and_challenge_in_member_order
}

/// Contract: `fields_to_value` over fields of the declared kinds returns
///   `Ok` holding a `ChallengeVector` whose members are the fields' values,
///   each moved from its field.
/// Arrange:  `build_canonical_fields` with `values`
///   `CanonicalFieldValue::Unsigned16(2)`, `CanonicalFieldValue::Bytes(
///   vec![0x0A])`, `CanonicalFieldValue::Bytes(vec![0x0B, 0x0C])`, and
///   `CanonicalFieldValue::Unsigned256([0x0D; 32])`; the description from
///   `build_challenge_vector_description(Default::default())`.
/// Act:      `fields_to_value` on the description over the fields.
/// Assert:   `success.described` equals `build_challenge_vector` with
///   `purpose` `2`, `draws` `vec![0x0A]`, `encoding` `vec![0x0B, 0x0C]`,
///   and `challenge` `[0x0D; 32]`.
/// Collaborators: none — the description is a leaf.
#[test]
fn challenge_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned16(2),
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ]),
    });
    let description = build_challenge_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let success = description
        .fields_to_value(params, fields)
        .expect("fields of the declared kinds rebuild the record");

    // Assert
    assert_eq!(
        success.described,
        build_challenge_vector(ChallengeVectorOverrides {
            purpose: Some(2),
            draws: Some(vec![0x0A]),
            encoding: Some(vec![0x0B, 0x0C]),
            challenge: Some([0x0D; 32]),
        })
    );
    // challenge_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(ChallengeVectorFromFieldsErrorReturn::FieldCount { expected:
///   CHALLENGE_VECTOR_FIELD_COUNT, actual })`, `actual` the list's
///   length.
/// Arrange:  `build_canonical_fields` with the first three values of
///   `challenge_vector_description_rebuilds_the_record_from_its_fields`,
///   one short of the declared count; the description from
///   `build_challenge_vector_description(Default::default())`.
/// Act:      `fields_to_value` on the description over the fields.
/// Assert:   `result.err()` equals `Some(
///   ChallengeVectorFromFieldsErrorReturn::FieldCount { expected: 4,
///   actual: 3 })`.
/// Collaborators: none — the description is a leaf.
#[test]
fn challenge_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned16(2),
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
        ]),
    });
    let description = build_challenge_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(ChallengeVectorFromFieldsErrorReturn::FieldCount {
            expected: 4,
            actual: 3,
        })
    );
    // challenge_vector_description_refuses_a_wrong_field_count
}

/// Contract: `payload.values` holds at the lowest index `i` a value that is
///   not the kind `CHALLENGE_VECTOR_FIELD_KINDS` declares at `i` →
///   `Err(ChallengeVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`, `expected` that kind.
/// Arrange:  `build_canonical_fields` with the four values of
///   `challenge_vector_description_rebuilds_the_record_from_its_fields`,
///   the value at index `0` replaced by
///   `CanonicalFieldValue::Unsigned64(2)` and the value at index `3` by
///   `CanonicalFieldValue::Bytes(vec![0x0D])`, so two indices are of the
///   wrong kind and only the lowest is reported; the description from
///   `build_challenge_vector_description(Default::default())`.
/// Act:      `fields_to_value` on the description over the fields.
/// Assert:   `result.err()` equals `Some(
///   ChallengeVectorFromFieldsErrorReturn::FieldKind { index: 0,
///   expected: CanonicalFieldKind::Unsigned16 })`.
/// Collaborators: none — the description is a leaf.
#[test]
fn challenge_vector_description_refuses_a_field_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned64(2),
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_challenge_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(ChallengeVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Unsigned16,
        })
    );
    // challenge_vector_description_refuses_a_field_of_the_wrong_kind
}

/// Contract: `members()` returns `Ok` holding the record members `purpose`,
///   `draws`, `encoding`, and `challenge`, in that order, each of its
///   field's kind.
/// Arrange:  `build_solidity_record_members` over
///   `build_solidity_record_member` for `purpose` of kind
///   `CanonicalFieldKind::Unsigned16`, `draws` and `encoding` of kind
///   `CanonicalFieldKind::Bytes`, and `challenge` of kind
///   `CanonicalFieldKind::Unsigned256`, in that order, each name from
///   `build_solidity_member_name`, built as the expectation; the
///   description from
///   `build_challenge_vector_description(Default::default())`.
/// Act:      `members()` on the description.
/// Assert:   `result.ok()` equals `Some` of the arranged members.
/// Collaborators: none — the description is a leaf.
#[test]
fn challenge_vector_description_declares_its_members() {
    // Arrange
    let description = build_challenge_vector_description(Default::default());
    let expected = build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
        members: Some(vec![
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("purpose".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned16),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("draws".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("encoding".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("challenge".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned256),
            }),
        ]),
    });

    // Act
    let result = description.members();

    // Assert
    assert_eq!(result.ok(), Some(expected));
    // challenge_vector_description_declares_its_members
}
