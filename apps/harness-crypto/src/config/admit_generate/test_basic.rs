#![allow(clippy::expect_used)]

use pairing::PairingCurve;
use std::collections::BTreeMap;
use std::path::PathBuf;

use super::super::layers::provides::{
    GenerateKindTableOverrides, GenerateTableOverrides, LibraryTableOverrides,
    build_generate_kind_table, build_generate_table, build_library_table,
};
use super::admit_generate;
use super::mock::{
    AdmitGeneratePayloadOverrides, AdmittedGenerateOverrides, build_admit_generate_deps,
    build_admit_generate_params, build_admit_generate_payload, build_admitted_generate,
    declared_name,
};
use crate::generate::provides::{
    GENERATE_CONCRETES, GenerateConcrete, GeneratedDirectoryConstructorParamsOverrides,
    GeneratedFileNameConstructorParamsOverrides, MsmTermCountConstructorParamsOverrides,
    SolidityCompilerVersionOverrides, SolidityConstantNameConstructorParamsOverrides,
    SolidityCurveDirectoriesOverrides, SolidityGenerateSettingsOverrides,
    SolidityLibraryNameConstructorParamsOverrides, SolidityLibraryNamesOverrides,
    SolidityMemberNameConstructorParamsOverrides, SolidityTypeNameConstructorParamsOverrides,
    SolidityVectorKindNamesOverrides, SolidityVectorKindSettingsOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    VectorKind, VectorKindConfiguration, VectorKindQuantities, build_generated_directory,
    build_generated_file_name, build_msm_term_count, build_solidity_compiler_version,
    build_solidity_constant_name, build_solidity_curve_directories,
    build_solidity_generate_settings, build_solidity_library_name, build_solidity_library_names,
    build_solidity_member_name, build_solidity_type_name, build_solidity_vector_kind_names,
    build_solidity_vector_kind_settings, build_spdx_license_identifier, build_vector_count,
};

/// Contract: Success → a payload whose every text and number the builders
///   admit admits to the generate settings and kind configuration built from
///   the same values.
/// Arrange: `build_admit_generate_payload(Default::default())`.
/// Act:     `admit_generate`, once.
/// Assert:  the result is `Ok` and its `admitted` equals
///   `build_admitted_generate(Default::default())`.
#[test]
fn admit_generate_admits_the_default_payload() {
    // Arrange
    let payload = build_admit_generate_payload(Default::default());
    let expected = build_admitted_generate(Default::default());
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result.expect("the admission succeeds").admitted, expected);
    // admit_generate_admits_the_default_payload
}

/// Contract: Concrete → the text of the first member of `GENERATE_CONCRETES`
///   admits to `GenerateConcrete::Solidity`.
/// Arrange: the default payload.
/// Act:     `admit_generate`, once.
/// Assert:  `admitted.concrete` matches `GenerateConcrete::Solidity { .. }`.
#[test]
fn admit_generate_admits_the_concrete_the_text_names() {
    // Arrange
    let payload = build_admit_generate_payload(Default::default());
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert!(matches!(
        admitted.concrete,
        GenerateConcrete::Solidity { .. }
    ));
    // admit_generate_admits_the_concrete_the_text_names
}

