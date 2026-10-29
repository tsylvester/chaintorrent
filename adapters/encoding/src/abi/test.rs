#![allow(clippy::expect_used)]

use super::provides::{
    AbiDecoderErrorReturn, AbiEncoding, build_abi_encoding, build_abi_encoding_constructor_params,
};
use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, DecodeErrorReturn,
    EncodeErrorReturn, EncodingTypeName, EncodingTypeNameConstructorParamsOverrides,
    EncodingTypeNameErrorReturn, IDecoderAdapter, IEncoderAdapter, IEncodingContract,
    MockIEncodingContract, build_canonical_fields, build_decode_params, build_encode_params,
    build_encoding_type_name_constructor_params, build_type_name_params,
};
use alloy::dyn_abi::DynSolType;
use std::collections::BTreeSet;

const REFERENCE_VECTOR_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000100",
    "0000000000000000000000000000000000000000000000000000000000000140",
    "1111111111111111111111111111111111111111111111111111111111111111",
    "0000000000000000000000000000000000000000000000000000000000000102",
    "0000000000000000000000000000000000000000000000000000000001020304",
    "0000000000000000000000000000000000000000000000000102030405060708",
    "2222222222222222222222222222222222222222000000000000000000000000",
    "8000000000000000000000000000000000000000000000000000000000000000",
    "0000000000000000000000000000000000000000000000000000000000000008",
    "6c6566742d706164000000000000000000000000000000000000000000000000",
    "0000000000000000000000000000000000000000000000000000000000000003",
    "a1a2a30000000000000000000000000000000000000000000000000000000000",
);

fn reference_vector() -> Vec<u8> {
    hex::decode(REFERENCE_VECTOR_HEX).expect("the reference vector is valid hex")
}

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
/// Arrange: the adapter, the `MockIEncodingContract` description, and the
///   reference fields as the payload; the values are distinct in every kind,
///   a byte string and a text precede the static items, and the 20-byte
///   string precedes the 256-bit integer, so a wrong map, a swapped word, a
///   misplaced tail, or a misaligned `bytes20` changes the bytes.
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
        Some(reference_vector())
    );
}

/// Contract: `encode`, wrong field count: the emitted count differs from
///   `D::FIELDS.len()` → `Err(EncodeErrorReturn::FieldCount { expected,
///   actual })` before ABI encoding.
/// Arrange: the adapter and `build_canonical_fields` overriding `values`
///   with the reference values without index `7` as the payload, which the
///   mock description emits as its fields, every value present of its
///   declared kind, so only the count differs.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals
///   `Some(EncodeErrorReturn::FieldCount { expected:
///   <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 7 })` by `assert_eq!`.
#[test]
fn encode_rejects_a_description_emitting_one_value_short() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let mut values = build_canonical_fields(Default::default()).values;
    values.truncate(7);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldCount {
            expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
            actual: 7,
        })
    );
}

/// Contract: `encode`, wrong field count: the emitted count differs from
///   `D::FIELDS.len()` → `Err(EncodeErrorReturn::FieldCount { expected,
///   actual })` before ABI encoding.
/// Arrange: the adapter and `build_canonical_fields` overriding `values`
///   with the reference values followed by `Unsigned64(0)` as the payload,
///   every value at indexes `0` through `7` of its declared kind, so only
///   the count differs.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals
///   `Some(EncodeErrorReturn::FieldCount { expected:
///   <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 9 })` by `assert_eq!`.
#[test]
fn encode_rejects_a_description_emitting_one_value_long() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let mut values = build_canonical_fields(Default::default()).values;
    values.push(CanonicalFieldValue::Unsigned64(0));
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldCount {
            expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
            actual: 9,
        })
    );
}

/// Contract: `encode`, ordering: the count check precedes the kind check.
/// Arrange: the adapter and `build_canonical_fields` overriding `values`
///   with the reference values without index `7` and with the value at index
///   `0` replaced by `Unsigned16(1)`, a payload both short and holding a
///   wrong kind, so a kind check ahead of the count reports the kind.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals
///   `Some(EncodeErrorReturn::FieldCount { expected:
///   <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 7 })` by `assert_eq!`.
#[test]
fn encode_checks_the_count_before_any_kind() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let mut values = build_canonical_fields(Default::default()).values;
    values.truncate(7);
    values[0] = CanonicalFieldValue::Unsigned16(1);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldCount {
            expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
            actual: 7,
        })
    );
}

