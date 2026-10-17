#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    EvmChainIdentifierForm, EvmChainIdentifierFormConstructorParams, EvmEntitlementForm,
    EvmEntitlementFormConstructorParams, EvmForms, EvmFormsConstructorParams, EvmIdentityForm,
    EvmIdentityFormConstructorParams, EvmIntervalForm, EvmIntervalFormConstructorParams,
};

#[derive(Default)]
pub struct EvmFormsConstructorParamsOverrides;

pub fn build_evm_forms_constructor_params(
    _overrides: EvmFormsConstructorParamsOverrides,
) -> EvmFormsConstructorParams {
    EvmFormsConstructorParams
}

pub fn build_evm_forms(overrides: EvmFormsConstructorParamsOverrides) -> EvmForms {
    let Ok(forms) = EvmForms::try_new(build_evm_forms_constructor_params(overrides));
    forms
}

#[derive(Default)]
pub struct EvmIdentityFormConstructorParamsOverrides {
    pub bytes: Option<[u8; 20]>,
}

pub fn build_evm_identity_form_constructor_params(
    overrides: EvmIdentityFormConstructorParamsOverrides,
) -> EvmIdentityFormConstructorParams {
    EvmIdentityFormConstructorParams {
        bytes: overrides.bytes.unwrap_or([0xA1; 20]),
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
        bytes: overrides.bytes.unwrap_or([0xA2; 32]),
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
        value: overrides.value.unwrap_or(0xA3),
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
        bytes: overrides.bytes.unwrap_or([0xA4; 32]),
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
