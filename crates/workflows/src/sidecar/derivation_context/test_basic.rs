#![allow(clippy::expect_used)]

use super::provides::{
    DerivationContext, DerivationContextConstructorParamsOverrides, DerivationContextDescription,
    build_derivation_context, build_derivation_context_constructor_params,
    build_derivation_context_description, build_derivation_context_description_constructor_params,
};
use domain::{
    GroupIndexConstructorParamsOverrides, PieceGeometryConstructorParamsOverrides,
    build_group_index, build_piece_geometry,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params, build_to_fields_params,
};

/// Contract: `DerivationContext::try_new`, admitted: the index is below the
///   geometry's group count → `Ok(DerivationContext { … })`, each component
///   moved from the params.
/// Arrange: `build_derivation_context_constructor_params` overriding
///   `group_index` with `build_group_index` at `63`, the last of the default
///   geometry's sixty-four groups, so a bound one lower refuses it; every
///   other component at its builder's default.
/// Act:     `DerivationContext::try_new(params)`.
/// Assert:  the success arm, taken through `expect`, holds an asset whose
///   `as_bytes()` is `[0x01; 32]`, a deployment whose `as_bytes()` is
///   `[0x11; 32]`, a suite whose `identifier()` is `[0x22; 32]` and
///   `version()` is `1`, a parameter set whose `as_bytes()` is `[0x33; 32]`,
///   a group index whose `value()` is `63`, and a geometry whose
///   `piece_size()`, `piece_group_size()`, and `total_extent()` are `16384`,
///   `16384`, and `1048576`, each by `assert_eq!` against a literal.
#[test]
fn try_new_admits_the_last_group_of_the_geometry() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(63),
            })),
            ..Default::default()
        });

    // Act
    let context = DerivationContext::try_new(params).expect("the last group's index is admitted");

    // Assert
    assert_eq!(context.asset.as_bytes(), &[0x01; 32]);
    assert_eq!(context.deployment.as_bytes(), &[0x11; 32]);
    assert_eq!(context.suite.identifier(), &[0x22; 32]);
    assert_eq!(context.suite.version(), 1);
    assert_eq!(context.parameter_set.as_bytes(), &[0x33; 32]);
    assert_eq!(context.group_index.value(), 63);
    assert_eq!(context.geometry.piece_size(), 16384);
    assert_eq!(context.geometry.piece_group_size(), 16384);
    assert_eq!(context.geometry.total_extent(), 1048576);
}

/// Contract: `DerivationContextDescription::try_new`, one branch: any params →
///   `Ok(DerivationContextDescription)`; the error arm has no branch.
/// Arrange: the params from `build_derivation_context_description_constructor_params`.
/// Act:     `DerivationContextDescription::try_new(params)`.
/// Assert:  `result.is_ok()` is true by `assert!`.
#[test]
fn description_try_new_returns_the_description() {
    // Arrange
    let params = build_derivation_context_description_constructor_params(Default::default());

    // Act
    let result = DerivationContextDescription::try_new(params);

    // Assert
    assert!(result.is_ok());
}

/// Contract: the description implements `IEncodingContract` for
///   `DerivationContext` with `FIELDS` the asset, deployment, suite
///   identifier, suite version, parameter set, group index, piece size,
///   piece-group size, and total extent.
/// Arrange: no fixture.
/// Act:     the read of `<DerivationContextDescription as IEncodingContract>::FIELDS`.
/// Assert:  `assert_eq!` against the literal nine kinds in order.
#[test]
fn fields_declares_the_nine_canonical_kinds() {
    // Arrange

    // Act
    let fields = <DerivationContextDescription as IEncodingContract>::FIELDS;

    // Assert
    assert_eq!(
        fields,
        [
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::Unsigned16,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::Unsigned64,
            CanonicalFieldKind::Unsigned32,
            CanonicalFieldKind::Unsigned32,
            CanonicalFieldKind::Unsigned64,
        ]
    );
}

