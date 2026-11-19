#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json;

use super::super::super::factory::provides::{
    GenerateDeclarationOverrides, MOCK_FIXED_LICENSE_SETTING_STATEMENTS, VectorKind,
    build_generate_declaration,
};
use super::super::render::provides::{
    SolidityCompilerVersionOverrides, SolidityConstantNameConstructorParamsOverrides,
    SolidityConstantNameTryNewErrorReturn, SolidityLibraryNameConstructorParamsOverrides,
    SolidityLibraryNameTryNewErrorReturn, SolidityMemberNameConstructorParamsOverrides,
    SolidityMemberNameTryNewErrorReturn, SolidityTypeNameConstructorParamsOverrides,
    SolidityTypeNameTryNewErrorReturn, SpdxLicenseIdentifierConstructorParamsOverrides,
    SpdxLicenseIdentifierTryNewErrorReturn, build_solidity_compiler_version,
    build_solidity_constant_name, build_solidity_library_name, build_solidity_member_name,
    build_solidity_type_name, build_spdx_license_identifier,
};
use super::admit_solidity_generate_settings;
use super::interface::*;
use super::mock::*;

/// Contract: every byte inside the admitted set → `Ok(GeneratedFileName { text })`.
/// Arrange: `build_generated_file_name_constructor_params` with `text` `a-Z_0.txt`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: `as_str()` equals `a-Z_0.txt`.
#[test]
fn generated_file_name_admits_letters_digits_dot_underscore_and_hyphen() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("a-Z_0.txt".to_string()),
        });

    // Act
    let name = GeneratedFileName::try_new(params).expect("the admitted set is valid");

    // Assert
    assert_eq!(name.as_str(), "a-Z_0.txt");
}
// generated_file_name_admits_letters_digits_dot_underscore_and_hyphen

/// Contract: no byte → `Err(Empty)`.
/// Arrange: `text` empty.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: `error` equals `GeneratedFileNameTryNewErrorReturn::Empty`.
#[test]
fn generated_file_name_refuses_an_empty_text() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some(String::new()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Empty));
}
// generated_file_name_refuses_an_empty_text

/// Contract: the lowest index whose byte is outside the set →
///   `Err(InvalidCharacter { index, byte })`.
/// Arrange: `text` `ab c/d`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: `error` equals `InvalidCharacter { index: 2, byte: b' ' }`.
#[test]
fn generated_file_name_refuses_the_first_byte_outside_the_set() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("ab c/d".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b' ',
        })
    );
}
// generated_file_name_refuses_the_first_byte_outside_the_set

/// Contract: the text is `.` or `..` → `Err(Reserved)`.
/// Arrange: `text` `..`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: `error` equals `GeneratedFileNameTryNewErrorReturn::Reserved`.
#[test]
fn generated_file_name_refuses_the_reserved_names() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("..".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Reserved));
}
// generated_file_name_refuses_the_reserved_names

/// Contract: empty segments → `Err(Empty)`.
/// Arrange: `segments` empty.
/// Act:    `GeneratedDirectory::try_new`.
/// Assert: `error` equals `GeneratedDirectoryTryNewErrorReturn::Empty`.
#[test]
fn generated_directory_refuses_no_segments() {
    // Arrange
    let params = build_generated_directory_constructor_params(
        GeneratedDirectoryConstructorParamsOverrides {
            segments: Some(vec![]),
        },
    );

    // Act
    let result = GeneratedDirectory::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedDirectoryTryNewErrorReturn::Empty));
}
// generated_directory_refuses_no_segments

/// Contract: a non-empty list → `Ok` holding the segments in order.
/// Arrange: two segments `one` and `two` each from `build_generated_file_name`.
/// Act:    `GeneratedDirectory::try_new`.
/// Assert: the directory equals `build_generated_directory` over those two
///   segments.
#[test]
fn generated_directory_admits_its_segments_in_order() {
    // Arrange
    let segments = vec![
        build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some("one".to_string()),
        }),
        build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some("two".to_string()),
        }),
    ];
    let params = build_generated_directory_constructor_params(
        GeneratedDirectoryConstructorParamsOverrides {
            segments: Some(segments.clone()),
        },
    );

    // Act
    let result = GeneratedDirectory::try_new(params);

    // Assert
    assert_eq!(
        result,
        Ok(build_generated_directory(
            GeneratedDirectoryConstructorParamsOverrides {
                segments: Some(segments),
            },
        ))
    );
}
// generated_directory_admits_its_segments_in_order

