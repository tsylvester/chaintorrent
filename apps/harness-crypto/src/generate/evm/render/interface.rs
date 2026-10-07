use core::convert::Infallible;
use encoding::CanonicalFieldKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityLibraryName {
    pub(super) text: String,
}

pub struct SolidityLibraryNameConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityLibraryNameTryNewErrorReturn {
    Empty,
    LeadingCharacter { byte: u8 },
    InvalidCharacter { index: usize, byte: u8 },
}

pub type SolidityLibraryNameTryNewReturn =
    Result<SolidityLibraryName, SolidityLibraryNameTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityConstantName {
    pub(super) text: String,
}

pub struct SolidityConstantNameConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityConstantNameTryNewErrorReturn {
    Empty,
    LeadingCharacter { byte: u8 },
    InvalidCharacter { index: usize, byte: u8 },
}

pub type SolidityConstantNameTryNewReturn =
    Result<SolidityConstantName, SolidityConstantNameTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpdxLicenseIdentifier {
    pub(super) text: String,
}

pub struct SpdxLicenseIdentifierConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpdxLicenseIdentifierTryNewErrorReturn {
    Empty,
    InvalidCharacter { index: usize, byte: u8 },
}

pub type SpdxLicenseIdentifierTryNewReturn =
    Result<SpdxLicenseIdentifier, SpdxLicenseIdentifierTryNewErrorReturn>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolidityCompilerVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityStringLiteral {
    pub(super) text: String,
}

pub struct SolidityStringLiteralConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityStringLiteralTryNewErrorReturn {
    InvalidCharacter { index: usize, byte: u8 },
}

pub type SolidityStringLiteralTryNewReturn =
    Result<SolidityStringLiteral, SolidityStringLiteralTryNewErrorReturn>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolidityUint256 {
    pub(super) big_endian: [u8; 32],
}

pub struct SolidityUint256ConstructorParams {
    pub big_endian: [u8; 32],
}

pub type SolidityUint256TryNewReturn = Result<SolidityUint256, Infallible>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolidityConstantValue {
    Bytes(Vec<u8>),
    Uint256(SolidityUint256),
    Uint64(u64),
    Uint16(u16),
    Bool(bool),
    String(SolidityStringLiteral),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityLibraryEntry {
    pub name: SolidityConstantName,
    pub value: SolidityConstantValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityLibraryEntries {
    pub(super) entries: Vec<SolidityLibraryEntry>,
}

pub struct SolidityLibraryEntriesConstructorParams {
    pub entries: Vec<SolidityLibraryEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityLibraryEntriesTryNewErrorReturn {
    Empty,
    DuplicateName {
        first_index: usize,
        duplicate_index: usize,
    },
}

pub type SolidityLibraryEntriesTryNewReturn =
    Result<SolidityLibraryEntries, SolidityLibraryEntriesTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityTypeName {
    pub(super) text: String,
}

pub struct SolidityTypeNameConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityTypeNameTryNewErrorReturn {
    Empty,
    LeadingCharacter { byte: u8 },
    InvalidCharacter { index: usize, byte: u8 },
}

pub type SolidityTypeNameTryNewReturn = Result<SolidityTypeName, SolidityTypeNameTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityMemberName {
    pub(super) text: String,
}

pub struct SolidityMemberNameConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityMemberNameTryNewErrorReturn {
    Empty,
    LeadingCharacter { byte: u8 },
    InvalidCharacter { index: usize, byte: u8 },
}

pub type SolidityMemberNameTryNewReturn =
    Result<SolidityMemberName, SolidityMemberNameTryNewErrorReturn>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolidityAbiType {
    pub(super) kind: CanonicalFieldKind,
}

pub struct SolidityAbiTypeConstructorParams {
    pub kind: CanonicalFieldKind,
}

pub type SolidityAbiTypeTryNewReturn = Result<SolidityAbiType, Infallible>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityRecordMember {
    pub name: SolidityMemberName,
    pub kind: CanonicalFieldKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityRecordMembers {
    pub(super) members: Vec<SolidityRecordMember>,
}

pub struct SolidityRecordMembersConstructorParams {
    pub members: Vec<SolidityRecordMember>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityRecordMembersTryNewErrorReturn {
    TooFew {
        actual: usize,
    },
    ReservedName {
        index: usize,
    },
    DuplicateName {
        first_index: usize,
        duplicate_index: usize,
    },
}

pub type SolidityRecordMembersTryNewReturn =
    Result<SolidityRecordMembers, SolidityRecordMembersTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityRecordDeclaration {
    pub struct_name: SolidityTypeName,
    pub members: SolidityRecordMembers,
    pub decode_function_name: SolidityMemberName,
    pub path_constant_name: SolidityConstantName,
    pub path: SolidityStringLiteral,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityRecordDeclarations {
    pub(super) declarations: Vec<SolidityRecordDeclaration>,
}

pub struct SolidityRecordDeclarationsConstructorParams {
    pub declarations: Vec<SolidityRecordDeclaration>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidityRecordDeclarationsTryNewErrorReturn {
    Empty,
    DuplicateStructName {
        first_index: usize,
        duplicate_index: usize,
    },
    DuplicateDecodeFunctionName {
        first_index: usize,
        duplicate_index: usize,
    },
    DuplicatePathConstantName {
        first_index: usize,
        duplicate_index: usize,
    },
}

pub type SolidityRecordDeclarationsTryNewReturn =
    Result<SolidityRecordDeclarations, SolidityRecordDeclarationsTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolidityLibraryBody {
    Constants(SolidityLibraryEntries),
    Records(SolidityRecordDeclarations),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SoliditySourceText {
    pub(super) text: String,
}

pub struct RenderDeps;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderParams {
    pub license: SpdxLicenseIdentifier,
    pub compiler_version: SolidityCompilerVersion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderPayload {
    pub library_name: SolidityLibraryName,
    pub body: SolidityLibraryBody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderSuccessReturn {
    pub source: SoliditySourceText,
}

pub type RenderReturn = Result<RenderSuccessReturn, Infallible>;

pub type RenderFn = fn(&RenderDeps, RenderParams, RenderPayload) -> RenderReturn;
