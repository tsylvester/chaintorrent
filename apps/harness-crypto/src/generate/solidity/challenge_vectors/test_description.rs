#![allow(clippy::expect_used)]

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params, build_to_fields_params,
};

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::interface::{ChallengeVectorDescription, ChallengeVectorFromFieldsErrorReturn};
use super::mock::{
    ChallengeVectorOverrides, build_challenge_vector, build_challenge_vector_description,
    build_challenge_vector_description_constructor_params,
};

/// Contract: `ChallengeVectorDescription::try_new` over its constructor params
///   returns `Ok(ChallengeVectorDescription)`.
/// Arrange: `build_challenge_vector_description_constructor_params` at its
///   defaults.
/// Act:    `ChallengeVectorDescription::try_new` over the params.
/// Assert: the result is `Ok`.
#[test]
fn challenge_vector_description_is_constructed_from_its_params() {
    // Arrange
    let params = build_challenge_vector_description_constructor_params(Default::default());

    // Act
    let result = ChallengeVectorDescription::try_new(params);

    // Assert
    assert!(result.is_ok());
}
// challenge_vector_description_is_constructed_from_its_params

/// Contract: `to_fields` returns, in member order,
///   `CanonicalFieldValue::Unsigned16` of `purpose`, `CanonicalFieldValue::Bytes`
///   of `draws` and of `encoding`, and `CanonicalFieldValue::Unsigned256` of
///   `challenge`.
/// Arrange: `build_challenge_vector` with `purpose` `2`, `draws` `vec![0x0A]`,
///   `encoding` `vec![0x0B, 0x0C]`, and `challenge` `[0x0D; 32]`, every value
///   different from every other and from the builder default, and
///   `build_challenge_vector_description(Default::default())`.
/// Act:    `to_fields` over `build_to_fields_params` at its defaults and the
///   record.
/// Assert: the success's `fields.values` equal
///   `CanonicalFieldValue::Unsigned16(2)`, `CanonicalFieldValue::Bytes(vec![0x0A])`,
///   `CanonicalFieldValue::Bytes(vec![0x0B, 0x0C])`, and
///   `CanonicalFieldValue::Unsigned256([0x0D; 32])`, in that order.
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
                CanonicalFieldValue::Unsigned16(2),
                CanonicalFieldValue::Bytes(vec![0x0A]),
                CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
                CanonicalFieldValue::Unsigned256([0x0D; 32]),
            ]),
        })
    );
}
// challenge_vector_description_maps_purpose_draws_encoding_and_challenge_in_member_order

/// Contract: `fields_to_value` over fields of the declared kinds returns `Ok`
///   holding a `ChallengeVector` whose members are the fields' values, each
///   moved from its field.
/// Arrange: `build_canonical_fields` with `values`
///   `CanonicalFieldValue::Unsigned16(2)`, `CanonicalFieldValue::Bytes(vec![0x0A])`,
///   `CanonicalFieldValue::Bytes(vec![0x0B, 0x0C])`, and
///   `CanonicalFieldValue::Unsigned256([0x0D; 32])`, and
///   `build_challenge_vector_description(Default::default())`.
/// Act:    `fields_to_value` over `build_from_fields_params` at its defaults.
/// Assert: the success's `described` equals `build_challenge_vector` with
///   `purpose` `2`, `draws` `vec![0x0A]`, `encoding` `vec![0x0B, 0x0C]`, and
///   `challenge` `[0x0D; 32]`.
#[test]
fn challenge_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned16(2),
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ]),
    });
    let description = build_challenge_vector_description(Default::default());

    // Act
    let described = description
        .fields_to_value(build_from_fields_params(Default::default()), payload)
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(
        described,
        build_challenge_vector(ChallengeVectorOverrides {
            purpose: Some(2),
            draws: Some(vec![0x0A]),
            encoding: Some(vec![0x0B, 0x0C]),
            challenge: Some([0x0D; 32]),
        })
    );
}
// challenge_vector_description_rebuilds_the_record_from_its_fields

/// Contract: `fields_to_value` over a field list whose length is not the
///   declared count returns
///   `Err(ChallengeVectorFromFieldsErrorReturn::FieldCount { expected, actual })`,
///   `actual` the list's length.
/// Arrange: `build_canonical_fields` with the first three values of the
///   previous block, one short of the declared count, and
///   `build_challenge_vector_description(Default::default())`.
/// Act:    `fields_to_value` over `build_from_fields_params` at its defaults.
/// Assert: `result.err()` equals
///   `Some(ChallengeVectorFromFieldsErrorReturn::FieldCount { expected: 4,
///   actual: 3 })`.
#[test]
fn challenge_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned16(2),
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
        ]),
    });
    let description = build_challenge_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(ChallengeVectorFromFieldsErrorReturn::FieldCount {
            expected: 4,
            actual: 3,
        })
    );
}
// challenge_vector_description_refuses_a_wrong_field_count

/// Contract: `fields_to_value` over a field list whose value at the lowest
///   index `i` is not the kind declared at `i` returns
///   `Err(ChallengeVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`.
/// Arrange: `build_canonical_fields` with the four values of the previous
///   block, the value at index `0` replaced by
///   `CanonicalFieldValue::Unsigned64(2)` and the value at index `3` by
///   `CanonicalFieldValue::Bytes(vec![0x0D])`, so two indices are of the wrong
///   kind and only the lowest is reported, and
///   `build_challenge_vector_description(Default::default())`.
/// Act:    `fields_to_value` over `build_from_fields_params` at its defaults.
/// Assert: `result.err()` equals
///   `Some(ChallengeVectorFromFieldsErrorReturn::FieldKind { index: 0,
///   expected: CanonicalFieldKind::Unsigned16 })`.
#[test]
fn challenge_vector_description_refuses_a_field_of_the_wrong_kind() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned64(2),
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_challenge_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(ChallengeVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Unsigned16,
        })
    );
}
// challenge_vector_description_refuses_a_field_of_the_wrong_kind

/// Contract: `member_names()` returns `Ok` holding the names `purpose`,
///   `draws`, `encoding`, and `challenge`, in that order.
/// Arrange: the names `purpose`, `draws`, `encoding`, and `challenge`, in that
///   order, each from `build_solidity_member_name`, built as the expectation,
///   and `build_challenge_vector_description(Default::default())`.
/// Act:    `member_names()` on the description.
/// Assert: `result.ok()` equals `Some` of the arranged names.
#[test]
fn challenge_vector_description_declares_its_member_names() {
    // Arrange
    let names = vec![
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("purpose".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("draws".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("encoding".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("challenge".to_string()),
        }),
    ];
    let description = build_challenge_vector_description(Default::default());

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(result.ok(), Some(names));
}
// challenge_vector_description_declares_its_member_names
