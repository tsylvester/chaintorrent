use crate::factory::provides::{
    CHAIN_INTERFACE_VERSION, ChainDeclaration, ChainEndpoint, ChainFormsIdentifier,
    ChainMemberIdentifier, ChainSettingBounds, ChainSettingDisposition, ChainSettingStatement,
    FeeEntryDeclaration,
};
use pairing::PrecompileEncoding;
use url::ParseError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseChainSettings {
    pub endpoint: ChainEndpoint,
    pub settlement_endpoint: ChainEndpoint,
    pub chain_identifier: [u8; 32],
    pub registry: [u8; 20],
    pub entitlement_contract: [u8; 20],
    pub gas_price_oracle: [u8; 20],
    pub anchor_state_registry: [u8; 20],
    pub batch_page_limit: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseChainSettingsDocument {
    pub endpoint: Option<String>,
    pub settlement_endpoint: Option<String>,
    pub chain_identifier: Option<String>,
    pub registry: Option<String>,
    pub entitlement_contract: Option<String>,
    pub gas_price_oracle: Option<String>,
    pub anchor_state_registry: Option<String>,
    pub batch_page_limit: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BaseChainSettingsDocumentErrorReturn {
    NotATable { path: String },
    UnknownKey { path: String, key: String },
    NotText { path: String },
}

pub type BaseChainSettingsDocumentTryFromReturn =
    Result<BaseChainSettingsDocument, BaseChainSettingsDocumentErrorReturn>;

pub struct AdmitBaseChainSettingsDeps<'a> {
    pub declaration: &'a ChainDeclaration,
}

pub struct AdmitBaseChainSettingsParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitBaseChainSettingsPayload {
    pub document: BaseChainSettingsDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitBaseChainSettingsSuccessReturn {
    pub settings: BaseChainSettings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitBaseChainSettingsErrorReturn {
    UndeclaredSetting { setting: String },
    FixedSetting { setting: String, supplied: String },
    OutOfBounds { setting: String, supplied: String },
    Endpoint { setting: String, error: ParseError },
    ChainIdentifier { setting: String, text: String },
    Address { setting: String, text: String },
    BatchPageLimit { setting: String, text: String },
}

pub type AdmitBaseChainSettingsReturn =
    Result<AdmitBaseChainSettingsSuccessReturn, AdmitBaseChainSettingsErrorReturn>;

pub type AdmitBaseChainSettingsFn<'a> = fn(
    &AdmitBaseChainSettingsDeps<'a>,
    AdmitBaseChainSettingsParams,
    AdmitBaseChainSettingsPayload,
) -> AdmitBaseChainSettingsReturn;

pub const BASE_CHAIN_DECLARATION: ChainDeclaration = ChainDeclaration {
    identifier: ChainMemberIdentifier::Base,
    name: "Base",
    adapter_version: 1,
    interface_version: CHAIN_INTERFACE_VERSION,
    forms_identifier: ChainFormsIdentifier::EvmV1,
    precompile_encodings: &[
        PrecompileEncoding::Eip196Eip197,
        PrecompileEncoding::Eip2537,
    ],
    fee_entries: &[
        FeeEntryDeclaration {
            name: "execution_gas",
            unit: "gas",
        },
        FeeEntryDeclaration {
            name: "l1_data_fee",
            unit: "wei",
        },
    ],
    settings: &[
        ChainSettingStatement {
            name: "endpoint",
            disposition: ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::TextPrefixedBy {
                    prefixes: &["http://", "https://"],
                },
                default: "https://mainnet.base.org",
            },
        },
        ChainSettingStatement {
            name: "settlement_endpoint",
            disposition: ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::TextPrefixedBy {
                    prefixes: &["http://", "https://"],
                },
                default: "https://ethereum-rpc.publicnode.com",
            },
        },
        ChainSettingStatement {
            name: "chain_identifier",
            disposition: ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::Unbounded,
                default: "8453",
            },
        },
        ChainSettingStatement {
            name: "registry",
            disposition: ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::Unbounded,
                default: "0xdddddddddddddddddddddddddddddddddddddddd",
            },
        },
        ChainSettingStatement {
            name: "entitlement_contract",
            disposition: ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::Unbounded,
                default: "0x0000000000000000000000000000000000000000",
            },
        },
        ChainSettingStatement {
            name: "gas_price_oracle",
            disposition: ChainSettingDisposition::Fixed {
                value: "0x420000000000000000000000000000000000000f",
            },
        },
        ChainSettingStatement {
            name: "anchor_state_registry",
            disposition: ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::Unbounded,
                default: "0x909f6cf47ed12f010a796527f562bfc26c7f4e72",
            },
        },
        ChainSettingStatement {
            name: "batch_page_limit",
            disposition: ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::Unbounded,
                default: "64",
            },
        },
    ],
};
