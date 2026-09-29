#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams,
};

#[derive(Default)]
pub struct ParameterSetIdentifierConstructorParamsOverrides {
    pub bytes: Option<[u8; PARAMETER_SET_IDENTIFIER_LENGTH]>,
}

pub fn build_parameter_set_identifier_constructor_params(
    overrides: ParameterSetIdentifierConstructorParamsOverrides,
) -> ParameterSetIdentifierConstructorParams {
    ParameterSetIdentifierConstructorParams {
        bytes: overrides
            .bytes
            .unwrap_or([0x33; PARAMETER_SET_IDENTIFIER_LENGTH]),
    }
}

pub fn build_parameter_set_identifier(
    overrides: ParameterSetIdentifierConstructorParamsOverrides,
) -> ParameterSetIdentifier {
    ParameterSetIdentifier::try_new(build_parameter_set_identifier_constructor_params(overrides))
        .expect("built parameter set identifier constructor params are admitted")
}
