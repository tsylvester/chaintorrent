#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    RenderDeps, SolidityConstantName, SolidityConstantNameTryNewErrorReturn, SolidityConstantValue,
    SolidityLibraryBody, SolidityLibraryEntries, SolidityLibraryEntriesTryNewErrorReturn,
    SolidityLibraryName, SolidityLibraryNameTryNewErrorReturn, SolidityMemberName,
    SolidityMemberNameTryNewErrorReturn, SolidityRecordDeclarations,
    SolidityRecordDeclarationsTryNewErrorReturn, SolidityRecordMembers,
    SolidityRecordMembersTryNewErrorReturn, SolidityStringLiteral,
    SolidityStringLiteralTryNewErrorReturn, SolidityTypeName, SolidityTypeNameTryNewErrorReturn,
    SpdxLicenseIdentifier, SpdxLicenseIdentifierTryNewErrorReturn,
};
use super::mock::{
    RenderParamsOverrides, RenderPayloadOverrides, SolidityAbiTypeConstructorParamsOverrides,
    SolidityCompilerVersionOverrides, SolidityConstantNameConstructorParamsOverrides,
    SolidityLibraryEntriesConstructorParamsOverrides, SolidityLibraryEntryOverrides,
    SolidityLibraryNameConstructorParamsOverrides, SolidityMemberNameConstructorParamsOverrides,
    SolidityRecordDeclarationOverrides, SolidityRecordDeclarationsConstructorParamsOverrides,
    SolidityRecordMemberOverrides, SolidityRecordMembersConstructorParamsOverrides,
    SolidityStringLiteralConstructorParamsOverrides, SolidityTypeNameConstructorParamsOverrides,
    SolidityUint256ConstructorParamsOverrides, SpdxLicenseIdentifierConstructorParamsOverrides,
    build_render_params, build_render_payload, build_solidity_abi_type,
    build_solidity_compiler_version, build_solidity_constant_name,
    build_solidity_constant_name_constructor_params, build_solidity_library_entries,
    build_solidity_library_entries_constructor_params, build_solidity_library_entry,
    build_solidity_library_name, build_solidity_library_name_constructor_params,
    build_solidity_member_name, build_solidity_member_name_constructor_params,
    build_solidity_record_declaration, build_solidity_record_declarations,
    build_solidity_record_declarations_constructor_params, build_solidity_record_member,
    build_solidity_record_members, build_solidity_record_members_constructor_params,
    build_solidity_string_literal, build_solidity_string_literal_constructor_params,
    build_solidity_type_name, build_solidity_type_name_constructor_params, build_solidity_uint256,
    build_spdx_license_identifier, build_spdx_license_identifier_constructor_params,
};
use super::render;
use encoding::CanonicalFieldKind;

/// Contract: a text whose leading byte is an uppercase ASCII letter and whose
///   remaining bytes are ASCII letters or digits is admitted, the text moved
///   from the params.
/// Arrange: the text "PairingConstants".
/// Act:     SolidityLibraryName::try_new over the built params.
/// Assert:  the returned value equals the name built over the same text.
#[test]
fn solidity_library_name_admits_an_upper_camel_identifier() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some("PairingConstants".to_string()),
        },
    );

    // Act
    let Ok(value) = SolidityLibraryName::try_new(params) else {
        panic!("the identifier is admitted")
    };

    // Assert
    assert_eq!(
        value,
        build_solidity_library_name(SolidityLibraryNameConstructorParamsOverrides {
            text: Some("PairingConstants".to_string()),
        })
    );
}

/// Contract: a text with no byte is refused as empty.
/// Arrange: the text "".
/// Act:     SolidityLibraryName::try_new over the built params.
/// Assert:  the error is SolidityLibraryNameTryNewErrorReturn::Empty.
#[test]
fn solidity_library_name_refuses_empty_text() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let Err(error) = SolidityLibraryName::try_new(params) else {
        panic!("the empty text is refused")
    };

    // Assert
    assert_eq!(error, SolidityLibraryNameTryNewErrorReturn::Empty);
}

/// Contract: a leading byte outside `A`-`Z` is refused with that byte.
/// Arrange: the text "pairingConstants", leading byte b'p'.
/// Act:     SolidityLibraryName::try_new over the built params.
/// Assert:  the error is LeadingCharacter { byte: b'p' }.
#[test]
fn solidity_library_name_refuses_a_leading_byte_that_is_not_an_uppercase_letter() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some("pairingConstants".to_string()),
        },
    );

    // Act
    let Err(error) = SolidityLibraryName::try_new(params) else {
        panic!("the lowercase leading byte is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte: b'p' }
    );
}

/// Contract: the lowest later byte outside `A`-`Z`, `a`-`z`, `0`-`9` is refused
///   with its index and byte.
/// Arrange: the text "Pairing_Lib-V1", first failing later byte '_' at index 7.
/// Act:     SolidityLibraryName::try_new over the built params.
/// Assert:  the error is InvalidCharacter { index: 7, byte: b'_' }.
#[test]
fn solidity_library_name_refuses_the_lowest_later_byte_outside_its_set() {
    // Arrange
    let params = build_solidity_library_name_constructor_params(
        SolidityLibraryNameConstructorParamsOverrides {
            text: Some("Pairing_Lib-V1".to_string()),
        },
    );

    // Act
    let Err(error) = SolidityLibraryName::try_new(params) else {
        panic!("the underscore is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
            index: 7,
            byte: b'_'
        }
    );
}

