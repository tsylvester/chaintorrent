#![allow(clippy::expect_used)]

use super::provides::{
    AbiDecoderErrorReturn, AbiEncoding, build_abi_encoding, build_abi_encoding_constructor_params,
    build_long_vector, build_reference_vector,
};
use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, DecodeErrorReturn,
    EncodeErrorReturn, IDecoderAdapter, IEncoderAdapter, IEncodingContract, MockIEncodingContract,
    build_canonical_fields, build_decode_params, build_encode_params,
};

/// Contract: `AbiEncoding::try_new(params)`, one branch: any params →
///   `Ok(AbiEncoding)`; the error arm has no branch.
/// Arrange: the params from `build_abi_encoding_constructor_params`.
/// Act:     `AbiEncoding::try_new(params)`.
/// Assert:  `result.is_ok()` is true.
#[test]
fn try_new_returns_the_adapter() {
    // Arrange
    let params = build_abi_encoding_constructor_params(Default::default());

    // Act
    let result = AbiEncoding::try_new(params);

    // Assert
    assert!(result.is_ok());
}

/// Contract: `encode`, admitted fields: every emitted value matches
///   `D::FIELDS` in count and kind → `Ok(EncodeSuccessReturn { bytes })`
///   holding the parameter encoding, each canonical value mapped in order to
///   its ABI type.
/// Arrange: the reference fields as the payload; the values are distinct in
///   every kind, a byte string and a text precede the static items, and the
///   20-byte string precedes the 256-bit integer, so a wrong map, a swapped
///   word, a misplaced tail, or a misaligned `bytes20` changes the bytes.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.ok().map(|success| success.bytes)` equals `Some` of the
///   reference vector's bytes by `assert_eq!`, the expectation authored from
///   the ABI specification and not read back from the arrangement.
#[test]
fn encode_writes_every_kind_as_its_abi_parameter_encoding() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let fields = build_canonical_fields(Default::default());

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.bytes),
        Some(build_reference_vector())
    );
}

/// Contract: `encode`, wrong field count: the emitted count differs from
///   `D::FIELDS.len()` → `Err(EncodeErrorReturn::FieldCount { expected,
///   actual })` before ABI encoding.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values without index `7` as the payload, which the mock description emits
///   as its fields, every value present of its declared kind, so only the
///   count differs.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals `Some(EncodeErrorReturn::FieldCount {
///   expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 7 })` by `assert_eq!`.
#[test]
fn encode_rejects_a_description_emitting_one_value_short() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("left-pad".to_string()),
            CanonicalFieldValue::Bytes(vec![0xa1, 0xa2, 0xa3]),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::Unsigned16(0x0102),
            CanonicalFieldValue::Unsigned32(0x01020304),
            CanonicalFieldValue::Unsigned64(0x0102030405060708),
            CanonicalFieldValue::FixedBytes20([0x22; 20]),
        ]),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldCount {
            expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
            actual: 7
        })
    );
}

/// Contract: `encode`, wrong field count: the emitted count differs from
///   `D::FIELDS.len()` → `Err(EncodeErrorReturn::FieldCount { expected,
///   actual })` before ABI encoding.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values followed by `Unsigned64(0)` as the payload, every value at indexes
///   `0` through `7` of its declared kind, so only the count differs.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals `Some(EncodeErrorReturn::FieldCount {
///   expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 9 })` by `assert_eq!`.
#[test]
fn encode_rejects_a_description_emitting_one_value_long() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("left-pad".to_string()),
            CanonicalFieldValue::Bytes(vec![0xa1, 0xa2, 0xa3]),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::Unsigned16(0x0102),
            CanonicalFieldValue::Unsigned32(0x01020304),
            CanonicalFieldValue::Unsigned64(0x0102030405060708),
            CanonicalFieldValue::FixedBytes20([0x22; 20]),
            CanonicalFieldValue::Unsigned256({
                let mut word = [0x00; 32];
                word[0] = 0x80;
                word
            }),
            CanonicalFieldValue::Unsigned64(0),
        ]),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldCount {
            expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
            actual: 9
        })
    );
}

