use crate::factory::provides::KemConcrete;
use serde_json::Value;

pub struct AdmitKemSelectionDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitKemSelectionParams {
    pub concrete: KemConcrete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitKemSelectionSuccessReturn {
    pub selection: KemConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitKemSelectionErrorReturn {
    SettingsSupplied { concrete: KemConcrete },
}

pub type AdmitKemSelectionReturn =
    Result<AdmitKemSelectionSuccessReturn, AdmitKemSelectionErrorReturn>;

pub type AdmitKemSelectionFn =
    fn(&AdmitKemSelectionDeps, AdmitKemSelectionParams, Value) -> AdmitKemSelectionReturn;
