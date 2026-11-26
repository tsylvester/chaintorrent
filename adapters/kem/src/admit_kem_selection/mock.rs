#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    AdmitKemSelectionDeps, AdmitKemSelectionFn, AdmitKemSelectionParams, AdmitKemSelectionReturn,
    AdmitKemSelectionSuccessReturn,
};
use crate::factory::provides::{KemConcrete, KemConcreteOverrides, build_kem_concrete};
use serde_json::Value;

#[derive(Default)]
pub struct AdmitKemSelectionDepsOverrides;

pub fn build_admit_kem_selection_deps(
    _overrides: AdmitKemSelectionDepsOverrides,
) -> AdmitKemSelectionDeps {
    AdmitKemSelectionDeps
}

#[derive(Default)]
pub struct AdmitKemSelectionParamsOverrides {
    pub concrete: Option<KemConcrete>,
}

pub fn build_admit_kem_selection_params(
    overrides: AdmitKemSelectionParamsOverrides,
) -> AdmitKemSelectionParams {
    AdmitKemSelectionParams {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_kem_concrete(KemConcrete::Bb1DepthOne, KemConcreteOverrides)),
    }
}

#[derive(Default)]
pub struct AdmitKemSelectionSuccessReturnOverrides {
    pub selection: Option<KemConcrete>,
}

pub fn build_admit_kem_selection_success_return(
    overrides: AdmitKemSelectionSuccessReturnOverrides,
) -> AdmitKemSelectionSuccessReturn {
    AdmitKemSelectionSuccessReturn {
        selection: overrides
            .selection
            .unwrap_or_else(|| build_kem_concrete(KemConcrete::Bb1DepthOne, KemConcreteOverrides)),
    }
}

pub fn mock_admit_kem_selection(
    _deps: &AdmitKemSelectionDeps,
    _params: AdmitKemSelectionParams,
    _payload: Value,
) -> AdmitKemSelectionReturn {
    let _: AdmitKemSelectionFn = mock_admit_kem_selection;
    Ok(build_admit_kem_selection_success_return(Default::default()))
}
