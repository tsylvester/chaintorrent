#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    DerivationContextDescription, DerivationContextDescriptionConstructorParams,
    DerivationContextFromFieldsErrorReturn,
};
use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, FromFieldsParams,
    IEncodingContract, ToFieldsParams, build_canonical_fields,
};
use domain::{
    AssetIdentityConstructorParamsOverrides, AssetIdentityTryNewErrorReturn,
    DeploymentIdentityConstructorParamsOverrides, DeploymentIdentityTryNewErrorReturn,
    DerivationContextConstructorParamsOverrides, DerivationContextTryNewErrorReturn,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    ParameterSetIdentifierTryNewErrorReturn, PieceGeometryConstructorParamsOverrides,
    PieceGeometryTryNewErrorReturn, SuiteIdentifierConstructorParamsOverrides,
    SuiteIdentifierTryNewErrorReturn, build_asset_identity, build_deployment_identity,
    build_derivation_context, build_group_index, build_parameter_set_identifier,
    build_piece_geometry, build_suite_identifier,
};

/// Contract: an admitted context maps to its fields in canonical order with
///   each value unchanged.
/// Arrange: the reference context — asset name "@scope/example-package",
///   asset version "2.1.0-beta.3", deployment 0x0a×32, suite 0x0b×32 version 3,
///   parameter set 0x0c×32, group index 5, geometry 16384/32768/1048576.
/// Act:     `description.to_fields(ToFieldsParams, &context)`.
/// Assert:  `success.fields.values` equals the reference fields.
#[test]
fn to_fields_lists_the_reference_context_fields_in_canonical_order() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
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

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &context);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]
    );
}

/// Contract: the canonical fields map back to the context they describe.
/// Arrange: `build_canonical_fields` over the reference fields.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `success.described` equals the reference context.
#[test]
fn from_fields_returns_the_context_the_reference_fields_describe() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
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
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Ok(success) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("the canonical fields map back to the context they describe")
    };

    // Assert
    assert_eq!(success.described, context);
}

/// Contract: a field list shorter than the canonical sequence is refused with
///   both lengths.
/// Arrange: the reference fields without their last entry.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::FieldCount
///   { expected: 10, actual: 9 }`.
#[test]
fn from_fields_rejects_a_field_list_one_short() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a field list one short is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: 10,
            actual: 9
        }
    );
}

/// Contract: a field list longer than the canonical sequence is refused with
///   both lengths.
/// Arrange: the reference fields followed by `CanonicalFieldValue::Unsigned64(1)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::FieldCount
///   { expected: 10, actual: 11 }`.
#[test]
fn from_fields_rejects_a_field_list_one_long() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
            CanonicalFieldValue::Unsigned64(1),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a field list one long is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::FieldCount {
            expected: 10,
            actual: 11
        }
    );
}

/// Contract: a value whose kind differs from the canonical kind at its index
///   is refused, naming the index and the expected kind.
/// Arrange: the reference fields with index `4` replaced by
///   `CanonicalFieldValue::Unsigned32(3)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::FieldKind
///   { index: 4, expected: CanonicalFieldKind::Unsigned16 }`.
#[test]
fn from_fields_rejects_a_field_of_the_wrong_kind() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned32(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a field of the wrong kind is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 4,
            expected: CanonicalFieldKind::Unsigned16
        }
    );
}

/// Contract: of several misplaced kinds, the lowest index decides.
/// Arrange: the reference fields with index `2` replaced by
///   `CanonicalFieldValue::Text("x".to_string())` and index `7` replaced by
///   `CanonicalFieldValue::Unsigned64(16384)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::FieldKind
///   { index: 2, expected: CanonicalFieldKind::FixedBytes32 }`.
#[test]
fn from_fields_reports_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::Text("x".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned64(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a field list with misplaced kinds is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::FixedBytes32
        }
    );
}

/// Contract: a kind mismatch at any index is reported before a component
///   refusal at a lower index.
/// Arrange: the reference fields with index `0` replaced by
///   `CanonicalFieldValue::Text(String::new())` — which `AssetIdentity` would
///   refuse — and index `9` replaced by
///   `CanonicalFieldValue::Unsigned32(1048576)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::FieldKind
///   { index: 9, expected: CanonicalFieldKind::Unsigned64 }`.
#[test]
fn from_fields_checks_every_kind_before_constructing_any_component() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text(String::new()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned32(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a field list with a misplaced kind is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::FieldKind {
            index: 9,
            expected: CanonicalFieldKind::Unsigned64
        }
    );
}

/// Contract: a component's refusal is carried unchanged in its own variant.
/// Arrange: the reference fields with index `0` replaced by
///   `CanonicalFieldValue::Text(String::new())`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::
///   AssetIdentity(AssetIdentityTryNewErrorReturn::EmptyName)`.
#[test]
fn from_fields_returns_the_asset_identity_refusal_unchanged() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text(String::new()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a refused component returns its refusal")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::AssetIdentity(
            AssetIdentityTryNewErrorReturn::EmptyName
        )
    );
}

