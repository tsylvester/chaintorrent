#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use serde_json;
use std::path::PathBuf;

use super::super::factory::provides::{
    GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteKind, GenerateConcreteOverrides,
    build_generate_concrete,
};
use super::super::lookup_generate_declaration::provides::lookup_generate_declaration;
use super::super::solidity::provides::{
    AdmitSolidityGenerateSettingsErrorReturn, SolidityCompilerVersionOverrides,
    SolidityGenerateSettingsDocumentCorruptions, SolidityGenerateSettingsOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, admit_solidity_generate_settings,
    build_solidity_compiler_version, build_solidity_generate_settings,
    build_spdx_license_identifier, invalidate_solidity_generate_settings_document,
};
use super::admit_generate_selection;
use super::interface::*;
use super::mock::*;

/// Contract: a document supplying `license`, `compiler_version`, and
///   `foundry_root` → `Ok` whose selection holds them.
/// Arrange: `invalidate_solidity_generate_settings_document` with `license`
///   `MIT`, `compiler_version` `0.8.40`, and `foundry_root`
///   `elsewhere/suite`, each differing from its declared default.
/// Act:    `admit_generate_selection` over `GenerateConcreteKind::Solidity`.
/// Assert: the selection equals `GenerateConcrete::Solidity` boxing
///   `build_solidity_generate_settings` with `license` from
///   `build_spdx_license_identifier` over the text `MIT`,
///   `compiler_version` from `build_solidity_compiler_version` with major
///   `0`, minor `8`, and patch `40`, and `foundry_root`
///   `PathBuf::from("elsewhere/suite")`.
/// Boundary: `admit_generate_selection`, the document guard,
///   `lookup_generate_declaration`, and `admit_solidity_generate_settings`
///   with the real name constructors.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings() {
    // Arrange
    let deps = build_admit_generate_selection_deps(
        lookup_generate_declaration,
        admit_solidity_generate_settings,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            license: Some(serde_json::json!("MIT")),
            compiler_version: Some(serde_json::json!("0.8.40")),
            foundry_root: Some(serde_json::json!("elsewhere/suite")),
            ..Default::default()
        },
    );

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_generate_selection_success_return(
            AdmitGenerateSelectionSuccessReturnOverrides {
                selection: Some(GenerateConcrete::Solidity {
                    settings: Box::new(build_solidity_generate_settings(
                        SolidityGenerateSettingsOverrides {
                            license: Some(build_spdx_license_identifier(
                                SpdxLicenseIdentifierConstructorParamsOverrides {
                                    text: Some("MIT".into()),
                                },
                            )),
                            compiler_version: Some(build_solidity_compiler_version(
                                SolidityCompilerVersionOverrides {
                                    major: Some(0),
                                    minor: Some(8),
                                    patch: Some(40),
                                },
                            )),
                            foundry_root: Some(PathBuf::from("elsewhere/suite")),
                            ..Default::default()
                        },
                    )),
                }),
            },
        ))
    );
}
// a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings

/// Contract: a `compiler_version` at the declared upper bound →
///   `Err(Solidity(Settings(OutOfBounds { setting, supplied })))`.
/// Arrange: `invalidate_solidity_generate_settings_document` with
///   `compiler_version` `0.9.0`.
/// Act:    `admit_generate_selection` over `GenerateConcreteKind::Solidity`.
/// Assert: `error` equals `AdmitGenerateSelectionErrorReturn::Solidity(
///   AdmitGenerateSelectionSolidityErrorReturn::Settings(
///   AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds { setting:
///   "compiler_version".into(), supplied: "0.9.0".into() }))`.
/// Boundary: `admit_generate_selection`, the document guard,
///   `lookup_generate_declaration`, and `admit_solidity_generate_settings`
///   with the real name constructors.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_compiler_version_outside_the_declared_bounds_is_refused_by_the_admission_in_the_settings_variant()
 {
    // Arrange
    let deps = build_admit_generate_selection_deps(
        lookup_generate_declaration,
        admit_solidity_generate_settings,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            compiler_version: Some(serde_json::json!("0.9.0")),
            ..Default::default()
        },
    );

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateSelectionErrorReturn::Solidity(
            AdmitGenerateSelectionSolidityErrorReturn::Settings(
                AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds {
                    setting: "compiler_version".into(),
                    supplied: "0.9.0".into(),
                },
            ),
        ))
    );
}
// a_compiler_version_outside_the_declared_bounds_is_refused_by_the_admission_in_the_settings_variant

