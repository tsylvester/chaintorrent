use super::provides::*;
use encoding::CanonicalFieldKind;

/// Contract: `SolidityLibraryName::try_new(params)`: empty text →
///   `Err(SolidityLibraryNameTryNewErrorReturn::Empty)`.
/// Arrange: `build_solidity_library_name_constructor_params` overriding
///   `text` with the empty string.
/// Act:     `SolidityLibraryName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityLibraryNameTryNewErrorReturn::Empty)`
///   by `assert_eq!`.
#[test]
fn solidity_library_name_refuses_empty_text() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let result = SolidityLibraryName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityLibraryNameTryNewErrorReturn::Empty)
    );
}

/// Contract: `SolidityLibraryName::try_new(params)`: a leading byte outside
///   `b'A'..=b'Z'` → `Err(LeadingCharacter { byte })`, decided before the
///   later bytes.
/// Arrange: `build_solidity_library_name_constructor_params` overriding
///   `text` with `a-`, a leading byte the later set admits and a later byte
///   it refuses.
/// Act:     `SolidityLibraryName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityLibraryNameTryNewErrorReturn::
///   LeadingCharacter { byte: b'a' })` by `assert_eq!`.
#[test]
fn solidity_library_name_refuses_a_leading_byte_outside_uppercase_letters() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some("a-".to_string()),
        },
    );

    // Act
    let result = SolidityLibraryName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte: b'a' })
    );
}

/// Contract: `SolidityLibraryName::try_new(params)`: the lowest later index
///   whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'` →
///   `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_library_name_constructor_params` overriding
///   `text` with `Ab_c-d`, whose `_` at index `2` is refused here and
///   admitted by a constant name, and whose `-` at index `4` is a later
///   refusal.
/// Act:     `SolidityLibraryName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityLibraryNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'_' })` by `assert_eq!`.
#[test]
fn solidity_library_name_refuses_the_lowest_later_byte_outside_letters_and_digits() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some("Ab_c-d".to_string()),
        },
    );

    // Act
    let result = SolidityLibraryName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        })
    );
}

/// Contract: `SolidityLibraryName::try_new(params)`: a text whose leading
///   byte is in `b'A'..=b'Z'` and whose later bytes are in `b'A'..=b'Z'`,
///   `b'a'..=b'z'`, or `b'0'..=b'9'` → `Ok(SolidityLibraryName { text })`,
///   the text moved from the params.
/// Arrange: `build_solidity_library_name_constructor_params` overriding
///   `text` with `AZaz09`, the inclusive ends of every range.
/// Act:     `SolidityLibraryName::try_new(params)`.
/// Assert:  `result.ok().map(|name| name.text)` equals `Some("AZaz09")`
///   by `assert_eq!`.
#[test]
fn solidity_library_name_admits_every_boundary_byte_of_its_sets() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some("AZaz09".to_string()),
        },
    );

    // Act
    let result = SolidityLibraryName::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|name| name.text),
        Some("AZaz09".to_string())
    );
}

/// Contract: `SolidityConstantName::try_new(params)`: empty text →
///   `Err(SolidityConstantNameTryNewErrorReturn::Empty)`.
/// Arrange: `build_solidity_constant_name_constructor_params` overriding
///   `text` with the empty string.
/// Act:     `SolidityConstantName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityConstantNameTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn solidity_constant_name_refuses_empty_text() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let result = SolidityConstantName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityConstantNameTryNewErrorReturn::Empty)
    );
}

/// Contract: `SolidityConstantName::try_new(params)`: a leading byte outside
///   `b'A'..=b'Z'` → `Err(LeadingCharacter { byte })`, decided before the
///   later bytes.
/// Arrange: `build_solidity_constant_name_constructor_params` overriding
///   `text` with `_a`, a leading `_` the later set admits and a later
///   lowercase byte it refuses.
/// Act:     `SolidityConstantName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityConstantNameTryNewErrorReturn::
///   LeadingCharacter { byte: b'_' })` by `assert_eq!`.
#[test]
fn solidity_constant_name_refuses_a_leading_byte_outside_uppercase_letters() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some("_a".to_string()),
        },
    );

    // Act
    let result = SolidityConstantName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: b'_' })
    );
}

/// Contract: `SolidityConstantName::try_new(params)`: the lowest later index
///   whose byte is outside `b'A'..=b'Z'`, `b'0'..=b'9'`, and `b'_'` →
///   `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_constant_name_constructor_params` overriding
///   `text` with `A_b-`, an admitted `_` at index `1`, a refused lowercase
///   `b` at index `2`, and a later refused `-`.
/// Act:     `SolidityConstantName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityConstantNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'b' })` by `assert_eq!`.
#[test]
fn solidity_constant_name_refuses_the_lowest_later_byte_outside_uppercase_digits_and_underscore() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some("A_b-".to_string()),
        },
    );

    // Act
    let result = SolidityConstantName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityConstantNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'b'
        })
    );
}

/// Contract: `SolidityConstantName::try_new(params)`: a text within the
///   leading and later sets → `Ok(SolidityConstantName { text })`, the text
///   moved from the params.
/// Arrange: `build_solidity_constant_name_constructor_params` overriding
///   `text` with `AZ_09`, the inclusive ends of every range and the
///   underscore.
/// Act:     `SolidityConstantName::try_new(params)`.
/// Assert:  `result.ok().map(|name| name.text)` equals `Some("AZ_09")`
///   by `assert_eq!`.
#[test]
fn solidity_constant_name_admits_every_boundary_byte_of_its_sets() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some("AZ_09".to_string()),
        },
    );

    // Act
    let result = SolidityConstantName::try_new(params);

    // Assert
    assert_eq!(result.ok().map(|name| name.text), Some("AZ_09".to_string()));
}

/// Contract: `SolidityTypeName::try_new(params)`: empty text →
///   `Err(SolidityTypeNameTryNewErrorReturn::Empty)`.
/// Arrange: `build_solidity_type_name_constructor_params` overriding `text`
///   with the empty string.
/// Act:     `SolidityTypeName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityTypeNameTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn solidity_type_name_refuses_empty_text() {
    // Arrange
    let params =
        build_solidity_type_name_constructor_params(SolidityTypeNameConstructorParamsOverrides {
            text: Some(String::new()),
        });

    // Act
    let result = SolidityTypeName::try_new(params);

    // Assert
    assert_eq!(result.err(), Some(SolidityTypeNameTryNewErrorReturn::Empty));
}

/// Contract: `SolidityTypeName::try_new(params)`: a leading byte outside
///   `b'A'..=b'Z'` → `Err(LeadingCharacter { byte })`, decided before the
///   later bytes.
/// Arrange: `build_solidity_type_name_constructor_params` overriding `text`
///   with a lowercase `a` followed by `-`.
/// Act:     `SolidityTypeName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityTypeNameTryNewErrorReturn::
///   LeadingCharacter { byte: b'a' })` by `assert_eq!`.
#[test]
fn solidity_type_name_refuses_a_leading_byte_outside_uppercase_letters() {
    // Arrange
    let params =
        build_solidity_type_name_constructor_params(SolidityTypeNameConstructorParamsOverrides {
            text: Some("a-".to_string()),
        });

    // Act
    let result = SolidityTypeName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'a' })
    );
}

