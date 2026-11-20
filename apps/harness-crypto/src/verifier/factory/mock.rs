#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use chain::{
    AdmitChainCompositionFn, ChainFormsIdentifier, ChainMemberIdentifier, ConsumeChainFormsParams,
    ConsumeChainFormsPayload, ConsumeChainParams, ConsumeChainPayload, CreateChainFormsParams,
    CreateChainParams, FeeDescription, IChainAdapter, IChainConsumer, IChainForms,
    IChainFormsConsumer, IEntitlementStateAdapter, admit_chain_composition,
    build_create_chain_deps, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, build_create_chain_params, build_create_chain_payload,
    build_fee_description, create_chain, create_chain_forms,
};
use core::convert::Infallible;
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    EncodingDeclaration, EncodingIdentifier, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, VerifierGroupArithmetic,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use proof::{
    AlgebraicStatement, ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload,
    CreateDeliveryProofParams, DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofComponents,
    DeliveryRelation, IDeliveryProofAdapter, IDeliveryProofConsumer, ProofComponentsParams,
    ProofComponentsPayload, VerifyPayload, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};

use super::interface::{
    ConsumeCreateVerifierCaseParams, ConsumeCreateVerifierCasePayload, ConsumeVerifierParams,
    ConsumeVerifierPayload, CreateVerifierCaseDeps, CreateVerifierCaseErrorReturn,
    CreateVerifierCaseParams, CreateVerifierCasePayload, CreateVerifierCaseReturn,
    CreateVerifierCaseSuccessReturn, CreateVerifierDeps, CreateVerifierFn, CreateVerifierParams,
    CreateVerifierPayload, CreateVerifierReturn, CreateVerifierSuccessReturn,
    ICreateVerifierCaseConsumer, IVerifierAdapter, IVerifierConsumer,
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    RunCreateVerifierCaseFn, VERIFIER_INTERFACE_VERSION, VerifierConcrete, VerifierConcreteKind,
    VerifierDeclaration, VerifierSettingBounds, VerifierSettingDisposition,
    VerifierSettingStatement, VerifyCost, VerifyOnChainErrorReturn, VerifyOnChainParams,
    VerifyOnChainReturn, VerifyOnChainSuccessReturn,
};

use super::super::admit_verifier_composition::provides::{
    AdmitVerifierCompositionFn, mock_admit_verifier_composition,
};
use super::super::evm::provides::{
    EvmVerifierSettings, build_evm_verifier_settings, invalidate_evm_verifier_settings,
};
use crate::generate::provides::GenerateDeclaration;

#[derive(Default)]
pub struct VerifierDeclarationOverrides {
    pub identifier: Option<VerifierConcreteKind>,
    pub name: Option<&'static str>,
    pub verifier_forms: Option<&'static [VerifierGroupArithmetic]>,
    pub statement_versions: Option<&'static [u16]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
    pub chain_forms_identifier: Option<ChainFormsIdentifier>,
    pub chain_member: Option<ChainMemberIdentifier>,
    pub encoding_identifier: Option<EncodingIdentifier>,
    pub contract_suite: Option<&'static str>,
    pub settings: Option<&'static [VerifierSettingStatement]>,
}

pub fn build_verifier_declaration(overrides: VerifierDeclarationOverrides) -> VerifierDeclaration {
    VerifierDeclaration {
        identifier: overrides.identifier.unwrap_or(VerifierConcreteKind::Evm),
        name: overrides.name.unwrap_or("evm"),
        verifier_forms: overrides.verifier_forms.unwrap_or(&[
            VerifierGroupArithmetic::FirstGroupOnly,
            VerifierGroupArithmetic::BothGroups,
        ]),
        statement_versions: overrides
            .statement_versions
            .unwrap_or(&[DELIVERY_STATEMENT_VERSION_ONE]),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(VERIFIER_INTERFACE_VERSION),
        chain_forms_identifier: overrides
            .chain_forms_identifier
            .unwrap_or(ChainFormsIdentifier::EvmV1),
        chain_member: overrides
            .chain_member
            .unwrap_or(ChainMemberIdentifier::Base),
        encoding_identifier: overrides
            .encoding_identifier
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
        contract_suite: overrides.contract_suite.unwrap_or("evm"),
        settings: overrides.settings.unwrap_or(&[]),
    }
}

