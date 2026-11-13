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
use core::marker::PhantomData;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
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
    DELIVERY_STATEMENT_VERSION_ONE, IDeliveryProofAdapter, IDeliveryProofConsumer,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};
use std::path::PathBuf;

use super::super::factory::provides::{
    IGenerateAdapter, VectorKind, VectorKindSettings, VectorKindSize, VectorKindSizing,
    build_msm_term_count, build_vector_count,
};
use super::challenge_vectors::provides::{ChallengeVectorsFn, mock_challenge_vectors};
use super::constants::provides::{ConstantsFn, mock_constants};
use super::group_vectors::provides::{GroupVectorsFn, mock_group_vectors};
use super::interface::{
    ConsumeSolidityGenerateCaseParams, ConsumeSolidityGenerateCasePayload, GeneratedDirectory,
    GeneratedDirectoryConstructorParams, GeneratedFileName, GeneratedFileNameConstructorParams,
    ISolidityGenerateCaseConsumer, RunSolidityGenerateCaseFn, SOLIDITY_GENERATE_DECLARATION,
    SolidityGenerate, SolidityGenerateCaseDeps, SolidityGenerateCaseErrorReturn,
    SolidityGenerateCaseParams, SolidityGenerateCasePayload, SolidityGenerateCaseReturn,
    SolidityGenerateCaseSuccessReturn, SolidityGenerateConstructorParams, SolidityGenerateSettings,
    SolidityLibraryNames, SolidityVectorKindNames, SolidityVectorKindSettings,
};
use super::mapping_vectors::provides::{MappingVectorsFn, mock_mapping_vectors};
use super::possession_vectors::provides::{PossessionVectorsFn, mock_possession_vectors};
use super::proof_vectors::provides::{ProofVectorsFn, mock_proof_vectors};
use super::rejection_vectors::provides::{RejectionVectorsFn, mock_rejection_vectors};
use super::render::provides::{
    RenderFn, SolidityCompilerVersion, SolidityConstantName,
    SolidityConstantNameConstructorParamsOverrides, SolidityLibraryName,
    SolidityLibraryNameConstructorParamsOverrides, SolidityMemberName,
    SolidityMemberNameConstructorParamsOverrides, SolidityTypeName,
    SolidityTypeNameConstructorParamsOverrides, SpdxLicenseIdentifier,
    build_solidity_compiler_version, build_solidity_constant_name, build_solidity_library_name,
    build_solidity_member_name, build_solidity_type_name, build_spdx_license_identifier,
    mock_render,
};
use crate::delivery::provides::AdmittedKemIdentity;

#[derive(Default)]
pub struct GeneratedFileNameConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_generated_file_name_constructor_params(
    overrides: GeneratedFileNameConstructorParamsOverrides,
) -> GeneratedFileNameConstructorParams {
    GeneratedFileNameConstructorParams {
        text: overrides.text.unwrap_or_else(|| "vectors.txt".to_string()),
    }
}

#[derive(Default)]
pub struct GeneratedFileNameConstructorParamsCorruptions {
    pub text: Option<Value>,
}

pub fn invalidate_generated_file_name_constructor_params(
    corruptions: GeneratedFileNameConstructorParamsCorruptions,
) -> Value {
    let params = build_generated_file_name_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("text".to_string(), Value::from(params.text));
    if let Some(text) = corruptions.text {
        fields.insert("text".to_string(), text);
    }
    Value::Object(fields)
}