/// Contract: `SolidityTypeName::try_new(params)`: the lowest later index
///   whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'` →
///   `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_type_name_constructor_params` overriding `text`
///   with `Ab_c-d`.
/// Act:     `SolidityTypeName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityTypeNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'_' })` by `assert_eq!`.
#[test]
fn solidity_type_name_refuses_the_lowest_later_byte_outside_letters_and_digits() {
    // Arrange
    let params =
        build_solidity_type_name_constructor_params(SolidityTypeNameConstructorParamsOverrides {
            text: Some("Ab_c-d".to_string()),
        });

    // Act
    let result = SolidityTypeName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityTypeNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        })
    );
}

/// Contract: `SolidityTypeName::try_new(params)`: a text within the leading
///   and later sets → `Ok(SolidityTypeName { text })`, the text moved from
///   the params.
/// Arrange: `build_solidity_type_name_constructor_params` overriding `text`
///   with `AZaz09`.
/// Act:     `SolidityTypeName::try_new(params)`.
/// Assert:  `result.ok().map(|name| name.text)` equals `Some("AZaz09")`
///   by `assert_eq!`.
#[test]
fn solidity_type_name_admits_every_boundary_byte_of_its_sets() {
    // Arrange
    let params =
        build_solidity_type_name_constructor_params(SolidityTypeNameConstructorParamsOverrides {
            text: Some("AZaz09".to_string()),
        });

    // Act
    let result = SolidityTypeName::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|name| name.text),
        Some("AZaz09".to_string())
    );
}

/// Contract: `SolidityMemberName::try_new(params)`: empty text →
///   `Err(SolidityMemberNameTryNewErrorReturn::Empty)`.
/// Arrange: `build_solidity_member_name_constructor_params` overriding
///   `text` with the empty string.
/// Act:     `SolidityMemberName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityMemberNameTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn solidity_member_name_refuses_empty_text() {
    // Arrange
    let params = build_solidity_member_name_constructor_params(
        SolidityMemberNameConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let result = SolidityMemberName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityMemberNameTryNewErrorReturn::Empty)
    );
}

/// Contract: `SolidityMemberName::try_new(params)`: a leading byte outside
///   `b'a'..=b'z'` → `Err(LeadingCharacter { byte })`, decided before the
///   later bytes.
/// Arrange: `build_solidity_member_name_constructor_params` overriding
///   `text` with an uppercase `A` followed by `-`, a leading byte the later
///   set admits.
/// Act:     `SolidityMemberName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityMemberNameTryNewErrorReturn::
///   LeadingCharacter { byte: b'A' })` by `assert_eq!`.
#[test]
fn solidity_member_name_refuses_a_leading_byte_outside_lowercase_letters() {
    // Arrange
    let params = build_solidity_member_name_constructor_params(
        SolidityMemberNameConstructorParamsOverrides {
            text: Some("A-".to_string()),
        },
    );

    // Act
    let result = SolidityMemberName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'A' })
    );
}

/// Contract: `SolidityMemberName::try_new(params)`: the lowest later index
///   whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'` →
///   `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_member_name_constructor_params` overriding
///   `text` with `ab_c-d`.
/// Act:     `SolidityMemberName::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityMemberNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'_' })` by `assert_eq!`.
#[test]
fn solidity_member_name_refuses_the_lowest_later_byte_outside_letters_and_digits() {
    // Arrange
    let params = build_solidity_member_name_constructor_params(
        SolidityMemberNameConstructorParamsOverrides {
            text: Some("ab_c-d".to_string()),
        },
    );

    // Act
    let result = SolidityMemberName::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityMemberNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        })
    );
}

/// Contract: `SolidityMemberName::try_new(params)`: a text within the
///   leading and later sets → `Ok(SolidityMemberName { text })`, the text
///   moved from the params.
/// Arrange: `build_solidity_member_name_constructor_params` overriding
///   `text` with `azAZ09`.
/// Act:     `SolidityMemberName::try_new(params)`.
/// Assert:  `result.ok().map(|name| name.text)` equals `Some("azAZ09")`
///   by `assert_eq!`.
#[test]
fn solidity_member_name_admits_every_boundary_byte_of_its_sets() {
    // Arrange
    let params = build_solidity_member_name_constructor_params(
        SolidityMemberNameConstructorParamsOverrides {
            text: Some("azAZ09".to_string()),
        },
    );

    // Act
    let result = SolidityMemberName::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|name| name.text),
        Some("azAZ09".to_string())
    );
}

/// Contract: `SpdxLicenseIdentifier::try_new(params)`: an identifier part
///   with no byte → `Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty)`.
/// Arrange: `build_spdx_license_identifier_constructor_params` overriding
///   `text` with the empty string.
/// Act:     `SpdxLicenseIdentifier::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SpdxLicenseIdentifierTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn spdx_license_identifier_refuses_empty_text() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let result = SpdxLicenseIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SpdxLicenseIdentifierTryNewErrorReturn::Empty)
    );
}

/// Contract: `SpdxLicenseIdentifier::try_new(params)`: a text whose only
///   byte is the final `+` has an empty identifier part →
///   `Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty)`.
/// Arrange: `build_spdx_license_identifier_constructor_params` overriding
///   `text` with `+`.
/// Act:     `SpdxLicenseIdentifier::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SpdxLicenseIdentifierTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn spdx_license_identifier_refuses_a_lone_plus_as_empty() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("+".to_string()),
        },
    );

    // Act
    let result = SpdxLicenseIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SpdxLicenseIdentifierTryNewErrorReturn::Empty)
    );
}

/// Contract: `SpdxLicenseIdentifier::try_new(params)`: the lowest index of
///   the identifier part whose byte is outside letters, digits, `.`, and `-`
///   → `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_spdx_license_identifier_constructor_params` overriding
///   `text` with `A_B C`, a refused `_` at index `1` and a later refused
///   space.
/// Act:     `SpdxLicenseIdentifier::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SpdxLicenseIdentifierTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: b'_' })` by `assert_eq!`.
#[test]
fn spdx_license_identifier_refuses_the_lowest_byte_outside_its_set() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("A_B C".to_string()),
        },
    );

    // Act
    let result = SpdxLicenseIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: b'_'
        })
    );
}

/// Contract: `SpdxLicenseIdentifier::try_new(params)`: only one `+`, as the
///   final byte, is outside the identifier part, so a second `+` is refused
///   with its index in the whole text.
/// Arrange: `build_spdx_license_identifier_constructor_params` overriding
///   `text` with `MIT++`.
/// Act:     `SpdxLicenseIdentifier::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SpdxLicenseIdentifierTryNewErrorReturn::
///   InvalidCharacter { index: 3, byte: b'+' })` by `assert_eq!`.
#[test]
fn spdx_license_identifier_refuses_a_plus_that_is_not_the_only_final_byte() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("MIT++".to_string()),
        },
    );

    // Act
    let result = SpdxLicenseIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
            index: 3,
            byte: b'+'
        })
    );
}

