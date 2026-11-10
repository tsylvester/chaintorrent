#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use core::time::Duration;
use interface::{
    OutputDeps, OutputErrorReturn, OutputFn, OutputParams, OutputPayload, OutputReturn,
    OutputSuccessReturn,
};
use serde_json::{Value, json};
use std::io::Write;

use crate::config::provides::OutputDestination;
use crate::run::provides::RunReturn;
use crate::verifier::provides::L1DataFee;

pub fn output(_deps: &OutputDeps, params: OutputParams, payload: OutputPayload) -> OutputReturn {
    let _: OutputFn = output;
    let text = render_outcome(&payload.outcome);
    for destination in params.destinations {
        match destination {
            OutputDestination::File { path } => {
                if let Err(error) = std::fs::write(&path, text.as_bytes()) {
                    return Err(OutputErrorReturn::WriteFile { path, error });
                }
            }
            OutputDestination::StandardOutput => {
                if let Err(error) = std::io::stdout().write_all(text.as_bytes()) {
                    return Err(OutputErrorReturn::WriteStandardOutput(error));
                }
            }
        }
    }
    Ok(OutputSuccessReturn { text })
}

fn render_outcome(outcome: &RunReturn) -> String {
    let duration = |value: Duration| -> Value {
        json!({ "seconds": value.as_secs(), "subsec_nanos": value.subsec_nanos() })
    };
    let l1_data_fee = |fee: L1DataFee| -> Value {
        match fee {
            L1DataFee::Charged { wei } => json!({ "state": "charged", "wei": wei.to_string() }),
            L1DataFee::NotApplicable => json!({ "state": "not_applicable" }),
        }
    };
    let document = match outcome {
        Ok(success) => json!({
            "outcome": "succeeded",
            "benchmarks": success
                .benchmarks
                .iter()
                .map(|benchmark| {
                    let timings = &benchmark.timings;
                    json!({
                        "concrete": format!("{:?}", benchmark.concrete),
                        "iterations": benchmark.iterations.get(),
                        "timings": {
                            "mul_g1": duration(timings.mul_g1),
                            "mul_g2": duration(timings.mul_g2),
                            "msm_g1": duration(timings.msm_g1),
                            "msm_g2": duration(timings.msm_g2),
                            "pairing_product": duration(timings.pairing_product)
                        }
                    })
                })
                .collect::<Vec<Value>>(),
            "curves": success
                .curves
                .iter()
                .map(|curve| {
                    let measurement = &curve.measurement;
                    let timings = &measurement.timings;
                    let sizes = &measurement.sizes;
                    let costs = &measurement.costs;
                    json!({
                        "label": curve.label.as_str(),
                        "files": curve
                            .files
                            .iter()
                            .map(|file| {
                                json!({
                                    "kind": match file.kind {
                                        Some(kind) => Value::from(format!("{kind:?}")),
                                        None => Value::Null,
                                    },
                                    "path": file.path.to_string_lossy().into_owned()
                                })
                            })
                            .collect::<Vec<Value>>(),
                        "measurement": {
                            "iterations": measurement.iterations.get(),
                            "timings": {
                                "prove_mint": duration(timings.prove_mint),
                                "prove_transfer": duration(timings.prove_transfer),
                                "verify_mint": duration(timings.verify_mint),
                                "verify_transfer": duration(timings.verify_transfer),
                                "decapsulate": duration(timings.decapsulate)
                            },
                            "sizes_bytes": {
                                "capsule": sizes.capsule,
                                "envelope": sizes.envelope,
                                "mint_proof": sizes.mint_proof,
                                "transfer_proof": sizes.transfer_proof
                            },
                            "costs": {
                                "mint": {
                                    "l2_execution_gas": costs.mint.l2_execution_gas,
                                    "l1_data_fee": l1_data_fee(costs.mint.l1_data_fee)
                                },
                                "transfer": {
                                    "l2_execution_gas": costs.transfer.l2_execution_gas,
                                    "l1_data_fee": l1_data_fee(costs.transfer.l1_data_fee)
                                }
                            }
                        },
                        "report": curve.report.document.as_str()
                    })
                })
                .collect::<Vec<Value>>()
        }),
        Err(error) => json!({ "outcome": "refused", "refusal": format!("{error:?}") }),
    };
    format!("{document:#}")
}
