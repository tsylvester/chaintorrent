#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams, IChainForms};
use core::convert::Infallible;
use encoding::{CanonicalFieldKind, CanonicalFieldReaderErrorReturn, IEncoderAdapter};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
use envelope::KeyAgreementDeclaration;
use kem::KemDeclaration;
use pairing::IPairingReference;
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic};
use proof::DeliveryProofDeclaration;

use super::super::render::provides::{
    SolidityAbiTypeTryNewErrorReturn, SolidityConstantNameTryNewErrorReturn, SolidityLibraryEntry,
    SolidityMemberName, SolidityMemberNameTryNewErrorReturn,
    SolidityStringLiteralTryNewErrorReturn,
};

pub struct ConstantsDeps<'a, P: IPairingReference, E: IEncoderAdapter> {
    pub pairing: &'a P,
    pub encoder: &'a E,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantsParams {
    pub statement_version: u16,
}

pub struct ConstantsPayload<'a> {
    pub kem: &'a KemDeclaration,
    pub key_agreement: &'a KeyAgreementDeclaration,
    pub delivery_proof: &'a DeliveryProofDeclaration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellValues {
    pub curve_name: String,
    pub precompile_encoding: String,
    pub scalar_field_order: [u8; 32],
    pub g1_generator: Vec<u8>,
    pub g2_generator: Vec<u8>,
    pub second_group_arithmetic: bool,
}

pub const CELL_VALUES_FIELD_COUNT: usize = 6;

pub const CELL_VALUES_MEMBER_NAMES: [&str; CELL_VALUES_FIELD_COUNT] = [
    "curveName",
    "precompileEncoding",
    "scalarFieldOrder",
    "g1Generator",
    "g2Generator",
    "secondGroupArithmetic",
];

pub const CELL_VALUES_FIELD_KINDS: [CanonicalFieldKind; CELL_VALUES_FIELD_COUNT] = [
    CanonicalFieldKind::Text,
    CanonicalFieldKind::Text,
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::Unsigned16,
];

pub struct CellValuesDescription;

pub struct CellValuesDescriptionConstructorParams;

pub type CellValuesDescriptionTryNewReturn = Result<CellValuesDescription, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellValuesFromFieldsErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
    SecondGroupArithmetic { actual: u16 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellValuesMemberNamesErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
}

pub type CellValuesMemberNamesReturn =
    Result<Vec<SolidityMemberName>, CellValuesMemberNamesErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantsSuccessReturn {
    pub entries: Vec<SolidityLibraryEntry>,
    pub cell_values: CellValues,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstantsErrorReturn {
    UnsupportedStatementVersion { version: u16 },
    ScalarFieldOrderLength { actual: usize },
    ConstantName(SolidityConstantNameTryNewErrorReturn),
    AbiType(SolidityAbiTypeTryNewErrorReturn),
    StringLiteral(SolidityStringLiteralTryNewErrorReturn),
}

pub type ConstantsReturn = Result<ConstantsSuccessReturn, ConstantsErrorReturn>;

pub type ConstantsFn<'a, P, E> =
    fn(&ConstantsDeps<'a, P, E>, ConstantsParams, ConstantsPayload<'_>) -> ConstantsReturn;

#[cfg(any(test, feature = "mocks"))]
pub trait IConstantsCaseConsumer {
    type Output;

    fn consume_constants_case<
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    >(
        &self,
        pairing: &P,
        encoder: &E,
        forms: &F,
        payload: &ConstantsCasePayload,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConstantsCaseDeps<C: IConstantsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConstantsCaseParams {
    pub pairing: CreatePairingParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConstantsCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct ConstantsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum ConstantsCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type ConstantsCaseReturn<O> = Result<ConstantsCaseSuccessReturn<O>, ConstantsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunConstantsCaseFn<C> = fn(
    &ConstantsCaseDeps<C>,
    ConstantsCaseParams,
    ConstantsCasePayload,
) -> ConstantsCaseReturn<<C as IConstantsCaseConsumer>::Output>;
