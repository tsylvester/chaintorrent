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
mod test;

use core::num::NonZeroU32;

use admit_generate::provides::{AdmitGenerateDeps, AdmitGenerateParams, AdmitGeneratePayload};
use admit_output::provides::{AdmitOutputDeps, AdmitOutputParams, AdmitOutputPayload};
use admit_selections::provides::{
    AdmitSelectionsDeps, AdmitSelectionsParams, AdmitSelectionsPayload,
};
use admit_statements::provides::{
    AdmitStatementsDeps, AdmitStatementsParams, AdmitStatementsPayload,
};
use interface::{
    ConfigureDeps, ConfigureErrorReturn, ConfigureParams, ConfigurePayload, ConfigureReturn,
    ConfigureSuccessReturn, HarnessConfiguration,
};
use layers::provides::{LayersDeps, LayersParams, LayersPayload};

pub fn configure(
    deps: &ConfigureDeps,
    _params: ConfigureParams,
    payload: ConfigurePayload<'_>,
) -> ConfigureReturn {
    let document = (deps.layers)(
        &LayersDeps,
        LayersParams,
        LayersPayload {
            defaults: payload.defaults,
            arguments: payload.arguments,
        },
    )
    .map_err(ConfigureErrorReturn::Layers)?
    .document;
    let selections = (deps.admit_selections)(
        &AdmitSelectionsDeps,
        AdmitSelectionsParams,
        AdmitSelectionsPayload {
            random: &document.random,
            encoding: &document.encoding,
            hash_to_scalar: &document.hash_to_scalar,
            chain_forms: &document.chain_forms,
            kem: &document.kem,
            key_agreement: &document.key_agreement,
            delivery_proof: &document.delivery_proof,
            curves: &document.curves,
            benchmark_concretes: &document.benchmark.concretes,
            precompile_encodings: &document.network.precompile_encodings,
        },
    )
    .map_err(ConfigureErrorReturn::Selections)?
    .selections;
    let statements = (deps.admit_statements)(
        &AdmitStatementsDeps,
        AdmitStatementsParams,
        AdmitStatementsPayload {
            network: &document.network,
            statements: &document.statements,
        },
    )
    .map_err(ConfigureErrorReturn::Statements)?;
    let generate = (deps.admit_generate)(
        &AdmitGenerateDeps,
        AdmitGenerateParams,
        AdmitGeneratePayload {
            generate: &document.generate,
        },
    )
    .map_err(ConfigureErrorReturn::Generate)?;
    let destinations = (deps.admit_output)(
        &AdmitOutputDeps,
        AdmitOutputParams,
        AdmitOutputPayload {
            output: &document.output,
        },
    )
    .map_err(ConfigureErrorReturn::Output)?
    .destinations;
    let Some(benchmark_iterations) = NonZeroU32::new(document.benchmark.iterations) else {
        return Err(ConfigureErrorReturn::ZeroBenchmarkIterations);
    };
    Ok(ConfigureSuccessReturn {
        configuration: HarnessConfiguration {
            selections,
            network: statements.network,
            statements: statements.statements,
            generate: generate.concrete,
            generate_kinds: generate.kinds,
            benchmark_iterations,
            output: destinations,
        },
    })
}
