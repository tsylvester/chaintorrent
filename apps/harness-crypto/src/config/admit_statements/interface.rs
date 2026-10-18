use domain::{
    AssetIdentityHashTryNewErrorReturn, ParameterSetIdentifierTryNewErrorReturn,
    SuiteIdentifierTryNewErrorReturn,
};
use encoding::{CanonicalFieldKind, CanonicalFieldValue};
use proof::{DeliveryRelation, MintPurpose, TransferPurpose, UndeclaredDeliveryPurposeCode};

use super::super::layers::provides::{NetworkDocument, StatementsDocument};
use crate::generate::provides::CanonicalDeliveryContext;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkProfile {
    pub chain: CanonicalFieldValue,
    pub entitlement_contract: CanonicalFieldValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarnessStatements {
    pub mint: CanonicalDeliveryContext,
    pub transfer: CanonicalDeliveryContext,
    pub mint_purpose: MintPurpose,
    pub transfer_purpose: TransferPurpose,
    pub identities: Vec<Vec<u8>>,
    pub proof_identity: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextField {
    SuiteIdentifier,
    AssetIdentityHash,
    ParameterSetIdentifier,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    NewInterval,
    Seller,
    Buyer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatementField {
    NetworkChain,
    NetworkEntitlementContract,
    Context {
        relation: DeliveryRelation,
        field: ContextField,
    },
    Identity {
        index: usize,
    },
    ProofIdentity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexTextErrorReturn {
    MissingPrefix,
    InvalidDigit { index: usize, byte: u8 },
    OddDigitCount { count: usize },
    WrongLength { expected: usize, actual: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanonicalFieldAdmissionErrorReturn {
    UnknownKind {
        text: String,
    },
    ExpectedInteger {
        kind: CanonicalFieldKind,
    },
    ExpectedText {
        kind: CanonicalFieldKind,
    },
    OutOfRange {
        kind: CanonicalFieldKind,
        value: u64,
    },
    Hex(HexTextErrorReturn),
}

pub struct AdmitStatementsDeps;

pub struct AdmitStatementsParams;

pub struct AdmitStatementsPayload<'a> {
    pub network: &'a NetworkDocument,
    pub statements: &'a StatementsDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitStatementsSuccessReturn {
    pub network: NetworkProfile,
    pub statements: HarnessStatements,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitStatementsErrorReturn {
    CanonicalField {
        field: StatementField,
        error: CanonicalFieldAdmissionErrorReturn,
    },
    Hex {
        field: StatementField,
        error: HexTextErrorReturn,
    },
    SuiteIdentifier {
        relation: DeliveryRelation,
        error: SuiteIdentifierTryNewErrorReturn,
    },
    AssetIdentityHash {
        relation: DeliveryRelation,
        error: AssetIdentityHashTryNewErrorReturn,
    },
    ParameterSetIdentifier {
        relation: DeliveryRelation,
        error: ParameterSetIdentifierTryNewErrorReturn,
    },
    MintPurpose(UndeclaredDeliveryPurposeCode),
    TransferPurpose(UndeclaredDeliveryPurposeCode),
}

pub type AdmitStatementsReturn = Result<AdmitStatementsSuccessReturn, AdmitStatementsErrorReturn>;

pub type AdmitStatementsFn = fn(
    &AdmitStatementsDeps,
    AdmitStatementsParams,
    AdmitStatementsPayload<'_>,
) -> AdmitStatementsReturn;
