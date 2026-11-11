#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use pairing::{PAIRING_CURVES, PairingCurve};

use harness_crypto::{
    AdmitGenerateErrorReturn, CurveLibrariesTableOverrides, GenerateConcrete,
    GenerateKindTableOverrides, GeneratedFileNameConstructorParamsOverrides, LibraryTableOverrides,
    MockIGenerateAdapterFailureMode, MsmTermCountConstructorParamsOverrides,
    SOLIDITY_GENERATE_DECLARATION, SolidityConstantNameConstructorParamsOverrides,
    SolidityCurveDirectoriesOverrides, SolidityCurveLibrariesOverrides,
    SolidityCurveLibraryNamesOverrides, SolidityGenerateSettingsOverrides,
    SolidityLibraryNameConstructorParamsOverrides, SolidityLibraryNamesOverrides,
    SolidityMemberNameConstructorParamsOverrides, SolidityTypeNameConstructorParamsOverrides,
    SolidityVectorKindNamesOverrides, SolidityVectorKindSettingsOverrides,
    VectorCountConstructorParamsOverrides, VectorKindConfiguration, VectorKindQuantities,
    VectorKindSizing, admit_generate, build_admit_generate_deps, build_admit_generate_params,
    build_admit_generate_payload, build_curve_libraries_table, build_generate_kind_table,
    build_generated_file_name, build_library_table, build_msm_term_count,
    build_solidity_constant_name, build_solidity_curve_directories, build_solidity_curve_libraries,
    build_solidity_curve_library_names, build_solidity_generate_settings,
    build_solidity_library_name, build_solidity_library_names, build_solidity_member_name,
    build_solidity_type_name, build_solidity_vector_kind_names,
    build_solidity_vector_kind_settings, build_vector_count, declared_name,
};

/// Contract: own entry `every_declared_kind_is_admitted_to_its_own_settings_and_configuration`
///   — `admit_generate` over a document whose `kinds` names one kind of
///   `SOLIDITY_GENERATE_DECLARATION.kinds` with the quantities its declared
///   sizing takes and names distinct from every other kind's returns `Ok`
///   whose `admitted.kinds` holds the `VectorKindConfiguration` of that kind
///   with the quantities the document holds and whose settings hold that
///   kind's `SolidityVectorKindSettings` with the names the document holds,
///   as an outside caller observes it.
/// Arrange: for each `VectorKindDeclaration` of
///   `SOLIDITY_GENERATE_DECLARATION.kinds` a payload from
///   `build_admit_generate_payload` whose single `kinds` entry is keyed
///   `declared_name` of the declaration's `kind`, carrying names distinct
///   from the default's and quantities chosen by an exhaustive `match` on
///   the declaration's `sizing`, and the expected configuration and settings
///   built from the generate family's builders over the same values.
/// Act:     `admit_generate` through the crate root, once per payload.
/// Assert:  the result is `Ok` and the admitted values equal the expected
///   values.
/// Boundary: the public call `admit_generate` and the real declaration.
/// Mocked:  none.
#[test]
fn every_declared_kind_is_admitted_to_its_own_settings_and_configuration() {
    // Arrange — the deps and params are invariant across every declaration.
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());

    // Act / Assert — every `VectorKindDeclaration` in turn.
    for declaration in SOLIDITY_GENERATE_DECLARATION.kinds {
        let kind_name = declared_name(&declaration.kind);
        let file_text = format!("{}.vectors", kind_name.to_lowercase());
        let struct_text = format!("{}Vector", kind_name);
        let decode_text = format!("decode{}", kind_name);
        let path_text = format!("{}_PATH", kind_name.to_uppercase());
        let (count, terms, message_length) = match declaration.sizing {
            VectorKindSizing::Drawn => (Some(2), None, None),
            VectorKindSizing::DrawnTerms => (Some(2), Some(4), None),
            VectorKindSizing::DrawnMessages => (Some(2), None, Some(32)),
            VectorKindSizing::Enumerated | VectorKindSizing::OverConfiguredInputs => {
                (None, None, None)
            }
        };
        let mut payload = build_admit_generate_payload(Default::default());
        payload.generate.kinds = BTreeMap::from([(
            kind_name,
            build_generate_kind_table(GenerateKindTableOverrides {
                count: Some(count),
                terms: Some(terms),
                message_length: Some(message_length),
                file: Some(file_text.clone()),
                struct_name: Some(struct_text.clone()),
                decode_function: Some(decode_text.clone()),
                path_constant: Some(path_text.clone()),
            }),
        )]);
        let expected_configuration = VectorKindConfiguration {
            kind: declaration.kind,
            quantities: VectorKindQuantities {
                count: count.map(|count| {
                    build_vector_count(VectorCountConstructorParamsOverrides { count: Some(count) })
                }),
                terms: terms.map(|terms| {
                    build_msm_term_count(MsmTermCountConstructorParamsOverrides {
                        count: Some(terms),
                    })
                }),
                message_length,
            },
        };
        let expected_settings =
            build_solidity_vector_kind_settings(SolidityVectorKindSettingsOverrides {
                kind: Some(declaration.kind),
                names: Some(build_solidity_vector_kind_names(
                    SolidityVectorKindNamesOverrides {
                        file_name: Some(build_generated_file_name(
                            GeneratedFileNameConstructorParamsOverrides {
                                text: Some(file_text),
                            },
                        )),
                        struct_name: Some(build_solidity_type_name(
                            SolidityTypeNameConstructorParamsOverrides {
                                text: Some(struct_text),
                            },
                        )),
                        decode_function_name: Some(build_solidity_member_name(
                            SolidityMemberNameConstructorParamsOverrides {
                                text: Some(decode_text),
                            },
                        )),
                        path_constant_name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some(path_text),
                            },
                        )),
                    },
                )),
            });

        let admitted = admit_generate(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;

        assert_eq!(admitted.kinds, vec![expected_configuration]);
        assert_eq!(
            admitted.concrete,
            GenerateConcrete::Solidity {
                settings: Box::new(build_solidity_generate_settings(
                    SolidityGenerateSettingsOverrides {
                        kinds: Some(vec![expected_settings]),
                        foundry_root: Some(PathBuf::from("root")),
                        ..Default::default()
                    },
                ))
            }
        );
    }
    // every_declared_kind_is_admitted_to_its_own_settings_and_configuration
}