/// Contract: `SpdxLicenseIdentifier::try_new(params)`: a text within the
///   set → `Ok(SpdxLicenseIdentifier { text })`, the text moved from the
///   params.
/// Arrange: `build_spdx_license_identifier_constructor_params` overriding
///   `text` with `AZaz09.-`.
/// Act:     `SpdxLicenseIdentifier::try_new(params)`.
/// Assert:  `result.ok().map(|identifier| identifier.text)` equals
///   `Some("AZaz09.-")` by `assert_eq!`.
#[test]
fn spdx_license_identifier_admits_every_boundary_byte_of_its_set() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("AZaz09.-".to_string()),
        },
    );

    // Act
    let result = SpdxLicenseIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|identifier| identifier.text),
        Some("AZaz09.-".to_string())
    );
}

/// Contract: `SpdxLicenseIdentifier::try_new(params)`: a final `+` is
///   admitted and the whole text, `+` included, is held.
/// Arrange: `build_spdx_license_identifier_constructor_params` overriding
///   `text` with `GPL-2.0+`.
/// Act:     `SpdxLicenseIdentifier::try_new(params)`.
/// Assert:  `result.ok().map(|identifier| identifier.text)` equals
///   `Some("GPL-2.0+")` by `assert_eq!`.
#[test]
fn spdx_license_identifier_admits_a_final_plus_and_keeps_it() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("GPL-2.0+".to_string()),
        },
    );

    // Act
    let result = SpdxLicenseIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|identifier| identifier.text),
        Some("GPL-2.0+".to_string())
    );
}

/// Contract: `SolidityStringLiteral::try_new(params)`: a byte equal to
///   `b'"'` → `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with `ab"c`.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: 0x22 })` by `assert_eq!`.
#[test]
fn solidity_string_literal_refuses_a_double_quote() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("ab\"c".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: 0x22
        })
    );
}

/// Contract: `SolidityStringLiteral::try_new(params)`: a byte equal to
///   `b'\\'` → `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with `ab\c`.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: 0x5C })` by `assert_eq!`.
#[test]
fn solidity_string_literal_refuses_a_backslash() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("ab\\c".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: 0x5C
        })
    );
}

/// Contract: `SolidityStringLiteral::try_new(params)`: a byte below `0x20`
///   → `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with `a` followed by the byte `0x1F`.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: 0x1F })` by `assert_eq!`.
#[test]
fn solidity_string_literal_refuses_a_byte_below_space() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\u{1F}".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: 0x1F
        })
    );
}

/// Contract: `SolidityStringLiteral::try_new(params)`: a byte above `0x7E`
///   → `Err(InvalidCharacter { index, byte })`.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with `a` followed by the byte `0x7F`.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: 0x7F })` by `assert_eq!`.
#[test]
fn solidity_string_literal_refuses_a_byte_above_tilde() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\u{7F}".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: 0x7F
        })
    );
}

/// Contract: `SolidityStringLiteral::try_new(params)`: the decision is over
///   bytes, so a non-ASCII character is refused at its first byte.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with the one character `é`, whose UTF-8 bytes are `0xC3 0xA9`.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: 0xC3 })` by `assert_eq!`.
#[test]
fn solidity_string_literal_refuses_the_first_byte_of_a_non_ascii_character() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("é".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: 0xC3
        })
    );
}

/// Contract: `SolidityStringLiteral::try_new(params)`: the lowest index
///   outside the set is reported.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with `a"b\`, a `"` at index `1` and a `\` at index `3`.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: 0x22 })` by `assert_eq!`.
#[test]
fn solidity_string_literal_refuses_the_lowest_index_when_several_bytes_are_refused() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\"b\\".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: 0x22
        })
    );
}

/// Contract: `SolidityStringLiteral::try_new(params)`: the empty string is
///   admitted → `Ok(SolidityStringLiteral { text })`.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with the empty string.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.ok().map(|literal| literal.text)` equals `Some("")`
///   by `assert_eq!`.
#[test]
fn solidity_string_literal_admits_the_empty_string() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(result.ok().map(|literal| literal.text), Some(String::new()));
}

/// Contract: `SolidityStringLiteral::try_new(params)`: bytes `0x20` through
///   `0x7E` other than `"` and `\` are admitted and the text is moved from
///   the params.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with the space, `!`, `#`, `[`, `]`, `^`, and `~`, the printable
///   boundaries and the neighbours of the two excluded bytes.
/// Act:     `SolidityStringLiteral::try_new(params)`.
/// Assert:  `result.ok().map(|literal| literal.text)` equals
///   `Some(" !#[]^~")` by `assert_eq!`.
#[test]
fn solidity_string_literal_admits_every_printable_byte_beside_the_excluded_ones() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some(" !#[]^~".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|literal| literal.text),
        Some(" !#[]^~".to_string())
    );
}

/// Contract: `SolidityUint256::try_new(params)`: one branch →
///   `Ok(SolidityUint256 { big_endian })`, the array moved from the params.
/// Arrange: `build_solidity_uint256_constructor_params` overriding
///   `big_endian` with the array whose byte at each index equals the index,
///   which differs from the builder default.
/// Act:     `SolidityUint256::try_new(params)`.
/// Assert:  `result.ok().map(|value| value.big_endian)` equals `Some` of the
///   array `0` through `31` in order by `assert_eq!`.
#[test]
fn solidity_uint256_holds_the_array_moved_from_the_params() {
    // Arrange
    let params =
        build_solidity_uint256_constructor_params(SolidityUint256ConstructorParamsOverrides {
            big_endian: Some([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ]),
        });

    // Act
    let result = SolidityUint256::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|value| value.big_endian),
        Some([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
    );
}

/// Contract: `SolidityLibraryEntries::try_new(params)`: empty `entries` →
///   `Err(SolidityLibraryEntriesTryNewErrorReturn::Empty)`.
/// Arrange: `build_solidity_library_entries_constructor_params` overriding
///   `entries` with the empty list.
/// Act:     `SolidityLibraryEntries::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityLibraryEntriesTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn solidity_library_entries_refuses_an_empty_list() {
    // Arrange
    let params = build_solidity_library_entries_constructor_params(
        SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(Vec::new()),
        },
    );

    // Act
    let result = SolidityLibraryEntries::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityLibraryEntriesTryNewErrorReturn::Empty)
    );
}