#[derive(Default)]
pub struct VerifierSettingStatementOverrides {
    pub name: Option<&'static str>,
    pub disposition: Option<VerifierSettingDisposition>,
}

pub fn build_verifier_setting_statement(
    overrides: VerifierSettingStatementOverrides,
) -> VerifierSettingStatement {
    VerifierSettingStatement {
        name: overrides.name.unwrap_or("setting"),
        disposition: overrides
            .disposition
            .unwrap_or(VerifierSettingDisposition::Settable {
                bounds: VerifierSettingBounds::Unbounded,
                default: "default",
            }),
    }
}

#[derive(Default)]
pub struct VerifyOnChainParamsOverrides;

pub fn build_verify_on_chain_params(
    _overrides: VerifyOnChainParamsOverrides,
) -> VerifyOnChainParams {
    VerifyOnChainParams
}

#[derive(Default)]
pub struct VerifyCostOverrides {
    pub description: Option<FeeDescription>,
}

pub fn build_verify_cost(overrides: VerifyCostOverrides) -> VerifyCost {
    VerifyCost {
        description: overrides
            .description
            .unwrap_or_else(|| build_fee_description(Default::default())),
    }
}

pub(crate) const MOCK_I_VERIFIER_ADAPTER_DECLARATION: VerifierDeclaration = VerifierDeclaration {
    identifier: VerifierConcreteKind::Mock,
    name: "mock",
    verifier_forms: &[
        VerifierGroupArithmetic::FirstGroupOnly,
        VerifierGroupArithmetic::BothGroups,
    ],
    statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
    adapter_version: 1,
    interface_version: VERIFIER_INTERFACE_VERSION,
    chain_forms_identifier: ChainFormsIdentifier::EvmV1,
    chain_member: ChainMemberIdentifier::Mock,
    encoding_identifier: EncodingIdentifier::Mock,
    contract_suite: "evm",
    settings: &[],
};

pub(crate) struct MockIVerifierAdapterConstructorParams<'a, D: IDeliveryProofAdapter> {
    pub failure_mode: MockIVerifierAdapterFailureMode,
    pub delivery_proof: &'a D,
}

#[derive(Default)]
pub struct MockIVerifierAdapterConstructorParamsOverrides {
    pub failure_mode: Option<MockIVerifierAdapterFailureMode>,
}

pub(crate) fn build_mock_i_verifier_adapter_constructor_params<'a, D: IDeliveryProofAdapter>(
    delivery_proof: &'a D,
    overrides: MockIVerifierAdapterConstructorParamsOverrides,
) -> MockIVerifierAdapterConstructorParams<'a, D> {
    MockIVerifierAdapterConstructorParams {
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIVerifierAdapterFailureMode::NoFailure),
        delivery_proof,
    }
}

pub(crate) struct MockIVerifierAdapter<'a, D: IDeliveryProofAdapter> {
    pub(super) failure_mode: MockIVerifierAdapterFailureMode,
    pub(super) delivery_proof: &'a D,
}

pub(crate) type MockIVerifierAdapterTryNewReturn<'a, D> =
    Result<MockIVerifierAdapter<'a, D>, Infallible>;

impl<'a, D: IDeliveryProofAdapter> MockIVerifierAdapter<'a, D> {
    pub(crate) fn try_new(
        params: MockIVerifierAdapterConstructorParams<'a, D>,
    ) -> MockIVerifierAdapterTryNewReturn<'a, D> {
        Ok(MockIVerifierAdapter {
            failure_mode: params.failure_mode,
            delivery_proof: params.delivery_proof,
        })
    }
}

