#![allow(dead_code, clippy::too_many_arguments)]

use std::path::PathBuf;

#[cfg(test)]
pub use super::super::admit_selections::provides::declared_name;

use super::super::layers::provides::{GenerateTable, build_generate_table};
use super::interface::{
    AdmitGenerateDeps, AdmitGenerateFn, AdmitGenerateParams, AdmitGeneratePayload,
    AdmitGenerateReturn, AdmitGenerateSuccessReturn, AdmittedGenerate,
};
use crate::generate::provides::{
    GenerateConcrete, SolidityGenerateSettingsOverrides, SolidityVectorKindSettingsOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    VectorKind, VectorKindConfiguration, VectorKindQuantities, build_solidity_generate_settings,
    build_solidity_vector_kind_settings, build_spdx_license_identifier, build_vector_count,
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
        generate: overrides
            .generate
            .unwrap_or_else(|| build_generate_table(Default::default())),
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
