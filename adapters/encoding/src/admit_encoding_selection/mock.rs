#![allow(dead_code)]

use super::interface::{
    AdmitEncodingSelectionDeps, AdmitEncodingSelectionFn, AdmitEncodingSelectionParams,
    AdmitEncodingSelectionReturn, AdmitEncodingSelectionSuccessReturn,
};
use crate::factory::provides::{
    EncodingConcrete, EncodingConcreteOverrides, build_encoding_concrete,
};

#[derive(Default)]
pub struct AdmitEncodingSelectionDepsOverrides;

pub fn build_admit_encoding_selection_deps(
    _overrides: AdmitEncodingSelectionDepsOverrides,
) -> AdmitEncodingSelectionDeps {
    AdmitEncodingSelectionDeps
}

#[derive(Default)]
pub struct AdmitEncodingSelectionParamsOverrides {
    pub concrete: Option<EncodingConcrete>,
}

pub fn build_admit_encoding_selection_params(
    overrides: AdmitEncodingSelectionParamsOverrides,
) -> AdmitEncodingSelectionParams {
    AdmitEncodingSelectionParams {
        concrete: overrides.concrete.unwrap_or_else(|| {
            build_encoding_concrete(EncodingConcrete::Abi, EncodingConcreteOverrides)
        }),
    }
}

#[derive(Default)]
pub struct AdmitEncodingSelectionSuccessReturnOverrides {
    pub selection: Option<EncodingConcrete>,
}

pub fn build_admit_encoding_selection_success_return(
    overrides: AdmitEncodingSelectionSuccessReturnOverrides,
) -> AdmitEncodingSelectionSuccessReturn {
    AdmitEncodingSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or_else(|| {
            build_encoding_concrete(EncodingConcrete::Abi, EncodingConcreteOverrides)
        }),
    }
}

pub fn mock_admit_encoding_selection(
    _deps: &AdmitEncodingSelectionDeps,
    _params: AdmitEncodingSelectionParams,
    _payload: serde_json::Value,
) -> AdmitEncodingSelectionReturn {
    Ok(build_admit_encoding_selection_success_return(
        AdmitEncodingSelectionSuccessReturnOverrides::default(),
    ))
}

const _: AdmitEncodingSelectionFn = mock_admit_encoding_selection;
