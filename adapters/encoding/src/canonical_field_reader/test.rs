#![allow(clippy::expect_used)]

use super::provides::{CanonicalFieldReader, CanonicalFieldReaderErrorReturn};
use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, FromFieldErrorReturn,
    MockICanonicalField, MockICanonicalFieldOverrides, build_canonical_fields,
    build_mock_i_canonical_field,
};

/// Contract: `CanonicalFieldValue::kind`, one total branch: every
///   `CanonicalFieldValue` variant → the like-named `CanonicalFieldKind`
///   variant.
/// Arrange: one value of each of the eight variants; the expected kinds are
///   written from the enums' declarations and not read back from the
///   arrangement.
/// Act:     `value.kind()` for each value.
/// Assert:  the result equals the expected kind for each value by `assert_eq!`,
///   each compared apart, so a variant taken to a sibling kind fails.
#[test]
fn canonical_field_value_kind_returns_the_like_named_kind() {
    // Arrange
    let fixed_bytes32 = CanonicalFieldValue::FixedBytes32([0x11; 32]);
    let unsigned16 = CanonicalFieldValue::Unsigned16(1);
    let unsigned32 = CanonicalFieldValue::Unsigned32(1);
    let unsigned64 = CanonicalFieldValue::Unsigned64(1);
    let text = CanonicalFieldValue::Text("x".to_owned());
    let bytes = CanonicalFieldValue::Bytes(vec![0x01]);
    let fixed_bytes20 = CanonicalFieldValue::FixedBytes20([0x22; 20]);
    let unsigned256 = CanonicalFieldValue::Unsigned256([0x01; 32]);

    // Act
    let fixed_bytes32_kind = fixed_bytes32.kind();
    let unsigned16_kind = unsigned16.kind();
    let unsigned32_kind = unsigned32.kind();
    let unsigned64_kind = unsigned64.kind();
    let text_kind = text.kind();
    let bytes_kind = bytes.kind();
    let fixed_bytes20_kind = fixed_bytes20.kind();
    let unsigned256_kind = unsigned256.kind();

    // Assert
    assert_eq!(fixed_bytes32_kind, CanonicalFieldKind::FixedBytes32);
    assert_eq!(unsigned16_kind, CanonicalFieldKind::Unsigned16);
    assert_eq!(unsigned32_kind, CanonicalFieldKind::Unsigned32);
    assert_eq!(unsigned64_kind, CanonicalFieldKind::Unsigned64);
    assert_eq!(text_kind, CanonicalFieldKind::Text);
    assert_eq!(bytes_kind, CanonicalFieldKind::Bytes);
    assert_eq!(fixed_bytes20_kind, CanonicalFieldKind::FixedBytes20);
    assert_eq!(unsigned256_kind, CanonicalFieldKind::Unsigned256);
}
// canonical_field_value_kind_returns_the_like_named_kind

/// Contract: `try_new`, wrong count →
///   `Err(CanonicalFieldReaderErrorReturn::Count { expected, actual })`.
/// Arrange: the field list `Text`, `Bytes`, and `FixedBytes32`;
///   `build_canonical_fields` overriding `values` with `Text("a")` and `Bytes`
///   of the byte `0x01`, each of its declared kind, so only the count differs.
/// Act:     `CanonicalFieldReader::try_new(payload, fields)`.
/// Assert:  `result.err()` equals `Some(CanonicalFieldReaderErrorReturn::Count
///   { expected: 3, actual: 2 })` by `assert_eq!`.
#[test]
fn try_new_refuses_a_list_one_value_short() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes32,
    ];
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("a".to_owned()),
            CanonicalFieldValue::Bytes(vec![0x01]),
        ]),
    });

    // Act
    let result = CanonicalFieldReader::try_new(payload, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(CanonicalFieldReaderErrorReturn::Count {
            expected: 3,
            actual: 2
        })
    );
}
// try_new_refuses_a_list_one_value_short

