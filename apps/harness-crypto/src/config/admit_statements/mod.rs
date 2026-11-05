pub mod canonical_delivery_context;
#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use domain::{
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams, SUITE_IDENTIFIER_LENGTH, SuiteIdentifier,
    SuiteIdentifierConstructorParams,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, DECLARED_FIXED_WIDTH_FIELD_KINDS,
    DECLARED_UNSIGNED_FIELD_KINDS,
};
use hashing::{Root, RootConstructorParams};
use kem::IdentityScope;
use proof::{MintPurpose, TransferPurpose};

use super::layers::provides::{CanonicalValueTable, ContextTable};
use canonical_delivery_context::provides::CanonicalDeliveryContext;
use interface::{
    AdmitStatementsDeps, AdmitStatementsErrorReturn, AdmitStatementsParams, AdmitStatementsPayload,
    AdmitStatementsReturn, AdmitStatementsSuccessReturn, AdmittedIdentities, AdmittedIdentity,
    AdmittedNetworkProfile, AdmittedStatements, CanonicalValueRefusal, ContextField,
    ContextRefusal, ContextRelation, HexRefusal, IdentityField, IdentityRefusal,
    NetworkProfileField,
};

fn kind_named(text: &str) -> Option<CanonicalFieldKind> {
    for kind in DECLARED_FIXED_WIDTH_FIELD_KINDS
        .iter()
        .chain(DECLARED_UNSIGNED_FIELD_KINDS)
    {
        if format!("{:?}", kind) == text {
            return Some(*kind);
        }
    }
    None
}

fn hex_digit(character: char) -> Option<u8> {
    u8::try_from(character.to_digit(16)?).ok()
}

fn bytes_from_hex(text: &str) -> Result<Vec<u8>, HexRefusal> {
    let malformed = || HexRefusal::Malformed {
        text: text.to_string(),
    };
    let digits = text.strip_prefix("0x").ok_or_else(&malformed)?;
    if digits.len() % 2 != 0
        || !digits
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Err(malformed());
    }
    let mut bytes = Vec::with_capacity(digits.len() / 2);
    for pair in digits.as_bytes().as_chunks::<2>().0 {
        match (
            hex_digit(char::from(pair[0])),
            hex_digit(char::from(pair[1])),
        ) {
            (Some(high), Some(low)) => bytes.push(high * 16 + low),
            _ => return Err(malformed()),
        }
    }
    Ok(bytes)
}

fn fixed_from_hex<const N: usize>(text: &str) -> Result<[u8; N], HexRefusal> {
    let bytes = bytes_from_hex(text)?;
    let actual = bytes.len();
    <[u8; N]>::try_from(bytes.as_slice()).map_err(|_| HexRefusal::WrongWidth {
        expected: N,
        actual,
    })
}

fn unsigned256_from_decimal(text: &str) -> Result<[u8; 32], CanonicalValueRefusal> {
    let mut accumulator = [0u8; 32];
    for digit in text.bytes() {
        let mut carry = u16::from(digit - b'0');
        for byte in accumulator.iter_mut().rev() {
            let value = u16::from(*byte) * 10 + carry;
            *byte = value.to_le_bytes()[0];
            carry = value >> 8;
        }
        if carry != 0 {
            return Err(CanonicalValueRefusal::OutOfRange {
                text: text.to_string(),
            });
        }
    }
    Ok(accumulator)
}

