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
    PieceGeometryConstructorParamsOverrides, SecretConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity, build_deployment_identity,
    build_derivation_context, build_group_index, build_parameter_set_identifier,
    build_piece_geometry, build_secret, build_suite_identifier,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps,
    CreateEncodingParamsOverrides, CreateEncodingPayload, DerivationContextDescription,
    DerivationContextDescriptionConstructorParams, EncodeParams, EncodingConcrete,
    EncodingIdentifier, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_params, create_encoding,
};
use hex::decode;
use kdf::{
    CreateKeyDerivationDeps, CreateKeyDerivationParamsOverrides, CreateKeyDerivationPayload,
    DerivationPurpose, DeriveKeyParamsOverrides, DeriveKeyPayload, KdfConcrete, KdfIdentifier,
    build_create_key_derivation_params, build_derive_key_params, create_key_derivation,
};

const WRAPPING_KEY_VECTOR_HEX: &str = "549874404f0b25b65da55dbce0b410d0aa34085695af4b91303e2c1c0410f760a75cb805271fbaf45eb7a1fa33596b3cb082ec41f923d4fb7055ba82699ec4f3";

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
            panic!("the reference context encodes through the adapter")
        };
        success.bytes
    }
}

/// Contract: the reference context encoded through the encoding factory, and
///   the wrapping key derived over it through the key-derivation factory's
///   concrete, match the independent vector, with nothing mocked.
/// Arrange: `build_create_encoding_params` with `concrete:
///   Some(EncodingConcrete::Abi)` and `identifier:
///   Some(EncodingIdentifier::EthereumAbiV1)` and `CreateEncodingDeps {
///   consumer: EncodeContext { context } }` holding the reference context;
///   `build_create_key_derivation_params` with `concrete:
///   Some(KdfConcrete::Blake3Keyed)` and `identifier:
///   Some(KdfIdentifier::Blake3KeyedV1)`; the reference secret.
/// Act:     `adapter.derive_key` for `WrappingKey`, length 64, over the
///   encoded context.
/// Assert:  `success.key.expose()` equals the wrapping-key vector.
/// Boundary: the `kdf` and `encoding` crates' public surfaces to `alloy` and
///   `blake3`, the outer edges — the real `create_encoding`, `AbiEncoding`,
///   `DerivationContextDescription`, `create_key_derivation`, and
///   `Blake3KeyedKdf`.
/// Mocked:   nothing; `alloy` performs the encoding and `blake3` the
///   derivation at the edges.
#[test]
fn the_blake3_keyed_concrete_from_the_factory_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes()
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
    let encoding_deps = CreateEncodingDeps {
        consumer: EncodeContext { context },
    };
    let encoding_params = build_create_encoding_params(CreateEncodingParamsOverrides {
        concrete: Some(EncodingConcrete::Abi),
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
    });
    let kdf_params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        concrete: Some(KdfConcrete::Blake3Keyed),
        identifier: Some(KdfIdentifier::Blake3KeyedV1),
    });
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(expected) = decode(WRAPPING_KEY_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    let Ok(encoding_success) =
        create_encoding(&encoding_deps, encoding_params, CreateEncodingPayload)
    else {
        panic!("the ABI concrete is admitted for the Ethereum ABI identifier")
    };
    let context_bytes = encoding_success.output;
    let Ok(kdf_success) = create_key_derivation(
        &CreateKeyDerivationDeps,
        kdf_params,
        CreateKeyDerivationPayload,
    ) else {
        panic!("the BLAKE3 concrete is admitted for its identifier")
    };

    // Act
    let Ok(success) = kdf_success.adapter.derive_key(
        build_derive_key_params(DeriveKeyParamsOverrides {
            purpose: Some(DerivationPurpose::WrappingKey),
            length: Some(64),
        }),
        DeriveKeyPayload {
            key_material: &secret,
            context: &context_bytes,
        },
    ) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}
