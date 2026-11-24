mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::CommitmentConcrete;
use interface::{
    AdmitHashingSelectionDeps, AdmitHashingSelectionErrorReturn, AdmitHashingSelectionParams,
    AdmitHashingSelectionReturn, AdmitHashingSelectionSuccessReturn,
};
use serde_json::Value;

pub fn admit_hashing_selection(
    _deps: &AdmitHashingSelectionDeps,
    params: AdmitHashingSelectionParams,
    payload: Value,
) -> AdmitHashingSelectionReturn {
    match params.concrete {
        CommitmentConcrete::Blake3Bao => admit(params.concrete, &payload),
        #[cfg(any(test, feature = "mocks"))]
        CommitmentConcrete::Mock(_) => admit(params.concrete, &payload),
    }
}

fn admit(concrete: CommitmentConcrete, payload: &Value) -> AdmitHashingSelectionReturn {
    let setting_supplied = match payload {
        Value::Null => false,
        Value::Object(entries) => !entries.is_empty(),
        _ => true,
    };
    if setting_supplied {
        Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete })
    } else {
        Ok(AdmitHashingSelectionSuccessReturn {
            selection: concrete,
        })
    }
}