/// Contract: `try_new`, wrong count →
///   `Err(CanonicalFieldReaderErrorReturn::Count { expected, actual })`.
/// Arrange: the field list `Text`, `Bytes`, and `FixedBytes32`;
///   `build_canonical_fields` overriding `values` with `Text("a")`, `Bytes` of
///   the byte `0x01`, `FixedBytes32` of bytes each `0x11`, and `Unsigned64(0)`,
///   the first three of their declared kinds, so only the count differs.
/// Act:     `CanonicalFieldReader::try_new(payload, fields)`.
/// Assert:  `result.err()` equals `Some(CanonicalFieldReaderErrorReturn::Count
///   { expected: 3, actual: 4 })` by `assert_eq!`.
#[test]
fn try_new_refuses_a_list_one_value_long() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes32,
    ];
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("a".to_owned()),
            CanonicalFieldValue::Bytes(vec![0x01]),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::Unsigned64(0),
        ]),
    });

    // Act
    let result = CanonicalFieldReader::try_new(payload, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(CanonicalFieldReaderErrorReturn::Count {
            expected: 3,
            actual: 4
        })
    );
}
// try_new_refuses_a_list_one_value_long

/// Contract: ordering: `try_new` checks the count before any read compares a
///   kind.
/// Arrange: the field list `Text`, `Bytes`, and `FixedBytes32`;
///   `build_canonical_fields` overriding `values` with `Unsigned16(1)` and
///   `Bytes` of the byte `0x01`, a list both short and holding a wrong kind at
///   index `0`, so a kind check ahead of the count reports the kind.
/// Act:     `CanonicalFieldReader::try_new(payload, fields)`.
/// Assert:  `result.err()` equals `Some(CanonicalFieldReaderErrorReturn::Count
///   { expected: 3, actual: 2 })` by `assert_eq!`.
#[test]
fn try_new_checks_the_count_before_any_kind() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes32,
    ];
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Bytes(vec![0x01]),
        ]),
    });

    // Act
    let result = CanonicalFieldReader::try_new(payload, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(CanonicalFieldReaderErrorReturn::Count {
            expected: 3,
            actual: 2
        })
    );
}
// try_new_checks_the_count_before_any_kind

/// Contract: `read_value`, admitted → `Ok` of the value at `index`, and `index`
///   advanced.
/// Arrange: the field list `Text` and `Bytes`; the reader over `Text("a")` and
///   `Bytes` of the byte `0x01`, through `try_new` and bound by `expect`.
/// Act:     `reader.read_value()` twice.
/// Assert:  the first result equals `Ok(CanonicalFieldValue::Text("a"))` and
///   the second `Ok(CanonicalFieldValue::Bytes(vec![0x01]))` by `assert_eq!`,
///   each compared apart.
#[test]
fn read_value_returns_each_value_in_order() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] =
        &[CanonicalFieldKind::Text, CanonicalFieldKind::Bytes];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Text("a".to_owned()),
                CanonicalFieldValue::Bytes(vec![0x01]),
            ]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");

    // Act
    let first = reader.read_value();
    let second = reader.read_value();

    // Assert
    assert_eq!(first, Ok(CanonicalFieldValue::Text("a".to_owned())));
    assert_eq!(second, Ok(CanonicalFieldValue::Bytes(vec![0x01])));
}
// read_value_returns_each_value_in_order

/// Contract: `read_value`, wrong kind →
///   `Err(CanonicalFieldReaderErrorReturn::Kind { index, expected, actual })`.
/// Arrange: the field list `Text` and `Bytes`; the reader over `Unsigned16(1)`
///   and `Bytes` of the byte `0x01`, through `try_new` and bound by `expect`.
/// Act:     `reader.read_value()`.
/// Assert:  `result.err()` equals `Some(CanonicalFieldReaderErrorReturn::Kind
///   { index: 0, expected: CanonicalFieldKind::Text, actual:
///   CanonicalFieldKind::Unsigned16 })` by `assert_eq!`.
#[test]
fn read_value_refuses_a_wrong_kind_at_its_index() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] =
        &[CanonicalFieldKind::Text, CanonicalFieldKind::Bytes];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Unsigned16(1),
                CanonicalFieldValue::Bytes(vec![0x01]),
            ]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");

    // Act
    let result = reader.read_value();

    // Assert
    assert_eq!(
        result.err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::Text,
            actual: CanonicalFieldKind::Unsigned16
        })
    );
}
// read_value_refuses_a_wrong_kind_at_its_index

