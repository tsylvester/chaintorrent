mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::HashToScalarConcrete;
use interface::{
    AdmitHashToScalarSelectionDeps, AdmitHashToScalarSelectionErrorReturn,
    AdmitHashToScalarSelectionParams, AdmitHashToScalarSelectionReturn,
    AdmitHashToScalarSelectionSuccessReturn,
};

pub fn admit_hash_to_scalar_selection(
    _deps: &AdmitHashToScalarSelectionDeps,
    params: AdmitHashToScalarSelectionParams,
    payload: serde_json::Value,
) -> AdmitHashToScalarSelectionReturn {
    let document_supplies_no_setting =
        payload.is_null() || payload.as_object().is_some_and(|map| map.is_empty());
    match params.concrete {
        HashToScalarConcrete::Keccak256 => {
            if document_supplies_no_setting {
                Ok(AdmitHashToScalarSelectionSuccessReturn {
                    selection: params.concrete,
                })
            } else {
                Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied {
                    concrete: params.concrete,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        HashToScalarConcrete::Mock(_) => {
            if document_supplies_no_setting {
                Ok(AdmitHashToScalarSelectionSuccessReturn {
                    selection: params.concrete,
                })
            } else {
                Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied {
                    concrete: params.concrete,
                })
            }
        }
    }
}