/// Contract: `SolidityLibraryEntries::try_new(params)`: the lowest index `j`
///   whose name equals the name of an entry at a lower index `i` →
///   `Err(DuplicateName { first_index: i, duplicate_index: j })`, judged by
///   name alone.
/// Arrange: `build_solidity_library_entries_constructor_params` overriding
///   `entries` with four `build_solidity_library_entry` values, each
///   overriding `name` with `build_solidity_constant_name` and `value` with
///   `SolidityConstantValue::Uint64`, named `X`, `Y`, `Y`, `X` with the
///   values `1`, `2`, `3`, `4`.
/// Act:     `SolidityLibraryEntries::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityLibraryEntriesTryNewErrorReturn::
///   DuplicateName { first_index: 1, duplicate_index: 2 })` by `assert_eq!`.
#[test]
fn solidity_library_entries_refuses_the_lowest_index_whose_name_repeats_an_earlier_name() {
    // Arrange
    let entry = |text: &str, value: u64| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Uint64(value)),
        })
    };
    let params = build_solidity_library_entries_constructor_params(
        SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(vec![
                entry("X", 1),
                entry("Y", 2),
                entry("Y", 3),
                entry("X", 4),
            ]),
        },
    );

    // Act
    let result = SolidityLibraryEntries::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityLibraryEntriesTryNewErrorReturn::DuplicateName {
            first_index: 1,
            duplicate_index: 2
        })
    );
}

/// Contract: `SolidityLibraryEntries::try_new(params)`: distinct names →
///   `Ok(SolidityLibraryEntries { entries })`, the vector moved from the
///   params in its order.
/// Arrange: `build_solidity_library_entries_constructor_params` overriding
///   `entries` with three `build_solidity_library_entry` values named
///   `GAMMA`, `ALPHA`, and `BETA`, in that unsorted order, with
///   `SolidityConstantValue::Uint64` values `3`, `1`, and `2`.
/// Act:     `SolidityLibraryEntries::try_new(params)`.
/// Assert:  `result.ok().map(|entries| entries.entries)` equals `Some` of
///   the three entries with name texts `GAMMA`, `ALPHA`, `BETA` and values
///   `Uint64(3)`, `Uint64(1)`, `Uint64(2)` in that order by `assert_eq!`.
#[test]
fn solidity_library_entries_admits_distinct_names_and_keeps_their_order() {
    // Arrange
    let entry = |text: &str, value: u64| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Uint64(value)),
        })
    };
    let params = build_solidity_library_entries_constructor_params(
        SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(vec![entry("GAMMA", 3), entry("ALPHA", 1), entry("BETA", 2)]),
        },
    );

    // Act
    let result = SolidityLibraryEntries::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|entries| {
            entries
                .entries
                .iter()
                .map(|entry| (entry.name.text.clone(), entry.value.clone()))
                .collect::<Vec<_>>()
        }),
        Some(vec![
            ("GAMMA".to_string(), SolidityConstantValue::Uint64(3)),
            ("ALPHA".to_string(), SolidityConstantValue::Uint64(1)),
            ("BETA".to_string(), SolidityConstantValue::Uint64(2)),
        ])
    );
}

/// Contract: `SolidityAbiType::try_new(params)`: one branch →
///   `Ok(SolidityAbiType { kind })`, the kind moved from the params.
/// Arrange: `build_solidity_abi_type_constructor_params` overriding `kind`
///   with `CanonicalFieldKind::Unsigned256`, which differs from the builder
///   default.
/// Act:     `SolidityAbiType::try_new(params)`.
/// Assert:  `result.ok().map(|abi| abi.kind)` equals
///   `Some(CanonicalFieldKind::Unsigned256)` by `assert_eq!`.
#[test]
fn solidity_abi_type_holds_the_kind_moved_from_the_params() {
    // Arrange
    let params =
        build_solidity_abi_type_constructor_params(SolidityAbiTypeConstructorParamsOverrides {
            kind: Some(CanonicalFieldKind::Unsigned256),
        });

    // Act
    let result = SolidityAbiType::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|abi| abi.kind),
        Some(CanonicalFieldKind::Unsigned256)
    );
}

/// Contract: `SolidityAbiType::as_str`: `FixedBytes32` → `bytes32`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::FixedBytes32`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `bytes32` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_fixed_bytes_32_as_bytes32() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::FixedBytes32),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "bytes32");
}

/// Contract: `SolidityAbiType::as_str`: `Unsigned16` → `uint16`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned16`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `uint16` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_unsigned_16_as_uint16() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned16),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "uint16");
}

/// Contract: `SolidityAbiType::as_str`: `Unsigned32` → `uint32`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned32`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `uint32` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_unsigned_32_as_uint32() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned32),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "uint32");
}

/// Contract: `SolidityAbiType::as_str`: `Unsigned64` → `uint64`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned64`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `uint64` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_unsigned_64_as_uint64() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned64),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "uint64");
}

/// Contract: `SolidityAbiType::as_str`: `Text` → `string`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Text`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `string` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_text_as_string() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Text),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "string");
}

/// Contract: `SolidityAbiType::as_str`: `Bytes` → `bytes`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Bytes`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `bytes` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_bytes_as_bytes() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Bytes),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "bytes");
}

/// Contract: `SolidityAbiType::as_str`: `FixedBytes20` → `bytes20`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::FixedBytes20`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `bytes20` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_fixed_bytes_20_as_bytes20_never_address() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::FixedBytes20),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "bytes20");
}

/// Contract: `SolidityAbiType::as_str`: `Unsigned256` → `uint256`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned256`.
/// Act:     `abi.as_str()`.
/// Assert:  the result equals `uint256` by `assert_eq!`.
#[test]
fn solidity_abi_type_names_unsigned_256_as_uint256() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned256),
    });

    // Act
    let name = abi.as_str();

    // Assert
    assert_eq!(name, "uint256");
}

/// Contract: `SolidityAbiType::decodes_into_memory`: `Text` → `true`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Text`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `true` by `assert!`.
#[test]
fn solidity_abi_type_decodes_text_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Text),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(memory);
}

/// Contract: `SolidityAbiType::decodes_into_memory`: `Bytes` → `true`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Bytes`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `true` by `assert!`.
#[test]
fn solidity_abi_type_decodes_bytes_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Bytes),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(memory);
}

/// Contract: `SolidityAbiType::decodes_into_memory`: every kind other than
///   `Text` and `Bytes` → `false`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::FixedBytes32`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `false` by `assert!`.
#[test]
fn solidity_abi_type_does_not_decode_fixed_bytes_32_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::FixedBytes32),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(!memory);
}

/// Contract: `SolidityAbiType::decodes_into_memory`: `Unsigned16` → `false`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned16`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `false` by `assert!`.
#[test]
fn solidity_abi_type_does_not_decode_unsigned_16_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned16),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(!memory);
}

/// Contract: `SolidityAbiType::decodes_into_memory`: `Unsigned32` → `false`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned32`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `false` by `assert!`.
#[test]
fn solidity_abi_type_does_not_decode_unsigned_32_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned32),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(!memory);
}

/// Contract: `SolidityAbiType::decodes_into_memory`: `Unsigned64` → `false`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned64`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `false` by `assert!`.
#[test]
fn solidity_abi_type_does_not_decode_unsigned_64_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned64),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(!memory);
}

/// Contract: `SolidityAbiType::decodes_into_memory`: `FixedBytes20` →
///   `false`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::FixedBytes20`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `false` by `assert!`.
#[test]
fn solidity_abi_type_does_not_decode_fixed_bytes_20_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::FixedBytes20),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(!memory);
}

