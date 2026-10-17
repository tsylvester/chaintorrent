#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    
)]

use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DELIVERY_STATEMENT_VERSION_ONE, IDeliveryProofAdapter};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};
use std::env;
use std::path::PathBuf;
use std::process;
use std::thread;

use super::interface::{
    EVM_GENERATE_DECLARATION, EvmCurveDirectories, EvmGenerate, EvmGenerateConstructorParams,
    EvmGenerateSettings, EvmLibraryNames, EvmVectorKindNames, EvmVectorKindSettings,
    GeneratedDirectory, GeneratedDirectoryConstructorParams, GeneratedFileName,
    GeneratedFileNameConstructorParams,
};

use super::super::factory::provides::{
    IGenerateAdapter, VectorKind, VectorKindSettings, VectorKindSize, VectorKindSizing,
};

use super::group_vectors::provides::{
    MsmTermCountConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    build_msm_term_count, build_vector_count,
};

use super::render::provides::{
    SolidityCompilerVersion, SolidityConstantName, SolidityConstantNameConstructorParamsOverrides,
    SolidityLibraryName, SolidityLibraryNameConstructorParamsOverrides, SolidityMemberName,
    SolidityMemberNameConstructorParamsOverrides, SolidityTypeName,
    SolidityTypeNameConstructorParamsOverrides, SpdxLicenseIdentifier,
    build_solidity_compiler_version, build_solidity_constant_name, build_solidity_library_name,
    build_solidity_member_name, build_solidity_type_name, build_spdx_license_identifier,
    build_spdx_license_identifier_constructor_params,
};

const DEFAULT_STRUCT_NAME: &str = "G1AddVector";
const DEFAULT_DECODE_FUNCTION_NAME: &str = "decodeG1Add";
const DEFAULT_PATH_CONSTANT_NAME: &str = "G1_ADD_PATH";
const DEFAULT_LIBRARY_NAME: &str = "Constants";

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

fn vector_kind_names(
    file_name: &str,
    struct_name: &str,
    decode_function_name: &str,
    path_constant_name: &str,
) -> EvmVectorKindNames {
    EvmVectorKindNames {
        file_name: build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
            text: Some(file_name.to_string()),
        }),
        struct_name: build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
            text: Some(struct_name.to_string()),
        }),
        decode_function_name: build_solidity_member_name(
            SolidityMemberNameConstructorParamsOverrides {
                text: Some(decode_function_name.to_string()),
            },
        ),
        path_constant_name: build_solidity_constant_name(
            SolidityConstantNameConstructorParamsOverrides {
                text: Some(path_constant_name.to_string()),
            },
        ),
    }
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
    }
}

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
                .map(|segment| Value::from(segment.text.clone()))
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
pub struct EvmVectorKindNamesOverrides {
    pub file_name: Option<GeneratedFileName>,
    pub struct_name: Option<SolidityTypeName>,
    pub decode_function_name: Option<SolidityMemberName>,
    pub path_constant_name: Option<SolidityConstantName>,
}

pub fn build_evm_vector_kind_names(overrides: EvmVectorKindNamesOverrides) -> EvmVectorKindNames {
    EvmVectorKindNames {
        file_name: overrides.file_name.unwrap_or_else(|| {
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("g1_add.vectors".to_string()),
            })
        }),
        struct_name: overrides.struct_name.unwrap_or_else(|| {
            build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
                text: Some(DEFAULT_STRUCT_NAME.to_string()),
            })
        }),
        decode_function_name: overrides.decode_function_name.unwrap_or_else(|| {
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some(DEFAULT_DECODE_FUNCTION_NAME.to_string()),
            })
        }),
        path_constant_name: overrides.path_constant_name.unwrap_or_else(|| {
            build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
                text: Some(DEFAULT_PATH_CONSTANT_NAME.to_string()),
            })
        }),
    }
}

#[derive(Default)]
pub struct EvmVectorKindNamesCorruptions {
    pub file_name: Option<Value>,
    pub struct_name: Option<Value>,
    pub decode_function_name: Option<Value>,
    pub path_constant_name: Option<Value>,
}