/// Contract: Kinds → each entry admitted, the settings and the configuration
///   holding their entries in the map's key order.
/// Arrange: two `kinds` entries keyed `declared_name(&VectorKind::HashToScalar)`
///   and `declared_name(&VectorKind::G1Addition)` with distinct names and
///   counts, inserted in that order, and the expected settings and
///   configuration built in key order, `G1Addition` first.
/// Act:     `admit_generate`, once.
/// Assert:  `admitted.kinds` equals the two `VectorKindConfiguration`s in key
///   order and the concrete's `settings.kinds` equals the two
///   `SolidityVectorKindSettings` in that order, each with the names its entry
///   holds.
#[test]
fn admit_generate_admits_each_kind_in_key_order() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            kinds: Some(BTreeMap::from([
                (
                    declared_name(&VectorKind::HashToScalar),
                    build_generate_kind_table(GenerateKindTableOverrides {
                        count: Some(Some(3)),
                        file: Some("hash.vectors".to_string()),
                        struct_name: Some("HashVector".to_string()),
                        decode_function: Some("decodeHash".to_string()),
                        path_constant: Some("HASH_PATH".to_string()),
                        ..Default::default()
                    }),
                ),
                (
                    declared_name(&VectorKind::G1Addition),
                    build_generate_kind_table(GenerateKindTableOverrides {
                        count: Some(Some(2)),
                        terms: Some(None),
                        message_length: Some(None),
                        file: Some("g1_add.vectors".to_string()),
                        struct_name: Some("G1AddVector".to_string()),
                        decode_function: Some("decodeG1Add".to_string()),
                        path_constant: Some("G1_ADD_PATH".to_string()),
                    }),
                ),
            ])),
            ..Default::default()
        })),
    });
    let expected_kinds = vec![
        VectorKindConfiguration {
            kind: VectorKind::G1Addition,
            quantities: VectorKindQuantities {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(2),
                })),
                terms: None,
                message_length: None,
            },
        },
        VectorKindConfiguration {
            kind: VectorKind::HashToScalar,
            quantities: VectorKindQuantities {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(3),
                })),
                terms: None,
                message_length: None,
            },
        },
    ];
    let expected_concrete = GenerateConcrete::Solidity {
        settings: Box::new(build_solidity_generate_settings(
            SolidityGenerateSettingsOverrides {
                kinds: Some(vec![
                    build_solidity_vector_kind_settings(SolidityVectorKindSettingsOverrides {
                        kind: Some(VectorKind::G1Addition),
                        ..Default::default()
                    }),
                    build_solidity_vector_kind_settings(SolidityVectorKindSettingsOverrides {
                        kind: Some(VectorKind::HashToScalar),
                        names: Some(build_solidity_vector_kind_names(
                            SolidityVectorKindNamesOverrides {
                                file_name: Some(build_generated_file_name(
                                    GeneratedFileNameConstructorParamsOverrides {
                                        text: Some("hash.vectors".to_string()),
                                    },
                                )),
                                struct_name: Some(build_solidity_type_name(
                                    SolidityTypeNameConstructorParamsOverrides {
                                        text: Some("HashVector".to_string()),
                                    },
                                )),
                                decode_function_name: Some(build_solidity_member_name(
                                    SolidityMemberNameConstructorParamsOverrides {
                                        text: Some("decodeHash".to_string()),
                                    },
                                )),
                                path_constant_name: Some(build_solidity_constant_name(
                                    SolidityConstantNameConstructorParamsOverrides {
                                        text: Some("HASH_PATH".to_string()),
                                    },
                                )),
                            },
                        )),
                    }),
                ]),
                foundry_root: Some(PathBuf::from("root")),
                ..Default::default()
            },
        )),
    };
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.kinds, expected_kinds);
    assert_eq!(admitted.concrete, expected_concrete);
    // admit_generate_admits_each_kind_in_key_order
}

