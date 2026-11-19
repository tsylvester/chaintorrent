#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{Map, Value};

use super::super::super::factory::provides::{GenerateDeclaration, VectorKind};
use super::super::render::provides::{
    SolidityCompilerVersion, SolidityConstantName, SolidityLibraryName, SolidityMemberName,
    SolidityTypeName, SpdxLicenseIdentifier, build_solidity_constant_name,
    build_solidity_library_name, build_solidity_member_name, build_solidity_type_name,
};
use super::admit_solidity_generate_settings;
use super::interface::{
    AdmitSolidityGenerateSettingsDeps, AdmitSolidityGenerateSettingsFn,
    AdmitSolidityGenerateSettingsParams, AdmitSolidityGenerateSettingsPayload,
    AdmitSolidityGenerateSettingsReturn, AdmitSolidityGenerateSettingsSuccessReturn,
    GeneratedDirectory, GeneratedDirectoryConstructorParams, GeneratedFileName,
    GeneratedFileNameConstructorParams, SOLIDITY_GENERATE_DECLARATION, SolidityGenerateSettings,
    SolidityGenerateSettingsDocument, SolidityLibraryNames, SolidityLibraryNamesDocument,
    SolidityVectorKindNames, SolidityVectorKindNamesDocument, SolidityVectorKindSettings,
};

#[derive(Default)]
pub struct GeneratedFileNameConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_generated_file_name_constructor_params(
    overrides: GeneratedFileNameConstructorParamsOverrides,
) -> GeneratedFileNameConstructorParams {
    GeneratedFileNameConstructorParams {
        text: overrides
            .text
            .unwrap_or_else(|| "generated.txt".to_string()),
    }
}

