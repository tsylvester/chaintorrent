#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    RenderParams, RenderPayload, SolidityAbiType, SolidityAbiTypeConstructorParams,
    SolidityCompilerVersion, SolidityConstantName, SolidityConstantNameConstructorParams,
    SolidityConstantValue, SolidityLibraryBody, SolidityLibraryEntries,
    SolidityLibraryEntriesConstructorParams, SolidityLibraryEntry, SolidityLibraryName,
    SolidityLibraryNameConstructorParams, SolidityMemberName, SolidityMemberNameConstructorParams,
    SolidityRecordDeclaration, SolidityRecordDeclarations,
    SolidityRecordDeclarationsConstructorParams, SolidityRecordMember, SolidityRecordMembers,
    SolidityRecordMembersConstructorParams, SolidityStringLiteral,
    SolidityStringLiteralConstructorParams, SolidityTypeName, SolidityTypeNameConstructorParams,
    SolidityUint256, SolidityUint256ConstructorParams, SpdxLicenseIdentifier,
    SpdxLicenseIdentifierConstructorParams,
};
use encoding::CanonicalFieldKind;

#[derive(Default)]
pub struct SolidityLibraryNameConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_solidity_library_name_constructor_params(
    overrides: SolidityLibraryNameConstructorParamsOverrides,
) -> SolidityLibraryNameConstructorParams {
    SolidityLibraryNameConstructorParams {
        text: overrides.text.unwrap_or_else(|| "Constants".to_string()),
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
        text: overrides.text.unwrap_or_else(|| "VALUE".to_string()),
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
        text: overrides.text.unwrap_or_else(|| "UNLICENSED".to_string()),
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
        text: overrides.text.unwrap_or_else(|| "uint256".to_string()),
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
        big_endian: overrides.big_endian.unwrap_or([0x01; 32]),
    }
}

pub fn build_solidity_uint256(
    overrides: SolidityUint256ConstructorParamsOverrides,
) -> SolidityUint256 {
    let Ok(value) = SolidityUint256::try_new(build_solidity_uint256_constructor_params(overrides));
    value
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
        patch: overrides.patch.unwrap_or(28),
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
        value: overrides.value.unwrap_or(SolidityConstantValue::Uint16(1)),
    }
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
        text: overrides.text.unwrap_or_else(|| "Record".to_string()),
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
        text: overrides.text.unwrap_or_else(|| "value".to_string()),
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
    pub kind: Option<CanonicalFieldKind>,
}

pub fn build_solidity_abi_type_constructor_params(
    overrides: SolidityAbiTypeConstructorParamsOverrides,
) -> SolidityAbiTypeConstructorParams {
    SolidityAbiTypeConstructorParams {
        kind: overrides.kind.unwrap_or(CanonicalFieldKind::Bytes),
    }
}

pub fn build_solidity_abi_type(
    overrides: SolidityAbiTypeConstructorParamsOverrides,
) -> SolidityAbiType {
    let Ok(value) = SolidityAbiType::try_new(build_solidity_abi_type_constructor_params(overrides));
    value
}

#[derive(Default)]
pub struct SolidityRecordMemberOverrides {
    pub name: Option<SolidityMemberName>,
    pub kind: Option<CanonicalFieldKind>,
}

pub fn build_solidity_record_member(
    overrides: SolidityRecordMemberOverrides,
) -> SolidityRecordMember {
    SolidityRecordMember {
        name: overrides
            .name
            .unwrap_or_else(|| build_solidity_member_name(Default::default())),
        kind: overrides.kind.unwrap_or(CanonicalFieldKind::Bytes),
    }
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
                            text: Some("left".to_string()),
                        },
                    )),
                    kind: Some(CanonicalFieldKind::Bytes),
                }),
                build_solidity_record_member(SolidityRecordMemberOverrides {
                    name: Some(build_solidity_member_name(
                        SolidityMemberNameConstructorParamsOverrides {
                            text: Some("right".to_string()),
                        },
                    )),
                    kind: Some(CanonicalFieldKind::Unsigned256),
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
        decode_function_name: overrides.decode_function_name.unwrap_or_else(|| {
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("decodeRecord".to_string()),
            })
        }),
        path_constant_name: overrides.path_constant_name.unwrap_or_else(|| {
            build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
                text: Some("RECORD_PATH".to_string()),
            })
        }),
        path: overrides.path.unwrap_or_else(|| {
            build_solidity_string_literal(SolidityStringLiteralConstructorParamsOverrides {
                text: Some("vectors/record.txt".to_string()),
            })
        }),
    }
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