/// Contract: `segments(&self) -> &[GeneratedFileName]` returns the segments in
///   the order `try_new` received them.
/// Arrange: `build_generated_directory` with `segments` two
///   `build_generated_file_name` values over `suite` and then `generated`, an
///   order that is neither sorted nor the builder's.
/// Act:    `segments`.
/// Assert: the returned slice equals the two names in that order.
#[test]
fn generated_directory_segments_returns_the_admitted_segments_in_order() {
    // Arrange
    let first = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("suite".to_string()),
    });
    let second = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("generated".to_string()),
    });
    let directory = build_generated_directory(GeneratedDirectoryConstructorParamsOverrides {
        segments: Some(vec![first.clone(), second.clone()]),
    });

    // Act
    let segments = directory.segments();

    // Assert
    assert_eq!(segments, [first, second].as_slice());
}
// generated_directory_segments_returns_the_admitted_segments_in_order

/// Contract: an object → `Ok` of the document with every field `None`.
/// Arrange: `serde_json::json!({})`.
/// Act:    `SolidityGenerateSettingsDocument::try_from`.
/// Assert: the result equals `Ok(build_solidity_generate_settings_document(
///   Default::default()))`.
#[test]
fn the_document_guard_admits_an_object_without_keys() {
    // Arrange
    let value = serde_json::json!({});

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Ok(build_solidity_generate_settings_document(Default::default()))
    );
}
// the_document_guard_admits_an_object_without_keys

/// Contract: every declared key with a value of its kind → `Ok` holding each.
/// Arrange: `invalidate_solidity_generate_settings_document` with `license`
///   `MIT`, `foundry_root` `root`, `compiler_version` `0.8.40`, and
///   `generated_directory` the list `a`, `b`.
/// Act:    `try_from` over it.
/// Assert: the document's `license`, `foundry_root`, `compiler_version`, and
///   `generated_directory` equal `Some` of those values.
#[test]
fn the_document_guard_admits_every_key_it_declares() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            license: Some(serde_json::json!("MIT")),
            foundry_root: Some(serde_json::json!("root")),
            compiler_version: Some(serde_json::json!("0.8.40")),
            generated_directory: Some(serde_json::json!(["a", "b"])),
            ..Default::default()
        },
    );

    // Act
    let document =
        SolidityGenerateSettingsDocument::try_from(value).expect("every declared key is admitted");

    // Assert
    assert_eq!(document.license, Some("MIT".to_string()));
    assert_eq!(document.foundry_root, Some("root".to_string()));
    assert_eq!(document.compiler_version, Some("0.8.40".to_string()));
    assert_eq!(
        document.generated_directory,
        Some(vec!["a".to_string(), "b".to_string()])
    );
}
// the_document_guard_admits_every_key_it_declares

/// Contract: not an object → `Err(NotATable { path: "" })`.
/// Arrange: `serde_json::json!(3)`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotATable { path: String::new() }`.
#[test]
fn the_document_guard_refuses_a_value_that_is_not_a_table() {
    // Arrange
    let value = serde_json::json!(3);

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotATable {
            path: String::new(),
        })
    );
}
// the_document_guard_refuses_a_value_that_is_not_a_table

/// Contract: a key outside the declared keys → `Err(UnknownKey { path: "", key })`.
/// Arrange: an object with the key `licence`.
/// Act:    `try_from`.
/// Assert: `error` equals `UnknownKey { path: String::new(), key: "licence".into() }`.
#[test]
fn the_document_guard_refuses_an_unknown_key() {
    // Arrange
    let value = serde_json::json!({ "licence": "MIT" });

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::UnknownKey {
            path: String::new(),
            key: "licence".into(),
        })
    );
}
// the_document_guard_refuses_an_unknown_key

/// Contract: a present `license` that is not a string → `Err(NotText { path: "license" })`.
/// Arrange: `invalidate_solidity_generate_settings_document` with `license` `7`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotText { path: "license".into() }`.
#[test]
fn the_document_guard_refuses_a_non_text_license() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            license: Some(serde_json::json!(7)),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotText {
            path: "license".into(),
        })
    );
}
// the_document_guard_refuses_a_non_text_license