/// Contract: own entry `every_declared_curve_directory_reaches_the_field_its_curve_names`
///   — `admit_generate` over a document whose `curve_directories` names every
///   member of `PAIRING_CURVES`, the member under test with one directory
///   text and every other member with another, returns `Ok` whose
///   `SolidityCurveDirectories` holds the member under test's text in the
///   field that member names and the other text in the other field, as an
///   outside caller observes it.
/// Arrange: for each member of `PAIRING_CURVES` a payload whose
///   `curve_directories` names every member by `declared_name`, the member
///   under test with `"directory-a"` and every other member with
///   `"directory-b"`, and an expected `SolidityCurveDirectories` built by
///   `build_solidity_curve_directories` whose field that member names is
///   `"directory-a"` and whose other field is `"directory-b"`.
/// Act:     `admit_generate` through the crate root, once per member.
/// Assert:  the result is `Ok` and the directories equal the expected
///   directories.
/// Boundary: the public call `admit_generate` and the real declared set.
/// Mocked:  none.
#[test]
fn every_declared_curve_directory_reaches_the_field_its_curve_names() {
    // Arrange — the deps, params, and directory names are invariant across
    // every member.
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let directory_a = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("directory-a".to_string()),
    });
    let directory_b = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("directory-b".to_string()),
    });

    // Act / Assert — every member of `PAIRING_CURVES` in turn as the member
    // under test.
    for member_under_test in PAIRING_CURVES {
        let mut payload = build_admit_generate_payload(Default::default());
        payload.generate.curve_directories = PAIRING_CURVES
            .iter()
            .map(|member| {
                (
                    declared_name(member),
                    if member == member_under_test {
                        "directory-a".to_string()
                    } else {
                        "directory-b".to_string()
                    },
                )
            })
            .collect();
        let (bn254, bls12_381) = match member_under_test {
            PairingCurve::Bn254 => (directory_a.clone(), directory_b.clone()),
            PairingCurve::Bls12381 => (directory_b.clone(), directory_a.clone()),
            PairingCurve::Mock => panic!("PAIRING_CURVES names no Mock member"),
        };
        let expected_directories =
            build_solidity_curve_directories(SolidityCurveDirectoriesOverrides {
                bn254: Some(bn254),
                bls12_381: Some(bls12_381),
                ..Default::default()
            });

        let admitted = admit_generate(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;

        assert_eq!(
            admitted.concrete,
            GenerateConcrete::Solidity {
                settings: Box::new(build_solidity_generate_settings(
                    SolidityGenerateSettingsOverrides {
                        kinds: Some(vec![build_solidity_vector_kind_settings(
                            SolidityVectorKindSettingsOverrides::default(),
                        )]),
                        curve_directories: Some(expected_directories),
                        foundry_root: Some(PathBuf::from("root")),
                        ..Default::default()
                    },
                ))
            }
        );
    }
    // every_declared_curve_directory_reaches_the_field_its_curve_names
}

