#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use alloy::transports::mock::Asserter;
use encoding::CanonicalFieldValue;
use pairing::IPairingAdapter;
use proof::IDeliveryProofAdapter;
use serde_json::{Map, Value};

use super::interface::{
    EvmVerifier, EvmVerifierConstructorParams, EvmVerifierEndpoint,
    EvmVerifierEndpointConstructorParams, EvmVerifierSettings,
};

#[derive(Default)]
pub struct EvmVerifierEndpointConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_evm_verifier_endpoint_constructor_params(
    overrides: EvmVerifierEndpointConstructorParamsOverrides,
) -> EvmVerifierEndpointConstructorParams {
    EvmVerifierEndpointConstructorParams {
        text: overrides
            .text
            .unwrap_or_else(|| "http://127.0.0.1:8545".to_string()),
    }
}

#[derive(Default)]
pub struct EvmVerifierEndpointConstructorParamsCorruptions {
    pub text: Option<Value>,
}

pub fn invalidate_evm_verifier_endpoint_constructor_params(
    corruptions: EvmVerifierEndpointConstructorParamsCorruptions,
) -> Value {
    let params = build_evm_verifier_endpoint_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("text".to_string(), Value::from(params.text));
    if let Some(text) = corruptions.text {
        fields.insert("text".to_string(), text);
    }
    Value::Object(fields)
}

pub fn build_evm_verifier_endpoint(
    overrides: EvmVerifierEndpointConstructorParamsOverrides,
) -> EvmVerifierEndpoint {
    EvmVerifierEndpoint::try_new(build_evm_verifier_endpoint_constructor_params(overrides))
        .expect("built params are a URL")
}

#[derive(Default)]
pub struct EvmVerifierSettingsOverrides {
    pub endpoint: Option<EvmVerifierEndpoint>,
    pub verifier_address: Option<CanonicalFieldValue>,
    pub gas_price_oracle: Option<CanonicalFieldValue>,
}

pub fn build_evm_verifier_settings(overrides: EvmVerifierSettingsOverrides) -> EvmVerifierSettings {
    EvmVerifierSettings {
        endpoint: overrides
            .endpoint
            .unwrap_or_else(|| build_evm_verifier_endpoint(Default::default())),
        verifier_address: overrides
            .verifier_address
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x11; 20])),
        gas_price_oracle: overrides
            .gas_price_oracle
            .unwrap_or(CanonicalFieldValue::FixedBytes20([
                0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x0f,
            ])),
    }
}

#[derive(Default)]
pub struct EvmVerifierSettingsCorruptions {
    pub endpoint: Option<Value>,
    pub verifier_address: Option<Value>,
    pub gas_price_oracle: Option<Value>,
}

fn address_text(value: &CanonicalFieldValue) -> String {
    let CanonicalFieldValue::FixedBytes20(bytes) = value else {
        panic!("the built address is a fixed-bytes-twenty field")
    };
    format!(
        "0x{}",
        bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    )
}

pub fn invalidate_evm_verifier_settings(corruptions: EvmVerifierSettingsCorruptions) -> Value {
    let settings = build_evm_verifier_settings(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "endpoint".to_string(),
        Value::from(settings.endpoint.url.as_str()),
    );
    fields.insert(
        "verifier_address".to_string(),
        Value::from(address_text(&settings.verifier_address)),
    );
    fields.insert(
        "gas_price_oracle".to_string(),
        Value::from(address_text(&settings.gas_price_oracle)),
    );
    if let Some(endpoint) = corruptions.endpoint {
        fields.insert("endpoint".to_string(), endpoint);
    }
    if let Some(verifier_address) = corruptions.verifier_address {
        fields.insert("verifier_address".to_string(), verifier_address);
    }
    if let Some(gas_price_oracle) = corruptions.gas_price_oracle {
        fields.insert("gas_price_oracle".to_string(), gas_price_oracle);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EvmVerifierConstructorParamsOverrides {
    pub settings: Option<EvmVerifierSettings>,
}

pub(crate) fn build_evm_verifier_constructor_params<
    'a,
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    delivery_proof: &'a D,
    overrides: EvmVerifierConstructorParamsOverrides,
) -> EvmVerifierConstructorParams<'a, P, D> {
    EvmVerifierConstructorParams {
        pairing,
        delivery_proof,
        settings: overrides
            .settings
            .unwrap_or_else(|| build_evm_verifier_settings(Default::default())),
    }
}

pub(crate) fn build_evm_verifier<'a, P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>>(
    pairing: &'a P,
    delivery_proof: &'a D,
    asserter: Asserter,
    overrides: EvmVerifierConstructorParamsOverrides,
) -> EvmVerifier<'a, P, D> {
    EvmVerifier::try_new_mocked(
        build_evm_verifier_constructor_params(pairing, delivery_proof, overrides),
        asserter,
    )
    .expect("built params are admitted and the runtime builds")
}
