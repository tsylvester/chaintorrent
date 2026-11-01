#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use encoding::CanonicalFieldKind;

use self::interface::{
    RenderDeps, RenderParams, RenderPayload, RenderReturn, RenderSuccessReturn, SolidityAbiType,
    SolidityAbiTypeConstructorParams, SolidityAbiTypeTryNewErrorReturn,
    SolidityAbiTypeTryNewReturn, SolidityConstantName, SolidityConstantNameConstructorParams,
    SolidityConstantNameTryNewErrorReturn, SolidityConstantNameTryNewReturn, SolidityConstantValue,
    SolidityLibraryBody, SolidityLibraryEntries, SolidityLibraryEntriesConstructorParams,
    SolidityLibraryEntriesTryNewErrorReturn, SolidityLibraryEntriesTryNewReturn,
    SolidityLibraryName, SolidityLibraryNameConstructorParams,
    SolidityLibraryNameTryNewErrorReturn, SolidityLibraryNameTryNewReturn, SolidityMemberName,
    SolidityMemberNameConstructorParams, SolidityMemberNameTryNewErrorReturn,
    SolidityMemberNameTryNewReturn, SolidityRecordDeclarations,
    SolidityRecordDeclarationsConstructorParams, SolidityRecordDeclarationsTryNewErrorReturn,
    SolidityRecordDeclarationsTryNewReturn, SolidityRecordMember, SolidityRecordMembers,
    SolidityRecordMembersConstructorParams, SolidityRecordMembersTryNewErrorReturn,
    SolidityRecordMembersTryNewReturn, SoliditySourceText, SolidityStringLiteral,
    SolidityStringLiteralConstructorParams, SolidityStringLiteralTryNewErrorReturn,
    SolidityStringLiteralTryNewReturn, SolidityTypeName, SolidityTypeNameConstructorParams,
    SolidityTypeNameTryNewErrorReturn, SolidityTypeNameTryNewReturn, SolidityUint256,
    SolidityUint256ConstructorParams, SolidityUint256TryNewReturn, SpdxLicenseIdentifier,
    SpdxLicenseIdentifierConstructorParams, SpdxLicenseIdentifierTryNewErrorReturn,
    SpdxLicenseIdentifierTryNewReturn,
};

