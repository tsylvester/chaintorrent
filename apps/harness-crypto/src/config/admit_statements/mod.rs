mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;
#[cfg(test)]
mod test_order;

use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParams, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams, SuiteIdentifier, SuiteIdentifierConstructorParams,
};
use encoding::{CanonicalFieldKind, CanonicalFieldValue};
use proof::{DeliveryRelation, MintPurpose, TransferPurpose};

use super::layers::provides::{
    CanonicalFieldDocument, CanonicalFieldDocumentValue, DeliveryContextDocument,
};
use crate::generate::provides::CanonicalDeliveryContext;
use interface::{
    AdmitStatementsDeps, AdmitStatementsErrorReturn, AdmitStatementsParams, AdmitStatementsPayload,
    AdmitStatementsReturn, AdmitStatementsSuccessReturn, CanonicalFieldAdmissionErrorReturn,
    ContextField, HarnessStatements, HexTextErrorReturn, NetworkProfile, StatementField,
};

pub fn admit_statements(
    _deps: &AdmitStatementsDeps,
    _params: AdmitStatementsParams,
    payload: AdmitStatementsPayload<'_>,
) -> AdmitStatementsReturn {
    let hex = |text: &str, width: Option<usize>| -> Result<Vec<u8>, HexTextErrorReturn> {
        let Some(digits) = text.strip_prefix("0x") else {
            return Err(HexTextErrorReturn::MissingPrefix);
        };
        for (offset, byte) in digits.bytes().enumerate() {
            if !byte.is_ascii_hexdigit() {
                return Err(HexTextErrorReturn::InvalidDigit {
                    index: offset + 2,
                    byte,
                });
            }
        }
        if digits.len() % 2 == 1 {
            return Err(HexTextErrorReturn::OddDigitCount {
                count: digits.len(),
            });
        }
        let actual = digits.len() / 2;
        if let Some(expected) = width
            && actual != expected
        {
            return Err(HexTextErrorReturn::WrongLength { expected, actual });
        }
        let mut bytes = Vec::with_capacity(actual);
        let mut high = 0;
        for (offset, byte) in digits.bytes().enumerate() {
            let digit = match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => byte - b'A' + 10,
            };
            if offset % 2 == 0 {
                high = digit;
            } else {
                bytes.push(high * 16 + digit);
            }
        }
        Ok(bytes)
    };
    let canonical_field = |document: &CanonicalFieldDocument|
     -> Result<CanonicalFieldValue, CanonicalFieldAdmissionErrorReturn> {
        let kind = match document.kind.as_str() {
            "fixed_bytes32" => CanonicalFieldKind::FixedBytes32,
            "unsigned16" => CanonicalFieldKind::Unsigned16,
            "unsigned32" => CanonicalFieldKind::Unsigned32,
            "unsigned64" => CanonicalFieldKind::Unsigned64,
            "text" => CanonicalFieldKind::Text,
            "bytes" => CanonicalFieldKind::Bytes,
            "fixed_bytes20" => CanonicalFieldKind::FixedBytes20,
            "unsigned256" => CanonicalFieldKind::Unsigned256,
            _ => {
                return Err(CanonicalFieldAdmissionErrorReturn::UnknownKind {
                    text: document.kind.clone(),
                });
            }
        };
        match (kind, &document.value) {
            (CanonicalFieldKind::Unsigned16, CanonicalFieldDocumentValue::Integer(value)) => {
                u16::try_from(*value)
                    .map(CanonicalFieldValue::Unsigned16)
                    .map_err(|_| CanonicalFieldAdmissionErrorReturn::OutOfRange {
                        kind,
                        value: *value,
                    })
            }
            (CanonicalFieldKind::Unsigned32, CanonicalFieldDocumentValue::Integer(value)) => {
                u32::try_from(*value)
                    .map(CanonicalFieldValue::Unsigned32)
                    .map_err(|_| CanonicalFieldAdmissionErrorReturn::OutOfRange {
                        kind,
                        value: *value,
                    })
            }
            (CanonicalFieldKind::Unsigned64, CanonicalFieldDocumentValue::Integer(value)) => {
                Ok(CanonicalFieldValue::Unsigned64(*value))
            }
            (CanonicalFieldKind::Unsigned256, CanonicalFieldDocumentValue::Integer(value)) => {
                let mut bytes = [0; 32];
                bytes[24..].copy_from_slice(&value.to_be_bytes());
                Ok(CanonicalFieldValue::Unsigned256(bytes))
            }
            (
                CanonicalFieldKind::Unsigned16
                | CanonicalFieldKind::Unsigned32
                | CanonicalFieldKind::Unsigned64,
                CanonicalFieldDocumentValue::Text(_),
            ) => Err(CanonicalFieldAdmissionErrorReturn::ExpectedInteger { kind }),
            (CanonicalFieldKind::Unsigned256, CanonicalFieldDocumentValue::Text(text)) => {
                let bytes =
                    hex(text, Some(32)).map_err(CanonicalFieldAdmissionErrorReturn::Hex)?;
                match <[u8; 32]>::try_from(bytes.as_slice()) {
                    Ok(array) => Ok(CanonicalFieldValue::Unsigned256(array)),
                    Err(_) => Err(CanonicalFieldAdmissionErrorReturn::Hex(
                        HexTextErrorReturn::WrongLength {
                            expected: 32,
                            actual: bytes.len(),
                        },
                    )),
                }
            }
            (CanonicalFieldKind::FixedBytes32, CanonicalFieldDocumentValue::Text(text)) => {
                let bytes =
                    hex(text, Some(32)).map_err(CanonicalFieldAdmissionErrorReturn::Hex)?;
                match <[u8; 32]>::try_from(bytes.as_slice()) {
                    Ok(array) => Ok(CanonicalFieldValue::FixedBytes32(array)),
                    Err(_) => Err(CanonicalFieldAdmissionErrorReturn::Hex(
                        HexTextErrorReturn::WrongLength {
                            expected: 32,
                            actual: bytes.len(),
                        },
                    )),
                }
            }
            (CanonicalFieldKind::FixedBytes20, CanonicalFieldDocumentValue::Text(text)) => {
                let bytes =
                    hex(text, Some(20)).map_err(CanonicalFieldAdmissionErrorReturn::Hex)?;
                match <[u8; 20]>::try_from(bytes.as_slice()) {
                    Ok(array) => Ok(CanonicalFieldValue::FixedBytes20(array)),
                    Err(_) => Err(CanonicalFieldAdmissionErrorReturn::Hex(
                        HexTextErrorReturn::WrongLength {
                            expected: 20,
                            actual: bytes.len(),
                        },
                    )),
                }
            }
            (CanonicalFieldKind::Bytes, CanonicalFieldDocumentValue::Text(text)) => hex(text, None)
                .map(CanonicalFieldValue::Bytes)
                .map_err(CanonicalFieldAdmissionErrorReturn::Hex),
            (CanonicalFieldKind::Text, CanonicalFieldDocumentValue::Text(text)) => {
                Ok(CanonicalFieldValue::Text(text.clone()))
            }
            (_, CanonicalFieldDocumentValue::Integer(_)) => {
                Err(CanonicalFieldAdmissionErrorReturn::ExpectedText { kind })
            }
        }
    };
    let admit_context = |relation: DeliveryRelation,
                         context: &DeliveryContextDocument,
                         network: &NetworkProfile|
     -> Result<CanonicalDeliveryContext, AdmitStatementsErrorReturn> {
        let identifier = hex(&context.suite_identifier, Some(32)).map_err(|error| {
            AdmitStatementsErrorReturn::Hex {
                field: StatementField::Context {
                    relation,
                    field: ContextField::SuiteIdentifier,
                },
                error,
            }
        })?;
        let suite_identifier = SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier: match <[u8; 32]>::try_from(identifier.as_slice()) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(AdmitStatementsErrorReturn::Hex {
                        field: StatementField::Context {
                            relation,
                            field: ContextField::SuiteIdentifier,
                        },
                        error: HexTextErrorReturn::WrongLength {
                            expected: 32,
                            actual: identifier.len(),
                        },
                    });
                }
            },
            version: context.suite_version,
        })
        .map_err(|error| AdmitStatementsErrorReturn::SuiteIdentifier { relation, error })?;
        let bytes = hex(&context.asset_identity_hash, Some(32)).map_err(|error| {
            AdmitStatementsErrorReturn::Hex {
                field: StatementField::Context {
                    relation,
                    field: ContextField::AssetIdentityHash,
                },
                error,
            }
        })?;
        let asset_identity_hash = AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
            bytes: match <[u8; 32]>::try_from(bytes.as_slice()) {
                Ok(array) => array,
                Err(_) => {
                    return Err(AdmitStatementsErrorReturn::Hex {
                        field: StatementField::Context {
                            relation,
                            field: ContextField::AssetIdentityHash,
                        },
                        error: HexTextErrorReturn::WrongLength {
                            expected: 32,
                            actual: bytes.len(),
                        },
                    });
                }
            },
        })
        .map_err(|error| AdmitStatementsErrorReturn::AssetIdentityHash { relation, error })?;
        let bytes = hex(&context.parameter_set_identifier, Some(32)).map_err(|error| {
            AdmitStatementsErrorReturn::Hex {
                field: StatementField::Context {
                    relation,
                    field: ContextField::ParameterSetIdentifier,
                },
                error,
            }
        })?;
        let parameter_set_identifier =
            ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                bytes: match <[u8; 32]>::try_from(bytes.as_slice()) {
                    Ok(array) => array,
                    Err(_) => {
                        return Err(AdmitStatementsErrorReturn::Hex {
                            field: StatementField::Context {
                                relation,
                                field: ContextField::ParameterSetIdentifier,
                            },
                            error: HexTextErrorReturn::WrongLength {
                                expected: 32,
                                actual: bytes.len(),
                            },
                        });
                    }
                },
            })
            .map_err(|error| AdmitStatementsErrorReturn::ParameterSetIdentifier {
                relation,
                error,
            })?;
        let source_entitlement = canonical_field(&context.source_entitlement).map_err(|error| {
            AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::Context {
                    relation,
                    field: ContextField::SourceEntitlement,
                },
                error,
            }
        })?;
        let target_entitlement = canonical_field(&context.target_entitlement).map_err(|error| {
            AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::Context {
                    relation,
                    field: ContextField::TargetEntitlement,
                },
                error,
            }
        })?;
        let old_interval = canonical_field(&context.old_interval).map_err(|error| {
            AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::Context {
                    relation,
                    field: ContextField::OldInterval,
                },
                error,
            }
        })?;
        let new_interval = canonical_field(&context.new_interval).map_err(|error| {
            AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::Context {
                    relation,
                    field: ContextField::NewInterval,
                },
                error,
            }
        })?;
        let seller = canonical_field(&context.seller).map_err(|error| {
            AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::Context {
                    relation,
                    field: ContextField::Seller,
                },
                error,
            }
        })?;
        let buyer = canonical_field(&context.buyer).map_err(|error| {
            AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::Context {
                    relation,
                    field: ContextField::Buyer,
                },
                error,
            }
        })?;
        Ok(CanonicalDeliveryContext {
            suite_identifier,
            chain: network.chain.clone(),
            entitlement_contract: network.entitlement_contract.clone(),
            asset_identity_hash,
            parameter_set_identifier,
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            seller,
            buyer,
            expiry: context.expiry,
        })
    };
    let network = NetworkProfile {
        chain: canonical_field(&payload.network.chain).map_err(|error| {
            AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::NetworkChain,
                error,
            }
        })?,
        entitlement_contract: canonical_field(&payload.network.entitlement_contract).map_err(
            |error| AdmitStatementsErrorReturn::CanonicalField {
                field: StatementField::NetworkEntitlementContract,
                error,
            },
        )?,
    };
    let mint = admit_context(DeliveryRelation::Mint, &payload.statements.mint, &network)?;
    let transfer = admit_context(
        DeliveryRelation::Transfer,
        &payload.statements.transfer,
        &network,
    )?;
    let mint_purpose = MintPurpose::try_from(payload.statements.mint_purpose)
        .map_err(AdmitStatementsErrorReturn::MintPurpose)?;
    let transfer_purpose = TransferPurpose::try_from(payload.statements.transfer_purpose)
        .map_err(AdmitStatementsErrorReturn::TransferPurpose)?;
    let mut identities = Vec::with_capacity(payload.statements.identities.len());
    for (index, text) in payload.statements.identities.iter().enumerate() {
        identities.push(
            hex(text, None).map_err(|error| AdmitStatementsErrorReturn::Hex {
                field: StatementField::Identity { index },
                error,
            })?,
        );
    }
    let proof_identity = hex(&payload.statements.proof_identity, None).map_err(|error| {
        AdmitStatementsErrorReturn::Hex {
            field: StatementField::ProofIdentity,
            error,
        }
    })?;
    Ok(AdmitStatementsSuccessReturn {
        network,
        statements: HarnessStatements {
            mint,
            transfer,
            mint_purpose,
            transfer_purpose,
            identities,
            proof_identity,
        },
    })
}
