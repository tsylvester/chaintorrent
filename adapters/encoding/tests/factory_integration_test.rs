#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, CreateEncodingErrorReturn,
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, DecodeErrorReturn,
    ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodingIdentifier, IEncodingContract,
    MockIEncodingContract, MockIEncodingContractFromFieldsErrorReturn, build_canonical_fields,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_mock_i_encoding_consumer, create_encoding,
};

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

/// Contract: the own entry, every concrete is handed to the consumer under
///   the identifier it declares: `create_encoding` called with a concrete's
///   selection and the identifier it declares → `Ok(CreateEncodingSuccessReturn
///   { output })` with `output.declaration` the declaration the consumer read
///   through `E::DECLARATION`, whose identifier is the identifier required and
///   whose interface version is `ENCODING_INTERFACE_VERSION`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes,
///   `build_create_encoding_params` overriding `concrete` and `identifier` with
///   the case's selection and identifier, and `build_create_encoding_payload`,
///   all from the crate's public surface under the `mocks` feature; a hand-over
///   of a concrete other than the selected one returns another identifier.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `declaration` of the `output` of the `Ok` arm declares the
///   case's identifier by `assert!` and the interface version
///   `ENCODING_INTERFACE_VERSION` by `assert_eq!`, each compared apart.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the consumer's
///   read of the concrete's `E::DECLARATION`.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real consumer
///   or a real description does.
#[test]
fn every_concrete_is_handed_to_the_consumer_under_the_identifier_it_declares() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert!(success.output.declaration.identifier == case.identifier);
        assert_eq!(
            success.output.declaration.interface_version,
            ENCODING_INTERFACE_VERSION
        );
    }
}
// every_concrete_is_handed_to_the_consumer_under_the_identifier_it_declares

/// Contract: the own entry, every concrete is refused under an identifier it
///   does not declare: `create_encoding` called with the selection of a
///   concrete and an identifier that concrete does not declare →
///   `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes,
///   `build_create_encoding_params` overriding `concrete` with the case's
///   selection and `identifier` with `EncodingIdentifier::Mock`, an identifier
///   no case of the declared set declares, and `build_create_encoding_payload`,
///   all from the crate's public surface under the `mocks` feature; the
///   pairing carries an identifier the selected concrete does not declare, so
///   an admission that compares against a constant or ignores the selection
///   admits a pairing it must refuse.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  `result.err()` equals
///   `Some(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)` by
///   `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`
///   reading the selected concrete's declaration before any construction.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   which the refusal never reaches.
#[test]
fn every_concrete_is_refused_under_an_identifier_it_does_not_declare() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(EncodingIdentifier::Mock),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        assert_eq!(
            result.err(),
            Some(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)
        );
    }
}
// every_concrete_is_refused_under_an_identifier_it_does_not_declare