/// Contract: `read_value`, admitted, then wrong kind: the index advances past
///   each admitted value.
/// Arrange: the field list `Text` and `Bytes`; the reader over `Text("a")` and
///   `Text("b")`, through `try_new` and bound by `expect`.
/// Act:     `reader.read_value()` twice.
/// Assert:  the second result's `err()` equals
///   `Some(CanonicalFieldReaderErrorReturn::Kind { index: 1, expected:
///   CanonicalFieldKind::Bytes, actual: CanonicalFieldKind::Text })` by
///   `assert_eq!`.
#[test]
fn read_value_reports_the_index_after_a_successful_read() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] =
        &[CanonicalFieldKind::Text, CanonicalFieldKind::Bytes];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Text("a".to_owned()),
                CanonicalFieldValue::Text("b".to_owned()),
            ]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");

    // Act
    let first = reader.read_value();
    let second = reader.read_value();

    // Assert
    assert!(first.is_ok());
    assert_eq!(
        second.err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
            actual: CanonicalFieldKind::Text
        })
    );
}
// read_value_reports_the_index_after_a_successful_read

/// Contract: ordering: reads take values in index order, so the first refusal
///   met is at the lowest refused index.
/// Arrange: the field list `Text`, `Bytes`, and `FixedBytes32`; the reader
///   over `Text("a")`, `Unsigned16(1)`, and `Unsigned64(2)`, wrong at indexes
///   `1` and `2`, through `try_new` and bound by `expect`.
/// Act:     `reader.read_value()` until the first refusal.
/// Assert:  the first refusal equals `CanonicalFieldReaderErrorReturn::Kind
///   { index: 1, expected: CanonicalFieldKind::Bytes, actual:
///   CanonicalFieldKind::Unsigned16 }` by `assert_eq!`.
#[test]
fn reads_report_the_lowest_wrong_value() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes32,
    ];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Text("a".to_owned()),
                CanonicalFieldValue::Unsigned16(1),
                CanonicalFieldValue::Unsigned64(2),
            ]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");

    // Act
    let mut refusal = None;
    for _ in 0..fields.len() {
        if let Err(error) = reader.read_value() {
            refusal = Some(error);
            break;
        }
    }

    // Assert
    assert_eq!(
        refusal,
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
            actual: CanonicalFieldKind::Unsigned16
        })
    );
}
// reads_report_the_lowest_wrong_value

/// Contract: `read_value`, past the end →
///   `Err(CanonicalFieldReaderErrorReturn::Count { expected: fields.len(),
///   actual: index + 1 })`.
/// Arrange: the field list `Text`; the reader over `Text("a")`, through
///   `try_new` and bound by `expect`, and one admitted `read_value`.
/// Act:     `reader.read_value()` once more.
/// Assert:  `result.err()` equals `Some(CanonicalFieldReaderErrorReturn::Count
///   { expected: 1, actual: 2 })` by `assert_eq!`.
#[test]
fn read_value_refuses_a_read_past_the_last_field() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] = &[CanonicalFieldKind::Text];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Text("a".to_owned())]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");
    reader.read_value().expect("the only field reads");

    // Act
    let result = reader.read_value();

    // Assert
    assert_eq!(
        result.err(),
        Some(CanonicalFieldReaderErrorReturn::Count {
            expected: 1,
            actual: 2
        })
    );
}
// read_value_refuses_a_read_past_the_last_field

