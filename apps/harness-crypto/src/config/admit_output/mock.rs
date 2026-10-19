#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::super::layers::provides::OutputDocument;
use super::interface::{
    AdmitOutputDeps, AdmitOutputFn, AdmitOutputParams, AdmitOutputPayload, AdmitOutputReturn,
    AdmitOutputSuccessReturn, OutputDestination,
};

#[derive(Default)]
pub struct AdmitOutputDepsOverrides;

pub fn build_admit_output_deps(_overrides: AdmitOutputDepsOverrides) -> AdmitOutputDeps {
    AdmitOutputDeps
}

#[derive(Default)]
pub struct AdmitOutputParamsOverrides;

pub fn build_admit_output_params(_overrides: AdmitOutputParamsOverrides) -> AdmitOutputParams {
    AdmitOutputParams
}

#[derive(Default)]
pub struct AdmitOutputPayloadOverrides;

pub fn build_admit_output_payload<'a>(
    output: &'a OutputDocument,
    _overrides: AdmitOutputPayloadOverrides,
) -> AdmitOutputPayload<'a> {
    AdmitOutputPayload { output }
}

#[derive(Default)]
pub struct AdmitOutputSuccessReturnOverrides {
    pub destinations: Option<Vec<OutputDestination>>,
}

pub fn build_admit_output_success_return(
    overrides: AdmitOutputSuccessReturnOverrides,
) -> AdmitOutputSuccessReturn {
    AdmitOutputSuccessReturn {
        destinations: overrides
            .destinations
            .unwrap_or_else(|| vec![OutputDestination::StandardOutput]),
    }
}

pub fn mock_admit_output(
    _deps: &AdmitOutputDeps,
    _params: AdmitOutputParams,
    _payload: AdmitOutputPayload<'_>,
) -> AdmitOutputReturn {
    let _: AdmitOutputFn = mock_admit_output;
    Ok(build_admit_output_success_return(Default::default()))
}
