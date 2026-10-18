#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use domain::{
    AssetIdentityHashTryNewErrorReturn, ParameterSetIdentifierTryNewErrorReturn,
    SuiteIdentifierTryNewErrorReturn,
};
use proof::{DeliveryRelation, UndeclaredDeliveryPurposeCode};

use super::super::layers::provides::{
    CanonicalFieldDocumentOverrides, CanonicalFieldDocumentValue, DeliveryContextDocumentOverrides,
    NetworkDocumentOverrides, StatementsDocumentOverrides, build_canonical_field_document,
    build_delivery_context_document, build_network_document, build_statements_document,
};
use super::admit_statements;
use super::interface::{
    AdmitStatementsErrorReturn, CanonicalFieldAdmissionErrorReturn, ContextField,
    HexTextErrorReturn, StatementField,
};
use super::mock::{
    build_admit_statements_deps, build_admit_statements_params, build_admit_statements_payload,
};

/// Contract: the first refusal in the stated order returns; an invalid digit
///   refuses before the digit-count check.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x1g3"`, invalid at `g` and odd in count; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Identity { index: 0 }, error:
///   HexTextErrorReturn::InvalidDigit { index: 3, byte: b'g' } })`.
#[test]
fn admit_statements_refuses_an_invalid_digit_before_an_odd_digit_count() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0x1g3".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::Hex {
            field: StatementField::Identity { index: 0 },
            error: HexTextErrorReturn::InvalidDigit {
                index: 3,
                byte: b'g',
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; an odd digit count
///   refuses before the width check.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `suite_identifier` `0x` followed
///   by sixty-three `1` digits, odd in count and short of the width; `network`
///   its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::SuiteIdentifier }, error:
///   HexTextErrorReturn::OddDigitCount { count: 63 } })`.
#[test]
fn admit_statements_refuses_an_odd_digit_count_before_a_wrong_width() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_identifier: Some(format!("0x{}", "1".repeat(63))),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::Hex {
            field: StatementField::Context {
                relation: DeliveryRelation::Mint,
                field: ContextField::SuiteIdentifier,
            },
            error: HexTextErrorReturn::OddDigitCount { count: 63 },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the lowest-indexed
///   invalid digit refuses.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x0g0h"`, invalid at `g` and `h`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Identity { index: 0 }, error:
///   HexTextErrorReturn::InvalidDigit { index: 3, byte: b'g' } })`.
#[test]
fn admit_statements_refuses_the_lowest_invalid_digit() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0x0g0h".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::Hex {
            field: StatementField::Identity { index: 0 },
            error: HexTextErrorReturn::InvalidDigit {
                index: 3,
                byte: b'g',
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the network chain
///   is read before the network entitlement contract.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `entitlement_contract` `build_canonical_field_document` over `"uint512"`
///   and `Integer(1)`; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_network_chain_before_the_network_entitlement_contract() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("uint256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(1)),
            },
        )),
        entitlement_contract: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("uint512".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(1)),
            },
        )),
        ..Default::default()
    });
    let statements = build_statements_document(Default::default());
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::NetworkChain,
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the network is
///   admitted before the mint context.
/// Arrange: `network` `build_network_document` with `entitlement_contract`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `suite_version` `0`.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkEntitlementContract, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_network_before_the_mint_context() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        entitlement_contract: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("uint256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(1)),
            },
        )),
        ..Default::default()
    });
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_version: Some(0),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::NetworkEntitlementContract,
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the suite
///   identifier is admitted before the asset identity hash.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `suite_version` `0` and
///   `asset_identity_hash` `0x` followed by sixty-four `0` digits; `network`
///   its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::SuiteIdentifier
///   { relation: DeliveryRelation::Mint, error:
///   SuiteIdentifierTryNewErrorReturn::ZeroVersion })`.
#[test]
fn admit_statements_refuses_the_suite_identifier_before_the_asset_identity_hash() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_version: Some(0),
                asset_identity_hash: Some(format!("0x{}", "0".repeat(64))),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::SuiteIdentifier {
            relation: DeliveryRelation::Mint,
            error: SuiteIdentifierTryNewErrorReturn::ZeroVersion,
        })
    );
}

/// Contract: the first refusal in the stated order returns; the asset identity
///   hash is admitted before the parameter-set identifier.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `asset_identity_hash` and
///   `parameter_set_identifier` each `0x` followed by sixty-four `0` digits;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::AssetIdentityHash
///   { relation: DeliveryRelation::Mint, error:
///   AssetIdentityHashTryNewErrorReturn::AllZero })`.
#[test]
fn admit_statements_refuses_the_asset_identity_hash_before_the_parameter_set_identifier() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                asset_identity_hash: Some(format!("0x{}", "0".repeat(64))),
                parameter_set_identifier: Some(format!("0x{}", "0".repeat(64))),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::AssetIdentityHash {
            relation: DeliveryRelation::Mint,
            error: AssetIdentityHashTryNewErrorReturn::AllZero,
        })
    );
}

