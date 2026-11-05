#![allow(dead_code, clippy::too_many_arguments)]

use domain::{
    ParameterSetIdentifier, SuiteIdentifier, build_parameter_set_identifier, build_suite_identifier,
};
use encoding::CanonicalFieldValue;
use hashing::{Root, build_root};
use serde_json::{Map, Value};

use super::interface::CanonicalDeliveryContext;

#[derive(Default)]
pub struct CanonicalDeliveryContextOverrides {
    pub suite_identifier: Option<SuiteIdentifier>,
    pub chain: Option<CanonicalFieldValue>,
    pub entitlement_contract: Option<CanonicalFieldValue>,
    pub asset_root: Option<Root>,
    pub parameter_set_identifier: Option<ParameterSetIdentifier>,
    pub source_entitlement: Option<CanonicalFieldValue>,
    pub target_entitlement: Option<CanonicalFieldValue>,
    pub old_interval: Option<CanonicalFieldValue>,
    pub new_interval: Option<CanonicalFieldValue>,
    pub seller: Option<CanonicalFieldValue>,
    pub buyer: Option<CanonicalFieldValue>,
    pub expiry: Option<u64>,
}

pub fn build_canonical_delivery_context(
    overrides: CanonicalDeliveryContextOverrides,
) -> CanonicalDeliveryContext {
    CanonicalDeliveryContext {
        suite_identifier: overrides
            .suite_identifier
            .unwrap_or_else(|| build_suite_identifier(Default::default())),
        chain: overrides
            .chain
            .unwrap_or(CanonicalFieldValue::Unsigned256([0x21; 32])),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x31; 20])),
        asset_root: overrides
            .asset_root
            .unwrap_or_else(|| build_root(Default::default())),
        parameter_set_identifier: overrides
            .parameter_set_identifier
            .unwrap_or_else(|| build_parameter_set_identifier(Default::default())),
        source_entitlement: overrides
            .source_entitlement
            .unwrap_or(CanonicalFieldValue::Unsigned256([0x41; 32])),
        target_entitlement: overrides
            .target_entitlement
            .unwrap_or(CanonicalFieldValue::Unsigned256([0x42; 32])),
        old_interval: overrides
            .old_interval
            .unwrap_or(CanonicalFieldValue::Unsigned64(1)),
        new_interval: overrides
            .new_interval
            .unwrap_or(CanonicalFieldValue::Unsigned64(2)),
        seller: overrides
            .seller
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x61; 20])),
        buyer: overrides
            .buyer
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x62; 20])),
        expiry: overrides.expiry.unwrap_or(1_800_000_000),
    }
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

fn canonical_delivery_context_fields(context: &CanonicalDeliveryContext) -> Map<String, Value> {
    let mut fields = Map::new();
    let mut suite_identifier = Map::new();
    suite_identifier.insert(
        "identifier".to_string(),
        byte_array(context.suite_identifier.identifier()),
    );
    suite_identifier.insert(
        "version".to_string(),
        Value::from(context.suite_identifier.version()),
    );
    fields.insert(
        "suite_identifier".to_string(),
        Value::Object(suite_identifier),
    );
    fields.insert("chain".to_string(), canonical_field_value(&context.chain));
    fields.insert(
        "entitlement_contract".to_string(),
        canonical_field_value(&context.entitlement_contract),
    );
    fields.insert(
        "asset_root".to_string(),
        byte_array(context.asset_root.as_bytes()),
    );
    fields.insert(
        "parameter_set_identifier".to_string(),
        byte_array(context.parameter_set_identifier.as_bytes()),
    );
    fields.insert(
        "source_entitlement".to_string(),
        canonical_field_value(&context.source_entitlement),
    );
    fields.insert(
        "target_entitlement".to_string(),
        canonical_field_value(&context.target_entitlement),
    );
    fields.insert(
        "old_interval".to_string(),
        canonical_field_value(&context.old_interval),
    );
    fields.insert(
        "new_interval".to_string(),
        canonical_field_value(&context.new_interval),
    );
    fields.insert("seller".to_string(), canonical_field_value(&context.seller));
    fields.insert("buyer".to_string(), canonical_field_value(&context.buyer));
    fields.insert("expiry".to_string(), Value::from(context.expiry));
    fields
}

#[derive(Default)]
pub struct CanonicalDeliveryContextCorruptions {
    pub suite_identifier: Option<Value>,
    pub chain: Option<Value>,
    pub entitlement_contract: Option<Value>,
    pub asset_root: Option<Value>,
    pub parameter_set_identifier: Option<Value>,
    pub source_entitlement: Option<Value>,
    pub target_entitlement: Option<Value>,
    pub old_interval: Option<Value>,
    pub new_interval: Option<Value>,
    pub seller: Option<Value>,
    pub buyer: Option<Value>,
    pub expiry: Option<Value>,
}

pub fn invalidate_canonical_delivery_context(
    corruptions: CanonicalDeliveryContextCorruptions,
) -> Value {
    let mut fields =
        canonical_delivery_context_fields(&build_canonical_delivery_context(Default::default()));
    if let Some(suite_identifier) = corruptions.suite_identifier {
        fields.insert("suite_identifier".to_string(), suite_identifier);
    }
    if let Some(chain) = corruptions.chain {
        fields.insert("chain".to_string(), chain);
    }
    if let Some(entitlement_contract) = corruptions.entitlement_contract {
        fields.insert("entitlement_contract".to_string(), entitlement_contract);
    }
    if let Some(asset_root) = corruptions.asset_root {
        fields.insert("asset_root".to_string(), asset_root);
    }
    if let Some(parameter_set_identifier) = corruptions.parameter_set_identifier {
        fields.insert(
            "parameter_set_identifier".to_string(),
            parameter_set_identifier,
        );
    }
    if let Some(source_entitlement) = corruptions.source_entitlement {
        fields.insert("source_entitlement".to_string(), source_entitlement);
    }
    if let Some(target_entitlement) = corruptions.target_entitlement {
        fields.insert("target_entitlement".to_string(), target_entitlement);
    }
    if let Some(old_interval) = corruptions.old_interval {
        fields.insert("old_interval".to_string(), old_interval);
    }
    if let Some(new_interval) = corruptions.new_interval {
        fields.insert("new_interval".to_string(), new_interval);
    }
    if let Some(seller) = corruptions.seller {
        fields.insert("seller".to_string(), seller);
    }
    if let Some(buyer) = corruptions.buyer {
        fields.insert("buyer".to_string(), buyer);
    }
    if let Some(expiry) = corruptions.expiry {
        fields.insert("expiry".to_string(), expiry);
    }
    Value::Object(fields)
}
