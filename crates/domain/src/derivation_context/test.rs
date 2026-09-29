#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{DerivationContext, DerivationContextTryNewErrorReturn};
use super::mock::{
    DerivationContextConstructorParamsOverrides, build_derivation_context_constructor_params,
};
use crate::asset_identity::provides::build_asset_identity;
use crate::deployment_identity::provides::build_deployment_identity;
use crate::group_index::provides::{GroupIndexConstructorParamsOverrides, build_group_index};
use crate::parameter_set_identifier::provides::build_parameter_set_identifier;
use crate::piece_geometry::provides::{
    PieceGeometryConstructorParamsOverrides, build_piece_geometry,
};
use crate::suite_identifier::provides::build_suite_identifier;

/// Contract: the highest index below the group count is admitted, and every
///   component reads back unchanged.
/// Arrange: params built with a two-group geometry (`piece_size: Some(16384)`,
///   `piece_group_size: Some(16384)`, `total_extent: Some(32768)`) and a
///   `group_index` of `value: Some(1)`; every other component defaulted.
/// Act:     `DerivationContext::try_new(params)`.
/// Assert:  `context.group_index().value()` equals `1`,
///   `context.geometry().group_count()` equals `2`, and `context.asset()`,
///   `context.deployment()`, `context.suite()`, and
///   `context.parameter_set()` equal the components the default builders
///   produce.
#[test]
fn try_new_admits_the_last_group_of_the_geometry() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(1),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(16384),
                    piece_group_size: Some(16384),
                    total_extent: Some(32768),
                },
            )),
            ..Default::default()
        });

    // Act
    let Ok(context) = DerivationContext::try_new(params) else {
        panic!("the last group of the geometry is admitted")
    };

    // Assert
    assert_eq!(context.group_index().value(), 1);
    assert_eq!(context.geometry().group_count(), 2);
    assert_eq!(context.asset(), &build_asset_identity(Default::default()));
    assert_eq!(
        context.deployment(),
        &build_deployment_identity(Default::default())
    );
    assert_eq!(context.suite(), &build_suite_identifier(Default::default()));
    assert_eq!(
        context.parameter_set(),
        &build_parameter_set_identifier(Default::default())
    );
}

/// Contract: the smallest group count admits its only index.
/// Arrange: params built with a one-group geometry (`total_extent: Some(1)`)
///   and a `group_index` of `value: Some(0)`.
/// Act:     `DerivationContext::try_new(params)`.
/// Assert:  `context.group_index().value()` equals `0`.
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
                    total_extent: Some(1),
                    ..Default::default()
                },
            )),
            ..Default::default()
        });

    // Act
    let Ok(context) = DerivationContext::try_new(params) else {
        panic!("index zero of a one-group geometry is admitted")
    };

    // Assert
    assert_eq!(context.group_index().value(), 0);
}

/// Contract: the index one past the last group is refused, naming both
///   values.
/// Arrange: params built with the two-group geometry and a `group_index` of
///   `value: Some(2)`.
/// Act:     `DerivationContext::try_new(params)`.
/// Assert:  `error` equals
///   `DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index: 2, group_count: 2 }`.
#[test]
fn try_new_rejects_a_group_index_equal_to_the_group_count() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(2),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(16384),
                    piece_group_size: Some(16384),
                    total_extent: Some(32768),
                },
            )),
            ..Default::default()
        });

    // Act
    let Err(error) = DerivationContext::try_new(params) else {
        panic!("a group index equal to the group count is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
            group_index: 2,
            group_count: 2
        }
    );
}

/// Contract: an index far beyond the group count is refused without
///   overflow.
/// Arrange: params built with the two-group geometry and a `group_index` of
///   `value: Some(u64::MAX)`.
/// Act:     `DerivationContext::try_new(params)`.
/// Assert:  `error` equals
///   `DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index: 18446744073709551615, group_count: 2 }`.
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
                    total_extent: Some(32768),
                },
            )),
            ..Default::default()
        });

    // Act
    let Err(error) = DerivationContext::try_new(params) else {
        panic!("the largest group index is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
            group_index: 18446744073709551615,
            group_count: 2
        }
    );
}
