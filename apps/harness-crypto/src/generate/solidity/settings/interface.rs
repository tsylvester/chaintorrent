use std::collections::BTreeMap;
use std::path::PathBuf;

use chain::ChainFormsIdentifier;
use encoding::EncodingIdentifier;

use super::super::super::factory::provides::{
    GENERATE_INTERFACE_VERSION, GenerateConcreteKind, GenerateDeclaration, SettingBounds,
    SettingDisposition, SettingStatement, SettingVersion, VectorKind, VectorKindDeclaration,
    VectorKindSizing,
};
use super::super::render::provides::{
    SolidityCompilerVersion, SolidityConstantName, SolidityConstantNameTryNewErrorReturn,
    SolidityLibraryName, SolidityLibraryNameTryNewErrorReturn, SolidityMemberName,
    SolidityMemberNameTryNewErrorReturn, SolidityTypeName, SolidityTypeNameTryNewErrorReturn,
    SpdxLicenseIdentifier, SpdxLicenseIdentifierTryNewErrorReturn,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedFileName {
    pub(super) text: String,
}

pub struct GeneratedFileNameConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeneratedFileNameTryNewErrorReturn {
    Empty,
    Reserved,
    InvalidCharacter { index: usize, byte: u8 },
}

pub type GeneratedFileNameTryNewReturn =
    Result<GeneratedFileName, GeneratedFileNameTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedDirectory {
    pub(super) segments: Vec<GeneratedFileName>,
}

pub struct GeneratedDirectoryConstructorParams {
    pub segments: Vec<GeneratedFileName>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeneratedDirectoryTryNewErrorReturn {
    Empty,
}

pub type GeneratedDirectoryTryNewReturn =
    Result<GeneratedDirectory, GeneratedDirectoryTryNewErrorReturn>;

pub const SOLIDITY_VECTOR_KINDS: &'static [VectorKind] = &[
    VectorKind::G1Addition,
    VectorKind::G1Multiplication,
    VectorKind::PairingCheck,
    VectorKind::G2Addition,
    VectorKind::G1MultiScalarMultiplication,
    VectorKind::G2MultiScalarMultiplication,
    VectorKind::DecodeRejection,
    VectorKind::HashToScalar,
    VectorKind::IdentityMapping,
    VectorKind::Possession,
    VectorKind::Challenge,
    VectorKind::MintProof,
    VectorKind::TransferProof,
    VectorKind::CellValues,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityVectorKindNames {
    pub file_name: GeneratedFileName,
    pub struct_name: SolidityTypeName,
    pub decode_function_name: SolidityMemberName,
    pub path_constant_name: SolidityConstantName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityVectorKindSettings {
    pub kind: VectorKind,
    pub names: SolidityVectorKindNames,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityLibraryNames {
    pub file_name: GeneratedFileName,
    pub library_name: SolidityLibraryName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityGenerateSettings {
    pub kinds: Vec<SolidityVectorKindSettings>,
    pub constants_library: SolidityLibraryNames,
    pub records_library: SolidityLibraryNames,
    pub foundry_root: PathBuf,
    pub generated_directory: GeneratedDirectory,
    pub compiler_version: SolidityCompilerVersion,
    pub license: SpdxLicenseIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityVectorKindNamesDocument {
    pub file: Option<String>,
    pub struct_name: Option<String>,
    pub decode_function: Option<String>,
    pub path_constant: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityLibraryNamesDocument {
    pub file: Option<String>,
    pub library: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityGenerateSettingsDocument {
    pub kinds: Option<BTreeMap<String, SolidityVectorKindNamesDocument>>,
    pub constants_library: Option<SolidityLibraryNamesDocument>,
    pub records_library: Option<SolidityLibraryNamesDocument>,
    pub foundry_root: Option<String>,
    pub generated_directory: Option<Vec<String>>,
    pub compiler_version: Option<String>,
    pub license: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolidityGenerateSettingsDocumentErrorReturn {
    NotATable { path: String },
    UnknownKey { path: String, key: String },
    NotText { path: String },
    NotTextList { path: String },
}

pub type SolidityGenerateSettingsDocumentTryFromReturn =
    Result<SolidityGenerateSettingsDocument, SolidityGenerateSettingsDocumentErrorReturn>;

pub struct AdmitSolidityGenerateSettingsDeps<'a> {
    pub declaration: &'a GenerateDeclaration,
}

pub struct AdmitSolidityGenerateSettingsParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitSolidityGenerateSettingsPayload {
    pub document: SolidityGenerateSettingsDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitSolidityGenerateSettingsSuccessReturn {
    pub settings: SolidityGenerateSettings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitSolidityGenerateSettingsErrorReturn {
    UndeclaredSetting {
        setting: String,
    },
    UnknownKind {
        text: String,
    },
    FixedSetting {
        setting: String,
        supplied: String,
    },
    OutOfBounds {
        setting: String,
        supplied: String,
    },
    CompilerVersionParts {
        setting: String,
        text: String,
    },
    CompilerVersionPart {
        setting: String,
        index: usize,
        text: String,
    },
    FileName {
        setting: String,
        error: GeneratedFileNameTryNewErrorReturn,
    },
    TypeName {
        setting: String,
        error: SolidityTypeNameTryNewErrorReturn,
    },
    MemberName {
        setting: String,
        error: SolidityMemberNameTryNewErrorReturn,
    },
    ConstantName {
        setting: String,
        error: SolidityConstantNameTryNewErrorReturn,
    },
    LibraryName {
        setting: String,
        error: SolidityLibraryNameTryNewErrorReturn,
    },
    DirectorySegment {
        index: usize,
        error: GeneratedFileNameTryNewErrorReturn,
    },
    Directory {
        error: GeneratedDirectoryTryNewErrorReturn,
    },
    License {
        error: SpdxLicenseIdentifierTryNewErrorReturn,
    },
}

pub type AdmitSolidityGenerateSettingsReturn =
    Result<AdmitSolidityGenerateSettingsSuccessReturn, AdmitSolidityGenerateSettingsErrorReturn>;

pub type AdmitSolidityGenerateSettingsFn<'a> = fn(
    &AdmitSolidityGenerateSettingsDeps<'a>,
    AdmitSolidityGenerateSettingsParams,
    AdmitSolidityGenerateSettingsPayload,
) -> AdmitSolidityGenerateSettingsReturn;

const SOLIDITY_VECTOR_KIND_DECLARATIONS: &'static [VectorKindDeclaration] = &[
    VectorKindDeclaration {
        kind: VectorKind::G1Addition,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::G1Multiplication,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::PairingCheck,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::G2Addition,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: true,
    },
    VectorKindDeclaration {
        kind: VectorKind::G1MultiScalarMultiplication,
        sizing: VectorKindSizing::DrawnTerms,
        requires_second_group_arithmetic: true,
    },
    VectorKindDeclaration {
        kind: VectorKind::G2MultiScalarMultiplication,
        sizing: VectorKindSizing::DrawnTerms,
        requires_second_group_arithmetic: true,
    },
    VectorKindDeclaration {
        kind: VectorKind::DecodeRejection,
        sizing: VectorKindSizing::Enumerated,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::HashToScalar,
        sizing: VectorKindSizing::DrawnMessages,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::IdentityMapping,
        sizing: VectorKindSizing::OverConfiguredInputs,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::Possession,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::Challenge,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::MintProof,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::TransferProof,
        sizing: VectorKindSizing::Drawn,
        requires_second_group_arithmetic: false,
    },
    VectorKindDeclaration {
        kind: VectorKind::CellValues,
        sizing: VectorKindSizing::Enumerated,
        requires_second_group_arithmetic: false,
    },
];

const fn unbounded(name: &'static str, default: &'static str) -> SettingStatement {
    SettingStatement {
        name,
        disposition: SettingDisposition::Settable {
            bounds: SettingBounds::Unbounded,
            default,
        },
    }
}

const SOLIDITY_SETTING_STATEMENTS: &'static [SettingStatement] = &[
    SettingStatement {
        name: "compiler_version",
        disposition: SettingDisposition::Settable {
            bounds: SettingBounds::VersionWithin {
                at_least: SettingVersion {
                    major: 0,
                    minor: 8,
                    patch: 38,
                },
                below: SettingVersion {
                    major: 0,
                    minor: 9,
                    patch: 0,
                },
            },
            default: "0.8.38",
        },
    },
    unbounded("license", "UNLICENSED"),
    unbounded("foundry_root", "contracts/evm"),
    unbounded("generated_directory", "generated"),
    unbounded("constants_library.file", "Constants.sol"),
    unbounded("constants_library.library", "Constants"),
    unbounded("records_library.file", "VectorRecords.sol"),
    unbounded("records_library.library", "VectorRecords"),
    unbounded("kinds.G1Addition.file", "g1_addition.txt"),
    unbounded("kinds.G1Addition.struct_name", "G1AdditionVector"),
    unbounded("kinds.G1Addition.decode_function", "decodeG1Addition"),
    unbounded("kinds.G1Addition.path_constant", "G1_ADDITION_PATH"),
    unbounded("kinds.G1Multiplication.file", "g1_multiplication.txt"),
    unbounded(
        "kinds.G1Multiplication.struct_name",
        "G1MultiplicationVector",
    ),
    unbounded(
        "kinds.G1Multiplication.decode_function",
        "decodeG1Multiplication",
    ),
    unbounded(
        "kinds.G1Multiplication.path_constant",
        "G1_MULTIPLICATION_PATH",
    ),
    unbounded("kinds.PairingCheck.file", "pairing_check.txt"),
    unbounded("kinds.PairingCheck.struct_name", "PairingCheckVector"),
    unbounded("kinds.PairingCheck.decode_function", "decodePairingCheck"),
    unbounded("kinds.PairingCheck.path_constant", "PAIRING_CHECK_PATH"),
    unbounded("kinds.G2Addition.file", "g2_addition.txt"),
    unbounded("kinds.G2Addition.struct_name", "G2AdditionVector"),
    unbounded("kinds.G2Addition.decode_function", "decodeG2Addition"),
    unbounded("kinds.G2Addition.path_constant", "G2_ADDITION_PATH"),
    unbounded(
        "kinds.G1MultiScalarMultiplication.file",
        "g1_multi_scalar_multiplication.txt",
    ),
    unbounded(
        "kinds.G1MultiScalarMultiplication.struct_name",
        "G1MultiScalarMultiplicationVector",
    ),
    unbounded(
        "kinds.G1MultiScalarMultiplication.decode_function",
        "decodeG1MultiScalarMultiplication",
    ),
    unbounded(
        "kinds.G1MultiScalarMultiplication.path_constant",
        "G1_MULTI_SCALAR_MULTIPLICATION_PATH",
    ),
    unbounded(
        "kinds.G2MultiScalarMultiplication.file",
        "g2_multi_scalar_multiplication.txt",
    ),
    unbounded(
        "kinds.G2MultiScalarMultiplication.struct_name",
        "G2MultiScalarMultiplicationVector",
    ),
    unbounded(
        "kinds.G2MultiScalarMultiplication.decode_function",
        "decodeG2MultiScalarMultiplication",
    ),
    unbounded(
        "kinds.G2MultiScalarMultiplication.path_constant",
        "G2_MULTI_SCALAR_MULTIPLICATION_PATH",
    ),
    unbounded("kinds.DecodeRejection.file", "decode_rejection.txt"),
    unbounded("kinds.DecodeRejection.struct_name", "DecodeRejectionVector"),
    unbounded(
        "kinds.DecodeRejection.decode_function",
        "decodeDecodeRejection",
    ),
    unbounded(
        "kinds.DecodeRejection.path_constant",
        "DECODE_REJECTION_PATH",
    ),
    unbounded("kinds.HashToScalar.file", "hash_to_scalar.txt"),
    unbounded("kinds.HashToScalar.struct_name", "HashToScalarVector"),
    unbounded("kinds.HashToScalar.decode_function", "decodeHashToScalar"),
    unbounded("kinds.HashToScalar.path_constant", "HASH_TO_SCALAR_PATH"),
    unbounded("kinds.IdentityMapping.file", "identity_mapping.txt"),
    unbounded("kinds.IdentityMapping.struct_name", "IdentityMappingVector"),
    unbounded(
        "kinds.IdentityMapping.decode_function",
        "decodeIdentityMapping",
    ),
    unbounded(
        "kinds.IdentityMapping.path_constant",
        "IDENTITY_MAPPING_PATH",
    ),
    unbounded("kinds.Possession.file", "possession.txt"),
    unbounded("kinds.Possession.struct_name", "PossessionVector"),
    unbounded("kinds.Possession.decode_function", "decodePossession"),
    unbounded("kinds.Possession.path_constant", "POSSESSION_PATH"),
    unbounded("kinds.Challenge.file", "challenge.txt"),
    unbounded("kinds.Challenge.struct_name", "ChallengeVector"),
    unbounded("kinds.Challenge.decode_function", "decodeChallenge"),
    unbounded("kinds.Challenge.path_constant", "CHALLENGE_PATH"),
    unbounded("kinds.MintProof.file", "mint_proof.txt"),
    unbounded("kinds.MintProof.struct_name", "MintProofVector"),
    unbounded("kinds.MintProof.decode_function", "decodeMintProof"),
    unbounded("kinds.MintProof.path_constant", "MINT_PROOF_PATH"),
    unbounded("kinds.TransferProof.file", "transfer_proof.txt"),
    unbounded("kinds.TransferProof.struct_name", "TransferProofVector"),
    unbounded("kinds.TransferProof.decode_function", "decodeTransferProof"),
    unbounded("kinds.TransferProof.path_constant", "TRANSFER_PROOF_PATH"),
    unbounded("kinds.CellValues.file", "cell_values.txt"),
    unbounded("kinds.CellValues.struct_name", "CellValuesVector"),
    unbounded("kinds.CellValues.decode_function", "decodeCellValues"),
    unbounded("kinds.CellValues.path_constant", "CELL_VALUES_PATH"),
];

pub const SOLIDITY_GENERATE_DECLARATION: GenerateDeclaration = GenerateDeclaration {
    identifier: GenerateConcreteKind::Solidity,
    name: "Solidity",
    kinds: SOLIDITY_VECTOR_KIND_DECLARATIONS,
    adapter_version: 1,
    interface_version: GENERATE_INTERFACE_VERSION,
    encoding_identifier: EncodingIdentifier::EthereumAbiV1,
    chain_forms_identifier: ChainFormsIdentifier::EvmV1,
    contract_suite: "evm",
    settings: SOLIDITY_SETTING_STATEMENTS,
};
