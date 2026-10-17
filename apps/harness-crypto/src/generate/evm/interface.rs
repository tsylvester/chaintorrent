use core::convert::Infallible;
use encoding::{EncodeErrorReturn, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::IDeliveryProofAdapter;
use random::IRandomSourceAdapter;
use std::path::PathBuf;

use super::challenge_vectors::provides::{
    ChallengeVectorMembersErrorReturn, ChallengeVectorsErrorReturn,
};
use super::constants::provides::ConstantsErrorReturn;
use super::group_vectors::provides::{GroupVectorsErrorReturn, VectorRecordMembersErrorReturn};
use super::mapping_vectors::provides::{
    MappingVectorMembersErrorReturn, MappingVectorsErrorReturn,
};
use super::possession_vectors::provides::{
    PossessionVectorMembersErrorReturn, PossessionVectorsErrorReturn,
};
use super::proof_vectors::provides::{ProofVectorMembersErrorReturn, ProofVectorsErrorReturn};
use super::rejection_vectors::provides::{
    RejectionVectorMembersErrorReturn, RejectionVectorsErrorReturn,
};
use super::render::provides::{
    SolidityCompilerVersion, SolidityConstantName, SolidityLibraryEntriesTryNewErrorReturn,
    SolidityLibraryName, SolidityMemberName, SolidityRecordDeclarationsTryNewErrorReturn,
    SolidityStringLiteralTryNewErrorReturn, SolidityTypeName, SpdxLicenseIdentifier,
};

use super::super::factory::provides::{
    GENERATE_INTERFACE_VERSION, GenerateDeclaration, VectorKind, VectorKindDeclaration,
    VectorKindSettings, VectorKindSizing,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmVectorKindNames {
    pub file_name: GeneratedFileName,
    pub struct_name: SolidityTypeName,
    pub decode_function_name: SolidityMemberName,
    pub path_constant_name: SolidityConstantName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmVectorKindSettings {
    pub kind: VectorKind,
    pub names: EvmVectorKindNames,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmLibraryNames {
    pub file_name: GeneratedFileName,
    pub library_name: SolidityLibraryName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmCurveDirectories {
    pub bn254: GeneratedFileName,
    pub bls12_381: GeneratedFileName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmGenerateSettings {
    pub kinds: Vec<EvmVectorKindSettings>,
    pub constants_library: EvmLibraryNames,
    pub records_library: EvmLibraryNames,
    pub foundry_root: PathBuf,
    pub generated_directory: GeneratedDirectory,
    pub curve_directories: EvmCurveDirectories,
    pub compiler_version: SolidityCompilerVersion,
    pub license: SpdxLicenseIdentifier,
}

pub struct EvmGenerateConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub scope: IdentityScope,
    pub statement_version: u16,
    pub settings: EvmGenerateSettings,
    pub sizes: Vec<VectorKindSettings>,
}

pub struct EvmGenerate<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) kem: &'a K,
    pub(super) key_agreement: &'a A,
    pub(super) delivery_proof: &'a D,
    pub(super) scope: IdentityScope,
    pub(super) statement_version: u16,
    pub(super) settings: EvmGenerateSettings,
    pub(super) sizes: Vec<VectorKindSettings>,
}

pub type EvmGenerateTryNewReturn<'a, P, E, K, A, D> =
    Result<EvmGenerate<'a, P, E, K, A, D>, Infallible>;

pub const EVM_GENERATE_DECLARATION: GenerateDeclaration = GenerateDeclaration {
    kinds: &[
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
    ],
    adapter_version: 1,
    interface_version: GENERATE_INTERFACE_VERSION,
};

#[derive(Debug)]
pub enum EvmGenerateErrorReturn {
    MissingKindSettings {
        kind: VectorKind,
    },
    Constants(ConstantsErrorReturn),
    GroupVectors(GroupVectorsErrorReturn),
    RejectionVectors(RejectionVectorsErrorReturn),
    MappingVectors(MappingVectorsErrorReturn),
    PossessionVectors(PossessionVectorsErrorReturn),
    ChallengeVectors(ChallengeVectorsErrorReturn),
    ProofVectors(ProofVectorsErrorReturn),
    GroupMembers(VectorRecordMembersErrorReturn),
    RejectionMembers(RejectionVectorMembersErrorReturn),
    MappingMembers(MappingVectorMembersErrorReturn),
    PossessionMembers(PossessionVectorMembersErrorReturn),
    ChallengeMembers(ChallengeVectorMembersErrorReturn),
    ProofMembers(ProofVectorMembersErrorReturn),
    Encode(EncodeErrorReturn),
    Entries(SolidityLibraryEntriesTryNewErrorReturn),
    Declarations(SolidityRecordDeclarationsTryNewErrorReturn),
    Path(SolidityStringLiteralTryNewErrorReturn),
    CreateDirectory {
        path: PathBuf,
        error: std::io::Error,
    },
    WriteFile {
        path: PathBuf,
        error: std::io::Error,
    },
}
