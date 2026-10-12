#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    AssetIdentityConstructorParamsOverrides, DeploymentIdentityConstructorParamsOverrides,
    DeploymentIdentityTryNewErrorReturn, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    PieceGeometryConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity, build_deployment_identity, build_derivation_context, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, CreateEncodingErrorReturn,
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, DecodeErrorReturn,
    DerivationContextFromFieldsErrorReturn, ENCODING_INTERFACE_VERSION, EncodeErrorReturn,
    EncodingIdentifier, IEncodingContract, MockIEncodingContract,
    MockIEncodingContractFromFieldsErrorReturn, build_canonical_fields, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_derivation_context_reference_vector,
    build_mock_i_encoding_consumer, create_encoding,
};

/// Contract: the own entry, every concrete is handed to the consumer under
///   the identifier it declares: `create_encoding` called with a concrete's
///   selection and the identifier it declares →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.declaration`
///   the declaration the consumer read through `E::DECLARATION`, whose
///   identifier is the identifier required and whose interface version is
///   `ENCODING_INTERFACE_VERSION`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes,
///   `build_create_encoding_params` overriding `concrete` and `identifier`
///   with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; a hand-over of a concrete other than the
///   selected one returns another identifier.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `declaration` of the `output` of the `Ok` arm declares the
///   case's identifier by `assert!` and the interface version
///   `ENCODING_INTERFACE_VERSION` by `assert_eq!`, each compared apart.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   consumer's read of the concrete's `E::DECLARATION`.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real
///   consumer or a real description does.
#[test]
fn every_concrete_is_handed_to_the_consumer_under_the_identifier_it_declares() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert!(success.output.declaration.identifier == selection.identifier);
        assert_eq!(
            success.output.declaration.interface_version,
            ENCODING_INTERFACE_VERSION
        );
    }
}

/// Contract: the own entry, every concrete is refused under an identifier it
///   does not declare: `create_encoding` called with a concrete's selection
///   and an identifier that concrete does not declare →
///   `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes,
///   `build_create_encoding_params` overriding `concrete` with the case's
///   selection and `identifier` with `EncodingIdentifier::Mock`, an
///   identifier no case of the declared set declares, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the pairing carries an identifier the
///   selected concrete does not declare, so an admission that compares
///   against a constant or ignores the selection admits a pairing it must
///   refuse.
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
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
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

/// Contract: the own entry, every concrete decodes what it encodes:
///   `create_encoding` called with a concrete's selection and the identifier
///   it declares, the consumer encoding a field list through the concrete
///   under the official description and decoding the bytes →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip`
///   an `Ok(DecodeSuccessReturn { described })` whose `described` equals the
///   field list encoded.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("round-trip")`,
///   `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each
///   `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`,
///   `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`,
///   and `Unsigned256` of a last byte `0x07` and every other byte `0x00`,
///   and no bytes, `build_create_encoding_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the field list differs from the builder's
///   default in every value, so a decode that returns the default field
///   list, or any value its bytes do not hold, fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `described` of the `Ok` arm of `output.round_trip` equals
///   `build_canonical_fields` overriding `values` with `Text("round-trip")`,
///   `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each
///   `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`,
///   `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`,
///   and `Unsigned256` of a last byte `0x07` and every other byte `0x00` by
///   `assert_eq!`, the expectation built apart from the field list the
///   consumer was given.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`, so the test does not prove what a real
///   consumer or a real description does.
#[test]
fn every_concrete_decodes_what_it_encodes() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
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
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert_eq!(
            success
                .output
                .round_trip
                .ok()
                .map(|decoded| decoded.described),
            Some(build_canonical_fields(CanonicalFieldsOverrides {
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
            }))
        );
    }
}

/// Contract: the own entry, every concrete refuses a description that emits
///   the wrong count: `create_encoding` called with a concrete's selection
///   and the identifier it declares, the consumer encoding a field list one
///   value short of the description's `FIELDS` →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded`
///   equal to `Err(EncodeErrorReturn::FieldCount { expected, actual })`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("short")`,
///   `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`,
///   `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, and `FixedBytes20` of
///   bytes each `0x66`, and no bytes, `build_create_encoding_params`
///   overriding `concrete` and `identifier` with the case's selection and
///   identifier, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the field list is one value
///   short, so a concrete that encodes a field list of the wrong count
///   returns bytes instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by `.err()`,
///   equals `Some(EncodeErrorReturn::FieldCount { expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(), actual: 7 })`
///   by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` as the consumer calls it.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_a_description_emitting_the_wrong_count() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
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
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert_eq!(
            success.output.encoded.err(),
            Some(EncodeErrorReturn::FieldCount {
                expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(),
                actual: 7,
            })
        );
    }
}