/// Contract: a text of uppercase letters, digits, and underscores after an
///   uppercase leading letter is admitted, the text moved from the params.
/// Arrange: the text "G1_GENERATOR".
/// Act:     SolidityConstantName::try_new over the built params.
/// Assert:  the returned value equals the name built over the same text.
#[test]
fn solidity_constant_name_admits_an_upper_snake_identifier() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some("G1_GENERATOR".to_string()),
        },
    );

    // Act
    let Ok(value) = SolidityConstantName::try_new(params) else {
        panic!("the identifier is admitted")
    };

    // Assert
    assert_eq!(
        value,
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some("G1_GENERATOR".to_string()),
        })
    );
}

/// Contract: a text with no byte is refused as empty.
/// Arrange: the text "".
/// Act:     SolidityConstantName::try_new over the built params.
/// Assert:  the error is SolidityConstantNameTryNewErrorReturn::Empty.
#[test]
fn solidity_constant_name_refuses_empty_text() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let Err(error) = SolidityConstantName::try_new(params) else {
        panic!("the empty text is refused")
    };

    // Assert
    assert_eq!(error, SolidityConstantNameTryNewErrorReturn::Empty);
}

/// Contract: a leading byte outside `A`-`Z` is refused with that byte.
/// Arrange: the text "1G_GENERATOR", leading byte b'1'.
/// Act:     SolidityConstantName::try_new over the built params.
/// Assert:  the error is LeadingCharacter { byte: b'1' }.
#[test]
fn solidity_constant_name_refuses_a_leading_digit() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some("1G_GENERATOR".to_string()),
        },
    );

    // Act
    let Err(error) = SolidityConstantName::try_new(params) else {
        panic!("the leading digit is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: b'1' }
    );
}

/// Contract: the lowest later byte outside `A`-`Z`, `0`-`9`, and `_` is refused
///   with its index and byte.
/// Arrange: the text "G1_Generator", first failing later byte 'e' at index 4.
/// Act:     SolidityConstantName::try_new over the built params.
/// Assert:  the error is InvalidCharacter { index: 4, byte: b'e' }.
#[test]
fn solidity_constant_name_refuses_the_lowest_later_byte_outside_its_set() {
    // Arrange
    let params = build_solidity_constant_name_constructor_params(
        SolidityConstantNameConstructorParamsOverrides {
            text: Some("G1_Generator".to_string()),
        },
    );

    // Act
    let Err(error) = SolidityConstantName::try_new(params) else {
        panic!("the lowercase later byte is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityConstantNameTryNewErrorReturn::InvalidCharacter {
            index: 4,
            byte: b'e'
        }
    );
}

/// Contract: an SPDX short identifier of letters, digits, `.`, and `-`, with at
///   most one `+` as its final byte, is admitted as the whole text.
/// Arrange: the texts "Apache-2.0" and "GPL-2.0+".
/// Act:     SpdxLicenseIdentifier::try_new over each built params.
/// Assert:  each returned value equals the identifier built over its text.
#[test]
fn spdx_license_identifier_admits_a_short_identifier_and_one_with_a_final_plus() {
    // Arrange
    let plain = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("Apache-2.0".to_string()),
        },
    );
    let plus = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("GPL-2.0+".to_string()),
        },
    );

    // Act
    let Ok(plain_value) = SpdxLicenseIdentifier::try_new(plain) else {
        panic!("the short identifier is admitted")
    };
    let Ok(plus_value) = SpdxLicenseIdentifier::try_new(plus) else {
        panic!("the identifier with a final plus is admitted")
    };

    // Assert
    assert_eq!(
        plain_value,
        build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("Apache-2.0".to_string()),
        })
    );
    assert_eq!(
        plus_value,
        build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("GPL-2.0+".to_string()),
        })
    );
}

/// Contract: a text whose identifier part has no byte is refused as empty.
/// Arrange: the texts "" and "+".
/// Act:     SpdxLicenseIdentifier::try_new over each built params.
/// Assert:  each error is SpdxLicenseIdentifierTryNewErrorReturn::Empty.
#[test]
fn spdx_license_identifier_refuses_empty_text_and_a_lone_plus() {
    // Arrange
    let empty = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );
    let lone_plus = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("+".to_string()),
        },
    );

    // Act
    let Err(empty_error) = SpdxLicenseIdentifier::try_new(empty) else {
        panic!("the empty text is refused")
    };
    let Err(plus_error) = SpdxLicenseIdentifier::try_new(lone_plus) else {
        panic!("the lone plus is refused")
    };

    // Assert
    assert_eq!(empty_error, SpdxLicenseIdentifierTryNewErrorReturn::Empty);
    assert_eq!(plus_error, SpdxLicenseIdentifierTryNewErrorReturn::Empty);
}

/// Contract: the lowest byte of the identifier part outside the identifier set
///   is refused with its index in the whole text.
/// Arrange: the text "MIT OR Apache-2.0", first failing byte b' ' at index 3.
/// Act:     SpdxLicenseIdentifier::try_new over the built params.
/// Assert:  the error is InvalidCharacter { index: 3, byte: b' ' }.
#[test]
fn spdx_license_identifier_refuses_a_compound_expression() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("MIT OR Apache-2.0".to_string()),
        },
    );

    // Act
    let Err(error) = SpdxLicenseIdentifier::try_new(params) else {
        panic!("the space is refused")
    };

    // Assert
    assert_eq!(
        error,
        SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
            index: 3,
            byte: b' '
        }
    );
}