/// Contract: a present `generated_directory` that is not an array of strings →
///   `Err(NotTextList { path })`.
/// Arrange: `generated_directory` the list `a`, `1`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotTextList { path: "generated_directory".into() }`.
#[test]
fn the_document_guard_refuses_a_directory_that_is_not_a_list_of_text() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            generated_directory: Some(serde_json::json!(["a", 1])),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotTextList {
            path: "generated_directory".into(),
        })
    );
}
// the_document_guard_refuses_a_directory_that_is_not_a_list_of_text

/// Contract: an entry of `kinds` that is not an object →
///   `Err(NotATable { path: "kinds.<key>" })`.
/// Arrange: `kinds` holding `G1Addition` as `true`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotATable { path: "kinds.G1Addition".into() }`.
#[test]
fn the_document_guard_refuses_a_kind_entry_that_is_not_a_table() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            kinds: Some(serde_json::json!({ "G1Addition": true })),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotATable {
            path: "kinds.G1Addition".into(),
        })
    );
}
// the_document_guard_refuses_a_kind_entry_that_is_not_a_table

/// Contract: a key outside `file`, `struct_name`, `decode_function`, and
///   `path_constant` → `Err(UnknownKey { path: "kinds.<key>", key })`.
/// Arrange: `kinds.G1Addition` holding the key `extension`.
/// Act:    `try_from`.
/// Assert: `error` equals `UnknownKey { path: "kinds.G1Addition".into(), key: "extension".into() }`.
#[test]
fn the_document_guard_refuses_an_unknown_key_in_a_kind_table() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            kinds: Some(serde_json::json!({
                "G1Addition": { "extension": "dat" },
            })),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::UnknownKey {
            path: "kinds.G1Addition".into(),
            key: "extension".into(),
        })
    );
}
// the_document_guard_refuses_an_unknown_key_in_a_kind_table

/// Contract: a present `file` or `library` that is not a string →
///   `Err(NotText { path })`.
/// Arrange: `constants_library` holding `library` as `1`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotText { path: "constants_library.library".into() }`.
#[test]
fn the_document_guard_refuses_a_non_text_library_name() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            constants_library: Some(serde_json::json!({ "library": 1 })),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotText {
            path: "constants_library.library".into(),
        })
    );
}
// the_document_guard_refuses_a_non_text_library_name

/// Contract: a present `kinds` that is not an object → `Err(NotATable { path: "kinds" })`.
/// Arrange: `invalidate_solidity_generate_settings_document` with `kinds` `[]`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotATable { path: "kinds".into() }`.
#[test]
fn the_document_guard_refuses_a_kinds_value_that_is_not_a_table() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            kinds: Some(serde_json::json!([])),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotATable {
            path: "kinds".into(),
        })
    );
}
// the_document_guard_refuses_a_kinds_value_that_is_not_a_table

/// Contract: a present kind field that is not a string →
///   `Err(NotText { path: "kinds.<key>.<field>" })`.
/// Arrange: `kinds.MintProof` holding `struct_name` as `false`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotText { path: "kinds.MintProof.struct_name".into() }`.
#[test]
fn the_document_guard_refuses_a_non_text_kind_field() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            kinds: Some(serde_json::json!({
                "MintProof": { "struct_name": false },
            })),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotText {
            path: "kinds.MintProof.struct_name".into(),
        })
    );
}
// the_document_guard_refuses_a_non_text_kind_field

/// Contract: a key outside `file` and `library` → `Err(UnknownKey { path, key })`.
/// Arrange: `records_library` holding the key `name`.
/// Act:    `try_from`.
/// Assert: `error` equals `UnknownKey { path: "records_library".into(), key: "name".into() }`.
#[test]
fn the_document_guard_refuses_an_unknown_key_in_a_library_table() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            records_library: Some(serde_json::json!({ "name": "Records" })),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::UnknownKey {
            path: "records_library".into(),
            key: "name".into(),
        })
    );
}
// the_document_guard_refuses_an_unknown_key_in_a_library_table

