use crate::factory::provides::KeyAgreementConcrete;
use serde_json::Value;

pub struct AdmitEnvelopeSelectionDeps;

pub struct AdmitEnvelopeSelectionParams {
    pub concrete: KeyAgreementConcrete,
}

pub struct AdmitEnvelopeSelectionSuccessReturn {
    pub selection: KeyAgreementConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitEnvelopeSelectionErrorReturn {
    SettingSupplied { concrete: KeyAgreementConcrete },
}

pub type AdmitEnvelopeSelectionReturn =
    Result<AdmitEnvelopeSelectionSuccessReturn, AdmitEnvelopeSelectionErrorReturn>;

pub type AdmitEnvelopeSelectionFn = fn(
    &AdmitEnvelopeSelectionDeps,
    AdmitEnvelopeSelectionParams,
    Value,
) -> AdmitEnvelopeSelectionReturn;
