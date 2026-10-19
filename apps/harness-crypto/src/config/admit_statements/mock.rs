#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use domain::build_asset_identity_hash;
use encoding::CanonicalFieldValue;
use proof::{MintPurpose, TransferPurpose};
use serde_json::{Map, Value};

use super::super::layers::provides::{NetworkDocument, StatementsDocument};
use super::interface::{
    AdmitStatementsDeps, AdmitStatementsFn, AdmitStatementsParams, AdmitStatementsPayload,
    AdmitStatementsReturn, AdmitStatementsSuccessReturn, HarnessStatements, NetworkProfile,
};
use crate::generate::provides::{
    CanonicalDeliveryContext, build_canonical_delivery_context,
    invalidate_canonical_delivery_context,
};

#[derive(Default)]
pub struct AdmitStatementsDepsOverrides;

pub fn build_admit_statements_deps(
    _overrides: AdmitStatementsDepsOverrides,
) -> AdmitStatementsDeps {
    AdmitStatementsDeps
}

#[derive(Default)]
pub struct AdmitStatementsParamsOverrides;

pub fn build_admit_statements_params(
    _overrides: AdmitStatementsParamsOverrides,
) -> AdmitStatementsParams {
    AdmitStatementsParams
}

#[derive(Default)]
pub struct AdmitStatementsPayloadOverrides;

pub fn build_admit_statements_payload<'a>(
    network: &'a NetworkDocument,
    statements: &'a StatementsDocument,
    _overrides: AdmitStatementsPayloadOverrides,
) -> AdmitStatementsPayload<'a> {
    AdmitStatementsPayload {
        network,
        statements,
    }
}

#[derive(Default)]
pub struct NetworkProfileOverrides {
    pub chain: Option<CanonicalFieldValue>,
    pub entitlement_contract: Option<CanonicalFieldValue>,
}

pub fn build_network_profile(overrides: NetworkProfileOverrides) -> NetworkProfile {
    NetworkProfile {
        chain: overrides.chain.unwrap_or_else(|| {
            let mut bytes = [0x00; 32];
            bytes[24..].copy_from_slice(&84532u64.to_be_bytes());
            CanonicalFieldValue::Unsigned256(bytes)
        }),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x00; 20])),
    }
}

