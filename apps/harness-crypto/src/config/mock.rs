#![allow(dead_code, clippy::too_many_arguments)]

use core::num::NonZeroU32;
use std::ffi::OsString;

use super::admit_generate::provides::{
    AdmitGenerateFn, AdmittedGenerate, build_admitted_generate, mock_admit_generate,
};
use super::admit_output::provides::{
    AdmitOutputFn, AdmittedOutput, build_admitted_output, mock_admit_output,
};
use super::admit_selections::provides::{
    AdmitSelectionsFn, AdmittedSelections, build_admitted_selections, mock_admit_selections,
};
use super::admit_statements::provides::{
    AdmitStatementsFn, AdmittedIdentities, AdmittedNetworkProfile, AdmittedNetworkProfileOverrides,
    AdmittedStatements, build_admitted_identities, build_admitted_network_profile,
    build_admitted_statements, mock_admit_statements,
};
use super::interface::{
    ConfigDeps, ConfigFn, ConfigParams, ConfigPayload, ConfigReturn, ConfigSuccessReturn,
    HarnessConfiguration, MeasurementSettings, ReportSettings, SHIPPED_DEFAULTS,
};
use super::layers::provides::{LayersFn, mock_layers};
use crate::report::provides::{
    ReportAdjustment, ReportDefaults, ReportDefaultsOverrides, ReportText,
    ReportTextConstructorParamsOverrides, build_report_defaults, build_report_text,
};
use crate::verifier::provides::{
    EvmVerifierEndpointConstructorParamsOverrides, EvmVerifierSettingsOverrides, VerifierConcrete,
    build_evm_verifier_endpoint, build_evm_verifier_settings,
};

#[derive(Default)]
pub struct MeasurementSettingsOverrides {
    pub iterations: Option<NonZeroU32>,
    pub conditions: Option<ReportText>,
}

pub fn build_measurement_settings(overrides: MeasurementSettingsOverrides) -> MeasurementSettings {
    MeasurementSettings {
        iterations: overrides.iterations.unwrap_or(NonZeroU32::MIN),
        conditions: overrides.conditions.unwrap_or_else(|| {
            build_report_text(ReportTextConstructorParamsOverrides {
                text: Some("conditions-a".to_string()),
            })
        }),
    }
}

#[derive(Default)]
pub struct ReportSettingsOverrides {
    pub defaults: Option<ReportDefaults>,
    pub adjustment: Option<Option<ReportAdjustment>>,
}

