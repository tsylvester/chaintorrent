#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{Secret, SecretConstructorParams};
use zeroize::Zeroize;

#[derive(Default)]
pub struct SecretConstructorParamsOverrides;

pub fn build_secret_constructor_params<T: Zeroize>(
    value: T,
    _overrides: SecretConstructorParamsOverrides,
) -> SecretConstructorParams<T> {
    SecretConstructorParams { value }
}

pub fn build_secret<T: Zeroize>(
    value: T,
    overrides: SecretConstructorParamsOverrides,
) -> Secret<T> {
    let Ok(secret) = Secret::try_new(build_secret_constructor_params(value, overrides));
    secret
}
