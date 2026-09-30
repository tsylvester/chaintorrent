#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{AbiDecoderErrorReturn, AbiEncoding, AbiEncodingConstructorParams};
use crate::derivation_context::provides::{
    DerivationContextDescription, DerivationContextDescriptionConstructorParams,
    DerivationContextFromFieldsErrorReturn,
};
use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, DecodeErrorReturn, DecodeParams,
    ENCODING_INTERFACE_VERSION, EncodeParams, EncodingIdentifier, FromFieldsParams,
    FromFieldsReturn, FromFieldsSuccessReturn, IDecoderAdapter, IEncoderAdapter, IEncodingContract,
    ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn,
};
use domain::{
    AssetIdentityConstructorParamsOverrides, DeploymentIdentityConstructorParamsOverrides,
    DeploymentIdentityTryNewErrorReturn, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    PieceGeometryConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity, build_deployment_identity, build_derivation_context, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_suite_identifier,
};
use hex::decode;

const REFERENCE_VECTOR_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000140",
    "0000000000000000000000000000000000000000000000000000000000000180",
    "0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a",
    "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
    "0000000000000000000000000000000000000000000000000000000000000003",
    "0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c",
    "0000000000000000000000000000000000000000000000000000000000000005",
    "0000000000000000000000000000000000000000000000000000000000004000",
    "0000000000000000000000000000000000000000000000000000000000008000",
    "0000000000000000000000000000000000000000000000000000000000100000",
    "0000000000000000000000000000000000000000000000000000000000000016",
    "4073636f70652f6578616d706c652d7061636b61676500000000000000000000",
    "000000000000000000000000000000000000000000000000000000000000000c",
    "322e312e302d626574612e330000000000000000000000000000000000000000",
);

const FIELD_TOO_MANY_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000160",
    "00000000000000000000000000000000000000000000000000000000000001a0",
    "0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a",
    "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
    "0000000000000000000000000000000000000000000000000000000000000003",
    "0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c",
    "0000000000000000000000000000000000000000000000000000000000000005",
    "0000000000000000000000000000000000000000000000000000000000004000",
    "0000000000000000000000000000000000000000000000000000000000008000",
    "0000000000000000000000000000000000000000000000000000000000100000",
    "0000000000000000000000000000000000000000000000000000000000000001",
    "0000000000000000000000000000000000000000000000000000000000000016",
    "4073636f70652f6578616d706c652d7061636b61676500000000000000000000",
    "000000000000000000000000000000000000000000000000000000000000000c",
    "322e312e302d626574612e330000000000000000000000000000000000000000",
);

const BYTES_PROBE_VECTOR_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000040",
    "0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d0d",
    "0000000000000000000000000000000000000000000000000000000000000005",
    "0102030405000000000000000000000000000000000000000000000000000000",
);

const WIDTHS_PROBE_VECTOR_HEX: &str = concat!(
    "1111111111111111111111111111111111111111000000000000000000000000",
    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
);

#[derive(Debug, PartialEq, Eq)]
struct BytesProbe {
    bytes: Vec<u8>,
    word: [u8; 32],
}

#[derive(Debug)]
enum BytesProbeFromFieldsErrorReturn {
    Shape,
}

struct BytesProbeDescription;

