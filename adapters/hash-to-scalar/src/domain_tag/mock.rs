#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{DomainTag, DomainTagConstructorParams};

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
            .unwrap_or_else(|| b"ChainTorrent example tag".to_vec()),
    }
}

pub fn build_domain_tag(overrides: DomainTagConstructorParamsOverrides) -> DomainTag {
    DomainTag::try_new(build_domain_tag_constructor_params(overrides))
        .expect("built domain tag constructor params are admitted")
}
