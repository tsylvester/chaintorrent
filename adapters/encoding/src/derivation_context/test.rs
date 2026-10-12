#![allow(clippy::expect_used)]

use super::provides::{
    DERIVATION_CONTEXT_FIELD_COUNT, DerivationContextDescription,
    DerivationContextFromFieldsErrorReturn, build_derivation_context_description,
    build_derivation_context_description_constructor_params,
};
use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides,
    IEncodingContract, build_canonical_fields, build_from_fields_params, build_to_fields_params,
};
use domain::{
    AssetIdentityConstructorParamsOverrides, AssetIdentityTryNewErrorReturn,
    DEPLOYMENT_IDENTITY_LENGTH, DeploymentIdentityConstructorParamsOverrides,
    DeploymentIdentityTryNewErrorReturn, DerivationContext,
    DerivationContextConstructorParamsOverrides, DerivationContextTryNewErrorReturn,
    GroupIndexConstructorParamsOverrides, PARAMETER_SET_IDENTIFIER_LENGTH,
    ParameterSetIdentifierConstructorParamsOverrides, ParameterSetIdentifierTryNewErrorReturn,
    PieceGeometryConstructorParamsOverrides, PieceGeometryTryNewErrorReturn,
    SUITE_IDENTIFIER_LENGTH, SuiteIdentifierConstructorParamsOverrides,
    SuiteIdentifierTryNewErrorReturn, build_asset_identity, build_deployment_identity,
    build_derivation_context, build_group_index, build_parameter_set_identifier,
    build_piece_geometry, build_suite_identifier,
};

fn reference_values() -> Vec<CanonicalFieldValue> {
    vec![
        CanonicalFieldValue::Text("left-pad".to_string()),
        CanonicalFieldValue::Text("1.3.0".to_string()),
        CanonicalFieldValue::FixedBytes32([0x44; DEPLOYMENT_IDENTITY_LENGTH]),
        CanonicalFieldValue::FixedBytes32([0x55; SUITE_IDENTIFIER_LENGTH]),
        CanonicalFieldValue::Unsigned16(0x0102),
        CanonicalFieldValue::FixedBytes32([0x66; PARAMETER_SET_IDENTIFIER_LENGTH]),
        CanonicalFieldValue::Unsigned64(2),
        CanonicalFieldValue::Unsigned32(32768),
        CanonicalFieldValue::Unsigned32(65536),
        CanonicalFieldValue::Unsigned64(262144),
    ]
}

fn reference_fields() -> CanonicalFields {
    build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(reference_values()),
    })
}

fn reference_context() -> DerivationContext {
    build_derivation_context(DerivationContextConstructorParamsOverrides {
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
                version: Some(0x0102),
            },
        )),
        parameter_set: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x66; PARAMETER_SET_IDENTIFIER_LENGTH]),
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
    })
}

/// Contract: `DerivationContextDescription::try_new(params)`, one branch: any
///   params → `Ok(DerivationContextDescription)`; the error arm has no branch.
/// Arrange: the params from
///   `build_derivation_context_description_constructor_params`.
/// Act:     `DerivationContextDescription::try_new(params)`.
/// Assert:  `result.is_ok()` is true.
#[test]
fn try_new_returns_the_description() {
    // Arrange
    let params = build_derivation_context_description_constructor_params(Default::default());

    // Act
    let result = DerivationContextDescription::try_new(params);

    // Assert
    assert!(result.is_ok());
}

/// Contract: `to_fields`, one branch: any admitted context →
///   `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })` with
///   `values` the asset name and version as `Text`, the deployment bytes, suite
///   identifier, and parameter-set bytes as `FixedBytes32`, the suite version
///   as `Unsigned16`, the group index as `Unsigned64`, the piece size and
///   piece-group size as `Unsigned32`, and the total extent as `Unsigned64`,
///   in that order.
/// Arrange: the reference context, whose same-kind fields hold distinct
///   values, so a swap among the text values, among the 32-byte values, among
///   the 32-bit values, or among the 64-bit values changes the list.
/// Act:     `description.to_fields(params, &context)`.
/// Assert:  the `fields` of the `Ok` arm equal the reference fields by
///   `assert_eq!`, the expectation the reference values in canonical order,
///   stated apart from the context builder's overrides.
#[test]
fn to_fields_lists_the_reference_context_fields_in_canonical_order() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_to_fields_params(Default::default());
    let context = reference_context();

    // Act
    let result = description.to_fields(params, &context);

    // Assert
    let Ok(output) = result;
    assert_eq!(output.fields, reference_fields());
}

