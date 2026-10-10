#![allow(clippy::panic)]

use super::provides::{
    DerivationContext, DerivationContextConstructorParamsOverrides,
    DerivationContextTryNewErrorReturn, build_derivation_context,
    build_derivation_context_constructor_params,
};
use crate::asset_identity::provides::{
    AssetIdentityConstructorParamsOverrides, build_asset_identity,
};
use crate::deployment_identity::provides::{
    DEPLOYMENT_IDENTITY_LENGTH, DeploymentIdentityConstructorParamsOverrides,
    build_deployment_identity,
};
use crate::group_index::provides::{GroupIndexConstructorParamsOverrides, build_group_index};
use crate::parameter_set_identifier::provides::{
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier,
};
use crate::piece_geometry::provides::{
    PieceGeometryConstructorParamsOverrides, build_piece_geometry,
};
use crate::suite_identifier::provides::{
    SUITE_IDENTIFIER_LENGTH, SuiteIdentifierConstructorParamsOverrides, build_suite_identifier,
};

/// Contract: `params.group_index.value() >= params.geometry.group_count()` →
///   `Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index,
///   group_count })` holding the two values compared.
/// Arrange: `build_derivation_context_constructor_params` overriding the group
///   index with `build_group_index` at value `4` and the geometry with
///   `build_piece_geometry` at piece size `16384`, piece-group size `16384`, and
///   extent `65536`, a geometry of four groups, so a comparison of `>` instead
///   of `>=` fails; the other four components keep their builder defaults.
/// Act:     `DerivationContext::try_new` on the built params.
/// Assert:  the result equals `Err(DerivationContextTryNewErrorReturn::
///   GroupIndexOutOfRange { group_index: 4, group_count: 4 })` by `assert_eq!`,
///   the group count being the four groups the arrangement's extent holds at
///   its piece-group size.
#[test]
fn try_new_rejects_a_group_index_equal_to_the_group_count() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(4),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(16384),
                    piece_group_size: Some(16384),
                    total_extent: Some(65536),
                },
            )),
            ..Default::default()
        });

    // Act
    let result = DerivationContext::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
            group_index: 4,
            group_count: 4
        })
    );
}

/// Contract: `params.group_index.value() >= params.geometry.group_count()` →
///   `Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index,
///   group_count })` holding the two values compared.
/// Arrange: `build_derivation_context_constructor_params` overriding the group
///   index with `build_group_index` at value `u64::MAX` and the geometry with
///   `build_piece_geometry` at piece size `16384`, piece-group size `16384`, and
///   extent `65536`, a geometry of four groups, so the two values compared
///   differ and an error that swaps them, or a comparison that narrows the
///   index, fails; the other four components keep their builder defaults.
/// Act:     `DerivationContext::try_new` on the built params.
/// Assert:  the result equals `Err(DerivationContextTryNewErrorReturn::
///   GroupIndexOutOfRange { group_index: u64::MAX, group_count: 4 })` by
///   `assert_eq!`, the group count being the four groups the arrangement's
///   extent holds at its piece-group size.
#[test]
fn try_new_rejects_the_largest_group_index() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(u64::MAX),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(16384),
                    piece_group_size: Some(16384),
                    total_extent: Some(65536),
                },
            )),
            ..Default::default()
        });

    // Act
    let result = DerivationContext::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
            group_index: u64::MAX,
            group_count: 4
        })
    );
}

/// Contract: the group index is below the group count →
///   `Ok(DerivationContext { asset, deployment, suite, parameter_set,
///   group_index, geometry })`.
/// Arrange: `build_derivation_context_constructor_params` overriding the group
///   index with `build_group_index` at value `3` and the geometry with
///   `build_piece_geometry` at piece size `16384`, piece-group size `16384`, and
///   extent `65536`, a geometry of four groups, so index `3` is its last group
///   and a comparison that refuses the last group fails; the other four
///   components keep their builder defaults.
/// Act:     `DerivationContext::try_new` on the built params.
/// Assert:  the result is `Ok`.
#[test]
fn try_new_admits_the_last_group_of_the_geometry() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(3),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(16384),
                    piece_group_size: Some(16384),
                    total_extent: Some(65536),
                },
            )),
            ..Default::default()
        });

    // Act
    let result = DerivationContext::try_new(params);

    // Assert
    assert!(result.is_ok());
}

/// Contract: the group index is below the group count →
///   `Ok(DerivationContext { asset, deployment, suite, parameter_set,
///   group_index, geometry })`.
/// Arrange: `build_derivation_context_constructor_params` overriding the group
///   index with `build_group_index` at value `0` and the geometry with
///   `build_piece_geometry` at piece size `16384`, piece-group size `16384`, and
///   extent `16384`, a geometry of one group, so a comparison that refuses the
///   only group fails; the other four components keep their builder defaults.
/// Act:     `DerivationContext::try_new` on the built params.
/// Assert:  the result is `Ok`.
#[test]
fn try_new_admits_index_zero_of_a_one_group_geometry() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(0),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(16384),
                    piece_group_size: Some(16384),
                    total_extent: Some(16384),
                },
            )),
            ..Default::default()
        });

    // Act
    let result = DerivationContext::try_new(params);

    // Assert
    assert!(result.is_ok());
}

/// Contract: `DerivationContext::asset(&self) -> &AssetIdentity` → a shared
///   reference to the held asset.
/// Arrange: `build_derivation_context` overriding the asset with
///   `build_asset_identity` at name `left-pad` and version `1.3.0`, values that
///   differ from the builder's defaults, so an accessor that returns any other
///   asset fails.
/// Act:     `context.asset()`.
/// Assert:  the returned asset's `name()` equals the literal `"left-pad"` and
///   its `version()` equals the literal `"1.3.0"` by `assert_eq!`.
#[test]
fn asset_returns_the_held_asset() {
    // Arrange
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        asset: Some(build_asset_identity(
            AssetIdentityConstructorParamsOverrides {
                name: Some("left-pad".to_string()),
                version: Some("1.3.0".to_string()),
            },
        )),
        ..Default::default()
    });

    // Act
    let asset = context.asset();

    // Assert
    assert_eq!(asset.name(), "left-pad");
    assert_eq!(asset.version(), "1.3.0");
}

