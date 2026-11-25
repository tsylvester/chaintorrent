#![allow(clippy::expect_used)]

use super::create_encoding;
use super::provides::{
    CanonicalFieldReaderErrorReturn, CanonicalFieldValue, CanonicalFieldsOverrides,
    CreateEncodingDepsOverrides, CreateEncodingParamsOverrides, ENCODING_CONCRETES,
    ENCODING_IDENTIFIERS, ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodingConcrete,
    EncodingConcreteOverrides, IEncodingContract, MockIEncodingContract, build_canonical_fields,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_encoding_concrete, build_mock_i_encoding_consumer,
};
use crate::lookup_encoding_declaration::provides::lookup_encoding_declaration;
use std::collections::BTreeSet;

fn cases() -> impl Iterator<Item = EncodingConcrete> {
    ENCODING_CONCRETES
        .iter()
        .map(|member| build_encoding_concrete(*member, EncodingConcreteOverrides::default()))
        .chain(core::iter::once(EncodingConcrete::Mock))
}

/// Contract: the own entry, every concrete is handed to the consumer under
///   its own declaration: `create_encoding` called with a concrete's
///   selection and deps holding the real `lookup_encoding_declaration` →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.declaration`
///   the declaration the consumer received in its payload, whose `concrete`
///   is the selection passed, whose `identifier` is a member of
///   `ENCODING_IDENTIFIERS`, and whose `interface_version` is
///   `ENCODING_INTERFACE_VERSION`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`; the cases declare
///   different members, so a declaration read for another selection than the
///   one that chose the adapter returns another `concrete`.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `declaration` of the `output` of the `Ok` arm has `concrete`
///   equal to the case's selection by `assert_eq!`, its `identifier`
///   contained in `ENCODING_IDENTIFIERS` by `assert!`, and its
///   `interface_version` equal to `ENCODING_INTERFACE_VERSION` by
///   `assert_eq!`, each compared apart, `type_name` not compared.
/// Boundary: the crate-internal call `create_encoding`; the route
///   `create_encoding`, the real lookup its deps hold, the selected
///   concrete's constructor where it has one, and the declaration the
///   consumer receives in its payload.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, and the
///   description it carries, replaced by `MockIEncodingContract`, so the test
///   does not prove what a real consumer or a real description does; the
///   concretes are real, the mock selection reaching the family's official
///   mock concrete.
#[test]
fn every_concrete_is_handed_to_the_consumer_under_its_own_declaration() {
    // Arrange
    for selection in cases() {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(
            consumer,
            CreateEncodingDepsOverrides {
                lookup: Some(lookup_encoding_declaration),
            },
        );
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            selection: Some(selection),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selected concrete's construction is infallible");
        assert_eq!(success.output.declaration.concrete, selection);
        assert!(ENCODING_IDENTIFIERS.contains(&success.output.declaration.identifier));
        assert_eq!(
            success.output.declaration.interface_version,
            ENCODING_INTERFACE_VERSION
        );
    }
}
// every_concrete_is_handed_to_the_consumer_under_its_own_declaration

