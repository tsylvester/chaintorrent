mod admit_generate;
mod admit_output;
mod admit_selections;
mod admit_statements;
#[cfg(test)]
mod integration_test;
mod interface;
mod layers;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;

use core::num::NonZeroU32;

use crate::report::provides::{
    ReportAdjustment, ReportDefaults, ReportText, ReportTextConstructorParams,
};
use crate::verifier::provides::{
    EvmVerifierEndpoint, EvmVerifierEndpointConstructorParams, EvmVerifierSettings,
    VerifierConcrete, VerifierConcreteKind,
};
use admit_generate::provides::{AdmitGenerateDeps, AdmitGenerateParams, AdmitGeneratePayload};
use admit_output::provides::{AdmitOutputDeps, AdmitOutputParams, AdmitOutputPayload};
use admit_selections::provides::{
    AdmitSelectionsDeps, AdmitSelectionsParams, AdmitSelectionsPayload,
};
use admit_statements::provides::{
    AdmitStatementsDeps, AdmitStatementsParams, AdmitStatementsPayload,
    AdmitStatementsSuccessReturn,
};
use interface::{
    ConfigDeps, ConfigErrorReturn, ConfigFn, ConfigParams, ConfigPayload, ConfigReturn,
    ConfigSuccessReturn, HarnessConfiguration, MeasurementSettings, ReportSettings,
};
use layers::provides::{LayersDeps, LayersParams, LayersPayload};

pub fn config(deps: &ConfigDeps, params: ConfigParams, payload: ConfigPayload) -> ConfigReturn {
    let _: ConfigFn = config;

    let document = match (deps.layers)(
        &LayersDeps,
        LayersParams {
            arguments: params.arguments,
        },
        LayersPayload {
            defaults: payload.defaults,
        },
    ) {
        Ok(success) => success.document,
        Err(error) => return Err(ConfigErrorReturn::Layers(error)),
    };

    let admitted = match (deps.admit_selections)(
        &AdmitSelectionsDeps,
        AdmitSelectionsParams,
        AdmitSelectionsPayload {
            selections: document.selections.clone(),
            precompile_encodings: document.network_profile.precompile_encodings.clone(),
        },
    ) {
        Ok(success) => success.admitted,
        Err(error) => return Err(ConfigErrorReturn::Selections(error)),
    };

    let AdmitStatementsSuccessReturn {
        network_profile,
        statements,
        identities,
    } = match (deps.admit_statements)(
        &AdmitStatementsDeps,
        AdmitStatementsParams,
        AdmitStatementsPayload {
            network_profile: document.network_profile.clone(),
            statements: document.statements.clone(),
            identities: document.identities.clone(),
            scope: admitted.kem.scope,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(ConfigErrorReturn::Statements(error)),
    };

    let generate = match (deps.admit_generate)(
        &AdmitGenerateDeps,
        AdmitGenerateParams,
        AdmitGeneratePayload {
            generate: document.generate.clone(),
        },
    ) {
        Ok(success) => success.admitted,
        Err(error) => return Err(ConfigErrorReturn::Generate(error)),
    };

    let output = match (deps.admit_output)(
        &AdmitOutputDeps,
        AdmitOutputParams,
        AdmitOutputPayload {
            output: document.output.clone(),
            report: document.report.clone(),
        },
    ) {
        Ok(success) => success.admitted,
        Err(error) => return Err(ConfigErrorReturn::Output(error)),
    };

    let benchmark_iterations = match NonZeroU32::new(document.benchmark.iterations) {
        Some(count) => count,
        None => return Err(ConfigErrorReturn::BenchmarkIterations),
    };

    let measurement_iterations = match NonZeroU32::new(document.measurement.iterations) {
        Some(count) => count,
        None => return Err(ConfigErrorReturn::MeasurementIterations),
    };
    let conditions = match ReportText::try_new(ReportTextConstructorParams {
        text: document.measurement.conditions.clone(),
    }) {
        Ok(text) => text,
        Err(error) => return Err(ConfigErrorReturn::MeasurementConditions(error)),
    };
    let measurement = MeasurementSettings {
        iterations: measurement_iterations,
        conditions,
    };

    let piece_group_size = match NonZeroU32::new(document.report.piece_group_size) {
        Some(size) => size,
        None => return Err(ConfigErrorReturn::ReportPieceGroupSize),
    };
    let primary_curve = match ReportText::try_new(ReportTextConstructorParams {
        text: document.report.primary_curve.clone(),
    }) {
        Ok(text) => text,
        Err(error) => return Err(ConfigErrorReturn::ReportPrimaryCurve(error)),
    };
    let defaults = ReportDefaults {
        piece_group_size,
        primary_curve,
    };

    let adjustment = match &document.report.adjustment {
        None => None,
        Some(adjustment) => {
            let piece_group_size = match adjustment.piece_group_size {
                Some(size) => match NonZeroU32::new(size) {
                    Some(size) => Some(size),
                    None => return Err(ConfigErrorReturn::AdjustmentPieceGroupSize),
                },
                None => None,
            };
            let primary_curve = match &adjustment.primary_curve {
                Some(text) => {
                    match ReportText::try_new(ReportTextConstructorParams { text: text.clone() }) {
                        Ok(text) => Some(text),
                        Err(error) => return Err(ConfigErrorReturn::AdjustmentPrimaryCurve(error)),
                    }
                }
                None => None,
            };
            Some(ReportAdjustment {
                piece_group_size,
                primary_curve,
            })
        }
    };
    let report = ReportSettings {
        defaults,
        adjustment,
    };

    let mut curve_labels = Vec::with_capacity(admitted.curves.len());
    for (index, curve) in admitted.curves.iter().enumerate() {
        match ReportText::try_new(ReportTextConstructorParams {
            text: curve.label.clone(),
        }) {
            Ok(label) => curve_labels.push(label),
            Err(error) => return Err(ConfigErrorReturn::CurveLabel { index, error }),
        }
    }

    let endpoint = match EvmVerifierEndpoint::try_new(EvmVerifierEndpointConstructorParams {
        text: document.network_profile.verifier_endpoint,
    }) {
        Ok(endpoint) => endpoint,
        Err(error) => return Err(ConfigErrorReturn::VerifierEndpoint(error)),
    };

    let verifier = match admitted.verifier {
        VerifierConcreteKind::Evm => VerifierConcrete::Evm {
            settings: Box::new(EvmVerifierSettings {
                endpoint,
                verifier_address: network_profile.deployment_record.clone(),
                gas_price_oracle: network_profile.gas_price_oracle.clone(),
            }),
        },
    };

    Ok(ConfigSuccessReturn {
        configuration: HarnessConfiguration {
            selections: admitted,
            verifier,
            curve_labels,
            network_profile,
            statements,
            identities,
            generate,
            output,
            benchmark_iterations,
            measurement,
            report,
        },
    })
}