/// Contract: each `read_<kind>`, admitted → `Ok(contents)`.
/// Arrange: the field list `FixedBytes32`, `Unsigned16`, `Unsigned32`,
///   `Unsigned64`, `Text`, `Bytes`, `FixedBytes20`, and `Unsigned256`; the
///   reader over `FixedBytes32([0x11; 32])`, `Unsigned16(0x0102)`,
///   `Unsigned32(0x01020304)`, `Unsigned64(0x0102030405060708)`,
///   `Text("left-pad")`, `Bytes(vec![0xa1, 0xa2])`, `FixedBytes20([0x22; 20])`,
///   and `Unsigned256([0x80; 32])`, through `try_new` and bound by `expect`.
/// Act:     `read_fixed_bytes32`, `read_unsigned16`, `read_unsigned32`,
///   `read_unsigned64`, `read_text`, `read_bytes`, `read_fixed_bytes20`, and
///   `read_unsigned256`, in that order.
/// Assert:  each result equals `Ok` of its contents, `[0x11; 32]`, `0x0102`,
///   `0x01020304`, `0x0102030405060708`, `"left-pad"`, `vec![0xa1, 0xa2]`,
///   `[0x22; 20]`, and `[0x80; 32]`, by `assert_eq!`, each compared apart.
#[test]
fn each_kind_read_returns_its_contents() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::Unsigned16,
        CanonicalFieldKind::Unsigned32,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes20,
        CanonicalFieldKind::Unsigned256,
    ];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::FixedBytes32([0x11; 32]),
                CanonicalFieldValue::Unsigned16(0x0102),
                CanonicalFieldValue::Unsigned32(0x01020304),
                CanonicalFieldValue::Unsigned64(0x0102030405060708),
                CanonicalFieldValue::Text("left-pad".to_owned()),
                CanonicalFieldValue::Bytes(vec![0xa1, 0xa2]),
                CanonicalFieldValue::FixedBytes20([0x22; 20]),
                CanonicalFieldValue::Unsigned256([0x80; 32]),
            ]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");

    // Act & Assert
    assert_eq!(reader.read_fixed_bytes32(), Ok([0x11; 32]));
    assert_eq!(reader.read_unsigned16(), Ok(0x0102));
    assert_eq!(reader.read_unsigned32(), Ok(0x01020304));
    assert_eq!(reader.read_unsigned64(), Ok(0x0102030405060708));
    assert_eq!(reader.read_text(), Ok("left-pad".to_owned()));
    assert_eq!(reader.read_bytes(), Ok(vec![0xa1, 0xa2]));
    assert_eq!(reader.read_fixed_bytes20(), Ok([0x22; 20]));
    assert_eq!(reader.read_unsigned256(), Ok([0x80; 32]));
}
// each_kind_read_returns_its_contents

