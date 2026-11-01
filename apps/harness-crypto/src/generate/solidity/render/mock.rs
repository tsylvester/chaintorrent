#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use encoding::CanonicalFieldKind;

use super::interface::{
    RenderDeps, RenderParams, RenderPayload, RenderReturn, RenderSuccessReturn, SolidityAbiType,
    SolidityAbiTypeConstructorParams, SolidityCompilerVersion, SolidityConstantName,
    SolidityConstantNameConstructorParams, SolidityConstantValue, SolidityLibraryBody,
    SolidityLibraryEntries, SolidityLibraryEntriesConstructorParams, SolidityLibraryEntry,
    SolidityLibraryName, SolidityLibraryNameConstructorParams, SolidityMemberName,
    SolidityMemberNameConstructorParams, SolidityRecordDeclaration, SolidityRecordDeclarations,
    SolidityRecordDeclarationsConstructorParams, SolidityRecordMember, SolidityRecordMembers,
    SolidityRecordMembersConstructorParams, SoliditySourceText, SolidityStringLiteral,
    SolidityStringLiteralConstructorParams, SolidityTypeName, SolidityTypeNameConstructorParams,
    SolidityUint256, SolidityUint256ConstructorParams, SpdxLicenseIdentifier,
    SpdxLicenseIdentifierConstructorParams,
};

#[derive(Default)]
pub struct SolidityLibraryNameConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_solidity_library_name_constructor_params(
    overrides: SolidityLibraryNameConstructorParamsOverrides,
) -> SolidityLibraryNameConstructorParams {
    SolidityLibraryNameConstructorParams {
        text: overrides
            .text
            .unwrap_or_else(|| "SampleLibrary".to_string()),
    }
}