/// Contract: `encode`, wrong field kind: the count matches and an emitted
///   value's variant differs from the `D::FIELDS` kind at its index →
///   `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` before
///   ABI encoding.
/// Arrange: the adapter and `build_canonical_fields` overriding `values`
///   with the reference values and the value at index `1` replaced by
///   `Text("x")` as the payload, every other value of its declared kind, so
///   only the kind at index `1` differs.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals
///   `Some(EncodeErrorReturn::FieldKind { index: 1,
///   expected: CanonicalFieldKind::Bytes,
///   actual: CanonicalFieldKind::Text })` by `assert_eq!`.
#[test]
fn encode_rejects_a_description_emitting_the_wrong_kind() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let mut values = build_canonical_fields(Default::default()).values;
    values[1] = CanonicalFieldValue::Text("x".to_string());
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
            actual: CanonicalFieldKind::Text,
        })
    );
}

/// Contract: `encode`, wrong field kind: more than one emitted value differs
///   from its `D::FIELDS` kind → `Err(EncodeErrorReturn::FieldKind { index,
///   expected, actual })` for the lowest such index.
/// Arrange: the adapter and `build_canonical_fields` overriding `values`
///   with the reference values, the value at index `2` replaced by
///   `Unsigned16(1)` and the value at index `6` replaced by `Unsigned256` of
///   bytes each `0x00`, so a check that runs from the highest index reports
///   index `6`.
/// Act:     `adapter.encode(params, &fields)`.
/// Assert:  `result.err()` equals
///   `Some(EncodeErrorReturn::FieldKind { index: 2,
///   expected: CanonicalFieldKind::FixedBytes32,
///   actual: CanonicalFieldKind::Unsigned16 })` by `assert_eq!`.
#[test]
fn encode_reports_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_encode_params(&description);
    let mut values = build_canonical_fields(Default::default()).values;
    values[2] = CanonicalFieldValue::Unsigned16(1);
    values[6] = CanonicalFieldValue::Unsigned256([0x00; 32]);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = adapter.encode(params, &fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodeErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::FixedBytes32,
            actual: CanonicalFieldKind::Unsigned16,
        })
    );
}

/// Contract: `decode`, admitted: the re-encoding equals the payload →
///   `Ok(DecodeSuccessReturn { described })`.
/// Arrange: the adapter, the `MockIEncodingContract` description, and the
///   reference vector's bytes as the payload, a canonical encoding whose
///   every kind is distinct, so a field read from the wrong word or tail
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
    let payload = reference_vector();

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
/// Arrange: the adapter, the description, the reference vector's bytes cut
///   after the head, one word per field, so the `Text` and `Bytes` offsets
///   point past the end of the payload, and `error`, the vendor's own
///   `SolTypes(Overrun)` refusal of those bytes.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals
///   `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`
///   by `assert_eq!`.
#[test]
fn decode_rejects_input_truncated_to_the_head() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload.truncate(256);
    let tuple = DynSolType::Tuple(vec![
        DynSolType::String,
        DynSolType::Bytes,
        DynSolType::FixedBytes(32),
        DynSolType::Uint(16),
        DynSolType::Uint(32),
        DynSolType::Uint(64),
        DynSolType::FixedBytes(20),
        DynSolType::Uint(256),
    ]);
    let error = tuple
        .abi_decode_params(&payload)
        .expect_err("the vendor refuses the truncated head");

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(
            error
        )))
    );
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   followed by one `0x00` byte, which the dynamic decoder reads past.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_trailing_bytes() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload.push(0x00);

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   with the last padding byte of the `Text` tail set to `0x01`, which
///   leaves the decoded text unchanged.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_nonzero_string_padding() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload[319] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   with the last padding byte of the `Bytes` tail set to `0x01`, which
///   leaves the decoded bytes unchanged.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_nonzero_byte_string_padding() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload[383] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   with the last byte of the `bytes20` head word set to `0x01`, which
///   leaves the first twenty bytes of the word unchanged.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_nonzero_fixed_bytes20_padding() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload[223] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   with a zero word inserted between the head and the tails and the `Text`
///   and `Bytes` offsets each increased by one word, a valid layout that is
///   not the canonical one.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_a_non_canonical_offset() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let vector = reference_vector();
    let mut payload = Vec::with_capacity(vector.len() + 32);
    payload.extend_from_slice(&vector[..256]);
    payload.extend_from_slice(&[0x00; 32]);
    payload.extend_from_slice(&vector[256..]);
    payload[31] = 0x20;
    payload[63] = 0x60;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, non-canonical: the description admits the value and