pub fn build_generated_file_name(
    overrides: GeneratedFileNameConstructorParamsOverrides,
) -> GeneratedFileName {
    GeneratedFileName::try_new(build_generated_file_name_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct GeneratedDirectoryConstructorParamsOverrides {
    pub segments: Option<Vec<GeneratedFileName>>,
}

pub fn build_generated_directory_constructor_params(
    overrides: GeneratedDirectoryConstructorParamsOverrides,
) -> GeneratedDirectoryConstructorParams {
    GeneratedDirectoryConstructorParams {
        segments: overrides.segments.unwrap_or_else(|| {
            vec![build_generated_file_name(
                GeneratedFileNameConstructorParamsOverrides {
                    text: Some("generated".to_string()),
                },
            )]
        }),
    }
}

#[derive(Default)]
pub struct GeneratedDirectoryConstructorParamsCorruptions {
    pub segments: Option<Value>,
}

pub fn invalidate_generated_directory_constructor_params(
    corruptions: GeneratedDirectoryConstructorParamsCorruptions,
) -> Value {
    let params = build_generated_directory_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "segments".to_string(),
        Value::Array(
            params
                .segments
                .iter()
                .map(|segment| Value::from(segment.as_str()))
                .collect(),
        ),
    );
    if let Some(segments) = corruptions.segments {
        fields.insert("segments".to_string(), segments);
    }
    Value::Object(fields)
}

pub fn build_generated_directory(
    overrides: GeneratedDirectoryConstructorParamsOverrides,
) -> GeneratedDirectory {
    GeneratedDirectory::try_new(build_generated_directory_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityVectorKindNamesOverrides {
    pub file_name: Option<GeneratedFileName>,
    pub struct_name: Option<SolidityTypeName>,
    pub decode_function_name: Option<SolidityMemberName>,
    pub path_constant_name: Option<SolidityConstantName>,
}

pub fn build_solidity_vector_kind_names(
    overrides: SolidityVectorKindNamesOverrides,
) -> SolidityVectorKindNames {
    SolidityVectorKindNames {
        file_name: overrides.file_name.unwrap_or_else(|| {
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("g1_add.vectors".to_string()),
            })
        }),
        struct_name: overrides.struct_name.unwrap_or_else(|| {
            build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
                text: Some("G1AddVector".to_string()),
            })
        }),
        decode_function_name: overrides.decode_function_name.unwrap_or_else(|| {
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("decodeG1Add".to_string()),
            })
        }),
        path_constant_name: overrides.path_constant_name.unwrap_or_else(|| {
            build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
                text: Some("G1_ADD_PATH".to_string()),
            })
        }),
    }
}

#[derive(Default)]
pub struct SolidityVectorKindNamesCorruptions {
    pub file_name: Option<Value>,
    pub struct_name: Option<Value>,
    pub decode_function_name: Option<Value>,
    pub path_constant_name: Option<Value>,
}