/// Contract: a component's refusal is carried unchanged in its own variant.
/// Arrange: the reference fields with index `2` replaced by
///   `CanonicalFieldValue::FixedBytes32([0u8; 32])`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::
///   DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)`.
#[test]
fn from_fields_returns_the_deployment_identity_refusal_unchanged() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0u8; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a refused component returns its refusal")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
            DeploymentIdentityTryNewErrorReturn::AllZero
        )
    );
}

/// Contract: a component's refusal is carried unchanged in its own variant.
/// Arrange: the reference fields with index `4` replaced by
///   `CanonicalFieldValue::Unsigned16(0)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::
///   SuiteIdentifier(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`.
#[test]
fn from_fields_returns_the_suite_identifier_refusal_unchanged() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(0),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a refused component returns its refusal")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::SuiteIdentifier(
            SuiteIdentifierTryNewErrorReturn::ZeroVersion
        )
    );
}

/// Contract: a component's refusal is carried unchanged in its own variant.
/// Arrange: the reference fields with index `5` replaced by
///   `CanonicalFieldValue::FixedBytes32([0u8; 32])`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::
///   ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn::AllZero)`.
#[test]
fn from_fields_returns_the_parameter_set_identifier_refusal_unchanged() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0u8; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a refused component returns its refusal")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::ParameterSetIdentifier(
            ParameterSetIdentifierTryNewErrorReturn::AllZero
        )
    );
}

/// Contract: a component's refusal is carried unchanged in its own variant.
/// Arrange: the reference fields with index `9` replaced by
///   `CanonicalFieldValue::Unsigned64(0)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::
///   PieceGeometry(PieceGeometryTryNewErrorReturn::ZeroTotalExtent)`.
#[test]
fn from_fields_returns_the_piece_geometry_refusal_unchanged() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(0),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a refused component returns its refusal")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::PieceGeometry(
            PieceGeometryTryNewErrorReturn::ZeroTotalExtent
        )
    );
}

/// Contract: the context constructor's refusal is carried unchanged in its
///   own variant.
/// Arrange: the reference fields with index `6` replaced by
///   `CanonicalFieldValue::Unsigned64(32)`, the index equal to the reference
///   geometry's group count.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::
///   DerivationContext(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange
///   { group_index: 32, group_count: 32 })`.
#[test]
fn from_fields_returns_the_derivation_context_refusal_unchanged() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("@scope/example-package".to_string()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(32),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(1048576),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a refused context returns its refusal")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::DerivationContext(
            DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
                group_index: 32,
                group_count: 32
            }
        )
    );
}

/// Contract: when several components refuse, the refusal of the earliest
///   field decides.
/// Arrange: the reference fields with index `0` replaced by
///   `CanonicalFieldValue::Text(String::new())` and index `9` replaced by
///   `CanonicalFieldValue::Unsigned64(0)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`.
/// Assert:  `error` equals `DerivationContextFromFieldsErrorReturn::
///   AssetIdentity(AssetIdentityTryNewErrorReturn::EmptyName)`.
#[test]
fn from_fields_constructs_the_components_in_field_order() {
    // Arrange
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text(String::new()),
            CanonicalFieldValue::Text("2.1.0-beta.3".to_string()),
            CanonicalFieldValue::FixedBytes32([0x0a; 32]),
            CanonicalFieldValue::FixedBytes32([0x0b; 32]),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes32([0x0c; 32]),
            CanonicalFieldValue::Unsigned64(5),
            CanonicalFieldValue::Unsigned32(16384),
            CanonicalFieldValue::Unsigned32(32768),
            CanonicalFieldValue::Unsigned64(0),
        ]),
    });

    // Act
    let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
        panic!("a field list with a refused component is refused")
    };

    // Assert
    assert_eq!(
        error,
        DerivationContextFromFieldsErrorReturn::AssetIdentity(
            AssetIdentityTryNewErrorReturn::EmptyName
        )
    );
}

/// Contract: the description's declared kinds are the canonical sequence an
///   encoding concrete decodes against.
/// Arrange: nothing.
/// Act:     read `<DerivationContextDescription as IEncodingContract>::FIELDS`.
/// Assert:  it equals the ten-kind canonical sequence as a slice.
#[test]
fn derivation_context_description_declares_its_field_kinds_in_canonical_order() {
    // Arrange

    // Act
    let fields = <DerivationContextDescription as IEncodingContract>::FIELDS;

    // Assert
    assert_eq!(
        fields,
        [
            CanonicalFieldKind::Text,
            CanonicalFieldKind::Text,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::Unsigned16,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::Unsigned64,
            CanonicalFieldKind::Unsigned32,
            CanonicalFieldKind::Unsigned32,
            CanonicalFieldKind::Unsigned64,
        ]
        .as_slice()
    );
}