impl IEncodingContract for BytesProbeDescription {
    type Described = BytesProbe;
    type FromFieldsErrorReturn = BytesProbeFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] =
        &[CanonicalFieldKind::Bytes, CanonicalFieldKind::FixedBytes32];

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(payload.bytes.clone()),
                    CanonicalFieldValue::FixedBytes32(payload.word),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<BytesProbe, BytesProbeFromFieldsErrorReturn> {
        let Ok([bytes_value, word_value]): Result<
            [CanonicalFieldValue; 2],
            Vec<CanonicalFieldValue>,
        > = payload.values.try_into() else {
            return Err(BytesProbeFromFieldsErrorReturn::Shape);
        };
        let CanonicalFieldValue::Bytes(bytes) = bytes_value else {
            return Err(BytesProbeFromFieldsErrorReturn::Shape);
        };
        let CanonicalFieldValue::FixedBytes32(word) = word_value else {
            return Err(BytesProbeFromFieldsErrorReturn::Shape);
        };
        Ok(FromFieldsSuccessReturn {
            described: BytesProbe { bytes, word },
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct WidthsProbe {
    party: [u8; 20],
    integer: [u8; 32],
}

#[derive(Debug)]
enum WidthsProbeFromFieldsErrorReturn {
    Shape,
}

struct WidthsProbeDescription;

impl IEncodingContract for WidthsProbeDescription {
    type Described = WidthsProbe;
    type FromFieldsErrorReturn = WidthsProbeFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::FixedBytes20,
        CanonicalFieldKind::Unsigned256,
    ];

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::FixedBytes20(payload.party),
                    CanonicalFieldValue::Unsigned256(payload.integer),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<WidthsProbe, WidthsProbeFromFieldsErrorReturn> {
        let Ok([party_value, integer_value]): Result<
            [CanonicalFieldValue; 2],
            Vec<CanonicalFieldValue>,
        > = payload.values.try_into() else {
            return Err(WidthsProbeFromFieldsErrorReturn::Shape);
        };
        let CanonicalFieldValue::FixedBytes20(party) = party_value else {
            return Err(WidthsProbeFromFieldsErrorReturn::Shape);
        };
        let CanonicalFieldValue::Unsigned256(integer) = integer_value else {
            return Err(WidthsProbeFromFieldsErrorReturn::Shape);
        };
        Ok(FromFieldsSuccessReturn {
            described: WidthsProbe { party, integer },
        })
    }
}

/// Contract: a described value encodes as `abi.encode` over its canonical
///   fields.
/// Arrange: the reference context — asset name "@scope/example-package",
///   asset version "2.1.0-beta.3", deployment 0x0a×32, suite 0x0b×32 version
///   3, parameter set 0x0c×32, group index 5, geometry 16384/32768/1048576.
/// Act:     `encoding.encode(EncodeParams { description: &description }, &context)`.
/// Assert:  `success.bytes` equals the reference vector.
#[test]
fn encode_writes_the_reference_context_as_its_abi_parameter_encoding() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        asset: Some(build_asset_identity(
            AssetIdentityConstructorParamsOverrides {
                name: Some("@scope/example-package".to_string()),
                version: Some("2.1.0-beta.3".to_string()),
            },
        )),
        deployment: Some(build_deployment_identity(
            DeploymentIdentityConstructorParamsOverrides {
                bytes: Some([0x0a; 32]),
            },
        )),
        suite: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x0b; 32]),
                version: Some(3),
            },
        )),
        parameter_set: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x0c; 32]),
            },
        )),
        group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
            value: Some(5),
        })),
        geometry: Some(build_piece_geometry(
            PieceGeometryConstructorParamsOverrides {
                piece_size: Some(16384),
                piece_group_size: Some(32768),
                total_extent: Some(1048576),
            },
        )),
    });
    let Ok(bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };

    // Act
    let Ok(success) = encoding.encode(
        EncodeParams {
            description: &description,
        },
        &context,
    );

    // Assert
    assert_eq!(success.bytes, bytes);
}

/// Contract: the canonical encoding decodes to the value it encodes.
/// Arrange: the reference vector.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `success.described` equals the reference context.
#[test]
fn decode_reads_the_reference_vector_as_the_reference_context() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        asset: Some(build_asset_identity(
            AssetIdentityConstructorParamsOverrides {
                name: Some("@scope/example-package".to_string()),
                version: Some("2.1.0-beta.3".to_string()),
            },
        )),
        deployment: Some(build_deployment_identity(
            DeploymentIdentityConstructorParamsOverrides {
                bytes: Some([0x0a; 32]),
            },
        )),
        suite: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x0b; 32]),
                version: Some(3),
            },
        )),
        parameter_set: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x0c; 32]),
            },
        )),
        group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
            value: Some(5),
        })),
        geometry: Some(build_piece_geometry(
            PieceGeometryConstructorParamsOverrides {
                piece_size: Some(16384),
                piece_group_size: Some(32768),
                total_extent: Some(1048576),
            },
        )),
    });
    let Ok(bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };

    // Act
    let Ok(success) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        &bytes,
    ) else {
        panic!("the canonical encoding decodes to the value it encodes")
    };

    // Assert
    assert_eq!(success.described, context);
}

/// Contract: input missing the dynamic tail is refused by the ABI decoder.
/// Arrange: the reference vector's first 0x140 bytes.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   Malformed(_))`.
#[test]
fn decode_rejects_input_truncated_to_the_head() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let Ok(vector) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };
    let bytes = &vector[..0x140];

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        bytes,
    ) else {
        panic!("input truncated to the head is refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(_))
    ));
}

/// Contract: bytes past the canonical end are refused.
/// Arrange: the reference vector followed by 32 zero bytes.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   NonCanonical)`.
#[test]
fn decode_rejects_trailing_bytes() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let Ok(mut bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };
    bytes.extend([0u8; 32]);

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        &bytes,
    ) else {
        panic!("trailing bytes are refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)
    ));
}

