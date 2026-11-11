#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use alloy::transports::mock::Asserter;
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParams,
    IDeliveryProofAdapter, IDeliveryProofConsumer, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};

use super::interface::{
    EvmVerifier, EvmVerifierCaseDeps, EvmVerifierCaseErrorReturn, EvmVerifierCaseParams,
    EvmVerifierCasePayload, EvmVerifierCaseReturn, EvmVerifierCaseSuccessReturn,
    EvmVerifierConstructorParams, EvmVerifierEndpoint, EvmVerifierEndpointConstructorParams,
    EvmVerifierSettings, IEvmVerifierCaseConsumer, RunEvmVerifierCaseFn,
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

#[derive(Default)]
pub struct EvmVerifierCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_evm_verifier_case_params(
    overrides: EvmVerifierCaseParamsOverrides,
) -> EvmVerifierCaseParams {
    EvmVerifierCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_create_chain_forms_params(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct EvmVerifierCaseDepsOverrides;

pub fn build_evm_verifier_case_deps<C: IEvmVerifierCaseConsumer>(
    consumer: C,
    _overrides: EvmVerifierCaseDepsOverrides,
) -> EvmVerifierCaseDeps<C> {
    EvmVerifierCaseDeps { consumer }
}

#[derive(Default)]
pub struct EvmVerifierCasePayloadOverrides;

pub fn build_evm_verifier_case_payload(
    _overrides: EvmVerifierCasePayloadOverrides,
) -> EvmVerifierCasePayload {
    EvmVerifierCasePayload
}

#[derive(Default)]
pub struct EvmVerifierCaseSuccessReturnOverrides;

pub fn build_evm_verifier_case_success_return<O>(
    output: O,
    _overrides: EvmVerifierCaseSuccessReturnOverrides,
) -> EvmVerifierCaseSuccessReturn<O> {
    EvmVerifierCaseSuccessReturn { output }
}

struct CaseAtDeliveryProof<'a, P: IPairingArithmetic, F: IChainForms, C: IEvmVerifierCaseConsumer> {
    pairing: &'a P,
    forms: &'a F,
    consumer: &'a C,
}

impl<'a, P, F, C> IDeliveryProofConsumer<P, F> for CaseAtDeliveryProof<'a, P, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    F: IChainForms,
    C: IEvmVerifierCaseConsumer,
{
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(EvmVerifierCaseSuccessReturn {
            output: self.consumer.consume_evm_verifier_case(
                self.pairing,
                self.forms,
                &payload.adapter,
            ),
        })
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: IEvmVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    C: IEvmVerifierCaseConsumer,
{
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    forms: &payload.forms,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            CreateDeliveryProofParams {
                concrete: self.delivery_proof.concrete,
                algebra: self.delivery_proof.algebra,
                statement_version: self.delivery_proof.statement_version,
            },
            build_create_delivery_proof_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(EvmVerifierCaseErrorReturn::DeliveryProof(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: IEvmVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IEvmVerifierCaseConsumer,
{
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: &payload.adapter,
                    source: self.source,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(EvmVerifierCaseErrorReturn::ChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IEvmVerifierCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
}

impl<C: IEvmVerifierCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hashing = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            *self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(hashing) => hashing,
            Err(error) => {
                return Err(EvmVerifierCaseErrorReturn::HashToScalar(error));
            }
        };
        let source = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(EvmVerifierCaseErrorReturn::RandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hashing: &*hashing.adapter,
                source: &*source.adapter,
                chain_forms: self.chain_forms,
                delivery_proof: self.delivery_proof,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(EvmVerifierCaseErrorReturn::Encoding(error)),
        }
    }
}

pub fn run_evm_verifier_case<C: IEvmVerifierCaseConsumer>(
    deps: &EvmVerifierCaseDeps<C>,
    params: EvmVerifierCaseParams,
    _payload: EvmVerifierCasePayload,
) -> EvmVerifierCaseReturn<C::Output> {
    let _: RunEvmVerifierCaseFn<C> = run_evm_verifier_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                random_source: &params.random_source,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                delivery_proof: &params.delivery_proof,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(EvmVerifierCaseErrorReturn::Pairing(error)),
    }
}
