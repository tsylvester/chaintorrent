#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::CanonicalFieldKind;

use super::interface::{
    SolidityAbiType, SolidityAbiTypeTryNewErrorReturn, SolidityConstantName,
    SolidityConstantNameTryNewErrorReturn, SolidityConstantValue, SolidityLibraryBody,
    SolidityLibraryEntries, SolidityLibraryEntriesTryNewErrorReturn, SolidityLibraryName,
    SolidityLibraryNameTryNewErrorReturn, SolidityMemberName, SolidityMemberNameTryNewErrorReturn,
    SolidityRecordDeclarations, SolidityRecordDeclarationsTryNewErrorReturn, SolidityRecordMembers,
    SolidityRecordMembersTryNewErrorReturn, SolidityStringLiteral,
    SolidityStringLiteralTryNewErrorReturn, SolidityTypeName, SolidityTypeNameTryNewErrorReturn,
    SolidityUint256, SpdxLicenseIdentifier, SpdxLicenseIdentifierTryNewErrorReturn,
};
use super::mock::{
    RenderParamsOverrides, RenderPayloadOverrides, RenderSuccessReturnOverrides,
    SolidityAbiTypeConstructorParamsOverrides, SolidityCompilerVersionOverrides,
    SolidityConstantNameConstructorParamsOverrides,
    SolidityLibraryEntriesConstructorParamsOverrides, SolidityLibraryEntryOverrides,
    SolidityLibraryNameConstructorParamsOverrides, SolidityMemberNameConstructorParamsOverrides,
    SolidityRecordDeclarationOverrides, SolidityRecordDeclarationsConstructorParamsOverrides,
    SolidityRecordMemberOverrides, SolidityRecordMembersConstructorParamsOverrides,
    SoliditySourceTextOverrides, SolidityStringLiteralConstructorParamsOverrides,
    SolidityTypeNameConstructorParamsOverrides, SolidityUint256ConstructorParamsOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, build_render_deps, build_render_params,
    build_render_payload, build_render_success_return, build_solidity_abi_type_constructor_params,
    build_solidity_compiler_version, build_solidity_constant_name,
    build_solidity_constant_name_constructor_params,
    build_solidity_library_entries_constructor_params, build_solidity_library_entry,
    build_solidity_library_name, build_solidity_library_name_constructor_params,
    build_solidity_member_name, build_solidity_member_name_constructor_params,
    build_solidity_record_declaration, build_solidity_record_declarations_constructor_params,
    build_solidity_record_member, build_solidity_record_members_constructor_params,
    build_solidity_source_text, build_solidity_string_literal,
    build_solidity_string_literal_constructor_params, build_solidity_type_name,
    build_solidity_type_name_constructor_params, build_solidity_uint256_constructor_params,
    build_spdx_license_identifier, build_spdx_license_identifier_constructor_params,
};
use super::render;

/// Contract: empty text returns `Err(Empty)`.
/// Arrange: `build_solidity_library_name_constructor_params` overriding `text`
///   with the empty string.
/// Act:    `SolidityLibraryName::try_new` over it.
/// Assert: the result equals `Err(SolidityLibraryNameTryNewErrorReturn::Empty)`.
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
    assert_eq!(result, Err(SolidityLibraryNameTryNewErrorReturn::Empty));
}
// solidity_library_name_refuses_empty_text

/// Contract: a leading byte outside `b'A'..=b'Z'` returns
///   `LeadingCharacter { byte }`, decided before the later bytes.
/// Arrange: params overriding `text` with a lowercase `a` followed by `-`,
///   a leading byte the later set admits and a later byte it refuses.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte: b'a' })`.
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
        result,
        Err(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte: b'a' })
    );
}
// solidity_library_name_refuses_a_leading_byte_outside_uppercase_letters

/// Contract: the lowest later index whose byte is outside `b'A'..=b'Z'`,
///   `b'a'..=b'z'`, and `b'0'..=b'9'` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `Ab_c-d`, whose `_` at index `2` is
///   refused here and admitted by a constant name, and whose `-` at index `4`
///   is a later refusal.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityLibraryNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'_' })`.
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
        result,
        Err(SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        })
    );
}
// solidity_library_name_refuses_the_lowest_later_byte_outside_letters_and_digits

/// Contract: a text whose leading byte is in `b'A'..=b'Z'` and whose later
///   bytes are in `b'A'..=b'Z'`, `b'a'..=b'z'`, or `b'0'..=b'9'` returns `Ok`
///   holding the text moved from the params.
/// Arrange: params overriding `text` with `AZaz09`, the inclusive ends of
///   every range.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` equals `AZaz09`.
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
    let Ok(name) = result else {
        panic!("the text admits")
    };
    assert_eq!(name.text, "AZaz09");
}
// solidity_library_name_admits_every_boundary_byte_of_its_sets

/// Contract: empty text returns `Err(Empty)`.
/// Arrange: `build_solidity_constant_name_constructor_params` overriding
///   `text` with the empty string.
/// Act:    `SolidityConstantName::try_new` over it.
/// Assert: the result equals `Err(SolidityConstantNameTryNewErrorReturn::Empty)`.
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
    assert_eq!(result, Err(SolidityConstantNameTryNewErrorReturn::Empty));
}
// solidity_constant_name_refuses_empty_text

/// Contract: a leading byte outside `b'A'..=b'Z'` returns
///   `LeadingCharacter { byte }`, decided before the later bytes.
/// Arrange: params overriding `text` with `_a`, a leading `_` the later set
///   admits and a later lowercase byte it refuses.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: b'_' })`.
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
        result,
        Err(SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: b'_' })
    );
}
// solidity_constant_name_refuses_a_leading_byte_outside_uppercase_letters

/// Contract: the lowest later index whose byte is outside `b'A'..=b'Z'`,
///   `b'0'..=b'9'`, and `b'_'` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `A_b-`, an admitted `_` at index `1`,
///   a refused lowercase `b` at index `2`, and a later refused `-`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityConstantNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'b' })`.
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
        result,
        Err(SolidityConstantNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'b'
        })
    );
}
// solidity_constant_name_refuses_the_lowest_later_byte_outside_uppercase_digits_and_underscore