/// Contract: Count, Terms, and Message length → a quantity present is
///   admitted and a quantity absent stays absent.
/// Arrange: three entries: `count` `Some(3)` and `terms` `Some(4)`;
///   `count` `Some(5)` and `message_length` `Some(32)`; and none of the three.
/// Act:     `admit_generate`, once.
/// Assert:  each `VectorKindConfiguration`'s quantities equal
///   `VectorKindQuantities` built from `build_vector_count` and
///   `build_msm_term_count` over those numbers and the message length as
///   given, and `None` where the entry holds none.
#[test]
fn admit_generate_carries_the_quantities_each_entry_holds() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            kinds: Some(BTreeMap::from([
                (
                    declared_name(&VectorKind::DecodeRejection),
                    build_generate_kind_table(GenerateKindTableOverrides {
                        count: Some(None),
                        terms: Some(None),
                        message_length: Some(None),
                        file: Some("rejection.vectors".to_string()),
                        struct_name: Some("RejectionVector".to_string()),
                        decode_function: Some("decodeRejection".to_string()),
                        path_constant: Some("REJECTION_PATH".to_string()),
                    }),
                ),
                (
                    declared_name(&VectorKind::G1MultiScalarMultiplication),
                    build_generate_kind_table(GenerateKindTableOverrides {
                        count: Some(Some(3)),
                        terms: Some(Some(4)),
                        message_length: Some(None),
                        file: Some("msm.vectors".to_string()),
                        struct_name: Some("MsmVector".to_string()),
                        decode_function: Some("decodeMsm".to_string()),
                        path_constant: Some("MSM_PATH".to_string()),
                    }),
                ),
                (
                    declared_name(&VectorKind::HashToScalar),
                    build_generate_kind_table(GenerateKindTableOverrides {
                        count: Some(Some(5)),
                        terms: Some(None),
                        message_length: Some(Some(32)),
                        file: Some("hash.vectors".to_string()),
                        struct_name: Some("HashVector".to_string()),
                        decode_function: Some("decodeHash".to_string()),
                        path_constant: Some("HASH_PATH".to_string()),
                    }),
                ),
            ])),
            ..Default::default()
        })),
    });
    let expected_kinds = vec![
        VectorKindConfiguration {
            kind: VectorKind::DecodeRejection,
            quantities: VectorKindQuantities {
                count: None,
                terms: None,
                message_length: None,
            },
        },
        VectorKindConfiguration {
            kind: VectorKind::G1MultiScalarMultiplication,
            quantities: VectorKindQuantities {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(3),
                })),
                terms: Some(build_msm_term_count(
                    MsmTermCountConstructorParamsOverrides { count: Some(4) },
                )),
                message_length: None,
            },
        },
        VectorKindConfiguration {
            kind: VectorKind::HashToScalar,
            quantities: VectorKindQuantities {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(5),
                })),
                terms: None,
                message_length: Some(32),
            },
        },
    ];
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.kinds, expected_kinds);
    // admit_generate_carries_the_quantities_each_entry_holds
}

/// Contract: Constants library and Records library → each library table
///   admitted to its `SolidityLibraryNames`.
/// Arrange: `constants_library` of `"Constants.sol"` and `"ConstantsLib"`,
///   and `records_library` of `"Records.sol"` and `"RecordsLib"`.
/// Act:     `admit_generate`, once.
/// Assert:  the concrete's `settings.constants_library` and
///   `settings.records_library` equal `build_solidity_library_names` over
///   those names.
#[test]
fn admit_generate_admits_the_library_names() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            constants_library: Some(build_library_table(LibraryTableOverrides {
                file: Some("Constants.sol".to_string()),
                library: Some("ConstantsLib".to_string()),
            })),
            records_library: Some(build_library_table(LibraryTableOverrides {
                file: Some("Records.sol".to_string()),
                library: Some("RecordsLib".to_string()),
            })),
            ..Default::default()
        })),
    });
    let expected = GenerateConcrete::Solidity {
        settings: Box::new(build_solidity_generate_settings(
            SolidityGenerateSettingsOverrides {
                kinds: Some(vec![build_solidity_vector_kind_settings(
                    SolidityVectorKindSettingsOverrides::default(),
                )]),
                constants_library: Some(build_solidity_library_names(
                    SolidityLibraryNamesOverrides {
                        file_name: Some(build_generated_file_name(
                            GeneratedFileNameConstructorParamsOverrides {
                                text: Some("Constants.sol".to_string()),
                            },
                        )),
                        library_name: Some(build_solidity_library_name(
                            SolidityLibraryNameConstructorParamsOverrides {
                                text: Some("ConstantsLib".to_string()),
                            },
                        )),
                    },
                )),
                records_library: Some(build_solidity_library_names(
                    SolidityLibraryNamesOverrides {
                        file_name: Some(build_generated_file_name(
                            GeneratedFileNameConstructorParamsOverrides {
                                text: Some("Records.sol".to_string()),
                            },
                        )),
                        library_name: Some(build_solidity_library_name(
                            SolidityLibraryNameConstructorParamsOverrides {
                                text: Some("RecordsLib".to_string()),
                            },
                        )),
                    },
                )),
                foundry_root: Some(PathBuf::from("root")),
                ..Default::default()
            },
        )),
    };
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.concrete, expected);
    // admit_generate_admits_the_library_names
}

