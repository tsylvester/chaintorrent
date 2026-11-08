#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams, IChainForms};
use core::convert::Infallible;
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
use encoding::{EncodeErrorReturn, EncodingIdentifier, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
use hash_to_scalar::IHashToScalarAdapter;
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
use kem::ICredentialKemAdapter;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::IDeliveryProofAdapter;
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use random::IRandomSourceAdapter;
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use std::path::PathBuf;

use super::super::factory::provides::{
    GENERATE_INTERFACE_VERSION, GenerateDeclaration, VectorKind, VectorKindDeclaration,
    VectorKindSettings, VectorKindSizing,
};
use super::challenge_vectors::provides::{
    ChallengeVectorMemberNamesErrorReturn, ChallengeVectorsErrorReturn, ChallengeVectorsFn,
};
use super::constants::provides::{ConstantsErrorReturn, ConstantsFn};
use super::group_vectors::provides::{
    GroupVectorsErrorReturn, GroupVectorsFn, VectorRecordMemberNamesErrorReturn,
};
use super::mapping_vectors::provides::{
    MappingVectorMemberNamesErrorReturn, MappingVectorsErrorReturn, MappingVectorsFn,
};
use super::possession_vectors::provides::{
    PossessionVectorMemberNamesErrorReturn, PossessionVectorsErrorReturn, PossessionVectorsFn,
};
use super::proof_vectors::provides::{
    ProofVectorMemberNamesErrorReturn, ProofVectorsErrorReturn, ProofVectorsFn,
};
use super::rejection_vectors::provides::{
    RejectionVectorMemberNamesErrorReturn, RejectionVectorsErrorReturn, RejectionVectorsFn,
};
use super::render::provides::{
    RenderFn, SolidityAbiTypeTryNewErrorReturn, SolidityCompilerVersion, SolidityConstantName,
    SolidityLibraryEntriesTryNewErrorReturn, SolidityLibraryName, SolidityMemberName,
    SolidityRecordDeclarationsTryNewErrorReturn, SolidityRecordMembersTryNewErrorReturn,
    SolidityStringLiteralTryNewErrorReturn, SolidityTypeName, SpdxLicenseIdentifier,
};
#[cfg(any(test, feature = "mocks"))]
use crate::delivery::provides::AdmittedKemIdentity;

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
pub struct SolidityCurveLibraryNames {
    pub constants_library: SolidityLibraryNames,
    pub records_library: SolidityLibraryNames,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityCurveLibraries {
    pub bn254: SolidityCurveLibraryNames,
    pub bls12_381: SolidityCurveLibraryNames,
    #[cfg(any(test, feature = "mocks"))]
    pub mock: SolidityCurveLibraryNames,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityCurveDirectories {
    pub bn254: GeneratedFileName,
    pub bls12_381: GeneratedFileName,
    #[cfg(any(test, feature = "mocks"))]
    pub mock: GeneratedFileName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidityGenerateSettings {
    pub kinds: Vec<SolidityVectorKindSettings>,
    pub curve_libraries: SolidityCurveLibraries,
    pub foundry_root: PathBuf,
    pub generated_directory: GeneratedDirectory,
    pub curve_directories: SolidityCurveDirectories,
    pub compiler_version: SolidityCompilerVersion,
    pub license: SpdxLicenseIdentifier,
}

pub struct SolidityGenerateConstructorParams<
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
    pub render: RenderFn,
    pub constants: ConstantsFn<'a, P, E>,
    pub group_vectors: GroupVectorsFn<'a, P>,
    pub rejection_vectors: RejectionVectorsFn<'a, P>,
    pub mapping_vectors: MappingVectorsFn<'a, P, K>,
    pub possession_vectors: PossessionVectorsFn<'a, P, A>,
    pub challenge_vectors: ChallengeVectorsFn<'a, P, E, <D as IDeliveryProofAdapter>::Forms>,
    pub proof_vectors: ProofVectorsFn<'a, P, K, A, D>,
    pub statement_version: u16,
    pub settings: SolidityGenerateSettings,
    pub sizes: Vec<VectorKindSettings>,
}

pub struct SolidityGenerate<
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
    pub(super) render: RenderFn,
    pub(super) constants: ConstantsFn<'a, P, E>,
    pub(super) group_vectors: GroupVectorsFn<'a, P>,
    pub(super) rejection_vectors: RejectionVectorsFn<'a, P>,
    pub(super) mapping_vectors: MappingVectorsFn<'a, P, K>,
    pub(super) possession_vectors: PossessionVectorsFn<'a, P, A>,
    pub(super) challenge_vectors: ChallengeVectorsFn<'a, P, E, <D as IDeliveryProofAdapter>::Forms>,
    pub(super) proof_vectors: ProofVectorsFn<'a, P, K, A, D>,
    pub(super) statement_version: u16,
    pub(super) settings: SolidityGenerateSettings,
    pub(super) sizes: Vec<VectorKindSettings>,
}

pub type SolidityGenerateTryNewReturn<'a, P, E, K, A, D> =
    Result<SolidityGenerate<'a, P, E, K, A, D>, Infallible>;

pub const SOLIDITY_GENERATE_DECLARATION: GenerateDeclaration = GenerateDeclaration {
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
    encoding_identifier: EncodingIdentifier::EthereumAbiV1,
};

#[derive(Debug)]
pub enum SolidityGenerateErrorReturn {
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
    GroupMembers(VectorRecordMemberNamesErrorReturn),
    RejectionMembers(RejectionVectorMemberNamesErrorReturn),
    MappingMembers(MappingVectorMemberNamesErrorReturn),
    PossessionMembers(PossessionVectorMemberNamesErrorReturn),
    ChallengeMembers(ChallengeVectorMemberNamesErrorReturn),
    ProofMembers(ProofVectorMemberNamesErrorReturn),
    MemberCount {
        kind: VectorKind,
        expected: usize,
        actual: usize,
    },
    AbiType(SolidityAbiTypeTryNewErrorReturn),
    RecordMembers(SolidityRecordMembersTryNewErrorReturn),
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

#[cfg(any(test, feature = "mocks"))]
pub trait ISolidityGenerateCaseConsumer {
    type Output;

    fn consume_solidity_generate_case<P, E, F, K, A, D>(
        &self,
        pairing: &P,
        hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &E,
        decoder: &E,
        kem: &K,
        key_agreement: &A,
        delivery_proof: &D,
        random: &dyn IRandomSourceAdapter,
        identities: Vec<AdmittedKemIdentity<'_>>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        D: IDeliveryProofAdapter<Pairing = P, Forms = F>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct SolidityGenerateCaseDeps<C: ISolidityGenerateCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct SolidityGenerateCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct SolidityGenerateCasePayload<'a> {
    pub canonicals: &'a [&'a [u8]],
    pub roots: &'a [&'a Root],
}

#[cfg(any(test, feature = "mocks"))]
pub struct SolidityGenerateCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum SolidityGenerateCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateKeyAgreement(CreateKeyAgreementErrorReturn),
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type SolidityGenerateCaseReturn<O> =
    Result<SolidityGenerateCaseSuccessReturn<O>, SolidityGenerateCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunSolidityGenerateCaseFn<C> =
    fn(
        &SolidityGenerateCaseDeps<C>,
        SolidityGenerateCaseParams,
        SolidityGenerateCasePayload<'_>,
    ) -> SolidityGenerateCaseReturn<<C as ISolidityGenerateCaseConsumer>::Output>;
