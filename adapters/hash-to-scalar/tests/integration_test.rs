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
    CreateEncodingParamsOverrides, CreateEncodingPayload, DerivationContextDescription,
    DerivationContextDescriptionConstructorParams, EncodeParams, EncodingConcrete,
    EncodingIdentifier, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_params, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarParamsOverrides, CreateHashToScalarPayload,
    DomainTag, DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarIdentifier,
    HashToScalarParams, HashToScalarPayload, build_create_hash_to_scalar_params, build_domain_tag,
    create_hash_to_scalar,
};
use hex::decode;
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingConsumer, PairingConcrete, build_create_pairing_params, create_pairing,
};

const REFERENCE_TAG: &[u8] = b"ChainTorrent hash-to-scalar test";

const REFERENCE_BN254_SCALAR_HEX: &str =
    "10ed702d3a6c3c552391220194f0ed1c66d498f751ba2a51289840a3980f8de1";
const REFERENCE_BLS12_381_SCALAR_HEX: &str =
    "4151bea01b9ddc7edbe167b8167245798f08813fcb739ae26c7a3637880f8de2";

struct EncodeContext {
    context: DerivationContext,
}

impl IEncodingConsumer for EncodeContext {
    type Output = Vec<u8>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(description) =
            DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
        let Ok(success) = payload.adapter.encode(
            EncodeParams {
                description: &description,
            },
            &self.context,
        ) else {
            panic!("the reference context encodes")
        };
        success.bytes
    }
}

struct FactoryScalarProbe {
    tag: DomainTag,
    message: Vec<u8>,
}

impl IPairingConsumer for FactoryScalarProbe {
    type Output = Vec<u8>;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let Ok(created) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                concrete: Some(HashToScalarConcrete::Keccak256),
                identifier: Some(HashToScalarIdentifier::Keccak256V1),
            }),
            CreateHashToScalarPayload,
        ) else {
            panic!("the Keccak-256 concrete is admitted for its identifier")
        };
        let Ok(hashed) = created.adapter.hash_to_scalar(
            HashToScalarParams { tag: &self.tag },
            HashToScalarPayload {
                message: &self.message,
            },
        ) else {
            panic!("the reference inputs hash")
        };
        let Ok(encoded) = payload.adapter.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: hashed.scalar,
            },
        );
        encoded.bytes.expose().as_ref().to_vec()
    }
}

/// Contract: admitted + hashed — the reference context encoded through the
///   encoding factory's ABI concrete is hashed under the reference tag by the
///   concrete `create_hash_to_scalar` admits, and reduced modulo the BN254
///   group order by the resolved pairing's scalar type, matching the
///   independent implementation's scalar.
/// Boundary: `create_encoding` (real ABI adapter, real
///   `DerivationContextDescription`) → `create_pairing` (real BN254 arkworks
///   adapter) → `create_hash_to_scalar` (real Keccak-256 concrete) →
///   `IHashToScalarAdapter::hash_to_scalar` → `IPairingAdapter::encode_scalar`.
/// Mocked:   nothing — `alloy`, `sha3`, and the curve libraries are the outer
///   edges; the only test-double-shaped types are the consumer probes, which
///   are the mandated way the families hand their adapters over.
/// Arrange:  the reference derivation context (asset "@scope/example-package",
///   version "2.1.0-beta.3", deployment 0x0a×32, suite 0x0b×32 version 3,
///   parameter set 0x0c×32, group index 5, geometry 16384/32768/1048576) is
///   encoded by `create_encoding` with `concrete: Some(EncodingConcrete::Abi)`
///   and `identifier: Some(EncodingIdentifier::EthereumAbiV1)`; the reference
///   tag is `b"ChainTorrent hash-to-scalar test"`.
/// Act:     `create_pairing` with `concrete:
///   Some(PairingConcrete::Bn254Arkworks)` over the `FactoryScalarProbe`
///   consumer.
/// Assert:  `success.output` equals the reference message's independent BN254
///   scalar.
#[test]
fn the_keccak256_concrete_from_the_factory_maps_the_encoded_reference_context_to_the_independent_bn254_scalar()
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
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(encoded_context) = create_encoding(
        &CreateEncodingDeps {
            consumer: EncodeContext { context },
        },
        build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(EncodingConcrete::Abi),
            identifier: Some(EncodingIdentifier::EthereumAbiV1),
        }),
        CreateEncodingPayload,
    ) else {
        panic!("the encoding factory resolves the ABI concrete")
    };
    let Ok(expected) = decode(REFERENCE_BN254_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = FactoryScalarProbe {
        tag,
        message: encoded_context.output,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: admitted + hashed — the reference context encoded through the
///   encoding factory's ABI concrete is hashed under the reference tag by the
///   concrete `create_hash_to_scalar` admits, and reduced modulo the BLS12-381
///   group order by the resolved pairing's scalar type, matching the
///   independent implementation's scalar.
/// Boundary: `create_encoding` (real ABI adapter, real
///   `DerivationContextDescription`) → `create_pairing` (real BLS12-381
///   arkworks adapter) → `create_hash_to_scalar` (real Keccak-256 concrete) →
///   `IHashToScalarAdapter::hash_to_scalar` → `IPairingAdapter::encode_scalar`.
/// Mocked:   nothing — `alloy`, `sha3`, and the curve libraries are the outer
///   edges; the only test-double-shaped types are the consumer probes, which
///   are the mandated way the families hand their adapters over.
/// Arrange:  the reference derivation context (asset "@scope/example-package",
///   version "2.1.0-beta.3", deployment 0x0a×32, suite 0x0b×32 version 3,
///   parameter set 0x0c×32, group index 5, geometry 16384/32768/1048576) is
///   encoded by `create_encoding` with `concrete: Some(EncodingConcrete::Abi)`
///   and `identifier: Some(EncodingIdentifier::EthereumAbiV1)`; the reference
///   tag is `b"ChainTorrent hash-to-scalar test"`.
/// Act:     `create_pairing` with `concrete:
///   Some(PairingConcrete::Bls12381Arkworks)` over the `FactoryScalarProbe`
///   consumer.
/// Assert:  `success.output` equals the reference message's independent
///   BLS12-381 scalar.
#[test]
fn the_keccak256_concrete_from_the_factory_maps_the_encoded_reference_context_to_the_independent_bls12_381_scalar()
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
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(encoded_context) = create_encoding(
        &CreateEncodingDeps {
            consumer: EncodeContext { context },
        },
        build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(EncodingConcrete::Abi),
            identifier: Some(EncodingIdentifier::EthereumAbiV1),
        }),
        CreateEncodingPayload,
    ) else {
        panic!("the encoding factory resolves the ABI concrete")
    };
    let Ok(expected) = decode(REFERENCE_BLS12_381_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = FactoryScalarProbe {
        tag,
        message: encoded_context.output,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}
