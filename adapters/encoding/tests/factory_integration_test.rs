#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use encoding::{
    AdmitEncodingSelectionDepsOverrides, AdmitEncodingSelectionParamsOverrides, CanonicalFieldKind,
    CanonicalFieldReaderErrorReturn, CanonicalFieldValue, CanonicalFieldsOverrides,
    CreateEncodingDepsOverrides, CreateEncodingParamsOverrides, ENCODING_CONCRETES,
    ENCODING_IDENTIFIERS, ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodingConcrete,
    EncodingConcreteOverrides, EncodingIdentifier, IEncodingContract,
    LookupEncodingDeclarationDeps, LookupEncodingDeclarationParams,
    LookupEncodingDeclarationPayload, MockIEncodingContract, admit_encoding_selection,
    build_admit_encoding_selection_deps, build_admit_encoding_selection_params,
    build_canonical_fields, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_encoding_concrete, build_mock_i_encoding_consumer,
    create_encoding, lookup_encoding_declaration,
};
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

fn cases() -> impl Iterator<Item = EncodingConcrete> {
    ENCODING_CONCRETES
        .iter()
        .map(|member| build_encoding_concrete(*member, EncodingConcreteOverrides))
}

fn ethereum_abi_cases() -> impl Iterator<Item = EncodingConcrete> {
    cases().filter(|selection| {
        lookup_encoding_declaration(
            &LookupEncodingDeclarationDeps,
            LookupEncodingDeclarationParams {
                concrete: *selection,
            },
            LookupEncodingDeclarationPayload,
        )
        .is_ok_and(|looked_up| {
            looked_up.declaration.identifier == EncodingIdentifier::EthereumAbiV1
        })
    })
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
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; a declaration read for
///   another selection than the one that chose the adapter returns another
///   `concrete`.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `declaration` of the `output` of the `Ok` arm has `concrete`
///   equal to the case's selection by `assert_eq!`, its `identifier`
///   contained in `ENCODING_IDENTIFIERS` by `assert!`, and its
///   `interface_version` equal to `ENCODING_INTERFACE_VERSION` by
///   `assert_eq!`, each compared apart, `type_name` not compared.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the real lookup its deps hold, the selected concrete's constructor
///   where it has one, and the declaration the consumer receives in its
///   payload.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real consumer
///   or a real description does.
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

/// Contract: the own entry, an admitted selection is handed to the consumer
///   under its declaration: `admit_encoding_selection` called with a member
///   of `ENCODING_CONCRETES` and `serde_json::Value::Null`, then
///   `create_encoding` called with the selection it returns →
///   `Ok(CreateEncodingSuccessReturn { output })` with
///   `output.declaration.concrete` equal to the member passed to the
///   admission.
/// Arrange: for each case, the admission's result from
///   `admit_encoding_selection` over `build_admit_encoding_selection_deps`,
///   `build_admit_encoding_selection_params` overriding `concrete` with the
///   case's selection, and the payload `serde_json::Value::Null`, its
///   `selection` taken from the `Ok` arm; `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the admitted
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the expected `concrete` is
///   the case's member, written apart from the admission's result, so an
///   admission that returns another member, or a factory that hands another
///   concrete, returns another `concrete`.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `declaration` of the `output` of the `Ok` arm has `concrete`
///   equal to the case's member by `assert_eq!`.
/// Boundary: the public calls `admit_encoding_selection` and
///   `create_encoding`; the route `admit_encoding_selection`, then
///   `create_encoding`, the real lookup its deps hold, the selected
///   concrete's constructor, and the declaration the consumer receives in
///   its payload.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real consumer
///   or a real description does.
#[test]
fn an_admitted_selection_is_handed_to_the_consumer_under_its_declaration() {
    // Arrange
    for member in cases() {
        let admitted = admit_encoding_selection(
            &build_admit_encoding_selection_deps(AdmitEncodingSelectionDepsOverrides),
            build_admit_encoding_selection_params(AdmitEncodingSelectionParamsOverrides {
                concrete: Some(member),
            }),
            serde_json::Value::Null,
        )
        .expect("a declared member with no setting supplied is admitted")
        .selection;
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
            selection: Some(admitted),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("the selected concrete's construction is infallible");
        assert_eq!(success.output.declaration.concrete, member);
    }
}
// an_admitted_selection_is_handed_to_the_consumer_under_its_declaration

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
///   and `Unsigned256` of a last byte `0x07` and every other byte `0x00`,
///   and no bytes, overriding `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the field list differs from
///   the builder's default in every value, so a decode that returns the
///   default field list, or any value its bytes do not hold, fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `described` of the `Ok` arm of `output.round_trip` equals
///   `build_canonical_fields` overriding `values` with the same field list
///   by `assert_eq!`, the expectation built apart from the field list the
///   consumer was given.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real consumer
///   or a real description does.
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
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the field list is one value short, so a
///   concrete that encodes a field list the reader refuses returns bytes
///   instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals
///   `Some(EncodeErrorReturn::Fields(CanonicalFieldReaderErrorReturn::Count {
///   expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
///   actual: 7 }))` by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, the concrete's
///   `encode` as the consumer calls it, and the canonical field reader it
///   constructs.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
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
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the bytes decoded are a
///   complete encoding followed by one `0x00` byte, so a decode that accepts
///   bytes past a complete encoding returns a value instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `trailing_bytes` of the `output` of the `Ok` arm is an `Err`,
///   by `.is_err()`, the variant differing by concrete.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
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
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the bytes decoded are a
///   complete encoding without its last byte, so a decode that accepts
///   bytes ending inside a field returns a value instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `truncated_bytes` of the `output` of the `Ok` arm is an `Err`,
///   by `.is_err()`, the variant differing by concrete.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
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

/// Contract: the own entry, every concrete declaring the Ethereum ABI
///   identifier encodes the reference fields to the reference vector:
///   `create_encoding` called with the selection of a concrete whose
///   declaration `lookup_encoding_declaration` returns carries
///   `EncodingIdentifier::EthereumAbiV1`, the consumer encoding the
///   reference fields under the official description →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` an
///   `Ok(EncodeSuccessReturn { bytes })` with `bytes` the reference vector.
/// Arrange: for each case whose looked-up declaration carries
///   `EncodingIdentifier::EthereumAbiV1`, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the default values are
///   distinct in every kind, so a swapped field, a wrong word width, or a
///   misplaced tail changes the bytes.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by
///   `.ok().map(|success| success.bytes)`, equals `Some` of the reference
///   vector's bytes by `assert_eq!`, the reference vector being the bytes of
///   the `abi.encode` parameter encoding of `Text("left-pad")`, `Bytes` of
///   the bytes `0xa1`, `0xa2`, and `0xa3`, `FixedBytes32` of bytes each
///   `0x11`, `Unsigned16(0x0102)`, `Unsigned32(0x01020304)`,
///   `Unsigned64(0x0102030405060708)`, `FixedBytes20` of bytes each `0x22`,
///   and `Unsigned256` of a first byte `0x80` and every other byte `0x00`, a
///   hex string authored by hand from the Solidity Contract ABI
///   Specification and decoded by `hex::decode`, and not produced by running
///   `alloy` or this crate.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor, and `encode`.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_fields_to_the_reference_vector()
 {
    // Arrange
    for selection in ethereum_abi_cases() {
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
            success.output.encoded.ok().map(|encoded| encoded.bytes),
            Some(reference_vector())
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_fields_to_the_reference_vector

/// Contract: the own entry, every concrete declaring the Ethereum ABI
///   identifier decodes the reference vector to the reference fields:
///   `create_encoding` called with the selection of a concrete whose
///   declaration `lookup_encoding_declaration` returns carries
///   `EncodingIdentifier::EthereumAbiV1`, the consumer decoding the
///   reference vector under the official description →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.decoded` an
///   `Ok(DecodeSuccessReturn { described })` with `described` equal to the
///   reference fields.
/// Arrange: for each case whose looked-up declaration carries
///   `EncodingIdentifier::EthereumAbiV1`, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and the reference vector's
///   bytes, the `abi.encode` parameter encoding of the default values, a hex
///   string authored by hand from the Solidity Contract ABI Specification
///   and decoded by `hex::decode`, and not produced by running `alloy` or
///   this crate, overriding `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the default values are
///   distinct in every kind, so a field read from the wrong word or tail
///   yields a refusal or an unequal field list.
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
    for selection in ethereum_abi_cases() {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(Default::default()),
            reference_vector(),
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
            success.output.decoded.ok().map(|decoded| decoded.described),
            Some(build_canonical_fields(Default::default()))
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_fields

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
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `type_names` of the `output` of the `Ok` arm holds one name
///   per kind of `<MockIEncodingContract as IEncodingContract>::FIELDS` by
///   `assert_eq!` on its length, and its names are pairwise distinct by
///   `assert_eq!` on the count of the set of their text.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `type_name` as the consumer calls it.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
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

/// Contract: the own entry, every concrete declaring the Ethereum ABI
///   identifier names every kind by its ABI type: `create_encoding` called
///   with the selection of a concrete whose declaration
///   `lookup_encoding_declaration` returns carries
///   `EncodingIdentifier::EthereumAbiV1`, the consumer asking for the name
///   of each kind of the official description's `FIELDS` →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.type_names`
///   equal, in order, to `string`, `bytes`, `bytes32`, `uint16`, `uint32`,
///   `uint64`, `bytes20`, and `uint256`.
/// Arrange: for each case whose looked-up declaration carries
///   `EncodingIdentifier::EthereumAbiV1`, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_encoding_declaration`,
///   `build_create_encoding_params` overriding `selection` with the case's
///   selection, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the official description
///   holds one kind of each, so a swapped or missing name changes the list.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the text of each name of the `type_names` of the `output` of
///   the `Ok` arm equals, in order, `string`, `bytes`, `bytes32`, `uint16`,
///   `uint32`, `uint64`, `bytes20`, and `uint256` by `assert_eq!`, the
///   expectation written from the ABI specification and not read back from
///   the arrangement, and the name of the fixed twenty-byte kind does not
///   equal `address` by `assert_ne!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor, and `type_name`.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_names_every_kind_by_its_abi_type() {
    // Arrange
    for selection in ethereum_abi_cases() {
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
        let names: Vec<&str> = success
            .output
            .type_names
            .iter()
            .map(|name| name.name())
            .collect();
        assert_eq!(
            names,
            vec![
                "string", "bytes", "bytes32", "uint16", "uint32", "uint64", "bytes20", "uint256"
            ]
        );
        let position = <MockIEncodingContract as IEncodingContract>::FIELDS
            .iter()
            .position(|kind| *kind == CanonicalFieldKind::FixedBytes20)
            .expect("the official description names the fixed twenty-byte kind");
        assert_ne!(names[position], "address");
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_names_every_kind_by_its_abi_type
