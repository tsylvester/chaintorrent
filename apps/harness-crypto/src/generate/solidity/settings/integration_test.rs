#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use super::super::render::provides::{
    SolidityCompilerVersionOverrides, SolidityConstantNameConstructorParamsOverrides,
    SolidityLibraryNameConstructorParamsOverrides, SolidityMemberNameConstructorParamsOverrides,
    SolidityTypeNameConstructorParamsOverrides, SpdxLicenseIdentifierConstructorParamsOverrides,
    build_solidity_compiler_version, build_solidity_constant_name, build_solidity_library_name,
    build_solidity_member_name, build_solidity_type_name, build_spdx_license_identifier,
};
use super::admit_solidity_generate_settings;
use super::interface::*;
use super::mock::*;

/// Contract: the call over `SOLIDITY_GENERATE_DECLARATION` with the empty
///   document returns `Ok` holding one entry per declared kind in declared
///   order and each default admitted by its real constructor.
/// Arrange: `build_solidity_generate_settings_document(Default::default())`.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: the entries' kinds equal the declared kinds, each entry's names
///   equal the names `build_generated_file_name`, `build_solidity_type_name`,
///   `build_solidity_member_name`, and `build_solidity_constant_name` return
///   for the kind's default texts stated as independent literals, and the
///   libraries, root, directory, version, and license equal the defaults.
/// Boundary: `admit_solidity_generate_settings` and the real constructors of
///   `GeneratedFileName`, `GeneratedDirectory`, and the `render` name types.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn the_declared_defaults_are_admitted_by_the_real_constructors() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(Default::default())),
        },
    );

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the empty document is admitted")
        .settings;

    // Assert
    let expected: [(&str, &str, &str, &str); 14] = [
        (
            "g1_addition.txt",
            "G1AdditionVector",
            "decodeG1Addition",
            "G1_ADDITION_PATH",
        ),
        (
            "g1_multiplication.txt",
            "G1MultiplicationVector",
            "decodeG1Multiplication",
            "G1_MULTIPLICATION_PATH",
        ),
        (
            "pairing_check.txt",
            "PairingCheckVector",
            "decodePairingCheck",
            "PAIRING_CHECK_PATH",
        ),
        (
            "g2_addition.txt",
            "G2AdditionVector",
            "decodeG2Addition",
            "G2_ADDITION_PATH",
        ),
        (
            "g1_multi_scalar_multiplication.txt",
            "G1MultiScalarMultiplicationVector",
            "decodeG1MultiScalarMultiplication",
            "G1_MULTI_SCALAR_MULTIPLICATION_PATH",
        ),
        (
            "g2_multi_scalar_multiplication.txt",
            "G2MultiScalarMultiplicationVector",
            "decodeG2MultiScalarMultiplication",
            "G2_MULTI_SCALAR_MULTIPLICATION_PATH",
        ),
        (
            "decode_rejection.txt",
            "DecodeRejectionVector",
            "decodeDecodeRejection",
            "DECODE_REJECTION_PATH",
        ),
        (
            "hash_to_scalar.txt",
            "HashToScalarVector",
            "decodeHashToScalar",
            "HASH_TO_SCALAR_PATH",
        ),
        (
            "identity_mapping.txt",
            "IdentityMappingVector",
            "decodeIdentityMapping",
            "IDENTITY_MAPPING_PATH",
        ),
        (
            "possession.txt",
            "PossessionVector",
            "decodePossession",
            "POSSESSION_PATH",
        ),
        (
            "challenge.txt",
            "ChallengeVector",
            "decodeChallenge",
            "CHALLENGE_PATH",
        ),
        (
            "mint_proof.txt",
            "MintProofVector",
            "decodeMintProof",
            "MINT_PROOF_PATH",
        ),
        (
            "transfer_proof.txt",
            "TransferProofVector",
            "decodeTransferProof",
            "TRANSFER_PROOF_PATH",
        ),
        (
            "cell_values.txt",
            "CellValuesVector",
            "decodeCellValues",
            "CELL_VALUES_PATH",
        ),
    ];
    assert_eq!(settings.kinds.len(), expected.len());
    for (index, (file, struct_name, decode_function, path_constant)) in expected.iter().enumerate()
    {
        let entry = &settings.kinds[index];
        assert_eq!(entry.kind, SOLIDITY_VECTOR_KINDS[index]);
        assert_eq!(
            entry.names.file_name,
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some((*file).to_string()),
            })
        );
        assert_eq!(
            entry.names.struct_name,
            build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
                text: Some((*struct_name).to_string()),
            })
        );
        assert_eq!(
            entry.names.decode_function_name,
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some((*decode_function).to_string()),
            })
        );
        assert_eq!(
            entry.names.path_constant_name,
            build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
                text: Some((*path_constant).to_string()),
            })
        );
    }
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
    assert_eq!(settings.foundry_root, PathBuf::from("contracts/evm"));
    assert_eq!(
        settings.generated_directory,
        build_generated_directory(GeneratedDirectoryConstructorParamsOverrides {
            segments: Some(vec![build_generated_file_name(
                GeneratedFileNameConstructorParamsOverrides {
                    text: Some("generated".to_string()),
                },
            )]),
        })
    );
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
}
// the_declared_defaults_are_admitted_by_the_real_constructors

/// Contract: no two kinds share a file name, struct name, decode function
///   name, or path constant name, and the two libraries share neither file
///   name nor library name.
/// Arrange: the empty document.
/// Act:    `admit_solidity_generate_settings`.
/// Assert: every pair of entries differs in its file name, its struct name,
///   its decode function name, and its path constant name, and the libraries
///   differ in file name and library name.
/// Boundary: `admit_solidity_generate_settings` and the real constructors of
///   `GeneratedFileName`, `GeneratedDirectory`, and the `render` name types.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn the_default_names_are_distinct_across_the_kinds_and_the_libraries() {
    // Arrange
    let deps = build_admit_solidity_generate_settings_deps(
        &SOLIDITY_GENERATE_DECLARATION,
        Default::default(),
    );
    let params = build_admit_solidity_generate_settings_params(Default::default());
    let payload = build_admit_solidity_generate_settings_payload(
        AdmitSolidityGenerateSettingsPayloadOverrides {
            document: Some(build_solidity_generate_settings_document(Default::default())),
        },
    );

    // Act
    let settings = admit_solidity_generate_settings(&deps, params, payload)
        .expect("the empty document is admitted")
        .settings;

    // Assert
    for (outer_index, outer) in settings.kinds.iter().enumerate() {
        for inner in settings.kinds.iter().skip(outer_index + 1) {
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
    assert_ne!(
        settings.constants_library.file_name,
        settings.records_library.file_name
    );
    assert_ne!(
        settings.constants_library.library_name,
        settings.records_library.library_name
    );
}
// the_default_names_are_distinct_across_the_kinds_and_the_libraries
