#![allow(dead_code)]

use super::interface::{
    AdmitKdfSelectionDeps, AdmitKdfSelectionFn, AdmitKdfSelectionParams, AdmitKdfSelectionReturn,
    AdmitKdfSelectionSuccessReturn,
};
use crate::factory::provides::{KdfConcrete, KdfConcreteOverrides, build_kdf_concrete};
use serde_json::Value;

#[derive(Default)]
pub struct AdmitKdfSelectionDepsOverrides;

pub fn build_admit_kdf_selection_deps(
    _overrides: AdmitKdfSelectionDepsOverrides,
) -> AdmitKdfSelectionDeps {
    AdmitKdfSelectionDeps
}

#[derive(Default)]
pub struct AdmitKdfSelectionParamsOverrides {
    pub concrete: Option<KdfConcrete>,
}

pub fn build_admit_kdf_selection_params(
    overrides: AdmitKdfSelectionParamsOverrides,
) -> AdmitKdfSelectionParams {
    AdmitKdfSelectionParams {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_kdf_concrete(KdfConcrete::Blake3Keyed, KdfConcreteOverrides)),
    }
}

#[derive(Default)]
pub struct AdmitKdfSelectionSuccessReturnOverrides {
    pub selection: Option<KdfConcrete>,
}

pub fn build_admit_kdf_selection_success_return(
    overrides: AdmitKdfSelectionSuccessReturnOverrides,
) -> AdmitKdfSelectionSuccessReturn {
    AdmitKdfSelectionSuccessReturn {
        selection: overrides
            .selection
            .unwrap_or_else(|| build_kdf_concrete(KdfConcrete::Blake3Keyed, KdfConcreteOverrides)),
    }
}

pub fn mock_admit_kdf_selection(
    _deps: &AdmitKdfSelectionDeps,
    _params: AdmitKdfSelectionParams,
    _payload: Value,
) -> AdmitKdfSelectionReturn {
    Ok(build_admit_kdf_selection_success_return(Default::default()))
}

const _: AdmitKdfSelectionFn = mock_admit_kdf_selection;