/// Contract: a string's padding is zero in its one byte form.
/// Arrange: the reference vector with byte 447, the asset version's last
///   padding byte, set to 0x01.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   NonCanonical)`.
#[test]
fn decode_rejects_nonzero_string_padding() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let Ok(mut bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };
    bytes[447] = 0x01;

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        &bytes,
    ) else {
        panic!("nonzero string padding is refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)
    ));
}

/// Contract: an integer word with bits above its field's width is refused,
///   naming the field.
/// Arrange: the reference vector with byte 157 set to 0x01, the suite
///   version's word reading 0x010003.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   ValueOutOfRange { index: 4, kind: CanonicalFieldKind::Unsigned16 })`.
#[test]
fn decode_rejects_a_uint16_word_wider_than_sixteen_bits() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let Ok(mut bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };
    bytes[157] = 0x01;

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        &bytes,
    ) else {
        panic!("a uint16 word wider than sixteen bits is refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
            index: 4,
            kind: CanonicalFieldKind::Unsigned16
        })
    ));
}

/// Contract: an integer word with bits above its field's width is refused,
///   naming the field.
/// Arrange: the reference vector with byte 288, the total extent's word's
///   high byte, set to 0x80.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   ValueOutOfRange { index: 9, kind: CanonicalFieldKind::Unsigned64 })`.
#[test]
fn decode_rejects_a_uint64_word_wider_than_sixty_four_bits() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let Ok(mut bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };
    bytes[288] = 0x80;

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        &bytes,
    ) else {
        panic!("a uint64 word wider than sixty-four bits is refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange {
            index: 9,
            kind: CanonicalFieldKind::Unsigned64
        })
    ));
}

/// Contract: an encoding with a field beyond the description's sequence is
///   refused.
/// Arrange: the field-too-many vector — the reference fields' parameter
///   encoding followed by a uint64 of 1.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   NonCanonical)`.
#[test]
fn decode_rejects_the_encoding_of_one_field_too_many() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let Ok(bytes) = decode(FIELD_TOO_MANY_HEX) else {
        panic!("the field-too-many vector is valid hex")
    };

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        &bytes,
    ) else {
        panic!("the encoding of one field too many is refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)
    ));
}

/// Contract: a canonically encoded value the description refuses returns
///   that refusal.
/// Arrange: the reference vector with bytes 64 through 95, the deployment
///   identity, set to zero.
/// Act:     `encoding.decode(DecodeParams { description: &description }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Description(
///   DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
///   DeploymentIdentityTryNewErrorReturn::AllZero))`.
#[test]
fn decode_returns_the_description_refusal_unchanged() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let Ok(mut bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };
    bytes[64..96].fill(0);

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &description,
        },
        &bytes,
    ) else {
        panic!("a refused value returns the description's refusal")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
            DeploymentIdentityTryNewErrorReturn::AllZero
        ))
    ));
}

/// Contract: the concrete's declaration names the encoding identifier, its
///   adapter version, and the interface version it implements.
/// Arrange: none.
/// Act:     read `AbiEncoding::DECLARATION`.
/// Assert:  `identifier` matches `EncodingIdentifier::EthereumAbiV1`,
///   `adapter_version` equals 1, and `interface_version` equals
///   `ENCODING_INTERFACE_VERSION`.
#[test]
fn abi_encoding_declares_its_identifier_and_versions() {
    // Arrange

    // Act
    let declaration = AbiEncoding::DECLARATION;

    // Assert
    assert!(matches!(
        declaration.identifier,
        EncodingIdentifier::EthereumAbiV1
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, ENCODING_INTERFACE_VERSION);
}

/// Contract: a byte string encodes as `abi.encode` encodes `bytes`, its
///   offset in the head and its length and zero-padded bytes in the tail.
/// Arrange: the byte-string value — bytes 01 through 05, word 0x0d×32.
/// Act:     `encoding.encode(EncodeParams { description: &BytesProbeDescription }, &value)`.
/// Assert:  `success.bytes` equals the byte-string vector.
#[test]
fn encode_writes_a_byte_string_in_the_dynamic_tail() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let value = BytesProbe {
        bytes: vec![0x01, 0x02, 0x03, 0x04, 0x05],
        word: [0x0d; 32],
    };
    let Ok(bytes) = decode(BYTES_PROBE_VECTOR_HEX) else {
        panic!("the byte-string vector is valid hex")
    };

    // Act
    let Ok(success) = encoding.encode(
        EncodeParams {
            description: &BytesProbeDescription,
        },
        &value,
    );

    // Assert
    assert_eq!(success.bytes, bytes);
}