/// Contract: a present library that is not an object → `Err(NotATable { path })`.
/// Arrange: `constants_library` as `"Constants"`.
/// Act:    `try_from`.
/// Assert: `error` equals `NotATable { path: "constants_library".into() }`.
#[test]
fn the_document_guard_refuses_a_library_that_is_not_a_table() {
    // Arrange
    let value = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            constants_library: Some(serde_json::json!("Constants")),
            ..Default::default()
        },
    );

    // Act
    let result = SolidityGenerateSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(SolidityGenerateSettingsDocumentErrorReturn::NotATable {
            path: "constants_library".into(),
        })
    );
}
// the_document_guard_refuses_a_library_that_is_not_a_table

/// Contract: no supplied value → each setting's declared default.
/// Arrange: the empty document and `SOLIDITY_GENERATE_DECLARATION`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `foundry_root` equals `contracts/evm`, `compiler_version` equals
///   `0.8.38`, `license` equals the `UNLICENSED` identifier, the constants
///   library's file name `Constants.sol` and library name `Constants`, and the
///   records library's `VectorRecords.sol` and `VectorRecords`, each stated as
///   an independent literal through its builder.
#[test]
fn admission_takes_every_declared_default_when_the_document_supplies_nothing() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(Default::default());

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the empty document is admitted")
        .settings;

    // Assert
    assert_eq!(settings.foundry_root, PathBuf::from("contracts/evm"));
    assert_eq!(
        settings.compiler_version,
        build_solidity_compiler_version(SolidityCompilerVersionOverrides {
            major: Some(0),
            minor: Some(8),
            patch: Some(38),
        })
    );
    assert_eq!(
        settings.license,
        build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("UNLICENSED".to_string()),
        })
    );
    assert_eq!(
        settings.constants_library.file_name,
        build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some("Constants.sol".to_string()),
        })
    );
    assert_eq!(
        settings.constants_library.library_name,
        build_solidity_library_name(SolidityLibraryNameConstructorParamsOverrides {
            text: Some("Constants".to_string()),
        })
    );
    assert_eq!(
        settings.records_library.file_name,
        build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some("VectorRecords.sol".to_string()),
        })
    );
    assert_eq!(
        settings.records_library.library_name,
        build_solidity_library_name(SolidityLibraryNameConstructorParamsOverrides {
            text: Some("VectorRecords".to_string()),
        })
    );
}
// admission_takes_every_declared_default_when_the_document_supplies_nothing

/// Contract: one entry per declared kind in declared order, the cell's values
///   kind among them.
/// Arrange: the empty document.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: the `kind` of each entry equals the declared kinds `G1Addition`
///   through `CellValues` in order, and the first entry's names equal the
///   defaults `g1_addition.txt`, `G1AdditionVector`, `decodeG1Addition`, and
///   `G1_ADDITION_PATH` and the last's `cell_values.txt`, `CellValuesVector`,
///   `decodeCellValues`, and `CELL_VALUES_PATH`.
#[test]
fn admission_emits_one_kind_settings_per_declared_kind_in_declared_order() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(Default::default());

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the empty document is admitted")
        .settings;

    // Assert
    assert_eq!(
        settings
            .kinds
            .iter()
            .map(|entry| entry.kind)
            .collect::<Vec<VectorKind>>()
            .as_slice(),
        SOLIDITY_VECTOR_KINDS
    );
    let first = &settings.kinds[0];
    assert_eq!(
        first.names.file_name,
        build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some("g1_addition.txt".to_string()),
        })
    );
    assert_eq!(
        first.names.struct_name,
        build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
            text: Some("G1AdditionVector".to_string()),
        })
    );
    assert_eq!(
        first.names.decode_function_name,
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("decodeG1Addition".to_string()),
        })
    );
    assert_eq!(
        first.names.path_constant_name,
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some("G1_ADDITION_PATH".to_string()),
        })
    );
    let last = &settings.kinds[settings.kinds.len() - 1];
    assert_eq!(
        last.names.file_name,
        build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some("cell_values.txt".to_string()),
        })
    );
    assert_eq!(
        last.names.struct_name,
        build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
            text: Some("CellValuesVector".to_string()),
        })
    );
    assert_eq!(
        last.names.decode_function_name,
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("decodeCellValues".to_string()),
        })
    );
    assert_eq!(
        last.names.path_constant_name,
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some("CELL_VALUES_PATH".to_string()),
        })
    );
}
// admission_emits_one_kind_settings_per_declared_kind_in_declared_order

