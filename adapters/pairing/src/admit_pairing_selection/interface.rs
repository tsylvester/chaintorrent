use crate::factory::provides::PairingConcrete;
use serde_json::Value;

pub struct AdmitPairingSelectionDeps;

pub struct AdmitPairingSelectionParams {
    pub identifier: PairingConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AdmitPairingSelectionSuccessReturn {
    pub selection: PairingConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitPairingSelectionErrorReturn {
    SettingsSupplied { identifier: PairingConcrete },
}

pub type AdmitPairingSelectionReturn =
    Result<AdmitPairingSelectionSuccessReturn, AdmitPairingSelectionErrorReturn>;

pub type AdmitPairingSelectionFn = fn(
    &AdmitPairingSelectionDeps,
    AdmitPairingSelectionParams,
    Value,
) -> AdmitPairingSelectionReturn;
