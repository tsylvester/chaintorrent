#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    EvmChainIdentifierForm, EvmChainIdentifierFormConstructorParams, EvmEntitlementForm,
    EvmEntitlementFormConstructorParams, EvmForms, EvmFormsConstructorParams, EvmFormsTryNewReturn,
    EvmIdentityForm, EvmIdentityFormConstructorParams, EvmIntervalForm,
    EvmIntervalFormConstructorParams, EvmKeyIdentifierForm, EvmKeyIdentifierFormConstructorParams,
    EvmSignedIntentForm, EvmSignedIntentFormConstructorParams,
};
use crate::factory::provides::{
    DeriveKeyIdentifierParams, IntentCall, IntentField, SignedIntentParams, SignedIntentPayload,
};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};

pub struct EvmFormsBuildingConsumer;

impl IEncodingConsumer for EvmFormsBuildingConsumer {
    type Output = EvmFormsTryNewReturn;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        EvmForms::try_new(EvmFormsConstructorParams {
            encoder: &payload.adapter,
        })
    }
}

pub fn build_evm_forms_over(selection: CreateEncodingParams) -> EvmForms {
    let output = create_encoding(
        &build_create_encoding_deps(EvmFormsBuildingConsumer),
        selection,
        build_create_encoding_payload(Default::default()),
    )
    .expect("the selection names a concrete that declares its identifier")
    .output;
    let Ok(forms) = output;
    forms
}

#[derive(Default)]
pub struct EvmFormsConstructorParamsOverrides;

