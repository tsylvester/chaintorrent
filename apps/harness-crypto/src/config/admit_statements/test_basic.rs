#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use domain::{
    AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::CanonicalFieldValue;
use proof::{MintPurpose, TransferPurpose};

use super::super::layers::provides::{
    CanonicalFieldDocumentOverrides, CanonicalFieldDocumentValue, DeliveryContextDocumentOverrides,
    NetworkDocumentOverrides, StatementsDocumentOverrides, build_canonical_field_document,
    build_delivery_context_document, build_network_document, build_statements_document,
};
use super::admit_statements;
use super::mock::{
    build_admit_statements_deps, build_admit_statements_params, build_admit_statements_payload,
};

/// Contract: the network profile's chain is `network.chain` admitted and its
///   entitlement contract is `network.entitlement_contract` admitted.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned64"` and `Integer(11)`
///   and `entitlement_contract` `build_canonical_field_document` over
///   `"fixed_bytes20"` and `Text` of `0x` followed by forty `a` digits;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Unsigned64(11)` and its
///   `network.entitlement_contract` equals
///   `CanonicalFieldValue::FixedBytes20([0xaa; 20])`.
#[test]
fn admit_statements_reads_the_network_profile_from_the_network_document() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned64".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(11)),
            },
        )),
        entitlement_contract: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}",
                    "a".repeat(40)
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.network.chain, CanonicalFieldValue::Unsigned64(11));
    assert_eq!(
        success.network.entitlement_contract,
        CanonicalFieldValue::FixedBytes20([0xaa; 20])
    );
}

/// Contract: kind `"unsigned16"` with an integer →
///   `CanonicalFieldValue::Unsigned16` of it.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned16"` and `Integer(7)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Unsigned16(7)`.
#[test]
fn admit_statements_maps_an_unsigned16_field() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned16".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(7)),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.network.chain, CanonicalFieldValue::Unsigned16(7));
}

/// Contract: kind `"unsigned32"` with an integer →
///   `CanonicalFieldValue::Unsigned32` of it.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned32"` and `Integer(9)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Unsigned32(9)`.
#[test]
fn admit_statements_maps_an_unsigned32_field() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned32".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(9)),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.network.chain, CanonicalFieldValue::Unsigned32(9));
}

/// Contract: kind `"unsigned64"` with an integer →
///   `CanonicalFieldValue::Unsigned64` of it.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned64"` and `Integer(8453)`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Unsigned64(8453)`.
#[test]
fn admit_statements_maps_an_unsigned64_field() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned64".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(8453)),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.network.chain, CanonicalFieldValue::Unsigned64(8453));
}

/// Contract: kind `"unsigned256"` with an integer → the 32 bytes whose last
///   eight are the integer's big-endian bytes and the rest zero.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned256"` and
///   `Integer(72623859790382856)`; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Unsigned256` of the 32 bytes whose first
///   twenty-four are `0` and last eight are
///   `[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]`.
#[test]
fn admit_statements_maps_an_unsigned256_from_an_integer() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(72623859790382856)),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    let mut expected = [0x00; 32];
    expected[24..].copy_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    assert_eq!(
        success.network.chain,
        CanonicalFieldValue::Unsigned256(expected)
    );
}

/// Contract: kind `"unsigned256"` with hex text → the hex's 32 bytes.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned256"` and `Text` of `0x`
///   followed by fifty-nine `0` digits and `14a34`; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Unsigned256` of the 32 bytes whose last eight are
///   `84532u64.to_be_bytes()`.
#[test]
fn admit_statements_maps_an_unsigned256_from_hex_text() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}14a34",
                    "0".repeat(59)
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    let mut expected = [0x00; 32];
    expected[24..].copy_from_slice(&84532u64.to_be_bytes());
    assert_eq!(
        success.network.chain,
        CanonicalFieldValue::Unsigned256(expected)
    );
}

/// Contract: kind `"fixed_bytes32"` with hex text →
///   `CanonicalFieldValue::FixedBytes32` of the hex's 32 bytes.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"fixed_bytes32"` and `Text` of
///   `0x` followed by sixty-four `1` digits; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::FixedBytes32([0x11; 32])`.
#[test]
fn admit_statements_maps_a_fixed_bytes32_field() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes32".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}",
                    "1".repeat(64)
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.network.chain,
        CanonicalFieldValue::FixedBytes32([0x11; 32])
    );
}

/// Contract: kind `"fixed_bytes20"` with hex text →
///   `CanonicalFieldValue::FixedBytes20` of the hex's 20 bytes.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"fixed_bytes20"` and `Text` of
///   `0x` followed by forty `a` digits; `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::FixedBytes20([0xaa; 20])`.
#[test]
fn admit_statements_maps_a_fixed_bytes20_field() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}",
                    "a".repeat(40)
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.network.chain,
        CanonicalFieldValue::FixedBytes20([0xaa; 20])
    );
}

