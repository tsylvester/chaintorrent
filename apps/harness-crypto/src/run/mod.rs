mod admit_contexts;
mod chain_forms_consumer;
mod delivery_proof_consumer;
mod encoding_consumer;
mod generate_consumer;
#[cfg(test)]
mod integration_test;
mod interface;
mod kem_consumer;
mod key_agreement_consumer;
#[cfg(any(test, feature = "mocks"))]
mod mock;
mod pairing_consumer;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;
mod verifier_consumer;

use pairing::{CreatePairingDeps, CreatePairingParams, CreatePairingPayload, create_pairing};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    create_random_source,
};

use crate::benchmark::provides::{PairingBenchmark, PairingBenchmarkConstructorParams};
use crate::report::provides::{ReportDeps, ReportParams, ReportPayload, report};
use crate::run::provides::{PairingConsumer, PairingConsumerConstructorParams};
use interface::{
    RunCurveOutcome, RunDeps, RunErrorReturn, RunFn, RunParams, RunPayload, RunReturn,
    RunSuccessReturn,
};

pub fn run(_deps: &RunDeps, _params: RunParams, payload: RunPayload) -> RunReturn {
    let _: RunFn = run;
    let configuration = payload.configuration;
    let created = match create_random_source(
        &CreateRandomSourceDeps,
        CreateRandomSourceParams {
            kind: configuration.selections.random_source,
        },
        CreateRandomSourcePayload,
    ) {
        Ok(created) => created,
        Err(error) => return Err(RunErrorReturn::CreateRandomSource(error)),
    };
    let mut benchmarks = Vec::new();
    for pairing in &configuration.selections.benchmark_concretes {
        let source = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: configuration.selections.random_source,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => return Err(RunErrorReturn::CreateRandomSource(error)),
        };
        let Ok(consumer) = PairingBenchmark::try_new(PairingBenchmarkConstructorParams {
            random: source.adapter,
            iterations: configuration.benchmark_iterations,
        });
        let output = match create_pairing(
            &CreatePairingDeps { consumer },
            CreatePairingParams {
                concrete: pairing.concrete,
                supported_encodings: configuration.selections.precompile_encodings.clone(),
                target_group_encoding: pairing.target_group_encoding,
            },
            CreatePairingPayload,
        ) {
            Ok(success) => success.output,
            Err(error) => return Err(RunErrorReturn::CreatePairing(error)),
        };
        match output {
            Ok(measurement) => benchmarks.push(measurement),
            Err(error) => return Err(RunErrorReturn::Benchmark(error)),
        }
    }
    let mut curves = Vec::new();
    for ((curve, verifier), label) in configuration
        .selections
        .curves
        .iter()
        .zip(configuration.verifiers.iter())
        .zip(configuration.curve_labels.iter())
    {
        let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
            random: &*created.adapter,
            statements: &configuration.statements,
            identities: &configuration.identities,
            hash_to_scalar: configuration.selections.hash_to_scalar,
            encoding: configuration.selections.encoding,
            chain_forms: configuration.selections.chain_forms,
            kem: configuration.selections.kem,
            key_agreement: configuration.selections.key_agreement,
            delivery_proof: configuration.selections.delivery_proof.concrete,
            generate: configuration.generate.concrete.clone(),
            kinds: configuration.generate.kinds.clone(),
            verifier: verifier.clone(),
            statement_version: configuration.selections.delivery_proof.statement_version,
            iterations: configuration.measurement.iterations,
        });
        let output = match create_pairing(
            &CreatePairingDeps { consumer },
            CreatePairingParams {
                concrete: curve.pairing.concrete,
                supported_encodings: configuration.selections.precompile_encodings.clone(),
                target_group_encoding: curve.pairing.target_group_encoding,
            },
            CreatePairingPayload,
        ) {
            Ok(success) => success.output,
            Err(error) => return Err(RunErrorReturn::CreatePairing(error)),
        };
        let success = match output {
            Ok(success) => success,
            Err(error) => return Err(RunErrorReturn::PairingConsumer(error)),
        };
        let outcome = match report(
            &ReportDeps,
            ReportParams {
                destinations: configuration.output.report_destinations.clone(),
            },
            ReportPayload {
                measurement: success.measurement,
                curve: label.clone(),
                conditions: configuration.measurement.conditions.clone(),
                defaults: configuration.report.defaults.clone(),
                adjustment: configuration.report.adjustment.clone(),
            },
        ) {
            Ok(outcome) => outcome,
            Err(error) => return Err(RunErrorReturn::Report(error)),
        };
        curves.push(RunCurveOutcome {
            label: label.clone(),
            files: success.files,
            measurement: success.measurement,
            report: outcome,
        });
    }
    Ok(RunSuccessReturn { benchmarks, curves })
}
