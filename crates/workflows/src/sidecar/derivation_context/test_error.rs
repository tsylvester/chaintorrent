use super::provides::{
    DerivationContext, DerivationContextConstructorParamsCorruptions,
    DerivationContextConstructorParamsOverrides, DerivationContextFromFieldsErrorReturn,
    DerivationContextTryNewErrorReturn, build_derivation_context_constructor_params,
    build_derivation_context_description, invalidate_derivation_context_constructor_params,
};
use domain::{
    DeploymentIdentityTryNewErrorReturn, GroupIndexConstructorParamsOverrides,
    ParameterSetIdentifierTryNewErrorReturn, PieceGeometryTryNewErrorReturn,
    SuiteIdentifierTryNewErrorReturn, build_group_index,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params,
};
use hashing::RootTryNewErrorReturn;

/// Contract: `DerivationContext::try_new`, out of range: the index is not
///   below the geometry's group count → `Err(DerivationContextTryNewErrorReturn
///   ::GroupIndexOutOfRange { group_index, group_count })`.
/// Arrange: `build_derivation_context_constructor_params` overriding
///   `group_index` with `build_group_index` at `64`, the default geometry's
///   group count, the lowest index refused, so an inclusive bound or a bound
///   against the piece count admits it.
/// Act:     `DerivationContext::try_new(params)`.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn try_new_refuses_a_group_index_equal_to_the_group_count() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(64),
            })),
            ..Default::default()
        });

    // Act
    let result = DerivationContext::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
            group_index: 64,
            group_count: 64
        })
    );
}

/// Contract: `fields_to_value`, wrong count: `payload.values` does not
///   convert into the nine-field array →
///   `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected, actual })`.
/// Arrange: `build_canonical_fields` overriding `values` with the first eight
///   values of the valid rendering, so only the count differs.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_rejects_one_value_short() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::FixedBytes32([0x33; 32]),
            CanonicalFieldValue::Unsigned64(7),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(16384),
        ]),
    });

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: 9,
            actual: 8
        })
    );
}

/// Contract: `fields_to_value`, wrong count →
///   `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected, actual })`.
/// Arrange: `build_canonical_fields` overriding `values` with the nine values
///   of the valid rendering followed by `Unsigned64(1048576)`, so only the
///   count differs.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_rejects_one_value_long() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
            CanonicalFieldValue::FixedBytes32([0x11; 32]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::FixedBytes32([0x33; 32]),
            CanonicalFieldValue::Unsigned64(7),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned64(1048576),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: 9,
            actual: 10
        })
    );
}

/// Contract: ordering: the count check precedes the kind check.
/// Arrange: `build_canonical_fields` overriding `values` with one
///   `CanonicalFieldValue::Text`, a payload both short and of the wrong kind,
///   so a kind check ahead of the count reports the kind.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_checks_the_count_before_any_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![CanonicalFieldValue::Text(String::from("x"))]),
    });

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: 9,
            actual: 1
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the value at an index is not of
///   the kind named there →
///   `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index, expected })`.
/// Arrange: `invalidate_derivation_context_constructor_params` with `suite`
///   corrupted to `[FixedBytes32([0x22; 32]), Unsigned32(1)]`, the version at
///   the neighboring width, every other field valid, so a check that
///   compares only integer-ness admits it.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_suite_version_of_another_width() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            suite: Some(vec![
                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                CanonicalFieldValue::Unsigned32(1),
            ]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 3,
            expected: CanonicalFieldKind::Unsigned16
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: more than one value is of the
///   wrong kind → the lowest such index.
/// Arrange: `invalidate_derivation_context_constructor_params` with
///   `deployment` corrupted to `[Text("x")]` and `geometry` to
///   `[Unsigned32(16384), Unsigned32(16384), Text("x")]`, so a check that
///   runs from the highest index reports index `8`.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_reports_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            deployment: Some(vec![CanonicalFieldValue::Text(String::from("x"))]),
            geometry: Some(vec![
                CanonicalFieldValue::Unsigned32(16384),
                CanonicalFieldValue::Unsigned32(16384),
                CanonicalFieldValue::Text(String::from("x")),
            ]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::FixedBytes32
        })
    );
}

/// Contract: ordering: every kind is checked before any component's
///   constructor runs.
/// Arrange: `invalidate_derivation_context_constructor_params` with `asset`
///   corrupted to `[FixedBytes32([0x00; 32])]`, a value the root refuses, and
///   `geometry` to `[Unsigned32(16384), Unsigned32(16384), Text("x")]`, so a
///   constructor run ahead of the last kind check reports the asset's
///   refusal.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_checks_every_kind_before_admitting_any_component() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            asset: Some(vec![CanonicalFieldValue::FixedBytes32([0x00; 32])]),
            geometry: Some(vec![
                CanonicalFieldValue::Unsigned32(16384),
                CanonicalFieldValue::Unsigned32(16384),
                CanonicalFieldValue::Text(String::from("x")),
            ]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 8,
            expected: CanonicalFieldKind::Unsigned64
        })
    );
}

