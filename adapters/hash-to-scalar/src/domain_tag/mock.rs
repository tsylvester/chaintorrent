#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{DomainTag, DomainTagConstructorParams};
use serde_json::{Map, Value};

const DEFAULT_DOMAIN_TAG_BYTES: &[u8] = b"ChainTorrent example tag";

#[derive(Default)]
pub struct DomainTagConstructorParamsOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_domain_tag_constructor_params(
    overrides: DomainTagConstructorParamsOverrides,
) -> DomainTagConstructorParams {
    DomainTagConstructorParams {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| DEFAULT_DOMAIN_TAG_BYTES.to_vec()),
    }
}

#[derive(Default)]
pub struct DomainTagConstructorParamsCorruptions {
    pub bytes: Option<Value>,
}

pub fn invalidate_domain_tag_constructor_params(
    corruptions: DomainTagConstructorParamsCorruptions,
) -> Value {
    let params = build_domain_tag_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("bytes".into(), Value::from(params.bytes));
    if let Some(bytes) = corruptions.bytes {
        fields.insert("bytes".into(), bytes);
    }
    Value::Object(fields)
}

pub fn build_domain_tag(overrides: DomainTagConstructorParamsOverrides) -> DomainTag {
    DomainTag::try_new(build_domain_tag_constructor_params(overrides))
        .expect("built domain tag constructor params are admitted")
}