/// Contract: the own entry, every concrete refuses a description that emits
///   the wrong kind: `create_encoding` called with a concrete's selection
///   and the identifier it declares, the consumer encoding a field list
///   whose value at index `0` is not the kind the description's `FIELDS`
///   names there → `Ok(CreateEncodingSuccessReturn { output })` with
///   `output.encoded` equal to
///   `Err(EncodeErrorReturn::FieldKind { index, expected, actual })`.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Unsigned16(1)`,
///   `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`,
///   `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of
///   bytes each `0x66`, and `Unsigned256` of a last byte `0x08` and every
///   other byte `0x00`, and no bytes, `build_create_encoding_params`
///   overriding `concrete` and `identifier` with the case's selection and
///   identifier, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the value at index `0` is of
///   another kind, so a concrete that encodes a field list of the wrong
///   kind returns bytes instead.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by `.err()`,
///   equals `Some(EncodeErrorReturn::FieldKind { index: 0, expected: CanonicalFieldKind::Text, actual: CanonicalFieldKind::Unsigned16 })`
///   by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` as the consumer calls it.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_a_description_emitting_the_wrong_kind() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
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
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
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

/// Contract: the own entry, every concrete refuses bytes past a complete
///   encoding: `create_encoding` called with a concrete's selection and the
///   identifier it declares, the consumer decoding the bytes the concrete
///   wrote followed by one extra byte →
///   `Ok(CreateEncodingSuccessReturn { output })` with
///   `output.trailing_bytes` an `Err` in the concrete's own variant of
///   `DecodeErrorReturn`.
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
/// Assert:  the `trailing_bytes` of the `output` of the `Ok` arm is an
///   `Err`, by `.is_err()`, the variant differing by concrete.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_bytes_past_a_complete_encoding() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert!(success.output.trailing_bytes.is_err());
    }
}

/// Contract: the own entry, every concrete refuses bytes that end inside an
///   encoding: `create_encoding` called with a concrete's selection and the
///   identifier it declares, the consumer decoding the bytes the concrete
///   wrote with the last byte removed →
///   `Ok(CreateEncodingSuccessReturn { output })` with
///   `output.truncated_bytes` an `Err` in the concrete's own variant of
///   `DecodeErrorReturn`.
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
/// Assert:  the `truncated_bytes` of the `output` of the `Ok` arm is an
///   `Err`, by `.is_err()`, the variant differing by concrete.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_refuses_bytes_that_end_inside_an_encoding() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_canonical_fields(Default::default()),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert!(success.output.truncated_bytes.is_err());
    }
}

/// Contract: the own entry, every concrete returns the description's
///   refusal unchanged: `create_encoding` called with a concrete's
///   selection and the identifier it declares, the consumer encoding a
///   field list the official description refuses and decoding the bytes →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip`
///   equal to `Err(DecodeErrorReturn::Description(error))` with `error` the
///   description's refusal.
/// Arrange: for each case, `build_create_encoding_deps` over
///   `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`,
///   `build_canonical_fields` overriding `values` with `Text("")`, `Bytes`
///   of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`,
///   `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of
///   bytes each `0x66`, and `Unsigned256` of a last byte `0x08` and every
///   other byte `0x00`, and no bytes, `build_create_encoding_params`
///   overriding `concrete` and `identifier` with the case's selection and
///   identifier, and `build_create_encoding_payload`, all from the crate's
///   public surface under the `mocks` feature; the field list holds an
///   empty `Text` the description refuses, so a concrete that maps, drops,
///   or replaces the refusal fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `round_trip` of the `output` of the `Ok` arm, by `.err()`,
///   equals `Some(DecodeErrorReturn::Description(MockIEncodingContractFromFieldsErrorReturn::EmptyText { index: 0 }))`
///   by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route `create_encoding`,
///   the selected concrete's constructor where it has one, and the
///   concrete's `encode` and `decode` as the consumer calls them.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   and the description it carries, replaced by the official
///   `MockIEncodingContract`.
#[test]
fn every_concrete_returns_the_description_refusal_unchanged() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = MockIEncodingContract;
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Text(String::new()),
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
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert_eq!(
            success.output.round_trip.err(),
            Some(DecodeErrorReturn::Description(
                MockIEncodingContractFromFieldsErrorReturn::EmptyText { index: 0 }
            ))
        );
    }
}

