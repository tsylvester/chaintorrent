#![allow(dead_code, clippy::too_many_arguments)]

use crate::factory::provides::{CommitmentConcrete, build_commitment_concrete};
use serde_json::Value;

use super::interface::{
    AdmitHashingSelectionDeps, AdmitHashingSelectionParams, AdmitHashingSelectionReturn,
    AdmitHashingSelectionSuccessReturn,
};

#[derive(Default)]
pub struct AdmitHashingSelectionDepsOverrides;

pub fn build_admit_hashing_selection_deps(
    _overrides: AdmitHashingSelectionDepsOverrides,
) -> AdmitHashingSelectionDeps {
    AdmitHashingSelectionDeps
}

#[derive(Default)]
pub struct AdmitHashingSelectionParamsOverrides {
    pub concrete: Option<CommitmentConcrete>,
}

pub fn build_admit_hashing_selection_params(
    overrides: AdmitHashingSelectionParamsOverrides,
) -> AdmitHashingSelectionParams {
    AdmitHashingSelectionParams {
        concrete: overrides.concrete.unwrap_or(build_commitment_concrete(
            CommitmentConcrete::Blake3Bao,
            Default::default(),
        )),
    }
}

#[derive(Default)]
pub struct AdmitHashingSelectionSuccessReturnOverrides {
    pub selection: Option<CommitmentConcrete>,
}

pub fn build_admit_hashing_selection_success_return(
    overrides: AdmitHashingSelectionSuccessReturnOverrides,
) -> AdmitHashingSelectionSuccessReturn {
    AdmitHashingSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or(build_commitment_concrete(
            CommitmentConcrete::Blake3Bao,
            Default::default(),
        )),
    }
}

pub fn mock_admit_hashing_selection(
    _deps: &AdmitHashingSelectionDeps,
    _params: AdmitHashingSelectionParams,
    _payload: Value,
) -> AdmitHashingSelectionReturn {
    Ok(build_admit_hashing_selection_success_return(
        Default::default(),
    ))
}
