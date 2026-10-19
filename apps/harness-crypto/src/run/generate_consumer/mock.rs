#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{GenerateConsumer, GenerateConsumerConstructorParams};
use crate::config::provides::HarnessStatements;

#[derive(Default)]
pub struct GenerateConsumerConstructorParamsOverrides;

pub fn build_generate_consumer_constructor_params<'a>(
    statements: &'a HarnessStatements,
    _overrides: GenerateConsumerConstructorParamsOverrides,
) -> GenerateConsumerConstructorParams<'a> {
    GenerateConsumerConstructorParams { statements }
}

pub fn build_generate_consumer<'a>(
    statements: &'a HarnessStatements,
    overrides: GenerateConsumerConstructorParamsOverrides,
) -> GenerateConsumer<'a> {
    let Ok(consumer) = GenerateConsumer::try_new(build_generate_consumer_constructor_params(
        statements, overrides,
    ));
    consumer
}