pub fn invalidate_solidity_vector_kind_names(
    corruptions: SolidityVectorKindNamesCorruptions,
) -> Value {
    let names = build_solidity_vector_kind_names(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "file_name".to_string(),
        Value::from(names.file_name.as_str()),
    );
    fields.insert(
        "struct_name".to_string(),
        Value::from(names.struct_name.as_str().to_owned()),
    );
    fields.insert(
        "decode_function_name".to_string(),
        Value::from(names.decode_function_name.as_str().to_owned()),
    );
    fields.insert(
        "path_constant_name".to_string(),
        Value::from(names.path_constant_name.as_str().to_owned()),
    );
    if let Some(file_name) = corruptions.file_name {
        fields.insert("file_name".to_string(), file_name);
    }
    if let Some(struct_name) = corruptions.struct_name {
        fields.insert("struct_name".to_string(), struct_name);
    }
    if let Some(decode_function_name) = corruptions.decode_function_name {
        fields.insert("decode_function_name".to_string(), decode_function_name);
    }
    if let Some(path_constant_name) = corruptions.path_constant_name {
        fields.insert("path_constant_name".to_string(), path_constant_name);
    }
    Value::Object(fields)
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

#[derive(Default)]
pub struct SolidityVectorKindSettingsOverrides {
    pub kind: Option<VectorKind>,
    pub names: Option<SolidityVectorKindNames>,
}

pub fn build_solidity_vector_kind_settings(
    overrides: SolidityVectorKindSettingsOverrides,
) -> SolidityVectorKindSettings {
    SolidityVectorKindSettings {
        kind: overrides.kind.unwrap_or(VectorKind::G1Addition),
        names: overrides
            .names
            .unwrap_or_else(|| build_solidity_vector_kind_names(Default::default())),
    }
}

#[derive(Default)]
pub struct SolidityVectorKindSettingsCorruptions {
    pub kind: Option<Value>,
    pub names: Option<Value>,
}

pub fn invalidate_solidity_vector_kind_settings(
    corruptions: SolidityVectorKindSettingsCorruptions,
) -> Value {
    let settings = build_solidity_vector_kind_settings(Default::default());
    let mut fields = Map::new();
    fields.insert("kind".to_string(), vector_kind_value(settings.kind));
    fields.insert(
        "names".to_string(),
        invalidate_solidity_vector_kind_names(Default::default()),
    );
    if let Some(kind) = corruptions.kind {
        fields.insert("kind".to_string(), kind);
    }
    if let Some(names) = corruptions.names {
        fields.insert("names".to_string(), names);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct SolidityLibraryNamesOverrides {
    pub file_name: Option<GeneratedFileName>,
    pub library_name: Option<SolidityLibraryName>,
}

pub fn build_solidity_library_names(
    overrides: SolidityLibraryNamesOverrides,
) -> SolidityLibraryNames {
    SolidityLibraryNames {
        file_name: overrides.file_name.unwrap_or_else(|| {
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("Constants.sol".to_string()),
            })
        }),
        library_name: overrides.library_name.unwrap_or_else(|| {
            build_solidity_library_name(SolidityLibraryNameConstructorParamsOverrides {
                text: Some("Constants".to_string()),
            })
        }),
    }
}

#[derive(Default)]
pub struct SolidityLibraryNamesCorruptions {
    pub file_name: Option<Value>,
    pub library_name: Option<Value>,
}

pub fn invalidate_solidity_library_names(corruptions: SolidityLibraryNamesCorruptions) -> Value {
    let names = build_solidity_library_names(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "file_name".to_string(),
        Value::from(names.file_name.as_str()),
    );
    fields.insert(
        "library_name".to_string(),
        Value::from(names.library_name.as_str().to_owned()),
    );
    if let Some(file_name) = corruptions.file_name {
        fields.insert("file_name".to_string(), file_name);
    }
    if let Some(library_name) = corruptions.library_name {
        fields.insert("library_name".to_string(), library_name);
    }
    Value::Object(fields)
}

fn vector_kind_stems(kind: VectorKind) -> (&'static str, &'static str) {
    match kind {
        VectorKind::G1Addition => ("g1_add", "G1Add"),
        VectorKind::G1Multiplication => ("g1_mul", "G1Mul"),
        VectorKind::PairingCheck => ("pairing_check", "PairingCheck"),
        VectorKind::G2Addition => ("g2_add", "G2Add"),
        VectorKind::G1MultiScalarMultiplication => ("g1_msm", "G1Msm"),
        VectorKind::G2MultiScalarMultiplication => ("g2_msm", "G2Msm"),
        VectorKind::DecodeRejection => ("decode_rejection", "DecodeRejection"),
        VectorKind::HashToScalar => ("hash_to_scalar", "HashToScalar"),
        VectorKind::IdentityMapping => ("identity_mapping", "IdentityMapping"),
        VectorKind::Possession => ("possession", "Possession"),
        VectorKind::Challenge => ("challenge", "Challenge"),
        VectorKind::MintProof => ("mint_proof", "MintProof"),
        VectorKind::TransferProof => ("transfer_proof", "TransferProof"),
        VectorKind::CellValues => ("cell_values", "CellValues"),
    }
}

fn build_kind_vector_kind_settings(kind: VectorKind) -> SolidityVectorKindSettings {
    let (stem, struct_stem) = vector_kind_stems(kind);
    build_solidity_vector_kind_settings(SolidityVectorKindSettingsOverrides {
        kind: Some(kind),
        names: Some(build_solidity_vector_kind_names(
            SolidityVectorKindNamesOverrides {
                file_name: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some(format!("{stem}.vectors")),
                    },
                )),
                struct_name: Some(build_solidity_type_name(
                    SolidityTypeNameConstructorParamsOverrides {
                        text: Some(format!("{struct_stem}Vector")),
                    },
                )),
                decode_function_name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some(format!("decode{struct_stem}")),
                    },
                )),
                path_constant_name: Some(build_solidity_constant_name(
                    SolidityConstantNameConstructorParamsOverrides {
                        text: Some(format!("{}_PATH", stem.to_uppercase())),
                    },
                )),
            },
        )),
    })
}

