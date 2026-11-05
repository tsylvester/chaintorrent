#![allow(clippy::expect_used)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use pairing::{PAIRING_CURVES, PairingCurve};

use super::super::layers::provides::{GenerateKindTableOverrides, build_generate_kind_table};
use super::admit_generate;
use super::interface::AdmitGenerateErrorReturn;
use super::mock::{
    build_admit_generate_deps, build_admit_generate_params, build_admit_generate_payload,
    declared_name,
};
use crate::generate::provides::{
    GenerateConcrete, GeneratedFileNameConstructorParamsOverrides, MockIGenerateAdapterFailureMode,
    MsmTermCountConstructorParamsOverrides, SOLIDITY_GENERATE_DECLARATION,
    SolidityConstantNameConstructorParamsOverrides, SolidityCurveDirectoriesOverrides,
    SolidityGenerateSettingsOverrides, SolidityMemberNameConstructorParamsOverrides,
    SolidityTypeNameConstructorParamsOverrides, SolidityVectorKindNamesOverrides,
    SolidityVectorKindSettingsOverrides, VectorCountConstructorParamsOverrides,
    VectorKindConfiguration, VectorKindQuantities, VectorKindSizing, build_generated_file_name,
    build_msm_term_count, build_solidity_constant_name, build_solidity_curve_directories,
    build_solidity_generate_settings, build_solidity_member_name, build_solidity_type_name,
    build_solidity_vector_kind_names, build_solidity_vector_kind_settings, build_vector_count,
};

/// Contract: `every_declared_kind_is_admitted_to_its_own_settings_and_configuration`
///   → `admit_generate` over a document whose `kinds` names one kind of
///   `SOLIDITY_GENERATE_DECLARATION.kinds` with the quantities its declared
///   sizing takes and names distinct from every other kind's returns `Ok`
///   whose `admitted.kinds` holds the `VectorKindConfiguration` of that kind
///   with the quantities the document holds and whose settings hold that
///   kind's `SolidityVectorKindSettings` with the names the document holds.
/// Arrange: for each `VectorKindDeclaration` of
///   `SOLIDITY_GENERATE_DECLARATION.kinds` a payload from
///   `build_admit_generate_payload` whose single `kinds` entry is keyed
///   `declared_name` of the declaration's `kind`, carrying names distinct
///   from the default's and quantities chosen by an exhaustive `match` on
///   the declaration's `sizing`, and the expected configuration and settings
///   built from the generate family's builders over the same values.
/// Act:     `admit_generate`, once per payload.
/// Assert:  the result is `Ok` and the admitted configuration and the
///   concrete's settings entry equal the expected values.
/// Boundary: the real `admit_generate`, `member_named`, and each constructor
///   over the real declaration.
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

/// Contract: `every_declared_curve_directory_reaches_the_field_its_curve_names`
///   → `admit_generate` over a document whose `curve_directories` names every
///   member of `PAIRING_CURVES`, the member under test with one directory
///   text and every other member with another, returns `Ok` whose
///   `SolidityCurveDirectories` holds the member under test's text in the
///   field that member names and the other text in the other field.
/// Arrange: for each member of `PAIRING_CURVES` a payload whose
///   `curve_directories` names every member by `declared_name`, the member
///   under test with `"directory-a"` and every other member with
///   `"directory-b"`, and an expected `SolidityCurveDirectories` built by
///   `build_solidity_curve_directories` whose field that member names is
///   `"directory-a"` and whose other field is `"directory-b"`.
/// Act:     `admit_generate`, once per member.
/// Assert:  the result is `Ok` and the concrete's `settings.curve_directories`
///   equals the expected directories.
/// Boundary: the real `admit_generate`, `member_named`, and each constructor
///   over the real declared set.
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

/// Contract: `an_undeclared_name_is_never_admitted` → `admit_generate` over a
///   document whose `concrete` is the `Debug` text of the family's mock
///   selection, and over a document whose curve key is the `Debug` text of
///   the pairing family's mock variant, the other values admitted, returns
///   `Err` carrying `Concrete` or `CurveKey` and that text.
/// Arrange: a payload whose `concrete` is `declared_name` of
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)` and
///   a payload whose first curve key is `declared_name` of
///   `PairingCurve::Mock`, the other values admitted.
/// Act:     `admit_generate`, once per payload.
/// Assert:  the result equals `Err` of `Concrete` or `CurveKey` carrying the
///   text.
/// Boundary: the real `admit_generate`, `member_named`, and each constructor
///   over the real declared sets.
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
    // an_undeclared_name_is_never_admitted
}
