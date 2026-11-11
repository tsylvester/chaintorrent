#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{SignatureSchemeIdentifier, SignatureSchemeIdentifierConstructorParams};

#[derive(Default)]
pub struct SignatureSchemeIdentifierConstructorParamsOverrides {
    pub identifier: Option<u8>,
}

pub fn build_signature_scheme_identifier_constructor_params(
    overrides: SignatureSchemeIdentifierConstructorParamsOverrides,
) -> SignatureSchemeIdentifierConstructorParams {
    SignatureSchemeIdentifierConstructorParams {
        identifier: overrides.identifier.unwrap_or(1),
    }
}

pub fn build_signature_scheme_identifier(
    overrides: SignatureSchemeIdentifierConstructorParamsOverrides,
) -> SignatureSchemeIdentifier {
    SignatureSchemeIdentifier::try_new(build_signature_scheme_identifier_constructor_params(
        overrides,
    ))
    .expect("built signature scheme identifier constructor params are admitted")
}