/// Contract: `encode`, ordering: the count check precedes the kind check.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values without index `7` and with the value at index `0` replaced by
///   `Unsigned16(1)`, a payload both short and holding a wrong kind, so a kind
///   check ahead of the count reports the kind.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals `Some(EncodeErrorReturn::FieldCount {
///   expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 7 })` by `assert_eq!`.
#[test]
fn encode_checks_the_count_before_any_kind() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Bytes(vec![0xa1, 0xa2, 0xa3]),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::Unsigned16(0x0102),
            CanonicalFieldValue::Unsigned32(0x01020304),
            CanonicalFieldValue::Unsigned64(0x0102030405060708),
            CanonicalFieldValue::FixedBytes20([0x22; 20]),
        ]),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldCount {
            expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
            actual: 7
        })
    );
}

/// Contract: `encode`, wrong field kind: the count matches and an emitted
///   value's variant differs from the `D::FIELDS` kind at its index →
///   `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` before
///   ABI encoding.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `1` replaced by `Text("x")` as the payload,
///   every other value of its declared kind, so only the kind at index `1`
///   differs.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals `Some(EncodeErrorReturn::FieldKind {
///   index: 1, expected: CanonicalFieldKind::Bytes, actual:
///   CanonicalFieldKind::Text })` by `assert_eq!`.
#[test]
fn encode_rejects_a_description_emitting_the_wrong_kind() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("left-pad".to_string()),
            CanonicalFieldValue::Text("x".to_string()),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::Unsigned16(0x0102),
            CanonicalFieldValue::Unsigned32(0x01020304),
            CanonicalFieldValue::Unsigned64(0x0102030405060708),
            CanonicalFieldValue::FixedBytes20([0x22; 20]),
            CanonicalFieldValue::Unsigned256({
                let mut word = [0x00; 32];
                word[0] = 0x80;
                word
            }),
        ]),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
            actual: CanonicalFieldKind::Text
        })
    );
}

/// Contract: `encode`, wrong field kind: more than one emitted value differs
///   from its `D::FIELDS` kind → `Err(EncodeErrorReturn::FieldKind { index,
///   expected, actual })` for the lowest such index.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values, the value at index `2` replaced by `Unsigned16(1)` and the value
///   at index `6` replaced by `Unsigned256` of bytes each `0x00`, so a check
///   that runs from the highest index reports index `6`.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals `Some(EncodeErrorReturn::FieldKind {
///   index: 2, expected: CanonicalFieldKind::FixedBytes32, actual:
///   CanonicalFieldKind::Unsigned16 })` by `assert_eq!`.
#[test]
fn encode_reports_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("left-pad".to_string()),
            CanonicalFieldValue::Bytes(vec![0xa1, 0xa2, 0xa3]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned16(0x0102),
            CanonicalFieldValue::Unsigned32(0x01020304),
            CanonicalFieldValue::Unsigned64(0x0102030405060708),
            CanonicalFieldValue::Unsigned256([0x00; 32]),
            CanonicalFieldValue::Unsigned256({
                let mut word = [0x00; 32];
                word[0] = 0x80;
                word
            }),
        ]),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::FixedBytes32,
            actual: CanonicalFieldKind::Unsigned16
        })
    );
}

/// Contract: `decode`, admitted: the re-encoding equals the payload →
///   `Ok(DecodeSuccessReturn { described })`.
/// Arrange: the reference vector's bytes as the payload, a canonical encoding
///   whose every kind is distinct, so a field read from the wrong word or tail
///   changes the described value.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.ok().map(|success| success.described)` equals `Some` of
///   the reference fields by `assert_eq!`.
#[test]
fn decode_reads_the_vector_as_its_value() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let payload = build_reference_vector();

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.described),
        Some(build_canonical_fields(Default::default()))
    );
}

