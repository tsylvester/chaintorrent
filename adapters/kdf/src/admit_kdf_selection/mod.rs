mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::KdfConcrete;
use interface::{
    AdmitKdfSelectionDeps, AdmitKdfSelectionErrorReturn, AdmitKdfSelectionParams,
    AdmitKdfSelectionReturn, AdmitKdfSelectionSuccessReturn,
};
use serde_json::Value;

pub fn admit_kdf_selection(
    _deps: &AdmitKdfSelectionDeps,
    params: AdmitKdfSelectionParams,
    payload: Value,
) -> AdmitKdfSelectionReturn {
    let supplies_setting = !(payload.is_null()
        || payload
            .as_object()
            .is_some_and(|document| document.is_empty()));
    match params.concrete {
        KdfConcrete::Blake3Keyed => {
            if supplies_setting {
                Err(AdmitKdfSelectionErrorReturn::SettingSupplied {
                    concrete: KdfConcrete::Blake3Keyed,
                })
            } else {
                Ok(AdmitKdfSelectionSuccessReturn {
                    selection: KdfConcrete::Blake3Keyed,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        KdfConcrete::Mock(failure_mode) => {
            if supplies_setting {
                Err(AdmitKdfSelectionErrorReturn::SettingSupplied {
                    concrete: KdfConcrete::Mock(failure_mode),
                })
            } else {
                Ok(AdmitKdfSelectionSuccessReturn {
                    selection: KdfConcrete::Mock(failure_mode),
                })
            }
        }
    }
}
