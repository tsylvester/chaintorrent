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
use core::marker::PhantomData;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, EncodingIdentifier,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams,
    IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::Root;
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, KemIdentity, SetupScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParams,
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryContext, IDeliveryProofAdapter, IDeliveryProofConsumer,
    MintPurpose, TransferPurpose, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};
use std::path::PathBuf;

use super::super::solidity::provides::{
    SOLIDITY_GENERATE_DECLARATION, build_solidity_generate_settings,
    invalidate_solidity_generate_settings,
};
use super::interface::{
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload, ConsumeGenerateParams,
    ConsumeGeneratePayload, CreateGenerateCaseDeps, CreateGenerateCaseErrorReturn,
    CreateGenerateCaseParams, CreateGenerateCasePayload, CreateGenerateCaseReturn,
    CreateGenerateCaseSuccessReturn, CreateGenerateDeps, CreateGenerateFn, CreateGenerateParams,
    CreateGeneratePayload, CreateGenerateReturn, CreateGenerateSuccessReturn, EmitErrorReturn,
    EmitParams, EmitPayload, EmitReturn, EmitSuccessReturn, EmittedFile,
    GENERATE_INTERFACE_VERSION, GenerateConcrete, GenerateConcreteKind, GenerateDeclaration,
    ICreateGenerateCaseConsumer, IGenerateAdapter, IGenerateConsumer,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode, MsmTermCount,
    MsmTermCountConstructorParams, RunCreateGenerateCaseFn, VectorCount,
    VectorCountConstructorParams, VectorKind, VectorKindConfiguration, VectorKindDeclaration,
    VectorKindQuantities, VectorKindSettings, VectorKindSize, VectorKindSizing,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};

#[derive(Default)]
pub struct VectorCountConstructorParamsOverrides {
    pub count: Option<u32>,
}

pub fn build_vector_count_constructor_params(
    overrides: VectorCountConstructorParamsOverrides,
) -> VectorCountConstructorParams {
    VectorCountConstructorParams {
        count: overrides.count.unwrap_or(2),
    }
}