fn canonical_value(
    value: &CanonicalValueTable,
) -> Result<CanonicalFieldValue, CanonicalValueRefusal> {
    match kind_named(&value.kind) {
        Some(CanonicalFieldKind::FixedBytes32) => fixed_from_hex::<32>(&value.value)
            .map(CanonicalFieldValue::FixedBytes32)
            .map_err(CanonicalValueRefusal::Hex),
        Some(CanonicalFieldKind::FixedBytes20) => fixed_from_hex::<20>(&value.value)
            .map(CanonicalFieldValue::FixedBytes20)
            .map_err(CanonicalValueRefusal::Hex),
        Some(
            kind @ (CanonicalFieldKind::Unsigned16
            | CanonicalFieldKind::Unsigned32
            | CanonicalFieldKind::Unsigned64
            | CanonicalFieldKind::Unsigned256),
        ) => {
            if value.value.is_empty() || !value.value.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(CanonicalValueRefusal::MalformedDecimal {
                    text: value.value.clone(),
                });
            }
            match kind {
                CanonicalFieldKind::Unsigned16 => value
                    .value
                    .parse::<u16>()
                    .map(CanonicalFieldValue::Unsigned16)
                    .map_err(|_| CanonicalValueRefusal::OutOfRange {
                        text: value.value.clone(),
                    }),
                CanonicalFieldKind::Unsigned32 => value
                    .value
                    .parse::<u32>()
                    .map(CanonicalFieldValue::Unsigned32)
                    .map_err(|_| CanonicalValueRefusal::OutOfRange {
                        text: value.value.clone(),
                    }),
                CanonicalFieldKind::Unsigned64 => value
                    .value
                    .parse::<u64>()
                    .map(CanonicalFieldValue::Unsigned64)
                    .map_err(|_| CanonicalValueRefusal::OutOfRange {
                        text: value.value.clone(),
                    }),
                _ => unsigned256_from_decimal(&value.value).map(CanonicalFieldValue::Unsigned256),
            }
        }
        _ => Err(CanonicalValueRefusal::UnknownKind {
            text: value.kind.clone(),
        }),
    }
}

fn admit_context(
    relation: ContextRelation,
    table: &ContextTable,
    network_profile: &AdmittedNetworkProfile,
) -> Result<CanonicalDeliveryContext, AdmitStatementsErrorReturn> {
    let suite_identifier = fixed_from_hex::<SUITE_IDENTIFIER_LENGTH>(&table.suite_identifier)
        .map_err(|refusal| AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::SuiteIdentifier,
            refusal: ContextRefusal::Hex(refusal),
        })
        .and_then(|identifier| {
            u16::try_from(table.suite_version)
                .map(|version| (identifier, version))
                .map_err(|_| AdmitStatementsErrorReturn::Context {
                    relation,
                    field: ContextField::SuiteVersion,
                    refusal: ContextRefusal::SuiteVersion {
                        version: table.suite_version,
                    },
                })
        })
        .and_then(|(identifier, version)| {
            SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                identifier,
                version,
            })
            .map_err(|error| AdmitStatementsErrorReturn::Context {
                relation,
                field: ContextField::SuiteIdentifier,
                refusal: ContextRefusal::Suite(error),
            })
        })?;
    let asset_root = fixed_from_hex::<32>(&table.asset_root)
        .map_err(|refusal| AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::AssetRoot,
            refusal: ContextRefusal::Hex(refusal),
        })
        .and_then(|bytes| {
            Root::try_new(RootConstructorParams { bytes }).map_err(|error| {
                AdmitStatementsErrorReturn::Context {
                    relation,
                    field: ContextField::AssetRoot,
                    refusal: ContextRefusal::Root(error),
                }
            })
        })?;
    let parameter_set_identifier =
        fixed_from_hex::<PARAMETER_SET_IDENTIFIER_LENGTH>(&table.parameter_set_identifier)
            .map_err(|refusal| AdmitStatementsErrorReturn::Context {
                relation,
                field: ContextField::ParameterSetIdentifier,
                refusal: ContextRefusal::Hex(refusal),
            })
            .and_then(|bytes| {
                ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams { bytes })
                    .map_err(|error| AdmitStatementsErrorReturn::Context {
                        relation,
                        field: ContextField::ParameterSetIdentifier,
                        refusal: ContextRefusal::ParameterSet(error),
                    })
            })?;
    let source_entitlement = canonical_value(&table.source_entitlement).map_err(|refusal| {
        AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::SourceEntitlement,
            refusal: ContextRefusal::Value(refusal),
        }
    })?;
    let target_entitlement = canonical_value(&table.target_entitlement).map_err(|refusal| {
        AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::TargetEntitlement,
            refusal: ContextRefusal::Value(refusal),
        }
    })?;
    let old_interval = canonical_value(&table.old_interval).map_err(|refusal| {
        AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::OldInterval,
            refusal: ContextRefusal::Value(refusal),
        }
    })?;
    let new_interval = canonical_value(&table.new_interval).map_err(|refusal| {
        AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::NewInterval,
            refusal: ContextRefusal::Value(refusal),
        }
    })?;
    let seller =
        canonical_value(&table.seller).map_err(|refusal| AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::Seller,
            refusal: ContextRefusal::Value(refusal),
        })?;
    let buyer =
        canonical_value(&table.buyer).map_err(|refusal| AdmitStatementsErrorReturn::Context {
            relation,
            field: ContextField::Buyer,
            refusal: ContextRefusal::Value(refusal),
        })?;
    Ok(CanonicalDeliveryContext {
        suite_identifier,
        chain: network_profile.chain_identifier.clone(),
        entitlement_contract: network_profile.entitlement_contract.clone(),
        asset_root,
        parameter_set_identifier,
        source_entitlement,
        target_entitlement,
        old_interval,
        new_interval,
        seller,
        buyer,
        expiry: table.expiry,
    })
}