/// Contract: a text within the leading and later sets returns `Ok` holding
///   the text moved from the params.
/// Arrange: params overriding `text` with `AZ_09`, the inclusive ends of every
///   range and the underscore.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` equals `AZ_09`.
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
    let Ok(name) = result else {
        panic!("the text admits")
    };
    assert_eq!(name.text, "AZ_09");
}
// solidity_constant_name_admits_every_boundary_byte_of_its_sets

/// Contract: empty text returns `Err(Empty)`.
/// Arrange: `build_solidity_type_name_constructor_params` overriding `text`
///   with the empty string.
/// Act:    `SolidityTypeName::try_new` over it.
/// Assert: the result equals `Err(SolidityTypeNameTryNewErrorReturn::Empty)`.
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
    assert_eq!(result, Err(SolidityTypeNameTryNewErrorReturn::Empty));
}
// solidity_type_name_refuses_empty_text

/// Contract: a leading byte outside `b'A'..=b'Z'` returns
///   `LeadingCharacter { byte }`, decided before the later bytes.
/// Arrange: params overriding `text` with a lowercase `a` followed by `-`.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'a' })`.
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
        result,
        Err(SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'a' })
    );
}
// solidity_type_name_refuses_a_leading_byte_outside_uppercase_letters

/// Contract: the lowest later index whose byte is outside `b'A'..=b'Z'`,
///   `b'a'..=b'z'`, and `b'0'..=b'9'` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `Ab_c-d`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityTypeNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'_' })`.
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
        result,
        Err(SolidityTypeNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        })
    );
}
// solidity_type_name_refuses_the_lowest_later_byte_outside_letters_and_digits

/// Contract: a text within the leading and later sets returns `Ok` holding
///   the text moved from the params.
/// Arrange: params overriding `text` with `AZaz09`.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` equals `AZaz09`.
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
    let Ok(name) = result else {
        panic!("the text admits")
    };
    assert_eq!(name.text, "AZaz09");
}
// solidity_type_name_admits_every_boundary_byte_of_its_sets

/// Contract: empty text returns `Err(Empty)`.
/// Arrange: `build_solidity_member_name_constructor_params` overriding `text`
///   with the empty string.
/// Act:    `SolidityMemberName::try_new` over it.
/// Assert: the result equals `Err(SolidityMemberNameTryNewErrorReturn::Empty)`.
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
    assert_eq!(result, Err(SolidityMemberNameTryNewErrorReturn::Empty));
}
// solidity_member_name_refuses_empty_text

/// Contract: a leading byte outside `b'a'..=b'z'` returns
///   `LeadingCharacter { byte }`, decided before the later bytes.
/// Arrange: params overriding `text` with an uppercase `A` followed by `-`,
///   a leading byte the later set admits.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'A' })`.
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
        result,
        Err(SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'A' })
    );
}
// solidity_member_name_refuses_a_leading_byte_outside_lowercase_letters

/// Contract: the lowest later index whose byte is outside `b'A'..=b'Z'`,
///   `b'a'..=b'z'`, and `b'0'..=b'9'` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `ab_c-d`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityMemberNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'_' })`.
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
        result,
        Err(SolidityMemberNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        })
    );
}
// solidity_member_name_refuses_the_lowest_later_byte_outside_letters_and_digits

/// Contract: a text within the leading and later sets returns `Ok` holding
///   the text moved from the params.
/// Arrange: params overriding `text` with `azAZ09`.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` equals `azAZ09`.
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
    let Ok(name) = result else {
        panic!("the text admits")
    };
    assert_eq!(name.text, "azAZ09");
}
// solidity_member_name_admits_every_boundary_byte_of_its_sets

/// Contract: an identifier part with no byte returns `Err(Empty)`.
/// Arrange: `build_spdx_license_identifier_constructor_params` overriding
///   `text` with the empty string.
/// Act:    `SpdxLicenseIdentifier::try_new` over it.
/// Assert: the result equals `Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty)`.
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
    assert_eq!(result, Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty));
}
// spdx_license_identifier_refuses_empty_text

/// Contract: a text whose only byte is the final `+` has an empty identifier
///   part and returns `Err(Empty)`.
/// Arrange: params overriding `text` with `+`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty)`.
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
    assert_eq!(result, Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty));
}
// spdx_license_identifier_refuses_a_lone_plus_as_empty

/// Contract: the lowest index of the identifier part whose byte is outside
///   letters, digits, `.`, and `-` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `A_B C`, a refused `_` at index `1`
///   and a later refused space.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SpdxLicenseIdentifierTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: b'_' })`.
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
        result,
        Err(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: b'_'
        })
    );
}
// spdx_license_identifier_refuses_the_lowest_byte_outside_its_set

/// Contract: only one `+`, as the final byte, is outside the identifier part,
///   so a second `+` is refused with its index in the whole text.
/// Arrange: params overriding `text` with `MIT++`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SpdxLicenseIdentifierTryNewErrorReturn::
///   InvalidCharacter { index: 3, byte: b'+' })`.
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
        result,
        Err(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
            index: 3,
            byte: b'+'
        })
    );
}
// spdx_license_identifier_refuses_a_plus_that_is_not_the_only_final_byte

/// Contract: a text within the set returns `Ok` holding the text moved from
///   the params.
/// Arrange: params overriding `text` with `AZaz09.-`.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` equals `AZaz09.-`.
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
    let Ok(identifier) = result else {
        panic!("the text admits")
    };
    assert_eq!(identifier.text, "AZaz09.-");
}
// spdx_license_identifier_admits_every_boundary_byte_of_its_set

/// Contract: a final `+` is admitted and the whole text, `+` included, is held.
/// Arrange: params overriding `text` with `GPL-2.0+`.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` equals `GPL-2.0+`.
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
    let Ok(identifier) = result else {
        panic!("the text admits")
    };
    assert_eq!(identifier.text, "GPL-2.0+");
}
// spdx_license_identifier_admits_a_final_plus_and_keeps_it

/// Contract: a byte equal to `b'"'` returns `InvalidCharacter { index, byte }`.
/// Arrange: `build_solidity_string_literal_constructor_params` overriding
///   `text` with `ab"c`.
/// Act:    `SolidityStringLiteral::try_new` over it.
/// Assert: the result equals `Err(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: 0x22 })`.
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
        result,
        Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: 0x22
        })
    );
}
// solidity_string_literal_refuses_a_double_quote