pub fn build_evm_forms(_overrides: EvmFormsConstructorParamsOverrides) -> EvmForms {
    build_evm_forms_over(build_create_encoding_params(Default::default()))
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

#[derive(Default)]
pub struct EvmKeyIdentifierFormConstructorParamsOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub fn build_evm_key_identifier_form_constructor_params(
    overrides: EvmKeyIdentifierFormConstructorParamsOverrides,
) -> EvmKeyIdentifierFormConstructorParams {
    EvmKeyIdentifierFormConstructorParams {
        bytes: overrides.bytes.unwrap_or([0xA5; 32]),
    }
}

pub fn build_evm_key_identifier_form(
    overrides: EvmKeyIdentifierFormConstructorParamsOverrides,
) -> EvmKeyIdentifierForm {
    let Ok(form) =
        EvmKeyIdentifierForm::try_new(build_evm_key_identifier_form_constructor_params(overrides));
    form
}

#[derive(Default)]
pub struct EvmSignedIntentFormConstructorParamsOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub fn build_evm_signed_intent_form_constructor_params(
    overrides: EvmSignedIntentFormConstructorParamsOverrides,
) -> EvmSignedIntentFormConstructorParams {
    EvmSignedIntentFormConstructorParams {
        bytes: overrides.bytes.unwrap_or([0xA6; 32]),
    }
}

pub fn build_evm_signed_intent_form(
    overrides: EvmSignedIntentFormConstructorParamsOverrides,
) -> EvmSignedIntentForm {
    let Ok(form) =
        EvmSignedIntentForm::try_new(build_evm_signed_intent_form_constructor_params(overrides));
    form
}

#[derive(Default)]
pub struct IntentFieldOverrides {
    pub name: Option<String>,
    pub value: Option<CanonicalFieldValue>,
}

pub fn build_intent_field(overrides: IntentFieldOverrides) -> IntentField {
    IntentField {
        name: overrides.name.unwrap_or_else(|| "amount".to_string()),
        value: overrides
            .value
            .unwrap_or(CanonicalFieldValue::Unsigned256([0xB6; 32])),
    }
}

#[derive(Default)]
pub struct IntentCallOverrides {
    pub operation: Option<String>,
    pub fields: Option<Vec<IntentField>>,
}

pub fn build_intent_call(overrides: IntentCallOverrides) -> IntentCall {
    IntentCall {
        operation: overrides.operation.unwrap_or_else(|| "lock".to_string()),
        fields: overrides.fields.unwrap_or_else(|| {
            vec![
                build_intent_field(IntentFieldOverrides {
                    name: Some("fixedBytes32".to_string()),
                    value: Some(CanonicalFieldValue::FixedBytes32([0xC1; 32])),
                }),
                build_intent_field(IntentFieldOverrides {
                    name: Some("unsigned16".to_string()),
                    value: Some(CanonicalFieldValue::Unsigned16(0x0C02)),
                }),
                build_intent_field(IntentFieldOverrides {
                    name: Some("unsigned32".to_string()),
                    value: Some(CanonicalFieldValue::Unsigned32(0x0C03_0C03)),
                }),
                build_intent_field(IntentFieldOverrides {
                    name: Some("unsigned64".to_string()),
                    value: Some(CanonicalFieldValue::Unsigned64(0x0C04_0C04_0C04_0C04)),
                }),
                build_intent_field(IntentFieldOverrides {
                    name: Some("text".to_string()),
                    value: Some(CanonicalFieldValue::Text("c5-text".to_string())),
                }),
                build_intent_field(IntentFieldOverrides {
                    name: Some("bytes".to_string()),
                    value: Some(CanonicalFieldValue::Bytes(vec![0xC6, 0xC7, 0xC8])),
                }),
                build_intent_field(IntentFieldOverrides {
                    name: Some("fixedBytes20".to_string()),
                    value: Some(CanonicalFieldValue::FixedBytes20([0xC9; 20])),
                }),
                build_intent_field(IntentFieldOverrides {
                    name: Some("unsigned256".to_string()),
                    value: Some(CanonicalFieldValue::Unsigned256([0xCA; 32])),
                }),
            ]
        }),
    }
}

pub struct SignedIntentInputs {
    pub chain: EvmChainIdentifierForm,
    pub contract: EvmIdentityForm,
    pub identity: EvmIdentityForm,
    pub nonce: CanonicalFieldValue,
    pub expiry: CanonicalFieldValue,
    pub call: IntentCall,
}

#[derive(Default)]
pub struct SignedIntentInputsOverrides {
    pub chain: Option<EvmChainIdentifierForm>,
    pub contract: Option<EvmIdentityForm>,
    pub identity: Option<EvmIdentityForm>,
    pub nonce: Option<CanonicalFieldValue>,
    pub expiry: Option<CanonicalFieldValue>,
    pub call: Option<IntentCall>,
}

pub fn build_signed_intent_inputs(overrides: SignedIntentInputsOverrides) -> SignedIntentInputs {
    SignedIntentInputs {
        chain: overrides.chain.unwrap_or_else(|| {
            build_evm_chain_identifier_form(EvmChainIdentifierFormConstructorParamsOverrides {
                bytes: Some([0xB1; 32]),
            })
        }),
        contract: overrides.contract.unwrap_or_else(|| {
            build_evm_identity_form(EvmIdentityFormConstructorParamsOverrides {
                bytes: Some([0xB2; 20]),
            })
        }),
        identity: overrides.identity.unwrap_or_else(|| {
            build_evm_identity_form(EvmIdentityFormConstructorParamsOverrides {
                bytes: Some([0xB3; 20]),
            })
        }),
        nonce: overrides
            .nonce
            .unwrap_or(CanonicalFieldValue::Unsigned64(7)),
        expiry: overrides
            .expiry
            .unwrap_or(CanonicalFieldValue::Unsigned64(0x0A0B_0C0D_0E0F_1011)),
        call: overrides
            .call
            .unwrap_or_else(|| build_intent_call(Default::default())),
    }
}

pub fn build_signed_intent_payload(
    inputs: &SignedIntentInputs,
) -> SignedIntentPayload<'_, EvmForms> {
    SignedIntentPayload {
        chain: &inputs.chain,
        contract: &inputs.contract,
        identity: &inputs.identity,
        nonce: inputs.nonce.clone(),
        expiry: inputs.expiry.clone(),
        call: &inputs.call,
    }
}

#[derive(Default)]
pub struct SignedIntentParamsOverrides;

pub fn build_signed_intent_params(_overrides: SignedIntentParamsOverrides) -> SignedIntentParams {
    SignedIntentParams
}

#[derive(Default)]
pub struct DeriveKeyIdentifierParamsOverrides;

pub fn build_derive_key_identifier_params(
    _overrides: DeriveKeyIdentifierParamsOverrides,
) -> DeriveKeyIdentifierParams {
    DeriveKeyIdentifierParams
}
