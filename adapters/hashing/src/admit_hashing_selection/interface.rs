use crate::factory::provides::CommitmentConcrete;
use serde_json::Value;

pub struct AdmitHashingSelectionDeps;

pub struct AdmitHashingSelectionParams {
    pub concrete: CommitmentConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AdmitHashingSelectionSuccessReturn {
    pub selection: CommitmentConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitHashingSelectionErrorReturn {
    SettingsSupplied { concrete: CommitmentConcrete },
}

pub type AdmitHashingSelectionReturn =
    Result<AdmitHashingSelectionSuccessReturn, AdmitHashingSelectionErrorReturn>;

pub type AdmitHashingSelectionFn = fn(
    &AdmitHashingSelectionDeps,
    AdmitHashingSelectionParams,
    Value,
) -> AdmitHashingSelectionReturn;