pub(crate) fn build_mock_i_verifier_adapter<'a, D: IDeliveryProofAdapter>(
    delivery_proof: &'a D,
    overrides: MockIVerifierAdapterConstructorParamsOverrides,
) -> MockIVerifierAdapter<'a, D> {
    let Ok(adapter) = MockIVerifierAdapter::try_new(
        build_mock_i_verifier_adapter_constructor_params(delivery_proof, overrides),
    );
    adapter
}

impl<'a, D: IDeliveryProofAdapter> IVerifierAdapter for MockIVerifierAdapter<'a, D> {
    const DECLARATION: VerifierDeclaration = MOCK_I_VERIFIER_ADAPTER_DECLARATION;

    type DeliveryProof = D;

    fn verify_on_chain(
        &self,
        _params: VerifyOnChainParams,
        payload: VerifyPayload<
            '_,
            D::Forms,
            <D::Pairing as IPairingAdapter>::G1,
            <D::Pairing as IPairingAdapter>::G2,
            D::Proof,
        >,
    ) -> VerifyOnChainReturn {
        let statement = match &payload.statement {
            AlgebraicStatement::Mint(_) => DeliveryRelation::Mint,
            AlgebraicStatement::Transfer(_) => DeliveryRelation::Transfer,
        };
        let Ok(components) = self.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: payload.proof,
            },
        );
        let proof = match components.components {
            DeliveryProofComponents::MintBothGroups { .. }
            | DeliveryProofComponents::MintFirstGroupOnly { .. } => DeliveryRelation::Mint,
            DeliveryProofComponents::TransferBothGroups { .. }
            | DeliveryProofComponents::TransferFirstGroupOnly { .. } => DeliveryRelation::Transfer,
        };
        if statement != proof {
            return Err(VerifyOnChainErrorReturn::RelationMismatch { statement, proof });
        }
        match self.failure_mode {
            MockIVerifierAdapterFailureMode::RpcRefused => Err(VerifyOnChainErrorReturn::Mock(
                MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused,
            )),
            MockIVerifierAdapterFailureMode::NoFailure => {
                Ok(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(Default::default()),
                })
            }
        }
    }
}

#[derive(Default)]
pub struct CreateVerifierParamsOverrides {
    pub concrete: Option<VerifierConcrete>,
    pub verifier_address: Option<CanonicalFieldValue>,
    pub statement_version: Option<u16>,
}

