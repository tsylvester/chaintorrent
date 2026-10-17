#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use core::convert::Infallible;
use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE,
    IDeliveryProofAdapter, MintPurpose, TransferPurpose,
};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};
use std::path::PathBuf;

use super::interface::{
    ConsumeGenerateParams, ConsumeGeneratePayload, CreateGenerateDeps, CreateGenerateFn,
    CreateGenerateParams, CreateGeneratePayload, CreateGenerateReturn, CreateGenerateSuccessReturn,
    EmitErrorReturn, EmitParams, EmitPayload, EmitReturn, EmitSuccessReturn, EmittedFile,
    GENERATE_INTERFACE_VERSION, GenerateConcrete, GenerateConcreteKind, GenerateDeclaration,
    IGenerateAdapter, IGenerateConsumer, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, VectorKind, VectorKindConfiguration, VectorKindDeclaration,
    VectorKindQuantities, VectorKindSettings, VectorKindSize, VectorKindSizing,
};

use super::super::evm::provides::{
    CanonicalDeliveryContext, EVM_GENERATE_DECLARATION, MsmTermCount, VectorCount,
    build_evm_generate_settings, build_msm_term_count, build_vector_count,
    invalidate_evm_generate_settings,
};

fn vector_kind_name(kind: VectorKind) -> &'static str {
    match kind {
        VectorKind::G1Addition => "G1Addition",
        VectorKind::G1Multiplication => "G1Multiplication",
        VectorKind::PairingCheck => "PairingCheck",
        VectorKind::G2Addition => "G2Addition",
        VectorKind::G1MultiScalarMultiplication => "G1MultiScalarMultiplication",
        VectorKind::G2MultiScalarMultiplication => "G2MultiScalarMultiplication",
        VectorKind::DecodeRejection => "DecodeRejection",
        VectorKind::HashToScalar => "HashToScalar",
        VectorKind::IdentityMapping => "IdentityMapping",
        VectorKind::Possession => "Possession",
        VectorKind::Challenge => "Challenge",
        VectorKind::MintProof => "MintProof",
        VectorKind::TransferProof => "TransferProof",
    }
}

fn vector_kind_sizing_name(sizing: VectorKindSizing) -> &'static str {
    match sizing {
        VectorKindSizing::Drawn => "Drawn",
        VectorKindSizing::DrawnTerms => "DrawnTerms",
        VectorKindSizing::DrawnMessages => "DrawnMessages",
        VectorKindSizing::Enumerated => "Enumerated",
        VectorKindSizing::OverConfiguredInputs => "OverConfiguredInputs",
    }
}

#[derive(Default)]
pub struct VectorKindDeclarationOverrides {
    pub kind: Option<VectorKind>,
    pub sizing: Option<VectorKindSizing>,
    pub requires_second_group_arithmetic: Option<bool>,
}

pub fn build_vector_kind_declaration(
    overrides: VectorKindDeclarationOverrides,
) -> VectorKindDeclaration {
    VectorKindDeclaration {
        kind: overrides.kind.unwrap_or(VectorKind::G1Addition),
        sizing: overrides.sizing.unwrap_or(VectorKindSizing::Drawn),
        requires_second_group_arithmetic: overrides
            .requires_second_group_arithmetic
            .unwrap_or(false),
    }
}

#[derive(Default)]
pub struct VectorKindDeclarationCorruptions {
    pub kind: Option<Value>,
    pub sizing: Option<Value>,
    pub requires_second_group_arithmetic: Option<Value>,
}

