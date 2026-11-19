#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::admit_base_chain_settings;
use super::interface::{
    AdmitBaseChainSettingsDeps, AdmitBaseChainSettingsFn, AdmitBaseChainSettingsParams,
    AdmitBaseChainSettingsPayload, AdmitBaseChainSettingsReturn,
    AdmitBaseChainSettingsSuccessReturn, BASE_CHAIN_DECLARATION, BaseChainSettings,
    BaseChainSettingsDocument,
};
use crate::factory::provides::{ChainDeclaration, ChainEndpoint};
use serde_json::{Map, Value};

#[derive(Default)]
pub struct BaseChainSettingsOverrides {
    pub endpoint: Option<ChainEndpoint>,
    pub settlement_endpoint: Option<ChainEndpoint>,
    pub chain_identifier: Option<[u8; 32]>,
    pub registry: Option<[u8; 20]>,
    pub entitlement_contract: Option<[u8; 20]>,
    pub gas_price_oracle: Option<[u8; 20]>,
    pub anchor_state_registry: Option<[u8; 20]>,
    pub batch_page_limit: Option<u64>,
}

pub fn build_base_chain_settings(overrides: BaseChainSettingsOverrides) -> BaseChainSettings {
    let admitted = admit_base_chain_settings(
        &build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default()),
        build_admit_base_chain_settings_params(Default::default()),
        build_admit_base_chain_settings_payload(Default::default()),
    )
    .expect("the defaults BASE_CHAIN_DECLARATION states admit")
    .settings;
    BaseChainSettings {
        endpoint: overrides.endpoint.unwrap_or(admitted.endpoint),
        settlement_endpoint: overrides
            .settlement_endpoint
            .unwrap_or(admitted.settlement_endpoint),
        chain_identifier: overrides
            .chain_identifier
            .unwrap_or(admitted.chain_identifier),
        registry: overrides.registry.unwrap_or(admitted.registry),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or(admitted.entitlement_contract),
        gas_price_oracle: overrides
            .gas_price_oracle
            .unwrap_or(admitted.gas_price_oracle),
        anchor_state_registry: overrides
            .anchor_state_registry
            .unwrap_or(admitted.anchor_state_registry),
        batch_page_limit: overrides
            .batch_page_limit
            .unwrap_or(admitted.batch_page_limit),
    }
}

#[derive(Default)]
pub struct BaseChainSettingsDocumentOverrides {
    pub endpoint: Option<Option<String>>,
    pub settlement_endpoint: Option<Option<String>>,
    pub chain_identifier: Option<Option<String>>,
    pub registry: Option<Option<String>>,
    pub entitlement_contract: Option<Option<String>>,
    pub gas_price_oracle: Option<Option<String>>,
    pub anchor_state_registry: Option<Option<String>>,
    pub batch_page_limit: Option<Option<String>>,
}