pub fn build_generated_file_name(
    overrides: GeneratedFileNameConstructorParamsOverrides,
) -> GeneratedFileName {
    GeneratedFileName::try_new(build_generated_file_name_constructor_params(overrides))
        .expect("built params are valid")
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

#[derive(Default)]
pub struct GeneratedDirectoryConstructorParamsOverrides {
    pub segments: Option<Vec<GeneratedFileName>>,
}

pub fn build_generated_directory_constructor_params(
    overrides: GeneratedDirectoryConstructorParamsOverrides,
) -> GeneratedDirectoryConstructorParams {
    GeneratedDirectoryConstructorParams {
        segments: overrides
            .segments
            .unwrap_or_else(|| vec![build_generated_file_name(Default::default())]),
    }
}

pub fn build_generated_directory(
    overrides: GeneratedDirectoryConstructorParamsOverrides,
) -> GeneratedDirectory {
    GeneratedDirectory::try_new(build_generated_directory_constructor_params(overrides))
        .expect("built params are valid")
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
        file_name: overrides
            .file_name
            .unwrap_or_else(|| build_generated_file_name(Default::default())),
        struct_name: overrides
            .struct_name
            .unwrap_or_else(|| build_solidity_type_name(Default::default())),
        decode_function_name: overrides
            .decode_function_name
            .unwrap_or_else(|| build_solidity_member_name(Default::default())),
        path_constant_name: overrides
            .path_constant_name
            .unwrap_or_else(|| build_solidity_constant_name(Default::default())),
    }
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
pub struct SolidityLibraryNamesOverrides {
    pub file_name: Option<GeneratedFileName>,
    pub library_name: Option<SolidityLibraryName>,
}

pub fn build_solidity_library_names(
    overrides: SolidityLibraryNamesOverrides,
) -> SolidityLibraryNames {
    SolidityLibraryNames {
        file_name: overrides
            .file_name
            .unwrap_or_else(|| build_generated_file_name(Default::default())),
        library_name: overrides
            .library_name
            .unwrap_or_else(|| build_solidity_library_name(Default::default())),
    }
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
    let admitted = admit_solidity_generate_settings(
        &build_admit_solidity_generate_settings_deps(
            &SOLIDITY_GENERATE_DECLARATION,
            Default::default(),
        ),
        build_admit_solidity_generate_settings_params(Default::default()),
        build_admit_solidity_generate_settings_payload(Default::default()),
    )
    .expect("the declared defaults are admitted")
    .settings;
    SolidityGenerateSettings {
        kinds: overrides.kinds.unwrap_or(admitted.kinds),
        constants_library: overrides
            .constants_library
            .unwrap_or(admitted.constants_library),
        records_library: overrides
            .records_library
            .unwrap_or(admitted.records_library),
        foundry_root: overrides.foundry_root.unwrap_or(admitted.foundry_root),
        generated_directory: overrides
            .generated_directory
            .unwrap_or(admitted.generated_directory),
        compiler_version: overrides
            .compiler_version
            .unwrap_or(admitted.compiler_version),
        license: overrides.license.unwrap_or(admitted.license),
    }
}

#[derive(Default)]
pub struct SolidityVectorKindNamesDocumentOverrides {
    pub file: Option<Option<String>>,
    pub struct_name: Option<Option<String>>,
    pub decode_function: Option<Option<String>>,
    pub path_constant: Option<Option<String>>,
}

pub fn build_solidity_vector_kind_names_document(
    overrides: SolidityVectorKindNamesDocumentOverrides,
) -> SolidityVectorKindNamesDocument {
    SolidityVectorKindNamesDocument {
        file: overrides.file.unwrap_or_default(),
        struct_name: overrides.struct_name.unwrap_or_default(),
        decode_function: overrides.decode_function.unwrap_or_default(),
        path_constant: overrides.path_constant.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct SolidityLibraryNamesDocumentOverrides {
    pub file: Option<Option<String>>,
    pub library: Option<Option<String>>,
}

pub fn build_solidity_library_names_document(
    overrides: SolidityLibraryNamesDocumentOverrides,
) -> SolidityLibraryNamesDocument {
    SolidityLibraryNamesDocument {
        file: overrides.file.unwrap_or_default(),
        library: overrides.library.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct SolidityGenerateSettingsDocumentOverrides {
    pub kinds: Option<Option<BTreeMap<String, SolidityVectorKindNamesDocument>>>,
    pub constants_library: Option<Option<SolidityLibraryNamesDocument>>,
    pub records_library: Option<Option<SolidityLibraryNamesDocument>>,
    pub foundry_root: Option<Option<String>>,
    pub generated_directory: Option<Option<Vec<String>>>,
    pub compiler_version: Option<Option<String>>,
    pub license: Option<Option<String>>,
}

pub fn build_solidity_generate_settings_document(
    overrides: SolidityGenerateSettingsDocumentOverrides,
) -> SolidityGenerateSettingsDocument {
    SolidityGenerateSettingsDocument {
        kinds: overrides.kinds.unwrap_or_default(),
        constants_library: overrides.constants_library.unwrap_or_default(),
        records_library: overrides.records_library.unwrap_or_default(),
        foundry_root: overrides.foundry_root.unwrap_or_default(),
        generated_directory: overrides.generated_directory.unwrap_or_default(),
        compiler_version: overrides.compiler_version.unwrap_or_default(),
        license: overrides.license.unwrap_or_default(),
    }
}

fn render_kind_names_document(document: &SolidityVectorKindNamesDocument) -> Map<String, Value> {
    let mut fields = Map::new();
    if let Some(file) = &document.file {
        fields.insert("file".to_string(), Value::from(file.clone()));
    }
    if let Some(struct_name) = &document.struct_name {
        fields.insert("struct_name".to_string(), Value::from(struct_name.clone()));
    }
    if let Some(decode_function) = &document.decode_function {
        fields.insert(
            "decode_function".to_string(),
            Value::from(decode_function.clone()),
        );
    }
    if let Some(path_constant) = &document.path_constant {
        fields.insert(
            "path_constant".to_string(),
            Value::from(path_constant.clone()),
        );
    }
    fields
}

fn render_library_document(document: &SolidityLibraryNamesDocument) -> Map<String, Value> {
    let mut fields = Map::new();
    if let Some(file) = &document.file {
        fields.insert("file".to_string(), Value::from(file.clone()));
    }
    if let Some(library) = &document.library {
        fields.insert("library".to_string(), Value::from(library.clone()));
    }
    fields
}

#[derive(Default)]
pub struct SolidityVectorKindNamesDocumentCorruptions {
    pub file: Option<Value>,
    pub struct_name: Option<Value>,
    pub decode_function: Option<Value>,
    pub path_constant: Option<Value>,
}

pub fn invalidate_solidity_vector_kind_names_document(
    corruptions: SolidityVectorKindNamesDocumentCorruptions,
) -> Value {
    let document = build_solidity_vector_kind_names_document(Default::default());
    let mut fields = render_kind_names_document(&document);
    if let Some(value) = corruptions.file {
        fields.insert("file".to_string(), value);
    }
    if let Some(value) = corruptions.struct_name {
        fields.insert("struct_name".to_string(), value);
    }
    if let Some(value) = corruptions.decode_function {
        fields.insert("decode_function".to_string(), value);
    }
    if let Some(value) = corruptions.path_constant {
        fields.insert("path_constant".to_string(), value);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct SolidityLibraryNamesDocumentCorruptions {
    pub file: Option<Value>,
    pub library: Option<Value>,
}

pub fn invalidate_solidity_library_names_document(
    corruptions: SolidityLibraryNamesDocumentCorruptions,
) -> Value {
    let document = build_solidity_library_names_document(Default::default());
    let mut fields = render_library_document(&document);
    if let Some(value) = corruptions.file {
        fields.insert("file".to_string(), value);
    }
    if let Some(value) = corruptions.library {
        fields.insert("library".to_string(), value);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct SolidityGenerateSettingsDocumentCorruptions {
    pub kinds: Option<Value>,
    pub constants_library: Option<Value>,
    pub records_library: Option<Value>,
    pub foundry_root: Option<Value>,
    pub generated_directory: Option<Value>,
    pub compiler_version: Option<Value>,
    pub license: Option<Value>,
}

pub fn invalidate_solidity_generate_settings_document(
    corruptions: SolidityGenerateSettingsDocumentCorruptions,
) -> Value {
    let document = build_solidity_generate_settings_document(Default::default());
    let mut fields = Map::new();
    if let Some(kinds) = &document.kinds {
        fields.insert(
            "kinds".to_string(),
            Value::Object(
                kinds
                    .iter()
                    .map(|(key, names)| {
                        (
                            key.clone(),
                            Value::Object(render_kind_names_document(names)),
                        )
                    })
                    .collect(),
            ),
        );
    }
    if let Some(constants_library) = &document.constants_library {
        fields.insert(
            "constants_library".to_string(),
            Value::Object(render_library_document(constants_library)),
        );
    }
    if let Some(records_library) = &document.records_library {
        fields.insert(
            "records_library".to_string(),
            Value::Object(render_library_document(records_library)),
        );
    }
    if let Some(foundry_root) = &document.foundry_root {
        fields.insert(
            "foundry_root".to_string(),
            Value::from(foundry_root.clone()),
        );
    }
    if let Some(generated_directory) = &document.generated_directory {
        fields.insert(
            "generated_directory".to_string(),
            Value::Array(
                generated_directory
                    .iter()
                    .map(|segment| Value::from(segment.clone()))
                    .collect(),
            ),
        );
    }
    if let Some(compiler_version) = &document.compiler_version {
        fields.insert(
            "compiler_version".to_string(),
            Value::from(compiler_version.clone()),
        );
    }
    if let Some(license) = &document.license {
        fields.insert("license".to_string(), Value::from(license.clone()));
    }
    if let Some(value) = corruptions.kinds {
        fields.insert("kinds".to_string(), value);
    }
    if let Some(value) = corruptions.constants_library {
        fields.insert("constants_library".to_string(), value);
    }
    if let Some(value) = corruptions.records_library {
        fields.insert("records_library".to_string(), value);
    }
    if let Some(value) = corruptions.foundry_root {
        fields.insert("foundry_root".to_string(), value);
    }
    if let Some(value) = corruptions.generated_directory {
        fields.insert("generated_directory".to_string(), value);
    }
    if let Some(value) = corruptions.compiler_version {
        fields.insert("compiler_version".to_string(), value);
    }
    if let Some(value) = corruptions.license {
        fields.insert("license".to_string(), value);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct AdmitSolidityGenerateSettingsPayloadOverrides {
    pub document: Option<SolidityGenerateSettingsDocument>,
}

pub fn build_admit_solidity_generate_settings_payload(
    overrides: AdmitSolidityGenerateSettingsPayloadOverrides,
) -> AdmitSolidityGenerateSettingsPayload {
    AdmitSolidityGenerateSettingsPayload {
        document: overrides
            .document
            .unwrap_or_else(|| build_solidity_generate_settings_document(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmitSolidityGenerateSettingsParamsOverrides;

pub fn build_admit_solidity_generate_settings_params(
    _overrides: AdmitSolidityGenerateSettingsParamsOverrides,
) -> AdmitSolidityGenerateSettingsParams {
    AdmitSolidityGenerateSettingsParams
}

#[derive(Default)]
pub struct AdmitSolidityGenerateSettingsSuccessReturnOverrides {
    pub settings: Option<SolidityGenerateSettings>,
}

pub fn build_admit_solidity_generate_settings_success_return(
    overrides: AdmitSolidityGenerateSettingsSuccessReturnOverrides,
) -> AdmitSolidityGenerateSettingsSuccessReturn {
    AdmitSolidityGenerateSettingsSuccessReturn {
        settings: overrides
            .settings
            .unwrap_or_else(|| build_solidity_generate_settings(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmitSolidityGenerateSettingsDepsOverrides;

pub fn build_admit_solidity_generate_settings_deps<'a>(
    declaration: &'a GenerateDeclaration,
    _overrides: AdmitSolidityGenerateSettingsDepsOverrides,
) -> AdmitSolidityGenerateSettingsDeps<'a> {
    AdmitSolidityGenerateSettingsDeps { declaration }
}

pub fn mock_admit_solidity_generate_settings<'a>(
    _deps: &AdmitSolidityGenerateSettingsDeps<'a>,
    _params: AdmitSolidityGenerateSettingsParams,
    _payload: AdmitSolidityGenerateSettingsPayload,
) -> AdmitSolidityGenerateSettingsReturn {
    let _: AdmitSolidityGenerateSettingsFn<'a> = mock_admit_solidity_generate_settings;
    Ok(build_admit_solidity_generate_settings_success_return(
        Default::default(),
    ))
}