/// Contract: a supplied text for a settable setting replaces its default.
/// Arrange: a document supplying `kinds.MintProof.file` `mint.dat`, `license`
///   `MIT`, `compiler_version` `0.8.40`, and `generated_directory` the segments
///   `out` and `vectors`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: the `MintProof` entry's file name equals `mint.dat` and its other
///   three names their defaults, `license` equals `MIT`, `compiler_version`
///   equals `0.8.40`, and the directory holds `out` then `vectors`.
#[test]
fn admission_replaces_a_default_with_the_supplied_value() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    kinds: Some(Some(BTreeMap::from([(
                        "MintProof".to_string(),
                        build_solidity_vector_kind_names_document(
                            SolidityVectorKindNamesDocumentOverrides {
                                file: Some(Some("mint.dat".to_string())),
                                ..Default::default()
                            },
                        ),
                    )]))),
                    license: Some(Some("MIT".to_string())),
                    compiler_version: Some(Some("0.8.40".to_string())),
                    generated_directory: Some(Some(vec!["out".to_string(), "vectors".to_string()])),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the supplied values are admitted")
        .settings;

    // Assert
    let mint_proof = settings
        .kinds
        .iter()
        .find(|entry| entry.kind == VectorKind::MintProof)
        .expect("MintProof is a declared kind");
    assert_eq!(
        mint_proof.names.file_name,
        build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some("mint.dat".to_string()),
        })
    );
    assert_eq!(
        mint_proof.names.struct_name,
        build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
            text: Some("MintProofVector".to_string()),
        })
    );
    assert_eq!(
        mint_proof.names.decode_function_name,
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("decodeMintProof".to_string()),
        })
    );
    assert_eq!(
        mint_proof.names.path_constant_name,
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some("MINT_PROOF_PATH".to_string()),
        })
    );
    assert_eq!(
        settings.license,
        build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("MIT".to_string()),
        })
    );
    assert_eq!(
        settings.compiler_version,
        build_solidity_compiler_version(SolidityCompilerVersionOverrides {
            major: Some(0),
            minor: Some(8),
            patch: Some(40),
        })
    );
    assert_eq!(
        settings.generated_directory.segments(),
        [
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("out".to_string()),
            }),
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("vectors".to_string()),
            }),
        ]
        .as_slice()
    );
}
// admission_replaces_a_default_with_the_supplied_value

/// Contract: a `kinds` key equal to no declared kind's `Debug` text →
///   `Err(UnknownKind { text })`.
/// Arrange: a document whose `kinds` holds the key `G3Addition`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `UnknownKind { text: "G3Addition".into() }`.
#[test]
fn admission_refuses_a_kind_the_declaration_does_not_name() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    kinds: Some(Some(BTreeMap::from([(
                        "G3Addition".to_string(),
                        build_solidity_vector_kind_names_document(Default::default()),
                    )]))),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::UnknownKind {
            text: "G3Addition".into(),
        })
    );
}
// admission_refuses_a_kind_the_declaration_does_not_name

/// Contract: a fixed statement with a supplied text →
///   `Err(FixedSetting { setting, supplied })`.
/// Arrange: a declaration from `build_generate_declaration` with `settings`
///   `MOCK_FIXED_LICENSE_SETTING_STATEMENTS`, which fixes `license`, and a
///   document supplying `license` `MIT`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `FixedSetting { setting: "license".into(), supplied: "MIT".into() }`.
#[test]
fn admission_refuses_a_value_for_a_fixed_setting() {
    // Arrange
    let declaration = build_generate_declaration(GenerateDeclarationOverrides {
        settings: Some(MOCK_FIXED_LICENSE_SETTING_STATEMENTS),
        ..Default::default()
    });
    let deps = build_admit_solidity_generate_settings_deps(&declaration, Default::default());
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    license: Some(Some("MIT".to_string())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::FixedSetting {
            setting: "license".into(),
            supplied: "MIT".into(),
        })
    );
}
// admission_refuses_a_value_for_a_fixed_setting