/// Contract: `fields_to_value`, admitted: every check and constructor passes
///   → `Ok(FromFieldsSuccessReturn { described })` holding the constructed
///   context.
/// Arrange: the reference fields, whose same-kind values are distinct, so a
///   value taken from the wrong field changes the constructed context or is
///   refused.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  the `described` of the `Ok` arm equals the reference context by
///   `assert_eq!`, the reference context built by `domain`'s builders and not
///   by the subject.
#[test]
fn fields_to_value_returns_the_context_the_reference_fields_describe() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let fields = reference_fields();

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result
            .expect("the reference fields describe an admitted context")
            .described,
        reference_context()
    );
}

/// Contract: `fields_to_value`, wrong count: `payload.values` does not convert
///   into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]` →
///   `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected:
///   DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the vector's length.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values without index `9`, every value present of its declared kind, so
///   only the count refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual: 9 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_field_list_one_short() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values.remove(9);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: DERIVATION_CONTEXT_FIELD_COUNT,
            actual: 9
        })
    );
}

/// Contract: `fields_to_value`, wrong count: `payload.values` does not convert
///   into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]` →
///   `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected:
///   DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the vector's length.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values followed by `Unsigned64(0)`, every value at indexes `0` through `9`
///   of its declared kind, so only the count refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual: 11 })`
///   by `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_field_list_one_long() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values.push(CanonicalFieldValue::Unsigned64(0));
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: DERIVATION_CONTEXT_FIELD_COUNT,
            actual: 11
        })
    );
}

/// Contract: `fields_to_value`, ordering: the count precedes every kind.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values without index `9` and with the value at index `0` replaced by
///   `Unsigned16(1)`, a list both short and holding a wrong kind, so a kind
///   check ahead of the count reports the kind.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual: 9 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_checks_the_count_before_any_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values.remove(9);
    values[0] = CanonicalFieldValue::Unsigned16(1);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: DERIVATION_CONTEXT_FIELD_COUNT,
            actual: 9
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `0` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 0,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `0` replaced by `Unsigned16(1)`, every other
///   value of its declared kind, so only the kind at index `0` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 0, expected: CanonicalFieldKind::Text })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_an_asset_name_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[0] = CanonicalFieldValue::Unsigned16(1);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Text
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `1` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 1,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `1` replaced by `Unsigned16(1)`, every other
///   value of its declared kind, so only the kind at index `1` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 1, expected: CanonicalFieldKind::Text })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_an_asset_version_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[1] = CanonicalFieldValue::Unsigned16(1);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Text
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `2` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 2,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `2` replaced by `Text("left-pad")`, every
///   other value of its declared kind, so only the kind at index `2` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 2, expected: CanonicalFieldKind::FixedBytes32 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_deployment_identity_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[2] = CanonicalFieldValue::Text("left-pad".to_string());
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::FixedBytes32
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `3` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 3,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `3` replaced by `Text("left-pad")`, every
///   other value of its declared kind, so only the kind at index `3` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 3, expected: CanonicalFieldKind::FixedBytes32 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_suite_identifier_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[3] = CanonicalFieldValue::Text("left-pad".to_string());
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 3,
            expected: CanonicalFieldKind::FixedBytes32
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `4` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 4,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `4` replaced by `Unsigned32(0x0102)`, the
///   same number at a wider width, every other value of its declared kind, so
///   only the kind at index `4` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 4, expected: CanonicalFieldKind::Unsigned16 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_suite_version_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[4] = CanonicalFieldValue::Unsigned32(0x0102);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 4,
            expected: CanonicalFieldKind::Unsigned16
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `5` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 5,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `5` replaced by `Text("left-pad")`, every
///   other value of its declared kind, so only the kind at index `5` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 5, expected: CanonicalFieldKind::FixedBytes32 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_parameter_set_identifier_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[5] = CanonicalFieldValue::Text("left-pad".to_string());
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 5,
            expected: CanonicalFieldKind::FixedBytes32
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `6` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 6,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `6` replaced by `Unsigned32(2)`, the same
///   number at a narrower width, every other value of its declared kind, so
///   only the kind at index `6` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 6, expected: CanonicalFieldKind::Unsigned64 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_group_index_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[6] = CanonicalFieldValue::Unsigned32(2);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 6,
            expected: CanonicalFieldKind::Unsigned64
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `7` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 7,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `7` replaced by `Unsigned64(32768)`, the
///   same number at a wider width, every other value of its declared kind, so
///   only the kind at index `7` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 7, expected: CanonicalFieldKind::Unsigned32 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_piece_size_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[7] = CanonicalFieldValue::Unsigned64(32768);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 7,
            expected: CanonicalFieldKind::Unsigned32
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `8` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 8,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `8` replaced by `Unsigned64(65536)`, the
///   same number at a wider width, every other value of its declared kind, so
///   only the kind at index `8` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 8, expected: CanonicalFieldKind::Unsigned32 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_piece_group_size_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[8] = CanonicalFieldValue::Unsigned64(65536);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 8,
            expected: CanonicalFieldKind::Unsigned32
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   index `9` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there
///   → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 9,
///   expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `9` replaced by `Unsigned32(262144)`, the
///   same number at a narrower width, every other value of its declared kind,
///   so only the kind at index `9` refuses.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 9, expected: CanonicalFieldKind::Unsigned64 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_rejects_a_total_extent_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[9] = CanonicalFieldValue::Unsigned32(262144);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 9,
            expected: CanonicalFieldKind::Unsigned64
        })
    );
}

