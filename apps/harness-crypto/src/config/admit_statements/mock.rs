#![allow(dead_code, clippy::too_many_arguments)]

use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{CanonicalFieldKind, CanonicalFieldValue};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::IdentityScope;
use proof::{MintPurpose, TransferPurpose};

use super::super::layers::provides::{
    CanonicalValueTable, CanonicalValueTableOverrides, ContextTable, ContextTableOverrides,
    IdentitiesTable, IdentitiesTableOverrides, NetworkProfileTable, NetworkProfileTableOverrides,
    StatementsTable, StatementsTableOverrides, build_canonical_value_table, build_context_table,
    build_identities_table, build_network_profile_table, build_statements_table,
};
use super::canonical_delivery_context::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, build_canonical_delivery_context,
};
use super::interface::{
    AdmitStatementsDeps, AdmitStatementsFn, AdmitStatementsParams, AdmitStatementsPayload,
    AdmitStatementsReturn, AdmitStatementsSuccessReturn, AdmittedIdentities, AdmittedIdentity,
    AdmittedNetworkProfile, AdmittedStatements,
};

pub use super::super::admit_selections::provides::declared_name;

fn hex_text(digits: &str, width: usize) -> String {
    format!("0x{}", digits.repeat(width))
}

fn unsigned256(value: u8) -> CanonicalFieldValue {
    let mut bytes = [0u8; 32];
    bytes[31] = value;
    CanonicalFieldValue::Unsigned256(bytes)
}

fn unsigned256_table(value: &str) -> CanonicalValueTable {
    build_canonical_value_table(CanonicalValueTableOverrides {
        kind: Some(declared_name(&CanonicalFieldKind::Unsigned256)),
        value: Some(value.to_string()),
    })
}

fn unsigned64_table(value: &str) -> CanonicalValueTable {
    build_canonical_value_table(CanonicalValueTableOverrides {
        kind: Some(declared_name(&CanonicalFieldKind::Unsigned64)),
        value: Some(value.to_string()),
    })
}

fn fixed_bytes20_table(value: &str) -> CanonicalValueTable {
    build_canonical_value_table(CanonicalValueTableOverrides {
        kind: Some(declared_name(&CanonicalFieldKind::FixedBytes20)),
        value: Some(value.to_string()),
    })
}

fn context_table(purpose: u16) -> ContextTable {
    build_context_table(ContextTableOverrides {
        suite_identifier: Some(hex_text("22", 32)),
        suite_version: Some(1),
        asset_root: Some(hex_text("44", 32)),
        parameter_set_identifier: Some(hex_text("33", 32)),
        source_entitlement: Some(unsigned256_table("1")),
        target_entitlement: Some(unsigned256_table("2")),
        old_interval: Some(unsigned64_table("1")),
        new_interval: Some(unsigned64_table("2")),
        seller: Some(fixed_bytes20_table(&hex_text("61", 20))),
        buyer: Some(fixed_bytes20_table(&hex_text("62", 20))),
        expiry: Some(1_800_000_000),
        purpose: Some(purpose),
    })
}

fn admitted_context() -> CanonicalDeliveryContext {
    build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x22; 32]),
                version: Some(1),
            },
        )),
        chain: Some(unsigned256(1)),
        entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x11; 20])),
        asset_root: Some(build_root(RootConstructorParamsOverrides {
            bytes: Some([0x44; 32]),
        })),
        parameter_set_identifier: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x33; 32]),
            },
        )),
        source_entitlement: Some(unsigned256(1)),
        target_entitlement: Some(unsigned256(2)),
        old_interval: Some(CanonicalFieldValue::Unsigned64(1)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(2)),
        seller: Some(CanonicalFieldValue::FixedBytes20([0x61; 20])),
        buyer: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
        expiry: Some(1_800_000_000),
    })
}

pub fn representative_canonical_value(
    kind: CanonicalFieldKind,
) -> Option<(String, CanonicalFieldValue)> {
    match kind {
        CanonicalFieldKind::FixedBytes32 => Some((
            hex_text("a1", 32),
            CanonicalFieldValue::FixedBytes32([0xa1; 32]),
        )),
        CanonicalFieldKind::FixedBytes20 => Some((
            hex_text("a2", 20),
            CanonicalFieldValue::FixedBytes20([0xa2; 20]),
        )),
        CanonicalFieldKind::Unsigned16 => {
            Some(("513".to_string(), CanonicalFieldValue::Unsigned16(513)))
        }
        CanonicalFieldKind::Unsigned32 => Some((
            "16909060".to_string(),
            CanonicalFieldValue::Unsigned32(16909060),
        )),
        CanonicalFieldKind::Unsigned64 => Some((
            "72623859790382856".to_string(),
            CanonicalFieldValue::Unsigned64(72623859790382856),
        )),
        CanonicalFieldKind::Unsigned256 => {
            let mut bytes = [0u8; 32];
            bytes[30] = 0x01;
            bytes[31] = 0x02;
            Some(("258".to_string(), CanonicalFieldValue::Unsigned256(bytes)))
        }
        CanonicalFieldKind::Text | CanonicalFieldKind::Bytes => None,
    }
}

