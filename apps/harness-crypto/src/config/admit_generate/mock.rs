#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::super::layers::provides::{GenerateDocument, build_generate_document};
use super::interface::{
    AdmitGenerateDeps, AdmitGenerateFn, AdmitGenerateParams, AdmitGeneratePayload,
    AdmitGenerateReturn, AdmitGenerateSuccessReturn,
};
use crate::generate::provides::{
    EvmGenerateSettingsOverrides, GenerateConcrete, SolidityCompilerVersionOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, VectorKindConfiguration,
    build_evm_generate_settings, build_evm_vector_kind_settings, build_solidity_compiler_version,
    build_spdx_license_identifier, build_vector_kind_configuration,
};

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
pub struct AdmitGeneratePayloadOverrides;

pub fn build_admit_generate_payload<'a>(
    generate: &'a GenerateDocument,
    _overrides: AdmitGeneratePayloadOverrides,
) -> AdmitGeneratePayload<'a> {
    AdmitGeneratePayload { generate }
}

#[derive(Default)]
pub struct AdmitGenerateSuccessReturnOverrides {
    pub concrete: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
}

pub fn build_admit_generate_success_return(
    overrides: AdmitGenerateSuccessReturnOverrides,
) -> AdmitGenerateSuccessReturn {
    AdmitGenerateSuccessReturn {
        concrete: overrides.concrete.unwrap_or_else(|| {
            let document = build_generate_document(Default::default());
            GenerateConcrete::Evm {
                settings: Box::new(build_evm_generate_settings(EvmGenerateSettingsOverrides {
                    kinds: Some(vec![build_evm_vector_kind_settings(Default::default())]),
                    foundry_root: Some(document.foundry_root.clone()),
                    compiler_version: Some(build_solidity_compiler_version(
                        SolidityCompilerVersionOverrides {
                            major: Some(document.compiler_version.major),
                            minor: Some(document.compiler_version.minor),
                            patch: Some(document.compiler_version.patch),
                        },
                    )),
                    license: Some(build_spdx_license_identifier(
                        SpdxLicenseIdentifierConstructorParamsOverrides {
                            text: Some(document.license.clone()),
                        },
                    )),
                    ..Default::default()
                })),
            }
        }),
        kinds: overrides
            .kinds
            .unwrap_or_else(|| vec![build_vector_kind_configuration(Default::default())]),
    }
}

pub fn mock_admit_generate(
    _deps: &AdmitGenerateDeps,
    _params: AdmitGenerateParams,
    _payload: AdmitGeneratePayload<'_>,
) -> AdmitGenerateReturn {
    let _: AdmitGenerateFn = mock_admit_generate;
    Ok(build_admit_generate_success_return(Default::default()))
}
