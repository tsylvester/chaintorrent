#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

use serde_json::{Map, Value};

use self::interface::{
    AdmitSolidityGenerateSettingsDeps, AdmitSolidityGenerateSettingsErrorReturn,
    AdmitSolidityGenerateSettingsParams, AdmitSolidityGenerateSettingsPayload,
    AdmitSolidityGenerateSettingsReturn, AdmitSolidityGenerateSettingsSuccessReturn,
    GeneratedDirectory, GeneratedDirectoryConstructorParams, GeneratedDirectoryTryNewErrorReturn,
    GeneratedDirectoryTryNewReturn, GeneratedFileName, GeneratedFileNameConstructorParams,
    GeneratedFileNameTryNewErrorReturn, GeneratedFileNameTryNewReturn, SOLIDITY_VECTOR_KINDS,
    SolidityGenerateSettings, SolidityGenerateSettingsDocument,
    SolidityGenerateSettingsDocumentErrorReturn, SolidityGenerateSettingsDocumentTryFromReturn,
    SolidityLibraryNames, SolidityLibraryNamesDocument, SolidityVectorKindNames,
    SolidityVectorKindNamesDocument, SolidityVectorKindSettings,
};
use super::super::factory::provides::{
    GenerateDeclaration, SettingBounds, SettingDisposition, SettingStatement, SettingVersion,
};
use super::render::provides::{
    SolidityCompilerVersion, SolidityConstantName, SolidityConstantNameConstructorParams,
    SolidityLibraryName, SolidityLibraryNameConstructorParams, SolidityMemberName,
    SolidityMemberNameConstructorParams, SolidityTypeName, SolidityTypeNameConstructorParams,
    SpdxLicenseIdentifier, SpdxLicenseIdentifierConstructorParams,
};

impl GeneratedFileName {
    pub fn try_new(params: GeneratedFileNameConstructorParams) -> GeneratedFileNameTryNewReturn {
        let bytes = params.text.as_bytes();
        if bytes.is_empty() {
            return Err(GeneratedFileNameTryNewErrorReturn::Empty);
        }
        for (index, byte) in bytes.iter().copied().enumerate() {
            let admitted = matches!(
                byte,
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-'
            );
            if !admitted {
                return Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index, byte });
            }
        }
        if params.text == "." || params.text == ".." {
            return Err(GeneratedFileNameTryNewErrorReturn::Reserved);
        }
        Ok(GeneratedFileName { text: params.text })
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl GeneratedDirectory {
    pub fn try_new(params: GeneratedDirectoryConstructorParams) -> GeneratedDirectoryTryNewReturn {
        if params.segments.is_empty() {
            return Err(GeneratedDirectoryTryNewErrorReturn::Empty);
        }
        Ok(GeneratedDirectory {
            segments: params.segments,
        })
    }

    pub fn segments(&self) -> &[GeneratedFileName] {
        &self.segments
    }
}

fn is_top_level_key(key: &str) -> bool {
    matches!(
        key,
        "kinds"
            | "constants_library"
            | "records_library"
            | "foundry_root"
            | "generated_directory"
            | "compiler_version"
            | "license"
    )
}

fn is_kind_names_key(key: &str) -> bool {
    matches!(
        key,
        "file" | "struct_name" | "decode_function" | "path_constant"
    )
}

fn is_library_names_key(key: &str) -> bool {
    matches!(key, "file" | "library")
}

fn document_text_field(
    table: &Map<String, Value>,
    key: &str,
    path: String,
) -> Result<Option<String>, SolidityGenerateSettingsDocumentErrorReturn> {
    match table.get(key) {
        None => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(SolidityGenerateSettingsDocumentErrorReturn::NotText { path }),
    }
}