#[derive(Default)]
pub struct AdmittedNetworkProfileOverrides {
    pub chain_identifier: Option<CanonicalFieldValue>,
    pub entitlement_contract: Option<CanonicalFieldValue>,
    pub deployment_record: Option<CanonicalFieldValue>,
    pub gas_price_oracle: Option<CanonicalFieldValue>,
}

pub fn build_admitted_network_profile(
    overrides: AdmittedNetworkProfileOverrides,
) -> AdmittedNetworkProfile {
    AdmittedNetworkProfile {
        chain_identifier: overrides.chain_identifier.unwrap_or_else(|| unsigned256(1)),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x11; 20])),
        deployment_record: overrides
            .deployment_record
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x22; 20])),
        gas_price_oracle: overrides
            .gas_price_oracle
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x23; 20])),
    }
}

#[derive(Default)]
pub struct AdmittedStatementsOverrides {
    pub mint: Option<CanonicalDeliveryContext>,
    pub mint_purpose: Option<MintPurpose>,
    pub transfer: Option<CanonicalDeliveryContext>,
    pub transfer_purpose: Option<TransferPurpose>,
}

pub fn build_admitted_statements(overrides: AdmittedStatementsOverrides) -> AdmittedStatements {
    AdmittedStatements {
        mint: overrides.mint.unwrap_or_else(admitted_context),
        mint_purpose: overrides.mint_purpose.unwrap_or(MintPurpose::Mint),
        transfer: overrides.transfer.unwrap_or_else(admitted_context),
        transfer_purpose: overrides
            .transfer_purpose
            .unwrap_or(TransferPurpose::Transfer),
    }
}

#[derive(Default)]
pub struct AdmittedIdentitiesOverrides {
    pub proof: Option<AdmittedIdentity>,
    pub mapping: Option<Vec<AdmittedIdentity>>,
}

pub fn build_admitted_identities(overrides: AdmittedIdentitiesOverrides) -> AdmittedIdentities {
    AdmittedIdentities {
        proof: overrides.proof.unwrap_or(AdmittedIdentity::Entitlement {
            canonical: vec![0xaa, 0x01],
        }),
        mapping: overrides.mapping.unwrap_or_else(|| {
            vec![AdmittedIdentity::Entitlement {
                canonical: vec![0xbb, 0x02],
            }]
        }),
    }
}

#[derive(Default)]
pub struct AdmitStatementsDepsOverrides;

pub fn build_admit_statements_deps(
    _overrides: AdmitStatementsDepsOverrides,
) -> AdmitStatementsDeps {
    AdmitStatementsDeps
}

#[derive(Default)]
pub struct AdmitStatementsParamsOverrides;

pub fn build_admit_statements_params(
    _overrides: AdmitStatementsParamsOverrides,
) -> AdmitStatementsParams {
    AdmitStatementsParams
}

#[derive(Default)]
pub struct AdmitStatementsPayloadOverrides {
    pub network_profile: Option<NetworkProfileTable>,
    pub statements: Option<StatementsTable>,
    pub identities: Option<IdentitiesTable>,
    pub scope: Option<IdentityScope>,
}

pub fn build_admit_statements_payload(
    overrides: AdmitStatementsPayloadOverrides,
) -> AdmitStatementsPayload {
    AdmitStatementsPayload {
        network_profile: overrides.network_profile.unwrap_or_else(|| {
            build_network_profile_table(NetworkProfileTableOverrides {
                chain_identifier: Some(unsigned256_table("1")),
                entitlement_contract: Some(fixed_bytes20_table(&hex_text("11", 20))),
                deployment_record: Some(fixed_bytes20_table(&hex_text("22", 20))),
                gas_price_oracle: Some(fixed_bytes20_table(&hex_text("23", 20))),
                ..Default::default()
            })
        }),
        statements: overrides.statements.unwrap_or_else(|| {
            build_statements_table(StatementsTableOverrides {
                mint: Some(context_table(1)),
                transfer: Some(context_table(2)),
            })
        }),
        identities: overrides.identities.unwrap_or_else(|| {
            build_identities_table(IdentitiesTableOverrides {
                proof: Some("0xaa01".to_string()),
                mapping: Some(vec!["0xbb02".to_string()]),
            })
        }),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

#[derive(Default)]
pub struct AdmitStatementsSuccessReturnOverrides {
    pub network_profile: Option<AdmittedNetworkProfile>,
    pub statements: Option<AdmittedStatements>,
    pub identities: Option<AdmittedIdentities>,
}

pub fn build_admit_statements_success_return(
    overrides: AdmitStatementsSuccessReturnOverrides,
) -> AdmitStatementsSuccessReturn {
    AdmitStatementsSuccessReturn {
        network_profile: overrides
            .network_profile
            .unwrap_or_else(|| build_admitted_network_profile(Default::default())),
        statements: overrides
            .statements
            .unwrap_or_else(|| build_admitted_statements(Default::default())),
        identities: overrides
            .identities
            .unwrap_or_else(|| build_admitted_identities(Default::default())),
    }
}

pub fn mock_admit_statements(
    _deps: &AdmitStatementsDeps,
    _params: AdmitStatementsParams,
    _payload: AdmitStatementsPayload,
) -> AdmitStatementsReturn {
    let _: AdmitStatementsFn = mock_admit_statements;
    Ok(build_admit_statements_success_return(Default::default()))
}