#[derive(Default)]
pub struct SolidityGenerateSettingsOverrides {
    pub kinds: Option<Vec<SolidityVectorKindSettings>>,
    pub constants_library: Option<SolidityLibraryNames>,
    pub records_library: Option<SolidityLibraryNames>,
    pub foundry_root: Option<PathBuf>,
    pub generated_directory: Option<GeneratedDirectory>,
    pub compiler_version: Option<SolidityCompilerVersion>,
    pub license: Option<SpdxLicenseIdentifier>,
}

pub fn build_solidity_generate_settings(
    overrides: SolidityGenerateSettingsOverrides,
) -> SolidityGenerateSettings {
    SolidityGenerateSettings {
        kinds: overrides.kinds.unwrap_or_else(|| {
            SOLIDITY_GENERATE_DECLARATION
                .kinds
                .iter()
                .map(|declaration| build_kind_vector_kind_settings(declaration.kind))
                .collect()
        }),
        constants_library: overrides
            .constants_library
            .unwrap_or_else(|| build_solidity_library_names(Default::default())),
        records_library: overrides.records_library.unwrap_or_else(|| {
            build_solidity_library_names(SolidityLibraryNamesOverrides {
                file_name: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("VectorRecords.sol".to_string()),
                    },
                )),
                library_name: Some(build_solidity_library_name(
                    SolidityLibraryNameConstructorParamsOverrides {
                        text: Some("VectorRecords".to_string()),
                    },
                )),
            })
        }),
        foundry_root: overrides.foundry_root.unwrap_or_else(|| {
            std::env::temp_dir().join(format!(
                "chaintorrent-generate-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ))
        }),
        generated_directory: overrides
            .generated_directory
            .unwrap_or_else(|| build_generated_directory(Default::default())),
        compiler_version: overrides
            .compiler_version
            .unwrap_or_else(|| build_solidity_compiler_version(Default::default())),
        license: overrides
            .license
            .unwrap_or_else(|| build_spdx_license_identifier(Default::default())),
    }
}

#[derive(Default)]
pub struct SolidityGenerateSettingsCorruptions {
    pub kinds: Option<Value>,
    pub constants_library: Option<Value>,
    pub records_library: Option<Value>,
    pub foundry_root: Option<Value>,
    pub generated_directory: Option<Value>,
    pub compiler_version: Option<Value>,
    pub license: Option<Value>,
}

pub fn invalidate_solidity_generate_settings(
    corruptions: SolidityGenerateSettingsCorruptions,
) -> Value {
    let settings = build_solidity_generate_settings(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kinds".to_string(),
        Value::Array(
            settings
                .kinds
                .iter()
                .map(|_| invalidate_solidity_vector_kind_settings(Default::default()))
                .collect(),
        ),
    );
    fields.insert(
        "constants_library".to_string(),
        invalidate_solidity_library_names(Default::default()),
    );
    fields.insert(
        "records_library".to_string(),
        invalidate_solidity_library_names(Default::default()),
    );
    fields.insert(
        "foundry_root".to_string(),
        Value::from(settings.foundry_root.display().to_string()),
    );
    fields.insert(
        "generated_directory".to_string(),
        Value::Array(
            settings
                .generated_directory
                .segments
                .iter()
                .map(|segment| Value::from(segment.as_str()))
                .collect(),
        ),
    );
    fields.insert(
        "compiler_version".to_string(),
        Value::from(format!(
            "{}.{}.{}",
            settings.compiler_version.major,
            settings.compiler_version.minor,
            settings.compiler_version.patch
        )),
    );
    fields.insert(
        "license".to_string(),
        Value::from(settings.license.as_str().to_owned()),
    );
    if let Some(kinds) = corruptions.kinds {
        fields.insert("kinds".to_string(), kinds);
    }
    if let Some(constants_library) = corruptions.constants_library {
        fields.insert("constants_library".to_string(), constants_library);
    }
    if let Some(records_library) = corruptions.records_library {
        fields.insert("records_library".to_string(), records_library);
    }
    if let Some(foundry_root) = corruptions.foundry_root {
        fields.insert("foundry_root".to_string(), foundry_root);
    }
    if let Some(generated_directory) = corruptions.generated_directory {
        fields.insert("generated_directory".to_string(), generated_directory);
    }
    if let Some(compiler_version) = corruptions.compiler_version {
        fields.insert("compiler_version".to_string(), compiler_version);
    }
    if let Some(license) = corruptions.license {
        fields.insert("license".to_string(), license);
    }
    Value::Object(fields)
}