/// Contract: the carried entry `encoding/abi` reference context encodes to
///   the reference vector, in this node's terms: `create_encoding` called
///   with the selection of a concrete that declares
///   `EncodingIdentifier::EthereumAbiV1` and that identifier, the consumer
///   encoding the reference context through the concrete under the real
///   description → `Ok(CreateEncodingSuccessReturn { output })` with
///   `output.encoded` an `Ok(EncodeSuccessReturn { bytes })` with `bytes`
///   the reference vector.
/// Arrange: for each case whose identifier is
///   `EncodingIdentifier::EthereumAbiV1`,
///   `build_derivation_context_description`, `build_create_encoding_deps`
///   over `build_mock_i_encoding_consumer` taking that description, the
///   reference context built as `build_derivation_context` overriding the
///   asset with `build_asset_identity` at name `left-pad` and version
///   `1.3.0`, the deployment with `build_deployment_identity` at bytes each
///   `0x44`, the suite with `build_suite_identifier` at identifier bytes
///   each `0x55` and version `0x0102`, the parameter set with
///   `build_parameter_set_identifier` at bytes each `0x66`, the group index
///   with `build_group_index` at value `2`, and the geometry with
///   `build_piece_geometry` at piece size `32768`, piece-group size
///   `65536`, and extent `262144`, and no bytes,
///   `build_create_encoding_params` overriding `concrete` and `identifier`
///   with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the reference context's text fields are
///   dynamic items between static items and its same-kind fields differ, so
///   a swapped field, a wrong word width, or a misplaced tail changes the
///   bytes.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `encoded` of the `output` of the `Ok` arm, by
///   `.ok().map(|success| success.bytes)`, equals `Some` of the reference
///   vector's bytes by `assert_eq!`, the reference vector being the bytes
///   of the `abi.encode` parameter encoding of `Text("left-pad")`,
///   `Text("1.3.0")`, `FixedBytes32` of bytes each `0x44`, `FixedBytes32`
///   of bytes each `0x55`, `Unsigned16(0x0102)`, `FixedBytes32` of bytes
///   each `0x66`, `Unsigned64(2)`, `Unsigned32(32768)`, `Unsigned32(65536)`,
///   and `Unsigned64(262144)`, the bytes
///   `build_derivation_context_reference_vector` returns from the crate's
///   public surface, a hex string authored by hand from the Solidity
///   Contract ABI Specification with one head word per field, the text
///   offsets in their head words, and each length and zero-padded bytes in
///   the tail, and not produced by running `alloy` or this crate; the
///   expectation is not read back from the arrangement.
/// Boundary: the public call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor, `encode`,
///   `DerivationContextDescription::to_fields`, and `domain`'s accessors.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   so the test does not prove what a real consumer does with the
///   concrete; the description and `domain` run as themselves.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector()
 {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
    {
        let description = build_derivation_context_description(Default::default());
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_derivation_context(DerivationContextConstructorParamsOverrides {
                asset: Some(build_asset_identity(
                    AssetIdentityConstructorParamsOverrides {
                        name: Some("left-pad".to_string()),
                        version: Some("1.3.0".to_string()),
                    },
                )),
                deployment: Some(build_deployment_identity(
                    DeploymentIdentityConstructorParamsOverrides {
                        bytes: Some([0x44; 32]),
                    },
                )),
                suite: Some(build_suite_identifier(
                    SuiteIdentifierConstructorParamsOverrides {
                        identifier: Some([0x55; 32]),
                        version: Some(0x0102),
                    },
                )),
                parameter_set: Some(build_parameter_set_identifier(
                    ParameterSetIdentifierConstructorParamsOverrides {
                        bytes: Some([0x66; 32]),
                    },
                )),
                group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                    value: Some(2),
                })),
                geometry: Some(build_piece_geometry(
                    PieceGeometryConstructorParamsOverrides {
                        piece_size: Some(32768),
                        piece_group_size: Some(65536),
                        total_extent: Some(262144),
                    },
                )),
            }),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert_eq!(
            success.output.encoded.ok().map(|encoded| encoded.bytes),
            Some(build_derivation_context_reference_vector())
        );
    }
}

