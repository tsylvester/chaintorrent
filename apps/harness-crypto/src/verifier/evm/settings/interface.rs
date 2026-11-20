use core::convert::Infallible;

use chain::{ChainFormsIdentifier, ChainMemberIdentifier};
use encoding::EncodingIdentifier;
use pairing::VerifierGroupArithmetic;
use proof::DELIVERY_STATEMENT_VERSION_ONE;

use super::super::super::factory::provides::{
    VERIFIER_INTERFACE_VERSION, VerifierConcreteKind, VerifierDeclaration,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmVerifierSettings;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmVerifierSettingsDocument;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvmVerifierSettingsDocumentErrorReturn {
    NotATable { path: String },
    UnknownKey { path: String, key: String },
}

pub type EvmVerifierSettingsDocumentTryFromReturn =
    Result<EvmVerifierSettingsDocument, EvmVerifierSettingsDocumentErrorReturn>;

#[derive(Clone, Copy)]
pub struct AdmitEvmVerifierSettingsDeps<'a> {
    pub declaration: &'a VerifierDeclaration,
}

pub struct AdmitEvmVerifierSettingsParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitEvmVerifierSettingsPayload {
    pub document: EvmVerifierSettingsDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitEvmVerifierSettingsSuccessReturn {
    pub settings: EvmVerifierSettings,
}

pub type AdmitEvmVerifierSettingsReturn = Result<AdmitEvmVerifierSettingsSuccessReturn, Infallible>;

pub type AdmitEvmVerifierSettingsFn = for<'a> fn(
    &AdmitEvmVerifierSettingsDeps<'a>,
    AdmitEvmVerifierSettingsParams,
    AdmitEvmVerifierSettingsPayload,
) -> AdmitEvmVerifierSettingsReturn;

pub const EVM_VERIFIER_DECLARATION: VerifierDeclaration = VerifierDeclaration {
    identifier: VerifierConcreteKind::Evm,
    name: "evm",
    verifier_forms: &[
        VerifierGroupArithmetic::FirstGroupOnly,
        VerifierGroupArithmetic::BothGroups,
    ],
    statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
    adapter_version: 1,
    interface_version: VERIFIER_INTERFACE_VERSION,
    chain_forms_identifier: ChainFormsIdentifier::EvmV1,
    chain_member: ChainMemberIdentifier::Base,
    encoding_identifier: EncodingIdentifier::EthereumAbiV1,
    contract_suite: "evm",
    settings: &[],
};