///   re-encoding succeeds with bytes other than the payload →
///   `Err(DecodeErrorReturn::NonCanonical)`.
/// Arrange: the adapter, the description, and the `abi.encode` parameter
///   encoding of the reference values followed by an extra `Unsigned64(0)`,
///   authored from the ABI specification, a head one word longer than the
///   described tuple, which the dynamic decoder reads past.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::NonCanonical)` by
///   `assert_eq!`.
#[test]
fn decode_rejects_the_encoding_of_one_field_too_many() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let vector = reference_vector();
    let mut payload = Vec::with_capacity(vector.len() + 32);
    payload.extend_from_slice(&vector[..256]);
    payload.extend_from_slice(&[0x00; 32]);
    payload.extend_from_slice(&vector[256..]);
    payload[31] = 0x20;
    payload[63] = 0x60;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(result.err(), Some(DecodeErrorReturn::NonCanonical));
}

/// Contract: `decode`, integer out of range: an integer item's value does
///   not convert into the field's `u16` through `try_from` →
///   `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
///   index, kind }))`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   with the high-order byte of the `uint16` head word set to `0x01`, a
///   word wider than 16 bits.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals
///   `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
///   index: 3, kind: CanonicalFieldKind::Unsigned16 }))` by `assert_eq!`.
#[test]
fn decode_rejects_a_uint16_word_wider_than_sixteen_bits() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload[96] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(
            AbiDecoderErrorReturn::ValueOutOfRange {
                index: 3,
                kind: CanonicalFieldKind::Unsigned16,
            }
        ))
    );
}

/// Contract: `decode`, integer out of range: an integer item's value does
///   not convert into the field's `u32` through `try_from` →
///   `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
///   index, kind }))`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   with the high-order byte of the `uint32` head word set to `0x01`, a
///   word wider than 32 bits.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals
///   `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
///   index: 4, kind: CanonicalFieldKind::Unsigned32 }))` by `assert_eq!`.
#[test]
fn decode_rejects_a_uint32_word_wider_than_thirty_two_bits() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload[128] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(
            AbiDecoderErrorReturn::ValueOutOfRange {
                index: 4,
                kind: CanonicalFieldKind::Unsigned32,
            }
        ))
    );
}

/// Contract: `decode`, integer out of range: an integer item's value does
///   not convert into the field's `u64` through `try_from` →
///   `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
///   index, kind }))`.
/// Arrange: the adapter, the description, and the reference vector's bytes
///   with the high-order byte of the `uint64` head word set to `0x01`, a
///   word wider than 64 bits.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals
///   `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
///   index: 5, kind: CanonicalFieldKind::Unsigned64 }))` by `assert_eq!`.
#[test]
fn decode_rejects_a_uint64_word_wider_than_sixty_four_bits() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = MockIEncodingContract;
    let params = build_decode_params(&description);
    let mut payload = reference_vector();
    payload[160] = 0x01;

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Abi(
            AbiDecoderErrorReturn::ValueOutOfRange {
                index: 5,
                kind: CanonicalFieldKind::Unsigned64,
            }
        ))
    );
}

/// Contract: `type_name`, one total branch: every `CanonicalFieldKind` →
///   `Ok(TypeNameSuccessReturn { name })` with the ABI type name the table
///   states.
/// Arrange: the adapter, the eight kinds with their expected names written
///   from the ABI specification and not read back from the arrangement:
///   `FixedBytes32` `bytes32`, `Unsigned16` `uint16`, `Unsigned32` `uint32`,
///   `Unsigned64` `uint64`, `Text` `string`, `Bytes` `bytes`, `FixedBytes20`
///   `bytes20`, and `Unsigned256` `uint256`; the params from
///   `build_type_name_params`.
/// Act:     `adapter.type_name(params, kind)` for each kind.
/// Assert:  `result.ok().map(|success| success.name.as_str().to_owned())`
///   equals `Some` of the expected name for each kind by `assert_eq!`.
#[test]
fn type_name_gives_each_canonical_field_kind_its_abi_type_name() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let cases = [
        (CanonicalFieldKind::FixedBytes32, "bytes32"),
        (CanonicalFieldKind::Unsigned16, "uint16"),
        (CanonicalFieldKind::Unsigned32, "uint32"),
        (CanonicalFieldKind::Unsigned64, "uint64"),
        (CanonicalFieldKind::Text, "string"),
        (CanonicalFieldKind::Bytes, "bytes"),
        (CanonicalFieldKind::FixedBytes20, "bytes20"),
        (CanonicalFieldKind::Unsigned256, "uint256"),
    ];

    for (kind, expected) in cases {
        // Act
        let result = adapter.type_name(build_type_name_params(Default::default()), kind);

        // Assert
        assert_eq!(
            result.ok().map(|success| success.name.as_str().to_owned()),
            Some(expected.to_owned())
        );
    }
}
// type_name_gives_each_canonical_field_kind_its_abi_type_name