#[derive(Default)]
pub struct NetworkProfileCorruptions {
    pub chain: Option<Value>,
    pub entitlement_contract: Option<Value>,
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn canonical_field_value(value: &CanonicalFieldValue) -> Value {
    let mut variant = Map::new();
    match value {
        CanonicalFieldValue::FixedBytes32(bytes) => {
            variant.insert("FixedBytes32".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::Unsigned16(value) => {
            variant.insert("Unsigned16".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Unsigned32(value) => {
            variant.insert("Unsigned32".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Unsigned64(value) => {
            variant.insert("Unsigned64".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Text(text) => {
            variant.insert("Text".to_string(), Value::from(text.clone()));
        }
        CanonicalFieldValue::Bytes(bytes) => {
            variant.insert("Bytes".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::FixedBytes20(bytes) => {
            variant.insert("FixedBytes20".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::Unsigned256(bytes) => {
            variant.insert("Unsigned256".to_string(), byte_array(bytes));
        }
    }
    Value::Object(variant)
}

fn network_profile_fields(profile: &NetworkProfile) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("chain".to_string(), canonical_field_value(&profile.chain));
    fields.insert(
        "entitlement_contract".to_string(),
        canonical_field_value(&profile.entitlement_contract),
    );
    fields
}

pub fn invalidate_network_profile(corruptions: NetworkProfileCorruptions) -> Value {
    let mut fields = network_profile_fields(&build_network_profile(Default::default()));
    if let Some(chain) = corruptions.chain {
        fields.insert("chain".to_string(), chain);
    }
    if let Some(entitlement_contract) = corruptions.entitlement_contract {
        fields.insert("entitlement_contract".to_string(), entitlement_contract);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct HarnessStatementsOverrides {
    pub mint: Option<CanonicalDeliveryContext>,
    pub transfer: Option<CanonicalDeliveryContext>,
    pub mint_purpose: Option<MintPurpose>,
    pub transfer_purpose: Option<TransferPurpose>,
    pub identities: Option<Vec<Vec<u8>>>,
    pub proof_identity: Option<Vec<u8>>,
}

pub fn build_harness_statements(overrides: HarnessStatementsOverrides) -> HarnessStatements {
    HarnessStatements {
        mint: overrides
            .mint
            .unwrap_or_else(|| build_canonical_delivery_context(Default::default())),
        transfer: overrides
            .transfer
            .unwrap_or_else(|| build_canonical_delivery_context(Default::default())),
        mint_purpose: overrides.mint_purpose.unwrap_or(MintPurpose::Mint),
        transfer_purpose: overrides
            .transfer_purpose
            .unwrap_or(TransferPurpose::Transfer),
        identities: overrides.identities.unwrap_or_else(|| {
            vec![
                build_asset_identity_hash(Default::default())
                    .as_bytes()
                    .to_vec(),
            ]
        }),
        proof_identity: overrides.proof_identity.unwrap_or_else(|| {
            build_asset_identity_hash(Default::default())
                .as_bytes()
                .to_vec()
        }),
    }
}

#[derive(Default)]
pub struct HarnessStatementsCorruptions {
    pub mint: Option<Value>,
    pub transfer: Option<Value>,
    pub mint_purpose: Option<Value>,
    pub transfer_purpose: Option<Value>,
    pub identities: Option<Value>,
    pub proof_identity: Option<Value>,
}

fn harness_statements_fields(statements: &HarnessStatements) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "mint".to_string(),
        invalidate_canonical_delivery_context(Default::default()),
    );
    fields.insert(
        "transfer".to_string(),
        invalidate_canonical_delivery_context(Default::default()),
    );
    fields.insert(
        "mint_purpose".to_string(),
        Value::from(statements.mint_purpose.code()),
    );
    fields.insert(
        "transfer_purpose".to_string(),
        Value::from(statements.transfer_purpose.code()),
    );
    fields.insert(
        "identities".to_string(),
        Value::Array(
            statements
                .identities
                .iter()
                .map(|identity| byte_array(identity))
                .collect(),
        ),
    );
    fields.insert(
        "proof_identity".to_string(),
        byte_array(&statements.proof_identity),
    );
    fields
}

pub fn invalidate_harness_statements(corruptions: HarnessStatementsCorruptions) -> Value {
    let mut fields = harness_statements_fields(&build_harness_statements(Default::default()));
    if let Some(mint) = corruptions.mint {
        fields.insert("mint".to_string(), mint);
    }
    if let Some(transfer) = corruptions.transfer {
        fields.insert("transfer".to_string(), transfer);
    }
    if let Some(mint_purpose) = corruptions.mint_purpose {
        fields.insert("mint_purpose".to_string(), mint_purpose);
    }
    if let Some(transfer_purpose) = corruptions.transfer_purpose {
        fields.insert("transfer_purpose".to_string(), transfer_purpose);
    }
    if let Some(identities) = corruptions.identities {
        fields.insert("identities".to_string(), identities);
    }
    if let Some(proof_identity) = corruptions.proof_identity {
        fields.insert("proof_identity".to_string(), proof_identity);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct AdmitStatementsSuccessReturnOverrides {
    pub network: Option<NetworkProfile>,
    pub statements: Option<HarnessStatements>,
}

pub fn build_admit_statements_success_return(
    overrides: AdmitStatementsSuccessReturnOverrides,
) -> AdmitStatementsSuccessReturn {
    AdmitStatementsSuccessReturn {
        network: overrides
            .network
            .unwrap_or_else(|| build_network_profile(Default::default())),
        statements: overrides
            .statements
            .unwrap_or_else(|| build_harness_statements(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmitStatementsSuccessReturnCorruptions {
    pub network: Option<Value>,
    pub statements: Option<Value>,
}

pub fn invalidate_admit_statements_success_return(
    corruptions: AdmitStatementsSuccessReturnCorruptions,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "network".to_string(),
        invalidate_network_profile(Default::default()),
    );
    fields.insert(
        "statements".to_string(),
        invalidate_harness_statements(Default::default()),
    );
    if let Some(network) = corruptions.network {
        fields.insert("network".to_string(), network);
    }
    if let Some(statements) = corruptions.statements {
        fields.insert("statements".to_string(), statements);
    }
    Value::Object(fields)
}

pub fn mock_admit_statements(
    _deps: &AdmitStatementsDeps,
    _params: AdmitStatementsParams,
    _payload: AdmitStatementsPayload<'_>,
) -> AdmitStatementsReturn {
    let _: AdmitStatementsFn = mock_admit_statements;
    Ok(build_admit_statements_success_return(Default::default()))
}
