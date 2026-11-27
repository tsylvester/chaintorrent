#![allow(dead_code)]

use super::interface::{
    AdmitEnvelopeSelectionDeps, AdmitEnvelopeSelectionFn, AdmitEnvelopeSelectionParams,
    AdmitEnvelopeSelectionReturn, AdmitEnvelopeSelectionSuccessReturn,
};
use crate::factory::provides::{KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode};
use serde_json::Value;

#[derive(Default)]
pub struct AdmitEnvelopeSelectionDepsOverrides;

pub fn build_admit_envelope_selection_deps(
    _overrides: AdmitEnvelopeSelectionDepsOverrides,
) -> AdmitEnvelopeSelectionDeps {
    AdmitEnvelopeSelectionDeps
}

#[derive(Default)]
pub struct AdmitEnvelopeSelectionParamsOverrides {
    pub concrete: Option<KeyAgreementConcrete>,
}

pub fn build_admit_envelope_selection_params(
    overrides: AdmitEnvelopeSelectionParamsOverrides,
) -> AdmitEnvelopeSelectionParams {
    AdmitEnvelopeSelectionParams {
        concrete: overrides.concrete.unwrap_or(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
    }
}

#[derive(Default)]
pub struct AdmitEnvelopeSelectionSuccessReturnOverrides {
    pub selection: Option<KeyAgreementConcrete>,
}

pub fn build_admit_envelope_selection_success_return(
    overrides: AdmitEnvelopeSelectionSuccessReturnOverrides,
) -> AdmitEnvelopeSelectionSuccessReturn {
    AdmitEnvelopeSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
    }
}

pub fn mock_admit_envelope_selection(
    _deps: &AdmitEnvelopeSelectionDeps,
    _params: AdmitEnvelopeSelectionParams,
    _payload: Value,
) -> AdmitEnvelopeSelectionReturn {
    Ok(build_admit_envelope_selection_success_return(
        Default::default(),
    ))
}

const _: AdmitEnvelopeSelectionFn = mock_admit_envelope_selection;