/// Contract: Generated directory → the segments admitted in the document's
///   order.
/// Arrange: `generated_directory` of `"generated"` and `"nested"`.
/// Act:     `admit_generate`, once.
/// Assert:  `settings.generated_directory` equals
///   `build_generated_directory` over those two segments in that order.
#[test]
fn admit_generate_admits_the_generated_directory_segments_in_order() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            generated_directory: Some(vec!["generated".to_string(), "nested".to_string()]),
            ..Default::default()
        })),
    });
    let expected = GenerateConcrete::Solidity {
        settings: Box::new(build_solidity_generate_settings(
            SolidityGenerateSettingsOverrides {
                kinds: Some(vec![build_solidity_vector_kind_settings(
                    SolidityVectorKindSettingsOverrides::default(),
                )]),
                generated_directory: Some(build_generated_directory(
                    GeneratedDirectoryConstructorParamsOverrides {
                        segments: Some(vec![
                            build_generated_file_name(
                                GeneratedFileNameConstructorParamsOverrides {
                                    text: Some("generated".to_string()),
                                },
                            ),
                            build_generated_file_name(
                                GeneratedFileNameConstructorParamsOverrides {
                                    text: Some("nested".to_string()),
                                },
                            ),
                        ]),
                    },
                )),
                foundry_root: Some(PathBuf::from("root")),
                ..Default::default()
            },
        )),
    };
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.concrete, expected);
    // admit_generate_admits_the_generated_directory_segments_in_order
}

/// Contract: Curve directories → the entry keyed for a curve reaches the
///   field that curve names.
/// Arrange: `curve_directories` keyed `declared_name(&PairingCurve::Bn254)`
///   to `"directory-a"` and `declared_name(&PairingCurve::Bls12381)` to
///   `"directory-b"`.
/// Act:     `admit_generate`, once.
/// Assert:  `settings.curve_directories` equals
///   `build_solidity_curve_directories` with `bn254` `"directory-a"` and
///   `bls12_381` `"directory-b"`.
#[test]
fn admit_generate_admits_each_curve_directory_to_the_field_its_curve_names() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            curve_directories: Some(BTreeMap::from([
                (
                    declared_name(&PairingCurve::Bn254),
                    "directory-a".to_string(),
                ),
                (
                    declared_name(&PairingCurve::Bls12381),
                    "directory-b".to_string(),
                ),
            ])),
            ..Default::default()
        })),
    });
    let expected = GenerateConcrete::Solidity {
        settings: Box::new(build_solidity_generate_settings(
            SolidityGenerateSettingsOverrides {
                kinds: Some(vec![build_solidity_vector_kind_settings(
                    SolidityVectorKindSettingsOverrides::default(),
                )]),
                curve_directories: Some(build_solidity_curve_directories(
                    SolidityCurveDirectoriesOverrides {
                        bn254: Some(build_generated_file_name(
                            GeneratedFileNameConstructorParamsOverrides {
                                text: Some("directory-a".to_string()),
                            },
                        )),
                        bls12_381: Some(build_generated_file_name(
                            GeneratedFileNameConstructorParamsOverrides {
                                text: Some("directory-b".to_string()),
                            },
                        )),
                    },
                )),
                foundry_root: Some(PathBuf::from("root")),
                ..Default::default()
            },
        )),
    };
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.concrete, expected);
    // admit_generate_admits_each_curve_directory_to_the_field_its_curve_names
}