/// Contract: kind `"bytes"` with hex text → `CanonicalFieldValue::Bytes` of
///   the hex's bytes.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"bytes"` and `Text("0x0102")`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Bytes(vec![0x01, 0x02])`.
#[test]
fn admit_statements_maps_a_bytes_field() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("bytes".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text("0x0102".to_string())),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.network.chain,
        CanonicalFieldValue::Bytes(vec![0x01, 0x02])
    );
}

/// Contract: kind `"text"` with text → `CanonicalFieldValue::Text` of the
///   text as it is.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"text"` and `Text("issuer")`;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `network.chain` equals
///   `CanonicalFieldValue::Text("issuer".to_string())`.
#[test]
fn admit_statements_maps_a_text_field() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("text".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text("issuer".to_string())),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.network.chain,
        CanonicalFieldValue::Text("issuer".to_string())
    );
}

/// Contract: the mint context is admitted from `statements.mint` and the
///   transfer context from `statements.transfer`.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `expiry` `1_000` and `transfer`
///   `build_delivery_context_document` with `expiry` `2_000`; `network` its
///   default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.mint.expiry` equals `1_000` and
///   `statements.transfer.expiry` equals `2_000`.
#[test]
fn admit_statements_reads_each_context_from_its_own_document() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                expiry: Some(1_000),
                ..Default::default()
            },
        )),
        transfer: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                expiry: Some(2_000),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.statements.mint.expiry, 1_000);
    assert_eq!(success.statements.transfer.expiry, 2_000);
}

/// Contract: the suite identifier, the asset identity hash, and the
///   parameter-set identifier are each admitted from their own document
///   field.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` with `suite_identifier` `0x` followed
///   by sixty-four `1` digits, `suite_version` `7`, `asset_identity_hash`
///   `0x` followed by sixty-four `2` digits, and `parameter_set_identifier`
///   `0x` followed by sixty-four `3` digits; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.mint.suite_identifier` equals
///   `build_suite_identifier` over `SuiteIdentifierConstructorParamsOverrides`
///   with `identifier` `[0x11; 32]` and `version` `7`, its
///   `asset_identity_hash` equals `build_asset_identity_hash` over
///   `AssetIdentityHashConstructorParamsOverrides` with `bytes` `[0x22; 32]`,
///   and its `parameter_set_identifier` equals
///   `build_parameter_set_identifier` over
///   `ParameterSetIdentifierConstructorParamsOverrides` with `bytes`
///   `[0x33; 32]`.
#[test]
fn admit_statements_maps_each_context_identifier_from_its_own_field() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                suite_identifier: Some(format!("0x{}", "1".repeat(64))),
                suite_version: Some(7),
                asset_identity_hash: Some(format!("0x{}", "2".repeat(64))),
                parameter_set_identifier: Some(format!("0x{}", "3".repeat(64))),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.statements.mint.suite_identifier,
        build_suite_identifier(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some([0x11; 32]),
            version: Some(7),
        })
    );
    assert_eq!(
        success.statements.mint.asset_identity_hash,
        build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
            bytes: Some([0x22; 32]),
        })
    );
    assert_eq!(
        success.statements.mint.parameter_set_identifier,
        build_parameter_set_identifier(ParameterSetIdentifierConstructorParamsOverrides {
            bytes: Some([0x33; 32]),
        })
    );
}