fn library_names_document(
    value: Option<&Value>,
    path: &str,
) -> Result<Option<SolidityLibraryNamesDocument>, SolidityGenerateSettingsDocumentErrorReturn> {
    let table = match value {
        None => return Ok(None),
        Some(Value::Object(table)) => table,
        Some(_) => {
            return Err(SolidityGenerateSettingsDocumentErrorReturn::NotATable {
                path: path.to_string(),
            });
        }
    };
    if let Some(unknown) = table.keys().filter(|key| !is_library_names_key(key)).min() {
        return Err(SolidityGenerateSettingsDocumentErrorReturn::UnknownKey {
            path: path.to_string(),
            key: unknown.clone(),
        });
    }
    Ok(Some(SolidityLibraryNamesDocument {
        file: document_text_field(table, "file", format!("{path}.file"))?,
        library: document_text_field(table, "library", format!("{path}.library"))?,
    }))
}

impl TryFrom<serde_json::Value> for SolidityGenerateSettingsDocument {
    type Error = SolidityGenerateSettingsDocumentErrorReturn;

    fn try_from(value: serde_json::Value) -> SolidityGenerateSettingsDocumentTryFromReturn {
        let fields = match value {
            serde_json::Value::Object(fields) => fields,
            _ => {
                return Err(Self::Error::NotATable {
                    path: String::new(),
                });
            }
        };
        if let Some(key) = fields.keys().filter(|key| !is_top_level_key(key)).min() {
            return Err(Self::Error::UnknownKey {
                path: String::new(),
                key: key.clone(),
            });
        }
        let kinds = match fields.get("kinds") {
            None => None,
            Some(serde_json::Value::Object(table)) => {
                let mut entries: Vec<(&String, &serde_json::Value)> = table.iter().collect();
                entries.sort_by(|left, right| left.0.cmp(right.0));
                let mut admitted = BTreeMap::new();
                for (key, entry) in entries {
                    let path = format!("kinds.{key}");
                    let names = match entry {
                        serde_json::Value::Object(names) => names,
                        _ => return Err(Self::Error::NotATable { path }),
                    };
                    if let Some(unknown) = names.keys().filter(|key| !is_kind_names_key(key)).min()
                    {
                        return Err(Self::Error::UnknownKey {
                            path,
                            key: unknown.clone(),
                        });
                    }
                    admitted.insert(
                        key.clone(),
                        SolidityVectorKindNamesDocument {
                            file: document_text_field(names, "file", format!("{path}.file"))?,
                            struct_name: document_text_field(
                                names,
                                "struct_name",
                                format!("{path}.struct_name"),
                            )?,
                            decode_function: document_text_field(
                                names,
                                "decode_function",
                                format!("{path}.decode_function"),
                            )?,
                            path_constant: document_text_field(
                                names,
                                "path_constant",
                                format!("{path}.path_constant"),
                            )?,
                        },
                    );
                }
                Some(admitted)
            }
            Some(_) => {
                return Err(Self::Error::NotATable {
                    path: "kinds".to_string(),
                });
            }
        };
        let constants_library =
            library_names_document(fields.get("constants_library"), "constants_library")?;
        let records_library =
            library_names_document(fields.get("records_library"), "records_library")?;
        let foundry_root =
            document_text_field(&fields, "foundry_root", "foundry_root".to_string())?;
        let generated_directory = match fields.get("generated_directory") {
            None => None,
            Some(serde_json::Value::Array(items)) => {
                let mut texts = Vec::with_capacity(items.len());
                for item in items {
                    match item {
                        serde_json::Value::String(text) => texts.push(text.clone()),
                        _ => {
                            return Err(Self::Error::NotTextList {
                                path: "generated_directory".to_string(),
                            });
                        }
                    }
                }
                Some(texts)
            }
            Some(_) => {
                return Err(Self::Error::NotTextList {
                    path: "generated_directory".to_string(),
                });
            }
        };
        let compiler_version =
            document_text_field(&fields, "compiler_version", "compiler_version".to_string())?;
        let license = document_text_field(&fields, "license", "license".to_string())?;
        Ok(SolidityGenerateSettingsDocument {
            kinds,
            constants_library,
            records_library,
            foundry_root,
            generated_directory,
            compiler_version,
            license,
        })
    }
}

