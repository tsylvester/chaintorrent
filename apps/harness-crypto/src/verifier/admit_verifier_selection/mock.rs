#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use serde_json;

use super::super::evm::provides::AdmitEvmVerifierSettingsFn;
use super::super::factory::provides::{
    VerifierConcrete, VerifierConcreteKind, VerifierConcreteOverrides, build_verifier_concrete,
};
use super::super::lookup_verifier_declaration::provides::LookupVerifierDeclarationFn;
use super::interface::{
    AdmitVerifierSelectionDeps, AdmitVerifierSelectionFn, AdmitVerifierSelectionParams,
    AdmitVerifierSelectionReturn, AdmitVerifierSelectionSuccessReturn,
};

#[derive(Default)]
pub(crate) struct AdmitVerifierSelectionDepsOverrides;

pub(crate) fn build_admit_verifier_selection_deps(
    lookup_verifier_declaration: LookupVerifierDeclarationFn,
    admit_evm_verifier_settings: AdmitEvmVerifierSettingsFn,
    _overrides: AdmitVerifierSelectionDepsOverrides,
) -> AdmitVerifierSelectionDeps {
    AdmitVerifierSelectionDeps {
        lookup_verifier_declaration,
        admit_evm_verifier_settings,
    }
}

#[derive(Default)]
pub struct AdmitVerifierSelectionParamsOverrides {
    pub identifier: Option<VerifierConcreteKind>,
}

pub fn build_admit_verifier_selection_params(
    overrides: AdmitVerifierSelectionParamsOverrides,
) -> AdmitVerifierSelectionParams {
    AdmitVerifierSelectionParams {
        identifier: overrides.identifier.unwrap_or(VerifierConcreteKind::Evm),
    }
}

#[derive(Default)]
pub struct AdmitVerifierSelectionSuccessReturnOverrides {
    pub selection: Option<VerifierConcrete>,
}

pub fn build_admit_verifier_selection_success_return(
    overrides: AdmitVerifierSelectionSuccessReturnOverrides,
) -> AdmitVerifierSelectionSuccessReturn {
    AdmitVerifierSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or_else(|| {
            build_verifier_concrete(VerifierConcreteOverrides {
                kind: Some(VerifierConcreteKind::Evm),
                ..Default::default()
            })
        }),
    }
}

pub fn mock_admit_verifier_selection(
    _deps: &AdmitVerifierSelectionDeps,
    _params: AdmitVerifierSelectionParams,
    _payload: serde_json::Value,
) -> AdmitVerifierSelectionReturn {
    let _: AdmitVerifierSelectionFn = mock_admit_verifier_selection;
    Ok(build_admit_verifier_selection_success_return(
        Default::default(),
    ))
}