/// Contract: the own entry, every concrete decodes what it encodes:
///   `create_encoding` called with a concrete's selection and the identifier
///   it declares, the consumer encoding a field list through the concrete
///   under the official description and decoding the bytes →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip` an
///   `Ok(DecodeSuccessReturn { described })` whose `described` equals the field
///   list encoded.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("round-trip")`,
///   `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each
///   `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`,
///   `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`, and
///   `Unsigned256` of a last byte `0x07` and every other byte `0x00`, and no
///   bytes, `build_create_encoding_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the field list differs from the builder's
///   default in every value, so a decode that returns the default field list,
///   or any value its bytes do not hold, fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `described` of the `Ok` arm of `output.round_trip` equals
///   `build_canonical_fields` overriding `values` with the same field list by
///   `assert_eq!`, the expectation built apart from the field list the
///   consumer was given.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the concrete's
///   `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real consumer
///   or a real description does.
#[test]
fn every_concrete_decodes_what_it_encodes() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Text("round-trip".to_string()),
                    CanonicalFieldValue::Bytes(vec![0xb1, 0xb2]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned16(0x0304),
                    CanonicalFieldValue::Unsigned32(0x05060708),
                    CanonicalFieldValue::Unsigned64(0x090a0b0c0d0e0f10),
                    CanonicalFieldValue::FixedBytes20([0x44; 20]),
                    CanonicalFieldValue::Unsigned256({
                        let mut word = [0x00; 32];
                        word[31] = 0x07;
                        word
                    }),
                ]),
            }),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let expected = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Text("round-trip".to_string()),
                CanonicalFieldValue::Bytes(vec![0xb1, 0xb2]),
                CanonicalFieldValue::FixedBytes32([0x33; 32]),
                CanonicalFieldValue::Unsigned16(0x0304),
                CanonicalFieldValue::Unsigned32(0x05060708),
                CanonicalFieldValue::Unsigned64(0x090a0b0c0d0e0f10),
                CanonicalFieldValue::FixedBytes20([0x44; 20]),
                CanonicalFieldValue::Unsigned256({
                    let mut word = [0x00; 32];
                    word[31] = 0x07;
                    word
                }),
            ]),
        });
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert_eq!(
            success
                .output
                .round_trip
                .ok()
                .map(|decoded| decoded.described),
            Some(expected)
        );
    }
}
// every_concrete_decodes_what_it_encodes

/// Contract: the own entry, every concrete refuses a description that emits
///   the wrong count: `create_encoding` called with a concrete's selection
///   and the identifier it declares, the consumer encoding a field list one
///   value short of the description's `FIELDS` →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` equal
///   to `Err(EncodeErrorReturn::FieldCount { expected, actual })`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("short")`,
///   `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`,
///   `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, and `FixedBytes20` of
///   bytes each `0x66`, and no bytes, `build_create_encoding_params`
///   overriding `concrete` and `identifier` with the case's selection and
///   identifier, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the field list is one value
///   short, so a concrete that encodes a field list of the wrong count returns
///   bytes instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals
///   `Some(EncodeErrorReturn::FieldCount { expected: <MockIEncodingContract as
///   IEncodingContract>::FIELDS.len(), actual: 7 })` by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the concrete's
///   `encode` as the consumer calls it.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_a_description_emitting_the_wrong_count() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Text("short".to_string()),
                    CanonicalFieldValue::Bytes(vec![0xc1]),
                    CanonicalFieldValue::FixedBytes32([0x55; 32]),
                    CanonicalFieldValue::Unsigned16(5),
                    CanonicalFieldValue::Unsigned32(6),
                    CanonicalFieldValue::Unsigned64(7),
                    CanonicalFieldValue::FixedBytes20([0x66; 20]),
                ]),
            }),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert_eq!(
            success.output.encoded.err(),
            Some(EncodeErrorReturn::FieldCount {
                expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
                actual: 7,
            })
        );
    }
}
// every_concrete_refuses_a_description_emitting_the_wrong_count

/// Contract: the own entry, every concrete refuses a description that emits
///   the wrong kind: `create_encoding` called with a concrete's selection and
///   the identifier it declares, the consumer encoding a field list whose
///   value at index `0` is not the kind the description's `FIELDS` names there
///   → `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded`
///   equal to `Err(EncodeErrorReturn::FieldKind { index, expected, actual })`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Unsigned16(1)`,
///   `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`,
///   `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of bytes
///   each `0x66`, and `Unsigned256` of a last byte `0x08` and every other byte
///   `0x00`, and no bytes, `build_create_encoding_params` overriding `concrete`
///   and `identifier` with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the value at index `0` is of another kind, so
///   a concrete that encodes a field list of the wrong kind returns bytes
///   instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals
///   `Some(EncodeErrorReturn::FieldKind { index: 0, expected:
///   CanonicalFieldKind::Text, actual: CanonicalFieldKind::Unsigned16 })` by
///   `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the concrete's
///   `encode` as the consumer calls it.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_a_description_emitting_the_wrong_kind() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Bytes(vec![0xc1]),
                    CanonicalFieldValue::FixedBytes32([0x55; 32]),
                    CanonicalFieldValue::Unsigned16(5),
                    CanonicalFieldValue::Unsigned32(6),
                    CanonicalFieldValue::Unsigned64(7),
                    CanonicalFieldValue::FixedBytes20([0x66; 20]),
                    CanonicalFieldValue::Unsigned256({
                        let mut word = [0x00; 32];
                        word[31] = 0x08;
                        word
                    }),
                ]),
            }),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert_eq!(
            success.output.encoded.err(),
            Some(EncodeErrorReturn::FieldKind {
                index: 0,
                expected: CanonicalFieldKind::Text,
                actual: CanonicalFieldKind::Unsigned16,
            })
        );
    }
}
// every_concrete_refuses_a_description_emitting_the_wrong_kind