pub fn build_create_verifier_params(
    overrides: CreateVerifierParamsOverrides,
) -> CreateVerifierParams {
    CreateVerifierParams {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_verifier_concrete(Default::default())),
        verifier_address: overrides
            .verifier_address
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x11; 20])),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct CreateVerifierParamsCorruptions {
    pub concrete: Option<Value>,
    pub verifier_address: Option<Value>,
    pub statement_version: Option<Value>,
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn canonical_field_value(value: &CanonicalFieldValue) -> Value {
    let mut variant = Map::new();
    match value {
        CanonicalFieldValue::FixedBytes32(bytes) => {
            variant.insert("FixedBytes32".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::Unsigned16(value) => {
            variant.insert("Unsigned16".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Unsigned32(value) => {
            variant.insert("Unsigned32".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Unsigned64(value) => {
            variant.insert("Unsigned64".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Text(text) => {
            variant.insert("Text".to_string(), Value::from(text.clone()));
        }
        CanonicalFieldValue::Bytes(bytes) => {
            variant.insert("Bytes".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::FixedBytes20(bytes) => {
            variant.insert("FixedBytes20".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::Unsigned256(bytes) => {
            variant.insert("Unsigned256".to_string(), byte_array(bytes));
        }
    }
    Value::Object(variant)
}

pub fn invalidate_create_verifier_params(corruptions: CreateVerifierParamsCorruptions) -> Value {
    let params = build_create_verifier_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        match &params.concrete {
            VerifierConcrete::Evm { .. } => {
                let mut variant = Map::new();
                variant.insert(
                    "settings".to_string(),
                    invalidate_evm_verifier_settings(Default::default()),
                );
                let mut concrete = Map::new();
                concrete.insert("Evm".to_string(), Value::Object(variant));
                Value::Object(concrete)
            }
            VerifierConcrete::Mock(failure_mode) => {
                let mut concrete = Map::new();
                concrete.insert(
                    "Mock".to_string(),
                    Value::from(match failure_mode {
                        MockIVerifierAdapterFailureMode::NoFailure => "NoFailure",
                        MockIVerifierAdapterFailureMode::RpcRefused => "RpcRefused",
                    }),
                );
                Value::Object(concrete)
            }
        },
    );
    fields.insert(
        "verifier_address".to_string(),
        canonical_field_value(&params.verifier_address),
    );
    fields.insert(
        "statement_version".to_string(),
        Value::from(params.statement_version),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(verifier_address) = corruptions.verifier_address {
        fields.insert("verifier_address".to_string(), verifier_address);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct VerifierConcreteOverrides {
    pub kind: Option<VerifierConcreteKind>,
    pub settings: Option<EvmVerifierSettings>,
    pub failure_mode: Option<MockIVerifierAdapterFailureMode>,
}

pub fn build_verifier_concrete(overrides: VerifierConcreteOverrides) -> VerifierConcrete {
    match overrides.kind.unwrap_or(VerifierConcreteKind::Mock) {
        VerifierConcreteKind::Evm => VerifierConcrete::Evm {
            settings: Box::new(
                overrides
                    .settings
                    .unwrap_or_else(|| build_evm_verifier_settings(Default::default())),
            ),
        },
        VerifierConcreteKind::Mock => VerifierConcrete::Mock(
            overrides
                .failure_mode
                .unwrap_or(MockIVerifierAdapterFailureMode::NoFailure),
        ),
    }
}

#[derive(Default)]
pub struct CreateVerifierSuccessReturnOverrides;

pub fn build_create_verifier_success_return<O>(
    output: O,
    _overrides: CreateVerifierSuccessReturnOverrides,
) -> CreateVerifierSuccessReturn<O> {
    CreateVerifierSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeVerifierParamsOverrides;

pub fn build_consume_verifier_params(
    _overrides: ConsumeVerifierParamsOverrides,
) -> ConsumeVerifierParams {
    ConsumeVerifierParams
}

#[derive(Default)]
pub struct ConsumeVerifierPayloadOverrides;

pub fn build_consume_verifier_payload<V: IVerifierAdapter>(
    adapter: V,
    _overrides: ConsumeVerifierPayloadOverrides,
) -> ConsumeVerifierPayload<V> {
    ConsumeVerifierPayload { adapter }
}

#[derive(Default)]
pub struct CreateVerifierPayloadOverrides;

pub fn build_create_verifier_payload(
    _overrides: CreateVerifierPayloadOverrides,
) -> CreateVerifierPayload {
    CreateVerifierPayload
}

#[derive(Default)]
pub struct CreateVerifierDepsOverrides {
    pub admit_verifier_composition: Option<AdmitVerifierCompositionFn>,
}

pub fn build_create_verifier_deps<'a, P, C, D, K>(
    pairing: &'a P,
    chain: &'a C,
    delivery_proof: &'a D,
    encoder: &'a EncodingDeclaration,
    generate: &'a GenerateDeclaration,
    consumer: K,
    overrides: CreateVerifierDepsOverrides,
) -> CreateVerifierDeps<'a, P, C, D, K>
where
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
{
    CreateVerifierDeps {
        pairing,
        chain,
        delivery_proof,
        encoder,
        generate,
        admit_verifier_composition: overrides
            .admit_verifier_composition
            .unwrap_or(mock_admit_verifier_composition),
        consumer,
    }
}

pub struct MockIVerifierConsumer;

impl<D: IDeliveryProofAdapter> IVerifierConsumer<D> for MockIVerifierConsumer {
    type Output = ();

    fn consume_verifier<V: IVerifierAdapter<DeliveryProof = D>>(
        &self,
        _params: ConsumeVerifierParams,
        _payload: ConsumeVerifierPayload<V>,
    ) -> Self::Output {
    }
}

pub fn mock_create_verifier<'a, P, C, D, K>(
    deps: &CreateVerifierDeps<'a, P, C, D, K>,
    _params: CreateVerifierParams,
    _payload: CreateVerifierPayload,
) -> CreateVerifierReturn<K::Output>
where
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
    K: IVerifierConsumer<D>,
{
    let _: CreateVerifierFn<'a, P, C, D, K> = mock_create_verifier;
    let adapter = build_mock_i_verifier_adapter(deps.delivery_proof, Default::default());
    let output = deps.consumer.consume_verifier(
        build_consume_verifier_params(Default::default()),
        build_consume_verifier_payload(adapter, Default::default()),
    );
    Ok(build_create_verifier_success_return(
        output,
        Default::default(),
    ))
}

#[derive(Default)]
pub struct CreateVerifierCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub chain: Option<CreateChainParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_create_verifier_case_params(
    overrides: CreateVerifierCaseParamsOverrides,
) -> CreateVerifierCaseParams {
    CreateVerifierCaseParams {
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
        chain: overrides
            .chain
            .unwrap_or_else(|| build_create_chain_params(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct CreateVerifierCaseDepsOverrides;

pub fn build_create_verifier_case_deps<C: ICreateVerifierCaseConsumer>(
    consumer: C,
    _overrides: CreateVerifierCaseDepsOverrides,
) -> CreateVerifierCaseDeps<C> {
    CreateVerifierCaseDeps { consumer }
}

#[derive(Default)]
pub struct CreateVerifierCasePayloadOverrides;

pub fn build_create_verifier_case_payload<'a>(
    generate: &'a GenerateDeclaration,
    _overrides: CreateVerifierCasePayloadOverrides,
) -> CreateVerifierCasePayload<'a> {
    CreateVerifierCasePayload { generate }
}

#[derive(Default)]
pub struct ConsumeCreateVerifierCaseParamsOverrides;

pub fn build_consume_create_verifier_case_params(
    _overrides: ConsumeCreateVerifierCaseParamsOverrides,
) -> ConsumeCreateVerifierCaseParams {
    ConsumeCreateVerifierCaseParams
}

#[derive(Default)]
pub struct ConsumeCreateVerifierCasePayloadOverrides;

pub fn build_consume_create_verifier_case_payload<'a, P, C, D>(
    pairing: &'a P,
    chain: &'a C,
    delivery_proof: &'a D,
    encoder: &'a EncodingDeclaration,
    generate: &'a GenerateDeclaration,
    _overrides: ConsumeCreateVerifierCasePayloadOverrides,
) -> ConsumeCreateVerifierCasePayload<'a, P, C, D>
where
    P: IPairingArithmetic + IPairingReference,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
{
    ConsumeCreateVerifierCasePayload {
        pairing,
        chain,
        delivery_proof,
        encoder,
        generate,
    }
}

#[derive(Default)]
pub struct CreateVerifierCaseSuccessReturnOverrides;

pub fn build_create_verifier_case_success_return<O>(
    output: O,
    _overrides: CreateVerifierCaseSuccessReturnOverrides,
) -> CreateVerifierCaseSuccessReturn<O> {
    CreateVerifierCaseSuccessReturn { output }
}

struct CaseAtDeliveryProof<
    'a,
    P: IPairingArithmetic,
    K: IChainAdapter,
    C: ICreateVerifierCaseConsumer,
> {
    pairing: &'a P,
    chain: &'a K,
    encoder: &'a EncodingDeclaration,
    generate: &'a GenerateDeclaration,
    consumer: &'a C,
}

impl<'a, P, K, C> IDeliveryProofConsumer<P, K::Forms> for CaseAtDeliveryProof<'a, P, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    K: IChainAdapter,
    C: ICreateVerifierCaseConsumer,
{
    type Output = CreateVerifierCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = K::Forms>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(CreateVerifierCaseSuccessReturn {
            output: self.consumer.consume_create_verifier_case(
                build_consume_create_verifier_case_params(Default::default()),
                build_consume_create_verifier_case_payload(
                    self.pairing,
                    self.chain,
                    &payload.adapter,
                    self.encoder,
                    self.generate,
                    Default::default(),
                ),
            ),
        })
    }
}

struct CaseAtChain<'a, P: IPairingArithmetic, E, C: ICreateVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    encoder_declaration: &'a EncodingDeclaration,
    source: &'a dyn IRandomSourceAdapter,
    delivery_proof: &'a CreateDeliveryProofParams,
    generate: &'a GenerateDeclaration,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainConsumer for CaseAtChain<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    C: ICreateVerifierCaseConsumer,
{
    type Output = CreateVerifierCaseReturn<C::Output>;

    fn consume_chain<K: IChainAdapter>(
        &self,
        _params: ConsumeChainParams,
        payload: ConsumeChainPayload<K>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    chain: &payload.adapter,
                    encoder: self.encoder_declaration,
                    generate: self.generate,
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
            Err(error) => Err(CreateVerifierCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: ICreateVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    encoder_declaration: &'a EncodingDeclaration,
    source: &'a dyn IRandomSourceAdapter,
    chain: &'a CreateChainParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    generate: &'a GenerateDeclaration,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    C: ICreateVerifierCaseConsumer,
{
    type Output = CreateVerifierCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_chain::<F, _>(
            &build_create_chain_deps(
                admit_chain_composition as AdmitChainCompositionFn,
                CaseAtChain {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    encoder_declaration: self.encoder_declaration,
                    source: self.source,
                    delivery_proof: self.delivery_proof,
                    generate: self.generate,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            self.chain.clone(),
            build_create_chain_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateVerifierCaseErrorReturn::CreateChain(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: ICreateVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    chain: &'a CreateChainParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    generate: &'a GenerateDeclaration,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: ICreateVerifierCaseConsumer,
{
    type Output = CreateVerifierCaseReturn<C::Output>;

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
                    encoder_declaration: &E::DECLARATION,
                    source: self.source,
                    chain: self.chain,
                    delivery_proof: self.delivery_proof,
                    generate: self.generate,
                    consumer: self.consumer,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateVerifierCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: ICreateVerifierCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    chain: &'a CreateChainParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    generate: &'a GenerateDeclaration,
    consumer: &'a C,
}

impl<C: ICreateVerifierCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = CreateVerifierCaseReturn<C::Output>;

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
                return Err(CreateVerifierCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(CreateVerifierCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hashing: &*hashing.adapter,
                source: &*source.adapter,
                chain_forms: self.chain_forms,
                chain: self.chain,
                delivery_proof: self.delivery_proof,
                generate: self.generate,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateVerifierCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_create_verifier_case<C: ICreateVerifierCaseConsumer>(
    deps: &CreateVerifierCaseDeps<C>,
    params: CreateVerifierCaseParams,
    payload: CreateVerifierCasePayload<'_>,
) -> CreateVerifierCaseReturn<C::Output> {
    let _: RunCreateVerifierCaseFn<C> = run_create_verifier_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                random_source: &params.random_source,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                chain: &params.chain,
                delivery_proof: &params.delivery_proof,
                generate: payload.generate,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(CreateVerifierCaseErrorReturn::CreatePairing(error)),
    }
}
