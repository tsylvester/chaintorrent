#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    OutputDeps, OutputFn, OutputParams, OutputPayload, OutputReturn, OutputSuccessReturn,
};
use crate::config::provides::OutputDestination;
use crate::run::provides::{RunReturn, build_run_success_return};

#[derive(Default)]
pub struct OutputDepsOverrides;

pub fn build_output_deps(_overrides: OutputDepsOverrides) -> OutputDeps {
    OutputDeps
}

#[derive(Default)]
pub struct OutputParamsOverrides {
    pub destinations: Option<Vec<OutputDestination>>,
}

pub fn build_output_params(overrides: OutputParamsOverrides) -> OutputParams {
    OutputParams {
        destinations: overrides.destinations.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct OutputPayloadOverrides {
    pub outcome: Option<RunReturn>,
}

pub fn build_output_payload(overrides: OutputPayloadOverrides) -> OutputPayload {
    OutputPayload {
        outcome: overrides
            .outcome
            .unwrap_or_else(|| Ok(build_run_success_return(Default::default()))),
    }
}

#[derive(Default)]
pub struct OutputSuccessReturnOverrides {
    pub text: Option<String>,
}

pub fn build_output_success_return(overrides: OutputSuccessReturnOverrides) -> OutputSuccessReturn {
    OutputSuccessReturn {
        text: overrides.text.unwrap_or_else(|| "{}".to_string()),
    }
}

pub fn mock_output(
    _deps: &OutputDeps,
    _params: OutputParams,
    _payload: OutputPayload,
) -> OutputReturn {
    let _: OutputFn = mock_output;
    Ok(build_output_success_return(Default::default()))
}
