#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    AdmitPairingSelectionDeps, AdmitPairingSelectionParams, AdmitPairingSelectionReturn,
    AdmitPairingSelectionSuccessReturn,
};
use crate::factory::provides::{PairingConcrete, PairingConcreteOverrides, build_pairing_concrete};
use serde_json::Value;

#[derive(Default)]
pub struct AdmitPairingSelectionDepsOverrides;

pub fn build_admit_pairing_selection_deps(
    _overrides: AdmitPairingSelectionDepsOverrides,
) -> AdmitPairingSelectionDeps {
    AdmitPairingSelectionDeps
}

#[derive(Default)]
pub struct AdmitPairingSelectionParamsOverrides {
    pub identifier: Option<PairingConcrete>,
}

pub fn build_admit_pairing_selection_params(
    overrides: AdmitPairingSelectionParamsOverrides,
) -> AdmitPairingSelectionParams {
    AdmitPairingSelectionParams {
        identifier: overrides
            .identifier
            .unwrap_or(PairingConcrete::Bn254Arkworks),
    }
}

#[derive(Default)]
pub struct AdmitPairingSelectionSuccessReturnOverrides {
    pub selection: Option<PairingConcrete>,
}

pub fn build_admit_pairing_selection_success_return(
    overrides: AdmitPairingSelectionSuccessReturnOverrides,
) -> AdmitPairingSelectionSuccessReturn {
    AdmitPairingSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or_else(|| {
            build_pairing_concrete(PairingConcrete::Bn254Arkworks, PairingConcreteOverrides)
        }),
    }
}

pub fn mock_admit_pairing_selection(
    _deps: &AdmitPairingSelectionDeps,
    _params: AdmitPairingSelectionParams,
    _payload: Value,
) -> AdmitPairingSelectionReturn {
    Ok(build_admit_pairing_selection_success_return(
        Default::default(),
    ))
}
// it is the production function type and nothing else:
// `let f: AdmitPairingSelectionFn = mock_admit_pairing_selection;` holds
