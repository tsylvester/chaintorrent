#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use domain::{
    AssetIdentityHashTryNewErrorReturn, ParameterSetIdentifierTryNewErrorReturn,
    SuiteIdentifierTryNewErrorReturn,
};
use encoding::CanonicalFieldKind;
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

/// Contract: hex without `0x` is `MissingPrefix` carried with the field it was
///   read from.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `suite_identifier` sixty-two `0`
///   digits followed by `01`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::SuiteIdentifier }, error: HexTextErrorReturn::MissingPrefix
///   })`.
#[test]
fn admit_statements_refuses_a_suite_identifier_without_its_prefix() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_identifier: Some(format!("{}01", "0".repeat(62))),
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
            error: HexTextErrorReturn::MissingPrefix,
        })
    );
}

/// Contract: a hex refusal in the transfer context carries the transfer
///   relation.
/// Arrange: `statements` `build_statements_document` with `transfer`
///   `build_delivery_context_document` with `suite_identifier` sixty-two `0`
///   digits followed by `01`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Context { relation: DeliveryRelation::Transfer, field:
///   ContextField::SuiteIdentifier }, error: HexTextErrorReturn::MissingPrefix
///   })`.
#[test]
fn admit_statements_refuses_a_transfer_suite_identifier_without_its_prefix() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        transfer: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_identifier: Some(format!("{}01", "0".repeat(62))),
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
                relation: DeliveryRelation::Transfer,
                field: ContextField::SuiteIdentifier,
            },
            error: HexTextErrorReturn::MissingPrefix,
        })
    );
}

/// Contract: a width other than 32 bytes is `WrongLength` carried with the
///   asset identity hash field.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `asset_identity_hash` `0x`
///   followed by sixty-two `1` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::AssetIdentityHash }, error: HexTextErrorReturn::WrongLength
///   { expected: 32, actual: 31 } })`.
#[test]
fn admit_statements_refuses_an_asset_identity_hash_of_the_wrong_width() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                asset_identity_hash: Some(format!("0x{}", "1".repeat(62))),
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
                field: ContextField::AssetIdentityHash,
            },
            error: HexTextErrorReturn::WrongLength {
                expected: 32,
                actual: 31,
            },
        })
    );
}

/// Contract: an odd digit count is `OddDigitCount` carried with the
///   parameter-set identifier field.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `parameter_set_identifier` `0x`
///   followed by sixty-three `1` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::ParameterSetIdentifier }, error:
///   HexTextErrorReturn::OddDigitCount { count: 63 } })`.
#[test]
fn admit_statements_refuses_a_parameter_set_identifier_with_an_odd_digit_count() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                parameter_set_identifier: Some(format!("0x{}", "1".repeat(63))),
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
                field: ContextField::ParameterSetIdentifier,
            },
            error: HexTextErrorReturn::OddDigitCount { count: 63 },
        })
    );
}

/// Contract: a refused identity carries its index.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x01"` then `"0x0g"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::Identity { index: 1 }, error:
///   HexTextErrorReturn::InvalidDigit { index: 3, byte: b'g' } })`.
#[test]
fn admit_statements_refuses_an_identity_with_an_invalid_digit_at_its_index() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0x01".to_string(), "0x0g".to_string()]),
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
            field: StatementField::Identity { index: 1 },
            error: HexTextErrorReturn::InvalidDigit {
                index: 3,
                byte: b'g',
            },
        })
    );
}

/// Contract: a refused proof identity carries the proof identity field.
/// Arrange: `statements` `build_statements_document` with `proof_identity`
///   `"0x123"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::Hex { field:
///   StatementField::ProofIdentity, error: HexTextErrorReturn::OddDigitCount {
///   count: 3 } })`.
#[test]
fn admit_statements_refuses_a_proof_identity_with_an_odd_digit_count() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
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
            field: StatementField::ProofIdentity,
            error: HexTextErrorReturn::OddDigitCount { count: 3 },
        })
    );
}

/// Contract: a kind the vocabulary does not hold is `UnknownKind` carrying the
///   text.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("uint256".to_string()),
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

/// Contract: a canonical refusal carries the network entitlement contract
///   field.
/// Arrange: `network` `build_network_document` with `entitlement_contract`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkEntitlementContract, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_network_entitlement_contract() {
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
            field: StatementField::NetworkEntitlementContract,
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: a canonical refusal in a context carries the context field it was
///   read from and the mint relation.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `source_entitlement`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::SourceEntitlement }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_source_entitlement() {
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

/// Contract: as `an_unknown_kind_in_the_source_entitlement` for the target
///   entitlement.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `target_entitlement`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::TargetEntitlement }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_target_entitlement() {
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