pub fn build_report_settings(overrides: ReportSettingsOverrides) -> ReportSettings {
    ReportSettings {
        defaults: overrides.defaults.unwrap_or_else(|| {
            build_report_defaults(ReportDefaultsOverrides {
                piece_group_size: Some(NonZeroU32::MIN.saturating_add(16_383)),
                primary_curve: Some(build_report_text(ReportTextConstructorParamsOverrides {
                    text: Some("curve-a".to_string()),
                })),
            })
        }),
        adjustment: overrides.adjustment.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct HarnessConfigurationOverrides {
    pub selections: Option<AdmittedSelections>,
    pub verifiers: Option<Vec<VerifierConcrete>>,
    pub curve_labels: Option<Vec<ReportText>>,
    pub network_profile: Option<AdmittedNetworkProfile>,
    pub statements: Option<AdmittedStatements>,
    pub identities: Option<AdmittedIdentities>,
    pub generate: Option<AdmittedGenerate>,
    pub output: Option<AdmittedOutput>,
    pub benchmark_iterations: Option<NonZeroU32>,
    pub measurement: Option<MeasurementSettings>,
    pub report: Option<ReportSettings>,
}

pub fn build_harness_configuration(
    overrides: HarnessConfigurationOverrides,
) -> HarnessConfiguration {
    let default_selections = build_admitted_selections(Default::default());
    let default_network_profile = build_admitted_network_profile(AdmittedNetworkProfileOverrides {
        deployment_records: Some(
            build_admitted_network_profile(Default::default())
                .deployment_records
                .into_iter()
                .take(default_selections.curves.len())
                .collect(),
        ),
        ..Default::default()
    });
    let default_verifiers: Vec<VerifierConcrete> = default_selections
        .curves
        .iter()
        .zip(default_network_profile.deployment_records.iter())
        .map(|(_, record)| VerifierConcrete::Evm {
            settings: Box::new(build_evm_verifier_settings(EvmVerifierSettingsOverrides {
                endpoint: Some(build_evm_verifier_endpoint(
                    EvmVerifierEndpointConstructorParamsOverrides {
                        text: Some("http://endpoint-a.test".to_string()),
                    },
                )),
                verifier_address: Some(record.clone()),
                gas_price_oracle: Some(default_network_profile.gas_price_oracle.clone()),
            })),
        })
        .collect();
    let default_curve_labels: Vec<ReportText> = default_selections
        .curves
        .iter()
        .map(|curve| {
            build_report_text(ReportTextConstructorParamsOverrides {
                text: Some(curve.label.clone()),
            })
        })
        .collect();
    HarnessConfiguration {
        selections: overrides.selections.unwrap_or(default_selections),
        verifiers: overrides.verifiers.unwrap_or(default_verifiers),
        curve_labels: overrides.curve_labels.unwrap_or(default_curve_labels),
        network_profile: overrides.network_profile.unwrap_or(default_network_profile),
        statements: overrides
            .statements
            .unwrap_or_else(|| build_admitted_statements(Default::default())),
        identities: overrides
            .identities
            .unwrap_or_else(|| build_admitted_identities(Default::default())),
        generate: overrides
            .generate
            .unwrap_or_else(|| build_admitted_generate(Default::default())),
        output: overrides
            .output
            .unwrap_or_else(|| build_admitted_output(Default::default())),
        benchmark_iterations: overrides.benchmark_iterations.unwrap_or(NonZeroU32::MIN),
        measurement: overrides
            .measurement
            .unwrap_or_else(|| build_measurement_settings(Default::default())),
        report: overrides
            .report
            .unwrap_or_else(|| build_report_settings(Default::default())),
    }
}

#[derive(Default)]
pub struct ConfigDepsOverrides {
    pub layers: Option<LayersFn>,
    pub admit_selections: Option<AdmitSelectionsFn>,
    pub admit_statements: Option<AdmitStatementsFn>,
    pub admit_generate: Option<AdmitGenerateFn>,
    pub admit_output: Option<AdmitOutputFn>,
}

pub fn build_config_deps(overrides: ConfigDepsOverrides) -> ConfigDeps {
    ConfigDeps {
        layers: overrides.layers.unwrap_or(mock_layers),
        admit_selections: overrides.admit_selections.unwrap_or(mock_admit_selections),
        admit_statements: overrides.admit_statements.unwrap_or(mock_admit_statements),
        admit_generate: overrides.admit_generate.unwrap_or(mock_admit_generate),
        admit_output: overrides.admit_output.unwrap_or(mock_admit_output),
    }
}

#[derive(Default)]
pub struct ConfigParamsOverrides {
    pub arguments: Option<Vec<OsString>>,
}

pub fn build_config_params(overrides: ConfigParamsOverrides) -> ConfigParams {
    ConfigParams {
        arguments: overrides
            .arguments
            .unwrap_or_else(|| vec![OsString::from("harness-crypto")]),
    }
}

#[derive(Default)]
pub struct ConfigPayloadOverrides {
    pub defaults: Option<String>,
}

pub fn build_config_payload(overrides: ConfigPayloadOverrides) -> ConfigPayload {
    ConfigPayload {
        defaults: overrides
            .defaults
            .unwrap_or_else(|| SHIPPED_DEFAULTS.to_string()),
    }
}

#[derive(Default)]
pub struct ConfigSuccessReturnOverrides {
    pub configuration: Option<HarnessConfiguration>,
}

pub fn build_config_success_return(overrides: ConfigSuccessReturnOverrides) -> ConfigSuccessReturn {
    ConfigSuccessReturn {
        configuration: overrides
            .configuration
            .unwrap_or_else(|| build_harness_configuration(Default::default())),
    }
}

pub fn mock_config(
    _deps: &ConfigDeps,
    _params: ConfigParams,
    _payload: ConfigPayload,
) -> ConfigReturn {
    let _: ConfigFn = mock_config;
    Ok(build_config_success_return(Default::default()))
}
