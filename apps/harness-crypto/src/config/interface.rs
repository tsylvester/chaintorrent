use core::num::NonZeroU32;
use std::ffi::OsString;

use super::admit_generate::provides::{
    AdmitGenerateErrorReturn, AdmitGenerateFn, AdmittedGenerate,
};
use super::admit_output::provides::{AdmitOutputErrorReturn, AdmitOutputFn, AdmittedOutput};
use super::admit_selections::provides::{
    AdmitSelectionsErrorReturn, AdmitSelectionsFn, AdmittedSelections,
};
use super::admit_statements::provides::{
    AdmitStatementsErrorReturn, AdmitStatementsFn, AdmittedIdentities, AdmittedNetworkProfile,
    AdmittedStatements,
};
use super::layers::provides::{LayersErrorReturn, LayersFn};
use crate::report::provides::{
    ReportAdjustment, ReportDefaults, ReportText, ReportTextTryNewErrorReturn,
};
use crate::verifier::provides::{EvmVerifierEndpointTryNewErrorReturn, VerifierConcrete};

pub const SHIPPED_DEFAULTS: &str = include_str!("../../harness.toml");

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementSettings {
    pub iterations: NonZeroU32,
    pub conditions: ReportText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportSettings {
    pub defaults: ReportDefaults,
    pub adjustment: Option<ReportAdjustment>,
}

#[derive(Clone)]
pub struct HarnessConfiguration {
    pub selections: AdmittedSelections,
    pub verifier: VerifierConcrete,
    pub curve_labels: Vec<ReportText>,
    pub network_profile: AdmittedNetworkProfile,
    pub statements: AdmittedStatements,
    pub identities: AdmittedIdentities,
    pub generate: AdmittedGenerate,
    pub output: AdmittedOutput,
    pub benchmark_iterations: NonZeroU32,
    pub measurement: MeasurementSettings,
    pub report: ReportSettings,
}

#[derive(Clone, Copy)]
pub struct ConfigDeps {
    pub layers: LayersFn,
    pub admit_selections: AdmitSelectionsFn,
    pub admit_statements: AdmitStatementsFn,
    pub admit_generate: AdmitGenerateFn,
    pub admit_output: AdmitOutputFn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigParams {
    pub arguments: Vec<OsString>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigPayload {
    pub defaults: String,
}

#[derive(Clone)]
pub struct ConfigSuccessReturn {
    pub configuration: HarnessConfiguration,
}

#[derive(Debug)]
pub enum ConfigErrorReturn {
    Layers(LayersErrorReturn),
    Selections(AdmitSelectionsErrorReturn),
    Statements(AdmitStatementsErrorReturn),
    Generate(AdmitGenerateErrorReturn),
    Output(AdmitOutputErrorReturn),
    BenchmarkIterations,
    MeasurementIterations,
    MeasurementConditions(ReportTextTryNewErrorReturn),
    ReportPieceGroupSize,
    ReportPrimaryCurve(ReportTextTryNewErrorReturn),
    AdjustmentPieceGroupSize,
    AdjustmentPrimaryCurve(ReportTextTryNewErrorReturn),
    CurveLabel {
        index: usize,
        error: ReportTextTryNewErrorReturn,
    },
    VerifierEndpoint(EvmVerifierEndpointTryNewErrorReturn),
}

pub type ConfigReturn = Result<ConfigSuccessReturn, ConfigErrorReturn>;

pub type ConfigFn = fn(&ConfigDeps, ConfigParams, ConfigPayload) -> ConfigReturn;
