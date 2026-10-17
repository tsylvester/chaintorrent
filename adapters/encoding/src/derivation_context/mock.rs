#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::interface::{
    DerivationContextDescription, DerivationContextDescriptionConstructorParams,
};

#[derive(Default)]
pub struct DerivationContextDescriptionConstructorParamsOverrides;

pub fn build_derivation_context_description_constructor_params(
    _overrides: DerivationContextDescriptionConstructorParamsOverrides,
) -> DerivationContextDescriptionConstructorParams {
    DerivationContextDescriptionConstructorParams
}

pub fn build_derivation_context_description(
    overrides: DerivationContextDescriptionConstructorParamsOverrides,
) -> DerivationContextDescription {
    let Ok(description) = DerivationContextDescription::try_new(
        build_derivation_context_description_constructor_params(overrides),
    );
    description
}
