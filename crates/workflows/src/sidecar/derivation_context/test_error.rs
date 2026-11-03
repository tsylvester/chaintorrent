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
    CanonicalFieldReaderErrorReturn, CanonicalFieldValue, CanonicalFieldsOverrides,
    IEncodingContract, build_canonical_fields, build_from_fields_params,
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

/// Contract: `fields_to_value`, fields refused: the canonical field reader
///   refuses the list → `Err(DerivationContextFromFieldsErrorReturn
///   ::Fields(error))`, `error` unchanged.
/// Arrange: `build_canonical_fields` overriding `values` with the first eight
///   values of the valid rendering, so the reader refuses the count.
/// Act:     `fields_to_value` over those fields.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
#[test]
fn fields_to_value_carries_the_readers_refusal() {
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
        Some(DerivationContextFromFieldsErrorReturn::Fields(
            CanonicalFieldReaderErrorReturn::Count {
                expected: 9,
                actual: 8
            }
        ))
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