/// Contract: a `+` that is not the final byte is part of the identifier part
///   and is refused with its index.
/// Arrange: the text "GPL+2.0", first failing byte b'+' at index 3.
/// Act:     SpdxLicenseIdentifier::try_new over the built params.
/// Assert:  the error is InvalidCharacter { index: 3, byte: b'+' }.
#[test]
fn spdx_license_identifier_refuses_a_plus_that_is_not_final() {
    // Arrange
    let params = build_spdx_license_identifier_constructor_params(
        SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("GPL+2.0".to_string()),
        },
    );

    // Act
    let Err(error) = SpdxLicenseIdentifier::try_new(params) else {
        panic!("the non-final plus is refused")
    };

    // Assert
    assert_eq!(
        error,
        SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
            index: 3,
            byte: b'+'
        }
    );
}

/// Contract: printable ASCII text other than `"` and `\`, including the empty
///   string, is admitted, the text moved from the params.
/// Arrange: the texts "bytes32,bytes20,uint64" and "".
/// Act:     SolidityStringLiteral::try_new over each built params.
/// Assert:  each returned value equals the literal built over its text.
#[test]
fn solidity_string_literal_admits_printable_text_and_the_empty_string() {
    // Arrange
    let printable = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("bytes32,bytes20,uint64".to_string()),
        },
    );
    let empty = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some(String::new()),
        },
    );

    // Act
    let Ok(printable_value) = SolidityStringLiteral::try_new(printable) else {
        panic!("the printable text is admitted")
    };
    let Ok(empty_value) = SolidityStringLiteral::try_new(empty) else {
        panic!("the empty string is admitted")
    };

    // Assert
    assert_eq!(
        printable_value,
        build_solidity_string_literal(SolidityStringLiteralConstructorParamsOverrides {
            text: Some("bytes32,bytes20,uint64".to_string()),
        })
    );
    assert_eq!(
        empty_value,
        build_solidity_string_literal(SolidityStringLiteralConstructorParamsOverrides {
            text: Some(String::new()),
        })
    );
}

/// Contract: the lowest byte equal to `"` or `\` is refused with its index.
/// Arrange: the texts "a\"b" and "a\\b", each failing at index 1.
/// Act:     SolidityStringLiteral::try_new over each built params.
/// Assert:  the errors are InvalidCharacter { index: 1, byte: b'"' } and
///   InvalidCharacter { index: 1, byte: b'\\' }.
#[test]
fn solidity_string_literal_refuses_a_double_quote_and_a_backslash() {
    // Arrange
    let quote = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\"b".to_string()),
        },
    );
    let backslash = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\\b".to_string()),
        },
    );

    // Act
    let Err(quote_error) = SolidityStringLiteral::try_new(quote) else {
        panic!("the double quote is refused")
    };
    let Err(backslash_error) = SolidityStringLiteral::try_new(backslash) else {
        panic!("the backslash is refused")
    };

    // Assert
    assert_eq!(
        quote_error,
        SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: b'"'
        }
    );
    assert_eq!(
        backslash_error,
        SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: b'\\'
        }
    );
}

/// Contract: the lowest byte outside `0x20..=0x7E` is refused with its index.
/// Arrange: the texts "a\nb" and "é", failing at index 1 byte 0x0A and at
///   index 0 byte 0xC3.
/// Act:     SolidityStringLiteral::try_new over each built params.
/// Assert:  the errors are InvalidCharacter { index: 1, byte: 0x0A } and
///   InvalidCharacter { index: 0, byte: 0xC3 }.
#[test]
fn solidity_string_literal_refuses_a_control_byte_and_a_non_ascii_byte() {
    // Arrange
    let control = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("a\nb".to_string()),
        },
    );
    let non_ascii = build_solidity_string_literal_constructor_params(
        SolidityStringLiteralConstructorParamsOverrides {
            text: Some("é".to_string()),
        },
    );

    // Act
    let Err(control_error) = SolidityStringLiteral::try_new(control) else {
        panic!("the control byte is refused")
    };
    let Err(non_ascii_error) = SolidityStringLiteral::try_new(non_ascii) else {
        panic!("the non-ASCII byte is refused")
    };

    // Assert
    assert_eq!(
        control_error,
        SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 1,
            byte: 0x0A
        }
    );
    assert_eq!(
        non_ascii_error,
        SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: 0xC3
        }
    );
}

/// Contract: an empty entries list is refused.
/// Arrange: the entries vec is empty.
/// Act:     SolidityLibraryEntries::try_new over the built params.
/// Assert:  the error is SolidityLibraryEntriesTryNewErrorReturn::Empty.
#[test]
fn solidity_library_entries_refuses_an_empty_list() {
    // Arrange
    let params = build_solidity_library_entries_constructor_params(
        SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(vec![]),
        },
    );

    // Act
    let Err(error) = SolidityLibraryEntries::try_new(params) else {
        panic!("the empty list is refused")
    };

    // Assert
    assert_eq!(error, SolidityLibraryEntriesTryNewErrorReturn::Empty);
}

/// Contract: the lowest index whose name equals a name at a lower index is
///   refused with the lowest such earlier index and that index.
/// Arrange: entries named "A", "B", "A", and "B"; the lowest duplicate index
///   is 2, whose lowest earlier match is 0.
/// Act:     SolidityLibraryEntries::try_new over the built params.
/// Assert:  the error is DuplicateName { first_index: 0, duplicate_index: 2 }.
#[test]
fn solidity_library_entries_refuses_a_repeated_constant_name() {
    // Arrange
    let entry_named = |text: &str| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_library_entries_constructor_params(
        SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(vec![
                entry_named("A"),
                entry_named("B"),
                entry_named("A"),
                entry_named("B"),
            ]),
        },
    );

    // Act
    let Err(error) = SolidityLibraryEntries::try_new(params) else {
        panic!("the repeated name is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityLibraryEntriesTryNewErrorReturn::DuplicateName {
            first_index: 0,
            duplicate_index: 2
        }
    );
}

