mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::KemConcrete;
use interface::{
    AdmitKemSelectionDeps, AdmitKemSelectionErrorReturn, AdmitKemSelectionParams,
    AdmitKemSelectionReturn, AdmitKemSelectionSuccessReturn,
};
use serde_json::Value;

pub fn admit_kem_selection(
    _deps: &AdmitKemSelectionDeps,
    params: AdmitKemSelectionParams,
    payload: Value,
) -> AdmitKemSelectionReturn {
    let supplies_setting = match &payload {
        Value::Null => false,
        Value::Bool(_) => true,
        Value::Number(_) => true,
        Value::String(_) => true,
        Value::Array(_) => true,
        Value::Object(entries) => !entries.is_empty(),
    };
    match params.concrete {
        KemConcrete::Bb1DepthOne => {
            if supplies_setting {
                Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
                    concrete: KemConcrete::Bb1DepthOne,
                })
            } else {
                Ok(AdmitKemSelectionSuccessReturn {
                    selection: KemConcrete::Bb1DepthOne,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        KemConcrete::Mock => {
            if supplies_setting {
                Err(AdmitKemSelectionErrorReturn::SettingsSupplied {
                    concrete: KemConcrete::Mock,
                })
            } else {
                Ok(AdmitKemSelectionSuccessReturn {
                    selection: KemConcrete::Mock,
                })
            }
        }
    }
}
