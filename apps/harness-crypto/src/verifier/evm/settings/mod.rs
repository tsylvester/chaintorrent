mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use serde_json::Value;

use self::interface::{
    AdmitEvmVerifierSettingsDeps, AdmitEvmVerifierSettingsParams, AdmitEvmVerifierSettingsPayload,
    AdmitEvmVerifierSettingsReturn, AdmitEvmVerifierSettingsSuccessReturn, EvmVerifierSettings,
    EvmVerifierSettingsDocument, EvmVerifierSettingsDocumentErrorReturn,
};

impl TryFrom<Value> for EvmVerifierSettingsDocument {
    type Error = EvmVerifierSettingsDocumentErrorReturn;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(table) => match table.keys().min() {
                Some(key) => Err(EvmVerifierSettingsDocumentErrorReturn::UnknownKey {
                    path: String::new(),
                    key: key.clone(),
                }),
                None => Ok(EvmVerifierSettingsDocument),
            },
            _ => Err(EvmVerifierSettingsDocumentErrorReturn::NotATable {
                path: String::new(),
            }),
        }
    }
}

pub fn admit_evm_verifier_settings(
    _deps: &AdmitEvmVerifierSettingsDeps,
    _params: AdmitEvmVerifierSettingsParams,
    _payload: AdmitEvmVerifierSettingsPayload,
) -> AdmitEvmVerifierSettingsReturn {
    Ok(AdmitEvmVerifierSettingsSuccessReturn {
        settings: EvmVerifierSettings,
    })
}
