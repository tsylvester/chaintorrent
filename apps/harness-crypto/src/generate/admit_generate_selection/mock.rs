#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use serde_json;

use super::super::factory::provides::{
    GenerateConcrete, GenerateConcreteKind, build_generate_concrete,
};
use super::super::lookup_generate_declaration::provides::LookupGenerateDeclarationFn;
use super::interface::{
    AdmitGenerateSelectionDeps, AdmitGenerateSelectionFn, AdmitGenerateSelectionParams,
    AdmitGenerateSelectionReturn, AdmitGenerateSelectionSuccessReturn,
    AdmitSolidityGenerateSettingsHandle,
};

#[derive(Default)]
pub struct AdmitGenerateSelectionDepsOverrides;

pub fn build_admit_generate_selection_deps(
    lookup_declaration: LookupGenerateDeclarationFn,
    admit_solidity_settings: AdmitSolidityGenerateSettingsHandle,
    _overrides: AdmitGenerateSelectionDepsOverrides,
) -> AdmitGenerateSelectionDeps {
    AdmitGenerateSelectionDeps {
        lookup_declaration,
        admit_solidity_settings,
    }
}

#[derive(Default)]
pub struct AdmitGenerateSelectionParamsOverrides {
    pub identifier: Option<GenerateConcreteKind>,
}

pub fn build_admit_generate_selection_params(
    overrides: AdmitGenerateSelectionParamsOverrides,
) -> AdmitGenerateSelectionParams {
    AdmitGenerateSelectionParams {
        identifier: overrides
            .identifier
            .unwrap_or(GenerateConcreteKind::Solidity),
    }
}

#[derive(Default)]
pub struct AdmitGenerateSelectionSuccessReturnOverrides {
    pub selection: Option<GenerateConcrete>,
}

pub fn build_admit_generate_selection_success_return(
    overrides: AdmitGenerateSelectionSuccessReturnOverrides,
) -> AdmitGenerateSelectionSuccessReturn {
    AdmitGenerateSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or_else(|| {
            build_generate_concrete(GenerateConcreteKind::Solidity, Default::default())
        }),
    }
}

pub fn mock_admit_generate_selection(
    _deps: &AdmitGenerateSelectionDeps,
    _params: AdmitGenerateSelectionParams,
    _payload: serde_json::Value,
) -> AdmitGenerateSelectionReturn {
    let _: AdmitGenerateSelectionFn = mock_admit_generate_selection;
    Ok(build_admit_generate_selection_success_return(
        Default::default(),
    ))
}