/// Contract: own entry `every_declared_curve_libraries_reach_the_field_their_curve_names`
///   — `admit_generate` over a document whose `curve_libraries` names every
///   member of `PAIRING_CURVES`, the member under test with one pair of
///   library names and every other member with another, returns `Ok` whose
///   `SolidityCurveLibraries` holds the member under test's
///   `SolidityCurveLibraryNames` in the field that member names and the
///   other pair in the other field, as an outside caller observes it.
/// Arrange: for each member of `PAIRING_CURVES` a payload whose
///   `curve_libraries` names every member by `declared_name`, the member
///   under test with the library names `"constants-a.sol"`, `"ConstantsA"`,
///   `"records-a.sol"`, and `"RecordsA"` and every other member with
///   `"constants-b.sol"`, `"ConstantsB"`, `"records-b.sol"`, and `"RecordsB"`,
///   and an expected `SolidityCurveLibraries` built by
///   `build_solidity_curve_libraries` whose field that member names holds the
///   `-a` names and whose other field holds the `-b` names.
/// Act:     `admit_generate` through the crate root, once per member.
/// Assert:  the result is `Ok` and the libraries equal the expected
///   libraries.
/// Boundary: the public call `admit_generate` and the real declared set.
/// Mocked:  none.
#[test]
fn every_declared_curve_libraries_reach_the_field_their_curve_names() {
    // Arrange — the deps, params, and library names are invariant across
    // every member.
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let table_a = build_curve_libraries_table(CurveLibrariesTableOverrides {
        constants_library: Some(build_library_table(LibraryTableOverrides {
            file: Some("constants-a.sol".to_string()),
            library: Some("ConstantsA".to_string()),
        })),
        records_library: Some(build_library_table(LibraryTableOverrides {
            file: Some("records-a.sol".to_string()),
            library: Some("RecordsA".to_string()),
        })),
    });
    let table_b = build_curve_libraries_table(CurveLibrariesTableOverrides {
        constants_library: Some(build_library_table(LibraryTableOverrides {
            file: Some("constants-b.sol".to_string()),
            library: Some("ConstantsB".to_string()),
        })),
        records_library: Some(build_library_table(LibraryTableOverrides {
            file: Some("records-b.sol".to_string()),
            library: Some("RecordsB".to_string()),
        })),
    });
    let libraries_a = build_solidity_curve_library_names(SolidityCurveLibraryNamesOverrides {
        constants_library: Some(build_solidity_library_names(
            SolidityLibraryNamesOverrides {
                file_name: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("constants-a.sol".to_string()),
                    },
                )),
                library_name: Some(build_solidity_library_name(
                    SolidityLibraryNameConstructorParamsOverrides {
                        text: Some("ConstantsA".to_string()),
                    },
                )),
            },
        )),
        records_library: Some(build_solidity_library_names(
            SolidityLibraryNamesOverrides {
                file_name: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("records-a.sol".to_string()),
                    },
                )),
                library_name: Some(build_solidity_library_name(
                    SolidityLibraryNameConstructorParamsOverrides {
                        text: Some("RecordsA".to_string()),
                    },
                )),
            },
        )),
    });
    let libraries_b = build_solidity_curve_library_names(SolidityCurveLibraryNamesOverrides {
        constants_library: Some(build_solidity_library_names(
            SolidityLibraryNamesOverrides {
                file_name: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("constants-b.sol".to_string()),
                    },
                )),
                library_name: Some(build_solidity_library_name(
                    SolidityLibraryNameConstructorParamsOverrides {
                        text: Some("ConstantsB".to_string()),
                    },
                )),
            },
        )),
        records_library: Some(build_solidity_library_names(
            SolidityLibraryNamesOverrides {
                file_name: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("records-b.sol".to_string()),
                    },
                )),
                library_name: Some(build_solidity_library_name(
                    SolidityLibraryNameConstructorParamsOverrides {
                        text: Some("RecordsB".to_string()),
                    },
                )),
            },
        )),
    });

    // Act / Assert — every member of `PAIRING_CURVES` in turn as the member
    // under test.
    for member_under_test in PAIRING_CURVES {
        let mut payload = build_admit_generate_payload(Default::default());
        payload.generate.curve_libraries = PAIRING_CURVES
            .iter()
            .map(|member| {
                (
                    declared_name(member),
                    if member == member_under_test {
                        table_a.clone()
                    } else {
                        table_b.clone()
                    },
                )
            })
            .collect();
        let (bn254, bls12_381) = match member_under_test {
            PairingCurve::Bn254 => (libraries_a.clone(), libraries_b.clone()),
            PairingCurve::Bls12381 => (libraries_b.clone(), libraries_a.clone()),
            PairingCurve::Mock => panic!("PAIRING_CURVES names no Mock member"),
        };
        let expected_libraries = build_solidity_curve_libraries(SolidityCurveLibrariesOverrides {
            bn254: Some(bn254),
            bls12_381: Some(bls12_381),
            ..Default::default()
        });

        let admitted = admit_generate(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;

        assert_eq!(
            admitted.concrete,
            GenerateConcrete::Solidity {
                settings: Box::new(build_solidity_generate_settings(
                    SolidityGenerateSettingsOverrides {
                        kinds: Some(vec![build_solidity_vector_kind_settings(
                            SolidityVectorKindSettingsOverrides::default(),
                        )]),
                        curve_libraries: Some(expected_libraries),
                        foundry_root: Some(PathBuf::from("root")),
                        ..Default::default()
                    },
                ))
            }
        );
    }
    // every_declared_curve_libraries_reach_the_field_their_curve_names
}

