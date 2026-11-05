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

use interface::{
    ReportDeps, ReportDestination, ReportErrorReturn, ReportFn, ReportParams, ReportPayload,
    ReportReturn, ReportSuccessReturn, ReportText, ReportTextConstructorParams,
    ReportTextTryNewErrorReturn, ReportTextTryNewReturn,
};
use serde_json::{Value, json};
use std::io::Write;

use crate::verifier::provides::L1DataFee;

impl ReportText {
    pub fn try_new(params: ReportTextConstructorParams) -> ReportTextTryNewReturn {
        if params.text.trim().is_empty() {
            return Err(ReportTextTryNewErrorReturn::Empty);
        }
        Ok(ReportText { text: params.text })
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

pub fn report(_deps: &ReportDeps, params: ReportParams, payload: ReportPayload) -> ReportReturn {
    let _: ReportFn = report;
    let measurement = &payload.measurement;
    let timings = &measurement.timings;
    let sizes = &measurement.sizes;
    let costs = &measurement.costs;
    let l1_data_fee = |fee: L1DataFee| -> Value {
        match fee {
            L1DataFee::Charged { wei } => json!({ "state": "charged", "wei": wei.to_string() }),
            L1DataFee::NotApplicable => json!({ "state": "not_applicable" }),
        }
    };
    let adjustment = match &payload.adjustment {
        Some(adjustment) => json!({
            "piece_group_size_bytes": adjustment.piece_group_size.map(|size| size.get()),
            "primary_curve": adjustment.primary_curve.as_ref().map(|curve| curve.as_str()),
        }),
        None => Value::Null,
    };
    let document = json!({
        "curve": payload.curve.as_str(),
        "conditions": payload.conditions.as_str(),
        "iterations": measurement.iterations.get(),
        "timings": {
            "prove_mint": {
                "seconds": timings.prove_mint.as_secs(),
                "subsec_nanos": timings.prove_mint.subsec_nanos()
            },
            "prove_transfer": {
                "seconds": timings.prove_transfer.as_secs(),
                "subsec_nanos": timings.prove_transfer.subsec_nanos()
            },
            "verify_mint": {
                "seconds": timings.verify_mint.as_secs(),
                "subsec_nanos": timings.verify_mint.subsec_nanos()
            },
            "verify_transfer": {
                "seconds": timings.verify_transfer.as_secs(),
                "subsec_nanos": timings.verify_transfer.subsec_nanos()
            },
            "decapsulate": {
                "seconds": timings.decapsulate.as_secs(),
                "subsec_nanos": timings.decapsulate.subsec_nanos()
            }
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
        },
        "defaults_in_force": {
            "piece_group_size_bytes": payload.defaults.piece_group_size.get(),
            "primary_curve": payload.defaults.primary_curve.as_str()
        },
        "adjustment": adjustment
    });
    let document = format!("{document:#}");
    for destination in params.destinations {
        match destination {
            ReportDestination::File { path } => {
                if let Err(error) = std::fs::write(&path, document.as_bytes()) {
                    return Err(ReportErrorReturn::WriteFile { path, error });
                }
            }
            ReportDestination::StandardOutput => {
                if let Err(error) = std::io::stdout().write_all(document.as_bytes()) {
                    return Err(ReportErrorReturn::WriteStandardOutput(error));
                }
            }
        }
    }
    Ok(ReportSuccessReturn { document })
}
