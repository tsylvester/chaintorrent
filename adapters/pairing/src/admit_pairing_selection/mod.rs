mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::PairingConcrete;
use interface::{
    AdmitPairingSelectionDeps, AdmitPairingSelectionErrorReturn, AdmitPairingSelectionParams,
    AdmitPairingSelectionReturn, AdmitPairingSelectionSuccessReturn,
};
use serde_json::Value;

pub fn admit_pairing_selection(
    _deps: &AdmitPairingSelectionDeps,
    params: AdmitPairingSelectionParams,
    payload: Value,
) -> AdmitPairingSelectionReturn {
    let supplies_setting = match &payload {
        Value::Null => false,
        Value::Bool(_) => true,
        Value::Number(_) => true,
        Value::String(_) => true,
        Value::Array(_) => true,
        Value::Object(entries) => !entries.is_empty(),
    };
    match params.identifier {
        PairingConcrete::Bn254Arkworks
        | PairingConcrete::Bn254Halo2curves
        | PairingConcrete::Bls12381Arkworks
        | PairingConcrete::Bls12381Halo2curves => {
            if supplies_setting {
                Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
                    identifier: params.identifier,
                })
            } else {
                Ok(AdmitPairingSelectionSuccessReturn {
                    selection: params.identifier,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        PairingConcrete::Mock(_) => {
            if supplies_setting {
                Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
                    identifier: params.identifier,
                })
            } else {
                Ok(AdmitPairingSelectionSuccessReturn {
                    selection: params.identifier,
                })
            }
        }
    }
}