/// Contract: an admitted list moves the vector from the params in its order.
/// Arrange: entries named "B" then "A".
/// Act:     SolidityLibraryEntries::try_new over the built params.
/// Assert:  the returned value equals the entries built over the same ordered
///   list.
#[test]
fn solidity_library_entries_keeps_its_entries_in_order() {
    // Arrange
    let entry_named = |text: &str| {
        build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let ordered = vec![entry_named("B"), entry_named("A")];
    let params = build_solidity_library_entries_constructor_params(
        SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(ordered.clone()),
        },
    );

    // Act
    let Ok(value) = SolidityLibraryEntries::try_new(params) else {
        panic!("the list is admitted")
    };

    // Assert
    assert_eq!(
        value,
        build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
            entries: Some(ordered),
        })
    );
}

/// Contract: the rendered source opens with the SPDX license line, the exact
///   compiler version pragma, and an empty line.
/// Arrange: the license "Apache-2.0" and compiler version 0.8.28; the default
///   payload.
/// Act:     render over the built params and payload.
/// Assert:  the first three lines are "// SPDX-License-Identifier: Apache-2.0",
///   "pragma solidity 0.8.28;", and the empty line.
#[test]
fn render_opens_with_the_license_line_the_exact_pragma_and_an_empty_line() {
    // Arrange
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("Apache-2.0".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(0),
                minor: Some(8),
                patch: Some(28),
            },
        )),
    });
    let payload = build_render_payload(Default::default());

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(lines[0], "// SPDX-License-Identifier: Apache-2.0");
    assert_eq!(lines[1], "pragma solidity 0.8.28;");
    assert_eq!(lines[2], "");
}

/// Contract: the rendered source declares the library by name and closes with
///   a single closing brace and one final newline.
/// Arrange: the library name "PairingVectors"; the default params.
/// Act:     render over the built payload.
/// Assert:  the fourth line is "library PairingVectors {"; the text ends with
///   "}\n" and does not end with "\n\n".
#[test]
fn render_declares_the_library_by_name_and_closes_it_with_one_final_newline() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("PairingVectors".to_string()),
            },
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let text = success.source.as_str();
    let lines: Vec<&str> = text.lines().collect();

    // Assert
    assert_eq!(lines[3], "library PairingVectors {");
    assert!(text.ends_with("}\n"));
    assert!(!text.ends_with("\n\n"));
}

/// Contract: a `Bytes` entry renders as `bytes` with a `hex"…"` literal of two
///   lowercase hex digits per byte.
/// Arrange: one entry CHALLENGE_TAG of value Bytes([0x00, 0xAB, 0x10]).
/// Act:     render over the built payload.
/// Assert:  the fifth line is
///   `    bytes internal constant CHALLENGE_TAG = hex"00ab10";`.
#[test]
fn render_writes_a_byte_string_as_a_lowercase_hex_literal() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("CHALLENGE_TAG".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bytes(vec![0x00, 0xAB, 0x10])),
                    },
                )]),
            }),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(
        lines[4],
        "    bytes internal constant CHALLENGE_TAG = hex\"00ab10\";"
    );
}

/// Contract: an empty `Bytes` entry renders the empty hex literal `hex""`.
/// Arrange: one entry EMPTY of value Bytes([]).
/// Act:     render over the built payload.
/// Assert:  the fifth line is
///   `    bytes internal constant EMPTY = hex"";`.
#[test]
fn render_writes_an_empty_byte_string_as_an_empty_hex_literal() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("EMPTY".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bytes(vec![])),
                    },
                )]),
            }),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(lines[4], "    bytes internal constant EMPTY = hex\"\";");
}

/// Contract: a `Uint256` entry renders as `uint256` with `0x` and two lowercase
///   hex digits per byte, most significant first.
/// Arrange: one entry SCALAR_FIELD_ORDER of value Uint256 over the bytes 0x00
///   through 0x1F ascending.
/// Act:     render over the built payload.
/// Assert:  the fifth line is the uint256 declaration with the 64 hex digits in
///   array order.
#[test]
fn render_writes_a_256_bit_value_as_big_endian_hex_digits() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("SCALAR_FIELD_ORDER".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint256(build_solidity_uint256(
                            SolidityUint256ConstructorParamsOverrides {
                                big_endian: Some(core::array::from_fn(|i| i as u8)),
                            },
                        ))),
                    },
                )]),
            }),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(
        lines[4],
        "    uint256 internal constant SCALAR_FIELD_ORDER = 0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f;"
    );
}

/// Contract: `Uint64` and `Uint16` entries render as `uint64` and `uint16` with
///   the value in decimal.
/// Arrange: entries VECTOR_COUNT of Uint64(u64::MAX) and STATEMENT_VERSION of
///   Uint16(u16::MAX).
/// Act:     render over the built payload.
/// Assert:  the fifth line is the uint64 declaration of 18446744073709551615
///   and the sixth the uint16 declaration of 65535.
#[test]
fn render_writes_64_bit_and_16_bit_values_in_decimal() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("VECTOR_COUNT".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint64(18446744073709551615)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("STATEMENT_VERSION".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint16(65535)),
                    }),
                ]),
            }),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(
        lines[4],
        "    uint64 internal constant VECTOR_COUNT = 18446744073709551615;"
    );
    assert_eq!(
        lines[5],
        "    uint16 internal constant STATEMENT_VERSION = 65535;"
    );
}