/// Contract: `SolidityAbiType::decodes_into_memory`: `Unsigned256` →
///   `false`.
/// Arrange: `build_solidity_abi_type` overriding `kind` with
///   `CanonicalFieldKind::Unsigned256`.
/// Act:     `abi.decodes_into_memory()`.
/// Assert:  the result is `false` by `assert!`.
#[test]
fn solidity_abi_type_does_not_decode_unsigned_256_into_memory() {
    // Arrange
    let abi = build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides {
        kind: Some(CanonicalFieldKind::Unsigned256),
    });

    // Act
    let memory = abi.decodes_into_memory();

    // Assert
    assert!(!memory);
}

/// Contract: `SolidityRecordMembers::try_new(params)`: fewer than two
///   members → `Err(TooFew { actual })` with the length.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with the empty list.
/// Act:     `SolidityRecordMembers::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordMembersTryNewErrorReturn::
///   TooFew { actual: 0 })` by `assert_eq!`.
#[test]
fn solidity_record_members_refuse_an_empty_list_as_too_few() {
    // Arrange
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(Vec::new()),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 0 })
    );
}

/// Contract: `SolidityRecordMembers::try_new(params)`: fewer than two
///   members → `Err(TooFew { actual })` with the length.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with one `build_solidity_record_member`.
/// Act:     `SolidityRecordMembers::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordMembersTryNewErrorReturn::
///   TooFew { actual: 1 })` by `assert_eq!`.
#[test]
fn solidity_record_members_refuse_a_single_member_as_too_few() {
    // Arrange
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![build_solidity_record_member(Default::default())]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 1 })
    );
}

/// Contract: `SolidityRecordMembers::try_new(params)`: the count is decided
///   before the names.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with one `build_solidity_record_member` whose `name` is
///   `line`.
/// Act:     `SolidityRecordMembers::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordMembersTryNewErrorReturn::
///   TooFew { actual: 1 })` by `assert_eq!`.
#[test]
fn solidity_record_members_refuse_a_single_reserved_name_as_too_few() {
    // Arrange
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![build_solidity_record_member(
                SolidityRecordMemberOverrides {
                    name: Some(build_solidity_member_name(
                        SolidityMemberNameConstructorParamsOverrides {
                            text: Some("line".to_string()),
                        },
                    )),
                    ..Default::default()
                },
            )]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 1 })
    );
}

/// Contract: `SolidityRecordMembers::try_new(params)`: the lowest index `k`
///   whose name text equals `line` → `Err(ReservedName { index: k })`.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with four members named `alpha`, `line`, `beta`, `line`.
/// Act:     `SolidityRecordMembers::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordMembersTryNewErrorReturn::
///   ReservedName { index: 1 })` by `assert_eq!`.
#[test]
fn solidity_record_members_refuse_the_lowest_index_named_line() {
    // Arrange
    let member = |text: &str| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("alpha"),
                member("line"),
                member("beta"),
                member("line"),
            ]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: 1 })
    );
}

/// Contract: `SolidityRecordMembers::try_new(params)`: the reserved check is
///   decided before the duplicate check.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with two members both named `line`.
/// Act:     `SolidityRecordMembers::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordMembersTryNewErrorReturn::
///   ReservedName { index: 0 })` by `assert_eq!`.
#[test]
fn solidity_record_members_refuse_a_reserved_name_before_a_duplicate() {
    // Arrange
    let member = || {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some("line".to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![member(), member()]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: 0 })
    );
}

/// Contract: `SolidityRecordMembers::try_new(params)`: the lowest index `j`
///   whose name equals the name of a member at a lower index `i` →
///   `Err(DuplicateName { first_index: i, duplicate_index: j })`, judged by
///   name alone.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with four `build_solidity_record_member` values named
///   `alpha`, `beta`, `beta`, `alpha` with the kinds `Text`, `Bytes`,
///   `Unsigned16`, `Unsigned64`.
/// Act:     `SolidityRecordMembers::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordMembersTryNewErrorReturn::
///   DuplicateName { first_index: 1, duplicate_index: 2 })` by `assert_eq!`.
#[test]
fn solidity_record_members_refuse_the_lowest_index_whose_name_repeats_an_earlier_name() {
    // Arrange
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("alpha", CanonicalFieldKind::Text),
                member("beta", CanonicalFieldKind::Bytes),
                member("beta", CanonicalFieldKind::Unsigned16),
                member("alpha", CanonicalFieldKind::Unsigned64),
            ]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityRecordMembersTryNewErrorReturn::DuplicateName {
            first_index: 1,
            duplicate_index: 2
        })
    );
}

/// Contract: `SolidityRecordMembers::try_new(params)`: members that are at
///   least two, unique, and not named `line` → `Ok(SolidityRecordMembers {
///   members })`, the vector moved from the params in its order.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with three `build_solidity_record_member` values named
///   `zeta`, `lineCount`, and `alpha`, in that unsorted order, with the
///   kinds `Text`, `Unsigned16`, and `Bytes`.
/// Act:     `SolidityRecordMembers::try_new(params)`.
/// Assert:  `result.ok().map(|members| members.members)` equals `Some` of
///   the three members with name texts `zeta`, `lineCount`, `alpha` and
///   kinds `Text`, `Unsigned16`, `Bytes` in that order by `assert_eq!`.
#[test]
fn solidity_record_members_admit_unique_names_that_contain_line_and_keep_their_order() {
    // Arrange
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("zeta", CanonicalFieldKind::Text),
                member("lineCount", CanonicalFieldKind::Unsigned16),
                member("alpha", CanonicalFieldKind::Bytes),
            ]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|members| {
            members
                .members
                .iter()
                .map(|member| (member.name.text.clone(), member.kind))
                .collect::<Vec<_>>()
        }),
        Some(vec![
            ("zeta".to_string(), CanonicalFieldKind::Text),
            ("lineCount".to_string(), CanonicalFieldKind::Unsigned16),
            ("alpha".to_string(), CanonicalFieldKind::Bytes),
        ])
    );
}

/// Contract: `SolidityRecordDeclarations::try_new(params)`: empty
///   `declarations` → `Err(SolidityRecordDeclarationsTryNewErrorReturn::
///   Empty)`.
/// Arrange: `build_solidity_record_declarations_constructor_params`
///   overriding `declarations` with the empty list.
/// Act:     `SolidityRecordDeclarations::try_new(params)`.
/// Assert:  `result.err()` equals
///   `Some(SolidityRecordDeclarationsTryNewErrorReturn::Empty)` by
///   `assert_eq!`.
#[test]
fn solidity_record_declarations_refuse_an_empty_list() {
    // Arrange
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(Vec::new()),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(SolidityRecordDeclarationsTryNewErrorReturn::Empty)
    );
}

