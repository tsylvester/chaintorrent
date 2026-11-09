use domain::{ParameterSetIdentifierTryNewErrorReturn, SuiteIdentifierTryNewErrorReturn};
use encoding::CanonicalFieldValue;
use hashing::{Root, RootTryNewErrorReturn};
use kem::IdentityScope;
use proof::{MintPurpose, TransferPurpose, UndeclaredDeliveryPurposeCode};

use super::super::layers::provides::{IdentitiesTable, NetworkProfileTable, StatementsTable};
use super::canonical_delivery_context::provides::CanonicalDeliveryContext;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedNetworkProfile {
    pub chain_identifier: CanonicalFieldValue,
    pub entitlement_contract: CanonicalFieldValue,
    pub deployment_records: Vec<CanonicalFieldValue>,
    pub gas_price_oracle: CanonicalFieldValue,
}

#[derive(Clone)]
pub struct AdmittedStatements {
    pub mint: CanonicalDeliveryContext,
    pub mint_purpose: MintPurpose,
    pub transfer: CanonicalDeliveryContext,
    pub transfer_purpose: TransferPurpose,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmittedIdentity {
    Entitlement { canonical: Vec<u8> },
    Asset { root: Root },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedIdentities {
    pub proof: AdmittedIdentity,
    pub mapping: Vec<AdmittedIdentity>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkProfileField {
    ChainIdentifier,
    EntitlementContract,
    GasPriceOracle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextRelation {
    Mint,
    Transfer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextField {
    SuiteIdentifier,
    SuiteVersion,
    AssetRoot,
    ParameterSetIdentifier,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    NewInterval,
    Seller,
    Buyer,
    Purpose,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityField {
    Proof,
    Mapping { index: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HexRefusal {
    Malformed { text: String },
    WrongWidth { expected: usize, actual: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanonicalValueRefusal {
    UnknownKind { text: String },
    Hex(HexRefusal),
    MalformedDecimal { text: String },
    OutOfRange { text: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContextRefusal {
    Value(CanonicalValueRefusal),
    Hex(HexRefusal),
    Suite(SuiteIdentifierTryNewErrorReturn),
    SuiteVersion { version: u32 },
    Root(RootTryNewErrorReturn),
    ParameterSet(ParameterSetIdentifierTryNewErrorReturn),
    Purpose(UndeclaredDeliveryPurposeCode),
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentityRefusal {
    Hex(HexRefusal),
    Root(RootTryNewErrorReturn),
}

#[derive(Clone, Copy, Debug)]
pub struct AdmitStatementsDeps;

#[derive(Clone, Copy, Debug)]
pub struct AdmitStatementsParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitStatementsPayload {
    pub network_profile: NetworkProfileTable,
    pub statements: StatementsTable,
    pub identities: IdentitiesTable,
    pub curve_labels: Vec<String>,
    pub scope: IdentityScope,
}

#[derive(Clone)]
pub struct AdmitStatementsSuccessReturn {
    pub network_profile: AdmittedNetworkProfile,
    pub statements: AdmittedStatements,
    pub identities: AdmittedIdentities,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitStatementsErrorReturn {
    NetworkProfile {
        field: NetworkProfileField,
        refusal: CanonicalValueRefusal,
    },
    DeploymentRecord {
        label: String,
        refusal: CanonicalValueRefusal,
    },
    MissingDeploymentRecord { label: String },
    UndeclaredDeploymentRecord { label: String },
    Context {
        relation: ContextRelation,
        field: ContextField,
        refusal: ContextRefusal,
    },
    Identity {
        identity: IdentityField,
        refusal: IdentityRefusal,
    },
}

pub type AdmitStatementsReturn = Result<AdmitStatementsSuccessReturn, AdmitStatementsErrorReturn>;

pub type AdmitStatementsFn = fn(
    &AdmitStatementsDeps,
    AdmitStatementsParams,
    AdmitStatementsPayload,
) -> AdmitStatementsReturn;
