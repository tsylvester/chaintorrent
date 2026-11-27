mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::KeyAgreementConcrete;
use interface::{
    AdmitEnvelopeSelectionDeps, AdmitEnvelopeSelectionErrorReturn, AdmitEnvelopeSelectionParams,
    AdmitEnvelopeSelectionReturn, AdmitEnvelopeSelectionSuccessReturn,
};
use serde_json::Value;

pub fn admit_envelope_selection(
    _deps: &AdmitEnvelopeSelectionDeps,
    params: AdmitEnvelopeSelectionParams,
    payload: serde_json::Value,
) -> AdmitEnvelopeSelectionReturn {
    match &params.concrete {
        KeyAgreementConcrete::PairingElGamal => match &payload {
            Value::Null => Ok(AdmitEnvelopeSelectionSuccessReturn {
                selection: params.concrete,
            }),
            Value::Object(map) if map.is_empty() => Ok(AdmitEnvelopeSelectionSuccessReturn {
                selection: params.concrete,
            }),
            _ => Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied {
                concrete: params.concrete,
            }),
        },
        #[cfg(any(test, feature = "mocks"))]
        KeyAgreementConcrete::Mock(failure_mode) => match &payload {
            Value::Null => Ok(AdmitEnvelopeSelectionSuccessReturn {
                selection: KeyAgreementConcrete::Mock(*failure_mode),
            }),
            Value::Object(map) if map.is_empty() => Ok(AdmitEnvelopeSelectionSuccessReturn {
                selection: KeyAgreementConcrete::Mock(*failure_mode),
            }),
            _ => Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied {
                concrete: KeyAgreementConcrete::Mock(*failure_mode),
            }),
        },
    }
}
