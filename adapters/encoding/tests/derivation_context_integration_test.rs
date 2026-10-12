#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    AssetIdentityConstructorParamsOverrides, DEPLOYMENT_IDENTITY_LENGTH,
    DeploymentIdentityConstructorParamsOverrides, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, PARAMETER_SET_IDENTIFIER_LENGTH,
    ParameterSetIdentifierConstructorParamsOverrides, PieceGeometryConstructorParamsOverrides,
    SUITE_IDENTIFIER_LENGTH, SuiteIdentifierConstructorParamsOverrides, build_asset_identity,
    build_deployment_identity, build_derivation_context, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_suite_identifier,
};
use encoding::{
    IEncodingContract, build_derivation_context_description, build_from_fields_params,
    build_to_fields_params,
};

/// Contract: entry `a_context_survives_the_round_trip_through_its_canonical_fields`;
///   an admitted `DerivationContext` handed to `to_fields`, and the fields it
///   returns handed to `fields_to_value`, yields `Ok(FromFieldsSuccessReturn
///   { described })` with `described` equal to the original context.
/// Boundary: the public calls `IEncodingContract::to_fields` and
///   `IEncodingContract::fields_to_value` on `DerivationContextDescription`, a
///   public entry of the crate; the route `to_fields` then `fields_to_value`,
///   each running `domain`'s accessors and constructors as themselves.
/// Mocked:   nothing; `domain` is a pure value library with no outer-edge
///   collaborator on this route.
/// Arrange:  `build_derivation_context_description`, `build_to_fields_params`,
///   and `build_from_fields_params` from the crate's public surface under the
///   `mocks` feature; `build_derivation_context` overriding the asset with
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
/// Act:      `description.to_fields(to_fields_params, &context)`, then
///   `description.fields_to_value(from_fields_params, fields)` over the
///   fields the first call returned.
/// Assert:   the `described` of the second call's `Ok` arm equals the arranged
///   context by `assert_eq!`.
#[test]
fn a_context_survives_the_round_trip_through_its_canonical_fields() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let to_fields_params = build_to_fields_params(Default::default());
    let from_fields_params = build_from_fields_params(Default::default());
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        asset: Some(build_asset_identity(
            AssetIdentityConstructorParamsOverrides {
                name: Some("left-pad".to_string()),
                version: Some("1.3.0".to_string()),
            },
        )),
        deployment: Some(build_deployment_identity(
            DeploymentIdentityConstructorParamsOverrides {
                bytes: Some([0x44; DEPLOYMENT_IDENTITY_LENGTH]),
            },
        )),
        suite: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x55; SUITE_IDENTIFIER_LENGTH]),
                version: Some(u16::MAX),
            },
        )),
        parameter_set: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x66; PARAMETER_SET_IDENTIFIER_LENGTH]),
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
    let Ok(output) = description.to_fields(to_fields_params, &context);
    let result = description.fields_to_value(from_fields_params, output.fields);

    // Assert
    assert_eq!(
        result
            .expect("the canonical fields describe an admitted context")
            .described,
        context
    );
}