/// Contract: the carried entry `encoding/abi` reference vector decodes to
///   the reference context, in this node's terms: `create_encoding` called
///   with the selection of a concrete that declares
///   `EncodingIdentifier::EthereumAbiV1` and that identifier, the consumer
///   decoding the reference vector under the real description →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.decoded` an
///   `Ok(DecodeSuccessReturn { described })` with `described` equal to the
///   reference context.
/// Arrange: for each case whose identifier is
///   `EncodingIdentifier::EthereumAbiV1`,
///   `build_derivation_context_description`, `build_create_encoding_deps`
///   over `build_mock_i_encoding_consumer` taking that description, the
///   reference context built as `build_derivation_context` overriding the
///   asset with `build_asset_identity` at name `left-pad` and version
///   `1.3.0`, the deployment with `build_deployment_identity` at bytes each
///   `0x44`, the suite with `build_suite_identifier` at identifier bytes
///   each `0x55` and version `0x0102`, the parameter set with
///   `build_parameter_set_identifier` at bytes each `0x66`, the group index
///   with `build_group_index` at value `2`, and the geometry with
///   `build_piece_geometry` at piece size `32768`, piece-group size
///   `65536`, and extent `262144`, and the reference vector's bytes, the
///   `abi.encode` parameter encoding of `Text("left-pad")`, `Text("1.3.0")`,
///   `FixedBytes32` of bytes each `0x44`, `FixedBytes32` of bytes each
///   `0x55`, `Unsigned16(0x0102)`, `FixedBytes32` of bytes each `0x66`,
///   `Unsigned64(2)`, `Unsigned32(32768)`, `Unsigned32(65536)`, and
///   `Unsigned64(262144)`, the bytes
///   `build_derivation_context_reference_vector` returns from the crate's
///   public surface, a hex string authored by hand from the Solidity
///   Contract ABI Specification with one head word per field, the text
///   offsets in their head words, and each length and zero-padded bytes in
///   the tail, and not produced by running `alloy` or this crate,
///   `build_create_encoding_params` overriding
///   `concrete` and `identifier` with the case's selection and identifier,
///   and `build_create_encoding_payload`, all from the crate's public
///   surface under the `mocks` feature; the reference context's same-kind
///   fields differ, so a field read from the wrong word or tail yields a
///   refusal or an unequal context.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `decoded` of the `output` of the `Ok` arm, by
///   `.ok().map(|success| success.described)`, equals `Some` of the
///   reference context built again by `build_derivation_context` with the
///   same overrides, by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor, `decode`,
///   `DerivationContextDescription::fields_to_value`, `domain`'s
///   constructors, and the canonical re-encoding through `encode`,
///   `DerivationContextDescription::to_fields`, and `domain`'s accessors.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   so the test does not prove what a real consumer does with the
///   concrete; the description and `domain` run as themselves.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context()
 {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
    {
        let description = build_derivation_context_description(Default::default());
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_derivation_context(DerivationContextConstructorParamsOverrides {
                asset: Some(build_asset_identity(
                    AssetIdentityConstructorParamsOverrides {
                        name: Some("left-pad".to_string()),
                        version: Some("1.3.0".to_string()),
                    },
                )),
                deployment: Some(build_deployment_identity(
                    DeploymentIdentityConstructorParamsOverrides {
                        bytes: Some([0x44; 32]),
                    },
                )),
                suite: Some(build_suite_identifier(
                    SuiteIdentifierConstructorParamsOverrides {
                        identifier: Some([0x55; 32]),
                        version: Some(0x0102),
                    },
                )),
                parameter_set: Some(build_parameter_set_identifier(
                    ParameterSetIdentifierConstructorParamsOverrides {
                        bytes: Some([0x66; 32]),
                    },
                )),
                group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                    value: Some(2),
                })),
                geometry: Some(build_piece_geometry(
                    PieceGeometryConstructorParamsOverrides {
                        piece_size: Some(32768),
                        piece_group_size: Some(65536),
                        total_extent: Some(262144),
                    },
                )),
            }),
            build_derivation_context_reference_vector(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert_eq!(
            success.output.decoded.ok().map(|decoded| decoded.described),
            Some(build_derivation_context(
                DerivationContextConstructorParamsOverrides {
                    asset: Some(build_asset_identity(
                        AssetIdentityConstructorParamsOverrides {
                            name: Some("left-pad".to_string()),
                            version: Some("1.3.0".to_string()),
                        },
                    )),
                    deployment: Some(build_deployment_identity(
                        DeploymentIdentityConstructorParamsOverrides {
                            bytes: Some([0x44; 32]),
                        },
                    )),
                    suite: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x55; 32]),
                            version: Some(0x0102),
                        },
                    )),
                    parameter_set: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x66; 32]),
                        },
                    )),
                    group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                        value: Some(2),
                    },)),
                    geometry: Some(build_piece_geometry(
                        PieceGeometryConstructorParamsOverrides {
                            piece_size: Some(32768),
                            piece_group_size: Some(65536),
                            total_extent: Some(262144),
                        },
                    )),
                },
            ))
        );
    }
}