/// Contract: as `an_unknown_kind_in_the_source_entitlement` for the old
///   interval.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `old_interval`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::OldInterval }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_old_interval() {
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

/// Contract: as `an_unknown_kind_in_the_source_entitlement` for the new
///   interval.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `new_interval`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::NewInterval }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_new_interval() {
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

/// Contract: as `an_unknown_kind_in_the_source_entitlement` for the seller.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `seller`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::Seller }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_seller() {
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

/// Contract: as `an_unknown_kind_in_the_source_entitlement` for the buyer.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `buyer`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Mint, field:
///   ContextField::Buyer }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_buyer() {
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

/// Contract: a canonical refusal in the transfer context carries the transfer
///   relation.
/// Arrange: `statements` `build_statements_document` with `transfer`
///   `build_delivery_context_document` with `source_entitlement`
///   `build_canonical_field_document` over `"uint256"` and `Integer(1)`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::Context { relation: DeliveryRelation::Transfer,
///   field: ContextField::SourceEntitlement }, error:
///   CanonicalFieldAdmissionErrorReturn::UnknownKind { text:
///   "uint256".to_string() } })`.
#[test]
fn admit_statements_refuses_an_unknown_kind_in_the_transfer_context() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        transfer: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::CanonicalField {
            field: StatementField::Context {
                relation: DeliveryRelation::Transfer,
                field: ContextField::SourceEntitlement,
            },
            error: CanonicalFieldAdmissionErrorReturn::UnknownKind {
                text: "uint256".to_string(),
            },
        })
    );
}

/// Contract: text for an integer kind is `ExpectedInteger` carrying the kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned16"` and `Text("0")`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::ExpectedInteger { kind:
///   CanonicalFieldKind::Unsigned16 } })`.
#[test]
fn admit_statements_refuses_text_for_an_unsigned16_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned16".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text("0".to_string())),
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
            error: CanonicalFieldAdmissionErrorReturn::ExpectedInteger {
                kind: CanonicalFieldKind::Unsigned16,
            },
        })
    );
}

/// Contract: as `text_for_an_unsigned16_kind` for the 32-bit kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned32"` and `Text("0")`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::ExpectedInteger { kind:
///   CanonicalFieldKind::Unsigned32 } })`.
#[test]
fn admit_statements_refuses_text_for_an_unsigned32_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned32".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text("0".to_string())),
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
            error: CanonicalFieldAdmissionErrorReturn::ExpectedInteger {
                kind: CanonicalFieldKind::Unsigned32,
            },
        })
    );
}

/// Contract: as `text_for_an_unsigned16_kind` for the 64-bit kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned64"` and `Text("0")`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::ExpectedInteger { kind:
///   CanonicalFieldKind::Unsigned64 } })`.
#[test]
fn admit_statements_refuses_text_for_an_unsigned64_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned64".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text("0".to_string())),
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
            error: CanonicalFieldAdmissionErrorReturn::ExpectedInteger {
                kind: CanonicalFieldKind::Unsigned64,
            },
        })
    );
}

/// Contract: an integer for a kind that takes text is `ExpectedText` carrying
///   the kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"fixed_bytes32"` and `Integer(1)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::ExpectedText { kind:
///   CanonicalFieldKind::FixedBytes32 } })`.
#[test]
fn admit_statements_refuses_an_integer_for_a_fixed_bytes32_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes32".to_string()),
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
            error: CanonicalFieldAdmissionErrorReturn::ExpectedText {
                kind: CanonicalFieldKind::FixedBytes32,
            },
        })
    );
}

/// Contract: as `an_integer_for_a_fixed_bytes32_kind` for the 20-byte kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"fixed_bytes20"` and `Integer(1)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::ExpectedText { kind:
///   CanonicalFieldKind::FixedBytes20 } })`.
#[test]
fn admit_statements_refuses_an_integer_for_a_fixed_bytes20_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
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
            error: CanonicalFieldAdmissionErrorReturn::ExpectedText {
                kind: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
}

/// Contract: as `an_integer_for_a_fixed_bytes32_kind` for the byte-string
///   kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"bytes"` and `Integer(1)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::ExpectedText { kind:
///   CanonicalFieldKind::Bytes } })`.
#[test]
fn admit_statements_refuses_an_integer_for_a_bytes_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("bytes".to_string()),
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
            error: CanonicalFieldAdmissionErrorReturn::ExpectedText {
                kind: CanonicalFieldKind::Bytes,
            },
        })
    );
}

