use crate::factory::provides::HashToScalarConcrete;

pub struct AdmitHashToScalarSelectionDeps;

pub struct AdmitHashToScalarSelectionParams {
    pub concrete: HashToScalarConcrete,
}

pub struct AdmitHashToScalarSelectionSuccessReturn {
    pub selection: HashToScalarConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitHashToScalarSelectionErrorReturn {
    SettingSupplied { concrete: HashToScalarConcrete },
}

pub type AdmitHashToScalarSelectionReturn =
    Result<AdmitHashToScalarSelectionSuccessReturn, AdmitHashToScalarSelectionErrorReturn>;

pub type AdmitHashToScalarSelectionFn = fn(
    &AdmitHashToScalarSelectionDeps,
    AdmitHashToScalarSelectionParams,
    serde_json::Value,
) -> AdmitHashToScalarSelectionReturn;