/// Contract: a fixed statement with no supplied text → its value, which
///   differs from the declared default.
/// Arrange: the declaration of `admission_refuses_a_value_for_a_fixed_setting`
///   and the empty document.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `license` equals `build_spdx_license_identifier` with `text`
///   `Apache-2.0`.
#[test]
fn admission_takes_the_fixed_value_when_the_document_supplies_none() {
    // Arrange
    let declaration = build_generate_declaration(GenerateDeclarationOverrides {
        settings: Some(MOCK_FIXED_LICENSE_SETTING_STATEMENTS),
        ..Default::default()
    });
    let deps = build_admit_solidity_generate_settings_deps(&declaration, Default::default());
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(Default::default());

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the fixed value is admitted")
        .settings;

    // Assert
    assert_eq!(
        settings.license,
        build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("Apache-2.0".to_string()),
        })
    );
}
// admission_takes_the_fixed_value_when_the_document_supplies_none

/// Contract: a release less than `at_least` → `Err(OutOfBounds { setting, supplied })`.
/// Arrange: `compiler_version` `0.8.37`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `OutOfBounds { setting: "compiler_version".into(), supplied: "0.8.37".into() }`.
#[test]
fn admission_refuses_a_compiler_version_below_its_lower_bound() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    compiler_version: Some(Some("0.8.37".to_string())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds {
            setting: "compiler_version".into(),
            supplied: "0.8.37".into(),
        })
    );
}
// admission_refuses_a_compiler_version_below_its_lower_bound

/// Contract: a release not less than `below` → `Err(OutOfBounds)`.
/// Arrange: `compiler_version` `0.9.0`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `OutOfBounds { setting: "compiler_version".into(), supplied: "0.9.0".into() }`.
#[test]
fn admission_refuses_a_compiler_version_at_its_upper_bound() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    compiler_version: Some(Some("0.9.0".to_string())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds {
            setting: "compiler_version".into(),
            supplied: "0.9.0".into(),
        })
    );
}
// admission_refuses_a_compiler_version_at_its_upper_bound

/// Contract: a release equal to `at_least` is within the bounds.
/// Arrange: `compiler_version` `0.8.38`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `compiler_version` equals `0.8.38`.
#[test]
fn admission_admits_a_compiler_version_at_its_lower_bound() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    compiler_version: Some(Some("0.8.38".to_string())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the lower bound is admitted")
        .settings;

    // Assert
    assert_eq!(
        settings.compiler_version,
        build_solidity_compiler_version(SolidityCompilerVersionOverrides {
            major: Some(0),
            minor: Some(8),
            patch: Some(38),
        })
    );
}
// admission_admits_a_compiler_version_at_its_lower_bound

/// Contract: not three parts → `Err(CompilerVersionParts)`.
/// Arrange: `compiler_version` `0.8`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `CompilerVersionParts { setting: "compiler_version".into(), text: "0.8".into() }`.
#[test]
fn admission_refuses_a_compiler_version_without_three_parts() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    compiler_version: Some(Some("0.8".to_string())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(
            AdmitSolidityGenerateSettingsErrorReturn::CompilerVersionParts {
                setting: "compiler_version".into(),
                text: "0.8".into(),
            },
        )
    );
}
// admission_refuses_a_compiler_version_without_three_parts

/// Contract: the lowest index whose part `u16::from_str` refuses →
///   `Err(CompilerVersionPart)`.
/// Arrange: `compiler_version` `0.x.38`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `CompilerVersionPart { setting: "compiler_version".into(), index: 1, text: "x".into() }`.
#[test]
fn admission_refuses_a_compiler_version_part_that_is_not_a_number() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    compiler_version: Some(Some("0.x.38".to_string())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(
            AdmitSolidityGenerateSettingsErrorReturn::CompilerVersionPart {
                setting: "compiler_version".into(),
                index: 1,
                text: "x".into(),
            },
        )
    );
}
// admission_refuses_a_compiler_version_part_that_is_not_a_number

/// Contract: no statement of the setting's name → `Err(UndeclaredSetting)`.
/// Arrange: a declaration from `build_generate_declaration` with `kinds` and
///   `settings` at `&[]` and the empty document.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `UndeclaredSetting { setting: "constants_library.file".into() }`,
///   the first setting read when no kind is declared.
#[test]
fn admission_refuses_a_setting_the_declaration_does_not_state() {
    // Arrange
    let declaration = build_generate_declaration(Default::default());
    let deps = build_admit_solidity_generate_settings_deps(&declaration, Default::default());
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(Default::default());

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(
            AdmitSolidityGenerateSettingsErrorReturn::UndeclaredSetting {
                setting: "constants_library.file".into(),
            },
        )
    );
}
// admission_refuses_a_setting_the_declaration_does_not_state