/// Contract: `SolidityRecordDeclarations::try_new(params)`: the lowest
///   index `j` whose `struct_name` equals that of a declaration at a lower
///   index `i` → `Err(DuplicateStructName { first_index: i, duplicate_index:
///   j })`.
/// Arrange: `build_solidity_record_declarations_constructor_params`
///   overriding `declarations` with four `build_solidity_record_declaration`
///   values, each overriding `struct_name` with `build_solidity_type_name`,
///   `decode_function_name` with `build_solidity_member_name`, and
///   `path_constant_name` with `build_solidity_constant_name`, the struct
///   names `First`, `Second`, `Second`, `First`, the decode function names
///   `decodeA`, `decodeB`, `decodeC`, `decodeD`, and the path constant names
///   `PATH_A`, `PATH_B`, `PATH_C`, `PATH_D`.
/// Act:     `SolidityRecordDeclarations::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateStructName { first_index: 1, duplicate_index: 2 })` by
///   `assert_eq!`.
#[test]
fn solidity_record_declarations_refuse_the_lowest_index_whose_struct_name_repeats() {
    // Arrange
    let declaration = |struct_name: &str, decode_function_name: &str, path_constant_name: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode_function_name.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant_name.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![
                declaration("First", "decodeA", "PATH_A"),
                declaration("Second", "decodeB", "PATH_B"),
                declaration("Second", "decodeC", "PATH_C"),
                declaration("First", "decodeD", "PATH_D"),
            ]),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                first_index: 1,
                duplicate_index: 2
            }
        )
    );
}

/// Contract: `SolidityRecordDeclarations::try_new(params)`: the lowest
///   index `j` whose `decode_function_name` equals that of a declaration at
///   a lower index `i` → `Err(DuplicateDecodeFunctionName { first_index: i,
///   duplicate_index: j })`.
/// Arrange: `build_solidity_record_declarations_constructor_params`
///   overriding `declarations` with three `build_solidity_record_declaration`
///   values, each overriding `struct_name`, `decode_function_name`, and
///   `path_constant_name` through their name builders, the struct names
///   `Alpha`, `Beta`, `Gamma`, the decode function names `decodeA`,
///   `decodeA`, `decodeB`, and the path constant names `PATH_A`, `PATH_B`,
///   `PATH_C`.
/// Act:     `SolidityRecordDeclarations::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateDecodeFunctionName { first_index: 0, duplicate_index: 1 })` by
///   `assert_eq!`.
#[test]
fn solidity_record_declarations_refuse_the_lowest_index_whose_decode_function_name_repeats() {
    // Arrange
    let declaration = |struct_name: &str, decode_function_name: &str, path_constant_name: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode_function_name.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant_name.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![
                declaration("Alpha", "decodeA", "PATH_A"),
                declaration("Beta", "decodeA", "PATH_B"),
                declaration("Gamma", "decodeB", "PATH_C"),
            ]),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                first_index: 0,
                duplicate_index: 1
            }
        )
    );
}

/// Contract: `SolidityRecordDeclarations::try_new(params)`: the lowest
///   index `j` whose `path_constant_name` equals that of a declaration at a
///   lower index `i` → `Err(DuplicatePathConstantName { first_index: i,
///   duplicate_index: j })`.
/// Arrange: `build_solidity_record_declarations_constructor_params`
///   overriding `declarations` with four `build_solidity_record_declaration`
///   values, each overriding `struct_name`, `decode_function_name`, and
///   `path_constant_name` through their name builders, the struct names
///   `Alpha`, `Beta`, `Gamma`, `Delta`, the decode function names `decodeA`,
///   `decodeB`, `decodeC`, `decodeD`, and the path constant names `PATH_A`,
///   `PATH_B`, `PATH_C`, `PATH_B`.
/// Act:     `SolidityRecordDeclarations::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicatePathConstantName { first_index: 1, duplicate_index: 3 })` by
///   `assert_eq!`.
#[test]
fn solidity_record_declarations_refuse_the_lowest_index_whose_path_constant_name_repeats() {
    // Arrange
    let declaration = |struct_name: &str, decode_function_name: &str, path_constant_name: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode_function_name.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant_name.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![
                declaration("Alpha", "decodeA", "PATH_A"),
                declaration("Beta", "decodeB", "PATH_B"),
                declaration("Gamma", "decodeC", "PATH_C"),
                declaration("Delta", "decodeD", "PATH_B"),
            ]),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName {
                first_index: 1,
                duplicate_index: 3
            }
        )
    );
}

/// Contract: `SolidityRecordDeclarations::try_new(params)`: the struct-name
///   check is decided before the decode-function-name check.
/// Arrange: `build_solidity_record_declarations_constructor_params`
///   overriding `declarations` with three `build_solidity_record_declaration`
///   values, each overriding `struct_name`, `decode_function_name`, and
///   `path_constant_name` through their name builders, the struct names
///   `Alpha`, `Beta`, `Beta`, the decode function names `decodeA`,
///   `decodeA`, `decodeB`, and the path constant names `PATH_A`, `PATH_B`,
///   `PATH_C`.
/// Act:     `SolidityRecordDeclarations::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateStructName { first_index: 1, duplicate_index: 2 })` by
///   `assert_eq!`.
#[test]
fn solidity_record_declarations_refuse_a_repeated_struct_name_before_a_repeated_decode_function_name()
 {
    // Arrange
    let declaration = |struct_name: &str, decode_function_name: &str, path_constant_name: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode_function_name.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant_name.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![
                declaration("Alpha", "decodeA", "PATH_A"),
                declaration("Beta", "decodeA", "PATH_B"),
                declaration("Beta", "decodeB", "PATH_C"),
            ]),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                first_index: 1,
                duplicate_index: 2
            }
        )
    );
}

/// Contract: `SolidityRecordDeclarations::try_new(params)`: the
///   decode-function-name check is decided before the path-constant-name
///   check.
/// Arrange: `build_solidity_record_declarations_constructor_params`
///   overriding `declarations` with three `build_solidity_record_declaration`
///   values, each overriding `struct_name`, `decode_function_name`, and
///   `path_constant_name` through their name builders, the struct names
///   `Alpha`, `Beta`, `Gamma`, the decode function names `decodeA`,
///   `decodeB`, `decodeB`, and the path constant names `PATH_A`, `PATH_A`,
///   `PATH_B`.
/// Act:     `SolidityRecordDeclarations::try_new(params)`.
/// Assert:  `result.err()` equals `Some(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateDecodeFunctionName { first_index: 1, duplicate_index: 2 })` by
///   `assert_eq!`.
#[test]
fn solidity_record_declarations_refuse_a_repeated_decode_function_name_before_a_repeated_path_constant_name()
 {
    // Arrange
    let declaration = |struct_name: &str, decode_function_name: &str, path_constant_name: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode_function_name.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant_name.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![
                declaration("Alpha", "decodeA", "PATH_A"),
                declaration("Beta", "decodeB", "PATH_A"),
                declaration("Gamma", "decodeB", "PATH_B"),
            ]),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                first_index: 1,
                duplicate_index: 2
            }
        )
    );
}