pub fn build_solidity_library_name(
    overrides: SolidityLibraryNameConstructorParamsOverrides,
) -> SolidityLibraryName {
    SolidityLibraryName::try_new(build_solidity_library_name_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityConstantNameConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_solidity_constant_name_constructor_params(
    overrides: SolidityConstantNameConstructorParamsOverrides,
) -> SolidityConstantNameConstructorParams {
    SolidityConstantNameConstructorParams {
        text: overrides
            .text
            .unwrap_or_else(|| "SAMPLE_CONSTANT".to_string()),
    }
}

pub fn build_solidity_constant_name(
    overrides: SolidityConstantNameConstructorParamsOverrides,
) -> SolidityConstantName {
    SolidityConstantName::try_new(build_solidity_constant_name_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SpdxLicenseIdentifierConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_spdx_license_identifier_constructor_params(
    overrides: SpdxLicenseIdentifierConstructorParamsOverrides,
) -> SpdxLicenseIdentifierConstructorParams {
    SpdxLicenseIdentifierConstructorParams {
        text: overrides.text.unwrap_or_else(|| "MIT".to_string()),
    }
}

pub fn build_spdx_license_identifier(
    overrides: SpdxLicenseIdentifierConstructorParamsOverrides,
) -> SpdxLicenseIdentifier {
    SpdxLicenseIdentifier::try_new(build_spdx_license_identifier_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityStringLiteralConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_solidity_string_literal_constructor_params(
    overrides: SolidityStringLiteralConstructorParamsOverrides,
) -> SolidityStringLiteralConstructorParams {
    SolidityStringLiteralConstructorParams {
        text: overrides.text.unwrap_or_else(|| "sample".to_string()),
    }
}

pub fn build_solidity_string_literal(
    overrides: SolidityStringLiteralConstructorParamsOverrides,
) -> SolidityStringLiteral {
    SolidityStringLiteral::try_new(build_solidity_string_literal_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityUint256ConstructorParamsOverrides {
    pub big_endian: Option<[u8; 32]>,
}

pub fn build_solidity_uint256_constructor_params(
    overrides: SolidityUint256ConstructorParamsOverrides,
) -> SolidityUint256ConstructorParams {
    SolidityUint256ConstructorParams {
        big_endian: overrides.big_endian.unwrap_or_else(|| {
            core::array::from_fn(|index| {
                255 - u8::try_from(index).expect("a 32-element index fits a u8")
            })
        }),
    }
}

pub fn build_solidity_uint256(
    overrides: SolidityUint256ConstructorParamsOverrides,
) -> SolidityUint256 {
    SolidityUint256::try_new(build_solidity_uint256_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityLibraryEntriesConstructorParamsOverrides {
    pub entries: Option<Vec<SolidityLibraryEntry>>,
}

pub fn build_solidity_library_entries_constructor_params(
    overrides: SolidityLibraryEntriesConstructorParamsOverrides,
) -> SolidityLibraryEntriesConstructorParams {
    SolidityLibraryEntriesConstructorParams {
        entries: overrides
            .entries
            .unwrap_or_else(|| vec![build_solidity_library_entry(Default::default())]),
    }
}

pub fn build_solidity_library_entries(
    overrides: SolidityLibraryEntriesConstructorParamsOverrides,
) -> SolidityLibraryEntries {
    SolidityLibraryEntries::try_new(build_solidity_library_entries_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityTypeNameConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_solidity_type_name_constructor_params(
    overrides: SolidityTypeNameConstructorParamsOverrides,
) -> SolidityTypeNameConstructorParams {
    SolidityTypeNameConstructorParams {
        text: overrides.text.unwrap_or_else(|| "SampleRecord".to_string()),
    }
}

pub fn build_solidity_type_name(
    overrides: SolidityTypeNameConstructorParamsOverrides,
) -> SolidityTypeName {
    SolidityTypeName::try_new(build_solidity_type_name_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityMemberNameConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_solidity_member_name_constructor_params(
    overrides: SolidityMemberNameConstructorParamsOverrides,
) -> SolidityMemberNameConstructorParams {
    SolidityMemberNameConstructorParams {
        text: overrides.text.unwrap_or_else(|| "sampleMember".to_string()),
    }
}

pub fn build_solidity_member_name(
    overrides: SolidityMemberNameConstructorParamsOverrides,
) -> SolidityMemberName {
    SolidityMemberName::try_new(build_solidity_member_name_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityAbiTypeConstructorParamsOverrides {
    pub name: Option<String>,
}

pub fn build_solidity_abi_type_constructor_params(
    overrides: SolidityAbiTypeConstructorParamsOverrides,
) -> SolidityAbiTypeConstructorParams {
    SolidityAbiTypeConstructorParams {
        name: overrides.name.unwrap_or_else(|| "bytes32".to_string()),
    }
}

pub fn build_solidity_abi_type(
    overrides: SolidityAbiTypeConstructorParamsOverrides,
) -> SolidityAbiType {
    SolidityAbiType::try_new(build_solidity_abi_type_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityRecordMembersConstructorParamsOverrides {
    pub members: Option<Vec<SolidityRecordMember>>,
}

pub fn build_solidity_record_members_constructor_params(
    overrides: SolidityRecordMembersConstructorParamsOverrides,
) -> SolidityRecordMembersConstructorParams {
    SolidityRecordMembersConstructorParams {
        members: overrides.members.unwrap_or_else(|| {
            vec![
                build_solidity_record_member(SolidityRecordMemberOverrides {
                    name: Some(build_solidity_member_name(
                        SolidityMemberNameConstructorParamsOverrides {
                            text: Some("alphaMember".to_string()),
                        },
                    )),
                    ..Default::default()
                }),
                build_solidity_record_member(SolidityRecordMemberOverrides {
                    name: Some(build_solidity_member_name(
                        SolidityMemberNameConstructorParamsOverrides {
                            text: Some("betaMember".to_string()),
                        },
                    )),
                    ..Default::default()
                }),
            ]
        }),
    }
}

pub fn build_solidity_record_members(
    overrides: SolidityRecordMembersConstructorParamsOverrides,
) -> SolidityRecordMembers {
    SolidityRecordMembers::try_new(build_solidity_record_members_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityRecordDeclarationsConstructorParamsOverrides {
    pub declarations: Option<Vec<SolidityRecordDeclaration>>,
}

pub fn build_solidity_record_declarations_constructor_params(
    overrides: SolidityRecordDeclarationsConstructorParamsOverrides,
) -> SolidityRecordDeclarationsConstructorParams {
    SolidityRecordDeclarationsConstructorParams {
        declarations: overrides
            .declarations
            .unwrap_or_else(|| vec![build_solidity_record_declaration(Default::default())]),
    }
}

pub fn build_solidity_record_declarations(
    overrides: SolidityRecordDeclarationsConstructorParamsOverrides,
) -> SolidityRecordDeclarations {
    SolidityRecordDeclarations::try_new(build_solidity_record_declarations_constructor_params(
        overrides,
    ))
    .expect("built params are valid")
}

#[derive(Default)]
pub struct SolidityCompilerVersionOverrides {
    pub major: Option<u16>,
    pub minor: Option<u16>,
    pub patch: Option<u16>,
}

pub fn build_solidity_compiler_version(
    overrides: SolidityCompilerVersionOverrides,
) -> SolidityCompilerVersion {
    SolidityCompilerVersion {
        major: overrides.major.unwrap_or(0),
        minor: overrides.minor.unwrap_or(8),
        patch: overrides.patch.unwrap_or(24),
    }
}

#[derive(Default)]
pub struct SolidityLibraryEntryOverrides {
    pub name: Option<SolidityConstantName>,
    pub value: Option<SolidityConstantValue>,
}

pub fn build_solidity_library_entry(
    overrides: SolidityLibraryEntryOverrides,
) -> SolidityLibraryEntry {
    SolidityLibraryEntry {
        name: overrides
            .name
            .unwrap_or_else(|| build_solidity_constant_name(Default::default())),
        value: overrides.value.unwrap_or(SolidityConstantValue::Bool(true)),
    }
}

#[derive(Default)]
pub struct SolidityRecordMemberOverrides {
    pub name: Option<SolidityMemberName>,
    pub kind: Option<CanonicalFieldKind>,
    pub abi_type: Option<SolidityAbiType>,
}

pub fn build_solidity_record_member(
    overrides: SolidityRecordMemberOverrides,
) -> SolidityRecordMember {
    SolidityRecordMember {
        name: overrides
            .name
            .unwrap_or_else(|| build_solidity_member_name(Default::default())),
        kind: overrides.kind.unwrap_or(CanonicalFieldKind::FixedBytes32),
        abi_type: overrides
            .abi_type
            .unwrap_or_else(|| build_solidity_abi_type(Default::default())),
    }
}

#[derive(Default)]
pub struct SolidityRecordDeclarationOverrides {
    pub struct_name: Option<SolidityTypeName>,
    pub members: Option<SolidityRecordMembers>,
    pub decode_function_name: Option<SolidityMemberName>,
    pub path_constant_name: Option<SolidityConstantName>,
    pub path: Option<SolidityStringLiteral>,
}

pub fn build_solidity_record_declaration(
    overrides: SolidityRecordDeclarationOverrides,
) -> SolidityRecordDeclaration {
    SolidityRecordDeclaration {
        struct_name: overrides
            .struct_name
            .unwrap_or_else(|| build_solidity_type_name(Default::default())),
        members: overrides
            .members
            .unwrap_or_else(|| build_solidity_record_members(Default::default())),
        decode_function_name: overrides
            .decode_function_name
            .unwrap_or_else(|| build_solidity_member_name(Default::default())),
        path_constant_name: overrides
            .path_constant_name
            .unwrap_or_else(|| build_solidity_constant_name(Default::default())),
        path: overrides
            .path
            .unwrap_or_else(|| build_solidity_string_literal(Default::default())),
    }
}

#[derive(Default)]
pub struct RenderDepsOverrides;

pub fn build_render_deps(_overrides: RenderDepsOverrides) -> RenderDeps {
    RenderDeps
}

#[derive(Default)]
pub struct RenderParamsOverrides {
    pub license: Option<SpdxLicenseIdentifier>,
    pub compiler_version: Option<SolidityCompilerVersion>,
}

pub fn build_render_params(overrides: RenderParamsOverrides) -> RenderParams {
    RenderParams {
        license: overrides
            .license
            .unwrap_or_else(|| build_spdx_license_identifier(Default::default())),
        compiler_version: overrides
            .compiler_version
            .unwrap_or_else(|| build_solidity_compiler_version(Default::default())),
    }
}

#[derive(Default)]
pub struct RenderPayloadOverrides {
    pub library_name: Option<SolidityLibraryName>,
    pub body: Option<SolidityLibraryBody>,
}

pub fn build_render_payload(overrides: RenderPayloadOverrides) -> RenderPayload {
    RenderPayload {
        library_name: overrides
            .library_name
            .unwrap_or_else(|| build_solidity_library_name(Default::default())),
        body: overrides.body.unwrap_or_else(|| {
            SolidityLibraryBody::Constants(build_solidity_library_entries(Default::default()))
        }),
    }
}

#[derive(Default)]
pub struct SoliditySourceTextOverrides {
    pub text: Option<String>,
}

pub fn build_solidity_source_text(overrides: SoliditySourceTextOverrides) -> SoliditySourceText {
    SoliditySourceText {
        text: overrides.text.unwrap_or_else(|| "// sample\n".to_string()),
    }
}

#[derive(Default)]
pub struct RenderSuccessReturnOverrides {
    pub source: Option<SoliditySourceText>,
}

pub fn build_render_success_return(overrides: RenderSuccessReturnOverrides) -> RenderSuccessReturn {
    RenderSuccessReturn {
        source: overrides
            .source
            .unwrap_or_else(|| build_solidity_source_text(Default::default())),
    }
}

pub fn mock_render(
    _deps: &RenderDeps,
    _params: RenderParams,
    _payload: RenderPayload,
) -> RenderReturn {
    Ok(build_render_success_return(Default::default()))
}