/// Contract: a byte equal to `b'\\'` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `ab\c`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: 0x5C })`.
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
        result,
        Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: 0x5C
        })
    );
}
// solidity_string_literal_refuses_a_backslash

/// Contract: a byte below `0x20` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `a` followed by the byte `0x1F`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: 0x1F })`.
#[test]
fn solidity_string_literal_refuses_a_byte_below_space() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\u{1f}".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: 0x1F
        })
    );
}
// solidity_string_literal_refuses_a_byte_below_space

/// Contract: a byte above `0x7E` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `text` with `a` followed by the byte `0x7F`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: 0x7F })`.
#[test]
fn solidity_string_literal_refuses_a_byte_above_tilde() {
    // Arrange
    let params = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\u{7f}".to_string()),
        },
    );

    // Act
    let result = SolidityStringLiteral::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: 0x7F
        })
    );
}
// solidity_string_literal_refuses_a_byte_above_tilde

/// Contract: the decision is over bytes, so a non-ASCII character is refused
///   at its first byte.
/// Arrange: params overriding `text` with the one character `é`, whose UTF-8
///   bytes are `0xC3 0xA9`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: 0xC3 })`.
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
        result,
        Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: 0xC3
        })
    );
}
// solidity_string_literal_refuses_the_first_byte_of_a_non_ascii_character

/// Contract: the lowest index outside the set is reported.
/// Arrange: params overriding `text` with `a"b\`, a `"` at index `1` and a `\`
///   at index `3`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityStringLiteralTryNewErrorReturn::
///   InvalidCharacter { index: 1, byte: 0x22 })`.
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
        result,
        Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: 0x22
        })
    );
}
// solidity_string_literal_refuses_the_lowest_index_when_several_bytes_are_refused

/// Contract: the empty string is admitted.
/// Arrange: params overriding `text` with the empty string.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` is empty.
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
    let Ok(literal) = result else {
        panic!("the text admits")
    };
    assert_eq!(literal.text, "");
}
// solidity_string_literal_admits_the_empty_string

/// Contract: bytes `0x20` through `0x7E` other than `"` and `\` are admitted
///   and the text is moved from the params.
/// Arrange: params overriding `text` with the space, `!`, `#`, `[`, `]`, `^`,
///   and `~`, the printable boundaries and the neighbours of the two excluded
///   bytes.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `text` equals that literal.
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
    let Ok(literal) = result else {
        panic!("the text admits")
    };
    assert_eq!(literal.text, " !#[]^~");
}
// solidity_string_literal_admits_every_printable_byte_beside_the_excluded_ones

/// Contract: one branch, `Ok(SolidityUint256 { big_endian })` with the array
///   moved from the params.
/// Arrange: `build_solidity_uint256_constructor_params` overriding
///   `big_endian` with the array whose byte at each index equals the index,
///   which differs from the builder default.
/// Act:    `SolidityUint256::try_new` over it.
/// Assert: the result is `Ok` and its `big_endian` equals the array `0`
///   through `31` in order.
#[test]
fn solidity_uint256_holds_the_array_moved_from_the_params() {
    // Arrange
    let params =
        build_solidity_uint256_constructor_params(SolidityUint256ConstructorParamsOverrides {
            big_endian: Some(core::array::from_fn(|index| {
                u8::try_from(index).expect("a 32-element index fits a u8")
            })),
        });

    // Act
    let result = SolidityUint256::try_new(params);

    // Assert
    let Ok(value) = result else {
        panic!("the params admit")
    };
    assert_eq!(
        value.big_endian,
        core::array::from_fn::<u8, 32, _>(|index| {
            u8::try_from(index).expect("a 32-element index fits a u8")
        })
    );
}
// solidity_uint256_holds_the_array_moved_from_the_params

/// Contract: empty `entries` returns `Err(Empty)`.
/// Arrange: `build_solidity_library_entries_constructor_params` overriding
///   `entries` with the empty list.
/// Act:    `SolidityLibraryEntries::try_new` over it.
/// Assert: the result equals `Err(SolidityLibraryEntriesTryNewErrorReturn::Empty)`.
#[test]
fn solidity_library_entries_refuses_an_empty_list() {
    // Arrange
    let params = build_solidity_library_entries_constructor_params(
        SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(vec![]),
        },
    );

    // Act
    let result = SolidityLibraryEntries::try_new(params);

    // Assert
    assert_eq!(result, Err(SolidityLibraryEntriesTryNewErrorReturn::Empty));
}
// solidity_library_entries_refuses_an_empty_list

/// Contract: the lowest index `j` whose name equals the name of an entry at a
///   lower index `i` returns `DuplicateName { first_index: i, duplicate_index: j }`,
///   judged by name alone.
/// Arrange: params overriding `entries` with four `build_solidity_library_entry`
///   values, each overriding `name` with `build_solidity_constant_name` and
///   `value` with `SolidityConstantValue::Uint64`, named `X`, `Y`, `Y`, `X`
///   with the values `1`, `2`, `3`, `4`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityLibraryEntriesTryNewErrorReturn::
///   DuplicateName { first_index: 1, duplicate_index: 2 })`.
#[test]
fn solidity_library_entries_refuses_the_lowest_index_whose_name_repeats_an_earlier_name() {
    // Arrange
    let entry = |name: &str, value: u64| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
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
        result,
        Err(SolidityLibraryEntriesTryNewErrorReturn::DuplicateName {
            first_index: 1,
            duplicate_index: 2
        })
    );
}
// solidity_library_entries_refuses_the_lowest_index_whose_name_repeats_an_earlier_name

/// Contract: distinct names return `Ok` holding the vector moved from the
///   params in its order.
/// Arrange: params overriding `entries` with three `build_solidity_library_entry`
///   values named `GAMMA`, `ALPHA`, and `BETA`, in that unsorted order, with
///   `SolidityConstantValue::Uint64` values `3`, `1`, and `2`.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `entries`, read in order, hold the name
///   texts `GAMMA`, `ALPHA`, `BETA` with the values `Uint64(3)`, `Uint64(1)`,
///   `Uint64(2)`.
#[test]
fn solidity_library_entries_admits_distinct_names_and_keeps_their_order() {
    // Arrange
    let entry = |name: &str, value: u64| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
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
    let Ok(entries) = result else {
        panic!("the entries admit")
    };
    let names: Vec<&str> = entries
        .entries
        .iter()
        .map(|entry| entry.name.text.as_str())
        .collect();
    assert_eq!(names, ["GAMMA", "ALPHA", "BETA"]);
    let values: Vec<u64> = entries
        .entries
        .iter()
        .map(|entry| match entry.value {
            SolidityConstantValue::Uint64(value) => value,
            _ => panic!("the arranged value is a uint64"),
        })
        .collect();
    assert_eq!(values, [3, 1, 2]);
}
// solidity_library_entries_admits_distinct_names_and_keeps_their_order