/// Contract: `decode`, malformed: `DynSolType::Tuple` over the description's
///   `FIELDS` refuses the payload through `abi_decode_params` →
///   `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`,
///   the error unchanged.
/// Arrange: the reference vector's bytes cut after the head, one word per
///   field, so the `Text` and `Bytes` offsets point past the end of the
///   payload.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::Abi(
///   AbiDecoderErrorReturn::Malformed(error)))` by `assert_eq!`, `error` the
///   vendor's `Overrun` refusal.
#[test]
fn decode_rejects_input_truncated_to_the_head() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload.truncate(256);

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(
            alloy::dyn_abi::Error::SolTypes(alloy::sol_types::Error::Overrun)
        )))
    );
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the reference vector's bytes followed by one `0x00` byte, which
///   the dynamic decoder reads past.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_trailing_bytes() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload.push(0x00);

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the reference vector's bytes with the last padding byte of the
///   `Text` tail set to `0x01`, which leaves the decoded text unchanged.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_nonzero_string_padding() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload[319] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the reference vector's bytes with the last padding byte of the
///   `Bytes` tail set to `0x01`, which leaves the decoded bytes unchanged.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_nonzero_byte_string_padding() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload[383] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the reference vector's bytes with the last byte of the `bytes20`
///   head word set to `0x01`, which leaves the first twenty bytes of the word
///   unchanged.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_nonzero_fixed_bytes20_padding() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload[223] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the reference vector's bytes with a zero word inserted between the
///   head and the tails and the `Text` and `Bytes` offsets each increased by
///   one word, a valid layout that is not the canonical one.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_a_non_canonical_offset() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload[31] = 0x20;
    payload[63] = 0x60;
    payload.splice(256..256, [0x00; 32]);

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the `abi.encode` parameter encoding of the reference values
///   followed by an extra `Unsigned64(0)`, authored from the ABI
///   specification, a head one word longer than the described tuple, which
///   the dynamic decoder reads past.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_the_encoding_of_one_field_too_many() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let payload = build_long_vector();

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, integer out of range: an integer item's value does not
///   convert into the field's `u16` through `try_from` →
///   `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange
///   { index, kind }))`.
/// Arrange: the reference vector's bytes with the high-order byte of the
///   `uint16` head word set to `0x01`, a word wider than 16 bits.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::Abi(
///   AbiDecoderErrorReturn::ValueOutOfRange { index: 3, kind:
///   CanonicalFieldKind::Unsigned16 }))` by `assert_eq!`.
#[test]
fn decode_rejects_a_uint16_word_wider_than_sixteen_bits() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload[96] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(
            AbiDecoderErrorReturn::ValueOutOfRange {
                index: 3,
                kind: CanonicalFieldKind::Unsigned16
            }
        ))
    );
}

/// Contract: `decode`, integer out of range: an integer item's value does not
///   convert into the field's `u32` through `try_from` →
///   `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange
///   { index, kind }))`.
/// Arrange: the reference vector's bytes with the high-order byte of the
///   `uint32` head word set to `0x01`, a word wider than 32 bits.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::Abi(
///   AbiDecoderErrorReturn::ValueOutOfRange { index: 4, kind:
///   CanonicalFieldKind::Unsigned32 }))` by `assert_eq!`.
#[test]
fn decode_rejects_a_uint32_word_wider_than_thirty_two_bits() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload[128] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(
            AbiDecoderErrorReturn::ValueOutOfRange {
                index: 4,
                kind: CanonicalFieldKind::Unsigned32
            }
        ))
    );
}

/// Contract: `decode`, integer out of range: an integer item's value does not
///   convert into the field's `u64` through `try_from` →
///   `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange
///   { index, kind }))`.
/// Arrange: the reference vector's bytes with the high-order byte of the
///   `uint64` head word set to `0x01`, a word wider than 64 bits.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::Abi(
///   AbiDecoderErrorReturn::ValueOutOfRange { index: 5, kind:
///   CanonicalFieldKind::Unsigned64 }))` by `assert_eq!`.
#[test]
fn decode_rejects_a_uint64_word_wider_than_sixty_four_bits() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = build_reference_vector();
    payload[160] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(
            AbiDecoderErrorReturn::ValueOutOfRange {
                index: 5,
                kind: CanonicalFieldKind::Unsigned64
            }
        ))
    );
}
