use crate::factory::provides::KdfConcrete;
use serde_json::Value;

pub struct AdmitKdfSelectionDeps;

pub struct AdmitKdfSelectionParams {
    pub concrete: KdfConcrete,
}

pub struct AdmitKdfSelectionSuccessReturn {
    pub selection: KdfConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitKdfSelectionErrorReturn {
    SettingSupplied { concrete: KdfConcrete },
}

pub type AdmitKdfSelectionReturn =
    Result<AdmitKdfSelectionSuccessReturn, AdmitKdfSelectionErrorReturn>;

pub type AdmitKdfSelectionFn =
    fn(&AdmitKdfSelectionDeps, AdmitKdfSelectionParams, Value) -> AdmitKdfSelectionReturn;