pub struct SolidityGenerateConstructorParamsOverrides<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub statement_version: Option<u16>,
    pub settings: Option<SolidityGenerateSettings>,
    pub sizes: Option<Vec<VectorKindSettings>>,
    pub render: Option<RenderFn>,
    pub constants: Option<ConstantsFn<'a, P, E>>,
    pub group_vectors: Option<GroupVectorsFn<'a, P>>,
    pub rejection_vectors: Option<RejectionVectorsFn<'a, P>>,
    pub mapping_vectors: Option<MappingVectorsFn<'a, P, K>>,
    pub possession_vectors: Option<PossessionVectorsFn<'a, P, A>>,
    pub challenge_vectors:
        Option<ChallengeVectorsFn<'a, P, E, <D as IDeliveryProofAdapter>::Forms>>,
    pub proof_vectors: Option<ProofVectorsFn<'a, P, K, A, D>>,
}

impl<'a, P, E, K, A, D> Default for SolidityGenerateConstructorParamsOverrides<'a, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    fn default() -> Self {
        Self {
            statement_version: None,
            settings: None,
            sizes: None,
            render: None,
            constants: None,
            group_vectors: None,
            rejection_vectors: None,
            mapping_vectors: None,
            possession_vectors: None,
            challenge_vectors: None,
            proof_vectors: None,
        }
    }
}

pub(crate) fn build_solidity_generate_constructor_params<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    overrides: SolidityGenerateConstructorParamsOverrides<'a, P, E, K, A, D>,
) -> SolidityGenerateConstructorParams<'a, P, E, K, A, D> {
    SolidityGenerateConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        delivery_proof,
        render: overrides.render.unwrap_or(mock_render),
        constants: overrides.constants.unwrap_or(mock_constants),
        group_vectors: overrides.group_vectors.unwrap_or(mock_group_vectors),
        rejection_vectors: overrides
            .rejection_vectors
            .unwrap_or(mock_rejection_vectors),
        mapping_vectors: overrides.mapping_vectors.unwrap_or(mock_mapping_vectors),
        possession_vectors: overrides
            .possession_vectors
            .unwrap_or(mock_possession_vectors),
        challenge_vectors: overrides
            .challenge_vectors
            .unwrap_or(mock_challenge_vectors),
        proof_vectors: overrides.proof_vectors.unwrap_or(mock_proof_vectors),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
        settings: overrides
            .settings
            .unwrap_or_else(|| build_solidity_generate_settings(Default::default())),
        sizes: overrides.sizes.unwrap_or_else(|| {
            <SolidityGenerate<'a, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                .kinds
                .iter()
                .map(|declaration| VectorKindSettings {
                    kind: declaration.kind,
                    size: match declaration.sizing {
                        VectorKindSizing::Drawn => VectorKindSize::Drawn {
                            count: build_vector_count(Default::default()),
                        },
                        VectorKindSizing::DrawnTerms => VectorKindSize::DrawnTerms {
                            count: build_vector_count(Default::default()),
                            terms: build_msm_term_count(Default::default()),
                        },
                        VectorKindSizing::DrawnMessages => VectorKindSize::DrawnMessages {
                            count: build_vector_count(Default::default()),
                            message_length: 32,
                        },
                        VectorKindSizing::Enumerated => VectorKindSize::Enumerated,
                        VectorKindSizing::OverConfiguredInputs => {
                            VectorKindSize::OverConfiguredInputs
                        }
                    },
                })
                .collect()
        }),
    }
}

pub(crate) fn build_solidity_generate<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    overrides: SolidityGenerateConstructorParamsOverrides<'a, P, E, K, A, D>,
) -> SolidityGenerate<'a, P, E, K, A, D> {
    let Ok(generate) = SolidityGenerate::try_new(build_solidity_generate_constructor_params(
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        delivery_proof,
        overrides,
    ));
    generate
}