/// Contract: `Bool` entries render as `bool` with `true` or `false`.
/// Arrange: entries SECOND_GROUP_ARITHMETIC of Bool(true) and FIRST_GROUP_ONLY
///   of Bool(false).
/// Act:     render over the built payload.
/// Assert:  the fifth and sixth lines are the bool declarations of `true` and
///   `false`.
#[test]
fn render_writes_booleans_as_true_and_false() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("SECOND_GROUP_ARITHMETIC".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bool(true)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("FIRST_GROUP_ONLY".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bool(false)),
                    }),
                ]),
            }),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(
        lines[4],
        "    bool internal constant SECOND_GROUP_ARITHMETIC = true;"
    );
    assert_eq!(
        lines[5],
        "    bool internal constant FIRST_GROUP_ONLY = false;"
    );
}

/// Contract: a `String` entry renders as `string` with its text between double
///   quotes.
/// Arrange: one entry MINT_FIELDS of value String over "bytes32,bytes20,uint64".
/// Act:     render over the built payload.
/// Assert:  the fifth line is the string declaration of the quoted text.
#[test]
fn render_writes_a_string_between_double_quotes() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![build_solidity_library_entry(
                    SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("MINT_FIELDS".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::String(
                            build_solidity_string_literal(
                                SolidityStringLiteralConstructorParamsOverrides {
                                    text: Some("bytes32,bytes20,uint64".to_string()),
                                },
                            ),
                        )),
                    },
                )]),
            }),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(
        lines[4],
        "    string internal constant MINT_FIELDS = \"bytes32,bytes20,uint64\";"
    );
}

/// Contract: entries render one declaration per entry in payload order.
/// Arrange: entries ZETA of Uint16(2) then ALPHA of Uint16(1).
/// Act:     render over the built payload.
/// Assert:  the fifth line declares ZETA and the sixth declares ALPHA.
#[test]
fn render_writes_entries_in_payload_order() {
    // Arrange
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("ZETA".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint16(2)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("ALPHA".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint16(1)),
                    }),
                ]),
            }),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let lines: Vec<&str> = success.source.as_str().lines().collect();

    // Assert
    assert_eq!(lines[4], "    uint16 internal constant ZETA = 2;");
    assert_eq!(lines[5], "    uint16 internal constant ALPHA = 1;");
}

/// Contract: the constants body renders the license line, the pragma, the
///   library declaration, one constant line per entry in order, and the closing
///   brace, each line ending in `\n`.
/// Arrange: the license "Apache-2.0", compiler version 0.8.28, library name
///   "PairingConstants", and entries CHALLENGE_TAG of Bytes([0x43, 0x54]),
///   STATEMENT_VERSION of Uint16(1), and SECOND_GROUP_ARITHMETIC of Bool(true).
/// Act:     render over the built params and payload.
/// Assert:  the source text equals the reference library, byte for byte.
#[test]
fn render_produces_the_exact_source_of_a_reference_library() {
    // Arrange
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("Apache-2.0".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(0),
                minor: Some(8),
                patch: Some(28),
            },
        )),
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("PairingConstants".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Constants(
            build_solidity_library_entries(SolidityLibraryEntriesConstructorParamsOverrides {
                entries: Some(vec![
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("CHALLENGE_TAG".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bytes(vec![0x43, 0x54])),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("STATEMENT_VERSION".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Uint16(1)),
                    }),
                    build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some("SECOND_GROUP_ARITHMETIC".to_string()),
                            },
                        )),
                        value: Some(SolidityConstantValue::Bool(true)),
                    }),
                ]),
            }),
        )),
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);

    // Assert
    assert_eq!(
        success.source.as_str(),
        "// SPDX-License-Identifier: Apache-2.0\npragma solidity 0.8.28;\n\nlibrary PairingConstants {\n    bytes internal constant CHALLENGE_TAG = hex\"4354\";\n    uint16 internal constant STATEMENT_VERSION = 1;\n    bool internal constant SECOND_GROUP_ARITHMETIC = true;\n}\n"
    );
}

/// Contract: a text whose leading byte is an uppercase ASCII letter and whose
///   remaining bytes are ASCII letters or digits is admitted, the text moved
///   from the params.
/// Arrange: the text "G1AddRecord".
/// Act:     SolidityTypeName::try_new over the built params.
/// Assert:  the returned value equals the name built over the same text.
#[test]
fn solidity_type_name_admits_an_upper_camel_identifier() {
    // Arrange
    let params =
        build_solidity_type_name_constructor_params(SolidityTypeNameConstructorParamsOverrides {
            text: Some("G1AddRecord".to_string()),
        });

    // Act
    let Ok(value) = SolidityTypeName::try_new(params) else {
        panic!("the identifier is admitted")
    };

    // Assert
    assert_eq!(
        value,
        build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
            text: Some("G1AddRecord".to_string()),
        })
    );
}

/// Contract: empty text, a leading byte outside `A`-`Z`, and the lowest later
///   byte outside `A`-`Z`, `a`-`z`, `0`-`9` are each refused.
/// Arrange: the texts "", "g1AddRecord", and "G1_Add".
/// Act:     SolidityTypeName::try_new over each built params.
/// Assert:  the errors are Empty, LeadingCharacter { byte: b'g' }, and
///   InvalidCharacter { index: 2, byte: b'_' }.
#[test]
fn solidity_type_name_refuses_empty_text_a_lowercase_leading_byte_and_an_underscore() {
    // Arrange
    let params_of = |text: &str| {
        build_solidity_type_name_constructor_params(SolidityTypeNameConstructorParamsOverrides {
            text: Some(text.to_string()),
        })
    };

    // Act
    let Err(empty_error) = SolidityTypeName::try_new(params_of("")) else {
        panic!("the empty text is refused")
    };
    let Err(leading_error) = SolidityTypeName::try_new(params_of("g1AddRecord")) else {
        panic!("the lowercase leading byte is refused")
    };
    let Err(later_error) = SolidityTypeName::try_new(params_of("G1_Add")) else {
        panic!("the underscore is refused")
    };

    // Assert
    assert_eq!(empty_error, SolidityTypeNameTryNewErrorReturn::Empty);
    assert_eq!(
        leading_error,
        SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'g' }
    );
    assert_eq!(
        later_error,
        SolidityTypeNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'_'
        }
    );
}