/// Contract: `DerivationContext::deployment(&self) -> &DeploymentIdentity` → a
///   shared reference to the held deployment.
/// Arrange: `build_derivation_context` overriding the deployment with
///   `build_deployment_identity` at bytes of `DEPLOYMENT_IDENTITY_LENGTH` bytes
///   each `0x44`, a value that differs from the builder's default, so an
///   accessor that returns any other deployment fails.
/// Act:     `context.deployment()`.
/// Assert:  the returned deployment's `as_bytes()` equals the array literal of
///   `DEPLOYMENT_IDENTITY_LENGTH` bytes each `0x44` by `assert_eq!`.
#[test]
fn deployment_returns_the_held_deployment() {
    // Arrange
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        deployment: Some(build_deployment_identity(
            DeploymentIdentityConstructorParamsOverrides {
                bytes: Some([0x44; DEPLOYMENT_IDENTITY_LENGTH]),
            },
        )),
        ..Default::default()
    });

    // Act
    let deployment = context.deployment();

    // Assert
    assert_eq!(deployment.as_bytes(), &[0x44; DEPLOYMENT_IDENTITY_LENGTH]);
}

/// Contract: `DerivationContext::suite(&self) -> &SuiteIdentifier` → a shared
///   reference to the held suite.
/// Arrange: `build_derivation_context` overriding the suite with
///   `build_suite_identifier` at identifier of `SUITE_IDENTIFIER_LENGTH` bytes
///   each `0x55` and version `0x0102`, values that differ from the builder's
///   defaults, so an accessor that returns any other suite fails.
/// Act:     `context.suite()`.
/// Assert:  the returned suite's `identifier()` equals the array literal of
///   `SUITE_IDENTIFIER_LENGTH` bytes each `0x55` and its `version()` equals the
///   literal `0x0102` by `assert_eq!`.
#[test]
fn suite_returns_the_held_suite() {
    // Arrange
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        suite: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x55; SUITE_IDENTIFIER_LENGTH]),
                version: Some(0x0102),
            },
        )),
        ..Default::default()
    });

    // Act
    let suite = context.suite();

    // Assert
    assert_eq!(suite.identifier(), &[0x55; SUITE_IDENTIFIER_LENGTH]);
    assert_eq!(suite.version(), 0x0102);
}

/// Contract: `DerivationContext::parameter_set(&self) -> &ParameterSetIdentifier`
///   → a shared reference to the held parameter set.
/// Arrange: `build_derivation_context` overriding the parameter set with
///   `build_parameter_set_identifier` at bytes of
///   `PARAMETER_SET_IDENTIFIER_LENGTH` bytes each `0x66`, a value that differs
///   from the builder's default, so an accessor that returns any other
///   parameter set fails.
/// Act:     `context.parameter_set()`.
/// Assert:  the returned parameter set's `as_bytes()` equals the array literal
///   of `PARAMETER_SET_IDENTIFIER_LENGTH` bytes each `0x66` by `assert_eq!`.
#[test]
fn parameter_set_returns_the_held_parameter_set() {
    // Arrange
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        parameter_set: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x66; PARAMETER_SET_IDENTIFIER_LENGTH]),
            },
        )),
        ..Default::default()
    });

    // Act
    let parameter_set = context.parameter_set();

    // Assert
    assert_eq!(
        parameter_set.as_bytes(),
        &[0x66; PARAMETER_SET_IDENTIFIER_LENGTH]
    );
}

/// Contract: `DerivationContext::group_index(&self) -> &GroupIndex` → a shared
///   reference to the held group index.
/// Arrange: `build_derivation_context` overriding the group index with
///   `build_group_index` at value `2`, a value that differs from the builder's
///   default and falls within the default geometry's groups, so an accessor
///   that returns any other group index fails.
/// Act:     `context.group_index()`.
/// Assert:  the returned group index's `value()` equals the literal `2` by
///   `assert_eq!`.
#[test]
fn group_index_returns_the_held_group_index() {
    // Arrange
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
            value: Some(2),
        })),
        ..Default::default()
    });

    // Act
    let group_index = context.group_index();

    // Assert
    assert_eq!(group_index.value(), 2);
}

/// Contract: `DerivationContext::geometry(&self) -> &PieceGeometry` → a shared
///   reference to the held geometry.
/// Arrange: `build_derivation_context` overriding the geometry with
///   `build_piece_geometry` at piece size `32768` and piece-group size `65536`,
///   values that differ from the builder's defaults, with the extent keeping
///   its builder default so the default group index falls within the
///   geometry's groups, so an accessor that returns any other geometry fails.
/// Act:     `context.geometry()`.
/// Assert:  the returned geometry's `piece_size()` equals the literal `32768`
///   and its `piece_group_size()` equals the literal `65536` by `assert_eq!`.
#[test]
fn geometry_returns_the_held_geometry() {
    // Arrange
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        geometry: Some(build_piece_geometry(
            PieceGeometryConstructorParamsOverrides {
                piece_size: Some(32768),
                piece_group_size: Some(65536),
                ..Default::default()
            },
        )),
        ..Default::default()
    });

    // Act
    let geometry = context.geometry();

    // Assert
    assert_eq!(geometry.piece_size(), 32768);
    assert_eq!(geometry.piece_group_size(), 65536);
}