/// Contract: the carried entry `encoding/abi` a context survives the ABI
///   round trip, in this node's terms: `create_encoding` called with a
///   concrete's selection and the identifier it declares, the consumer
///   encoding the wide context through the concrete under the real
///   description and decoding the bytes →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip`
///   an `Ok(DecodeSuccessReturn { described })` with `described` equal to
///   the wide context.
/// Arrange: for each case, `build_derivation_context_description`,
///   `build_create_encoding_deps` over `build_mock_i_encoding_consumer`
///   taking that description, the wide context built as
///   `build_derivation_context` overriding the asset with
///   `build_asset_identity` at name `left-pad` and version `1.3.0`, the
///   deployment with `build_deployment_identity` at bytes each `0x44`, the
///   suite with `build_suite_identifier` at identifier bytes each `0x55`
///   and version `u16::MAX`, the parameter set with
///   `build_parameter_set_identifier` at bytes each `0x66`, the group index
///   with `build_group_index` at value `4294967296`, and the geometry with
///   `build_piece_geometry` at piece size `16384`, piece-group size
///   `2147483648`, and extent `u64::MAX`, and no bytes,
///   `build_create_encoding_params` overriding `concrete` and `identifier`
///   with the case's selection and identifier, and
///   `build_create_encoding_payload`, all from the crate's public surface
///   under the `mocks` feature; the same-kind fields hold distinct values
///   and the widths are full, so a swap of same-kind fields, a dropped
///   field, or a narrowed width yields a refusal or an unequal context.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `round_trip` of the `output` of the `Ok` arm, by
///   `.ok().map(|success| success.described)`, equals `Some` of the wide
///   context built again by `build_derivation_context` with the same
///   overrides, by `assert_eq!`.
/// Boundary: the public call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor where it has
///   one, `encode`, `decode`, `DerivationContextDescription::to_fields` and
///   `fields_to_value`, and `domain`'s accessors and constructors.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   so the test does not prove what a real consumer does with the
///   concrete; the description and `domain` run as themselves.
#[test]
fn every_concrete_round_trips_a_context_through_the_real_description() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS {
        let description = build_derivation_context_description(Default::default());
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_derivation_context(DerivationContextConstructorParamsOverrides {
                asset: Some(build_asset_identity(
                    AssetIdentityConstructorParamsOverrides {
                        name: Some("left-pad".to_string()),
                        version: Some("1.3.0".to_string()),
                    },
                )),
                deployment: Some(build_deployment_identity(
                    DeploymentIdentityConstructorParamsOverrides {
                        bytes: Some([0x44; 32]),
                    },
                )),
                suite: Some(build_suite_identifier(
                    SuiteIdentifierConstructorParamsOverrides {
                        identifier: Some([0x55; 32]),
                        version: Some(u16::MAX),
                    },
                )),
                parameter_set: Some(build_parameter_set_identifier(
                    ParameterSetIdentifierConstructorParamsOverrides {
                        bytes: Some([0x66; 32]),
                    },
                )),
                group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                    value: Some(4294967296),
                })),
                geometry: Some(build_piece_geometry(
                    PieceGeometryConstructorParamsOverrides {
                        piece_size: Some(16384),
                        piece_group_size: Some(2147483648),
                        total_extent: Some(u64::MAX),
                    },
                )),
            }),
            Vec::new(),
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert_eq!(
            success
                .output
                .round_trip
                .ok()
                .map(|decoded| decoded.described),
            Some(build_derivation_context(
                DerivationContextConstructorParamsOverrides {
                    asset: Some(build_asset_identity(
                        AssetIdentityConstructorParamsOverrides {
                            name: Some("left-pad".to_string()),
                            version: Some("1.3.0".to_string()),
                        },
                    )),
                    deployment: Some(build_deployment_identity(
                        DeploymentIdentityConstructorParamsOverrides {
                            bytes: Some([0x44; 32]),
                        },
                    )),
                    suite: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x55; 32]),
                            version: Some(u16::MAX),
                        },
                    )),
                    parameter_set: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x66; 32]),
                        },
                    )),
                    group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                        value: Some(4294967296),
                    },)),
                    geometry: Some(build_piece_geometry(
                        PieceGeometryConstructorParamsOverrides {
                            piece_size: Some(16384),
                            piece_group_size: Some(2147483648),
                            total_extent: Some(u64::MAX),
                        },
                    )),
                },
            ))
        );
    }
}

