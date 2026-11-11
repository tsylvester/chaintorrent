#![allow(dead_code, clippy::too_many_arguments)]

use std::collections::BTreeMap;
use std::path::PathBuf;

pub use super::super::admit_selections::provides::declared_name;

use pairing::PairingCurve;

use super::super::layers::provides::{
    CurveLibrariesTableOverrides, GenerateKindTableOverrides, GenerateTable,
    GenerateTableOverrides, LibraryTableOverrides, build_curve_libraries_table,
    build_generate_kind_table, build_generate_table, build_library_table,
};
use super::interface::{
    AdmitGenerateDeps, AdmitGenerateFn, AdmitGenerateParams, AdmitGeneratePayload,
    AdmitGenerateReturn, AdmitGenerateSuccessReturn, AdmittedGenerate,
};
use crate::generate::provides::{
    GENERATE_CONCRETES, GenerateConcrete, SolidityGenerateSettingsOverrides,
    SolidityVectorKindSettingsOverrides, SpdxLicenseIdentifierConstructorParamsOverrides,
    VectorCountConstructorParamsOverrides, VectorKind, VectorKindConfiguration,
    VectorKindQuantities, build_solidity_generate_settings, build_solidity_vector_kind_settings,
    build_spdx_license_identifier, build_vector_count,
};

#[derive(Default)]
pub struct AdmittedGenerateOverrides {
    pub concrete: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
}

pub fn build_admitted_generate(overrides: AdmittedGenerateOverrides) -> AdmittedGenerate {
    AdmittedGenerate {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| GenerateConcrete::Solidity {
                settings: Box::new(build_solidity_generate_settings(
                    SolidityGenerateSettingsOverrides {
                        kinds: Some(vec![build_solidity_vector_kind_settings(
                            SolidityVectorKindSettingsOverrides::default(),
                        )]),
                        foundry_root: Some(PathBuf::from("root")),
                        license: Some(build_spdx_license_identifier(
                            SpdxLicenseIdentifierConstructorParamsOverrides {
                                text: Some("MIT".to_string()),
                            },
                        )),
                        ..Default::default()
                    },
                )),
            }),
        kinds: overrides.kinds.unwrap_or_else(|| {
            vec![VectorKindConfiguration {
                kind: VectorKind::G1Addition,
                quantities: VectorKindQuantities {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                    terms: None,
                    message_length: None,
                },
            }]
        }),
    }
}

#[derive(Default)]
pub struct AdmitGenerateDepsOverrides;

pub fn build_admit_generate_deps(_overrides: AdmitGenerateDepsOverrides) -> AdmitGenerateDeps {
    AdmitGenerateDeps
}

#[derive(Default)]
pub struct AdmitGenerateParamsOverrides;

pub fn build_admit_generate_params(
    _overrides: AdmitGenerateParamsOverrides,
) -> AdmitGenerateParams {
    AdmitGenerateParams
}

#[derive(Default)]
pub struct AdmitGeneratePayloadOverrides {
    pub generate: Option<GenerateTable>,
}

pub fn build_admit_generate_payload(
    overrides: AdmitGeneratePayloadOverrides,
) -> AdmitGeneratePayload {
    AdmitGeneratePayload {
        generate: overrides.generate.unwrap_or_else(|| {
            build_generate_table(GenerateTableOverrides {
                concrete: Some(declared_name(&GENERATE_CONCRETES[0])),
                kinds: Some(BTreeMap::from([(
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
                )])),
                curve_libraries: Some(BTreeMap::from([
                    (
                        declared_name(&PairingCurve::Bn254),
                        build_curve_libraries_table(CurveLibrariesTableOverrides {
                            constants_library: Some(build_library_table(LibraryTableOverrides {
                                file: Some("Constants.sol".to_string()),
                                library: Some("Bn254Constants".to_string()),
                            })),
                            records_library: Some(build_library_table(LibraryTableOverrides {
                                file: Some("VectorRecords.sol".to_string()),
                                library: Some("Bn254VectorRecords".to_string()),
                            })),
                        }),
                    ),
                    (
                        declared_name(&PairingCurve::Bls12381),
                        build_curve_libraries_table(CurveLibrariesTableOverrides {
                            constants_library: Some(build_library_table(LibraryTableOverrides {
                                file: Some("Constants.sol".to_string()),
                                library: Some("Bls12381Constants".to_string()),
                            })),
                            records_library: Some(build_library_table(LibraryTableOverrides {
                                file: Some("VectorRecords.sol".to_string()),
                                library: Some("Bls12381VectorRecords".to_string()),
                            })),
                        }),
                    ),
                ])),
                foundry_root: Some(PathBuf::from("root")),
                generated_directory: Some(vec!["generated".to_string()]),
                curve_directories: Some(BTreeMap::from([
                    (declared_name(&PairingCurve::Bn254), "bn254".to_string()),
                    (
                        declared_name(&PairingCurve::Bls12381),
                        "bls12_381".to_string(),
                    ),
                ])),
                compiler_version: Some("0.8.24".to_string()),
                license: Some("MIT".to_string()),
            })
        }),
    }
}

#[derive(Default)]
pub struct AdmitGenerateSuccessReturnOverrides {
    pub admitted: Option<AdmittedGenerate>,
}

pub fn build_admit_generate_success_return(
    overrides: AdmitGenerateSuccessReturnOverrides,
) -> AdmitGenerateSuccessReturn {
    AdmitGenerateSuccessReturn {
        admitted: overrides
            .admitted
            .unwrap_or_else(|| build_admitted_generate(Default::default())),
    }
}

pub fn mock_admit_generate(
    _deps: &AdmitGenerateDeps,
    _params: AdmitGenerateParams,
    _payload: AdmitGeneratePayload,
) -> AdmitGenerateReturn {
    let _: AdmitGenerateFn = mock_admit_generate;
    Ok(build_admit_generate_success_return(Default::default()))
}
