#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    AdmitRandomSelectionDeps, AdmitRandomSelectionFn, AdmitRandomSelectionParams,
    AdmitRandomSelectionReturn, AdmitRandomSelectionSuccessReturn,
};
use crate::factory::provides::{RandomSourceKind, build_random_source_kind};

#[derive(Default)]
pub struct AdmitRandomSelectionDepsOverrides;

pub fn build_admit_random_selection_deps(
    _overrides: AdmitRandomSelectionDepsOverrides,
) -> AdmitRandomSelectionDeps {
    AdmitRandomSelectionDeps
}

#[derive(Default)]
pub struct AdmitRandomSelectionParamsOverrides {
    pub identifier: Option<RandomSourceKind>,
}

pub fn build_admit_random_selection_params(
    overrides: AdmitRandomSelectionParamsOverrides,
) -> AdmitRandomSelectionParams {
    AdmitRandomSelectionParams {
        identifier: overrides
            .identifier
            .unwrap_or(RandomSourceKind::OperatingSystem),
    }
}

#[derive(Default)]
pub struct AdmitRandomSelectionSuccessReturnOverrides {
    pub selection: Option<RandomSourceKind>,
}

pub fn build_admit_random_selection_success_return(
    overrides: AdmitRandomSelectionSuccessReturnOverrides,
) -> AdmitRandomSelectionSuccessReturn {
    AdmitRandomSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or_else(|| {
            build_random_source_kind(RandomSourceKind::OperatingSystem, Default::default())
        }),
    }
}

pub fn mock_admit_random_selection(
    _deps: &AdmitRandomSelectionDeps,
    _params: AdmitRandomSelectionParams,
    _payload: serde_json::Value,
) -> AdmitRandomSelectionReturn {
    Ok(build_admit_random_selection_success_return(
        Default::default(),
    ))
}

const _: AdmitRandomSelectionFn = mock_admit_random_selection;
