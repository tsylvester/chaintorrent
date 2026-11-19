use crate::base::provides::{
    AdmitBaseChainSettingsDeps, AdmitBaseChainSettingsErrorReturn, AdmitBaseChainSettingsParams,
    AdmitBaseChainSettingsPayload, AdmitBaseChainSettingsReturn,
    BaseChainSettingsDocumentErrorReturn, admit_base_chain_settings,
};
use crate::factory::provides::{ChainMemberIdentifier, ChainSelection};
use crate::lookup_chain_declaration::provides::{
    LookupChainDeclarationFn, lookup_chain_declaration,
};

#[derive(Clone, Copy)]
pub struct AdmitChainSelectionDeps {
    pub(crate) lookup_chain_declaration: LookupChainDeclarationFn,
    pub(crate) admit_base_chain_settings: for<'a> fn(
        &AdmitBaseChainSettingsDeps<'a>,
        AdmitBaseChainSettingsParams,
        AdmitBaseChainSettingsPayload,
    ) -> AdmitBaseChainSettingsReturn,
}

pub const ADMIT_CHAIN_SELECTION_DEPS: AdmitChainSelectionDeps = AdmitChainSelectionDeps {
    lookup_chain_declaration,
    admit_base_chain_settings,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitChainSelectionParams {
    pub identifier: ChainMemberIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitChainSelectionSuccessReturn {
    pub selection: ChainSelection,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitChainSelectionErrorReturn {
    BaseDocument(BaseChainSettingsDocumentErrorReturn),
    Base(AdmitBaseChainSettingsErrorReturn),
}

pub type AdmitChainSelectionReturn =
    Result<AdmitChainSelectionSuccessReturn, AdmitChainSelectionErrorReturn>;

pub type AdmitChainSelectionFn = fn(
    &AdmitChainSelectionDeps,
    AdmitChainSelectionParams,
    serde_json::Value,
) -> AdmitChainSelectionReturn;