pub fn invalidate_evm_vector_kind_names(corruptions: EvmVectorKindNamesCorruptions) -> Value {
    let names = build_evm_vector_kind_names(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "file_name".to_string(),
        Value::from(names.file_name.text.clone()),
    );
    fields.insert("struct_name".to_string(), Value::from(DEFAULT_STRUCT_NAME));
    fields.insert(
        "decode_function_name".to_string(),
        Value::from(DEFAULT_DECODE_FUNCTION_NAME),
    );
    fields.insert(
        "path_constant_name".to_string(),
        Value::from(DEFAULT_PATH_CONSTANT_NAME),
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

#[derive(Default)]
pub struct EvmVectorKindSettingsOverrides {
    pub kind: Option<VectorKind>,
    pub names: Option<EvmVectorKindNames>,
}

pub fn build_evm_vector_kind_settings(
    overrides: EvmVectorKindSettingsOverrides,
) -> EvmVectorKindSettings {
    EvmVectorKindSettings {
        kind: overrides.kind.unwrap_or(VectorKind::G1Addition),
        names: overrides
            .names
            .unwrap_or_else(|| build_evm_vector_kind_names(Default::default())),
    }
}

#[derive(Default)]
pub struct EvmVectorKindSettingsCorruptions {
    pub kind: Option<Value>,
    pub names: Option<Value>,
}

pub fn invalidate_evm_vector_kind_settings(corruptions: EvmVectorKindSettingsCorruptions) -> Value {
    let settings = build_evm_vector_kind_settings(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kind".to_string(),
        Value::from(vector_kind_name(settings.kind)),
    );
    fields.insert(
        "names".to_string(),
        invalidate_evm_vector_kind_names(Default::default()),
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
pub struct EvmLibraryNamesOverrides {
    pub file_name: Option<GeneratedFileName>,
    pub library_name: Option<SolidityLibraryName>,
}

pub fn build_evm_library_names(overrides: EvmLibraryNamesOverrides) -> EvmLibraryNames {
    EvmLibraryNames {
        file_name: overrides.file_name.unwrap_or_else(|| {
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("Constants.sol".to_string()),
            })
        }),
        library_name: overrides.library_name.unwrap_or_else(|| {
            build_solidity_library_name(SolidityLibraryNameConstructorParamsOverrides {
                text: Some(DEFAULT_LIBRARY_NAME.to_string()),
            })
        }),
    }
}

#[derive(Default)]
pub struct EvmLibraryNamesCorruptions {
    pub file_name: Option<Value>,
    pub library_name: Option<Value>,
}

pub fn invalidate_evm_library_names(corruptions: EvmLibraryNamesCorruptions) -> Value {
    let names = build_evm_library_names(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "file_name".to_string(),
        Value::from(names.file_name.text.clone()),
    );
    fields.insert(
        "library_name".to_string(),
        Value::from(DEFAULT_LIBRARY_NAME),
    );
    if let Some(file_name) = corruptions.file_name {
        fields.insert("file_name".to_string(), file_name);
    }
    if let Some(library_name) = corruptions.library_name {
        fields.insert("library_name".to_string(), library_name);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EvmCurveDirectoriesOverrides {
    pub bn254: Option<GeneratedFileName>,
    pub bls12_381: Option<GeneratedFileName>,
}

pub fn build_evm_curve_directories(overrides: EvmCurveDirectoriesOverrides) -> EvmCurveDirectories {
    EvmCurveDirectories {
        bn254: overrides.bn254.unwrap_or_else(|| {
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("bn254".to_string()),
            })
        }),
        bls12_381: overrides.bls12_381.unwrap_or_else(|| {
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("bls12_381".to_string()),
            })
        }),
    }
}

#[derive(Default)]
pub struct EvmCurveDirectoriesCorruptions {
    pub bn254: Option<Value>,
    pub bls12_381: Option<Value>,
}

