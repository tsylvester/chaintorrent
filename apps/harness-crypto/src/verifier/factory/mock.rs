#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use core::convert::Infallible;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
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
    ProofComponentsPayload, VerifyAdapterPayload, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};

use super::interface::{
    ConsumeVerifierParams, ConsumeVerifierPayload, CreateVerifierCaseDeps,
    CreateVerifierCaseErrorReturn, CreateVerifierCaseParams, CreateVerifierCasePayload,
    CreateVerifierCaseReturn, CreateVerifierCaseSuccessReturn, CreateVerifierDeps,
    CreateVerifierFn, CreateVerifierParams, CreateVerifierPayload, CreateVerifierReturn,
    CreateVerifierSuccessReturn, ICreateVerifierCaseConsumer, IVerifierAdapter, IVerifierConsumer,
    L1DataFee, MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    RunCreateVerifierCaseFn, VERIFIER_INTERFACE_VERSION, VerifierConcrete, VerifierConcreteKind,
    VerifierDeclaration, VerifyCost, VerifyOnChainErrorReturn, VerifyOnChainParams,
    VerifyOnChainReturn, VerifyOnChainSuccessReturn,
};

use super::super::evm::provides::{build_evm_verifier_settings, invalidate_evm_verifier_settings};

#[derive(Default)]
pub struct VerifierDeclarationOverrides {
    pub verifier_forms: Option<&'static [VerifierGroupArithmetic]>,
    pub statement_versions: Option<&'static [u16]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_verifier_declaration(overrides: VerifierDeclarationOverrides) -> VerifierDeclaration {
    VerifierDeclaration {
        verifier_forms: overrides
            .verifier_forms
            .unwrap_or(&[VerifierGroupArithmetic::BothGroups]),
        statement_versions: overrides
            .statement_versions
            .unwrap_or(&[DELIVERY_STATEMENT_VERSION_ONE]),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(VERIFIER_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct VerifierDeclarationCorruptions {
    pub verifier_forms: Option<Value>,
    pub statement_versions: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_verifier_declaration(corruptions: VerifierDeclarationCorruptions) -> Value {
    let declaration = build_verifier_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "verifier_forms".to_string(),
        Value::Array(
            declaration
                .verifier_forms
                .iter()
                .map(|form| {
                    Value::from(match form {
                        VerifierGroupArithmetic::BothGroups => "BothGroups",
                        VerifierGroupArithmetic::FirstGroupOnly => "FirstGroupOnly",
                    })
                })
                .collect(),
        ),
    );
    fields.insert(
        "statement_versions".to_string(),
        Value::Array(
            declaration
                .statement_versions
                .iter()
                .map(|version| Value::from(*version))
                .collect(),
        ),
    );
    fields.insert(
        "adapter_version".to_string(),
        Value::from(declaration.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(declaration.interface_version),
    );
    if let Some(verifier_forms) = corruptions.verifier_forms {
        fields.insert("verifier_forms".to_string(), verifier_forms);
    }
    if let Some(statement_versions) = corruptions.statement_versions {
        fields.insert("statement_versions".to_string(), statement_versions);
    }
    if let Some(adapter_version) = corruptions.adapter_version {
        fields.insert("adapter_version".to_string(), adapter_version);
    }
    if let Some(interface_version) = corruptions.interface_version {
        fields.insert("interface_version".to_string(), interface_version);
    }
    Value::Object(fields)
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
    pub l2_execution_gas: Option<u64>,
    pub l1_data_fee: Option<L1DataFee>,
}

pub fn build_verify_cost(overrides: VerifyCostOverrides) -> VerifyCost {
    VerifyCost {
        l2_execution_gas: overrides.l2_execution_gas.unwrap_or(100_000),
        l1_data_fee: overrides
            .l1_data_fee
            .unwrap_or(L1DataFee::Charged { wei: 1_000 }),
    }
}

#[derive(Default)]
pub struct VerifyCostCorruptions {
    pub l2_execution_gas: Option<Value>,
    pub l1_data_fee: Option<Value>,
}

pub fn invalidate_verify_cost(corruptions: VerifyCostCorruptions) -> Value {
    let cost = build_verify_cost(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "l2_execution_gas".to_string(),
        Value::from(cost.l2_execution_gas),
    );
    fields.insert(
        "l1_data_fee".to_string(),
        match cost.l1_data_fee {
            L1DataFee::Charged { wei } => {
                let mut charged = Map::new();
                let mut wei_field = Map::new();
                wei_field.insert("wei".to_string(), serde_json::to_value(wei).unwrap());
                charged.insert("Charged".to_string(), Value::Object(wei_field));
                Value::Object(charged)
            }
            L1DataFee::NotApplicable => Value::from("NotApplicable"),
        },
    );
    if let Some(l2_execution_gas) = corruptions.l2_execution_gas {
        fields.insert("l2_execution_gas".to_string(), l2_execution_gas);
    }
    if let Some(l1_data_fee) = corruptions.l1_data_fee {
        fields.insert("l1_data_fee".to_string(), l1_data_fee);
    }
    Value::Object(fields)
}

pub(crate) const MOCK_I_VERIFIER_ADAPTER_DECLARATION: VerifierDeclaration = VerifierDeclaration {
    verifier_forms: &[
        VerifierGroupArithmetic::FirstGroupOnly,
        VerifierGroupArithmetic::BothGroups,
    ],
    statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
    adapter_version: 1,
    interface_version: VERIFIER_INTERFACE_VERSION,
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
        payload: VerifyAdapterPayload<'_, D>,
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
    pub statement_version: Option<u16>,
}

pub fn build_create_verifier_params(
    overrides: CreateVerifierParamsOverrides,
) -> CreateVerifierParams {
    CreateVerifierParams {
        concrete: overrides.concrete.unwrap_or(VerifierConcrete::Mock(
            MockIVerifierAdapterFailureMode::NoFailure,
        )),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct CreateVerifierParamsCorruptions {
    pub concrete: Option<Value>,
    pub statement_version: Option<Value>,
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
        "statement_version".to_string(),
        Value::from(params.statement_version),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct VerifierConcreteOverrides;

pub fn build_verifier_concrete(
    kind: VerifierConcreteKind,
    _overrides: VerifierConcreteOverrides,
) -> VerifierConcrete {
    match kind {
        VerifierConcreteKind::Evm => VerifierConcrete::Evm {
            settings: Box::new(build_evm_verifier_settings(Default::default())),
        },
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
pub struct CreateVerifierDepsOverrides;

pub fn build_create_verifier_deps<
    'a,
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
    C,
>(
    pairing: &'a P,
    delivery_proof: &'a D,
    consumer: C,
    _overrides: CreateVerifierDepsOverrides,
) -> CreateVerifierDeps<'a, P, D, C> {
    CreateVerifierDeps {
        pairing,
        delivery_proof,
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

pub fn mock_create_verifier<
    'a,
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
    C: IVerifierConsumer<D>,
>(
    deps: &CreateVerifierDeps<'a, P, D, C>,
    _params: CreateVerifierParams,
    _payload: CreateVerifierPayload,
) -> CreateVerifierReturn<C::Output> {
    let _: CreateVerifierFn<'a, P, D, C> = mock_create_verifier;
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

pub fn build_create_verifier_case_payload(
    _overrides: CreateVerifierCasePayloadOverrides,
) -> CreateVerifierCasePayload {
    CreateVerifierCasePayload
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
    F: IChainForms,
    C: ICreateVerifierCaseConsumer,
> {
    pairing: &'a P,
    forms: &'a F,
    consumer: &'a C,
}

impl<'a, P, F, C> IDeliveryProofConsumer<P, F> for CaseAtDeliveryProof<'a, P, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    F: IChainForms,
    C: ICreateVerifierCaseConsumer,
{
    type Output = CreateVerifierCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(CreateVerifierCaseSuccessReturn {
            output: self.consumer.consume_create_verifier_case(
                self.pairing,
                self.forms,
                &payload.adapter,
            ),
        })
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: ICreateVerifierCaseConsumer> {
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
    C: ICreateVerifierCaseConsumer,
{
    type Output = CreateVerifierCaseReturn<C::Output>;

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
            Err(error) => Err(CreateVerifierCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: ICreateVerifierCaseConsumer> {
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
            Err(error) => Err(CreateVerifierCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: ICreateVerifierCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    delivery_proof: &'a CreateDeliveryProofParams,
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
            Err(error) => Err(CreateVerifierCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_create_verifier_case<C: ICreateVerifierCaseConsumer>(
    deps: &CreateVerifierCaseDeps<C>,
    params: CreateVerifierCaseParams,
    _payload: CreateVerifierCasePayload,
) -> CreateVerifierCaseReturn<C::Output> {
    let _: RunCreateVerifierCaseFn<C> = run_create_verifier_case;
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
        Err(error) => Err(CreateVerifierCaseErrorReturn::CreatePairing(error)),
    }
}
