#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use domain::{
    AssetIdentityConstructorParamsOverrides, DeploymentIdentityConstructorParamsOverrides,
    DerivationContext, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    PieceGeometryConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity, build_deployment_identity, build_derivation_context, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_suite_identifier,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps,
    CreateEncodingParamsOverrides, CreateEncodingPayload, DecodeParams,
    DerivationContextDescription, DerivationContextDescriptionConstructorParams, EncodeParams,
    EncodingConcrete, EncodingIdentifier, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_params, create_encoding,
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

struct RoundTripResult {
    bytes: Vec<u8>,
    described: DerivationContext,
}

struct RoundTripCheck {
    context: DerivationContext,
}

impl IEncodingConsumer for RoundTripCheck {
    type Output = RoundTripResult;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(description) =
            DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
        let Ok(encoded) = payload.adapter.encode(
            EncodeParams {
                description: &description,
            },
            &self.context,
        );
        let Ok(decoded) = payload.adapter.decode(
            DecodeParams {
                description: &description,
            },
            &encoded.bytes,
        ) else {
            panic!("the encoded bytes decode through the family's decoder trait")
        };
        RoundTripResult {
            bytes: encoded.bytes,
            described: decoded.described,
        }
    }
}

/// Contract: the ABI concrete the factory constructs, admitted for the
///   Ethereum ABI identifier, encodes the reference context to the
///   known-answer vector and decodes it back through `IEncoderAdapter`,
///   `IDecoderAdapter`, and the description, with nothing mocked.
/// Arrange: `build_create_encoding_params` with `concrete:
///   Some(EncodingConcrete::Abi)` and `identifier:
///   Some(EncodingIdentifier::EthereumAbiV1)`, and `CreateEncodingDeps {
///   consumer: RoundTripCheck { context } }` holding the reference context.
/// Act:     `create_encoding(&deps, params, CreateEncodingPayload)`.
/// Assert:  `success.output.bytes` equals the reference vector and
///   `success.output.described` equals the reference context.
/// Boundary: the crate's public surface to `alloy`, the outer edge — the real
///   `create_encoding`, the real `AbiEncoding`, and the real
///   `DerivationContextDescription`.
/// Mocked:   nothing; `alloy` performs the encoding at the edge.
#[test]
fn the_ethereum_abi_concrete_from_the_factory_round_trips_the_reference_context_through_the_family_traits()
 {
    // Arrange
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
    let deps = CreateEncodingDeps {
        consumer: RoundTripCheck {
            context: build_derivation_context(DerivationContextConstructorParamsOverrides {
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
            }),
        },
    };
    let params = build_create_encoding_params(CreateEncodingParamsOverrides {
        concrete: Some(EncodingConcrete::Abi),
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
    });
    let Ok(bytes) = decode(REFERENCE_VECTOR_HEX) else {
        panic!("the reference vector is valid hex")
    };

    // Act
    let Ok(success) = create_encoding(&deps, params, CreateEncodingPayload) else {
        panic!("the ABI concrete is admitted for the Ethereum ABI identifier")
    };

    // Assert
    assert_eq!(success.output.bytes, bytes);
    assert_eq!(success.output.described, context);
}