/// Contract: each `read_<kind>`, read refused: the value is of another kind
///   than the field → `Err(CanonicalFieldReaderErrorReturn::Kind { index: 0,
///   expected, actual })`.
/// Arrange: for each of the eight kinds, a field list of that one kind and a
///   reader over one value of another kind, `Unsigned64(1)` for every kind but
///   `Unsigned64` and `Text("x")` for `Unsigned64`, through `try_new` and bound
///   by `expect`.
/// Act:     the kind's `read_<kind>` once.
/// Assert:  `result.err()` equals `Some(CanonicalFieldReaderErrorReturn::Kind
///   { index: 0, expected: <that kind>, actual: <the value's kind> })` by
///   `assert_eq!` for each kind, each compared apart.
#[test]
fn each_kind_read_refuses_a_value_of_another_variant() {
    // Arrange & Act & Assert — one field, one value of another kind, each kind
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Unsigned64(1)]),
        }),
        &[CanonicalFieldKind::FixedBytes32],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_fixed_bytes32().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::FixedBytes32,
            actual: CanonicalFieldKind::Unsigned64
        })
    );

    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Unsigned64(1)]),
        }),
        &[CanonicalFieldKind::Unsigned16],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_unsigned16().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::Unsigned16,
            actual: CanonicalFieldKind::Unsigned64
        })
    );

    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Unsigned64(1)]),
        }),
        &[CanonicalFieldKind::Unsigned32],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_unsigned32().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::Unsigned32,
            actual: CanonicalFieldKind::Unsigned64
        })
    );

    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Text("x".to_owned())]),
        }),
        &[CanonicalFieldKind::Unsigned64],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_unsigned64().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::Unsigned64,
            actual: CanonicalFieldKind::Text
        })
    );

    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Unsigned64(1)]),
        }),
        &[CanonicalFieldKind::Text],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_text().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::Text,
            actual: CanonicalFieldKind::Unsigned64
        })
    );

    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Unsigned64(1)]),
        }),
        &[CanonicalFieldKind::Bytes],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_bytes().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::Bytes,
            actual: CanonicalFieldKind::Unsigned64
        })
    );

    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Unsigned64(1)]),
        }),
        &[CanonicalFieldKind::FixedBytes20],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_fixed_bytes20().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::FixedBytes20,
            actual: CanonicalFieldKind::Unsigned64
        })
    );

    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::Text("x".to_owned())]),
        }),
        &[CanonicalFieldKind::Unsigned256],
    )
    .expect("the value list matches the field list in count");
    assert_eq!(
        reader.read_unsigned256().err(),
        Some(CanonicalFieldReaderErrorReturn::Kind {
            index: 0,
            expected: CanonicalFieldKind::Unsigned256,
            actual: CanonicalFieldKind::Text
        })
    );
}
// each_kind_read_refuses_a_value_of_another_variant

/// Contract: `read_form`, admitted → `Ok(value)`, and `index` advanced.
/// Arrange: the field list `FixedBytes32`; the reader over
///   `FixedBytes32([0x44; 32])`, through `try_new` and bound by `expect`.
/// Act:     `reader.read_form::<MockICanonicalField>()`.
/// Assert:  the result equals `Ok(build_mock_i_canonical_field(
///   MockICanonicalFieldOverrides { bytes: Some([0x44; 32]) }))` by
///   `assert_eq!`.
#[test]
fn read_form_returns_the_form() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] = &[CanonicalFieldKind::FixedBytes32];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![CanonicalFieldValue::FixedBytes32([0x44; 32])]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");

    // Act
    let result = reader.read_form::<MockICanonicalField>();

    // Assert
    assert_eq!(
        result,
        Ok(build_mock_i_canonical_field(MockICanonicalFieldOverrides {
            bytes: Some([0x44; 32])
        }))
    );
}
// read_form_returns_the_form

/// Contract: `read_form`, refused → `Err(CanonicalFieldReaderErrorReturn::Form
///   { index, error })`, `error` unchanged.
/// Arrange: the field list `Text` and `FixedBytes32`; the reader over
///   `Text("a")` and `Unsigned16(1)`, through `try_new` and bound by `expect`,
///   and one admitted `read_text`.
/// Act:     `reader.read_form::<MockICanonicalField>()`.
/// Assert:  `result.err()` equals `Some(CanonicalFieldReaderErrorReturn::Form
///   { index: 1, error: FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes32 } })` by `assert_eq!`.
#[test]
fn read_form_carries_the_forms_refusal_with_its_index() {
    // Arrange
    let fields: &'static [CanonicalFieldKind] =
        &[CanonicalFieldKind::Text, CanonicalFieldKind::FixedBytes32];
    let mut reader = CanonicalFieldReader::try_new(
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Text("a".to_owned()),
                CanonicalFieldValue::Unsigned16(1),
            ]),
        }),
        fields,
    )
    .expect("the value list matches the field list in count");
    reader.read_text().expect("the text field reads");

    // Act
    let result = reader.read_form::<MockICanonicalField>();

    // Assert
    assert_eq!(
        result.err(),
        Some(CanonicalFieldReaderErrorReturn::Form {
            index: 1,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes32
            }
        })
    );
}
// read_form_carries_the_forms_refusal_with_its_index