/// Contract: the own entry, every concrete refuses bytes past a complete
///   encoding: `create_encoding` called with a concrete's selection and the
///   identifier it declares, the consumer decoding the bytes the concrete
///   wrote followed by one extra byte → `Ok(CreateEncodingSuccessReturn {
///   output })` with `output.trailing_bytes` an `Err` in the concrete's own
///   variant of `DecodeErrorReturn`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes,
///   `build_create_encoding_params` overriding `concrete` and `identifier`
///   with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the bytes decoded are a complete encoding
///   followed by one `0x00` byte, so a decode that accepts bytes past a
///   complete encoding returns a value instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `trailing_bytes` of the `output` of the `Ok` arm is an `Err`,
///   by `.is_err()`, the variant differing by concrete.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the concrete's
///   `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_bytes_past_a_complete_encoding() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert!(success.output.trailing_bytes.is_err());
    }
}
// every_concrete_refuses_bytes_past_a_complete_encoding

/// Contract: the own entry, every concrete refuses bytes that end inside an
///   encoding: `create_encoding` called with a concrete's selection and the
///   identifier it declares, the consumer decoding the bytes the concrete
///   wrote with the last byte removed → `Ok(CreateEncodingSuccessReturn {
///   output })` with `output.truncated_bytes` an `Err` in the concrete's own
///   variant of `DecodeErrorReturn`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes,
///   `build_create_encoding_params` overriding `concrete` and `identifier`
///   with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the bytes decoded are a complete encoding
///   without its last byte, so a decode that accepts bytes ending inside a
///   field returns a value instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `truncated_bytes` of the `output` of the `Ok` arm is an `Err`,
///   by `.is_err()`, the variant differing by concrete.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the concrete's
///   `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_bytes_that_end_inside_an_encoding() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert!(success.output.truncated_bytes.is_err());
    }
}
// every_concrete_refuses_bytes_that_end_inside_an_encoding

/// Contract: the own entry, every concrete returns the description's refusal
///   unchanged: `create_encoding` called with a concrete's selection and the
///   identifier it declares, the consumer encoding a field list the official
///   description refuses and decoding the bytes →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip`
///   equal to `Err(DecodeErrorReturn::Description(error))` with `error` the
///   description's refusal.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("")`, `Bytes` of
///   the byte `0xc1`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(5)`,
///   `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of bytes each `0x66`, and
///   `Unsigned256` of a last byte `0x08` and every other byte `0x00`, and no
///   bytes, `build_create_encoding_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the field list holds an empty `Text` the
///   description refuses, so a concrete that maps, drops, or replaces the
///   refusal fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `round_trip` of the `output` of the `Ok` arm, by `.err()`,
///   equals `Some(DecodeErrorReturn::Description(
///   MockIEncodingContractFromFieldsErrorReturn::EmptyText { index: 0 }))` by
///   `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the concrete's
///   `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real consumer
///   or a real description does.
#[test]
fn every_concrete_returns_the_description_refusal_unchanged() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Text("".to_string()),
                    CanonicalFieldValue::Bytes(vec![0xc1]),
                    CanonicalFieldValue::FixedBytes32([0x55; 32]),
                    CanonicalFieldValue::Unsigned16(5),
                    CanonicalFieldValue::Unsigned32(6),
                    CanonicalFieldValue::Unsigned64(7),
                    CanonicalFieldValue::FixedBytes20([0x66; 20]),
                    CanonicalFieldValue::Unsigned256({
                        let mut word = [0x00; 32];
                        word[31] = 0x08;
                        word
                    }),
                ]),
            }),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert_eq!(
            success.output.round_trip.err(),
            Some(DecodeErrorReturn::Description(
                MockIEncodingContractFromFieldsErrorReturn::EmptyText { index: 0 }
            ))
        );
    }
}
// every_concrete_returns_the_description_refusal_unchanged

