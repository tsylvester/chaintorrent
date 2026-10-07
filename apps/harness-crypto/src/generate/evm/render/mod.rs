mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use core::convert::Infallible;
use encoding::CanonicalFieldKind;
use interface::{
    RenderDeps, RenderParams, RenderPayload, RenderReturn, RenderSuccessReturn, SolidityAbiType,
    SolidityAbiTypeConstructorParams, SolidityAbiTypeTryNewReturn, SolidityConstantName,
    SolidityConstantNameConstructorParams, SolidityConstantNameTryNewErrorReturn,
    SolidityConstantNameTryNewReturn, SolidityConstantValue, SolidityLibraryBody,
    SolidityLibraryEntries, SolidityLibraryEntriesConstructorParams,
    SolidityLibraryEntriesTryNewErrorReturn, SolidityLibraryEntriesTryNewReturn,
    SolidityLibraryName, SolidityLibraryNameConstructorParams,
    SolidityLibraryNameTryNewErrorReturn, SolidityLibraryNameTryNewReturn, SolidityMemberName,
    SolidityMemberNameConstructorParams, SolidityMemberNameTryNewErrorReturn,
    SolidityMemberNameTryNewReturn, SolidityRecordDeclarations,
    SolidityRecordDeclarationsConstructorParams, SolidityRecordDeclarationsTryNewErrorReturn,
    SolidityRecordDeclarationsTryNewReturn, SolidityRecordMembers,
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
        let bytes: Vec<u8> = params.text.bytes().collect();
        let Some(&first) = bytes.first() else {
            return Err(SolidityLibraryNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_uppercase() {
            return Err(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        if let Some(index) = bytes
            .iter()
            .enumerate()
            .skip(1)
            .find_map(|(index, byte)| (!byte.is_ascii_alphanumeric()).then_some(index))
        {
            return Err(SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
                index,
                byte: bytes[index],
            });
        }
        Ok(SolidityLibraryName { text: params.text })
    }
}

impl SolidityConstantName {
    pub fn try_new(
        params: SolidityConstantNameConstructorParams,
    ) -> SolidityConstantNameTryNewReturn {
        let bytes: Vec<u8> = params.text.bytes().collect();
        let Some(&first) = bytes.first() else {
            return Err(SolidityConstantNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_uppercase() {
            return Err(SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        if let Some(index) = bytes.iter().enumerate().skip(1).find_map(|(index, byte)| {
            (!(byte.is_ascii_uppercase() || byte.is_ascii_digit() || *byte == b'_'))
                .then_some(index)
        }) {
            return Err(SolidityConstantNameTryNewErrorReturn::InvalidCharacter {
                index,
                byte: bytes[index],
            });
        }
        Ok(SolidityConstantName { text: params.text })
    }
}

impl SolidityTypeName {
    pub fn try_new(params: SolidityTypeNameConstructorParams) -> SolidityTypeNameTryNewReturn {
        let bytes: Vec<u8> = params.text.bytes().collect();
        let Some(&first) = bytes.first() else {
            return Err(SolidityTypeNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_uppercase() {
            return Err(SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        if let Some(index) = bytes
            .iter()
            .enumerate()
            .skip(1)
            .find_map(|(index, byte)| (!byte.is_ascii_alphanumeric()).then_some(index))
        {
            return Err(SolidityTypeNameTryNewErrorReturn::InvalidCharacter {
                index,
                byte: bytes[index],
            });
        }
        Ok(SolidityTypeName { text: params.text })
    }
}

impl SolidityMemberName {
    pub fn try_new(params: SolidityMemberNameConstructorParams) -> SolidityMemberNameTryNewReturn {
        let bytes: Vec<u8> = params.text.bytes().collect();
        let Some(&first) = bytes.first() else {
            return Err(SolidityMemberNameTryNewErrorReturn::Empty);
        };
        if !first.is_ascii_lowercase() {
            return Err(SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: first });
        }
        if let Some(index) = bytes
            .iter()
            .enumerate()
            .skip(1)
            .find_map(|(index, byte)| (!byte.is_ascii_alphanumeric()).then_some(index))
        {
            return Err(SolidityMemberNameTryNewErrorReturn::InvalidCharacter {
                index,
                byte: bytes[index],
            });
        }
        Ok(SolidityMemberName { text: params.text })
    }
}

impl SpdxLicenseIdentifier {
    pub fn try_new(
        params: SpdxLicenseIdentifierConstructorParams,
    ) -> SpdxLicenseIdentifierTryNewReturn {
        let bytes: Vec<u8> = params.text.bytes().collect();
        let identifier = match bytes.last() {
            Some(b'+') => &bytes[..bytes.len() - 1],
            _ => &bytes[..],
        };
        if identifier.is_empty() {
            return Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty);
        }
        if let Some(index) = identifier.iter().enumerate().find_map(|(index, byte)| {
            (!(byte.is_ascii_alphanumeric() || *byte == b'.' || *byte == b'-')).then_some(index)
        }) {
            return Err(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter {
                index,
                byte: identifier[index],
            });
        }
        Ok(SpdxLicenseIdentifier { text: params.text })
    }
}

impl SolidityStringLiteral {
    pub fn try_new(
        params: SolidityStringLiteralConstructorParams,
    ) -> SolidityStringLiteralTryNewReturn {
        let bytes: Vec<u8> = params.text.bytes().collect();
        if let Some(index) = bytes.iter().enumerate().find_map(|(index, byte)| {
            (!(0x20..=0x7E).contains(byte) || *byte == b'"' || *byte == b'\\').then_some(index)
        }) {
            return Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter {
                index,
                byte: bytes[index],
            });
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

impl SolidityAbiType {
    pub fn try_new(params: SolidityAbiTypeConstructorParams) -> SolidityAbiTypeTryNewReturn {
        Ok(SolidityAbiType { kind: params.kind })
    }

    pub fn as_str(&self) -> &'static str {
        match self.kind {
            CanonicalFieldKind::FixedBytes32 => "bytes32",
            CanonicalFieldKind::Unsigned16 => "uint16",
            CanonicalFieldKind::Unsigned32 => "uint32",
            CanonicalFieldKind::Unsigned64 => "uint64",
            CanonicalFieldKind::Text => "string",
            CanonicalFieldKind::Bytes => "bytes",
            CanonicalFieldKind::FixedBytes20 => "bytes20",
            CanonicalFieldKind::Unsigned256 => "uint256",
        }
    }

    pub fn decodes_into_memory(&self) -> bool {
        match self.kind {
            CanonicalFieldKind::Text | CanonicalFieldKind::Bytes => true,
            CanonicalFieldKind::FixedBytes32
            | CanonicalFieldKind::Unsigned16
            | CanonicalFieldKind::Unsigned32
            | CanonicalFieldKind::Unsigned64
            | CanonicalFieldKind::FixedBytes20
            | CanonicalFieldKind::Unsigned256 => false,
        }
    }
}

impl SolidityLibraryEntries {
    pub fn try_new(
        params: SolidityLibraryEntriesConstructorParams,
    ) -> SolidityLibraryEntriesTryNewReturn {
        if params.entries.is_empty() {
            return Err(SolidityLibraryEntriesTryNewErrorReturn::Empty);
        }
        for (duplicate_index, entry) in params.entries.iter().enumerate() {
            if let Some(first_index) = params.entries[..duplicate_index]
                .iter()
                .position(|earlier| earlier.name == entry.name)
            {
                return Err(SolidityLibraryEntriesTryNewErrorReturn::DuplicateName {
                    first_index,
                    duplicate_index,
                });
            }
        }
        Ok(SolidityLibraryEntries {
            entries: params.entries,
        })
    }
}

impl SolidityRecordMembers {
    pub fn try_new(
        params: SolidityRecordMembersConstructorParams,
    ) -> SolidityRecordMembersTryNewReturn {
        if params.members.len() < 2 {
            return Err(SolidityRecordMembersTryNewErrorReturn::TooFew {
                actual: params.members.len(),
            });
        }
        if let Some(index) = params
            .members
            .iter()
            .position(|member| member.name.text == "line")
        {
            return Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index });
        }
        for (duplicate_index, member) in params.members.iter().enumerate() {
            if let Some(first_index) = params.members[..duplicate_index]
                .iter()
                .position(|earlier| earlier.name == member.name)
            {
                return Err(SolidityRecordMembersTryNewErrorReturn::DuplicateName {
                    first_index,
                    duplicate_index,
                });
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
        for (duplicate_index, declaration) in params.declarations.iter().enumerate() {
            if let Some(first_index) = params.declarations[..duplicate_index]
                .iter()
                .position(|earlier| earlier.struct_name == declaration.struct_name)
            {
                return Err(
                    SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                        first_index,
                        duplicate_index,
                    },
                );
            }
        }
        for (duplicate_index, declaration) in params.declarations.iter().enumerate() {
            if let Some(first_index) =
                params.declarations[..duplicate_index]
                    .iter()
                    .position(|earlier| {
                        earlier.decode_function_name == declaration.decode_function_name
                    })
            {
                return Err(
                    SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                        first_index,
                        duplicate_index,
                    },
                );
            }
        }
        for (duplicate_index, declaration) in params.declarations.iter().enumerate() {
            if let Some(first_index) = params.declarations[..duplicate_index]
                .iter()
                .position(|earlier| earlier.path_constant_name == declaration.path_constant_name)
            {
                return Err(
                    SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName {
                        first_index,
                        duplicate_index,
                    },
                );
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
    text.push('.');
    text.push_str(&params.compiler_version.minor.to_string());
    text.push('.');
    text.push_str(&params.compiler_version.patch.to_string());
    text.push_str(";\n\nlibrary ");
    text.push_str(&payload.library_name.text);
    text.push_str(" {\n");
    match payload.body {
        SolidityLibraryBody::Constants(entries) => {
            for entry in &entries.entries {
                match &entry.value {
                    SolidityConstantValue::Bytes(bytes) => {
                        text.push_str("    bytes internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = hex\"");
                        for byte in bytes {
                            text.push_str(&format!("{byte:02x}"));
                        }
                        text.push_str("\";\n");
                    }
                    SolidityConstantValue::Uint256(value) => {
                        text.push_str("    uint256 internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = 0x");
                        for byte in &value.big_endian {
                            text.push_str(&format!("{byte:02x}"));
                        }
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::Uint64(value) => {
                        text.push_str("    uint64 internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = ");
                        text.push_str(&value.to_string());
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::Uint16(value) => {
                        text.push_str("    uint16 internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = ");
                        text.push_str(&value.to_string());
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::Bool(value) => {
                        text.push_str("    bool internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = ");
                        text.push_str(&value.to_string());
                        text.push_str(";\n");
                    }
                    SolidityConstantValue::String(literal) => {
                        text.push_str("    string internal constant ");
                        text.push_str(&entry.name.text);
                        text.push_str(" = \"");
                        text.push_str(&literal.text);
                        text.push_str("\";\n");
                    }
                }
            }
        }
        SolidityLibraryBody::Records(declarations) => {
            for (declaration_index, declaration) in declarations.declarations.iter().enumerate() {
                if declaration_index > 0 {
                    text.push('\n');
                }
                let abis: Vec<SolidityAbiType> = declaration
                    .members
                    .members
                    .iter()
                    .map(|member| {
                        let abi: Result<SolidityAbiType, Infallible> =
                            SolidityAbiType::try_new(SolidityAbiTypeConstructorParams {
                                kind: member.kind,
                            });
                        let Ok(abi) = abi;
                        abi
                    })
                    .collect();
                text.push_str("    struct ");
                text.push_str(&declaration.struct_name.text);
                text.push_str(" {\n");
                for (member, abi) in declaration.members.members.iter().zip(&abis) {
                    text.push_str("        ");
                    text.push_str(abi.as_str());
                    text.push(' ');
                    text.push_str(&member.name.text);
                    text.push_str(";\n");
                }
                text.push_str("    }\n\n    string internal constant ");
                text.push_str(&declaration.path_constant_name.text);
                text.push_str(" = \"");
                text.push_str(&declaration.path.text);
                text.push_str("\";\n\n    function ");
                text.push_str(&declaration.decode_function_name.text);
                text.push_str("(bytes memory line) internal pure returns (");
                text.push_str(&declaration.struct_name.text);
                text.push_str(" memory) {\n        (");
                for (member_index, (member, abi)) in
                    declaration.members.members.iter().zip(&abis).enumerate()
                {
                    if member_index > 0 {
                        text.push_str(", ");
                    }
                    text.push_str(abi.as_str());
                    if abi.decodes_into_memory() {
                        text.push_str(" memory");
                    }
                    text.push(' ');
                    text.push_str(&member.name.text);
                }
                text.push_str(") = abi.decode(line, (");
                for (member_index, abi) in abis.iter().enumerate() {
                    if member_index > 0 {
                        text.push_str(", ");
                    }
                    text.push_str(abi.as_str());
                }
                text.push_str("));\n        return ");
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
                text.push_str("});\n    }\n");
            }
        }
    }
    text.push_str("}\n");
    Ok(RenderSuccessReturn {
        source: SoliditySourceText { text },
    })
}