fn admit_identity(
    identity: IdentityField,
    text: &str,
    scope: IdentityScope,
) -> Result<AdmittedIdentity, AdmitStatementsErrorReturn> {
    match scope {
        IdentityScope::Asset => {
            let bytes = fixed_from_hex::<32>(text).map_err(|refusal| {
                AdmitStatementsErrorReturn::Identity {
                    identity,
                    refusal: IdentityRefusal::Hex(refusal),
                }
            })?;
            Root::try_new(RootConstructorParams { bytes })
                .map(|root| AdmittedIdentity::Asset { root })
                .map_err(|error| AdmitStatementsErrorReturn::Identity {
                    identity,
                    refusal: IdentityRefusal::Root(error),
                })
        }
        IdentityScope::Entitlement => bytes_from_hex(text)
            .map(|canonical| AdmittedIdentity::Entitlement { canonical })
            .map_err(|refusal| AdmitStatementsErrorReturn::Identity {
                identity,
                refusal: IdentityRefusal::Hex(refusal),
            }),
    }
}

pub fn admit_statements(
    _deps: &AdmitStatementsDeps,
    _params: AdmitStatementsParams,
    payload: AdmitStatementsPayload,
) -> AdmitStatementsReturn {
    let network_profile = AdmittedNetworkProfile {
        chain_identifier: canonical_value(&payload.network_profile.chain_identifier).map_err(
            |refusal| AdmitStatementsErrorReturn::NetworkProfile {
                field: NetworkProfileField::ChainIdentifier,
                refusal,
            },
        )?,
        entitlement_contract: canonical_value(&payload.network_profile.entitlement_contract)
            .map_err(|refusal| AdmitStatementsErrorReturn::NetworkProfile {
                field: NetworkProfileField::EntitlementContract,
                refusal,
            })?,
        deployment_record: canonical_value(&payload.network_profile.deployment_record).map_err(
            |refusal| AdmitStatementsErrorReturn::NetworkProfile {
                field: NetworkProfileField::DeploymentRecord,
                refusal,
            },
        )?,
        gas_price_oracle: canonical_value(&payload.network_profile.gas_price_oracle).map_err(
            |refusal| AdmitStatementsErrorReturn::NetworkProfile {
                field: NetworkProfileField::GasPriceOracle,
                refusal,
            },
        )?,
    };
    let mint = admit_context(
        ContextRelation::Mint,
        &payload.statements.mint,
        &network_profile,
    )?;
    let mint_purpose = MintPurpose::try_from(payload.statements.mint.purpose).map_err(|error| {
        AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Mint,
            field: ContextField::Purpose,
            refusal: ContextRefusal::Purpose(error),
        }
    })?;
    let transfer = admit_context(
        ContextRelation::Transfer,
        &payload.statements.transfer,
        &network_profile,
    )?;
    let transfer_purpose =
        TransferPurpose::try_from(payload.statements.transfer.purpose).map_err(|error| {
            AdmitStatementsErrorReturn::Context {
                relation: ContextRelation::Transfer,
                field: ContextField::Purpose,
                refusal: ContextRefusal::Purpose(error),
            }
        })?;
    let proof = admit_identity(
        IdentityField::Proof,
        &payload.identities.proof,
        payload.scope,
    )?;
    let mut mapping = Vec::with_capacity(payload.identities.mapping.len());
    for (index, text) in payload.identities.mapping.iter().enumerate() {
        mapping.push(admit_identity(
            IdentityField::Mapping { index },
            text,
            payload.scope,
        )?);
    }
    Ok(AdmitStatementsSuccessReturn {
        network_profile,
        statements: AdmittedStatements {
            mint,
            mint_purpose,
            transfer,
            transfer_purpose,
        },
        identities: AdmittedIdentities { proof, mapping },
    })
}
