use serde_json;

use super::super::evm::provides::{
    AdmitEvmVerifierSettingsFn, EvmVerifierSettingsDocumentErrorReturn, admit_evm_verifier_settings,
};
use super::super::factory::provides::{VerifierConcrete, VerifierConcreteKind};
use super::super::lookup_verifier_declaration::provides::{
    LookupVerifierDeclarationFn, lookup_verifier_declaration,
};

#[derive(Clone, Copy)]
pub struct AdmitVerifierSelectionDeps {
    pub(crate) lookup_verifier_declaration: LookupVerifierDeclarationFn,
    pub(crate) admit_evm_verifier_settings: AdmitEvmVerifierSettingsFn,
}

pub(crate) const ADMIT_VERIFIER_SELECTION_DEPS: AdmitVerifierSelectionDeps =
    AdmitVerifierSelectionDeps {
        lookup_verifier_declaration,
        admit_evm_verifier_settings,
    };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitVerifierSelectionParams {
    pub identifier: VerifierConcreteKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitVerifierSelectionSuccessReturn {
    pub selection: VerifierConcrete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitVerifierSelectionErrorReturn {
    EvmDocument(EvmVerifierSettingsDocumentErrorReturn),
}

pub type AdmitVerifierSelectionReturn =
    Result<AdmitVerifierSelectionSuccessReturn, AdmitVerifierSelectionErrorReturn>;

pub type AdmitVerifierSelectionFn = fn(
    &AdmitVerifierSelectionDeps,
    AdmitVerifierSelectionParams,
    serde_json::Value,
) -> AdmitVerifierSelectionReturn;