/// Contract: `fields_to_value`, asset refused →
///   `Err(DerivationContextFromFieldsErrorReturn::Asset(error))`, the refusal
///   whole.
/// Arrange: `invalidate_derivation_context_constructor_params` with `asset`
///   corrupted to `[FixedBytes32([0x00; 32])]`, every other field valid.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_carries_the_root_refusal_whole() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            asset: Some(vec![CanonicalFieldValue::FixedBytes32([0x00; 32])]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::Asset(
            RootTryNewErrorReturn::AllZero
        ))
    );
}

/// Contract: `fields_to_value`, deployment refused →
///   `Err(DerivationContextFromFieldsErrorReturn::Deployment(error))`, the
///   refusal whole.
/// Arrange: `invalidate_derivation_context_constructor_params` with
///   `deployment` corrupted to `[FixedBytes32([0x00; 32])]`, every other
///   field valid.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_carries_the_deployment_refusal_whole() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            deployment: Some(vec![CanonicalFieldValue::FixedBytes32([0x00; 32])]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::Deployment(
            DeploymentIdentityTryNewErrorReturn::AllZero
        ))
    );
}

/// Contract: `fields_to_value`, suite refused →
///   `Err(DerivationContextFromFieldsErrorReturn::Suite(error))`, the refusal
///   whole.
/// Arrange: `invalidate_derivation_context_constructor_params` with `suite`
///   corrupted to `[FixedBytes32([0x22; 32]), Unsigned16(0)]`, every other
///   field valid, so the version and not the identifier is refused.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_carries_the_suite_refusal_whole() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            suite: Some(vec![
                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                CanonicalFieldValue::Unsigned16(0),
            ]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::Suite(
            SuiteIdentifierTryNewErrorReturn::ZeroVersion
        ))
    );
}

/// Contract: `fields_to_value`, parameter set refused →
///   `Err(DerivationContextFromFieldsErrorReturn::ParameterSet(error))`, the
///   refusal whole.
/// Arrange: `invalidate_derivation_context_constructor_params` with
///   `parameter_set` corrupted to `[FixedBytes32([0x00; 32])]`, every other
///   field valid.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_carries_the_parameter_set_refusal_whole() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            parameter_set: Some(vec![CanonicalFieldValue::FixedBytes32([0x00; 32])]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::ParameterSet(
            ParameterSetIdentifierTryNewErrorReturn::AllZero
        ))
    );
}

/// Contract: `fields_to_value`, geometry refused →
///   `Err(DerivationContextFromFieldsErrorReturn::Geometry(error))`, the
///   refusal whole.
/// Arrange: `invalidate_derivation_context_constructor_params` with
///   `geometry` corrupted to `[Unsigned32(16384), Unsigned32(24576),
///   Unsigned64(1048576)]`, a piece-group size of one and a half pieces,
///   every other field valid.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_carries_the_geometry_refusal_whole() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            geometry: Some(vec![
                CanonicalFieldValue::Unsigned32(16384),
                CanonicalFieldValue::Unsigned32(24576),
                CanonicalFieldValue::Unsigned64(1048576),
            ]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::Geometry(
            PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize {
                piece_group_size: 24576,
                piece_size: 16384
            }
        ))
    );
}

/// Contract: `fields_to_value`, context refused →
///   `Err(DerivationContextFromFieldsErrorReturn::Context(error))`, the
///   refusal whole.
/// Arrange: `invalidate_derivation_context_constructor_params` with
///   `group_index` corrupted to `[Unsigned64(64)]`, the default geometry's
///   group count, every other field valid.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_carries_the_context_refusal_whole() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            group_index: Some(vec![CanonicalFieldValue::Unsigned64(64)]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::Context(
            DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
                group_index: 64,
                group_count: 64
            }
        ))
    );
}

/// Contract: ordering: the components are admitted in field order and the
///   first refusal is returned.
/// Arrange: `invalidate_derivation_context_constructor_params` with `asset`
///   and `parameter_set` both corrupted to `[FixedBytes32([0x00; 32])]`, so
///   an admission that runs from the last component, or reports the last
///   refusal, returns the parameter set's.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_reports_the_first_component_refusal() {
    // Arrange
    let fields = invalidate_derivation_context_constructor_params(
        DerivationContextConstructorParamsCorruptions {
            asset: Some(vec![CanonicalFieldValue::FixedBytes32([0x00; 32])]),
            parameter_set: Some(vec![CanonicalFieldValue::FixedBytes32([0x00; 32])]),
            ..Default::default()
        },
    );

    // Act
    let result = build_derivation_context_description(Default::default())
        .fields_to_value(build_from_fields_params(Default::default()), fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::Asset(
            RootTryNewErrorReturn::AllZero
        ))
    );
}