/// Contract: Compiler version → three decimal parts admitted in order.
/// Arrange: `compiler_version` `"1.20.300"`.
/// Act:     `admit_generate`, once.
/// Assert:  `settings.compiler_version` equals
///   `build_solidity_compiler_version` with `major` `1`, `minor` `20`, and
///   `patch` `300`.
#[test]
fn admit_generate_parses_the_compiler_version_into_its_parts() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            compiler_version: Some("1.20.300".to_string()),
            ..Default::default()
        })),
    });
    let expected = GenerateConcrete::Solidity {
        settings: Box::new(build_solidity_generate_settings(
            SolidityGenerateSettingsOverrides {
                kinds: Some(vec![build_solidity_vector_kind_settings(
                    SolidityVectorKindSettingsOverrides::default(),
                )]),
                compiler_version: Some(build_solidity_compiler_version(
                    SolidityCompilerVersionOverrides {
                        major: Some(1),
                        minor: Some(20),
                        patch: Some(300),
                    },
                )),
                foundry_root: Some(PathBuf::from("root")),
                ..Default::default()
            },
        )),
    };
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.concrete, expected);
    // admit_generate_parses_the_compiler_version_into_its_parts
}

/// Contract: License → the text admitted as an SPDX license identifier.
/// Arrange: `license` `"GPL-3.0+"`.
/// Act:     `admit_generate`, once.
/// Assert:  `settings.license` equals `build_spdx_license_identifier` over
///   `"GPL-3.0+"`.
#[test]
fn admit_generate_admits_the_license_text() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            license: Some("GPL-3.0+".to_string()),
            ..Default::default()
        })),
    });
    let expected = GenerateConcrete::Solidity {
        settings: Box::new(build_solidity_generate_settings(
            SolidityGenerateSettingsOverrides {
                kinds: Some(vec![build_solidity_vector_kind_settings(
                    SolidityVectorKindSettingsOverrides::default(),
                )]),
                license: Some(build_spdx_license_identifier(
                    SpdxLicenseIdentifierConstructorParamsOverrides {
                        text: Some("GPL-3.0+".to_string()),
                    },
                )),
                foundry_root: Some(PathBuf::from("root")),
                ..Default::default()
            },
        )),
    };
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.concrete, expected);
    // admit_generate_admits_the_license_text
}

/// Contract: Success → the root is carried and not admitted against the
///   filesystem.
/// Arrange: `foundry_root` `PathBuf::from("a path that names no directory")`.
/// Act:     `admit_generate`, once.
/// Assert:  `settings.foundry_root` equals that path.
#[test]
fn admit_generate_carries_the_foundry_root_as_the_document_holds_it() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            foundry_root: Some(PathBuf::from("a path that names no directory")),
            ..Default::default()
        })),
    });
    let expected = GenerateConcrete::Solidity {
        settings: Box::new(build_solidity_generate_settings(
            SolidityGenerateSettingsOverrides {
                kinds: Some(vec![build_solidity_vector_kind_settings(
                    SolidityVectorKindSettingsOverrides::default(),
                )]),
                foundry_root: Some(PathBuf::from("a path that names no directory")),
                ..Default::default()
            },
        )),
    };
    // Act
    let admitted = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.concrete, expected);
    // admit_generate_carries_the_foundry_root_as_the_document_holds_it
}

/// Contract: Kinds → the function admits each kind it is given, and the kind
///   set is the generate factory's to admit.
/// Arrange: `kinds` empty.
/// Act:     `admit_generate`, once.
/// Assert:  the result is `Ok` and both `admitted.kinds` and `settings.kinds`
///   are empty.
#[test]
fn admit_generate_admits_a_document_naming_no_kinds() {
    // Arrange
    let payload = build_admit_generate_payload(AdmitGeneratePayloadOverrides {
        generate: Some(build_generate_table(GenerateTableOverrides {
            concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
            kinds: Some(BTreeMap::new()),
            ..Default::default()
        })),
    });
    let expected = build_admitted_generate(AdmittedGenerateOverrides {
        concrete: Some(GenerateConcrete::Solidity {
            settings: Box::new(build_solidity_generate_settings(
                SolidityGenerateSettingsOverrides {
                    kinds: Some(vec![]),
                    foundry_root: Some(PathBuf::from("root")),
                    ..Default::default()
                },
            )),
        }),
        kinds: Some(vec![]),
    });
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result.expect("the admission succeeds").admitted, expected);
    // admit_generate_admits_a_document_naming_no_kinds
}