/// Contract: the first refusal in the stated order returns; the parameter-set
///   identifier is admitted before the canonical fields.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `parameter_set_identifier` `0x`
///   followed by sixty-four `0` digits and `source_entitlement`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals
///   `Err(AdmitStatementsErrorReturn::ParameterSetIdentifier { relation:
///   DeliveryRelation::Mint, error:
///   ParameterSetIdentifierTryNewErrorReturn::AllZero })`.
#[test]
fn admit_statements_refuses_the_parameter_set_identifier_before_the_canonical_fields() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                parameter_set_identifier: Some(format!("0x{}", "0".repeat(64))),
                source_entitlement: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::ParameterSetIdentifier {
            relation: DeliveryRelation::Mint,
            error: ParameterSetIdentifierTryNewErrorReturn::AllZero,
        })
    );
}

/// Contract: the first refusal in the stated order returns; the source
///   entitlement is read before the target entitlement.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `source_entitlement`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `target_entitlement` `build_canonical_field_document` over `"uint512"`
///   and `Integer(1)`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::SourceEntitlement }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_source_entitlement_before_the_target_entitlement() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                source_entitlement: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                target_entitlement: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint512".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Mint,
                field: ContextField::SourceEntitlement,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the target
///   entitlement is read before the old interval.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `target_entitlement`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `old_interval` `build_canonical_field_document` over `"uint512"` and
///   `Integer(1)`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::TargetEntitlement }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_target_entitlement_before_the_old_interval() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                target_entitlement: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                old_interval: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint512".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Mint,
                field: ContextField::TargetEntitlement,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the old interval
///   is read before the new interval.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `old_interval`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `new_interval` `build_canonical_field_document` over `"uint512"` and
///   `Integer(1)`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::OldInterval }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_old_interval_before_the_new_interval() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                old_interval: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                new_interval: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint512".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Mint,
                field: ContextField::OldInterval,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the new interval
///   is read before the seller.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `new_interval`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `seller` `build_canonical_field_document` over `"uint512"` and
///   `Integer(1)`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::NewInterval }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_new_interval_before_the_seller() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                new_interval: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                seller: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint512".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Mint,
                field: ContextField::NewInterval,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the seller is read
///   before the buyer.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `seller`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `buyer` `build_canonical_field_document` over `"uint512"` and
///   `Integer(1)`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::Seller }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_seller_before_the_buyer() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                seller: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                buyer: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint512".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Mint,
                field: ContextField::Seller,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the mint context
///   is admitted before the transfer context.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `buyer`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `transfer` `build_delivery_context_document` with `suite_version` `0`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::Buyer }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_mint_context_before_the_transfer_context() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                buyer: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        transfer: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_version: Some(0),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Mint,
                field: ContextField::Buyer,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the transfer
///   context is admitted before the purposes.
/// Arrange: `statements` `build_statements_document` with `transfer`
///   `build_delivery_context_document` with `buyer`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)` and
///   `mint_purpose` `2`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Transfer,
///   field: ContextField::Buyer }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_the_transfer_context_before_the_purposes() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        transfer: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                buyer: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("uint256".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                ..Default::default()
            },
        )),
        mint_purpose: Some(2),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Transfer,
                field: ContextField::Buyer,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the mint purpose
///   is converted before the transfer purpose.
/// Arrange: `statements` `build_statements_document` with `mint_purpose` `2`
///   and `transfer_purpose` `1`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::MintPurpose(
///   UndeclaredDeliveryPurposeCode { code: 2 }))`.
#[test]
fn admit_statements_refuses_the_mint_purpose_before_the_transfer_purpose() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint_purpose: Some(2),
        transfer_purpose: Some(1),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::MintPurpose(
            UndeclaredDeliveryPurposeCode { code: 2 }
        ))
    );
}

/// Contract: the first refusal in the stated order returns; the purposes are
///   converted before the identities.
/// Arrange: `statements` `build_statements_document` with `transfer_purpose`
///   `1` and `identities` `"0x0g"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::TransferPurpose(
///   UndeclaredDeliveryPurposeCode { code: 1 }))`.
#[test]
fn admit_statements_refuses_the_purposes_before_the_identities() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        transfer_purpose: Some(1),
        identities: Some(vec!["0x0g".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::TransferPurpose(
            UndeclaredDeliveryPurposeCode { code: 1 }
        ))
    );
}

/// Contract: the first refusal in the stated order returns; an identity is
///   decoded before the next identity.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x0g"` then `"0x0h"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Identity { index: 0 }, error:
///   HexTextErrorReturn::InvalidDigit { index: 3, byte: b'g' } })`.
#[test]
fn admit_statements_refuses_an_identity_before_the_next_identity() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0x0g".to_string(), "0x0h".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::Hex {
            field: StatementField::Identity { index: 0 },
            error: HexTextErrorReturn::InvalidDigit {
                index: 3,
                byte: b'g',
            },
        })
    );
}

/// Contract: the first refusal in the stated order returns; the identities are
///   decoded before the proof identity.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x0g"` and `proof_identity` `"0x123"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Identity { index: 0 }, error:
///   HexTextErrorReturn::InvalidDigit { index: 3, byte: b'g' } })`.
#[test]
fn admit_statements_refuses_the_identities_before_the_proof_identity() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0x0g".to_string()]),
        proof_identity: Some("0x123".to_string()),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitStatementsErrorReturn::Hex {
            field: StatementField::Identity { index: 0 },
            error: HexTextErrorReturn::InvalidDigit {
                index: 3,
                byte: b'g',
            },
        })
    );
}
