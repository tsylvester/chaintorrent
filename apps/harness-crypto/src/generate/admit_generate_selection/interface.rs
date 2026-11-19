use serde_json;

use super::super::factory::provides::{GenerateConcrete, GenerateConcreteKind};
use super::super::lookup_generate_declaration::provides::LookupGenerateDeclarationFn;
use super::super::solidity::provides::{
    AdmitSolidityGenerateSettingsDeps, AdmitSolidityGenerateSettingsErrorReturn,
    AdmitSolidityGenerateSettingsParams, AdmitSolidityGenerateSettingsPayload,
    AdmitSolidityGenerateSettingsReturn, SolidityGenerateSettingsDocumentErrorReturn,
};

pub type AdmitSolidityGenerateSettingsHandle = for<'d> fn(
    &AdmitSolidityGenerateSettingsDeps<'d>,
    AdmitSolidityGenerateSettingsParams,
    AdmitSolidityGenerateSettingsPayload,
) -> AdmitSolidityGenerateSettingsReturn;

pub struct AdmitGenerateSelectionDeps {
    pub lookup_declaration: LookupGenerateDeclarationFn,
    pub admit_solidity_settings: AdmitSolidityGenerateSettingsHandle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitGenerateSelectionParams {
    pub identifier: GenerateConcreteKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitGenerateSelectionSuccessReturn {
    pub selection: GenerateConcrete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitGenerateSelectionSolidityErrorReturn {
    Document(SolidityGenerateSettingsDocumentErrorReturn),
    Settings(AdmitSolidityGenerateSettingsErrorReturn),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitGenerateSelectionErrorReturn {
    Solidity(AdmitGenerateSelectionSolidityErrorReturn),
}

pub type AdmitGenerateSelectionReturn =
    Result<AdmitGenerateSelectionSuccessReturn, AdmitGenerateSelectionErrorReturn>;

pub type AdmitGenerateSelectionFn = fn(
    &AdmitGenerateSelectionDeps,
    AdmitGenerateSelectionParams,
    serde_json::Value,
) -> AdmitGenerateSelectionReturn;
