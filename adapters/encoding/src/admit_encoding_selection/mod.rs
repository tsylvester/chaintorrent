mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::EncodingConcrete;
use interface::{
    AdmitEncodingSelectionDeps, AdmitEncodingSelectionErrorReturn, AdmitEncodingSelectionParams,
    AdmitEncodingSelectionReturn, AdmitEncodingSelectionSuccessReturn,
};

pub fn admit_encoding_selection(
    _deps: &AdmitEncodingSelectionDeps,
    params: AdmitEncodingSelectionParams,
    payload: serde_json::Value,
) -> AdmitEncodingSelectionReturn {
    let supplies_setting = !(payload.is_null()
        || payload
            .as_object()
            .is_some_and(|document| document.is_empty()));
    match params.concrete {
        EncodingConcrete::Abi => {
            if supplies_setting {
                Err(AdmitEncodingSelectionErrorReturn::SettingSupplied {
                    concrete: EncodingConcrete::Abi,
                })
            } else {
                Ok(AdmitEncodingSelectionSuccessReturn {
                    selection: EncodingConcrete::Abi,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        EncodingConcrete::Mock => {
            if supplies_setting {
                Err(AdmitEncodingSelectionErrorReturn::SettingSupplied {
                    concrete: EncodingConcrete::Mock,
                })
            } else {
                Ok(AdmitEncodingSelectionSuccessReturn {
                    selection: EncodingConcrete::Mock,
                })
            }
        }
    }
}