/// Contract: `to_fields` lists the root, the deployment identity, the suite
///   identifier and version, the parameter-set identifier, the group index,
///   and the geometry's three values in that order and kinds →
///   `Ok(ToFieldsSuccessReturn { fields })`.
/// Arrange: the description from `build_derivation_context_description`;
///   `build_derivation_context` overriding `geometry` with
///   `build_piece_geometry` at a piece-group size of `32768`, so the piece
///   size and the piece-group size differ and a swap of the two changes the
///   fields, every other component at its default.
/// Act:     `description.to_fields(params, &context)`.
/// Assert:  the success arm's `fields` equal `build_canonical_fields` with
///   `values` the literals `FixedBytes32([0x01; 32])`,
///   `FixedBytes32([0x11; 32])`, `FixedBytes32([0x22; 32])`, `Unsigned16(1)`,
///   `FixedBytes32([0x33; 32])`, `Unsigned64(7)`, `Unsigned32(16384)`,
///   `Unsigned32(32768)`, and `Unsigned64(1048576)`, by `assert_eq!`.
#[test]
fn to_fields_lists_the_context_in_canonical_order() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
        geometry: Some(build_piece_geometry(
            PieceGeometryConstructorParamsOverrides {
                piece_group_size: Some(32768),
                ..Default::default()
            },
        )),
        ..Default::default()
    });

    // Act
    let fields = description
        .to_fields(build_to_fields_params(Default::default()), &context)
        .expect("the rendering is infallible")
        .fields;

    // Assert
    assert_eq!(
        fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::FixedBytes32([0x01; 32]),
                CanonicalFieldValue::FixedBytes32([0x11; 32]),
                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                CanonicalFieldValue::Unsigned16(1),
                CanonicalFieldValue::FixedBytes32([0x33; 32]),
                CanonicalFieldValue::Unsigned64(7),
                CanonicalFieldValue::Unsigned32(16384),
                CanonicalFieldValue::Unsigned32(32768),
                CanonicalFieldValue::Unsigned64(1048576),
            ]),
        })
    );
}

/// Contract: `fields_to_value`, admitted: after the count and kind checks the
///   components and the context admit the values →
///   `Ok(FromFieldsSuccessReturn { described })`.
/// Arrange: the description from `build_derivation_context_description`;
///   `build_canonical_fields` with `values` the literals
///   `FixedBytes32([0x01; 32])`, `FixedBytes32([0x11; 32])`,
///   `FixedBytes32([0x22; 32])`, `Unsigned16(1)`, `FixedBytes32([0x33; 32])`,
///   `Unsigned64(7)`, `Unsigned32(16384)`, `Unsigned32(32768)`, and
///   `Unsigned64(1048576)`, distinct within each kind, so a value read from
///   the wrong position changes the context.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  the success arm's `described`, taken through `expect`, holds an
///   asset whose `as_bytes()` is `[0x01; 32]`, a deployment `[0x11; 32]`, a
///   suite identifier `[0x22; 32]` with version `1`, a parameter set
///   `[0x33; 32]`, a group index `7`, and a geometry of `16384`, `32768`,
///   and `1048576`, each by `assert_eq!` against a literal.
#[test]
fn fields_to_value_returns_the_context_the_fields_describe() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::FixedBytes32([0x33; 32]),
            CanonicalFieldValue::Unsigned64(7),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let described = description
        .fields_to_value(build_from_fields_params(Default::default()), fields)
        .expect("the fields describe an admitted context")
        .described;

    // Assert
    assert_eq!(described.asset.as_bytes(), &[0x01; 32]);
    assert_eq!(described.deployment.as_bytes(), &[0x11; 32]);
    assert_eq!(described.suite.identifier(), &[0x22; 32]);
    assert_eq!(described.suite.version(), 1);
    assert_eq!(described.parameter_set.as_bytes(), &[0x33; 32]);
    assert_eq!(described.group_index.value(), 7);
    assert_eq!(described.geometry.piece_size(), 16384);
    assert_eq!(described.geometry.piece_group_size(), 32768);
    assert_eq!(described.geometry.total_extent(), 1048576);
}
