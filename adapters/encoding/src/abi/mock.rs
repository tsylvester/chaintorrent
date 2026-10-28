#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{AbiEncoding, AbiEncodingConstructorParams};

#[derive(Default)]
pub struct AbiEncodingConstructorParamsOverrides;

pub fn build_abi_encoding_constructor_params(
    _overrides: AbiEncodingConstructorParamsOverrides,
) -> AbiEncodingConstructorParams {
    AbiEncodingConstructorParams
}

pub fn build_abi_encoding(overrides: AbiEncodingConstructorParamsOverrides) -> AbiEncoding {
    let Ok(adapter) = AbiEncoding::try_new(build_abi_encoding_constructor_params(overrides));
    adapter
}
