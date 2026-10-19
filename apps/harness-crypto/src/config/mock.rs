#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use core::num::NonZeroU32;
use std::ffi::OsString;

use super::admit_generate::provides::{
    AdmitGenerateFn, build_admit_generate_success_return, mock_admit_generate,
};
use super::admit_output::provides::{
    AdmitOutputFn, OutputDestination, build_admit_output_success_return, mock_admit_output,
};
use super::admit_selections::provides::{
    AdmitSelectionsFn, HarnessSelections, build_harness_selections, mock_admit_selections,
};
use super::admit_statements::provides::{
    AdmitStatementsFn, HarnessStatements, NetworkProfile, build_harness_statements,
    build_network_profile, mock_admit_statements,
};
use super::interface::{
    ConfigureDeps, ConfigureFn, ConfigureParams, ConfigurePayload, ConfigureReturn,
    ConfigureSuccessReturn, HarnessConfiguration,
};
use super::layers::provides::{
    LayersFn, build_benchmark_document, build_layers_payload, mock_layers,
};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

#[derive(Default)]
pub struct HarnessConfigurationOverrides {
    pub selections: Option<HarnessSelections>,
    pub network: Option<NetworkProfile>,
    pub statements: Option<HarnessStatements>,
    pub generate: Option<GenerateConcrete>,
    pub generate_kinds: Option<Vec<VectorKindConfiguration>>,
    pub benchmark_iterations: Option<NonZeroU32>,
    pub output: Option<Vec<OutputDestination>>,
}

pub fn build_harness_configuration(
    overrides: HarnessConfigurationOverrides,
) -> HarnessConfiguration {
    let generate = build_admit_generate_success_return(Default::default());
    HarnessConfiguration {
        selections: overrides
            .selections
            .unwrap_or_else(|| build_harness_selections(Default::default())),
        network: overrides
            .network
            .unwrap_or_else(|| build_network_profile(Default::default())),
        statements: overrides
            .statements
            .unwrap_or_else(|| build_harness_statements(Default::default())),
        generate: overrides.generate.unwrap_or(generate.concrete),
        generate_kinds: overrides.generate_kinds.unwrap_or(generate.kinds),
        benchmark_iterations: overrides.benchmark_iterations.unwrap_or_else(|| {
            NonZeroU32::new(build_benchmark_document(Default::default()).iterations)
                .expect("the document builder's iteration count is nonzero")
        }),
        output: overrides
            .output
            .unwrap_or_else(|| build_admit_output_success_return(Default::default()).destinations),
    }
}

#[derive(Default)]
pub struct ConfigureDepsOverrides {
    pub layers: Option<LayersFn>,
    pub admit_selections: Option<AdmitSelectionsFn>,
    pub admit_statements: Option<AdmitStatementsFn>,
    pub admit_generate: Option<AdmitGenerateFn>,
    pub admit_output: Option<AdmitOutputFn>,
}

pub fn build_configure_deps(overrides: ConfigureDepsOverrides) -> ConfigureDeps {
    ConfigureDeps {
        layers: overrides.layers.unwrap_or(mock_layers),
        admit_selections: overrides.admit_selections.unwrap_or(mock_admit_selections),
        admit_statements: overrides.admit_statements.unwrap_or(mock_admit_statements),
        admit_generate: overrides.admit_generate.unwrap_or(mock_admit_generate),
        admit_output: overrides.admit_output.unwrap_or(mock_admit_output),
    }
}

#[derive(Default)]
pub struct ConfigureParamsOverrides;

pub fn build_configure_params(_overrides: ConfigureParamsOverrides) -> ConfigureParams {
    ConfigureParams
}

#[derive(Default)]
pub struct ConfigurePayloadOverrides {
    pub arguments: Option<Vec<OsString>>,
}

pub fn build_configure_payload<'a>(
    defaults: &'a str,
    overrides: ConfigurePayloadOverrides,
) -> ConfigurePayload<'a> {
    ConfigurePayload {
        defaults,
        arguments: overrides
            .arguments
            .unwrap_or_else(|| build_layers_payload(defaults, Default::default()).arguments),
    }
}

#[derive(Default)]
pub struct ConfigureSuccessReturnOverrides {
    pub configuration: Option<HarnessConfiguration>,
}

pub fn build_configure_success_return(
    overrides: ConfigureSuccessReturnOverrides,
) -> ConfigureSuccessReturn {
    ConfigureSuccessReturn {
        configuration: overrides
            .configuration
            .unwrap_or_else(|| build_harness_configuration(Default::default())),
    }
}

pub fn mock_configure(
    _deps: &ConfigureDeps,
    _params: ConfigureParams,
    _payload: ConfigurePayload<'_>,
) -> ConfigureReturn {
    let _: ConfigureFn = mock_configure;
    Ok(build_configure_success_return(Default::default()))
}
