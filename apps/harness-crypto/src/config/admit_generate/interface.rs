use pairing::PairingCurve;

use super::super::layers::provides::GenerateTable;
use crate::generate::provides::{
    GenerateConcrete, GeneratedDirectoryTryNewErrorReturn, GeneratedFileNameTryNewErrorReturn,
    MsmTermCountTryNewErrorReturn, SolidityConstantNameTryNewErrorReturn,
    SolidityLibraryNameTryNewErrorReturn, SolidityMemberNameTryNewErrorReturn,
    SolidityTypeNameTryNewErrorReturn, SpdxLicenseIdentifierTryNewErrorReturn,
    VectorCountTryNewErrorReturn, VectorKindConfiguration,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedGenerate {
    pub concrete: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompilerVersionErrorReturn {
    Parts { count: usize },
    Part { index: usize, text: String },
}

#[derive(Clone, Copy, Debug)]
pub struct AdmitGenerateDeps;

#[derive(Clone, Copy, Debug)]
pub struct AdmitGenerateParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitGeneratePayload {
    pub generate: GenerateTable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitGenerateSuccessReturn {
    pub admitted: AdmittedGenerate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitGenerateErrorReturn {
    Concrete {
        text: String,
    },
    Kind {
        text: String,
    },
    Count {
        kind: String,
        error: VectorCountTryNewErrorReturn,
    },
    Terms {
        kind: String,
        error: MsmTermCountTryNewErrorReturn,
    },
    File {
        kind: String,
        error: GeneratedFileNameTryNewErrorReturn,
    },
    StructName {
        kind: String,
        error: SolidityTypeNameTryNewErrorReturn,
    },
    DecodeFunction {
        kind: String,
        error: SolidityMemberNameTryNewErrorReturn,
    },
    PathConstant {
        kind: String,
        error: SolidityConstantNameTryNewErrorReturn,
    },
    ConstantsLibraryFile {
        error: GeneratedFileNameTryNewErrorReturn,
    },
    ConstantsLibraryName {
        error: SolidityLibraryNameTryNewErrorReturn,
    },
    RecordsLibraryFile {
        error: GeneratedFileNameTryNewErrorReturn,
    },
    RecordsLibraryName {
        error: SolidityLibraryNameTryNewErrorReturn,
    },
    GeneratedDirectorySegment {
        index: usize,
        error: GeneratedFileNameTryNewErrorReturn,
    },
    GeneratedDirectory {
        error: GeneratedDirectoryTryNewErrorReturn,
    },
    CurveKey {
        text: String,
    },
    CurveDirectory {
        curve: PairingCurve,
        error: GeneratedFileNameTryNewErrorReturn,
    },
    MissingCurveDirectory {
        curve: PairingCurve,
    },
    CompilerVersion {
        text: String,
        error: CompilerVersionErrorReturn,
    },
    License {
        error: SpdxLicenseIdentifierTryNewErrorReturn,
    },
}

pub type AdmitGenerateReturn = Result<AdmitGenerateSuccessReturn, AdmitGenerateErrorReturn>;

pub type AdmitGenerateFn =
    fn(&AdmitGenerateDeps, AdmitGenerateParams, AdmitGeneratePayload) -> AdmitGenerateReturn;