pub fn build_base_chain_settings_document(
    overrides: BaseChainSettingsDocumentOverrides,
) -> BaseChainSettingsDocument {
    BaseChainSettingsDocument {
        endpoint: overrides.endpoint.unwrap_or_default(),
        settlement_endpoint: overrides.settlement_endpoint.unwrap_or_default(),
        chain_identifier: overrides.chain_identifier.unwrap_or_default(),
        registry: overrides.registry.unwrap_or_default(),
        entitlement_contract: overrides.entitlement_contract.unwrap_or_default(),
        gas_price_oracle: overrides.gas_price_oracle.unwrap_or_default(),
        anchor_state_registry: overrides.anchor_state_registry.unwrap_or_default(),
        batch_page_limit: overrides.batch_page_limit.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct BaseChainSettingsDocumentCorruptions {
    pub endpoint: Option<Value>,
    pub settlement_endpoint: Option<Value>,
    pub chain_identifier: Option<Value>,
    pub registry: Option<Value>,
    pub entitlement_contract: Option<Value>,
    pub gas_price_oracle: Option<Value>,
    pub anchor_state_registry: Option<Value>,
    pub batch_page_limit: Option<Value>,
}

pub fn invalidate_base_chain_settings_document(
    corruptions: BaseChainSettingsDocumentCorruptions,
) -> Value {
    let document = build_base_chain_settings_document(Default::default());
    let mut table = Map::new();
    if let Some(text) = document.endpoint {
        table.insert("endpoint".to_string(), Value::String(text));
    }
    if let Some(text) = document.settlement_endpoint {
        table.insert("settlement_endpoint".to_string(), Value::String(text));
    }
    if let Some(text) = document.chain_identifier {
        table.insert("chain_identifier".to_string(), Value::String(text));
    }
    if let Some(text) = document.registry {
        table.insert("registry".to_string(), Value::String(text));
    }
    if let Some(text) = document.entitlement_contract {
        table.insert("entitlement_contract".to_string(), Value::String(text));
    }
    if let Some(text) = document.gas_price_oracle {
        table.insert("gas_price_oracle".to_string(), Value::String(text));
    }
    if let Some(text) = document.anchor_state_registry {
        table.insert("anchor_state_registry".to_string(), Value::String(text));
    }
    if let Some(text) = document.batch_page_limit {
        table.insert("batch_page_limit".to_string(), Value::String(text));
    }
    if let Some(value) = corruptions.endpoint {
        table.insert("endpoint".to_string(), value);
    }
    if let Some(value) = corruptions.settlement_endpoint {
        table.insert("settlement_endpoint".to_string(), value);
    }
    if let Some(value) = corruptions.chain_identifier {
        table.insert("chain_identifier".to_string(), value);
    }
    if let Some(value) = corruptions.registry {
        table.insert("registry".to_string(), value);
    }
    if let Some(value) = corruptions.entitlement_contract {
        table.insert("entitlement_contract".to_string(), value);
    }
    if let Some(value) = corruptions.gas_price_oracle {
        table.insert("gas_price_oracle".to_string(), value);
    }
    if let Some(value) = corruptions.anchor_state_registry {
        table.insert("anchor_state_registry".to_string(), value);
    }
    if let Some(value) = corruptions.batch_page_limit {
        table.insert("batch_page_limit".to_string(), value);
    }
    Value::Object(table)
}

#[derive(Default)]
pub struct AdmitBaseChainSettingsPayloadOverrides {
    pub document: Option<BaseChainSettingsDocument>,
}

pub fn build_admit_base_chain_settings_payload(
    overrides: AdmitBaseChainSettingsPayloadOverrides,
) -> AdmitBaseChainSettingsPayload {
    AdmitBaseChainSettingsPayload {
        document: overrides
            .document
            .unwrap_or_else(|| build_base_chain_settings_document(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmitBaseChainSettingsParamsOverrides;

pub fn build_admit_base_chain_settings_params(
    _overrides: AdmitBaseChainSettingsParamsOverrides,
) -> AdmitBaseChainSettingsParams {
    AdmitBaseChainSettingsParams
}

#[derive(Default)]
pub struct AdmitBaseChainSettingsSuccessReturnOverrides {
    pub settings: Option<BaseChainSettings>,
}

pub fn build_admit_base_chain_settings_success_return(
    overrides: AdmitBaseChainSettingsSuccessReturnOverrides,
) -> AdmitBaseChainSettingsSuccessReturn {
    AdmitBaseChainSettingsSuccessReturn {
        settings: overrides
            .settings
            .unwrap_or_else(|| build_base_chain_settings(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmitBaseChainSettingsDepsOverrides;

pub fn build_admit_base_chain_settings_deps<'a>(
    declaration: &'a ChainDeclaration,
    _overrides: AdmitBaseChainSettingsDepsOverrides,
) -> AdmitBaseChainSettingsDeps<'a> {
    AdmitBaseChainSettingsDeps { declaration }
}

pub fn mock_admit_base_chain_settings<'a>(
    _deps: &AdmitBaseChainSettingsDeps<'a>,
    _params: AdmitBaseChainSettingsParams,
    _payload: AdmitBaseChainSettingsPayload,
) -> AdmitBaseChainSettingsReturn {
    Ok(build_admit_base_chain_settings_success_return(
        Default::default(),
    ))
}

const _: AdmitBaseChainSettingsFn<'_> = mock_admit_base_chain_settings;