#[derive(Default)]
pub struct SolidityGenerateCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_solidity_generate_case_params(
    overrides: SolidityGenerateCaseParamsOverrides,
) -> SolidityGenerateCaseParams {
    SolidityGenerateCaseParams {
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
pub struct SolidityGenerateCaseDepsOverrides;

pub fn build_solidity_generate_case_deps<C: ISolidityGenerateCaseConsumer>(
    consumer: C,
    _overrides: SolidityGenerateCaseDepsOverrides,
) -> SolidityGenerateCaseDeps<C> {
    SolidityGenerateCaseDeps { consumer }
}

#[derive(Default)]
pub struct SolidityGenerateCasePayloadOverrides<'a> {
    pub marker: PhantomData<&'a ()>,
}

pub fn build_solidity_generate_case_payload<'a>(
    canonicals: &'a [&'a [u8]],
    roots: &'a [&'a Root],
    _overrides: SolidityGenerateCasePayloadOverrides<'a>,
) -> SolidityGenerateCasePayload<'a> {
    SolidityGenerateCasePayload { canonicals, roots }
}

#[derive(Default)]
pub struct SolidityGenerateCaseSuccessReturnOverrides;

pub fn build_solidity_generate_case_success_return<O>(
    output: O,
    _overrides: SolidityGenerateCaseSuccessReturnOverrides,
) -> SolidityGenerateCaseSuccessReturn<O> {
    SolidityGenerateCaseSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeSolidityGenerateCaseParamsOverrides;

pub fn build_consume_solidity_generate_case_params(
    _overrides: ConsumeSolidityGenerateCaseParamsOverrides,
) -> ConsumeSolidityGenerateCaseParams {
    ConsumeSolidityGenerateCaseParams
}

#[derive(Default)]
pub struct ConsumeSolidityGenerateCasePayloadOverrides<'a> {
    pub marker: PhantomData<&'a ()>,
}

pub fn build_consume_solidity_generate_case_payload<
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
    _overrides: ConsumeSolidityGenerateCasePayloadOverrides<'a>,
) -> ConsumeSolidityGenerateCasePayload<'a, P, E, K, A, D> {
    ConsumeSolidityGenerateCasePayload {
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

struct CaseAtDeliveryProof<'a, P: IPairingArithmetic, E, F, K, A, C: ISolidityGenerateCaseConsumer>
{
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
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
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(SolidityGenerateCaseSuccessReturn {
            output: self.consumer.consume_solidity_generate_case(
                build_consume_solidity_generate_case_params(Default::default()),
                build_consume_solidity_generate_case_payload(
                    self.pairing,
                    self.hash_to_scalar,
                    self.encoder,
                    self.decoder,
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

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, F, K, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
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
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

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
                    decoder: self.decoder,
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
            Err(error) => Err(SolidityGenerateCaseErrorReturn::DeliveryProof(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

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
                    decoder: self.decoder,
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
            Err(error) => Err(SolidityGenerateCaseErrorReturn::KeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    decoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

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
                    decoder: self.decoder,
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
            Err(error) => Err(SolidityGenerateCaseErrorReturn::Kem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: ISolidityGenerateCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: ISolidityGenerateCaseConsumer,
{
    type Output = SolidityGenerateCaseReturn<C::Output>;

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
                    decoder: &payload.adapter,
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
            Err(error) => Err(SolidityGenerateCaseErrorReturn::ChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: ISolidityGenerateCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    random_source: &'a CreateRandomSourceParams,
    consumer: &'a C,
    payload: &'a SolidityGenerateCasePayload<'a>,
}

impl<C: ISolidityGenerateCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = SolidityGenerateCaseReturn<C::Output>;

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
                return Err(SolidityGenerateCaseErrorReturn::HashToScalar(error));
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
                return Err(SolidityGenerateCaseErrorReturn::RandomSource(error));
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
            Err(error) => Err(SolidityGenerateCaseErrorReturn::Encoding(error)),
        }
    }
}

pub fn run_solidity_generate_case<C: ISolidityGenerateCaseConsumer>(
    deps: &SolidityGenerateCaseDeps<C>,
    params: SolidityGenerateCaseParams,
    payload: SolidityGenerateCasePayload<'_>,
) -> SolidityGenerateCaseReturn<C::Output> {
    let _: RunSolidityGenerateCaseFn<C> = run_solidity_generate_case;
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
        Err(error) => Err(SolidityGenerateCaseErrorReturn::Pairing(error)),
    }
}