/// Contract: empty text returns `Err(Empty)`.
/// Arrange: `build_solidity_abi_type_constructor_params` overriding `name`
///   with the empty string.
/// Act:    `SolidityAbiType::try_new` over it.
/// Assert: the result equals `Err(SolidityAbiTypeTryNewErrorReturn::Empty)`.
#[test]
fn solidity_abi_type_refuses_empty_text() {
    // Arrange
    let params =
        build_solidity_abi_type_constructor_params(SolidityAbiTypeConstructorParamsOverrides {
            name: Some(String::new()),
        });

    // Act
    let result = SolidityAbiType::try_new(params);

    // Assert
    assert_eq!(result, Err(SolidityAbiTypeTryNewErrorReturn::Empty));
}
// solidity_abi_type_refuses_empty_text

/// Contract: a leading byte outside `b'A'..=b'Z'` and `b'a'..=b'z'` returns
///   `LeadingCharacter { byte }`, decided before the later bytes.
/// Arrange: params overriding `name` with a digit `1` followed by `-`, a
///   leading byte the later set admits and a later byte it refuses.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityAbiTypeTryNewErrorReturn::LeadingCharacter { byte: b'1' })`.
#[test]
fn solidity_abi_type_refuses_a_leading_byte_outside_letters() {
    // Arrange
    let params =
        build_solidity_abi_type_constructor_params(SolidityAbiTypeConstructorParamsOverrides {
            name: Some("1-".to_string()),
        });

    // Act
    let result = SolidityAbiType::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SolidityAbiTypeTryNewErrorReturn::LeadingCharacter { byte: b'1' })
    );
}
// solidity_abi_type_refuses_a_leading_byte_outside_letters

/// Contract: the lowest later index whose byte is outside `b'A'..=b'Z'`,
///   `b'a'..=b'z'`, and `b'0'..=b'9'` returns `InvalidCharacter { index, byte }`.
/// Arrange: params overriding `name` with `ab_c-d`, whose `_` at index `2` is
///   refused and a later `-` is also refused.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityAbiTypeTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'_' })`.
#[test]
fn solidity_abi_type_refuses_the_lowest_later_byte_outside_letters_and_digits() {
    // Arrange
    let params =
        build_solidity_abi_type_constructor_params(SolidityAbiTypeConstructorParamsOverrides {
            name: Some("ab_c-d".to_string()),
        });

    // Act
    let result = SolidityAbiType::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SolidityAbiTypeTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        })
    );
}
// solidity_abi_type_refuses_the_lowest_later_byte_outside_letters_and_digits

/// Contract: an admitted name returns `Ok` holding the text moved from the
///   params, letters of both cases and digits included.
/// Arrange: params overriding `name` with `azAZ09`, the inclusive ends of
///   every range, which differs from the builder default.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and `as_str` on it equals `azAZ09`.
#[test]
fn solidity_abi_type_holds_the_name_moved_from_the_params() {
    // Arrange
    let params =
        build_solidity_abi_type_constructor_params(SolidityAbiTypeConstructorParamsOverrides {
            name: Some("azAZ09".to_string()),
        });

    // Act
    let result = SolidityAbiType::try_new(params);

    // Assert
    let Ok(abi_type) = result else {
        panic!("the name admits")
    };
    assert_eq!(abi_type.as_str(), "azAZ09");
}
// solidity_abi_type_holds_the_name_moved_from_the_params

/// Contract: `Text` returns `true`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::Text`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `true`.
#[test]
fn solidity_record_member_decodes_text_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::Text),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(result);
}
// solidity_record_member_decodes_text_into_memory

/// Contract: `Bytes` returns `true`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::Bytes`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `true`.
#[test]
fn solidity_record_member_decodes_bytes_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::Bytes),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(result);
}
// solidity_record_member_decodes_bytes_into_memory

/// Contract: every kind other than `Text` and `Bytes` returns `false`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::FixedBytes32`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `false`.
#[test]
fn solidity_record_member_does_not_decode_fixed_bytes_32_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::FixedBytes32),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(!result);
}
// solidity_record_member_does_not_decode_fixed_bytes_32_into_memory

/// Contract: `Unsigned16` returns `false`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::Unsigned16`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `false`.
#[test]
fn solidity_record_member_does_not_decode_unsigned_16_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::Unsigned16),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(!result);
}
// solidity_record_member_does_not_decode_unsigned_16_into_memory

/// Contract: `Unsigned32` returns `false`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::Unsigned32`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `false`.
#[test]
fn solidity_record_member_does_not_decode_unsigned_32_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::Unsigned32),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(!result);
}
// solidity_record_member_does_not_decode_unsigned_32_into_memory

/// Contract: `Unsigned64` returns `false`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::Unsigned64`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `false`.
#[test]
fn solidity_record_member_does_not_decode_unsigned_64_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::Unsigned64),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(!result);
}
// solidity_record_member_does_not_decode_unsigned_64_into_memory

/// Contract: `FixedBytes20` returns `false`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::FixedBytes20`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `false`.
#[test]
fn solidity_record_member_does_not_decode_fixed_bytes_20_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::FixedBytes20),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(!result);
}
// solidity_record_member_does_not_decode_fixed_bytes_20_into_memory

/// Contract: `Unsigned256` returns `false`.
/// Arrange: `build_solidity_record_member` overriding `kind` with
///   `CanonicalFieldKind::Unsigned256`.
/// Act:    `decodes_into_memory` on it.
/// Assert: the result is `false`.
#[test]
fn solidity_record_member_does_not_decode_unsigned_256_into_memory() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        kind: Some(CanonicalFieldKind::Unsigned256),
        ..Default::default()
    });

    // Act
    let result = member.decodes_into_memory();

    // Assert
    assert!(!result);
}
// solidity_record_member_does_not_decode_unsigned_256_into_memory