/// Contract: the own entry, every concrete decodes what it encodes:
///   `create_encoding` called with a concrete's selection, the consumer
///   encoding a field list through the concrete under the official
///   description and decoding the bytes → `Ok(CreateEncodingSuccessReturn {
///   output })` with `output.round_trip` an `Ok(DecodeSuccessReturn {
///   described })` whose `described` equals the field list encoded.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("round-trip")`,
///   `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each
///   `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`,
///   `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`,
///   and `Unsigned256` of a last byte `0x07` and every other byte `0x00`, and
///   no bytes, overriding `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`; the cases write
///   different byte forms, and the field list differs from the builder's
///   default in every value, so a decode that returns the default field
///   list, or any value its bytes do not hold, fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `described` of the `Ok` arm of `output.round_trip` equals
///   `build_canonical_fields` overriding `values` with the same field list
///   by `assert_eq!`, the expectation built apart from the field list the
///   consumer was given.
/// Boundary: the crate-internal call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor where it has
///   one, and the concrete's `encode` and `decode` as the consumer calls
///   them.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, and the
///   description it carries, replaced by `MockIEncodingContract`, so the test
///   does not prove what a real consumer or a real description does; the
///   concretes are real, the mock selection reaching the family's official
///   mock concrete.
#[test]
fn every_concrete_decodes_what_it_encodes() {
    // Arrange
    for selection in cases() {
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
        let deps = build_create_encoding_deps(
            consumer,
            CreateEncodingDepsOverrides {
                lookup: Some(lookup_encoding_declaration),
            },
        );
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            selection: Some(selection),
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
        let success = result.expect("the selected concrete's construction is infallible");
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

/// Contract: the own entry, every concrete carries the reader's refusal of
///   a field list one value short: `create_encoding` called with a
///   concrete's selection, the consumer encoding a field list one value
///   short of the description's `FIELDS` → `Ok(CreateEncodingSuccessReturn {
///   output })` with `output.encoded` equal to `Err(EncodeErrorReturn::Fields(
///   CanonicalFieldReaderErrorReturn::Count { expected, actual }))`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("short")`,
///   `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`,
///   `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, and `FixedBytes20` of
///   bytes each `0x66`, and no bytes, overriding `lookup` with
///   `lookup_encoding_declaration`, `build_create_encoding_params`
///   overriding `selection` with the case's selection, and
///   `build_create_encoding_payload`; the field list is one value short, so
///   a concrete that encodes a field list the reader refuses returns bytes
///   instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals
///   `Some(EncodeErrorReturn::Fields(CanonicalFieldReaderErrorReturn::Count {
///   expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 7 }))` by `assert_eq!`.
/// Boundary: the crate-internal call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor where it has
///   one, the concrete's `encode` as the consumer calls it, and the
///   canonical field reader it constructs.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, and the
///   description it carries, replaced by `MockIEncodingContract`; the
///   concretes and the reader are real, the mock selection reaching the
///   family's official mock concrete.
#[test]
fn every_concrete_carries_the_readers_refusal_of_a_field_list_one_value_short() {
    // Arrange
    for selection in cases() {
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
        let deps = build_create_encoding_deps(
            consumer,
            CreateEncodingDepsOverrides {
                lookup: Some(lookup_encoding_declaration),
            },
        );
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            selection: Some(selection),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selected concrete's construction is infallible");
        assert_eq!(
            success.output.encoded.err(),
            Some(EncodeErrorReturn::Fields(
                CanonicalFieldReaderErrorReturn::Count {
                    expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
                    actual: 7,
                }
            ))
        );
    }
}
// every_concrete_carries_the_readers_refusal_of_a_field_list_one_value_short

/// Contract: the own entry, every concrete refuses bytes past a complete
///   encoding: `create_encoding` called with a concrete's selection, the
///   consumer decoding the bytes the concrete wrote followed by one extra
///   byte → `Ok(CreateEncodingSuccessReturn { output })` with
///   `output.trailing_bytes` an `Err` in the concrete's own variant of
///   `DecodeErrorReturn`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`; the bytes decoded are a
///   complete encoding followed by one `0x00` byte, so a decode that accepts
///   bytes past a complete encoding returns a value instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `trailing_bytes` of the `output` of the `Ok` arm is an `Err`,
///   by `.is_err()`, the variant differing by concrete.
/// Boundary: the crate-internal call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor where it has
///   one, and the concrete's `encode` and `decode` as the consumer calls
///   them.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, and the
///   description it carries, replaced by `MockIEncodingContract`; the
///   concretes are real, the mock selection reaching the family's official
///   mock concrete.
#[test]
fn every_concrete_refuses_bytes_past_a_complete_encoding() {
    // Arrange
    for selection in cases() {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(
            consumer,
            CreateEncodingDepsOverrides {
                lookup: Some(lookup_encoding_declaration),
            },
        );
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            selection: Some(selection),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selected concrete's construction is infallible");
        assert!(success.output.trailing_bytes.is_err());
    }
}
// every_concrete_refuses_bytes_past_a_complete_encoding

/// Contract: the own entry, every concrete refuses bytes that end inside an
///   encoding: `create_encoding` called with a concrete's selection, the
///   consumer decoding the bytes the concrete wrote with the last byte
///   removed → `Ok(CreateEncodingSuccessReturn { output })` with
///   `output.truncated_bytes` an `Err` in the concrete's own variant of
///   `DecodeErrorReturn`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`; the bytes decoded are a
///   complete encoding without its last byte, so a decode that accepts bytes
///   ending inside a field returns a value instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `truncated_bytes` of the `output` of the `Ok` arm is an `Err`,
///   by `.is_err()`, the variant differing by concrete.
/// Boundary: the crate-internal call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor where it has
///   one, and the concrete's `encode` and `decode` as the consumer calls
///   them.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, and the
///   description it carries, replaced by `MockIEncodingContract`; the
///   concretes are real, the mock selection reaching the family's official
///   mock concrete.
#[test]
fn every_concrete_refuses_bytes_that_end_inside_an_encoding() {
    // Arrange
    for selection in cases() {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(
            consumer,
            CreateEncodingDepsOverrides {
                lookup: Some(lookup_encoding_declaration),
            },
        );
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            selection: Some(selection),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selected concrete's construction is infallible");
        assert!(success.output.truncated_bytes.is_err());
    }
}
// every_concrete_refuses_bytes_that_end_inside_an_encoding

/// Contract: the own entry, every concrete names every canonical field
///   kind: `create_encoding` called with a concrete's selection, the
///   consumer asking for the name of each kind of the official
///   description's `FIELDS` → `Ok(CreateEncodingSuccessReturn { output })`
///   with `output.type_names` one `EncodingTypeName` per kind in order, no
///   two equal.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`; the field list holds
///   one kind of each, so a concrete that names two kinds alike fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `type_names` of the `output` of the `Ok` arm holds one name
///   per kind of `<MockIEncodingContract as IEncodingContract>::FIELDS` by
///   `assert_eq!` on its length, and its names are pairwise distinct by
///   `assert_eq!` on the count of the set of their text.
/// Boundary: the crate-internal call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor where it has
///   one, and the concrete's `type_name` as the consumer calls it.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, and the
///   description it carries, replaced by `MockIEncodingContract`; the
///   concretes are real, the mock selection reaching the family's official
///   mock concrete.
#[test]
fn every_concrete_names_every_canonical_field_kind() {
    // Arrange
    for selection in cases() {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(
            consumer,
            CreateEncodingDepsOverrides {
                lookup: Some(lookup_encoding_declaration),
            },
        );
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            selection: Some(selection),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selected concrete's construction is infallible");
        assert_eq!(
            success.output.type_names.len(),
            <MockIEncodingContract as IEncodingContract>::FIELDS.len()
        );
        let names: BTreeSet<&str> = success
            .output
            .type_names
            .iter()
            .map(|name| name.name())
            .collect();
        assert_eq!(
            names.len(),
            <MockIEncodingContract as IEncodingContract>::FIELDS.len()
        );
    }
}
// every_concrete_names_every_canonical_field_kind
