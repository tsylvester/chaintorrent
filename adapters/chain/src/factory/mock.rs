#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ChainFormsIdentifier, IChainForms,
};
use encoding::MockICanonicalField;

#[derive(Default)]
pub struct ChainFormsDeclarationOverrides {
    pub identifier: Option<ChainFormsIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_chain_forms_declaration(
    overrides: ChainFormsDeclarationOverrides,
) -> ChainFormsDeclaration {
    ChainFormsDeclaration {
        identifier: overrides.identifier.unwrap_or(ChainFormsIdentifier::EvmV1),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(CHAIN_FORMS_INTERFACE_VERSION),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MockIChainForms;

impl IChainForms for MockIChainForms {
    const DECLARATION: ChainFormsDeclaration = ChainFormsDeclaration {
        identifier: ChainFormsIdentifier::EvmV1,
        adapter_version: 1,
        interface_version: CHAIN_FORMS_INTERFACE_VERSION,
    };

    type Identity = MockICanonicalField;
    type Entitlement = MockICanonicalField;
    type Interval = MockICanonicalField;
    type ChainIdentifier = MockICanonicalField;
}