/// Contract: `SolidityRecordDeclarations::try_new(params)`: unique names →
///   `Ok(SolidityRecordDeclarations { declarations })`, the vector moved
///   from the params in its order.
/// Arrange: `build_solidity_record_declarations_constructor_params`
///   overriding `declarations` with three `build_solidity_record_declaration`
///   values, each overriding `struct_name`, `decode_function_name`, and
///   `path_constant_name` through their name builders, the struct names
///   `Gamma`, `Alpha`, `Beta`, in that unsorted order, the decode function
///   names `decodeC`, `decodeA`, `decodeB`, and the path constant names
///   `PATH_C`, `PATH_A`, `PATH_B`.
/// Act:     `SolidityRecordDeclarations::try_new(params)`.
/// Assert:  `result.ok().map(|declarations| declarations.declarations)`
///   equals `Some` of the three declarations whose struct name texts are
///   `Gamma`, `Alpha`, `Beta` in that order by `assert_eq!`.
#[test]
fn solidity_record_declarations_admit_unique_names_and_keep_their_order() {
    // Arrange
    let declaration = |struct_name: &str, decode_function_name: &str, path_constant_name: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode_function_name.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant_name.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![
                declaration("Gamma", "decodeC", "PATH_C"),
                declaration("Alpha", "decodeA", "PATH_A"),
                declaration("Beta", "decodeB", "PATH_B"),
            ]),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result.ok().map(|declarations| {
            declarations
                .declarations
                .iter()
                .map(|declaration| declaration.struct_name.text.clone())
                .collect::<Vec<_>>()
        }),
        Some(vec![
            "Gamma".to_string(),
            "Alpha".to_string(),
            "Beta".to_string(),
        ])
    );
}

/// Contract: `SoliditySourceText::as_str` returns the text.
/// Arrange: `build_solidity_source_text` overriding `text` with `first
///   line`, a newline, `second line`, and a newline, which differs from the
///   builder default.
/// Act:     `source.as_str()`.
/// Assert:  the result equals that literal by `assert_eq!`.
#[test]
fn solidity_source_text_returns_its_text() {
    // Arrange
    let source = build_solidity_source_text(SoliditySourceTextOverrides {
        text: Some("first line\nsecond line\n".to_string()),
    });

    // Act
    let text = source.as_str();

    // Assert
    assert_eq!(text, "first line\nsecond line\n");
}

/// Contract: `render` in the constants branch opens with the SPDX license
///   line, the exact pragma, and an empty line before the library
///   declaration.
/// Arrange: `build_render_deps(Default::default())`; `build_render_params`
///   overriding `license` with `build_spdx_license_identifier` over
///   `GPL-3.0+` and `compiler_version` with `build_solidity_compiler_version`
///   over `major` `7`, `minor` `28`, and `patch` `315`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry(Default::default())`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` begins with `// SPDX-License-Identifier:
///   GPL-3.0+`, a newline, `pragma solidity 7.28.315;`, a newline, an empty
///   line, and `library `, the three release numbers being different so a
///   swap fails.
#[test]
fn render_opens_with_the_license_line_the_exact_pragma_and_an_empty_line() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("GPL-3.0+".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(7),
                minor: Some(28),
                patch: Some(315),
            },
        )),
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(Default::default()),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().starts_with(
        "// SPDX-License-Identifier: GPL-3.0+\npragma solidity 7.28.315;\n\nlibrary "
    ));
}

/// Contract: `render` declares the library as `library `, the library
///   name's text, and ` {` with a newline.
/// Arrange: `build_render_deps(Default::default())`; `build_render_params`
///   overriding `license` with `build_spdx_license_identifier` over
///   `GPL-3.0+` and `compiler_version` with `build_solidity_compiler_version`
///   over `7`, `28`, and `315`; `build_render_payload` overriding
///   `library_name` with `build_solidity_library_name` over `ReferenceLib`
///   and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry(Default::default())`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains `library ReferenceLib {` followed by
///   a newline.
#[test]
fn render_declares_the_library_by_name() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("GPL-3.0+".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(7),
                minor: Some(28),
                patch: Some(315),
            },
        )),
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(Default::default()),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().contains("library ReferenceLib {\n"));
}

/// Contract: `render` closes the text with `}` and one newline.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry(Default::default())`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` ends with `}` and a newline and does not end
///   with two newlines.
#[test]
fn render_closes_the_library_with_one_final_newline() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(Default::default()),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().ends_with("}\n"));
    assert!(!success.source.as_str().ends_with("}\n\n"));
}

/// Contract: `render` in the constants branch produces the header, one line
///   per entry in payload order, and the closing brace, each line ending in
///   a newline.
/// Arrange: `build_render_deps(Default::default())`; `build_render_params`
///   overriding `license` with `build_spdx_license_identifier` over
///   `GPL-3.0+` and `compiler_version` with `build_solidity_compiler_version`
///   over `7`, `28`, and `315`; `build_render_payload` overriding
///   `library_name` with `build_solidity_library_name` over `ReferenceLib`
///   and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` are
///   `build_solidity_library_entry` values named `BYTES_VALUE` holding
///   `Bytes` over `0x00` and `0xab`, `WORD_VALUE` holding `Uint256` over a
///   `build_solidity_uint256` whose `big_endian` has the byte at each index
///   equal to the index, `LARGE_VALUE` holding `Uint64` over `u64::MAX`,
///   `SMALL_VALUE` holding `Uint16` over `u16::MAX`, `FLAG_VALUE` holding
///   `Bool(true)`, and `LABEL_VALUE` holding `String` over a
///   `build_solidity_string_literal` whose `text` is `label`, each name
///   through `build_solidity_constant_name`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `result.ok()` equals `Some` of `build_render_success_return`
///   whose `source` is `build_solidity_source_text` over the reference text,
///   a literal the block assembles from the spec's header, entry, and
///   closing lines, by `assert_eq!`.
#[test]
fn render_produces_the_exact_source_of_a_reference_library() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("GPL-3.0+".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(7),
                minor: Some(28),
                patch: Some(315),
            },
        )),
    });
    let name = |text: &str| {
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some(text.to_string()),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("BYTES_VALUE")),
                        value: Some(SolidityConstantValue::Bytes(vec![0x00, 0xab])),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("WORD_VALUE")),
                        value: Some(SolidityConstantValue::Uint256(build_solidity_uint256(
                            SolidityUint256ConstructorParamsOverrides {
                                big_endian: Some([
                                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                                    18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                                ]),
                            },
                        ))),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("LARGE_VALUE")),
                        value: Some(SolidityConstantValue::Uint64(u64::MAX)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("SMALL_VALUE")),
                        value: Some(SolidityConstantValue::Uint16(u16::MAX)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("FLAG_VALUE")),
                        value: Some(SolidityConstantValue::Bool(true)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("LABEL_VALUE")),
                        value: Some(SolidityConstantValue::String(
                            build_solidity_string_literal(
                                SolidityStringLiteralConstructorParamsOverrides {
                                    text: Some("label".to_string()),
                                },
                            ),
                        )),
                    }),
                ]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    assert_eq!(
        result.ok(),
        Some(build_render_success_return(RenderSuccessReturnOverrides {
            source: Some(build_solidity_source_text(SoliditySourceTextOverrides {
                text: Some(
                    concat!(
                        "// SPDX-License-Identifier: GPL-3.0+\n",
                        "pragma solidity 7.28.315;\n",
                        "\n",
                        "library ReferenceLib {\n",
                        "    bytes internal constant BYTES_VALUE = hex\"00ab\";\n",
                        "    uint256 internal constant WORD_VALUE = 0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f;\n",
                        "    uint64 internal constant LARGE_VALUE = 18446744073709551615;\n",
                        "    uint16 internal constant SMALL_VALUE = 65535;\n",
                        "    bool internal constant FLAG_VALUE = true;\n",
                        "    string internal constant LABEL_VALUE = \"label\";\n",
                        "}\n"
                    )
                    .to_string()
                ),
            })),
        }))
    );
}

