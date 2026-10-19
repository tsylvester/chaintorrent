use core::convert::Infallible;

use crate::config::provides::HarnessStatements;

pub struct GenerateConsumer<'a> {
    pub(super) statements: &'a HarnessStatements,
}

pub struct GenerateConsumerConstructorParams<'a> {
    pub statements: &'a HarnessStatements,
}

pub type GenerateConsumerTryNewReturn<'a> = Result<GenerateConsumer<'a>, Infallible>;
