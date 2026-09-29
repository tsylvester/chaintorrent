#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{Secret, SecretConstructorParams};
use zeroize::Zeroize;

#[derive(Default)]
pub struct SecretConstructorParamsOverrides<T> {
    pub value: Option<T>,
}

pub fn build_secret_constructor_params<T: Zeroize + Default>(
    overrides: SecretConstructorParamsOverrides<T>,
) -> SecretConstructorParams<T> {
    SecretConstructorParams {
        value: overrides.value.unwrap_or_default(),
    }
}

pub fn build_secret<T: Zeroize + Default>(
    overrides: SecretConstructorParamsOverrides<T>,
) -> Secret<T> {
    let Ok(secret) = Secret::try_new(build_secret_constructor_params(overrides));
    secret
}