/// Contract: the canonical encoding of a byte string decodes to the value
///   it encodes.
/// Arrange: the byte-string vector.
/// Act:     `encoding.decode(DecodeParams { description: &BytesProbeDescription }, &bytes)`.
/// Assert:  `success.described` equals the byte-string value.
#[test]
fn decode_reads_the_byte_string_vector_as_its_value() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let value = BytesProbe {
        bytes: vec![0x01, 0x02, 0x03, 0x04, 0x05],
        word: [0x0d; 32],
    };
    let Ok(bytes) = decode(BYTES_PROBE_VECTOR_HEX) else {
        panic!("the byte-string vector is valid hex")
    };

    // Act
    let Ok(success) = encoding.decode(
        DecodeParams {
            description: &BytesProbeDescription,
        },
        &bytes,
    ) else {
        panic!("the canonical byte-string encoding decodes to the value it encodes")
    };

    // Assert
    assert_eq!(success.described, value);
}

/// Contract: a byte string's padding is zero in its one byte form.
/// Arrange: the byte-string vector with byte 127, the byte string's last
///   padding byte, set to 0x01.
/// Act:     `encoding.decode(DecodeParams { description: &BytesProbeDescription }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   NonCanonical)`.
#[test]
fn decode_rejects_nonzero_byte_string_padding() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(mut bytes) = decode(BYTES_PROBE_VECTOR_HEX) else {
        panic!("the byte-string vector is valid hex")
    };
    bytes[127] = 0x01;

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &BytesProbeDescription,
        },
        &bytes,
    ) else {
        panic!("nonzero byte-string padding is refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)
    ));
}

/// Contract: a twenty-byte fixed string encodes as `abi.encode` encodes
///   `bytes20`, left-aligned in its word, and a 256-bit unsigned integer as
///   `uint256`, its big-endian word.
/// Arrange: the widths value — party 0x11×20, integer 0xff×32.
/// Act:     `encoding.encode(EncodeParams { description: &WidthsProbeDescription }, &value)`.
/// Assert:  `success.bytes` equals the widths vector.
#[test]
fn encode_writes_the_width_kinds_as_bytes20_and_uint256() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let value = WidthsProbe {
        party: [0x11; 20],
        integer: [0xff; 32],
    };
    let Ok(bytes) = decode(WIDTHS_PROBE_VECTOR_HEX) else {
        panic!("the widths vector is valid hex")
    };

    // Act
    let Ok(success) = encoding.encode(
        EncodeParams {
            description: &WidthsProbeDescription,
        },
        &value,
    );

    // Assert
    assert_eq!(success.bytes, bytes);
}

/// Contract: the canonical encoding of the width kinds decodes to the value
///   it encodes, the full 256-bit word admitted.
/// Arrange: the widths vector.
/// Act:     `encoding.decode(DecodeParams { description: &WidthsProbeDescription }, &bytes)`.
/// Assert:  `success.described` equals the widths value.
#[test]
fn decode_reads_the_widths_vector_as_its_value() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let value = WidthsProbe {
        party: [0x11; 20],
        integer: [0xff; 32],
    };
    let Ok(bytes) = decode(WIDTHS_PROBE_VECTOR_HEX) else {
        panic!("the widths vector is valid hex")
    };

    // Act
    let Ok(success) = encoding.decode(
        DecodeParams {
            description: &WidthsProbeDescription,
        },
        &bytes,
    ) else {
        panic!("the canonical widths encoding decodes to the value it encodes")
    };

    // Assert
    assert_eq!(success.described, value);
}

/// Contract: a twenty-byte fixed string's padding is zero in its one byte
///   form.
/// Arrange: the widths vector with byte 20, the first padding byte after
///   the twenty, set to 0x01.
/// Act:     `encoding.decode(DecodeParams { description: &WidthsProbeDescription }, &bytes)`.
/// Assert:  `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::
///   NonCanonical)`.
#[test]
fn decode_rejects_nonzero_fixed_bytes20_padding() {
    // Arrange
    let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);
    let Ok(mut bytes) = decode(WIDTHS_PROBE_VECTOR_HEX) else {
        panic!("the widths vector is valid hex")
    };
    bytes[20] = 0x01;

    // Act
    let Err(error) = encoding.decode(
        DecodeParams {
            description: &WidthsProbeDescription,
        },
        &bytes,
    ) else {
        panic!("nonzero bytes20 padding is refused")
    };

    // Assert
    assert!(matches!(
        error,
        DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)
    ));
}
