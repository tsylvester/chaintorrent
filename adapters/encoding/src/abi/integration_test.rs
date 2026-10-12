#![allow(clippy::expect_used)]

use super::provides::{build_abi_encoding, build_derivation_context_reference_vector};
use crate::derivation_context::provides::{
    DerivationContextFromFieldsErrorReturn, build_derivation_context_description,
};
use crate::factory::provides::{
    DecodeErrorReturn, IDecoderAdapter, IEncoderAdapter, build_decode_params, build_encode_params,
};
use domain::{
    AssetIdentityConstructorParamsOverrides, DeploymentIdentityConstructorParamsOverrides,
    DeploymentIdentityTryNewErrorReturn, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    PieceGeometryConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity, build_deployment_identity, build_derivation_context, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_suite_identifier,
};

/// Contract: the own entry, reference context encodes to the reference
///   vector: the reference derivation context handed to `encode` with
///   `DerivationContextDescription` → `Ok(EncodeSuccessReturn { bytes })` with
///   `bytes` the known-answer vector authored from the ABI specification for
///   the reference fields.
/// Arrange: `build_abi_encoding`, `build_derivation_context_description`, and
///   `build_encode_params` over the description; the reference context, whose
///   text fields are dynamic items between static items and whose same-kind
///   fields differ, so a swapped field, a wrong word width, or a misplaced
///   tail changes the bytes.
/// Act:     `adapter.encode(params, &context)`.
/// Assert:  `result.ok().map(|success| success.bytes)` equals `Some` of the
///   reference vector's bytes by `assert_eq!`, the expectation authored from
///   the ABI specification and not read back from the arrangement.
/// Boundary: the crate-internal call `AbiEncoding::encode`; the route
///   `encode`, `DerivationContextDescription::to_fields`, and `domain`'s
///   accessors, with the count and kind check against `FIELDS` between them.
/// Mocked:  nothing; `alloy` runs as the concrete's own vendor library and
///   `domain` is a pure value library.
#[test]
fn reference_context_encodes_to_the_reference_vector() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = build_derivation_context_description(Default::default());
    let params = build_encode_params(&description);
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
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
    });

    // Act
    let result = adapter.encode(params, &context);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.bytes),
        Some(build_derivation_context_reference_vector())
    );
}

/// Contract: the own entry, reference vector decodes to the reference
///   context: the reference vector handed to `decode` with
///   `DerivationContextDescription` → `Ok(DecodeSuccessReturn { described })`
///   with `described` equal to the reference context.
/// Arrange: `build_abi_encoding`, `build_derivation_context_description`, and
///   `build_decode_params` over the description; the reference vector's bytes
///   as the payload and the reference context as the expectation, whose
///   same-kind fields differ, so a field read from the wrong word or tail
///   yields a refusal or an unequal context.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.ok().map(|success| success.described)` equals `Some` of
///   the reference context by `assert_eq!`, the reference context built by
///   `domain`'s builders and not by the subject.
/// Boundary: the crate-internal call `AbiEncoding::decode`; the route
///   `decode`, the ABI decode, `DerivationContextDescription::
///   fields_to_value`, `domain`'s constructors, and the canonical re-encoding
///   through `encode`, `DerivationContextDescription::to_fields`, and
///   `domain`'s accessors.
/// Mocked:  nothing; `alloy` runs as the concrete's own vendor library and
///   `domain` is a pure value library.
#[test]
fn reference_vector_decodes_to_the_reference_context() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = build_derivation_context_description(Default::default());
    let params = build_decode_params(&description);
    let payload = build_derivation_context_reference_vector();

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.described),
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
                })),
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

