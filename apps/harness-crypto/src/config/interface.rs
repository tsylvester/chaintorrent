use core::num::NonZeroU32;
use std::ffi::OsString;

use super::admit_generate::provides::{AdmitGenerateErrorReturn, AdmitGenerateFn};
use super::admit_output::provides::{AdmitOutputErrorReturn, AdmitOutputFn, OutputDestination};
use super::admit_selections::provides::{
    AdmitSelectionsErrorReturn, AdmitSelectionsFn, HarnessSelections,
};
use super::admit_statements::provides::{
    AdmitStatementsErrorReturn, AdmitStatementsFn, HarnessStatements, NetworkProfile,
};
use super::layers::provides::{LayersErrorReturn, LayersFn};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

pub const HARNESS_DEFAULTS: &str = include_str!("../../harness.toml");

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarnessConfiguration {
    pub selections: HarnessSelections,
    pub network: NetworkProfile,
    pub statements: HarnessStatements,
    pub generate: GenerateConcrete,
    pub generate_kinds: Vec<VectorKindConfiguration>,
    pub benchmark_iterations: NonZeroU32,
    pub output: Vec<OutputDestination>,
}

pub struct ConfigureDeps {
    pub layers: LayersFn,
    pub admit_selections: AdmitSelectionsFn,
    pub admit_statements: AdmitStatementsFn,
    pub admit_generate: AdmitGenerateFn,
    pub admit_output: AdmitOutputFn,
}

pub struct ConfigureParams;

pub struct ConfigurePayload<'a> {
    pub defaults: &'a str,
    pub arguments: Vec<OsString>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigureSuccessReturn {
    pub configuration: HarnessConfiguration,
}

#[derive(Debug)]
pub enum ConfigureErrorReturn {
    Layers(LayersErrorReturn),
    Selections(AdmitSelectionsErrorReturn),
    Statements(AdmitStatementsErrorReturn),
    Generate(AdmitGenerateErrorReturn),
    Output(AdmitOutputErrorReturn),
    ZeroBenchmarkIterations,
}

pub type ConfigureReturn = Result<ConfigureSuccessReturn, ConfigureErrorReturn>;

pub type ConfigureFn = fn(&ConfigureDeps, ConfigureParams, ConfigurePayload<'_>) -> ConfigureReturn;