/// Contract: a text whose leading byte is a lowercase ASCII letter and whose
///   remaining bytes are ASCII letters or digits is admitted, the text moved
///   from the params.
/// Arrange: the text "decodeG1Add".
/// Act:     SolidityMemberName::try_new over the built params.
/// Assert:  the returned value equals the name built over the same text.
#[test]
fn solidity_member_name_admits_a_lower_camel_identifier() {
    // Arrange
    let params = build_solidity_member_name_constructor_params(
        SolidityMemberNameConstructorParamsOverrides {
            text: Some("decodeG1Add".to_string()),
        },
    );

    // Act
    let Ok(value) = SolidityMemberName::try_new(params) else {
        panic!("the identifier is admitted")
    };

    // Assert
    assert_eq!(
        value,
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("decodeG1Add".to_string()),
        })
    );
}

/// Contract: empty text, a leading byte outside `a`-`z`, and the lowest later
///   byte outside `A`-`Z`, `a`-`z`, `0`-`9` are each refused.
/// Arrange: the texts "", "Left", and "sum_point".
/// Act:     SolidityMemberName::try_new over each built params.
/// Assert:  the errors are Empty, LeadingCharacter { byte: b'L' }, and
///   InvalidCharacter { index: 3, byte: b'_' }.
#[test]
fn solidity_member_name_refuses_empty_text_an_uppercase_leading_byte_and_an_underscore() {
    // Arrange
    let params_of = |text: &str| {
        build_solidity_member_name_constructor_params(
            SolidityMemberNameConstructorParamsOverrides {
                text: Some(text.to_string()),
            },
        )
    };

    // Act
    let Err(empty_error) = SolidityMemberName::try_new(params_of("")) else {
        panic!("the empty text is refused")
    };
    let Err(leading_error) = SolidityMemberName::try_new(params_of("Left")) else {
        panic!("the uppercase leading byte is refused")
    };
    let Err(later_error) = SolidityMemberName::try_new(params_of("sum_point")) else {
        panic!("the underscore is refused")
    };

    // Assert
    assert_eq!(empty_error, SolidityMemberNameTryNewErrorReturn::Empty);
    assert_eq!(
        leading_error,
        SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'L' }
    );
    assert_eq!(
        later_error,
        SolidityMemberNameTryNewErrorReturn::InvalidCharacter {
            index: 3,
            byte: b'_'
        }
    );
}

/// Contract: each canonical field kind maps to exactly one ABI type name.
/// Arrange: a SolidityAbiType over each of the eight CanonicalFieldKind
///   variants.
/// Act:     as_str() on each built type.
/// Assert:  FixedBytes32 yields "bytes32", Unsigned16 "uint16", Unsigned32
///   "uint32", Unsigned64 "uint64", Text "string", Bytes "bytes", FixedBytes20
///   "bytes20", and Unsigned256 "uint256".
#[test]
fn solidity_abi_type_names_each_canonical_kind() {
    // Arrange
    let abi_type_of = |kind: CanonicalFieldKind| {
        build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides { kind: Some(kind) })
    };

    // Act
    let names: Vec<&'static str> = [
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::Unsigned16,
        CanonicalFieldKind::Unsigned32,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes20,
        CanonicalFieldKind::Unsigned256,
    ]
    .into_iter()
    .map(|kind| abi_type_of(kind).as_str())
    .collect();

    // Assert
    assert_eq!(
        names,
        [
            "bytes32", "uint16", "uint32", "uint64", "string", "bytes", "bytes20", "uint256"
        ]
    );
}

/// Contract: exactly the `Text` and `Bytes` kinds decode into a local variable
///   held in `memory`.
/// Arrange: a SolidityAbiType over each of the eight CanonicalFieldKind
///   variants.
/// Act:     decodes_into_memory() on each built type.
/// Assert:  true for Text and Bytes; false for the other six kinds.
#[test]
fn solidity_abi_type_decodes_only_text_and_bytes_into_memory() {
    // Arrange
    let abi_type_of = |kind: CanonicalFieldKind| {
        build_solidity_abi_type(SolidityAbiTypeConstructorParamsOverrides { kind: Some(kind) })
    };

    // Act
    let flags: Vec<bool> = [
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::Unsigned16,
        CanonicalFieldKind::Unsigned32,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes20,
        CanonicalFieldKind::Unsigned256,
    ]
    .into_iter()
    .map(|kind| abi_type_of(kind).decodes_into_memory())
    .collect();

    // Assert
    assert_eq!(
        flags,
        [false, false, false, false, true, true, false, false]
    );
}

/// Contract: a member list holding fewer than two members is refused with the
///   count.
/// Arrange: an empty member vec and a one-member vec.
/// Act:     SolidityRecordMembers::try_new over each built params.
/// Assert:  the errors are TooFew { actual: 0 } and TooFew { actual: 1 }.
#[test]
fn solidity_record_members_refuses_fewer_than_two_members() {
    // Arrange
    let empty = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![]),
        },
    );
    let single = build_solidity_record_members_constructor_params(
        SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![build_solidity_record_member(Default::default())]),
        },
    );

    // Act
    let Err(empty_error) = SolidityRecordMembers::try_new(empty) else {
        panic!("the empty list is refused")
    };
    let Err(single_error) = SolidityRecordMembers::try_new(single) else {
        panic!("the one-member list is refused")
    };

    // Assert
    assert_eq!(
        empty_error,
        SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 0 }
    );
    assert_eq!(
        single_error,
        SolidityRecordMembersTryNewErrorReturn::TooFew { actual: 1 }
    );
}