/// Contract: the source entitlement, target entitlement, old interval, new
///   interval, seller, and buyer are each admitted from their own document
///   field.
/// Arrange: `statements` `build_statements_document` with `mint`
///   `build_delivery_context_document` whose six fields are each
///   `build_canonical_field_document` over `"unsigned64"` and `Integer` of
///   `1`, `2`, `3`, `4`, `5`, and `6` in that order; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.mint` fields in that order equal
///   `CanonicalFieldValue::Unsigned64` of `1`, `2`, `3`, `4`, `5`, and `6`.
#[test]
fn admit_statements_reads_each_canonical_context_field_from_its_own_document_field() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint: Some(build_delivery_context_document(
            DeliveryContextDocumentOverrides {
                source_entitlement: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("unsigned64".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(1)),
                    },
                )),
                target_entitlement: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("unsigned64".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(2)),
                    },
                )),
                old_interval: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("unsigned64".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(3)),
                    },
                )),
                new_interval: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("unsigned64".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(4)),
                    },
                )),
                seller: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("unsigned64".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(5)),
                    },
                )),
                buyer: Some(build_canonical_field_document(
                    CanonicalFieldDocumentOverrides {
                        kind: Some("unsigned64".to_string()),
                        value: Some(CanonicalFieldDocumentValue::Integer(6)),
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.statements.mint.source_entitlement,
        CanonicalFieldValue::Unsigned64(1)
    );
    assert_eq!(
        success.statements.mint.target_entitlement,
        CanonicalFieldValue::Unsigned64(2)
    );
    assert_eq!(
        success.statements.mint.old_interval,
        CanonicalFieldValue::Unsigned64(3)
    );
    assert_eq!(
        success.statements.mint.new_interval,
        CanonicalFieldValue::Unsigned64(4)
    );
    assert_eq!(
        success.statements.mint.seller,
        CanonicalFieldValue::Unsigned64(5)
    );
    assert_eq!(
        success.statements.mint.buyer,
        CanonicalFieldValue::Unsigned64(6)
    );
}

/// Contract: each context's chain identifier and entitlement contract are
///   the network profile's admitted values.
/// Arrange: `network` `build_network_document` with `chain`
///   `build_canonical_field_document` over `"unsigned64"` and `Integer(8453)`
///   and `entitlement_contract` `build_canonical_field_document` over
///   `"fixed_bytes20"` and `Text` of `0x` followed by forty `a` digits;
///   `statements` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.mint.chain` and `statements.transfer.chain`
///   each equal `CanonicalFieldValue::Unsigned64(8453)`, and
///   `statements.mint.entitlement_contract` and
///   `statements.transfer.entitlement_contract` each equal
///   `CanonicalFieldValue::FixedBytes20([0xaa; 20])`.
#[test]
fn admit_statements_fills_each_context_with_the_network_profiles_chain_and_contract() {
    // Arrange
    let network = build_network_document(NetworkDocumentOverrides {
        chain: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("unsigned64".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(8453)),
            },
        )),
        entitlement_contract: Some(build_canonical_field_document(
            CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(format!(
                    "0x{}",
                    "a".repeat(40)
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
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.statements.mint.chain,
        CanonicalFieldValue::Unsigned64(8453)
    );
    assert_eq!(
        success.statements.transfer.chain,
        CanonicalFieldValue::Unsigned64(8453)
    );
    assert_eq!(
        success.statements.mint.entitlement_contract,
        CanonicalFieldValue::FixedBytes20([0xaa; 20])
    );
    assert_eq!(
        success.statements.transfer.entitlement_contract,
        CanonicalFieldValue::FixedBytes20([0xaa; 20])
    );
}

/// Contract: the mint purpose is admitted through `MintPurpose`'s conversion
///   and the transfer purpose through `TransferPurpose`'s.
/// Arrange: `statements` `build_statements_document` with `mint_purpose` `4`
///   and `transfer_purpose` `3`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.mint_purpose` equals
///   `MintPurpose::Replacement` and `statements.transfer_purpose` equals
///   `TransferPurpose::Grant`.
#[test]
fn admit_statements_maps_the_replacement_and_grant_purposes() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        mint_purpose: Some(4),
        transfer_purpose: Some(3),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.statements.mint_purpose, MintPurpose::Replacement);
    assert_eq!(success.statements.transfer_purpose, TransferPurpose::Grant);
}

/// Contract: the identities are decoded in the order the document lists them.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x01"`, `"0x02"`, and `"0x03"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.identities` equals
///   `vec![vec![0x01], vec![0x02], vec![0x03]]`.
#[test]
fn admit_statements_decodes_each_identity_in_order() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec![
            "0x01".to_string(),
            "0x02".to_string(),
            "0x03".to_string(),
        ]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(
        success.statements.identities,
        vec![vec![0x01], vec![0x02], vec![0x03]]
    );
}

/// Contract: two digits make a byte, the high digit first.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x1f2e"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.identities` equals
///   `vec![vec![0x1f, 0x2e]]`.
#[test]
fn admit_statements_decodes_a_hex_digit_pair_high_digit_first() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0x1f2e".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.statements.identities, vec![vec![0x1f, 0x2e]]);
}

/// Contract: each digit is `0`–`9`, `a`–`f`, or `A`–`F`.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0xAbCd"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.identities` equals
///   `vec![vec![0xab, 0xcd]]`.
#[test]
fn admit_statements_decodes_uppercase_and_lowercase_digits() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0xAbCd".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.statements.identities, vec![vec![0xab, 0xcd]]);
}

/// Contract: hex of no digits is no bytes.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0x"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.identities` equals `vec![Vec::new()]`.
#[test]
fn admit_statements_decodes_a_prefix_alone_to_no_bytes() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0x".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.statements.identities, vec![Vec::<u8>::new()]);
}

/// Contract: the proof identity is decoded from `statements.proof_identity`.
/// Arrange: `statements` `build_statements_document` with `identities`
///   `"0xaa"` and `proof_identity` `"0xbb02"`; `network` its default.
/// Act:    `admit_statements(&deps, params, payload)`.
/// Assert: the success's `statements.proof_identity` equals
///   `vec![0xbb, 0x02]`.
#[test]
fn admit_statements_decodes_the_proof_identity_from_its_own_field() {
    // Arrange
    let network = build_network_document(Default::default());
    let statements = build_statements_document(StatementsDocumentOverrides {
        identities: Some(vec!["0xaa".to_string()]),
        proof_identity: Some("0xbb02".to_string()),
        ..Default::default()
    });
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let payload = build_admit_statements_payload(&network, &statements, Default::default());

    // Act
    let result = admit_statements(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the document admits");
    };
    assert_eq!(success.statements.proof_identity, vec![0xbb, 0x02]);
}
