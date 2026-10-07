#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::ConstantsParams;
use proof::DELIVERY_STATEMENT_VERSION_ONE;

#[derive(Default)]
pub struct ConstantsParamsOverrides {
    pub statement_version: Option<u16>,
}

pub fn build_constants_params(overrides: ConstantsParamsOverrides) -> ConstantsParams {
    ConstantsParams {
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}
