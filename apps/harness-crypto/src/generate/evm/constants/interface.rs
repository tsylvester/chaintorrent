use envelope::KeyAgreementDeclaration;
use kem::KemDeclaration;
use pairing::IPairingReference;
use proof::DeliveryProofDeclaration;

use super::super::render::provides::{
    SolidityConstantNameTryNewErrorReturn, SolidityLibraryEntry,
    SolidityStringLiteralTryNewErrorReturn,
};

pub struct ConstantsDeps<'a, P: IPairingReference> {
    pub pairing: &'a P,
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
    StringLiteral(SolidityStringLiteralTryNewErrorReturn),
}

pub type ConstantsReturn = Result<ConstantsSuccessReturn, ConstantsErrorReturn>;

pub type ConstantsFn<'a, P> =
    fn(&ConstantsDeps<'a, P>, ConstantsParams, ConstantsPayload<'_>) -> ConstantsReturn;