/// Contract: `fields_to_value`, wrong kind: the count matches and more than
///   one value is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names at its
///   index → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index,
///   expected })` for the lowest such index.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values, the value at index `3` replaced by `Text("left-pad")` and the
///   value at index `8` replaced by `Unsigned64(65536)`, so a check that runs
///   from the highest index reports index `8`.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 3, expected: CanonicalFieldKind::FixedBytes32 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_reports_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[3] = CanonicalFieldValue::Text("left-pad".to_string());
    values[8] = CanonicalFieldValue::Unsigned64(65536);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 3,
            expected: CanonicalFieldKind::FixedBytes32
        })
    );
}

/// Contract: `fields_to_value`, ordering: every kind precedes every
///   constructor.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values, the value at index `0` replaced by `Text("")`, the empty asset
///   name `AssetIdentity::try_new` refuses, and the value at index `9`
///   replaced by `Unsigned32(262144)`, so a constructor that ran before the
///   kind at index `9` was checked returns the asset-identity refusal.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::FieldKind { index: 9, expected: CanonicalFieldKind::Unsigned64 })` by
///   `assert_eq!`.
#[test]
fn fields_to_value_checks_every_kind_before_constructing_any_component() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[0] = CanonicalFieldValue::Text(String::new());
    values[9] = CanonicalFieldValue::Unsigned32(262144);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 9,
            expected: CanonicalFieldKind::Unsigned64
        })
    );
}

/// Contract: `fields_to_value`, component refused: every kind matches and
///   `AssetIdentity::try_new` refuses → `Err` holding the refusal unchanged in
///   the `AssetIdentity` variant.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `0` replaced by `Text("left pad")`, whose
///   space at index `4` is outside visible ASCII, every other component
///   admitted.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::AssetIdentity(AssetIdentityTryNewErrorReturn::
///   NameByteOutsideVisibleAscii { index: 4, byte: 0x20 }))` by `assert_eq!`,
///   the whole refusal the asset identity's constructor declares for that
///   name.
#[test]
fn fields_to_value_returns_the_asset_identity_refusal_unchanged() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[0] = CanonicalFieldValue::Text("left pad".to_string());
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::AssetIdentity(
            AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii {
                index: 4,
                byte: 0x20
            }
        ))
    );
}

/// Contract: `fields_to_value`, component refused: every kind matches and
///   `DeploymentIdentity::try_new` refuses → `Err` holding the refusal
///   unchanged in the `DeploymentIdentity` variant.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `2` replaced by `FixedBytes32` of bytes each
///   `0x00`, every other component admitted.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero))` by
///   `assert_eq!`.
#[test]
fn fields_to_value_returns_the_deployment_identity_refusal_unchanged() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[2] = CanonicalFieldValue::FixedBytes32([0x00; DEPLOYMENT_IDENTITY_LENGTH]);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
            DeploymentIdentityTryNewErrorReturn::AllZero
        ))
    );
}