impl SolidityLibraryName {
    pub fn try_new(
        params: SolidityLibraryNameConstructorParams,
    ) -> SolidityLibraryNameTryNewReturn {
        let bytes = params.text.as_bytes();
        let Some(&first) = bytes.first() else {
            return Err(SolidityLibraryNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_uppercase() {
            return Err(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        for (index, &byte) in bytes.iter().enumerate().skip(1) {
            if !byte.is_ascii_alphanumeric() {
                return Err(SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index, byte });
            }
        }
        Ok(SolidityLibraryName { text: params.text })
    }
}

impl SolidityConstantName {
    pub fn try_new(
        params: SolidityConstantNameConstructorParams,
    ) -> SolidityConstantNameTryNewReturn {
        let bytes = params.text.as_bytes();
        let Some(&first) = bytes.first() else {
            return Err(SolidityConstantNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_uppercase() {
            return Err(SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        for (index, &byte) in bytes.iter().enumerate().skip(1) {
            if !(byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_') {
                return Err(SolidityConstantNameTryNewErrorReturn::InvalidCharacter {
                    index,
                    byte,
                });
            }
        }
        Ok(SolidityConstantName { text: params.text })
    }
}

impl SpdxLicenseIdentifier {
    pub fn try_new(
        params: SpdxLicenseIdentifierConstructorParams,
    ) -> SpdxLicenseIdentifierTryNewReturn {
        let bytes = params.text.as_bytes();
        let identifier = bytes.strip_suffix(b"+").unwrap_or(bytes);
        if identifier.is_empty() {
            return Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty);
        }
        for (index, &byte) in identifier.iter().enumerate() {
            if !(byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'-') {
                return Err(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
                    index,
                    byte,
                });
            }
        }
        Ok(SpdxLicenseIdentifier { text: params.text })
    }
}

impl SolidityStringLiteral {
    pub fn try_new(
        params: SolidityStringLiteralConstructorParams,
    ) -> SolidityStringLiteralTryNewReturn {
        for (index, &byte) in params.text.as_bytes().iter().enumerate() {
            if !(0x20..=0x7E).contains(&byte) || byte == b'"' || byte == b'\\' {
                return Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
                    index,
                    byte,
                });
            }
        }
        Ok(SolidityStringLiteral { text: params.text })
    }
}

impl SolidityUint256 {
    pub fn try_new(params: SolidityUint256ConstructorParams) -> SolidityUint256TryNewReturn {
        Ok(SolidityUint256 {
            big_endian: params.big_endian,
        })
    }
}

impl SolidityLibraryEntries {
    pub fn try_new(
        params: SolidityLibraryEntriesConstructorParams,
    ) -> SolidityLibraryEntriesTryNewReturn {
        if params.entries.is_empty() {
            return Err(SolidityLibraryEntriesTryNewErrorReturn::Empty);
        }
        for (j, entry) in params.entries.iter().enumerate() {
            for (i, earlier) in params.entries.iter().enumerate().take(j) {
                if entry.name == earlier.name {
                    return Err(SolidityLibraryEntriesTryNewErrorReturn::DuplicateName {
                        first_index: i,
                        duplicate_index: j,
                    });
                }
            }
        }
        Ok(SolidityLibraryEntries {
            entries: params.entries,
        })
    }
}

impl SolidityTypeName {
    pub fn try_new(params: SolidityTypeNameConstructorParams) -> SolidityTypeNameTryNewReturn {
        let bytes = params.text.as_bytes();
        let Some(&first) = bytes.first() else {
            return Err(SolidityTypeNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_uppercase() {
            return Err(SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        for (index, &byte) in bytes.iter().enumerate().skip(1) {
            if !byte.is_ascii_alphanumeric() {
                return Err(SolidityTypeNameTryNewErrorReturn::InvalidCharacter { index, byte });
            }
        }
        Ok(SolidityTypeName { text: params.text })
    }
}

impl SolidityMemberName {
    pub fn try_new(params: SolidityMemberNameConstructorParams) -> SolidityMemberNameTryNewReturn {
        let bytes = params.text.as_bytes();
        let Some(&first) = bytes.first() else {
            return Err(SolidityMemberNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_lowercase() {
            return Err(SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        for (index, &byte) in bytes.iter().enumerate().skip(1) {
            if !byte.is_ascii_alphanumeric() {
                return Err(SolidityMemberNameTryNewErrorReturn::InvalidCharacter { index, byte });
            }
        }
        Ok(SolidityMemberName { text: params.text })
    }
}

impl SolidityAbiType {
    pub fn try_new(params: SolidityAbiTypeConstructorParams) -> SolidityAbiTypeTryNewReturn {
        let bytes = params.name.as_bytes();
        let Some(&first) = bytes.first() else {
            return Err(SolidityAbiTypeTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_alphabetic() {
            return Err(SolidityAbiTypeTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        for (index, &byte) in bytes.iter().enumerate().skip(1) {
            if !byte.is_ascii_alphanumeric() {
                return Err(SolidityAbiTypeTryNewErrorReturn::InvalidCharacter { index, byte });
            }
        }
        Ok(SolidityAbiType { text: params.name })
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl SolidityRecordMember {
    pub fn decodes_into_memory(&self) -> bool {
        match self.kind {
            CanonicalFieldKind::Text | CanonicalFieldKind::Bytes => true,
            CanonicalFieldKind::FixedBytes20
            | CanonicalFieldKind::FixedBytes32
            | CanonicalFieldKind::Unsigned16
            | CanonicalFieldKind::Unsigned32
            | CanonicalFieldKind::Unsigned64
            | CanonicalFieldKind::Unsigned256 => false,
        }
    }
}

impl SolidityRecordMembers {
    pub fn try_new(
        params: SolidityRecordMembersConstructorParams,
    ) -> SolidityRecordMembersTryNewReturn {
        let actual = params.members.len();
        if actual < 2 {
            return Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual });
        }
        for (index, member) in params.members.iter().enumerate() {
            if member.name.text == "line" {
                return Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index });
            }
        }
        for (j, member) in params.members.iter().enumerate() {
            for (i, earlier) in params.members.iter().enumerate().take(j) {
                if member.name == earlier.name {
                    return Err(SolidityRecordMembersTryNewErrorReturn::DuplicateName {
                        first_index: i,
                        duplicate_index: j,
                    });
                }
            }
        }
        Ok(SolidityRecordMembers {
            members: params.members,
        })
    }
}

impl SolidityRecordDeclarations {
    pub fn try_new(
        params: SolidityRecordDeclarationsConstructorParams,
    ) -> SolidityRecordDeclarationsTryNewReturn {
        if params.declarations.is_empty() {
            return Err(SolidityRecordDeclarationsTryNewErrorReturn::Empty);
        }
        for (j, declaration) in params.declarations.iter().enumerate() {
            for (i, earlier) in params.declarations.iter().enumerate().take(j) {
                if declaration.struct_name == earlier.struct_name {
                    return Err(
                        SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                            first_index: i,
                            duplicate_index: j,
                        },
                    );
                }
            }
        }
        for (j, declaration) in params.declarations.iter().enumerate() {
            for (i, earlier) in params.declarations.iter().enumerate().take(j) {
                if declaration.decode_function_name == earlier.decode_function_name {
                    return Err(
                        SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                            first_index: i,
                            duplicate_index: j,
                        },
                    );
                }
            }
        }
        for (j, declaration) in params.declarations.iter().enumerate() {
            for (i, earlier) in params.declarations.iter().enumerate().take(j) {
                if declaration.path_constant_name == earlier.path_constant_name {
                    return Err(
                        SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName {
                            first_index: i,
                            duplicate_index: j,
                        },
                    );
                }
            }
        }
        Ok(SolidityRecordDeclarations {
            declarations: params.declarations,
        })
    }
}

impl SoliditySourceText {
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

pub fn render(_deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn {
    let mut text = String::new();
    text.push_str("// SPDX-License-Identifier: ");
    text.push_str(&params.license.text);
    text.push_str("\npragma solidity ");
    text.push_str(&params.compiler_version.major.to_string());
    text.push_str(".");
    text.push_str(&params.compiler_version.minor.to_string());
    text.push_str(".");
    text.push_str(&params.compiler_version.patch.to_string());
    text.push_str(";\n\nlibrary ");
    text.push_str(&payload.library_name.text);
    text.push_str(" {\n");
    match payload.body {
        SolidityLibraryBody::Constants(entries) => {
            for entry in entries.entries {
                text.push_str("    ");
                match entry.value {
                    SolidityConstantValue::Bytes(bytes) => {
                        text.push_str("bytes internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = hex\"");
                        for byte in bytes {
                            text.push_str(&format!("{byte:02x}"));
                        }
                        text.push_str("\";\n");
                    }
                    SolidityConstantValue::Uint256(value) => {
                        text.push_str("uint256 internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = 0x");
                        for byte in value.big_endian {
                            text.push_str(&format!("{byte:02x}"));
                        }
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::Uint64(value) => {
                        text.push_str("uint64 internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = ");
                        text.push_str(&value.to_string());
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::Uint16(value) => {
                        text.push_str("uint16 internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = ");
                        text.push_str(&value.to_string());
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::Bool(value) => {
                        text.push_str("bool internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = ");
                        text.push_str(if value { "true" } else { "false" });
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::String(literal) => {
                        text.push_str("string internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = \"");
                        text.push_str(&literal.text);
                        text.push_str("\";\n");
                    }
                }
            }
        }
        SolidityLibraryBody::Records(declarations) => {
            for (index, declaration) in declarations.declarations.iter().enumerate() {
                if index > 0 {
                    text.push_str("\n");
                }
                text.push_str("    struct ");
                text.push_str(&declaration.struct_name.text);
                text.push_str(" {\n");
                for member in &declaration.members.members {
                    text.push_str("        ");
                    text.push_str(member.abi_type.as_str());
                    text.push_str(" ");
                    text.push_str(&member.name.text);
                    text.push_str(";\n");
                }
                text.push_str("    }\n");
                text.push_str("\n");
                text.push_str("    string internal constant ");
                text.push_str(&declaration.path_constant_name.text);
                text.push_str(" = \"");
                text.push_str(&declaration.path.text);
                text.push_str("\";\n");
                text.push_str("\n");
                text.push_str("    function ");
                text.push_str(&declaration.decode_function_name.text);
                text.push_str("(bytes memory line) internal pure returns (");
                text.push_str(&declaration.struct_name.text);
                text.push_str(" memory) {\n");
                text.push_str("        (");
                for (member_index, member) in declaration.members.members.iter().enumerate() {
                    if member_index > 0 {
                        text.push_str(", ");
                    }
                    text.push_str(member.abi_type.as_str());
                    if member.decodes_into_memory() {
                        text.push_str(" memory");
                    }
                    text.push_str(" ");
                    text.push_str(&member.name.text);
                }
                text.push_str(") = abi.decode(line, (");
                for (member_index, member) in declaration.members.members.iter().enumerate() {
                    if member_index > 0 {
                        text.push_str(", ");
                    }
                    text.push_str(member.abi_type.as_str());
                }
                text.push_str("));\n");
                text.push_str("        return ");
                text.push_str(&declaration.struct_name.text);
                text.push_str("({");
                for (member_index, member) in declaration.members.members.iter().enumerate() {
                    if member_index > 0 {
                        text.push_str(", ");
                    }
                    text.push_str(&member.name.text);
                    text.push_str(": ");
                    text.push_str(&member.name.text);
                }
                text.push_str("});\n");
                text.push_str("    }\n");
            }
        }
    }
    text.push_str("}\n");
    Ok(RenderSuccessReturn {
        source: SoliditySourceText { text },
    })
}
