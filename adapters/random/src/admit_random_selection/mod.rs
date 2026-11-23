mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::RandomSourceKind;
use interface::{
    AdmitRandomSelectionDeps, AdmitRandomSelectionErrorReturn, AdmitRandomSelectionParams,
    AdmitRandomSelectionReturn, AdmitRandomSelectionSuccessReturn,
};

pub fn admit_random_selection(
    _deps: &AdmitRandomSelectionDeps,
    params: AdmitRandomSelectionParams,
    payload: serde_json::Value,
) -> AdmitRandomSelectionReturn {
    let supplies_a_setting = match &payload {
        serde_json::Value::Null => false,
        serde_json::Value::Object(entries) => !entries.is_empty(),
        serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_)
        | serde_json::Value::Array(_) => true,
    };
    match params.identifier {
        RandomSourceKind::OperatingSystem => {
            if supplies_a_setting {
                Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
                    identifier: params.identifier,
                })
            } else {
                Ok(AdmitRandomSelectionSuccessReturn {
                    selection: RandomSourceKind::OperatingSystem,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        RandomSourceKind::Mock(mode) => {
            if supplies_a_setting {
                Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
                    identifier: params.identifier,
                })
            } else {
                Ok(AdmitRandomSelectionSuccessReturn {
                    selection: RandomSourceKind::Mock(mode),
                })
            }
        }
    }
}
