use crate::factory::provides::RandomSourceKind;

pub struct AdmitRandomSelectionDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitRandomSelectionParams {
    pub identifier: RandomSourceKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitRandomSelectionSuccessReturn {
    pub selection: RandomSourceKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitRandomSelectionErrorReturn {
    SettingsSupplied { identifier: RandomSourceKind },
}

pub type AdmitRandomSelectionReturn =
    Result<AdmitRandomSelectionSuccessReturn, AdmitRandomSelectionErrorReturn>;

pub type AdmitRandomSelectionFn = fn(
    &AdmitRandomSelectionDeps,
    AdmitRandomSelectionParams,
    serde_json::Value,
) -> AdmitRandomSelectionReturn;
