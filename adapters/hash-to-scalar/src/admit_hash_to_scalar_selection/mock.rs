#![allow(dead_code)]

use super::interface::{
    AdmitHashToScalarSelectionDeps, AdmitHashToScalarSelectionFn, AdmitHashToScalarSelectionParams,
    AdmitHashToScalarSelectionReturn, AdmitHashToScalarSelectionSuccessReturn,
};
use crate::factory::provides::{HashToScalarConcrete, MockIHashToScalarAdapterFailureMode};
use crate::lookup_hash_to_scalar_declaration::provides::{
    LookupHashToScalarDeclarationFn, mock_lookup_hash_to_scalar_declaration,
};

#[derive(Default)]
pub struct AdmitHashToScalarSelectionDepsOverrides {
    pub lookup_declaration: Option<LookupHashToScalarDeclarationFn>,
}

pub fn build_admit_hash_to_scalar_selection_deps(
    overrides: AdmitHashToScalarSelectionDepsOverrides,
) -> AdmitHashToScalarSelectionDeps {
    AdmitHashToScalarSelectionDeps {
        lookup_declaration: overrides
            .lookup_declaration
            .unwrap_or(mock_lookup_hash_to_scalar_declaration),
    }
}

#[derive(Default)]
pub struct AdmitHashToScalarSelectionParamsOverrides {
    pub concrete: Option<HashToScalarConcrete>,
}

pub fn build_admit_hash_to_scalar_selection_params(
    overrides: AdmitHashToScalarSelectionParamsOverrides,
) -> AdmitHashToScalarSelectionParams {
    AdmitHashToScalarSelectionParams {
        concrete: overrides.concrete.unwrap_or(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::Succeeds,
        )),
    }
}

#[derive(Default)]
pub struct AdmitHashToScalarSelectionSuccessReturnOverrides {
    pub selection: Option<HashToScalarConcrete>,
}

pub fn build_admit_hash_to_scalar_selection_success_return(
    overrides: AdmitHashToScalarSelectionSuccessReturnOverrides,
) -> AdmitHashToScalarSelectionSuccessReturn {
    AdmitHashToScalarSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::Succeeds,
        )),
    }
}

pub fn mock_admit_hash_to_scalar_selection(
    _deps: &AdmitHashToScalarSelectionDeps,
    _params: AdmitHashToScalarSelectionParams,
    _payload: serde_json::Value,
) -> AdmitHashToScalarSelectionReturn {
    Ok(build_admit_hash_to_scalar_selection_success_return(
        Default::default(),
    ))
}

const _: AdmitHashToScalarSelectionFn = mock_admit_hash_to_scalar_selection;