/// Contract: `fields_to_value`, component refused: every kind matches and
///   `SuiteIdentifier::try_new` refuses → `Err` holding the refusal unchanged
///   in the `SuiteIdentifier` variant.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `3` replaced by `FixedBytes32` of bytes each
///   `0x00`, the suite version at index `4` keeping `Unsigned16(0x0102)`, every
///   other component admitted.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::SuiteIdentifier(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier))`
///   by `assert_eq!`.
#[test]
fn fields_to_value_returns_the_suite_identifier_refusal_unchanged() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[3] = CanonicalFieldValue::FixedBytes32([0x00; SUITE_IDENTIFIER_LENGTH]);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::SuiteIdentifier(
            SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier
        ))
    );
}

/// Contract: `fields_to_value`, component refused: every kind matches and
///   `ParameterSetIdentifier::try_new` refuses → `Err` holding the refusal
///   unchanged in the `ParameterSetIdentifier` variant.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `5` replaced by `FixedBytes32` of bytes each
///   `0x00`, every other component admitted.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn::
///   AllZero))` by `assert_eq!`.
#[test]
fn fields_to_value_returns_the_parameter_set_identifier_refusal_unchanged() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[5] = CanonicalFieldValue::FixedBytes32([0x00; PARAMETER_SET_IDENTIFIER_LENGTH]);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(
            DerivationContextFromFieldsErrorReturn::ParameterSetIdentifier(
                ParameterSetIdentifierTryNewErrorReturn::AllZero
            )
        )
    );
}

/// Contract: `fields_to_value`, component refused: every kind matches and
///   `PieceGeometry::try_new` refuses → `Err` holding the refusal unchanged in
///   the `PieceGeometry` variant.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the value at index `8` replaced by `Unsigned32(20000)`, a
///   piece-group size that is not a multiple of the piece size `32768` at
///   index `7`, every other component admitted.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::PieceGeometry(PieceGeometryTryNewErrorReturn::
///   PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 20000,
///   piece_size: 32768 }))` by `assert_eq!`.
#[test]
fn fields_to_value_returns_the_piece_geometry_refusal_unchanged() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[8] = CanonicalFieldValue::Unsigned32(20000);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::PieceGeometry(
            PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize {
                piece_group_size: 20000,
                piece_size: 32768
            }
        ))
    );
}

/// Contract: `fields_to_value`, component refused: the constructors are called
///   in field order and none after the first refusal → `Err` holding the first
///   refusal unchanged.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values, the value at index `0` replaced by `Text("")`, which
///   `AssetIdentity::try_new` refuses, and the value at index `8` replaced by
///   `Unsigned32(20000)`, which `PieceGeometry::try_new` refuses, the first and
///   the last component constructed, so a reverse or interleaved order reports
///   the geometry refusal.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::AssetIdentity(AssetIdentityTryNewErrorReturn::EmptyName))` by
///   `assert_eq!`.
#[test]
fn fields_to_value_constructs_the_components_in_field_order() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[0] = CanonicalFieldValue::Text(String::new());
    values[8] = CanonicalFieldValue::Unsigned32(20000);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::AssetIdentity(
            AssetIdentityTryNewErrorReturn::EmptyName
        ))
    );
}

/// Contract: `fields_to_value`, context refused: every component is admitted
///   and `DerivationContext::try_new` refuses →
///   `Err(DerivationContextFromFieldsErrorReturn::DerivationContext(error))`,
///   the refusal unchanged.
/// Arrange: `build_canonical_fields` overriding `values` with the reference
///   values and the values at indexes `6` through `9` replaced by
///   `Unsigned64(4)`, `Unsigned32(16384)`, `Unsigned32(16384)`, and
///   `Unsigned64(65536)`, a geometry of four groups and a group index of `4`,
///   one beyond its last group, every component admitted.
/// Act:     `description.fields_to_value(params, fields)`.
/// Assert:  `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn
///   ::DerivationContext(DerivationContextTryNewErrorReturn::
///   GroupIndexOutOfRange { group_index: 4, group_count: 4 }))` by
///   `assert_eq!`.
#[test]
fn fields_to_value_returns_the_derivation_context_refusal_unchanged() {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let params = build_from_fields_params(Default::default());
    let mut values = reference_values();
    values[6] = CanonicalFieldValue::Unsigned64(4);
    values[7] = CanonicalFieldValue::Unsigned32(16384);
    values[8] = CanonicalFieldValue::Unsigned32(16384);
    values[9] = CanonicalFieldValue::Unsigned64(65536);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    });

    // Act
    let result = description.fields_to_value(params, fields);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextFromFieldsErrorReturn::DerivationContext(
            DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
                group_index: 4,
                group_count: 4
            }
        ))
    );
}