pub fn build_vector_count(overrides: VectorCountConstructorParamsOverrides) -> VectorCount {
    VectorCount::try_new(build_vector_count_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct MsmTermCountConstructorParamsOverrides {
    pub count: Option<u32>,
}

pub fn build_msm_term_count_constructor_params(
    overrides: MsmTermCountConstructorParamsOverrides,
) -> MsmTermCountConstructorParams {
    MsmTermCountConstructorParams {
        count: overrides.count.unwrap_or(2),
    }
}

pub fn build_msm_term_count(overrides: MsmTermCountConstructorParamsOverrides) -> MsmTermCount {
    MsmTermCount::try_new(build_msm_term_count_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct VectorCountConstructorParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_vector_count_constructor_params(
    corruptions: VectorCountConstructorParamsCorruptions,
) -> Value {
    let params = build_vector_count_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MsmTermCountConstructorParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_msm_term_count_constructor_params(
    corruptions: MsmTermCountConstructorParamsCorruptions,
) -> Value {
    let params = build_msm_term_count_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
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

fn vector_kind_value(kind: VectorKind) -> Value {
    Value::from(match kind {
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
        VectorKind::CellValues => "CellValues",
    })
}

fn vector_kind_size_value(size: &VectorKindSize) -> Value {
    match size {
        VectorKindSize::Drawn { count } => {
            let mut variant = Map::new();
            variant.insert("count".to_string(), Value::from(count.get()));
            let mut fields = Map::new();
            fields.insert("Drawn".to_string(), Value::Object(variant));
            Value::Object(fields)
        }
        VectorKindSize::DrawnTerms { count, terms } => {
            let mut variant = Map::new();
            variant.insert("count".to_string(), Value::from(count.get()));
            variant.insert("terms".to_string(), Value::from(terms.get()));
            let mut fields = Map::new();
            fields.insert("DrawnTerms".to_string(), Value::Object(variant));
            Value::Object(fields)
        }
        VectorKindSize::DrawnMessages {
            count,
            message_length,
        } => {
            let mut variant = Map::new();
            variant.insert("count".to_string(), Value::from(count.get()));
            variant.insert("message_length".to_string(), Value::from(*message_length));
            let mut fields = Map::new();
            fields.insert("DrawnMessages".to_string(), Value::Object(variant));
            Value::Object(fields)
        }
        VectorKindSize::Enumerated => Value::from("Enumerated"),
        VectorKindSize::OverConfiguredInputs => Value::from("OverConfiguredInputs"),
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
    fields.insert("kind".to_string(), vector_kind_value(declaration.kind));
    fields.insert(
        "sizing".to_string(),
        Value::from(match declaration.sizing {
            VectorKindSizing::Drawn => "Drawn",
            VectorKindSizing::DrawnTerms => "DrawnTerms",
            VectorKindSizing::DrawnMessages => "DrawnMessages",
            VectorKindSizing::Enumerated => "Enumerated",
            VectorKindSizing::OverConfiguredInputs => "OverConfiguredInputs",
        }),
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
    pub encoding_identifier: Option<EncodingIdentifier>,
}

pub fn build_generate_declaration(overrides: GenerateDeclarationOverrides) -> GenerateDeclaration {
    GenerateDeclaration {
        kinds: overrides.kinds.unwrap_or(&[]),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(GENERATE_INTERFACE_VERSION),
        encoding_identifier: overrides
            .encoding_identifier
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
    }
}

#[derive(Default)]
pub struct GenerateDeclarationCorruptions {
    pub kinds: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
    pub encoding_identifier: Option<Value>,
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
    fields.insert(
        "encoding_identifier".to_string(),
        Value::from(match declaration.encoding_identifier {
            EncodingIdentifier::EthereumAbiV1 => "EthereumAbiV1",
            EncodingIdentifier::Mock => "Mock",
        }),
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
    if let Some(encoding_identifier) = corruptions.encoding_identifier {
        fields.insert("encoding_identifier".to_string(), encoding_identifier);
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

pub fn invalidate_vector_kind_settings(corruptions: VectorKindSettingsCorruptions) -> Value {
    let settings = build_vector_kind_settings(Default::default());
    let mut fields = Map::new();
    fields.insert("kind".to_string(), vector_kind_value(settings.kind));
    fields.insert("size".to_string(), vector_kind_size_value(&settings.size));
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
        match quantities.count {
            Some(count) => Value::from(count.get()),
            None => Value::Null,
        },
    );
    fields.insert(
        "terms".to_string(),
        match quantities.terms {
            Some(terms) => Value::from(terms.get()),
            None => Value::Null,
        },
    );
    fields.insert(
        "message_length".to_string(),
        match quantities.message_length {
            Some(message_length) => Value::from(message_length),
            None => Value::Null,
        },
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
        match file.kind {
            Some(kind) => vector_kind_value(kind),
            None => Value::Null,
        },
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
pub struct EmitPayloadOverrides<'a> {
    pub proof_identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_emit_payload<'a, F: IChainForms>(
    mint: &'a DeliveryContext<F, MintPurpose>,
    transfer: &'a DeliveryContext<F, TransferPurpose>,
    identities: &'a [AdmittedKemIdentity<'a>],
    overrides: EmitPayloadOverrides<'a>,
) -> EmitPayload<'a, F> {
    EmitPayload {
        mint,
        transfer,
        identities,
        proof_identity: overrides
            .proof_identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
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
    encoding_identifier: EncodingIdentifier::Mock,
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

pub(crate) struct MockIGenerateAdapter<F: IChainForms> {
    pub(super) failure_mode: MockIGenerateAdapterFailureMode,
    pub(super) forms: PhantomData<F>,
}

pub(crate) type MockIGenerateAdapterTryNewReturn<F> = Result<MockIGenerateAdapter<F>, Infallible>;

impl<F: IChainForms> MockIGenerateAdapter<F> {
    pub(crate) fn try_new(
        params: MockIGenerateAdapterConstructorParams,
    ) -> MockIGenerateAdapterTryNewReturn<F> {
        Ok(MockIGenerateAdapter {
            failure_mode: params.failure_mode,
            forms: PhantomData,
        })
    }
}

pub(crate) fn build_mock_i_generate_adapter<F: IChainForms>(
    overrides: MockIGenerateAdapterConstructorParamsOverrides,
) -> MockIGenerateAdapter<F> {
    let Ok(adapter) =
        MockIGenerateAdapter::try_new(build_mock_i_generate_adapter_constructor_params(overrides));
    adapter
}

impl<F: IChainForms> IGenerateAdapter for MockIGenerateAdapter<F> {
    type Forms = F;

    const DECLARATION: GenerateDeclaration = MOCK_I_GENERATE_ADAPTER_DECLARATION;

    fn emit(&self, _params: EmitParams, _payload: EmitPayload<'_, F>) -> EmitReturn {
        match self.failure_mode {
            MockIGenerateAdapterFailureMode::WriteRefused => Err(EmitErrorReturn::Mock(
                MockIGenerateAdapterEmitErrorReturn::WriteRefused,
            )),
            MockIGenerateAdapterFailureMode::NoFailure => {
                Ok(build_emit_success_return(EmitSuccessReturnOverrides {
                    files: Some(
                        Self::DECLARATION
                            .kinds
                            .iter()
                            .map(|declaration| EmittedFile {
                                kind: Some(declaration.kind),
                                path: PathBuf::from(format!("{:?}", declaration.kind)),
                            })
                            .collect(),
                    ),
                }))
            }
        }
    }
}

fn build_quantities_for_sizing(sizing: VectorKindSizing) -> VectorKindQuantities {
    match sizing {
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
    }
}

#[derive(Default)]
pub struct CreateGenerateParamsOverrides {
    pub concrete: Option<GenerateConcrete>,
    pub kinds: Option<Vec<VectorKindConfiguration>>,
    pub statement_version: Option<u16>,
}

pub fn build_create_generate_params(
    overrides: CreateGenerateParamsOverrides,
) -> CreateGenerateParams {
    CreateGenerateParams {
        concrete: overrides.concrete.unwrap_or(GenerateConcrete::Mock(
            MockIGenerateAdapterFailureMode::NoFailure,
        )),
        kinds: overrides.kinds.unwrap_or_else(|| {
            MOCK_I_GENERATE_ADAPTER_DECLARATION
                .kinds
                .iter()
                .map(|declaration| {
                    build_vector_kind_configuration(VectorKindConfigurationOverrides {
                        kind: Some(declaration.kind),
                        quantities: Some(build_quantities_for_sizing(declaration.sizing)),
                    })
                })
                .collect()
        }),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct CreateGenerateParamsCorruptions {
    pub concrete: Option<Value>,
    pub kinds: Option<Value>,
    pub statement_version: Option<Value>,
}

pub fn invalidate_create_generate_params(corruptions: CreateGenerateParamsCorruptions) -> Value {
    let params = build_create_generate_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        match &params.concrete {
            GenerateConcrete::Solidity { .. } => {
                let mut variant = Map::new();
                variant.insert(
                    "settings".to_string(),
                    invalidate_solidity_generate_settings(Default::default()),
                );
                let mut outer = Map::new();
                outer.insert("Solidity".to_string(), Value::Object(variant));
                Value::Object(outer)
            }
            GenerateConcrete::Mock(failure_mode) => {
                let mut outer = Map::new();
                outer.insert(
                    "Mock".to_string(),
                    Value::from(match failure_mode {
                        MockIGenerateAdapterFailureMode::NoFailure => "NoFailure",
                        MockIGenerateAdapterFailureMode::WriteRefused => "WriteRefused",
                    }),
                );
                Value::Object(outer)
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
        "statement_version".to_string(),
        Value::from(params.statement_version),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(kinds) = corruptions.kinds {
        fields.insert("kinds".to_string(), kinds);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
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
    fields.insert("kind".to_string(), vector_kind_value(configuration.kind));
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

#[derive(Default)]
pub struct GenerateConcreteOverrides;

pub fn build_generate_concrete(
    kind: GenerateConcreteKind,
    _overrides: GenerateConcreteOverrides,
) -> GenerateConcrete {
    match kind {
        GenerateConcreteKind::Solidity => GenerateConcrete::Solidity {
            settings: Box::new(build_solidity_generate_settings(Default::default())),
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
        GenerateConcreteKind::Solidity => SOLIDITY_GENERATE_DECLARATION
            .kinds
            .iter()
            .map(|declaration| {
                build_vector_kind_configuration(VectorKindConfigurationOverrides {
                    kind: Some(declaration.kind),
                    quantities: Some(build_quantities_for_sizing(declaration.sizing)),
                })
            })
            .collect(),
    }
}

#[derive(Default)]
pub struct CreateGenerateSuccessReturnOverrides;

pub fn build_create_generate_success_return<O>(
    output: O,
    _overrides: CreateGenerateSuccessReturnOverrides,
) -> CreateGenerateSuccessReturn<O> {
    CreateGenerateSuccessReturn { output }
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
pub struct CreateGeneratePayloadOverrides;

pub fn build_create_generate_payload(
    _overrides: CreateGeneratePayloadOverrides,
) -> CreateGeneratePayload {
    CreateGeneratePayload
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

pub struct MockIGenerateConsumer;

impl<F: IChainForms> IGenerateConsumer<F> for MockIGenerateConsumer {
    type Output = ();

    fn consume_generate<G: IGenerateAdapter<Forms = F>>(
        &self,
        _params: ConsumeGenerateParams,
        _payload: ConsumeGeneratePayload<G>,
    ) -> Self::Output {
    }
}

pub fn mock_create_generate<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    C: IGenerateConsumer<D::Forms>,
>(
    deps: &CreateGenerateDeps<'a, P, E, K, A, D, C>,
    _params: CreateGenerateParams,
    _payload: CreateGeneratePayload,
) -> CreateGenerateReturn<C::Output> {
    let _: CreateGenerateFn<'a, P, E, K, A, D, C> = mock_create_generate;
    let adapter = build_mock_i_generate_adapter::<D::Forms>(Default::default());
    let output = deps.consumer.consume_generate(
        build_consume_generate_params(Default::default()),
        build_consume_generate_payload(adapter, Default::default()),
    );
    Ok(build_create_generate_success_return(
        output,
        Default::default(),
    ))
}

#[derive(Default)]
pub struct CreateGenerateCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_create_generate_case_params(
    overrides: CreateGenerateCaseParamsOverrides,
) -> CreateGenerateCaseParams {
    CreateGenerateCaseParams {
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
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_create_key_agreement_params(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct CreateGenerateCaseDepsOverrides;

pub fn build_create_generate_case_deps<C: ICreateGenerateCaseConsumer>(
    consumer: C,
    _overrides: CreateGenerateCaseDepsOverrides,
) -> CreateGenerateCaseDeps<C> {
    CreateGenerateCaseDeps { consumer }
}

#[derive(Default)]
pub struct CreateGenerateCasePayloadOverrides<'a> {
    pub marker: PhantomData<&'a ()>,
}

pub fn build_create_generate_case_payload<'a>(
    canonicals: &'a [&'a [u8]],
    roots: &'a [&'a Root],
    _overrides: CreateGenerateCasePayloadOverrides<'a>,
) -> CreateGenerateCasePayload<'a> {
    CreateGenerateCasePayload { canonicals, roots }
}

#[derive(Default)]
pub struct CreateGenerateCaseSuccessReturnOverrides;

pub fn build_create_generate_case_success_return<O>(
    output: O,
    _overrides: CreateGenerateCaseSuccessReturnOverrides,
) -> CreateGenerateCaseSuccessReturn<O> {
    CreateGenerateCaseSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeCreateGenerateCaseParamsOverrides;

pub fn build_consume_create_generate_case_params(
    _overrides: ConsumeCreateGenerateCaseParamsOverrides,
) -> ConsumeCreateGenerateCaseParams {
    ConsumeCreateGenerateCaseParams
}

#[derive(Default)]
pub struct ConsumeCreateGenerateCasePayloadOverrides<'a> {
    pub marker: PhantomData<&'a ()>,
}

pub fn build_consume_create_generate_case_payload<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    identities: Vec<AdmittedKemIdentity<'a>>,
    _overrides: ConsumeCreateGenerateCasePayloadOverrides<'a>,
) -> ConsumeCreateGenerateCasePayload<'a, P, E, K, A, D> {
    ConsumeCreateGenerateCasePayload {
        pairing,
        hash_to_scalar,
        encoder,
        decoder,
        kem,
        key_agreement,
        delivery_proof,
        random,
        identities,
    }
}

struct CaseAtDeliveryProof<'a, P: IPairingArithmetic, E, F, K, A, C: ICreateGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    consumer: &'a C,
    identities: Vec<AdmittedKemIdentity<'a>>,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, A, C> IDeliveryProofConsumer<P, F>
    for CaseAtDeliveryProof<'a, P, E, F, K, A, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    C: ICreateGenerateCaseConsumer,
{
    type Output = CreateGenerateCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(CreateGenerateCaseSuccessReturn {
            output: self.consumer.consume_create_generate_case(
                build_consume_create_generate_case_params(Default::default()),
                build_consume_create_generate_case_payload(
                    self.pairing,
                    self.hash_to_scalar,
                    self.encoder,
                    self.encoder,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.random,
                    self.identities.clone(),
                    Default::default(),
                ),
            ),
        })
    }
}

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, F, K, C: ICreateGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    identities: Vec<AdmittedKemIdentity<'a>>,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: ICreateGenerateCaseConsumer,
{
    type Output = CreateGenerateCaseReturn<C::Output>;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.random,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    random: self.random,
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    consumer: self.consumer,
                    identities: self.identities.clone(),
                    marker: PhantomData::<F>,
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
            Err(error) => Err(CreateGenerateCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F, C: ICreateGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a CreateGenerateCasePayload<'a>,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    C: ICreateGenerateCaseConsumer,
{
    type Output = CreateGenerateCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let identities: Vec<AdmittedKemIdentity<'a>> = match payload.scope() {
            IdentityScope::Entitlement => self
                .payload
                .canonicals
                .iter()
                .map(|canonical| AdmittedKemIdentity {
                    setup_scope: SetupScope::Entitlement,
                    identity: KemIdentity::Entitlement { canonical },
                })
                .collect(),
            IdentityScope::Asset => self
                .payload
                .roots
                .iter()
                .map(|root| AdmittedKemIdentity {
                    setup_scope: SetupScope::Asset { root },
                    identity: KemIdentity::Asset { root },
                })
                .collect(),
        };
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.random,
                CaseAtKeyAgreement {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    random: self.random,
                    kem: &payload.adapter,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                    identities,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateGenerateCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: ICreateGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a CreateGenerateCasePayload<'a>,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: ICreateGenerateCaseConsumer,
{
    type Output = CreateGenerateCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hash_to_scalar,
                CaseAtKem {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    random: self.random,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                    payload: self.payload,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateGenerateCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: ICreateGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a CreateGenerateCasePayload<'a>,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: ICreateGenerateCaseConsumer,
{
    type Output = CreateGenerateCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: &payload.adapter,
                    random: self.random,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                    payload: self.payload,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateGenerateCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: ICreateGenerateCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    random_source: &'a CreateRandomSourceParams,
    consumer: &'a C,
    payload: &'a CreateGenerateCasePayload<'a>,
}

impl<C: ICreateGenerateCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = CreateGenerateCaseReturn<C::Output>;

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
                return Err(CreateGenerateCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(CreateGenerateCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hash_to_scalar: &*hashing.adapter,
                random: &*source.adapter,
                chain_forms: self.chain_forms,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
                consumer: self.consumer,
                payload: self.payload,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateGenerateCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_create_generate_case<C: ICreateGenerateCaseConsumer>(
    deps: &CreateGenerateCaseDeps<C>,
    params: CreateGenerateCaseParams,
    payload: CreateGenerateCasePayload<'_>,
) -> CreateGenerateCaseReturn<C::Output> {
    let _: RunCreateGenerateCaseFn<C> = run_create_generate_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                kem: &params.kem,
                key_agreement: &params.key_agreement,
                delivery_proof: &params.delivery_proof,
                random_source: &params.random_source,
                consumer: &deps.consumer,
                payload: &payload,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(CreateGenerateCaseErrorReturn::CreatePairing(error)),
    }
}