/// Contract: fewer than two members returns `TooFew { actual }` with the length.
/// Arrange: `build_solidity_record_members_constructor_params` overriding
///   `members` with the empty list.
/// Act:    `SolidityRecordMembers::try_new` over it.
/// Assert: the result equals
///   `Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 0 })`.
#[test]
fn solidity_record_members_refuse_an_empty_list_as_too_few() {
    // Arrange
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 0 })
    );
}
// solidity_record_members_refuse_an_empty_list_as_too_few

/// Contract: fewer than two members returns `TooFew { actual }` with the length.
/// Arrange: params overriding `members` with one `build_solidity_record_member`.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 1 })`.
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
        result,
        Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 1 })
    );
}
// solidity_record_members_refuse_a_single_member_as_too_few

/// Contract: the count is decided before the names.
/// Arrange: params overriding `members` with one `build_solidity_record_member`
///   whose `name` is `line`.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 1 })`.
#[test]
fn solidity_record_members_refuse_a_single_reserved_name_as_too_few() {
    // Arrange
    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
        name: Some(build_solidity_member_name(
            SolidityMemberNameConstructorParamsOverrides {
                text: Some("line".to_string()),
            },
        )),
        ..Default::default()
    });
    let params = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![member]),
        },
    );

    // Act
    let result = SolidityRecordMembers::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 1 })
    );
}
// solidity_record_members_refuse_a_single_reserved_name_as_too_few

/// Contract: the lowest index `k` whose name text equals `line` returns
///   `ReservedName { index: k }`.
/// Arrange: params overriding `members` with four members named `alpha`,
///   `line`, `beta`, `line`.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: 1 })`.
#[test]
fn solidity_record_members_refuse_the_lowest_index_named_line() {
    // Arrange
    let member = |name: &str| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
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
        result,
        Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: 1 })
    );
}
// solidity_record_members_refuse_the_lowest_index_named_line

/// Contract: the reserved check is decided before the duplicate check.
/// Arrange: params overriding `members` with two members both named `line`.
/// Act:    `try_new` over it.
/// Assert: the result equals
///   `Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: 0 })`.
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
        result,
        Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: 0 })
    );
}
// solidity_record_members_refuse_a_reserved_name_before_a_duplicate

/// Contract: the lowest index `j` whose name equals the name of a member at a
///   lower index `i` returns `DuplicateName { first_index: i, duplicate_index: j }`,
///   judged by name alone.
/// Arrange: params overriding `members` with four `build_solidity_record_member`
///   values named `alpha`, `beta`, `beta`, `alpha` with the kinds `Text`,
///   `Bytes`, `Unsigned16`, `Unsigned64`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityRecordMembersTryNewErrorReturn::
///   DuplicateName { first_index: 1, duplicate_index: 2 })`.
#[test]
fn solidity_record_members_refuse_the_lowest_index_whose_name_repeats_an_earlier_name() {
    // Arrange
    let member = |name: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            kind: Some(kind),
            ..Default::default()
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
        result,
        Err(SolidityRecordMembersTryNewErrorReturn::DuplicateName {
            first_index: 1,
            duplicate_index: 2
        })
    );
}
// solidity_record_members_refuse_the_lowest_index_whose_name_repeats_an_earlier_name

/// Contract: members that are at least two, unique, and not named `line`
///   return `Ok` holding the vector moved from the params in its order.
/// Arrange: params overriding `members` with three `build_solidity_record_member`
///   values named `zeta`, `lineCount`, and `alpha`, in that unsorted order,
///   with the kinds `Text`, `Unsigned16`, and `Bytes`.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and its `members`, read in order, hold the name
///   texts `zeta`, `lineCount`, `alpha` with the kinds `Text`, `Unsigned16`,
///   `Bytes`.
#[test]
fn solidity_record_members_admit_unique_names_that_contain_line_and_keep_their_order() {
    // Arrange
    let member = |name: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            kind: Some(kind),
            ..Default::default()
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
    let Ok(members) = result else {
        panic!("the members admit")
    };
    let names: Vec<&str> = members
        .members
        .iter()
        .map(|member| member.name.text.as_str())
        .collect();
    assert_eq!(names, ["zeta", "lineCount", "alpha"]);
    let kinds: Vec<CanonicalFieldKind> = members.members.iter().map(|member| member.kind).collect();
    assert_eq!(
        kinds,
        [
            CanonicalFieldKind::Text,
            CanonicalFieldKind::Unsigned16,
            CanonicalFieldKind::Bytes
        ]
    );
}
// solidity_record_members_admit_unique_names_that_contain_line_and_keep_their_order

/// Contract: empty `declarations` returns `Err(Empty)`.
/// Arrange: `build_solidity_record_declarations_constructor_params` overriding
///   `declarations` with the empty list.
/// Act:    `SolidityRecordDeclarations::try_new` over it.
/// Assert: the result equals
///   `Err(SolidityRecordDeclarationsTryNewErrorReturn::Empty)`.
#[test]
fn solidity_record_declarations_refuse_an_empty_list() {
    // Arrange
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![]),
        },
    );

    // Act
    let result = SolidityRecordDeclarations::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SolidityRecordDeclarationsTryNewErrorReturn::Empty)
    );
}
// solidity_record_declarations_refuse_an_empty_list

/// Contract: the lowest index `j` whose `struct_name` equals that of a
///   declaration at a lower index `i` returns
///   `DuplicateStructName { first_index: i, duplicate_index: j }`.
/// Arrange: params overriding `declarations` with four
///   `build_solidity_record_declaration` values, each overriding `struct_name`
///   with `build_solidity_type_name`, `decode_function_name` with
///   `build_solidity_member_name`, and `path_constant_name` with
///   `build_solidity_constant_name`, the struct names `First`, `Second`,
///   `Second`, `First`, the decode function names `decodeA`, `decodeB`,
///   `decodeC`, `decodeD`, and the path constant names `PATH_A`, `PATH_B`,
///   `PATH_C`, `PATH_D`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateStructName { first_index: 1, duplicate_index: 2 })`.
#[test]
fn solidity_record_declarations_refuse_the_lowest_index_whose_struct_name_repeats() {
    // Arrange
    let declaration = |struct_name: &str, decode: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path.to_string()),
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
        result,
        Err(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                first_index: 1,
                duplicate_index: 2
            }
        )
    );
}
// solidity_record_declarations_refuse_the_lowest_index_whose_struct_name_repeats