/// Contract: the lowest-indexed member named `line`, the decode function's
///   parameter, is refused with its index.
/// Arrange: members named "left" then "line".
/// Act:     SolidityRecordMembers::try_new over the built params.
/// Assert:  the error is ReservedName { index: 1 }.
#[test]
fn solidity_record_members_refuses_a_member_named_line() {
    // Arrange
    let member_named = |text: &str| {
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
            members: Some(vec![member_named("left"), member_named("line")]),
        },
    );

    // Act
    let Err(error) = SolidityRecordMembers::try_new(params) else {
        panic!("the reserved name is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityRecordMembersTryNewErrorReturn::ReservedName { index: 1 }
    );
}

/// Contract: the lowest index whose name equals a name at a lower index is
///   refused with the lowest such earlier index and that index.
/// Arrange: members named "a", "b", and "a"; the lowest duplicate index is 2,
///   whose lowest earlier match is 0.
/// Act:     SolidityRecordMembers::try_new over the built params.
/// Assert:  the error is DuplicateName { first_index: 0, duplicate_index: 2 }.
#[test]
fn solidity_record_members_refuses_a_repeated_member_name() {
    // Arrange
    let member_named = |text: &str| {
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
                member_named("a"),
                member_named("b"),
                member_named("a"),
            ]),
        },
    );

    // Act
    let Err(error) = SolidityRecordMembers::try_new(params) else {
        panic!("the repeated name is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityRecordMembersTryNewErrorReturn::DuplicateName {
            first_index: 0,
            duplicate_index: 2
        }
    );
}

/// Contract: an empty declarations list is refused.
/// Arrange: the declarations vec is empty.
/// Act:     SolidityRecordDeclarations::try_new over the built params.
/// Assert:  the error is SolidityRecordDeclarationsTryNewErrorReturn::Empty.
#[test]
fn solidity_record_declarations_refuses_an_empty_list() {
    // Arrange
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![]),
        },
    );

    // Act
    let Err(error) = SolidityRecordDeclarations::try_new(params) else {
        panic!("the empty list is refused")
    };

    // Assert
    assert_eq!(error, SolidityRecordDeclarationsTryNewErrorReturn::Empty);
}

/// Contract: the lowest index whose struct name equals one at a lower index is
///   refused with the lowest such earlier index and that index.
/// Arrange: two declarations sharing the struct name "G1AddRecord", with
///   decode function names "decodeA" and "decodeB" and path constant names
///   "A_PATH" and "B_PATH".
/// Act:     SolidityRecordDeclarations::try_new over the built params.
/// Assert:  the error is DuplicateStructName { first_index: 0,
///   duplicate_index: 1 }.
#[test]
fn solidity_record_declarations_refuses_a_repeated_struct_name() {
    // Arrange
    let declaration = |decode: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some("G1AddRecord".to_string()),
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
                declaration("decodeA", "A_PATH"),
                declaration("decodeB", "B_PATH"),
            ]),
        },
    );

    // Act
    let Err(error) = SolidityRecordDeclarations::try_new(params) else {
        panic!("the repeated struct name is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
            first_index: 0,
            duplicate_index: 1
        }
    );
}

/// Contract: the lowest index whose decode function name equals one at a lower
///   index is refused with the lowest such earlier index and that index.
/// Arrange: two declarations with the struct names "ARecord" and "BRecord",
///   sharing the decode function name "decodeRecord", with path constant names
///   "A_PATH" and "B_PATH".
/// Act:     SolidityRecordDeclarations::try_new over the built params.
/// Assert:  the error is DuplicateDecodeFunctionName { first_index: 0,
///   duplicate_index: 1 }.
#[test]
fn solidity_record_declarations_refuses_a_repeated_decode_function_name() {
    // Arrange
    let declaration = |struct_name: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some("decodeRecord".to_string()),
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
                declaration("ARecord", "A_PATH"),
                declaration("BRecord", "B_PATH"),
            ]),
        },
    );

    // Act
    let Err(error) = SolidityRecordDeclarations::try_new(params) else {
        panic!("the repeated decode function name is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
            first_index: 0,
            duplicate_index: 1
        }
    );
}

/// Contract: the lowest index whose path constant name equals one at a lower
///   index is refused with the lowest such earlier index and that index.
/// Arrange: two declarations with the struct names "ARecord" and "BRecord" and
///   the decode function names "decodeA" and "decodeB", sharing the path
///   constant name "RECORD_PATH".
/// Act:     SolidityRecordDeclarations::try_new over the built params.
/// Assert:  the error is DuplicatePathConstantName { first_index: 0,
///   duplicate_index: 1 }.
#[test]
fn solidity_record_declarations_refuses_a_repeated_path_constant_name() {
    // Arrange
    let declaration = |struct_name: &str, decode: &str| {
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
                    text: Some("RECORD_PATH".to_string()),
                },
            )),
            ..Default::default()
        })
    };
    let params = build_solidity_record_declarations_constructor_params(
        SolidityRecordDeclarationsConstructorParamsOverrides {
            declarations: Some(vec![
                declaration("ARecord", "decodeA"),
                declaration("BRecord", "decodeB"),
            ]),
        },
    );

    // Act
    let Err(error) = SolidityRecordDeclarations::try_new(params) else {
        panic!("the repeated path constant name is refused")
    };

    // Assert
    assert_eq!(
        error,
        SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName {
            first_index: 0,
            duplicate_index: 1
        }
    );
}