/// Contract: as `an_integer_for_a_fixed_bytes32_kind` for the text kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"text"` and `Integer(1)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::ExpectedText { kind:
///   CanonicalFieldKind::Text } })`.
#[test]
fn admit_statements_refuses_an_integer_for_a_text_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("text".to_string()),
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
            error: CanonicalFieldAdmissionErrorReturn::ExpectedText {
                kind: CanonicalFieldKind::Text,
            },
        })
    );
}

/// Contract: an integer beyond its kind's range is `OutOfRange` carrying the
///   kind and the value.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned16"` and
///   `Integer(65536)`; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::OutOfRange { kind:
///   CanonicalFieldKind::Unsigned16, value: 65536 } })`.
#[test]
fn admit_statements_refuses_an_integer_beyond_an_unsigned16_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned16".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(65536)),
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
            error: CanonicalFieldAdmissionErrorReturn::OutOfRange {
                kind: CanonicalFieldKind::Unsigned16,
                value: 65536,
            },
        })
    );
}

/// Contract: as `an_integer_beyond_an_unsigned16_kind` for the 32-bit kind.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned32"` and
///   `Integer(4294967296)`; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::OutOfRange { kind:
///   CanonicalFieldKind::Unsigned32, value: 4294967296 } })`.
#[test]
fn admit_statements_refuses_an_integer_beyond_an_unsigned32_kind() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned32".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(4294967296)),
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
            error: CanonicalFieldAdmissionErrorReturn::OutOfRange {
                kind: CanonicalFieldKind::Unsigned32,
                value: 4294967296,
            },
        })
    );
}

/// Contract: hex of a width other than its kind's is `WrongLength` carried in
///   `Hex` of the canonical refusal.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"fixed_bytes32"` and `Text` of
///   `0x` followed by sixty-two `1` digits; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::WrongLength {
///   expected: 32, actual: 31 }) })`.
#[test]
fn admit_statements_refuses_a_fixed_bytes32_field_of_the_wrong_width() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes32".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}",
                    "1".repeat(62)
                ))),
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
            error: CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::WrongLength {
                expected: 32,
                actual: 31,
            },),
        })
    );
}

/// Contract: as `a_fixed_bytes32_field_of_the_wrong_width` for the 20-byte
///   kind.
/// Arrange: `network` `build_network_document` with `entitlement_contract`
///   `build_canonical_field_document` over `"fixed_bytes20"` and `Text` of
///   `0x` followed by thirty-eight `0` digits; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkEntitlementContract, error:
///   CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::WrongLength {
///   expected: 20, actual: 19 }) })`.
#[test]
fn admit_statements_refuses_a_fixed_bytes20_field_of_the_wrong_width() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        entitlement_contract: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}",
                    "0".repeat(38)
                ))),
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
            field: StatementField::NetworkEntitlementContract,
            error: CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::WrongLength {
                expected: 20,
                actual: 19,
            },),
        })
    );
}

/// Contract: as `a_fixed_bytes32_field_of_the_wrong_width` for the 256-bit
///   kind's hex.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned256"` and `Text` of `0x`
///   followed by sixty-two `0` digits; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::WrongLength {
///   expected: 32, actual: 31 }) })`.
#[test]
fn admit_statements_refuses_an_unsigned256_field_of_the_wrong_width() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}",
                    "0".repeat(62)
                ))),
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
            error: CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::WrongLength {
                expected: 32,
                actual: 31,
            },),
        })
    );
}

/// Contract: the byte-string kind takes any even digit count and an odd one
///   is `OddDigitCount`.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"bytes"` and `Text("0x123")`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::CanonicalField {
///   field: StatementField::NetworkChain, error:
///   CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::OddDigitCount
///   { count: 3 }) })`.
#[test]
fn admit_statements_refuses_a_bytes_field_with_an_odd_digit_count() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("bytes".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text("0x123".to_string())),
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
            error: CanonicalFieldAdmissionErrorReturn::Hex(HexTextErrorReturn::OddDigitCount {
                count: 3
            },),
        })
    );
}

/// Contract: the suite identifier's refusal is carried with the relation.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `suite_version` `0`; `network` its
///   default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::SuiteIdentifier
///   { relation: DeliveryRelation::Mint, error:
///   SuiteIdentifierTryNewErrorReturn::ZeroVersion })`.
#[test]
fn admit_statements_refuses_a_zero_suite_version() {
    // Arrange
    let network = build_network_document(Default::default());
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
        Err(AdmitStatementsErrorReturn::SuiteIdentifier {
            relation: DeliveryRelation::Mint,
            error: SuiteIdentifierTryNewErrorReturn::ZeroVersion,
        })
    );
}