/// Contract: the lowest index `j` whose `decode_function_name` equals that of
///   a declaration at a lower index `i` returns
///   `DuplicateDecodeFunctionName { first_index: i, duplicate_index: j }`.
/// Arrange: params overriding `declarations` with three
///   `build_solidity_record_declaration` values, each overriding `struct_name`,
///   `decode_function_name`, and `path_constant_name` through their name
///   builders, the struct names `Alpha`, `Beta`, `Gamma`, the decode function
///   names `decodeA`, `decodeA`, `decodeB`, and the path constant names
///   `PATH_A`, `PATH_B`, `PATH_C`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateDecodeFunctionName { first_index: 0, duplicate_index: 1 })`.
#[test]
fn solidity_record_declarations_refuse_the_lowest_index_whose_decode_function_name_repeats() {
    // Arrange
    let declaration = |struct_name: &str, decode: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path.to_string()),
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
        result,
        Err(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                first_index: 0,
                duplicate_index: 1
            }
        )
    );
}
// solidity_record_declarations_refuse_the_lowest_index_whose_decode_function_name_repeats

/// Contract: the lowest index `j` whose `path_constant_name` equals that of a
///   declaration at a lower index `i` returns
///   `DuplicatePathConstantName { first_index: i, duplicate_index: j }`.
/// Arrange: params overriding `declarations` with four
///   `build_solidity_record_declaration` values, each overriding `struct_name`,
///   `decode_function_name`, and `path_constant_name` through their name
///   builders, the struct names `Alpha`, `Beta`, `Gamma`, `Delta`, the decode
///   function names `decodeA`, `decodeB`, `decodeC`, `decodeD`, and the path
///   constant names `PATH_A`, `PATH_B`, `PATH_C`, `PATH_B`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicatePathConstantName { first_index: 1, duplicate_index: 3 })`.
#[test]
fn solidity_record_declarations_refuse_the_lowest_index_whose_path_constant_name_repeats() {
    // Arrange
    let declaration = |struct_name: &str, decode: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path.to_string()),
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
        result,
        Err(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName {
                first_index: 1,
                duplicate_index: 3
            }
        )
    );
}
// solidity_record_declarations_refuse_the_lowest_index_whose_path_constant_name_repeats

/// Contract: the struct-name check is decided before the decode-function-name
///   check.
/// Arrange: params overriding `declarations` with three
///   `build_solidity_record_declaration` values, each overriding `struct_name`,
///   `decode_function_name`, and `path_constant_name` through their name
///   builders, the struct names `Alpha`, `Beta`, `Beta`, the decode function
///   names `decodeA`, `decodeA`, `decodeB`, and the path constant names
///   `PATH_A`, `PATH_B`, `PATH_C`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateStructName { first_index: 1, duplicate_index: 2 })`.
#[test]
fn solidity_record_declarations_refuse_a_repeated_struct_name_before_a_repeated_decode_function_name()
 {
    // Arrange
    let declaration = |struct_name: &str, decode: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path.to_string()),
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
        result,
        Err(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                first_index: 1,
                duplicate_index: 2
            }
        )
    );
}
// solidity_record_declarations_refuse_a_repeated_struct_name_before_a_repeated_decode_function_name

/// Contract: the decode-function-name check is decided before the
///   path-constant-name check.
/// Arrange: params overriding `declarations` with three
///   `build_solidity_record_declaration` values, each overriding `struct_name`,
///   `decode_function_name`, and `path_constant_name` through their name
///   builders, the struct names `Alpha`, `Beta`, `Gamma`, the decode function
///   names `decodeA`, `decodeB`, `decodeB`, and the path constant names
///   `PATH_A`, `PATH_A`, `PATH_B`.
/// Act:    `try_new` over it.
/// Assert: the result equals `Err(SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateDecodeFunctionName { first_index: 1, duplicate_index: 2 })`.
#[test]
fn solidity_record_declarations_refuse_a_repeated_decode_function_name_before_a_repeated_path_constant_name()
 {
    // Arrange
    let declaration = |struct_name: &str, decode: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path.to_string()),
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
        result,
        Err(
            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                first_index: 1,
                duplicate_index: 2
            }
        )
    );
}
// solidity_record_declarations_refuse_a_repeated_decode_function_name_before_a_repeated_path_constant_name

/// Contract: unique names return `Ok` holding the vector moved from the
///   params in its order.
/// Arrange: params overriding `declarations` with three
///   `build_solidity_record_declaration` values, each overriding `struct_name`,
///   `decode_function_name`, and `path_constant_name` through their name
///   builders, the struct names `Gamma`, `Alpha`, `Beta`, in that unsorted
///   order, the decode function names `decodeC`, `decodeA`, `decodeB`, and the
///   path constant names `PATH_C`, `PATH_A`, `PATH_B`.
/// Act:    `try_new` over it.
/// Assert: the result is `Ok` and the struct name texts of its `declarations`,
///   read in order, equal `Gamma`, `Alpha`, `Beta`.
#[test]
fn solidity_record_declarations_admit_unique_names_and_keep_their_order() {
    // Arrange
    let declaration = |struct_name: &str, decode: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path.to_string()),
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
    let Ok(declarations) = result else {
        panic!("the declarations admit")
    };
    let names: Vec<&str> = declarations
        .declarations
        .iter()
        .map(|declaration| declaration.struct_name.text.as_str())
        .collect();
    assert_eq!(names, ["Gamma", "Alpha", "Beta"]);
}
// solidity_record_declarations_admit_unique_names_and_keep_their_order

/// Contract: `as_str` returns the text.
/// Arrange: `build_solidity_source_text` overriding `text` with `first line`,
///   a newline, `second line`, and a newline, which differs from the builder
///   default.
/// Act:    `as_str` on it.
/// Assert: the result equals that literal.
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
// solidity_source_text_returns_its_text