/// Contract: the records body renders, per declaration, the struct, the path
///   constant, and the decode function destructuring the members' ABI types.
/// Arrange: the license "Apache-2.0", compiler version 0.8.28, library name
///   "PairingVectorRecords", and one declaration with struct name
///   "G1AddRecord", members "left" of Bytes, "scalar" of Unsigned256, and
///   "seller" of FixedBytes20, decode function "decodeG1Add", path constant
///   "G1_ADD_PATH", and path "generated/bn254/g1_add.txt".
/// Act:     render over the built params and payload.
/// Assert:  the source text equals the reference record library, byte for byte.
#[test]
fn render_produces_the_exact_source_of_a_reference_record_library() {
    // Arrange
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("Apache-2.0".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(0),
                minor: Some(8),
                patch: Some(28),
            },
        )),
    });
    let member_named = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
        struct_name: Some(build_solidity_type_name(
            SolidityTypeNameConstructorParamsOverrides {
                text: Some("G1AddRecord".to_string()),
            },
        )),
        members: Some(build_solidity_record_members(
            SolidityRecordMembersConstructorParamsOverrides {
                members: Some(vec![
                    member_named("left", CanonicalFieldKind::Bytes),
                    member_named("scalar", CanonicalFieldKind::Unsigned256),
                    member_named("seller", CanonicalFieldKind::FixedBytes20),
                ]),
            },
        )),
        decode_function_name: Some(build_solidity_member_name(
            SolidityMemberNameConstructorParamsOverrides {
                text: Some("decodeG1Add".to_string()),
            },
        )),
        path_constant_name: Some(build_solidity_constant_name(
            SolidityConstantNameConstructorParamsOverrides {
                text: Some("G1_ADD_PATH".to_string()),
            },
        )),
        path: Some(build_solidity_string_literal(
            SolidityStringLiteralConstructorParamsOverrides {
                text: Some("generated/bn254/g1_add.txt".to_string()),
            },
        )),
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("PairingVectorRecords".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);

    // Assert
    assert_eq!(
        success.source.as_str(),
        "// SPDX-License-Identifier: Apache-2.0\npragma solidity 0.8.28;\n\nlibrary PairingVectorRecords {\n    struct G1AddRecord {\n        bytes left;\n        uint256 scalar;\n        bytes20 seller;\n    }\n\n    string internal constant G1_ADD_PATH = \"generated/bn254/g1_add.txt\";\n\n    function decodeG1Add(bytes memory line) internal pure returns (G1AddRecord memory) {\n        (bytes memory left, uint256 scalar, bytes20 seller) = abi.decode(line, (bytes, uint256, bytes20));\n        return G1AddRecord({left: left, scalar: scalar, seller: seller});\n    }\n}\n"
    );
}

/// Contract: a decoded local variable carries `memory` only for the `Text` and
///   `Bytes` kinds.
/// Arrange: one declaration whose members are "a" of Text, "b" of Bytes, "c"
///   of Unsigned64, and "d" of FixedBytes32.
/// Act:     render over the built payload.
/// Assert:  the destructuring line is
///   `        (string memory a, bytes memory b, uint64 c, bytes32 d) = abi.decode(line, (string, bytes, uint64, bytes32));`.
#[test]
fn render_marks_only_bytes_and_string_locals_as_memory() {
    // Arrange
    let params = build_render_params(Default::default());
    let member_named = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
        members: Some(build_solidity_record_members(
            SolidityRecordMembersConstructorParamsOverrides {
                members: Some(vec![
                    member_named("a", CanonicalFieldKind::Text),
                    member_named("b", CanonicalFieldKind::Bytes),
                    member_named("c", CanonicalFieldKind::Unsigned64),
                    member_named("d", CanonicalFieldKind::FixedBytes32),
                ]),
            },
        )),
        ..Default::default()
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);

    // Assert
    assert!(success.source.as_str().contains(
        "        (string memory a, bytes memory b, uint64 c, bytes32 d) = abi.decode(line, (string, bytes, uint64, bytes32));"
    ));
}

/// Contract: every record declaration block after the first is preceded by one
///   empty line.
/// Arrange: two declarations with struct names "ARecord" and "BRecord", decode
///   function names "decodeA" and "decodeB", and path constant names "A_PATH"
///   and "B_PATH".
/// Act:     render over the built payload.
/// Assert:  the line after the `    }` closing decodeA is empty, the line after
///   it is `    struct BRecord {`, and the text contains no "\n\n\n".
#[test]
fn render_separates_record_declarations_by_one_empty_line() {
    // Arrange
    let params = build_render_params(Default::default());
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
    let payload = build_render_payload(RenderPayloadOverrides {
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![
                        declaration("ARecord", "decodeA", "A_PATH"),
                        declaration("BRecord", "decodeB", "B_PATH"),
                    ]),
                },
            ),
        )),
        ..Default::default()
    });

    // Act
    let Ok(success) = render(&RenderDeps, params, payload);
    let text = success.source.as_str();
    let lines: Vec<&str> = text.lines().collect();

    // Assert
    let closes: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| **line == "    }")
        .map(|(index, _)| index)
        .collect();
    let decode_a_close = closes[1];
    assert_eq!(lines[decode_a_close + 1], "");
    assert_eq!(lines[decode_a_close + 2], "    struct BRecord {");
    assert!(!text.contains("\n\n\n"));
}