/// Contract: own entry `an_undeclared_name_is_never_admitted` —
///   `admit_generate` over a document whose `concrete` is the `Debug` text of
///   the family's mock selection, and over a document whose curve key in
///   `curve_directories` and a document whose curve key in `curve_libraries`
///   is the `Debug` text of the pairing family's mock variant, the other
///   values admitted, returns `Err` carrying `Concrete`, `CurveKey`, or
///   `CurveLibrariesKey` and that text, as an outside caller observes it.
/// Arrange: a payload whose `concrete` is `declared_name` of
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`, a
///   payload whose first curve key in `curve_directories` is `declared_name`
///   of `PairingCurve::Mock`, and a payload whose first curve key in
///   `curve_libraries` is `declared_name` of `PairingCurve::Mock`, the other
///   values admitted.
/// Act:     `admit_generate` through the crate root, once per payload.
/// Assert:  the result equals `Err` of the variant carrying the text.
/// Boundary: the public call `admit_generate` and the real declared sets.
/// Mocked:  none.
#[test]
fn an_undeclared_name_is_never_admitted() {
    // Arrange — the deps and params are invariant across every payload.
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());

    // The mock concrete's `Debug` text names no member of `GENERATE_CONCRETES`.
    let text = declared_name(&GenerateConcrete::Mock(
        MockIGenerateAdapterFailureMode::NoFailure,
    ));
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.concrete = text.clone();
    // Act
    let result = admit_generate(&deps, params, payload);
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::Concrete { text }));

    // The mock curve's `Debug` text names no member of `PAIRING_CURVES`.
    let text = declared_name(&PairingCurve::Mock);
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.curve_directories = BTreeMap::from([(text.clone(), "directory".to_string())]);
    // Act
    let result = admit_generate(&deps, params, payload);
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::CurveKey { text }));

    // The mock curve's `Debug` text names no member of `PAIRING_CURVES` as a
    // `curve_libraries` key either.
    let text = declared_name(&PairingCurve::Mock);
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.curve_libraries = BTreeMap::from([(
        text.clone(),
        build_curve_libraries_table(Default::default()),
    )]);
    // Act
    let result = admit_generate(&deps, params, payload);
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::CurveLibrariesKey { text })
    );
    // an_undeclared_name_is_never_admitted
}