/// Contract: the header is the SPDX license line, the exact pragma, and an
///   empty line before the library declaration, in the constants branch.
/// Arrange: `build_render_deps` over defaults; `build_render_params` overriding
///   `license` with `build_spdx_license_identifier` over `GPL-3.0+` and
///   `compiler_version` with `build_solidity_compiler_version` over `major`
///   `7`, `minor` `28`, and `patch` `315`; `build_render_payload` overriding
///   `library_name` with `build_solidity_library_name` over `ReferenceLib` and
///   `body` with `SolidityLibraryBody::Constants` over
///   `build_solidity_library_entries` whose `entries` is one
///   `build_solidity_library_entry` default.
/// Act:    `render` over them.
/// Assert: the source text begins with `// SPDX-License-Identifier: GPL-3.0+`,
///   a newline, `pragma solidity 7.28.315;`, a newline, an empty line, and
///   `library `, the three release numbers being different so a swap fails.
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
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(success.source.as_str().starts_with(
        "// SPDX-License-Identifier: GPL-3.0+\npragma solidity 7.28.315;\n\nlibrary "
    ));
}
// render_opens_with_the_license_line_the_exact_pragma_and_an_empty_line

/// Contract: the library declaration is `library `, the library name's text,
///   and ` {` with a newline.
/// Arrange: `build_render_deps` over defaults; `build_render_params` overriding
///   `license` with `build_spdx_license_identifier` over `GPL-3.0+` and
///   `compiler_version` with `build_solidity_compiler_version` over `7`, `28`,
///   and `315`; `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` default.
/// Act:    `render` over them.
/// Assert: the source text contains `library ReferenceLib {` followed by a
///   newline.
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
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(success.source.as_str().contains("library ReferenceLib {\n"));
}
// render_declares_the_library_by_name

/// Contract: the text closes with `}` and one newline.
/// Arrange: `build_render_deps` over defaults; `build_render_params` over
///   defaults; `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` default.
/// Act:    `render` over them.
/// Assert: the source text ends with `}` and a newline and does not end with
///   two newlines.
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
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    let text = success.source.as_str();
    assert!(text.ends_with("}\n"));
    assert!(!text.ends_with("}\n\n"));
}
// render_closes_the_library_with_one_final_newline

/// Contract: the source is the header, one line per entry in payload order,
///   and the closing brace, each line ending in a newline.
/// Arrange: `build_render_deps` over defaults; `build_render_params` overriding
///   `license` with `build_spdx_license_identifier` over `GPL-3.0+` and
///   `compiler_version` with `build_solidity_compiler_version` over `7`, `28`,
///   and `315`; `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` are `build_solidity_library_entry` values named
///   `BYTES_VALUE` holding `Bytes` over `0x00` and `0xab`, `WORD_VALUE` holding
///   `Uint256` over a `build_solidity_uint256` whose `big_endian` has the byte
///   at each index equal to the index, `LARGE_VALUE` holding `Uint64` over
///   `u64::MAX`, `SMALL_VALUE` holding `Uint16` over `u16::MAX`, `FLAG_VALUE`
///   holding `Bool(true)`, and `LABEL_VALUE` holding `String` over a
///   `build_solidity_string_literal` whose `text` is `label`, each name through
///   `build_solidity_constant_name`.
/// Act:    `render` over them.
/// Assert: the return equals `Ok(build_render_success_return(..))` whose
///   `source` is `build_solidity_source_text` over the reference text.
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
    let word =
        build_solidity_uint256_constructor_params(SolidityUint256ConstructorParamsOverrides {
            big_endian: Some(core::array::from_fn(|index| {
                u8::try_from(index).expect("a 32-element index fits a u8")
            })),
        });
    let Ok(word) = SolidityUint256::try_new(word) else {
        panic!("the params admit")
    };
    let entries = vec![
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(name("BYTES_VALUE")),
            value: Some(SolidityConstantValue::Bytes(vec![0x00, 0xab])),
        }),
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(name("WORD_VALUE")),
            value: Some(SolidityConstantValue::Uint256(word)),
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
                build_solidity_string_literal(SolidityStringLiteralConstructorParamsOverrides {
                    text: Some("label".to_string()),
                }),
            )),
        }),
    ];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let expected = build_render_success_return(RenderSuccessReturnOverrides {
        source: Some(build_solidity_source_text(SoliditySourceTextOverrides {
            text: Some(
                "// SPDX-License-Identifier: GPL-3.0+\n\
                 pragma solidity 7.28.315;\n\
                 \n\
                 library ReferenceLib {\n\
                 \x20   bytes internal constant BYTES_VALUE = hex\"00ab\";\n\
                 \x20   uint256 internal constant WORD_VALUE = \
                 0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f;\n\
                 \x20   uint64 internal constant LARGE_VALUE = 18446744073709551615;\n\
                 \x20   uint16 internal constant SMALL_VALUE = 65535;\n\
                 \x20   bool internal constant FLAG_VALUE = true;\n\
                 \x20   string internal constant LABEL_VALUE = \"label\";\n\
                 }\n"
                .to_string(),
            ),
        })),
    });
    assert_eq!(result, Ok(expected));
}
// render_produces_the_exact_source_of_a_reference_library

/// Contract: `Bytes(bytes)` renders as `bytes` with `hex"` and two lowercase
///   hex digits per byte and `"`.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` named `BYTES_VALUE`
///   holding `Bytes` over `0x00`, `0x0a`, `0xab`, and `0xff`, a byte with a
///   zero high nibble beside bytes whose digits are letters.
/// Act:    `render` over them.
/// Assert: the source text contains the entry's line, four spaces,
///   `bytes internal constant BYTES_VALUE = `, the literal the spec's
///   byte-string rendering gives for those bytes, `;`, and a newline.
#[test]
fn render_writes_a_byte_string_as_a_lowercase_hex_literal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let entries = vec![build_solidity_library_entry(
        SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some("BYTES_VALUE".to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Bytes(vec![0x00, 0x0a, 0xab, 0xff])),
        },
    )];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("    bytes internal constant BYTES_VALUE = hex\"000aabff\";\n")
    );
}
// render_writes_a_byte_string_as_a_lowercase_hex_literal

/// Contract: an empty byte string renders as `hex""`.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` named `BYTES_VALUE`
///   holding `Bytes` over an empty vector.
/// Act:    `render` over them.
/// Assert: the source text contains the line
///   `    bytes internal constant BYTES_VALUE = hex"";` and a newline.
#[test]
fn render_writes_an_empty_byte_string_as_an_empty_hex_literal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let entries = vec![build_solidity_library_entry(
        SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some("BYTES_VALUE".to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Bytes(vec![])),
        },
    )];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("    bytes internal constant BYTES_VALUE = hex\"\";\n")
    );
}
// render_writes_an_empty_byte_string_as_an_empty_hex_literal