pub fn invalidate_evm_curve_directories(corruptions: EvmCurveDirectoriesCorruptions) -> Value {
    let directories = build_evm_curve_directories(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "bn254".to_string(),
        Value::from(directories.bn254.text.clone()),
    );
    fields.insert(
        "bls12_381".to_string(),
        Value::from(directories.bls12_381.text.clone()),
    );
    if let Some(bn254) = corruptions.bn254 {
        fields.insert("bn254".to_string(), bn254);
    }
    if let Some(bls12_381) = corruptions.bls12_381 {
        fields.insert("bls12_381".to_string(), bls12_381);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EvmGenerateSettingsOverrides {
    pub kinds: Option<Vec<EvmVectorKindSettings>>,
    pub constants_library: Option<EvmLibraryNames>,
    pub records_library: Option<EvmLibraryNames>,
    pub foundry_root: Option<PathBuf>,
    pub generated_directory: Option<GeneratedDirectory>,
    pub curve_directories: Option<EvmCurveDirectories>,
    pub compiler_version: Option<SolidityCompilerVersion>,
    pub license: Option<SpdxLicenseIdentifier>,
}

pub fn build_evm_generate_settings(overrides: EvmGenerateSettingsOverrides) -> EvmGenerateSettings {
    EvmGenerateSettings {
        kinds: overrides.kinds.unwrap_or_else(|| {
            EVM_GENERATE_DECLARATION
                .kinds
                .iter()
                .map(|declaration| {
                    let (stem, stem_name) = vector_kind_stems(declaration.kind);
                    build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                        kind: Some(declaration.kind),
                        names: Some(vector_kind_names(
                            &format!("{stem}.vectors"),
                            &format!("{stem_name}Vector"),
                            &format!("decode{stem_name}"),
                            &format!("{}_PATH", stem.to_uppercase()),
                        )),
                    })
                })
                .collect()
        }),
        constants_library: overrides
            .constants_library
            .unwrap_or_else(|| build_evm_library_names(Default::default())),
        records_library: overrides.records_library.unwrap_or_else(|| {
            build_evm_library_names(EvmLibraryNamesOverrides {
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
            env::temp_dir().join(format!(
                "chaintorrent-generate-{}-{:?}",
                process::id(),
                thread::current().id()
            ))
        }),
        generated_directory: overrides
            .generated_directory
            .unwrap_or_else(|| build_generated_directory(Default::default())),
        curve_directories: overrides
            .curve_directories
            .unwrap_or_else(|| build_evm_curve_directories(Default::default())),
        compiler_version: overrides
            .compiler_version
            .unwrap_or_else(|| build_solidity_compiler_version(Default::default())),
        license: overrides
            .license
            .unwrap_or_else(|| build_spdx_license_identifier(Default::default())),
    }
}

#[derive(Default)]
pub struct EvmGenerateSettingsCorruptions {
    pub kinds: Option<Value>,
    pub constants_library: Option<Value>,
    pub records_library: Option<Value>,
    pub foundry_root: Option<Value>,
    pub generated_directory: Option<Value>,
    pub curve_directories: Option<Value>,
    pub compiler_version: Option<Value>,
    pub license: Option<Value>,
}

pub fn invalidate_evm_generate_settings(corruptions: EvmGenerateSettingsCorruptions) -> Value {
    let settings = build_evm_generate_settings(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "kinds".to_string(),
        Value::Array(
            settings
                .kinds
                .iter()
                .map(|_| invalidate_evm_vector_kind_settings(Default::default()))
                .collect(),
        ),
    );
    fields.insert(
        "constants_library".to_string(),
        invalidate_evm_library_names(Default::default()),
    );
    fields.insert(
        "records_library".to_string(),
        invalidate_evm_library_names(Default::default()),
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
                .map(|segment| Value::from(segment.text.clone()))
                .collect(),
        ),
    );
    fields.insert(
        "curve_directories".to_string(),
        invalidate_evm_curve_directories(Default::default()),
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
        Value::from(build_spdx_license_identifier_constructor_params(Default::default()).text),
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
    if let Some(curve_directories) = corruptions.curve_directories {
        fields.insert("curve_directories".to_string(), curve_directories);
    }
    if let Some(compiler_version) = corruptions.compiler_version {
        fields.insert("compiler_version".to_string(), compiler_version);
    }
    if let Some(license) = corruptions.license {
        fields.insert("license".to_string(), license);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EvmGenerateConstructorParamsOverrides {
    pub scope: Option<IdentityScope>,
    pub statement_version: Option<u16>,
    pub settings: Option<EvmGenerateSettings>,
    pub sizes: Option<Vec<VectorKindSettings>>,
}

pub(crate) fn build_evm_generate_constructor_params<'a, P, E, K, A, D>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    overrides: EvmGenerateConstructorParamsOverrides,
) -> EvmGenerateConstructorParams<'a, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    EvmGenerateConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        delivery_proof,
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
        settings: overrides
            .settings
            .unwrap_or_else(|| build_evm_generate_settings(Default::default())),
        sizes: overrides.sizes.unwrap_or_else(|| {
            <EvmGenerate<'a, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                .kinds
                .iter()
                .map(|declaration| VectorKindSettings {
                    kind: declaration.kind,
                    size: match declaration.sizing {
                        VectorKindSizing::Drawn => VectorKindSize::Drawn {
                            count: build_vector_count(
                                VectorCountConstructorParamsOverrides::default(),
                            ),
                        },
                        VectorKindSizing::DrawnTerms => VectorKindSize::DrawnTerms {
                            count: build_vector_count(
                                VectorCountConstructorParamsOverrides::default(),
                            ),
                            terms: build_msm_term_count(
                                MsmTermCountConstructorParamsOverrides::default(),
                            ),
                        },
                        VectorKindSizing::DrawnMessages => VectorKindSize::DrawnMessages {
                            count: build_vector_count(
                                VectorCountConstructorParamsOverrides::default(),
                            ),
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

pub(crate) fn build_evm_generate<'a, P, E, K, A, D>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    overrides: EvmGenerateConstructorParamsOverrides,
) -> EvmGenerate<'a, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    let Ok(generate) = EvmGenerate::try_new(build_evm_generate_constructor_params(
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