fn resolve_setting<'a>(
    declaration: &'a GenerateDeclaration,
    setting: &str,
    supplied: Option<String>,
) -> Result<(String, &'a SettingStatement), AdmitSolidityGenerateSettingsErrorReturn> {
    let Some(statement) = declaration
        .settings
        .iter()
        .find(|statement| statement.name == setting)
    else {
        return Err(
            AdmitSolidityGenerateSettingsErrorReturn::UndeclaredSetting {
                setting: setting.to_string(),
            },
        );
    };
    let text = match (statement.disposition, supplied) {
        (SettingDisposition::Fixed { .. }, Some(supplied)) => {
            return Err(AdmitSolidityGenerateSettingsErrorReturn::FixedSetting {
                setting: setting.to_string(),
                supplied,
            });
        }
        (SettingDisposition::Fixed { value }, None) => value.to_string(),
        (SettingDisposition::Settable { .. }, Some(supplied)) => supplied,
        (SettingDisposition::Settable { default, .. }, None) => default.to_string(),
    };
    Ok((text, statement))
}

fn admit_vector_kind_names(
    declaration: &GenerateDeclaration,
    name: &str,
    document: Option<&SolidityVectorKindNamesDocument>,
) -> Result<SolidityVectorKindNames, AdmitSolidityGenerateSettingsErrorReturn> {
    let setting = format!("kinds.{name}.file");
    let file_name = GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
        text: resolve_setting(
            declaration,
            &setting,
            document.and_then(|names| names.file.clone()),
        )?
        .0,
    })
    .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::FileName { setting, error })?;
    let setting = format!("kinds.{name}.struct_name");
    let struct_name = SolidityTypeName::try_new(SolidityTypeNameConstructorParams {
        text: resolve_setting(
            declaration,
            &setting,
            document.and_then(|names| names.struct_name.clone()),
        )?
        .0,
    })
    .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::TypeName { setting, error })?;
    let setting = format!("kinds.{name}.decode_function");
    let decode_function_name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
        text: resolve_setting(
            declaration,
            &setting,
            document.and_then(|names| names.decode_function.clone()),
        )?
        .0,
    })
    .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::MemberName { setting, error })?;
    let setting = format!("kinds.{name}.path_constant");
    let path_constant_name = SolidityConstantName::try_new(SolidityConstantNameConstructorParams {
        text: resolve_setting(
            declaration,
            &setting,
            document.and_then(|names| names.path_constant.clone()),
        )?
        .0,
    })
    .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::ConstantName { setting, error })?;
    Ok(SolidityVectorKindNames {
        file_name,
        struct_name,
        decode_function_name,
        path_constant_name,
    })
}

fn admit_library_names(
    declaration: &GenerateDeclaration,
    name: &str,
    document: Option<&SolidityLibraryNamesDocument>,
) -> Result<SolidityLibraryNames, AdmitSolidityGenerateSettingsErrorReturn> {
    let setting = format!("{name}.file");
    let file_name = GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
        text: resolve_setting(
            declaration,
            &setting,
            document.and_then(|names| names.file.clone()),
        )?
        .0,
    })
    .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::FileName { setting, error })?;
    let setting = format!("{name}.library");
    let library_name = SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams {
        text: resolve_setting(
            declaration,
            &setting,
            document.and_then(|names| names.library.clone()),
        )?
        .0,
    })
    .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::LibraryName { setting, error })?;
    Ok(SolidityLibraryNames {
        file_name,
        library_name,
    })
}

fn version_is_less(version: &SettingVersion, bound: &SettingVersion) -> bool {
    (version.major, version.minor, version.patch) < (bound.major, bound.minor, bound.patch)
}

fn admit_compiler_version(
    statement: &SettingStatement,
    text: String,
) -> Result<SolidityCompilerVersion, AdmitSolidityGenerateSettingsErrorReturn> {
    const SETTING: &str = "compiler_version";
    let parts: Vec<&str> = text.split('.').collect();
    if parts.len() != 3 {
        return Err(
            AdmitSolidityGenerateSettingsErrorReturn::CompilerVersionParts {
                setting: SETTING.to_string(),
                text,
            },
        );
    }
    let mut version = [0_u16; 3];
    for (index, part) in parts.iter().enumerate() {
        version[index] = u16::from_str(part).map_err(|_| {
            AdmitSolidityGenerateSettingsErrorReturn::CompilerVersionPart {
                setting: SETTING.to_string(),
                index,
                text: (*part).to_string(),
            }
        })?;
    }
    if let SettingDisposition::Settable {
        bounds: SettingBounds::VersionWithin { at_least, below },
        ..
    } = statement.disposition
    {
        let supplied = SettingVersion {
            major: version[0],
            minor: version[1],
            patch: version[2],
        };
        if version_is_less(&supplied, &at_least) || !version_is_less(&supplied, &below) {
            return Err(AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds {
                setting: SETTING.to_string(),
                supplied: text,
            });
        }
    }
    Ok(SolidityCompilerVersion {
        major: version[0],
        minor: version[1],
        patch: version[2],
    })
}

