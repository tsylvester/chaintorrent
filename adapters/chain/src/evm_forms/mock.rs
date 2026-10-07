#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    EvmChainIdentifierForm, EvmChainIdentifierFormConstructorParams, EvmEntitlementForm,
    EvmEntitlementFormConstructorParams, EvmIdentityForm, EvmIdentityFormConstructorParams,
    EvmIntervalForm, EvmIntervalFormConstructorParams,
};

#[derive(Default)]
pub struct EvmIdentityFormConstructorParamsOverrides {
    pub bytes: Option<[u8; 20]>,
}

pub fn build_evm_identity_form_constructor_params(
    overrides: EvmIdentityFormConstructorParamsOverrides,
) -> EvmIdentityFormConstructorParams {
    EvmIdentityFormConstructorParams {
        bytes: overrides.bytes.unwrap_or([0x0a; 20]),
    }
}

pub fn build_evm_identity_form(
    overrides: EvmIdentityFormConstructorParamsOverrides,
) -> EvmIdentityForm {
    let Ok(form) = EvmIdentityForm::try_new(build_evm_identity_form_constructor_params(overrides));
    form
}

#[derive(Default)]
pub struct EvmEntitlementFormConstructorParamsOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub fn build_evm_entitlement_form_constructor_params(
    overrides: EvmEntitlementFormConstructorParamsOverrides,
) -> EvmEntitlementFormConstructorParams {
    EvmEntitlementFormConstructorParams {
        bytes: overrides.bytes.unwrap_or([0x0b; 32]),
    }
}

pub fn build_evm_entitlement_form(
    overrides: EvmEntitlementFormConstructorParamsOverrides,
) -> EvmEntitlementForm {
    let Ok(form) =
        EvmEntitlementForm::try_new(build_evm_entitlement_form_constructor_params(overrides));
    form
}

#[derive(Default)]
pub struct EvmIntervalFormConstructorParamsOverrides {
    pub value: Option<u64>,
}

pub fn build_evm_interval_form_constructor_params(
    overrides: EvmIntervalFormConstructorParamsOverrides,
) -> EvmIntervalFormConstructorParams {
    EvmIntervalFormConstructorParams {
        value: overrides.value.unwrap_or(1),
    }
}

pub fn build_evm_interval_form(
    overrides: EvmIntervalFormConstructorParamsOverrides,
) -> EvmIntervalForm {
    let Ok(form) = EvmIntervalForm::try_new(build_evm_interval_form_constructor_params(overrides));
    form
}

#[derive(Default)]
pub struct EvmChainIdentifierFormConstructorParamsOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub fn build_evm_chain_identifier_form_constructor_params(
    overrides: EvmChainIdentifierFormConstructorParamsOverrides,
) -> EvmChainIdentifierFormConstructorParams {
    EvmChainIdentifierFormConstructorParams {
        bytes: overrides.bytes.unwrap_or([0x0c; 32]),
    }
}

pub fn build_evm_chain_identifier_form(
    overrides: EvmChainIdentifierFormConstructorParamsOverrides,
) -> EvmChainIdentifierForm {
    let Ok(form) = EvmChainIdentifierForm::try_new(
        build_evm_chain_identifier_form_constructor_params(overrides),
    );
    form
}
