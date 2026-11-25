use crate::factory::provides::EncodingConcrete;

pub struct AdmitEncodingSelectionDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitEncodingSelectionParams {
    pub concrete: EncodingConcrete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitEncodingSelectionSuccessReturn {
    pub selection: EncodingConcrete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitEncodingSelectionErrorReturn {
    SettingSupplied { concrete: EncodingConcrete },
}

pub type AdmitEncodingSelectionReturn =
    Result<AdmitEncodingSelectionSuccessReturn, AdmitEncodingSelectionErrorReturn>;

pub type AdmitEncodingSelectionFn = fn(
    &AdmitEncodingSelectionDeps,
    AdmitEncodingSelectionParams,
    serde_json::Value,
) -> AdmitEncodingSelectionReturn;