pub fn admit_solidity_generate_settings<'a>(
    deps: &AdmitSolidityGenerateSettingsDeps<'a>,
    _params: AdmitSolidityGenerateSettingsParams,
    payload: AdmitSolidityGenerateSettingsPayload,
) -> AdmitSolidityGenerateSettingsReturn {
    let document = payload.document;

    if let Some(kinds) = &document.kinds {
        let declared: Vec<String> = deps
            .declaration
            .kinds
            .iter()
            .map(|declaration| format!("{:?}", declaration.kind))
            .collect();
        if let Some(text) = kinds
            .keys()
            .find(|key| !declared.iter().any(|name| name == *key))
        {
            return Err(AdmitSolidityGenerateSettingsErrorReturn::UnknownKind {
                text: text.clone(),
            });
        }
    }

    for declaration in deps.declaration.kinds {
        if !SOLIDITY_VECTOR_KINDS.contains(&declaration.kind) {
            return Err(AdmitSolidityGenerateSettingsErrorReturn::UnknownKind {
                text: format!("{:?}", declaration.kind),
            });
        }
    }

    let mut kinds = Vec::with_capacity(deps.declaration.kinds.len());
    for declaration in deps.declaration.kinds {
        let name = format!("{:?}", declaration.kind);
        let supplied = document.kinds.as_ref().and_then(|kinds| kinds.get(&name));
        kinds.push(SolidityVectorKindSettings {
            kind: declaration.kind,
            names: admit_vector_kind_names(deps.declaration, &name, supplied)?,
        });
    }

    let constants_library = admit_library_names(
        deps.declaration,
        "constants_library",
        document.constants_library.as_ref(),
    )?;
    let records_library = admit_library_names(
        deps.declaration,
        "records_library",
        document.records_library.as_ref(),
    )?;

    let foundry_root = PathBuf::from(
        resolve_setting(
            deps.declaration,
            "foundry_root",
            document.foundry_root.clone(),
        )?
        .0,
    );

    let default_directory = resolve_setting(deps.declaration, "generated_directory", None)?.0;
    let segment_texts: Vec<String> = match &document.generated_directory {
        Some(texts) => texts.clone(),
        None => default_directory.split('/').map(String::from).collect(),
    };
    let mut segments = Vec::with_capacity(segment_texts.len());
    for (index, text) in segment_texts.into_iter().enumerate() {
        segments.push(
            GeneratedFileName::try_new(GeneratedFileNameConstructorParams { text }).map_err(
                |error| AdmitSolidityGenerateSettingsErrorReturn::DirectorySegment { index, error },
            )?,
        );
    }
    let generated_directory =
        GeneratedDirectory::try_new(GeneratedDirectoryConstructorParams { segments })
            .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::Directory { error })?;

    let (version_text, version_statement) = resolve_setting(
        deps.declaration,
        "compiler_version",
        document.compiler_version.clone(),
    )?;
    let compiler_version = admit_compiler_version(version_statement, version_text)?;

    let license = SpdxLicenseIdentifier::try_new(SpdxLicenseIdentifierConstructorParams {
        text: resolve_setting(deps.declaration, "license", document.license.clone())?.0,
    })
    .map_err(|error| AdmitSolidityGenerateSettingsErrorReturn::License { error })?;

    Ok(AdmitSolidityGenerateSettingsSuccessReturn {
        settings: SolidityGenerateSettings {
            kinds,
            constants_library,
            records_library,
            foundry_root,
            generated_directory,
            compiler_version,
            license,
        },
    })
}