/// Contract: the own entry, a context survives the ABI round trip, which
///   absorbs the `encoding/derivation_context` entry round trip through the
///   canonical fields: an admitted `DerivationContext` handed to `encode`,
///   and the bytes it returns handed to `decode`, yields
///   `Ok(DecodeSuccessReturn { described })` with `described` equal to the
///   original context.
/// Arrange: `build_abi_encoding`, `build_derivation_context_description`,
///   `build_encode_params`, and `build_decode_params` over the description;
///   `build_derivation_context` overriding the asset with
///   `build_asset_identity` at name `left-pad` and version `1.3.0`, the
///   deployment with `build_deployment_identity` at bytes each `0x44`, the
///   suite with `build_suite_identifier` at identifier bytes each `0x55` and
///   version `u16::MAX`, the parameter set with
///   `build_parameter_set_identifier` at bytes each `0x66`, the group index
///   with `build_group_index` at value `4294967296`, and the geometry with
///   `build_piece_geometry` at piece size `16384`, piece-group size
///   `2147483648`, and extent `u64::MAX`; the same-kind fields hold distinct
///   values, the suite version is at its maximum, the piece-group size
///   exceeds the `i32` range, and the group index and extent exceed the
///   32-bit range, so a swap of same-kind fields, a dropped field, or a
///   narrowed width yields a refusal or an unequal context.
/// Act:     `adapter.encode(encode_params, &context)`, then
///   `adapter.decode(decode_params, &bytes)` over the bytes the first call
///   returned.
/// Assert:  the `described` of the second call's `Ok` arm equals the arranged
///   context by `assert_eq!`.
/// Boundary: the crate-internal calls `AbiEncoding::encode` and
///   `AbiEncoding::decode`; the route `encode`,
///   `DerivationContextDescription::to_fields`, then `decode`,
///   `DerivationContextDescription::fields_to_value`, and the canonical
///   re-encoding, each running `domain`'s accessors and constructors as
///   themselves.
/// Mocked:  nothing; `alloy` runs as the concrete's own vendor library and
///   `domain` is a pure value library.
#[test]
fn a_context_survives_the_abi_round_trip() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = build_derivation_context_description(Default::default());
    let encode_params = build_encode_params(&description);
    let decode_params = build_decode_params(&description);
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
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
    });

    // Act
    let encoded = adapter
        .encode(encode_params, &context)
        .expect("a canonical field list encodes");
    let result = adapter.decode(decode_params, &encoded.bytes);

    // Assert
    assert_eq!(result.ok().map(|success| success.described), Some(context));
}

/// Contract: the own entry, a value the description refuses is refused with
///   the description's refusal: a canonical ABI encoding whose
///   deployment-identity word is all zero handed to `decode` with
///   `DerivationContextDescription` → `Err(DecodeErrorReturn::Description(
///   DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
///   DeploymentIdentityTryNewErrorReturn::AllZero)))`.
/// Arrange: `build_abi_encoding`, `build_derivation_context_description`, and
///   `build_decode_params` over the description; the reference vector's bytes
///   with the deployment-identity word replaced by zero bytes, an encoding
///   that is canonical and well-kinded, so the ABI decode and the canonical
///   comparison admit it and only the domain's constructor refuses.
/// Act:     `adapter.decode(params, &payload)`.
/// Assert:  `result.err()` equals `Some(DecodeErrorReturn::Description(
///   DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
///   DeploymentIdentityTryNewErrorReturn::AllZero)))` by `assert_eq!`, the
///   whole refusal the deployment identity's constructor declares for
///   all-zero bytes.
/// Boundary: the crate-internal call `AbiEncoding::decode`; the route
///   `decode`, the ABI decode, the item pass,
///   `DerivationContextDescription::fields_to_value`, and
///   `DeploymentIdentity::try_new`.
/// Mocked:  nothing; `alloy` runs as the concrete's own vendor library and
///   `domain` is a pure value library.
#[test]
fn a_value_the_description_refuses_is_refused_with_the_description_refusal() {
    // Arrange
    let adapter = build_abi_encoding(Default::default());
    let description = build_derivation_context_description(Default::default());
    let params = build_decode_params(&description);
    let mut payload = build_derivation_context_reference_vector();
    payload[64..96].fill(0);

    // Act
    let result = adapter.decode(params, &payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DecodeErrorReturn::Description(
            DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
                DeploymentIdentityTryNewErrorReturn::AllZero
            )
        ))
    );
}