/// Contract: `type_name`, one total branch: `CanonicalFieldKind::FixedBytes20`
///   → the name `bytes20`.
/// Arrange: the adapter, the kind `FixedBytes20`, and the params from
///   `build_type_name_params`.
/// Act:     `adapter.type_name(params, CanonicalFieldKind::FixedBytes20)`.
/// Assert:  the name equals `bytes20` by `assert_eq!` and does not equal
///   `address` by `assert_ne!`, each compared apart.
#[test]
fn type_name_names_the_twenty_byte_kind_bytes20_and_never_address() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let params = build_type_name_params(Default::default());

    // Act
    let result = adapter.type_name(params, CanonicalFieldKind::FixedBytes20);
    let name = result.ok().map(|success| success.name.as_str().to_owned());

    // Assert
    assert_eq!(name, Some("bytes20".to_owned()));
    assert_ne!(name, Some("address".to_owned()));
}
// type_name_names_the_twenty_byte_kind_bytes20_and_never_address

/// Contract: `type_name`, one total branch: no two kinds share a name.
/// Arrange: the adapter, the eight kinds, and the params from
///   `build_type_name_params`.
/// Act:     `adapter.type_name(params, kind)` for each kind, collecting the
///   names.
/// Assert:  the collected names hold eight distinct values by `assert_eq!`
///   on the count of the set.
#[test]
fn type_name_gives_every_kind_exactly_one_distinct_name() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let kinds = [
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::Unsigned16,
        CanonicalFieldKind::Unsigned32,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes20,
        CanonicalFieldKind::Unsigned256,
    ];

    // Act
    let names: BTreeSet<String> = kinds
        .iter()
        .map(|kind| {
            adapter
                .type_name(build_type_name_params(Default::default()), *kind)
                .expect("every kind has a name")
                .name
                .as_str()
                .to_owned()
        })
        .collect();

    // Assert
    assert_eq!(names.len(), 8);
}
// type_name_gives_every_kind_exactly_one_distinct_name

/// Contract: `EncodingTypeName::try_new`, admitted name: a non-empty name of
///   ASCII letters and digits → `Ok(EncodingTypeName)` holding the text.
/// Arrange: `build_encoding_type_name_constructor_params` overriding `name`
///   with `uint256`.
/// Act:     `EncodingTypeName::try_new(params)`.
/// Assert:  `result.ok().map(|name| name.as_str().to_owned())` equals
///   `Some("uint256".to_owned())` by `assert_eq!`.
#[test]
fn encoding_type_name_try_new_admits_an_alphanumeric_name() {
    // Arrange
    let params =
        build_encoding_type_name_constructor_params(EncodingTypeNameConstructorParamsOverrides {
            name: Some("uint256".to_string()),
        });

    // Act
    let result = EncodingTypeName::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|name| name.as_str().to_owned()),
        Some("uint256".to_owned())
    );
}
// encoding_type_name_try_new_admits_an_alphanumeric_name

/// Contract: `EncodingTypeName::try_new`, empty name →
///   `Err(EncodingTypeNameErrorReturn::Empty)`.
/// Arrange: `build_encoding_type_name_constructor_params` overriding `name`
///   with the empty string.
/// Act:     `EncodingTypeName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(EncodingTypeNameErrorReturn::Empty)`
///   by `assert_eq!`.
#[test]
fn encoding_type_name_try_new_refuses_an_empty_name() {
    // Arrange
    let params =
        build_encoding_type_name_constructor_params(EncodingTypeNameConstructorParamsOverrides {
            name: Some(String::new()),
        });

    // Act
    let result = EncodingTypeName::try_new(params);

    // Assert
    assert_eq!(result.err(), Some(EncodingTypeNameErrorReturn::Empty));
}
// encoding_type_name_try_new_refuses_an_empty_name

/// Contract: `EncodingTypeName::try_new`, non-alphanumeric name →
///   `Err(EncodingTypeNameErrorReturn::NotAlphanumeric)`.
/// Arrange: `build_encoding_type_name_constructor_params` overriding `name`
///   with `uint 256`.
/// Act:     `EncodingTypeName::try_new(params)`.
/// Assert:  `result.err()` equals
///   `Some(EncodingTypeNameErrorReturn::NotAlphanumeric)` by `assert_eq!`.
#[test]
fn encoding_type_name_try_new_refuses_a_name_with_a_character_beyond_letters_and_digits() {
    // Arrange
    let params =
        build_encoding_type_name_constructor_params(EncodingTypeNameConstructorParamsOverrides {
            name: Some("uint 256".to_string()),
        });

    // Act
    let result = EncodingTypeName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(EncodingTypeNameErrorReturn::NotAlphanumeric)
    );
}
// encoding_type_name_try_new_refuses_a_name_with_a_character_beyond_letters_and_digits