/// Contract: `Uint256(value)` renders as `uint256` with `0x` and two
///   lowercase hex digits per byte of `big_endian` in array order.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` named `WORD_VALUE`
///   holding `Uint256` over a `build_solidity_uint256` whose `big_endian` has
///   the byte at each index equal to the index.
/// Act:    `render` over them.
/// Assert: the source text contains the entry's line, four spaces,
///   `uint256 internal constant WORD_VALUE = `, `0x` and the lowercase hex
///   digit pair of each byte in array order, `;`, and a newline.
#[test]
fn render_writes_a_256_bit_value_as_big_endian_hex_digits() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let word =
        build_solidity_uint256_constructor_params(SolidityUint256ConstructorParamsOverrides {
            big_endian: Some(core::array::from_fn(|index| {
                u8::try_from(index).expect("a 32-element index fits a u8")
            })),
        });
    let Ok(word) = SolidityUint256::try_new(word) else {
        panic!("the params admit")
    };
    let entries = vec![build_solidity_library_entry(
        SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some("WORD_VALUE".to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Uint256(word)),
        },
    )];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(success.source.as_str().contains(
        "    uint256 internal constant WORD_VALUE = \
         0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f;\n"
    ));
}
// render_writes_a_256_bit_value_as_big_endian_hex_digits

/// Contract: `Uint64(value)` renders as `uint64` with the value in decimal.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` named `LARGE_VALUE`
///   holding `Uint64` over `u64::MAX`.
/// Act:    `render` over them.
/// Assert: the source text contains the entry's line, four spaces,
///   `uint64 internal constant LARGE_VALUE = `, the decimal digits of
///   `u64::MAX`, `;`, and a newline.
#[test]
fn render_writes_a_64_bit_value_in_decimal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let entries = vec![build_solidity_library_entry(
        SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some("LARGE_VALUE".to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Uint64(u64::MAX)),
        },
    )];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("    uint64 internal constant LARGE_VALUE = 18446744073709551615;\n")
    );
}
// render_writes_a_64_bit_value_in_decimal

/// Contract: `Uint16(value)` renders as `uint16` with the value in decimal.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` named `SMALL_VALUE`
///   holding `Uint16` over `u16::MAX`.
/// Act:    `render` over them.
/// Assert: the source text contains the entry's line, four spaces,
///   `uint16 internal constant SMALL_VALUE = `, the decimal digits of
///   `u16::MAX`, `;`, and a newline.
#[test]
fn render_writes_a_16_bit_value_in_decimal() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let entries = vec![build_solidity_library_entry(
        SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some("SMALL_VALUE".to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Uint16(u16::MAX)),
        },
    )];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("    uint16 internal constant SMALL_VALUE = 65535;\n")
    );
}
// render_writes_a_16_bit_value_in_decimal

/// Contract: `Bool(value)` renders as `bool` with `true` or `false`.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` are `build_solidity_library_entry` values named `FLAG_ON`
///   holding `Bool(true)` and `FLAG_OFF` holding `Bool(false)`.
/// Act:    `render` over them.
/// Assert: the source text contains the lines
///   `    bool internal constant FLAG_ON = true;` and
///   `    bool internal constant FLAG_OFF = false;`, each followed by a newline.
#[test]
fn render_writes_booleans_as_true_and_false() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let flag = |name: &str, value: bool| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Bool(value)),
        })
    };
    let entries = vec![flag("FLAG_ON", true), flag("FLAG_OFF", false)];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
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
// render_writes_booleans_as_true_and_false

/// Contract: `String(literal)` renders as `string` with the literal's text
///   between double quotes.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` is one `build_solidity_library_entry` named `LABEL_VALUE`
///   holding `String` over a `build_solidity_string_literal` whose `text` is
///   `a ~b`, a space and a tilde inside the text.
/// Act:    `render` over them.
/// Assert: the source text contains the line
///   `    string internal constant LABEL_VALUE = "a ~b";` and a newline.
#[test]
fn render_writes_a_string_between_double_quotes() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let entries = vec![build_solidity_library_entry(
        SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some("LABEL_VALUE".to_string()),
                },
            )),
            value: Some(SolidityConstantValue::String(
                build_solidity_string_literal(SolidityStringLiteralConstructorParamsOverrides {
                    text: Some("a ~b".to_string()),
                }),
            )),
        },
    )];
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(entries),
            }),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("    string internal constant LABEL_VALUE = \"a ~b\";\n")
    );
}
// render_writes_a_string_between_double_quotes

/// Contract: entries are written one per line in payload order.
/// Arrange: `build_render_deps` over defaults; `build_render_params` overriding
///   `license` with `build_spdx_license_identifier` over `GPL-3.0+` and
///   `compiler_version` with `build_solidity_compiler_version` over `7`, `28`,
///   and `315`; `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Constants` over `build_solidity_library_entries`
///   whose `entries` are `build_solidity_library_entry` values named `GAMMA`,
///   `ALPHA`, and `BETA`, in that unsorted order, holding `Uint64` over `3`,
///   `1`, and `2`.
/// Act:    `render` over them.
/// Assert: the return equals `Ok(build_render_success_return(..))` whose
///   `source` is `build_solidity_source_text` over the header, the three entry
///   lines in `GAMMA`, `ALPHA`, `BETA` order, and the closing brace.
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
    let entry = |name: &str, value: u64| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
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
    let expected = build_render_success_return(RenderSuccessReturnOverrides {
        source: Some(build_solidity_source_text(SoliditySourceTextOverrides {
            text: Some(
                "// SPDX-License-Identifier: GPL-3.0+\n\
                 pragma solidity 7.28.315;\n\
                 \n\
                 library ReferenceLib {\n\
                 \x20   uint64 internal constant GAMMA = 3;\n\
                 \x20   uint64 internal constant ALPHA = 1;\n\
                 \x20   uint64 internal constant BETA = 2;\n\
                 }\n"
                .to_string(),
            ),
        })),
    });
    assert_eq!(result, Ok(expected));
}
// render_writes_entries_in_payload_order
