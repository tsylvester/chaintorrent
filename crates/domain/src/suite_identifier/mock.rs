#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    SUITE_IDENTIFIER_LENGTH, SuiteIdentifier, SuiteIdentifierConstructorParams,
};

#[derive(Default)]
pub struct SuiteIdentifierConstructorParamsOverrides {
    pub identifier: Option<[u8; SUITE_IDENTIFIER_LENGTH]>,
    pub version: Option<u16>,
}

pub fn build_suite_identifier_constructor_params(
    overrides: SuiteIdentifierConstructorParamsOverrides,
) -> SuiteIdentifierConstructorParams {
    SuiteIdentifierConstructorParams {
        identifier: overrides
            .identifier
            .unwrap_or([0x22; SUITE_IDENTIFIER_LENGTH]),
        version: overrides.version.unwrap_or(1),
    }
}

pub fn build_suite_identifier(
    overrides: SuiteIdentifierConstructorParamsOverrides,
) -> SuiteIdentifier {
    SuiteIdentifier::try_new(build_suite_identifier_constructor_params(overrides))
        .expect("built suite identifier constructor params are admitted")
}