/// Contract: the carried entry `encoding/abi` a value the description
///   refuses is refused with the description's refusal, in this node's
///   terms: `create_encoding` called with the selection of a concrete that
///   declares `EncodingIdentifier::EthereumAbiV1` and that identifier, the
///   consumer decoding a canonical encoding whose deployment-identity word
///   is all zero under the real description →
///   `Ok(CreateEncodingSuccessReturn { output })` with `output.decoded`
///   equal to `Err(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))`.
/// Arrange: for each case whose identifier is
///   `EncodingIdentifier::EthereumAbiV1`,
///   `build_derivation_context_description`, `build_create_encoding_deps`
///   over `build_mock_i_encoding_consumer` taking that description, the
///   reference context built as `build_derivation_context` overriding the
///   asset with `build_asset_identity` at name `left-pad` and version
///   `1.3.0`, the deployment with `build_deployment_identity` at bytes each
///   `0x44`, the suite with `build_suite_identifier` at identifier bytes
///   each `0x55` and version `0x0102`, the parameter set with
///   `build_parameter_set_identifier` at bytes each `0x66`, the group index
///   with `build_group_index` at value `2`, and the geometry with
///   `build_piece_geometry` at piece size `32768`, piece-group size
///   `65536`, and extent `262144`, and the bytes of
///   `build_derivation_context_reference_vector` with the
///   deployment-identity word replaced by zero bytes, the canonical
///   encoding of the same ten values with `FixedBytes32` of zero bytes in
///   place of the deployment identity's bytes,
///   `build_create_encoding_params` overriding
///   `concrete` and `identifier` with the case's selection and identifier,
///   and `build_create_encoding_payload`, all from the crate's public
///   surface under the `mocks` feature; the encoding is canonical and
///   well-kinded, so the ABI decode and the canonical comparison admit it
///   and only the domain's constructor refuses.
/// Act:     `create_encoding(&deps, params, payload)` for the case.
/// Assert:  the `decoded` of the `output` of the `Ok` arm, by `.err()`,
///   equals `Some(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))`
///   by `assert_eq!`, the whole refusal the deployment identity's
///   constructor declares for all-zero bytes.
/// Boundary: the public call `create_encoding`; the route
///   `create_encoding`, the selected concrete's constructor, `decode`, the
///   item pass, `DerivationContextDescription::fields_to_value`, and
///   `DeploymentIdentity::try_new`.
/// Mocked:  the consumer, replaced by the official `MockIEncodingConsumer`,
///   so the test does not prove what a real consumer does with the
///   concrete; the description and `domain` run as themselves.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_returns_the_domain_refusal_unchanged() {
    // Arrange
    for selection in DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
    {
        let description = build_derivation_context_description(Default::default());
        let mut bytes = build_derivation_context_reference_vector();
        bytes[64..96].fill(0);
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_derivation_context(DerivationContextConstructorParamsOverrides {
                asset: Some(build_asset_identity(
                    AssetIdentityConstructorParamsOverrides {
                        name: Some("left-pad".to_string()),
                        version: Some("1.3.0".to_string()),
                    },
                )),
                deployment: Some(build_deployment_identity(
                    DeploymentIdentityConstructorParamsOverrides {
                        bytes: Some([0x44; 32]),
                    },
                )),
                suite: Some(build_suite_identifier(
                    SuiteIdentifierConstructorParamsOverrides {
                        identifier: Some([0x55; 32]),
                        version: Some(0x0102),
                    },
                )),
                parameter_set: Some(build_parameter_set_identifier(
                    ParameterSetIdentifierConstructorParamsOverrides {
                        bytes: Some([0x66; 32]),
                    },
                )),
                group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                    value: Some(2),
                })),
                geometry: Some(build_piece_geometry(
                    PieceGeometryConstructorParamsOverrides {
                        piece_size: Some(32768),
                        piece_group_size: Some(65536),
                        total_extent: Some(262144),
                    },
                )),
            }),
            bytes,
        );
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the consumer's output");
        assert_eq!(
            success.output.decoded.err(),
            Some(DecodeErrorReturn::Description(
                DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
                    DeploymentIdentityTryNewErrorReturn::AllZero
                )
            ))
        );
    }
}