pub fn invalidate_vector_kind_declaration(corruptions: VectorKindDeclarationCorruptions) -> Value {
    let declaration = build_vector_kind_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kind".to_string(),
        Value::from(vector_kind_name(declaration.kind)),
    );
    fields.insert(
        "sizing".to_string(),
        Value::from(vector_kind_sizing_name(declaration.sizing)),
    );
    fields.insert(
        "requires_second_group_arithmetic".to_string(),
        Value::from(declaration.requires_second_group_arithmetic),
    );
    if let Some(kind) = corruptions.kind {
        fields.insert("kind".to_string(), kind);
    }
    if let Some(sizing) = corruptions.sizing {
        fields.insert("sizing".to_string(), sizing);
    }
    if let Some(requires_second_group_arithmetic) = corruptions.requires_second_group_arithmetic {
        fields.insert(
            "requires_second_group_arithmetic".to_string(),
            requires_second_group_arithmetic,
        );
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct GenerateDeclarationOverrides {
    pub kinds: Option<&'static [VectorKindDeclaration]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_generate_declaration(overrides: GenerateDeclarationOverrides) -> GenerateDeclaration {
    GenerateDeclaration {
        kinds: overrides.kinds.unwrap_or(&[]),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(GENERATE_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct GenerateDeclarationCorruptions {
    pub kinds: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_generate_declaration(corruptions: GenerateDeclarationCorruptions) -> Value {
    let declaration = build_generate_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kinds".to_string(),
        Value::Array(
            declaration
                .kinds
                .iter()
                .map(|_| invalidate_vector_kind_declaration(Default::default()))
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
    if let Some(kinds) = corruptions.kinds {
        fields.insert("kinds".to_string(), kinds);
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
pub struct VectorKindSettingsOverrides {
    pub kind: Option<VectorKind>,
    pub size: Option<VectorKindSize>,
}

pub fn build_vector_kind_settings(overrides: VectorKindSettingsOverrides) -> VectorKindSettings {
    VectorKindSettings {
        kind: overrides.kind.unwrap_or(VectorKind::G1Addition),
        size: overrides.size.unwrap_or(VectorKindSize::Drawn {
            count: build_vector_count(Default::default()),
        }),
    }
}

#[derive(Default)]
pub struct VectorKindSettingsCorruptions {
    pub kind: Option<Value>,
    pub size: Option<Value>,
}

fn vector_kind_size_value(size: VectorKindSize) -> Value {
    match size {
        VectorKindSize::Drawn { count } => {
            let mut fields = Map::new();
            fields.insert("count".to_string(), Value::from(count.get()));
            let mut size_fields = Map::new();
            size_fields.insert("Drawn".to_string(), Value::Object(fields));
            Value::Object(size_fields)
        }
        VectorKindSize::DrawnTerms { count, terms } => {
            let mut fields = Map::new();
            fields.insert("count".to_string(), Value::from(count.get()));
            fields.insert("terms".to_string(), Value::from(terms.get()));
            let mut size_fields = Map::new();
            size_fields.insert("DrawnTerms".to_string(), Value::Object(fields));
            Value::Object(size_fields)
        }
        VectorKindSize::DrawnMessages {
            count,
            message_length,
        } => {
            let mut fields = Map::new();
            fields.insert("count".to_string(), Value::from(count.get()));
            fields.insert(
                "message_length".to_string(),
                Value::from(
                    u64::try_from(message_length)
                        .expect("a usize fits a u64 on every supported target"),
                ),
            );
            let mut size_fields = Map::new();
            size_fields.insert("DrawnMessages".to_string(), Value::Object(fields));
            Value::Object(size_fields)
        }
        VectorKindSize::Enumerated => Value::from("Enumerated"),
        VectorKindSize::OverConfiguredInputs => Value::from("OverConfiguredInputs"),
    }
}

pub fn invalidate_vector_kind_settings(corruptions: VectorKindSettingsCorruptions) -> Value {
    let settings = build_vector_kind_settings(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kind".to_string(),
        Value::from(vector_kind_name(settings.kind)),
    );
    fields.insert("size".to_string(), vector_kind_size_value(settings.size));
    if let Some(kind) = corruptions.kind {
        fields.insert("kind".to_string(), kind);
    }
    if let Some(size) = corruptions.size {
        fields.insert("size".to_string(), size);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct VectorKindQuantitiesOverrides {
    pub count: Option<Option<VectorCount>>,
    pub terms: Option<Option<MsmTermCount>>,
    pub message_length: Option<Option<usize>>,
}

pub fn build_vector_kind_quantities(
    overrides: VectorKindQuantitiesOverrides,
) -> VectorKindQuantities {
    VectorKindQuantities {
        count: overrides
            .count
            .unwrap_or_else(|| Some(build_vector_count(Default::default()))),
        terms: overrides.terms.unwrap_or(None),
        message_length: overrides.message_length.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct VectorKindQuantitiesCorruptions {
    pub count: Option<Value>,
    pub terms: Option<Value>,
    pub message_length: Option<Value>,
}

pub fn invalidate_vector_kind_quantities(corruptions: VectorKindQuantitiesCorruptions) -> Value {
    let quantities = build_vector_kind_quantities(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "count".to_string(),
        quantities
            .count
            .map_or(Value::Null, |count| Value::from(count.get())),
    );
    fields.insert(
        "terms".to_string(),
        quantities
            .terms
            .map_or(Value::Null, |terms| Value::from(terms.get())),
    );
    fields.insert(
        "message_length".to_string(),
        quantities.message_length.map_or(Value::Null, |length| {
            Value::from(
                u64::try_from(length).expect("a usize fits a u64 on every supported target"),
            )
        }),
    );
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(terms) = corruptions.terms {
        fields.insert("terms".to_string(), terms);
    }
    if let Some(message_length) = corruptions.message_length {
        fields.insert("message_length".to_string(), message_length);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EmittedFileOverrides {
    pub kind: Option<Option<VectorKind>>,
    pub path: Option<PathBuf>,
}

pub fn build_emitted_file(overrides: EmittedFileOverrides) -> EmittedFile {
    EmittedFile {
        kind: overrides.kind.unwrap_or(Some(VectorKind::G1Addition)),
        path: overrides
            .path
            .unwrap_or_else(|| PathBuf::from("g1_add.vectors")),
    }
}

#[derive(Default)]
pub struct EmittedFileCorruptions {
    pub kind: Option<Value>,
    pub path: Option<Value>,
}

pub fn invalidate_emitted_file(corruptions: EmittedFileCorruptions) -> Value {
    let file = build_emitted_file(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kind".to_string(),
        file.kind
            .map_or(Value::Null, |kind| Value::from(vector_kind_name(kind))),
    );
    fields.insert(
        "path".to_string(),
        Value::from(file.path.display().to_string()),
    );
    if let Some(kind) = corruptions.kind {
        fields.insert("kind".to_string(), kind);
    }
    if let Some(path) = corruptions.path {
        fields.insert("path".to_string(), path);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EmitSuccessReturnOverrides {
    pub files: Option<Vec<EmittedFile>>,
}

pub fn build_emit_success_return(overrides: EmitSuccessReturnOverrides) -> EmitSuccessReturn {
    EmitSuccessReturn {
        files: overrides.files.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EmitSuccessReturnCorruptions {
    pub files: Option<Value>,
}

pub fn invalidate_emit_success_return(corruptions: EmitSuccessReturnCorruptions) -> Value {
    let success = build_emit_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "files".to_string(),
        Value::Array(
            success
                .files
                .iter()
                .map(|_| invalidate_emitted_file(Default::default()))
                .collect(),
        ),
    );
    if let Some(files) = corruptions.files {
        fields.insert("files".to_string(), files);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EmitParamsOverrides;

pub fn build_emit_params(_overrides: EmitParamsOverrides) -> EmitParams {
    EmitParams
}

#[derive(Default)]
pub struct EmitPayloadOverrides {
    pub mint_purpose: Option<MintPurpose>,
    pub transfer_purpose: Option<TransferPurpose>,
}

pub fn build_emit_payload<'a>(
    mint: &'a CanonicalDeliveryContext,
    transfer: &'a CanonicalDeliveryContext,
    identities: &'a [Vec<u8>],
    proof_identity: &'a [u8],
    overrides: EmitPayloadOverrides,
) -> EmitPayload<'a> {
    EmitPayload {
        mint,
        mint_purpose: overrides.mint_purpose.unwrap_or(DELIVERY_PURPOSE_MINT),
        transfer,
        transfer_purpose: overrides
            .transfer_purpose
            .unwrap_or(DELIVERY_PURPOSE_TRANSFER),
        identities,
        proof_identity,
    }
}

pub(crate) const MOCK_I_GENERATE_ADAPTER_DECLARATION: GenerateDeclaration = GenerateDeclaration {
    kinds: &[
        VectorKindDeclaration {
            kind: VectorKind::G1Addition,
            sizing: VectorKindSizing::Drawn,
            requires_second_group_arithmetic: false,
        },
        VectorKindDeclaration {
            kind: VectorKind::G1MultiScalarMultiplication,
            sizing: VectorKindSizing::DrawnTerms,
            requires_second_group_arithmetic: true,
        },
        VectorKindDeclaration {
            kind: VectorKind::HashToScalar,
            sizing: VectorKindSizing::DrawnMessages,
            requires_second_group_arithmetic: false,
        },
        VectorKindDeclaration {
            kind: VectorKind::DecodeRejection,
            sizing: VectorKindSizing::Enumerated,
            requires_second_group_arithmetic: false,
        },
        VectorKindDeclaration {
            kind: VectorKind::IdentityMapping,
            sizing: VectorKindSizing::OverConfiguredInputs,
            requires_second_group_arithmetic: false,
        },
    ],
    adapter_version: 1,
    interface_version: GENERATE_INTERFACE_VERSION,
};

pub(crate) struct MockIGenerateAdapterConstructorParams {
    pub failure_mode: MockIGenerateAdapterFailureMode,
}

#[derive(Default)]
pub struct MockIGenerateAdapterConstructorParamsOverrides {
    pub failure_mode: Option<MockIGenerateAdapterFailureMode>,
}

pub(crate) fn build_mock_i_generate_adapter_constructor_params(
    overrides: MockIGenerateAdapterConstructorParamsOverrides,
) -> MockIGenerateAdapterConstructorParams {
    MockIGenerateAdapterConstructorParams {
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIGenerateAdapterFailureMode::NoFailure),
    }
}

pub(crate) struct MockIGenerateAdapter {
    pub(super) failure_mode: MockIGenerateAdapterFailureMode,
}

pub(crate) type MockIGenerateAdapterTryNewReturn = Result<MockIGenerateAdapter, Infallible>;

impl MockIGenerateAdapter {
    pub(crate) fn try_new(
        params: MockIGenerateAdapterConstructorParams,
    ) -> MockIGenerateAdapterTryNewReturn {
        Ok(MockIGenerateAdapter {
            failure_mode: params.failure_mode,
        })
    }
}

pub(crate) fn build_mock_i_generate_adapter(
    overrides: MockIGenerateAdapterConstructorParamsOverrides,
) -> MockIGenerateAdapter {
    let Ok(adapter) =
        MockIGenerateAdapter::try_new(build_mock_i_generate_adapter_constructor_params(overrides));
    adapter
}

impl IGenerateAdapter for MockIGenerateAdapter {
    const DECLARATION: GenerateDeclaration = MOCK_I_GENERATE_ADAPTER_DECLARATION;

    fn emit(&self, _params: EmitParams, _payload: EmitPayload<'_>) -> EmitReturn {
        match self.failure_mode {
            MockIGenerateAdapterFailureMode::WriteRefused => Err(EmitErrorReturn::Mock(
                MockIGenerateAdapterEmitErrorReturn::WriteRefused,
            )),
            MockIGenerateAdapterFailureMode::NoFailure => {
                let files: Vec<EmittedFile> = Self::DECLARATION
                    .kinds
                    .iter()
                    .map(|declaration| EmittedFile {
                        kind: Some(declaration.kind),
                        path: PathBuf::from(format!("{:?}", declaration.kind)),
                    })
                    .collect();
                Ok(build_emit_success_return(EmitSuccessReturnOverrides {
                    files: Some(files),
                }))
            }
        }
    }
}

fn kind_configurations(kinds: &[VectorKindDeclaration]) -> Vec<VectorKindConfiguration> {
    kinds
        .iter()
        .map(|declaration| {
            build_vector_kind_configuration(VectorKindConfigurationOverrides {
                kind: Some(declaration.kind),
                quantities: Some(match declaration.sizing {
                    VectorKindSizing::Drawn => build_vector_kind_quantities(Default::default()),
                    VectorKindSizing::DrawnTerms => {
                        build_vector_kind_quantities(VectorKindQuantitiesOverrides {
                            terms: Some(Some(build_msm_term_count(Default::default()))),
                            ..Default::default()
                        })
                    }
                    VectorKindSizing::DrawnMessages => {
                        build_vector_kind_quantities(VectorKindQuantitiesOverrides {
                            message_length: Some(Some(32)),
                            ..Default::default()
                        })
                    }
                    VectorKindSizing::Enumerated | VectorKindSizing::OverConfiguredInputs => {
                        build_vector_kind_quantities(VectorKindQuantitiesOverrides {
                            count: Some(None),
                            ..Default::default()
                        })
                    }
                }),
            })
        })
        .collect()
}

#[derive(Default)]
pub struct GenerateConcreteOverrides;

pub fn build_generate_concrete(
    kind: GenerateConcreteKind,
    _overrides: GenerateConcreteOverrides,
) -> GenerateConcrete {
    match kind {
        GenerateConcreteKind::Evm => GenerateConcrete::Evm {
            settings: build_evm_generate_settings(Default::default()),
        },
    }
}

#[derive(Default)]
pub struct GenerateKindConfigurationsOverrides;

pub fn build_generate_kind_configurations(
    kind: GenerateConcreteKind,
    _overrides: GenerateKindConfigurationsOverrides,
) -> Vec<VectorKindConfiguration> {
    match kind {
        GenerateConcreteKind::Evm => kind_configurations(EVM_GENERATE_DECLARATION.kinds),
    }
}

#[derive(Default)]
pub struct VectorKindConfigurationOverrides {
    pub kind: Option<VectorKind>,
    pub quantities: Option<VectorKindQuantities>,
}

pub fn build_vector_kind_configuration(
    overrides: VectorKindConfigurationOverrides,
) -> VectorKindConfiguration {
    VectorKindConfiguration {
        kind: overrides.kind.unwrap_or(VectorKind::G1Addition),
        quantities: overrides
            .quantities
            .unwrap_or_else(|| build_vector_kind_quantities(Default::default())),
    }
}

#[derive(Default)]
pub struct VectorKindConfigurationCorruptions {
    pub kind: Option<Value>,
    pub quantities: Option<Value>,
}

pub fn invalidate_vector_kind_configuration(
    corruptions: VectorKindConfigurationCorruptions,
) -> Value {
    let configuration = build_vector_kind_configuration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kind".to_string(),
        Value::from(vector_kind_name(configuration.kind)),
    );
    fields.insert(
        "quantities".to_string(),
        invalidate_vector_kind_quantities(Default::default()),
    );
    if let Some(kind) = corruptions.kind {
        fields.insert("kind".to_string(), kind);
    }
    if let Some(quantities) = corruptions.quantities {
        fields.insert("quantities".to_string(), quantities);
    }
    Value::Object(fields)
}

pub struct MockIGenerateConsumer;

impl IGenerateConsumer for MockIGenerateConsumer {
    type Output = ();

    fn consume_generate<G: IGenerateAdapter>(
        &self,
        _params: ConsumeGenerateParams,
        _payload: ConsumeGeneratePayload<G>,
    ) -> Self::Output {
    }
}

#[derive(Default)]
pub struct ConsumeGenerateParamsOverrides;

pub fn build_consume_generate_params(
    _overrides: ConsumeGenerateParamsOverrides,
) -> ConsumeGenerateParams {
    ConsumeGenerateParams
}

#[derive(Default)]
pub struct ConsumeGeneratePayloadOverrides;

pub fn build_consume_generate_payload<G: IGenerateAdapter>(
    adapter: G,
    _overrides: ConsumeGeneratePayloadOverrides,
) -> ConsumeGeneratePayload<G> {
    ConsumeGeneratePayload { adapter }
}

#[derive(Default)]
pub struct CreateGenerateDepsOverrides;

pub fn build_create_generate_deps<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    C,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    consumer: C,
    _overrides: CreateGenerateDepsOverrides,
) -> CreateGenerateDeps<'a, P, E, K, A, D, C> {
    CreateGenerateDeps {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        delivery_proof,
        consumer,
    }
}

#[derive(Default)]
pub struct CreateGenerateParamsOverrides {
    pub concrete: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
    pub scope: Option<IdentityScope>,
    pub statement_version: Option<u16>,
}

pub fn build_create_generate_params(
    overrides: CreateGenerateParamsOverrides,
) -> CreateGenerateParams {
    CreateGenerateParams {
        concrete: overrides.concrete.unwrap_or(GenerateConcrete::Mock(
            MockIGenerateAdapterFailureMode::NoFailure,
        )),
        kinds: overrides
            .kinds
            .unwrap_or_else(|| kind_configurations(MOCK_I_GENERATE_ADAPTER_DECLARATION.kinds)),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct CreateGenerateParamsCorruptions {
    pub concrete: Option<Value>,
    pub kinds: Option<Value>,
    pub scope: Option<Value>,
    pub statement_version: Option<Value>,
}

pub fn invalidate_create_generate_params(corruptions: CreateGenerateParamsCorruptions) -> Value {
    let params = build_create_generate_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        match &params.concrete {
            GenerateConcrete::Evm { .. } => {
                let mut variant = Map::new();
                variant.insert(
                    "settings".to_string(),
                    invalidate_evm_generate_settings(Default::default()),
                );
                let mut concrete = Map::new();
                concrete.insert("Evm".to_string(), Value::Object(variant));
                Value::Object(concrete)
            }
            GenerateConcrete::Mock(failure_mode) => {
                let mut concrete = Map::new();
                concrete.insert(
                    "Mock".to_string(),
                    Value::from(match failure_mode {
                        MockIGenerateAdapterFailureMode::NoFailure => "NoFailure",
                        MockIGenerateAdapterFailureMode::WriteRefused => "WriteRefused",
                    }),
                );
                Value::Object(concrete)
            }
        },
    );
    fields.insert(
        "kinds".to_string(),
        Value::Array(
            params
                .kinds
                .iter()
                .map(|_| invalidate_vector_kind_configuration(Default::default()))
                .collect(),
        ),
    );
    fields.insert(
        "scope".to_string(),
        Value::from(match params.scope {
            IdentityScope::Entitlement => "Entitlement",
            IdentityScope::Asset => "Asset",
        }),
    );
    fields.insert(
        "statement_version".to_string(),
        Value::from(params.statement_version),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(kinds) = corruptions.kinds {
        fields.insert("kinds".to_string(), kinds);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CreateGeneratePayloadOverrides;

pub fn build_create_generate_payload(
    _overrides: CreateGeneratePayloadOverrides,
) -> CreateGeneratePayload {
    CreateGeneratePayload
}

#[derive(Default)]
pub struct CreateGenerateSuccessReturnOverrides;

pub fn build_create_generate_success_return<O>(
    output: O,
    _overrides: CreateGenerateSuccessReturnOverrides,
) -> CreateGenerateSuccessReturn<O> {
    CreateGenerateSuccessReturn { output }
}

pub fn mock_create_generate<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    C: IGenerateConsumer,
>(
    deps: &CreateGenerateDeps<'a, P, E, K, A, D, C>,
    _params: CreateGenerateParams,
    _payload: CreateGeneratePayload,
) -> CreateGenerateReturn<C::Output> {
    let _: CreateGenerateFn<'a, P, E, K, A, D, C> = mock_create_generate;
    let adapter = build_mock_i_generate_adapter(Default::default());
    let output = deps.consumer.consume_generate(
        build_consume_generate_params(Default::default()),
        build_consume_generate_payload(adapter, Default::default()),
    );
    Ok(build_create_generate_success_return(
        output,
        Default::default(),
    ))
}