/// Contract: `GeneratedFileName::try_new` refuses →
///   `Err(FileName { setting, error })`, the error unchanged.
/// Arrange: `kinds.G1Addition.file` `a b`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `FileName { setting: "kinds.G1Addition.file".into(), error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 1, byte: b' ' } }`.
#[test]
fn admission_carries_a_file_name_refusal_with_its_setting() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    kinds: Some(Some(BTreeMap::from([(
                        "G1Addition".to_string(),
                        build_solidity_vector_kind_names_document(
                            SolidityVectorKindNamesDocumentOverrides {
                                file: Some(Some("a b".to_string())),
                                ..Default::default()
                            },
                        ),
                    )]))),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::FileName {
            setting: "kinds.G1Addition.file".into(),
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 1,
                byte: b' ',
            },
        })
    );
}
// admission_carries_a_file_name_refusal_with_its_setting

/// Contract: `SolidityTypeName::try_new` refuses → `Err(TypeName { setting, error })`.
/// Arrange: `kinds.G2Addition.struct_name` `lower`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `TypeName { setting: "kinds.G2Addition.struct_name".into(), error:
///   SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'l' } }`.
#[test]
fn admission_carries_a_struct_name_refusal_with_its_setting() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    kinds: Some(Some(BTreeMap::from([(
                        "G2Addition".to_string(),
                        build_solidity_vector_kind_names_document(
                            SolidityVectorKindNamesDocumentOverrides {
                                struct_name: Some(Some("lower".to_string())),
                                ..Default::default()
                            },
                        ),
                    )]))),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::TypeName {
            setting: "kinds.G2Addition.struct_name".into(),
            error: SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'l' },
        })
    );
}
// admission_carries_a_struct_name_refusal_with_its_setting

/// Contract: `SolidityMemberName::try_new` refuses →
///   `Err(MemberName { setting, error })`.
/// Arrange: `kinds.Possession.decode_function` `Decode`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `MemberName { setting: "kinds.Possession.decode_function".into(),
///   error: SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'D' } }`.
#[test]
fn admission_carries_a_decode_function_name_refusal_with_its_setting() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    kinds: Some(Some(BTreeMap::from([(
                        "Possession".to_string(),
                        build_solidity_vector_kind_names_document(
                            SolidityVectorKindNamesDocumentOverrides {
                                decode_function: Some(Some("Decode".to_string())),
                                ..Default::default()
                            },
                        ),
                    )]))),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::MemberName {
            setting: "kinds.Possession.decode_function".into(),
            error: SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'D' },
        })
    );
}
// admission_carries_a_decode_function_name_refusal_with_its_setting

/// Contract: `SolidityConstantName::try_new` refuses →
///   `Err(ConstantName { setting, error })`.
/// Arrange: `kinds.Challenge.path_constant` `lower_path`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `ConstantName { setting: "kinds.Challenge.path_constant".into(),
///   error: SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: b'l' } }`.
#[test]
fn admission_carries_a_path_constant_name_refusal_with_its_setting() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    kinds: Some(Some(BTreeMap::from([(
                        "Challenge".to_string(),
                        build_solidity_vector_kind_names_document(
                            SolidityVectorKindNamesDocumentOverrides {
                                path_constant: Some(Some("lower_path".to_string())),
                                ..Default::default()
                            },
                        ),
                    )]))),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::ConstantName {
            setting: "kinds.Challenge.path_constant".into(),
            error: SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: b'l' },
        })
    );
}
// admission_carries_a_path_constant_name_refusal_with_its_setting

/// Contract: `GeneratedFileName::try_new` refuses a library's `file` →
///   `Err(FileName { setting, error })`.
/// Arrange: `constants_library.file` `..`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `FileName { setting: "constants_library.file".into(), error:
///   GeneratedFileNameTryNewErrorReturn::Reserved }`.
#[test]
fn admission_carries_a_library_file_name_refusal_with_its_setting() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    constants_library: Some(Some(build_solidity_library_names_document(
                        SolidityLibraryNamesDocumentOverrides {
                            file: Some(Some("..".to_string())),
                            ..Default::default()
                        },
                    ))),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::FileName {
            setting: "constants_library.file".into(),
            error: GeneratedFileNameTryNewErrorReturn::Reserved,
        })
    );
}
// admission_carries_a_library_file_name_refusal_with_its_setting

