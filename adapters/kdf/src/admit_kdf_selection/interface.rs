use crate::factory::provides::KdfConcrete;
use serde_json::{Map, Value};

pub struct AdmitKdfSelectionDeps;

pub struct AdmitKdfSelectionParams {
    pub concrete: KdfConcrete,
}

pub struct AdmitKdfSelectionPayload {
    pub settings: Map<String, Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitKdfSelectionPayloadError {
    NotATable,
}

pub struct AdmitKdfSelectionSuccessReturn {
    pub selection: KdfConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitKdfSelectionErrorReturn {
    Document(AdmitKdfSelectionPayloadError),
    SettingSupplied { concrete: KdfConcrete },
}

pub type AdmitKdfSelectionReturn =
    Result<AdmitKdfSelectionSuccessReturn, AdmitKdfSelectionErrorReturn>;

pub type AdmitKdfSelectionFn = fn(
    &AdmitKdfSelectionDeps,
    AdmitKdfSelectionParams,
    serde_json::Value,
) -> AdmitKdfSelectionReturn;