/// Contract: the refusal in the transfer context carries the transfer
///   relation.
/// Arrange: `statements` `build_statements_document` with `transfer`
///   `build_delivery_context_document` with `suite_version` `0`; `network` its
///   default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::SuiteIdentifier
///   { relation: DeliveryRelation::Transfer, error:
///   SuiteIdentifierTryNewErrorReturn::ZeroVersion })`.
#[test]
fn admit_statements_refuses_a_zero_suite_version_in_the_transfer_context() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::SuiteIdentifier {
            relation: DeliveryRelation::Transfer,
            error: SuiteIdentifierTryNewErrorReturn::ZeroVersion,
        })
    );
}

/// Contract: an all-zero identifier is the constructor's `AllZeroIdentifier`.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `suite_identifier` `0x` followed
///   by sixty-four `0` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::SuiteIdentifier
///   { relation: DeliveryRelation::Mint, error:
///   SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier })`.
#[test]
fn admit_statements_refuses_an_all_zero_suite_identifier() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_identifier: Some(format!("0x{}", "0".repeat(64))),
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
            error: SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier,
        })
    );
}

/// Contract: the asset identity hash's refusal is carried with the relation.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `asset_identity_hash` `0x`
///   followed by sixty-four `0` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::AssetIdentityHash
///   { relation: DeliveryRelation::Mint, error:
///   AssetIdentityHashTryNewErrorReturn::AllZero })`.
#[test]
fn admit_statements_refuses_an_all_zero_asset_identity_hash() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::AssetIdentityHash {
            relation: DeliveryRelation::Mint,
            error: AssetIdentityHashTryNewErrorReturn::AllZero,
        })
    );
}

/// Contract: the refusal in the transfer context carries the transfer
///   relation.
/// Arrange: `statements` `build_statements_document` with `transfer`
///   `build_delivery_context_document` with `asset_identity_hash` `0x`
///   followed by sixty-four `0` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::AssetIdentityHash
///   { relation: DeliveryRelation::Transfer, error:
///   AssetIdentityHashTryNewErrorReturn::AllZero })`.
#[test]
fn admit_statements_refuses_an_all_zero_asset_identity_hash_in_the_transfer_context() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        transfer: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::AssetIdentityHash {
            relation: DeliveryRelation::Transfer,
            error: AssetIdentityHashTryNewErrorReturn::AllZero,
        })
    );
}

/// Contract: the parameter-set identifier's refusal is carried with the
///   relation.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `parameter_set_identifier` `0x`
///   followed by sixty-four `0` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals
///   `Err(AdmitStatementsErrorReturn::ParameterSetIdentifier { relation:
///   DeliveryRelation::Mint, error:
///   ParameterSetIdentifierTryNewErrorReturn::AllZero })`.
#[test]
fn admit_statements_refuses_an_all_zero_parameter_set_identifier() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::ParameterSetIdentifier {
            relation: DeliveryRelation::Mint,
            error: ParameterSetIdentifierTryNewErrorReturn::AllZero,
        })
    );
}

/// Contract: the refusal in the transfer context carries the transfer
///   relation.
/// Arrange: `statements` `build_statements_document` with `transfer`
///   `build_delivery_context_document` with `parameter_set_identifier` `0x`
///   followed by sixty-four `0` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals
///   `Err(AdmitStatementsErrorReturn::ParameterSetIdentifier { relation:
///   DeliveryRelation::Transfer, error:
///   ParameterSetIdentifierTryNewErrorReturn::AllZero })`.
#[test]
fn admit_statements_refuses_an_all_zero_parameter_set_identifier_in_the_transfer_context() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        transfer: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::ParameterSetIdentifier {
            relation: DeliveryRelation::Transfer,
            error: ParameterSetIdentifierTryNewErrorReturn::AllZero,
        })
    );
}

/// Contract: a code the mint relation does not declare is refused with the
///   conversion's error.
/// Arrange: `statements` `build_statements_document` with `mint_purpose` `2`;
///   `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::MintPurpose(
///   UndeclaredDeliveryPurposeCode { code: 2 }))`.
#[test]
fn admit_statements_refuses_an_undeclared_mint_purpose() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::MintPurpose(
            UndeclaredDeliveryPurposeCode { code: 2 }
        ))
    );
}

/// Contract: a code the transfer relation does not declare is refused with
///   the conversion's error.
/// Arrange: `statements` `build_statements_document` with `transfer_purpose`
///   `1`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitStatementsErrorReturn::TransferPurpose(
///   UndeclaredDeliveryPurposeCode { code: 1 }))`.
#[test]
fn admit_statements_refuses_an_undeclared_transfer_purpose() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
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
        Err(AdmitStatementsErrorReturn::TransferPurpose(
            UndeclaredDeliveryPurposeCode { code: 1 }
        ))
    );
}
