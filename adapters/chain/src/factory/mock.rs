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
