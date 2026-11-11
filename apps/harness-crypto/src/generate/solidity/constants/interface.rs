#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams, IChainForms};
use encoding::IEncoderAdapter;
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
pub struct ConstantsSuccessReturn {
    pub entries: Vec<SolidityLibraryEntry>,
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
