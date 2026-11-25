mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::KdfConcrete;
use interface::{
    AdmitKdfSelectionDeps, AdmitKdfSelectionErrorReturn, AdmitKdfSelectionParams,
    AdmitKdfSelectionPayload, AdmitKdfSelectionPayloadError, AdmitKdfSelectionReturn,
    AdmitKdfSelectionSuccessReturn,
};
use serde_json::{Map, Value};

impl TryFrom<Value> for AdmitKdfSelectionPayload {
    type Error = AdmitKdfSelectionPayloadError;

    fn try_from(payload: Value) -> Result<AdmitKdfSelectionPayload, AdmitKdfSelectionPayloadError> {
        match payload {
            Value::Null => Ok(AdmitKdfSelectionPayload {
                settings: Map::new(),
            }),
            Value::Object(table) => Ok(AdmitKdfSelectionPayload { settings: table }),
            _ => Err(AdmitKdfSelectionPayloadError::NotATable),
        }
    }
}

pub fn admit_kdf_selection(
    _deps: &AdmitKdfSelectionDeps,
    params: AdmitKdfSelectionParams,
    payload: Value,
) -> AdmitKdfSelectionReturn {
    let payload = match AdmitKdfSelectionPayload::try_from(payload) {
        Ok(payload) => payload,
        Err(error) => return Err(AdmitKdfSelectionErrorReturn::Document(error)),
    };
    match params.concrete {
        KdfConcrete::Blake3Keyed => {
            if payload.settings.is_empty() {
                Ok(AdmitKdfSelectionSuccessReturn {
                    selection: KdfConcrete::Blake3Keyed,
                })
            } else {
                Err(AdmitKdfSelectionErrorReturn::SettingSupplied {
                    concrete: KdfConcrete::Blake3Keyed,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        KdfConcrete::Mock(failure_mode) => {
            if payload.settings.is_empty() {
                Ok(AdmitKdfSelectionSuccessReturn {
                    selection: KdfConcrete::Mock(failure_mode),
                })
            } else {
                Err(AdmitKdfSelectionErrorReturn::SettingSupplied {
                    concrete: KdfConcrete::Mock(failure_mode),
                })
            }
        }
    }
}