/// Contract: the call over `GenerateConcreteKind::Solidity` with a document
///   that supplies nothing returns `Ok` whose selection is
///   `GenerateConcrete::Solidity` holding the declared defaults.
/// Arrange: `invalidate_solidity_generate_settings_document` with no
///   corruption.
/// Act:    `admit_generate_selection` over `GenerateConcreteKind::Solidity`.
/// Assert: the selection equals `GenerateConcrete::Solidity` boxing
///   `build_solidity_generate_settings` with `foundry_root`
///   `PathBuf::from("contracts/evm")`, `compiler_version` from
///   `build_solidity_compiler_version` with major `0`, minor `8`, and
///   patch `38`, and `license` from `build_spdx_license_identifier` over
///   the text `UNLICENSED`, each stated as an independent literal.
/// Boundary: `admit_generate_selection`, the document guard,
///   `lookup_generate_declaration`, and `admit_solidity_generate_settings`
///   with the real name constructors.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_document_supplying_nothing_selects_the_declared_defaults_admitted_by_the_real_constructors() {
    // Arrange
    let deps = build_admit_generate_selection_deps(
        lookup_generate_declaration,
        admit_solidity_generate_settings,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = invalidate_solidity_generate_settings_document(Default::default());

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_generate_selection_success_return(
            AdmitGenerateSelectionSuccessReturnOverrides {
                selection: Some(GenerateConcrete::Solidity {
                    settings: Box::new(build_solidity_generate_settings(
                        SolidityGenerateSettingsOverrides {
                            foundry_root: Some(PathBuf::from("contracts/evm")),
                            compiler_version: Some(build_solidity_compiler_version(
                                SolidityCompilerVersionOverrides {
                                    major: Some(0),
                                    minor: Some(8),
                                    patch: Some(38),
                                },
                            )),
                            license: Some(build_spdx_license_identifier(
                                SpdxLicenseIdentifierConstructorParamsOverrides {
                                    text: Some("UNLICENSED".into()),
                                },
                            )),
                            ..Default::default()
                        },
                    )),
                }),
            },
        ))
    );
}
// a_document_supplying_nothing_selects_the_declared_defaults_admitted_by_the_real_constructors

/// Contract: the settings in the returned selection hold no two kinds
///   sharing a file name, struct name, decode function name, or path
///   constant name, and the constants library and the records library
///   share neither file name nor library name.
/// Arrange: `invalidate_solidity_generate_settings_document` with no
///   corruption.
/// Act:    `admit_generate_selection` over `GenerateConcreteKind::Solidity`,
///   the settings read from the `GenerateConcrete::Solidity` variant and
///   any other variant failing the block.
/// Assert: every pair of entries of `settings.kinds` differs in its file
///   name, struct name, decode function name, and path constant name, and
///   the libraries differ in file name and library name.
/// Boundary: `admit_generate_selection`, the document guard,
///   `lookup_generate_declaration`, and `admit_solidity_generate_settings`
///   with the real name constructors.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn the_selected_default_names_are_distinct_across_the_kinds_and_the_libraries() {
    // Arrange
    let deps = build_admit_generate_selection_deps(
        lookup_generate_declaration,
        admit_solidity_generate_settings,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = invalidate_solidity_generate_settings_document(Default::default());

    // Act
    let success = admit_generate_selection(&deps, params, payload)
        .expect("the selection admits the empty document");

    // Assert
    let GenerateConcrete::Solidity { settings } = success.selection else {
        panic!("the selection is not the Solidity member")
    };
    for (outer_index, outer) in settings.kinds.iter().enumerate() {
        for (inner_index, inner) in settings.kinds.iter().enumerate() {
            if outer_index != inner_index {
                assert_ne!(outer.names.file_name, inner.names.file_name);
                assert_ne!(outer.names.struct_name, inner.names.struct_name);
                assert_ne!(
                    outer.names.decode_function_name,
                    inner.names.decode_function_name
                );
                assert_ne!(
                    outer.names.path_constant_name,
                    inner.names.path_constant_name
                );
            }
        }
    }
    assert_ne!(
        settings.constants_library.file_name,
        settings.records_library.file_name
    );
    assert_ne!(
        settings.constants_library.library_name,
        settings.records_library.library_name
    );
}
// the_selected_default_names_are_distinct_across_the_kinds_and_the_libraries

/// Contract: each member of `GENERATE_CONCRETES` with a document that
///   supplies nothing → `Ok` whose selection equals `build_generate_concrete`
///   for that member.
/// Arrange: for each identifier of `GENERATE_CONCRETES`, deps over the real
///   handles and params from their builders naming the identifier, and the
///   payload `serde_json::json!({})`.
/// Act:    `admit_generate_selection`, once per identifier.
/// Assert: the result equals `Ok` of the success return built over the
///   selection `build_generate_concrete` returns for that member.
/// Boundary: `admit_generate_selection` and the real chain it runs for each
///   member.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn every_declared_member_is_selected_at_its_declared_defaults_by_the_real_chain() {
    for &identifier in GENERATE_CONCRETES {
        // Arrange
        let deps = build_admit_generate_selection_deps(
            lookup_generate_declaration,
            admit_solidity_generate_settings,
            Default::default(),
        );
        let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
            identifier: Some(identifier),
        });
        let payload = serde_json::json!({});

        // Act
        let result = admit_generate_selection(&deps, params, payload);

        // Assert
        assert_eq!(
            result,
            Ok(build_admit_generate_selection_success_return(
                AdmitGenerateSelectionSuccessReturnOverrides {
                    selection: Some(build_generate_concrete(
                        identifier,
                        GenerateConcreteOverrides::default(),
                    )),
                },
            ))
        );
    }
}
// every_declared_member_is_selected_at_its_declared_defaults_by_the_real_chain