/// Contract: `Bytes(bytes)` renders as `bytes` with `hex"` and two
///   lowercase hex digits per byte and `"`.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry` named `BYTES_VALUE` holding `Bytes`
///   over `0x00`, `0x0a`, `0xab`, and `0xff`, a byte with a zero high nibble
///   beside bytes whose digits are letters.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    bytes internal
///   constant BYTES_VALUE = hex"000aabff";` and a newline.
#[test]
fn render_writes_a_byte_string_as_a_lowercase_hex_literal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("BYTES_VALUE".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bytes(vec![0x00, 0x0a, 0xab, 0xff])),
                    },
                )]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    bytes internal constant BYTES_VALUE = hex\"000aabff\";\n")
    );
}

/// Contract: an empty byte string renders as `hex""`.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry` named `BYTES_VALUE` holding `Bytes`
///   over an empty vector.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    bytes internal
///   constant BYTES_VALUE = hex"";` and a newline.
#[test]
fn render_writes_an_empty_byte_string_as_an_empty_hex_literal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("BYTES_VALUE".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bytes(Vec::new())),
                    },
                )]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    bytes internal constant BYTES_VALUE = hex\"\";\n")
    );
}

/// Contract: `Uint256(value)` renders as `uint256` with `0x` and two
///   lowercase hex digits per byte of `big_endian` in array order.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry` named `WORD_VALUE` holding `Uint256`
///   over a `build_solidity_uint256` whose `big_endian` has the byte at
///   each index equal to the index.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    uint256 internal
///   constant WORD_VALUE = `, `0x` and the lowercase hex digit pair of each
///   byte in array order, `;`, and a newline.
#[test]
fn render_writes_a_256_bit_value_as_big_endian_hex_digits() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("WORD_VALUE".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint256(build_solidity_uint256(
                            SolidityUint256ConstructorParamsOverrides {
                                big_endian: Some([
                                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                                    18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                                ]),
                            },
                        ))),
                    },
                )]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().contains(
        "    uint256 internal constant WORD_VALUE = 0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f;\n"
    ));
}

/// Contract: `Uint64(value)` renders as `uint64` with the value in decimal.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry` named `LARGE_VALUE` holding `Uint64`
///   over `u64::MAX`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    uint64 internal
///   constant LARGE_VALUE = 18446744073709551615;` and a newline.
#[test]
fn render_writes_a_64_bit_value_in_decimal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("LARGE_VALUE".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint64(u64::MAX)),
                    },
                )]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    uint64 internal constant LARGE_VALUE = 18446744073709551615;\n")
    );
}

/// Contract: `Uint16(value)` renders as `uint16` with the value in decimal.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry` named `SMALL_VALUE` holding `Uint16`
///   over `u16::MAX`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    uint16 internal
///   constant SMALL_VALUE = 65535;` and a newline.
#[test]
fn render_writes_a_16_bit_value_in_decimal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("SMALL_VALUE".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint16(u16::MAX)),
                    },
                )]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    uint16 internal constant SMALL_VALUE = 65535;\n")
    );
}

/// Contract: `Bool(value)` renders as `bool` with `true` or `false`.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` are
///   `build_solidity_library_entry` values named `FLAG_ON` holding
///   `Bool(true)` and `FLAG_OFF` holding `Bool(false)`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the lines `    bool internal
///   constant FLAG_ON = true;` and `    bool internal constant FLAG_OFF =
///   false;`, each followed by a newline.
#[test]
fn render_writes_booleans_as_true_and_false() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let name = |text: &str| {
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some(text.to_string()),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("FLAG_ON")),
                        value: Some(SolidityConstantValue::Bool(true)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name("FLAG_OFF")),
                        value: Some(SolidityConstantValue::Bool(false)),
                    }),
                ]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    bool internal constant FLAG_ON = true;\n")
    );
    assert!(
        success
            .source
            .as_str()
            .contains("    bool internal constant FLAG_OFF = false;\n")
    );
}

/// Contract: `String(literal)` renders as `string` with the literal's text
///   between double quotes.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry` named `LABEL_VALUE` holding `String`
///   over a `build_solidity_string_literal` whose `text` is `a ~b`, a
///   space and a tilde inside the text.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    string internal
///   constant LABEL_VALUE = "a ~b";` and a newline.
#[test]
fn render_writes_a_string_between_double_quotes() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("LABEL_VALUE".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::String(
                            build_solidity_string_literal(
                                SolidityStringLiteralConstructorParamsOverrides {
                                    text: Some("a ~b".to_string()),
                                },
                            ),
                        )),
                    },
                )]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    string internal constant LABEL_VALUE = \"a ~b\";\n")
    );
}

/// Contract: `render` writes entries one per line in payload order.
/// Arrange: `build_render_deps(Default::default())`; `build_render_params`
///   overriding `license` with `build_spdx_license_identifier` over
///   `GPL-3.0+` and `compiler_version` with `build_solidity_compiler_version`
///   over `7`, `28`, and `315`; `build_render_payload` overriding
///   `library_name` with `build_solidity_library_name` over `ReferenceLib`
///   and `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` are
///   `build_solidity_library_entry` values named `GAMMA`, `ALPHA`, and
///   `BETA`, in that unsorted order, holding `Uint64` over `3`, `1`, and
///   `2`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `result.ok()` equals `Some` of `build_render_success_return`
///   whose `source` is `build_solidity_source_text` over the header, the
///   three entry lines in `GAMMA`, `ALPHA`, `BETA` order, and the closing
///   brace, stated as one literal, by `assert_eq!`.
#[test]
fn render_writes_entries_in_payload_order() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("GPL-3.0+".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(7),
                minor: Some(28),
                patch: Some(315),
            },
        )),
    });
    let entry = |text: &str, value: u64| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Uint64(value)),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![entry("GAMMA", 3), entry("ALPHA", 1), entry("BETA", 2)]),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    assert_eq!(
        result.ok(),
        Some(build_render_success_return(RenderSuccessReturnOverrides {
            source: Some(build_solidity_source_text(SoliditySourceTextOverrides {
                text: Some(
                    concat!(
                        "// SPDX-License-Identifier: GPL-3.0+\n",
                        "pragma solidity 7.28.315;\n",
                        "\n",
                        "library ReferenceLib {\n",
                        "    uint64 internal constant GAMMA = 3;\n",
                        "    uint64 internal constant ALPHA = 1;\n",
                        "    uint64 internal constant BETA = 2;\n",
                        "}\n"
                    )
                    .to_string()
                ),
            })),
        }))
    );
}
