#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    AdmitChainSelectionDeps, AdmitChainSelectionFn, AdmitChainSelectionParams,
    AdmitChainSelectionReturn, AdmitChainSelectionSuccessReturn,
};
use crate::base::provides::{
    AdmitBaseChainSettingsDeps, AdmitBaseChainSettingsParams, AdmitBaseChainSettingsPayload,
    AdmitBaseChainSettingsReturn,
};
use crate::factory::provides::{ChainMemberIdentifier, ChainSelection, build_chain_selection};
use crate::lookup_chain_declaration::provides::LookupChainDeclarationFn;

#[derive(Default)]
pub(crate) struct AdmitChainSelectionDepsOverrides;

pub(crate) fn build_admit_chain_selection_deps(
    lookup_chain_declaration: LookupChainDeclarationFn,
    admit_base_chain_settings: for<'a> fn(
        &AdmitBaseChainSettingsDeps<'a>,
        AdmitBaseChainSettingsParams,
        AdmitBaseChainSettingsPayload,
    ) -> AdmitBaseChainSettingsReturn,
    _overrides: AdmitChainSelectionDepsOverrides,
) -> AdmitChainSelectionDeps {
    AdmitChainSelectionDeps {
        lookup_chain_declaration,
        admit_base_chain_settings,
    }
}

#[derive(Default)]
pub struct AdmitChainSelectionParamsOverrides {
    pub identifier: Option<ChainMemberIdentifier>,
}

pub fn build_admit_chain_selection_params(
    overrides: AdmitChainSelectionParamsOverrides,
) -> AdmitChainSelectionParams {
    AdmitChainSelectionParams {
        identifier: overrides.identifier.unwrap_or(ChainMemberIdentifier::Base),
    }
}

#[derive(Default)]
pub struct AdmitChainSelectionSuccessReturnOverrides {
    pub selection: Option<ChainSelection>,
}

pub fn build_admit_chain_selection_success_return(
    overrides: AdmitChainSelectionSuccessReturnOverrides,
) -> AdmitChainSelectionSuccessReturn {
    AdmitChainSelectionSuccessReturn {
        selection: overrides.selection.unwrap_or_else(|| {
            build_chain_selection(ChainMemberIdentifier::Base, Default::default())
        }),
    }
}

pub fn mock_admit_chain_selection(
    _deps: &AdmitChainSelectionDeps,
    _params: AdmitChainSelectionParams,
    _payload: serde_json::Value,
) -> AdmitChainSelectionReturn {
    Ok(build_admit_chain_selection_success_return(
        Default::default(),
    ))
}

const _: AdmitChainSelectionFn = mock_admit_chain_selection;