/// Contract: a supplied `foundry_root` text → `PathBuf::from` of it.
/// Arrange: `foundry_root` `elsewhere/suite`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `foundry_root` equals `PathBuf::from("elsewhere/suite")`.
#[test]
fn admission_takes_the_supplied_foundry_root_as_a_path() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    foundry_root: Some(Some("elsewhere/suite".to_string())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the supplied root is admitted")
        .settings;

    // Assert
    assert_eq!(settings.foundry_root, PathBuf::from("elsewhere/suite"));
}
// admission_takes_the_supplied_foundry_root_as_a_path

/// Contract: `SolidityLibraryName::try_new` refuses →
///   `Err(LibraryName { setting, error })`.
/// Arrange: `records_library.library` empty.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `LibraryName { setting: "records_library.library".into(), error:
///   SolidityLibraryNameTryNewErrorReturn::Empty }`.
#[test]
fn admission_carries_a_library_name_refusal_with_its_setting() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    records_library: Some(Some(build_solidity_library_names_document(
                        SolidityLibraryNamesDocumentOverrides {
                            library: Some(Some(String::new())),
                            ..Default::default()
                        },
                    ))),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::LibraryName {
            setting: "records_library.library".into(),
            error: SolidityLibraryNameTryNewErrorReturn::Empty,
        })
    );
}
// admission_carries_a_library_name_refusal_with_its_setting

/// Contract: the lowest index whose segment `GeneratedFileName::try_new`
///   refuses → `Err(DirectorySegment { index, error })`.
/// Arrange: `generated_directory` the segments `ok` and `bad name`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `DirectorySegment { index: 1, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 3, byte: b' ' } }`.
#[test]
fn admission_refuses_the_first_directory_segment_the_constructor_refuses() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    generated_directory: Some(Some(vec!["ok".to_string(), "bad name".to_string()])),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::DirectorySegment {
            index: 1,
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 3,
                byte: b' ',
            },
        },)
    );
}
// admission_refuses_the_first_directory_segment_the_constructor_refuses

/// Contract: no segments → `Err(Directory { error })`.
/// Arrange: `generated_directory` the empty list.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `Directory { error: GeneratedDirectoryTryNewErrorReturn::Empty }`.
#[test]
fn admission_refuses_an_empty_directory_list() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    generated_directory: Some(Some(vec![])),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::Directory {
            error: GeneratedDirectoryTryNewErrorReturn::Empty,
        })
    );
}
// admission_refuses_an_empty_directory_list

/// Contract: `SpdxLicenseIdentifier::try_new` refuses → `Err(License { error })`.
/// Arrange: `license` empty.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` equals `License { error: SpdxLicenseIdentifierTryNewErrorReturn::Empty }`.
#[test]
fn admission_carries_a_license_refusal() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    license: Some(Some(String::new())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::License {
            error: SpdxLicenseIdentifierTryNewErrorReturn::Empty,
        })
    );
}
// admission_carries_a_license_refusal

/// Contract: the kinds, then the libraries, then the license are read in that
///   order and the first refusal returns.
/// Arrange: `kinds.G1Addition.file` `a b` and `license` empty.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: `error` is the file name refusal and not the license refusal.
#[test]
fn admission_returns_the_first_refusal_in_reading_order() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(
                SolidityGenerateSettingsDocumentOverrides {
                    kinds: Some(Some(BTreeMap::from([(
                        "G1Addition".to_string(),
                        build_solidity_vector_kind_names_document(
                            SolidityVectorKindNamesDocumentOverrides {
                                file: Some(Some("a b".to_string())),
                                ..Default::default()
                            },
                        ),
                    )]))),
                    license: Some(Some(String::new())),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let result = admit_solidity_generate_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitSolidityGenerateSettingsErrorReturn::FileName {
            setting: "kinds.G1Addition.file".into(),
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 1,
                byte: b' ',
            },
        })
    );
}
// admission_returns_the_first_refusal_in_reading_order
