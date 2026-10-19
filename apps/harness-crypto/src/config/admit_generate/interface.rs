use super::super::layers::provides::GenerateDocument;
use crate::generate::provides::{
    GenerateConcrete, GeneratedDirectoryTryNewErrorReturn, GeneratedFileNameTryNewErrorReturn,
    MsmTermCountTryNewErrorReturn, SolidityConstantNameTryNewErrorReturn,
    SolidityLibraryNameTryNewErrorReturn, SolidityMemberNameTryNewErrorReturn,
    SolidityTypeNameTryNewErrorReturn, SpdxLicenseIdentifierTryNewErrorReturn,
    VectorCountTryNewErrorReturn, VectorKindConfiguration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerateField {
    Concrete,
    Kind { index: usize },
    KindFileName { index: usize },
    ConstantsLibraryFileName,
    ConstantsLibraryName,
    RecordsLibraryFileName,
    RecordsLibraryName,
    GeneratedDirectorySegment { index: usize },
    Bn254Directory,
    Bls12381Directory,
}

pub struct AdmitGenerateDeps;

pub struct AdmitGenerateParams;

pub struct AdmitGeneratePayload<'a> {
    pub generate: &'a GenerateDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitGenerateSuccessReturn {
    pub concrete: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitGenerateErrorReturn {
    UnknownValue {
        field: GenerateField,
        text: String,
    },
    VectorCount {
        index: usize,
        error: VectorCountTryNewErrorReturn,
    },
    MsmTermCount {
        index: usize,
        error: MsmTermCountTryNewErrorReturn,
    },
    FileName {
        field: GenerateField,
        error: GeneratedFileNameTryNewErrorReturn,
    },
    StructName {
        index: usize,
        error: SolidityTypeNameTryNewErrorReturn,
    },
    DecodeFunctionName {
        index: usize,
        error: SolidityMemberNameTryNewErrorReturn,
    },
    PathConstantName {
        index: usize,
        error: SolidityConstantNameTryNewErrorReturn,
    },
    LibraryName {
        field: GenerateField,
        error: SolidityLibraryNameTryNewErrorReturn,
    },
    GeneratedDirectory(GeneratedDirectoryTryNewErrorReturn),
    License(SpdxLicenseIdentifierTryNewErrorReturn),
}

pub type AdmitGenerateReturn = Result<AdmitGenerateSuccessReturn, AdmitGenerateErrorReturn>;

pub type AdmitGenerateFn =
    fn(&AdmitGenerateDeps, AdmitGenerateParams, AdmitGeneratePayload<'_>) -> AdmitGenerateReturn;