/// Contract: the own entry, every concrete declaring the Ethereum ABI
///   identifier encodes the reference fields to the reference vector:
///   `create_encoding` called with the selection of a concrete that declares
///   `EncodingIdentifier::EthereumAbiV1` and that identifier, the consumer
///   encoding the reference fields under the official description →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` an
///   `Ok(EncodeSuccessReturn { bytes })` with `bytes` the reference vector.
/// Arrange: for each case whose identifier is `EncodingIdentifier::EthereumAbiV1`,
///   `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking
///   `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no
///   bytes, `build_create_encoding_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the default values are distinct in every kind,
///   so a swapped field, a wrong word width, or a misplaced tail changes the
///   bytes.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by
///   `.ok().map(|success| success.bytes)`, equals `Some` of the reference
///   vector's bytes by `assert_eq!`, the reference vector being the bytes of
///   the `abi.encode` parameter encoding of `Text("left-pad")`, `Bytes` of the
///   bytes `0xa1`, `0xa2`, and `0xa3`, `FixedBytes32` of bytes each `0x11`,
///   `Unsigned16(0x0102)`, `Unsigned32(0x01020304)`,
///   `Unsigned64(0x0102030405060708)`, `FixedBytes20` of bytes each `0x22`, and
///   `Unsigned256` of a first byte `0x80` and every other byte `0x00`, a hex
///   string authored by hand from the Solidity Contract ABI Specification and
///   decoded by `hex::decode`, and not produced by running `alloy` or this
///   crate.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor, and `encode`.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_fields_to_the_reference_vector()
 {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|case| case.identifier == EncodingIdentifier::EthereumAbiV1)
    {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert_eq!(
            success.output.encoded.ok().map(|encoded| encoded.bytes),
            Some(reference_vector())
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_fields_to_the_reference_vector

/// Contract: the own entry, every concrete declaring the Ethereum ABI
///   identifier decodes the reference vector to the reference fields:
///   `create_encoding` called with the selection of a concrete that declares
///   `EncodingIdentifier::EthereumAbiV1` and that identifier, the consumer
///   decoding the reference vector under the official description →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.decoded` an
///   `Ok(DecodeSuccessReturn { described })` with `described` equal to the
///   reference fields.
/// Arrange: for each case whose identifier is `EncodingIdentifier::EthereumAbiV1`,
///   `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking
///   `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and
///   the reference vector's bytes, the `abi.encode` parameter encoding of the
///   default values, a hex string authored by hand from the Solidity Contract
///   ABI Specification and decoded by `hex::decode`, and not produced by
///   running `alloy` or this crate, `build_create_encoding_params` overriding
///   `concrete` and `identifier` with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the default values are distinct in every kind,
///   so a field read from the wrong word or tail yields a refusal or an
///   unequal field list.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `decoded` of the `output` of the `Ok` arm, by
///   `.ok().map(|success| success.described)`, equals `Some` of
///   `build_canonical_fields` at its defaults by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor, `decode`, and the canonical
///   re-encoding through `encode`.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_fields()
 {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|case| case.identifier == EncodingIdentifier::EthereumAbiV1)
    {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            reference_vector(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selection is admitted under the identifier it declares");
        assert_eq!(
            success.output.decoded.ok().map(|decoded| decoded.described),
            Some(build_canonical_fields(Default::default()))
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_fields
