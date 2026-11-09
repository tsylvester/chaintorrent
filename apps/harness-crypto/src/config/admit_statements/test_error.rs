#![allow(clippy::expect_used)]

use domain::{ParameterSetIdentifierTryNewErrorReturn, SuiteIdentifierTryNewErrorReturn};
use encoding::{CanonicalFieldKind, CanonicalFieldValue};
use hashing::RootTryNewErrorReturn;
use kem::IdentityScope;
use proof::UndeclaredDeliveryPurposeCode;

use super::admit_statements;
use super::interface::{
    AdmitStatementsErrorReturn, CanonicalValueRefusal, ContextField, ContextRefusal,
    ContextRelation, HexRefusal, IdentityField, IdentityRefusal, NetworkProfileField,
};
use super::mock::{
    build_admit_statements_deps, build_admit_statements_params, build_admit_statements_payload,
    declared_name,
};

/// Contract: Network profile → `Err(AdmitStatementsErrorReturn::NetworkProfile
///   { field, refusal })` with `CanonicalValueRefusal::UnknownKind { text }`.
/// Arrange: the default payload with `chain_identifier` of kind `"unknown"`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals `AdmitStatementsErrorReturn::
///   NetworkProfile { field: NetworkProfileField::ChainIdentifier, refusal:
///   CanonicalValueRefusal::UnknownKind { text: "unknown".to_string() } }`.
#[test]
fn admit_statements_refuses_an_unknown_network_profile_kind() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.chain_identifier.kind = "unknown".to_string();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::ChainIdentifier,
            refusal: CanonicalValueRefusal::UnknownKind {
                text: "unknown".to_string()
            }
        })
    );
    // admit_statements_refuses_an_unknown_network_profile_kind
}

/// Contract: the naming rule → a kind text equals a member's `Debug` text
///   whole and case-sensitively.
/// Arrange: the default payload with `entitlement_contract` of the lowercase
///   of `declared_name(&CanonicalFieldKind::FixedBytes20)`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals `AdmitStatementsErrorReturn::
///   NetworkProfile { field: NetworkProfileField::EntitlementContract,
///   refusal: CanonicalValueRefusal::UnknownKind { text } }` with `text` the
///   lowercase text.
#[test]
fn admit_statements_refuses_a_kind_name_that_differs_only_in_case() {
    // Arrange
    let text = declared_name(&CanonicalFieldKind::FixedBytes20).to_lowercase();
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.entitlement_contract.kind = text.clone();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::EntitlementContract,
            refusal: CanonicalValueRefusal::UnknownKind { text }
        })
    );
    // admit_statements_refuses_a_kind_name_that_differs_only_in_case
}

/// Contract: Network profile → the refusal names the field of
///   `network_profile` that held the refused value, here `gas_price_oracle`.
/// Arrange: the default payload with `gas_price_oracle` of kind `"unknown"`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals `AdmitStatementsErrorReturn::
///   NetworkProfile { field: NetworkProfileField::GasPriceOracle, refusal:
///   CanonicalValueRefusal::UnknownKind { text: "unknown".to_string() } }`.
#[test]
fn admit_statements_refuses_the_network_profile_field_that_held_the_refused_value() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.gas_price_oracle.kind = "unknown".to_string();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::GasPriceOracle,
            refusal: CanonicalValueRefusal::UnknownKind {
                text: "unknown".to_string()
            }
        })
    );
    // admit_statements_refuses_the_network_profile_field_that_held_the_refused_value
}

/// Contract: Hex → a text lacking `0x` is
///   `CanonicalValueRefusal::Hex(HexRefusal::Malformed { text })`.
/// Arrange: the default payload with `entitlement_contract` of kind
///   `FixedBytes20` and 40 digits without the prefix.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `NetworkProfile` refusal at
///   `EntitlementContract` with `Hex(HexRefusal::Malformed { text })` carrying
///   that text.
#[test]
fn admit_statements_refuses_a_hex_text_without_the_prefix() {
    // Arrange
    let text = "11".repeat(20);
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.entitlement_contract.value = text.clone();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::EntitlementContract,
            refusal: CanonicalValueRefusal::Hex(HexRefusal::Malformed { text })
        })
    );
    // admit_statements_refuses_a_hex_text_without_the_prefix
}

/// Contract: Hex → an odd count of digits is malformed.
/// Arrange: the default payload with `entitlement_contract` of kind
///   `FixedBytes20` and `0x` followed by 39 digits.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `NetworkProfile` refusal with
///   `Hex(HexRefusal::Malformed { text })`.
#[test]
fn admit_statements_refuses_a_hex_text_of_an_odd_digit_count() {
    // Arrange
    let text = format!("0x{}", "1".repeat(39));
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.entitlement_contract.value = text.clone();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::EntitlementContract,
            refusal: CanonicalValueRefusal::Hex(HexRefusal::Malformed { text })
        })
    );
    // admit_statements_refuses_a_hex_text_of_an_odd_digit_count
}

/// Contract: Hex → a character outside the hexadecimal digits is malformed.
/// Arrange: the default payload with `entitlement_contract` of kind
///   `FixedBytes20` and `0x` followed by 39 digits and `g`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `NetworkProfile` refusal with
///   `Hex(HexRefusal::Malformed { text })`.
#[test]
fn admit_statements_refuses_a_hex_text_with_a_character_that_is_not_a_digit() {
    // Arrange
    let text = format!("0x{}g", "1".repeat(39));
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.entitlement_contract.value = text.clone();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::EntitlementContract,
            refusal: CanonicalValueRefusal::Hex(HexRefusal::Malformed { text })
        })
    );
    // admit_statements_refuses_a_hex_text_with_a_character_that_is_not_a_digit
}

/// Contract: Hex → the prefix is `0x` whole and case-sensitively.
/// Arrange: the default payload with `entitlement_contract` of kind
///   `FixedBytes20` and `0X` followed by 40 digits.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `NetworkProfile` refusal with
///   `Hex(HexRefusal::Malformed { text })`.
#[test]
fn admit_statements_refuses_an_uppercase_prefix() {
    // Arrange
    let text = format!("0X{}", "11".repeat(20));
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.entitlement_contract.value = text.clone();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::EntitlementContract,
            refusal: CanonicalValueRefusal::Hex(HexRefusal::Malformed { text })
        })
    );
    // admit_statements_refuses_an_uppercase_prefix
}

/// Contract: Hex → a width other than the kind's is `HexRefusal::WrongWidth
///   { expected, actual }`.
/// Arrange: the default payload with `entitlement_contract` of kind
///   `FixedBytes20` and `0x` followed by 19 bytes.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `NetworkProfile` refusal with
///   `Hex(HexRefusal::WrongWidth { expected: 20, actual: 19 })`.
#[test]
fn admit_statements_refuses_a_fixed_width_text_of_the_wrong_width() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.entitlement_contract.value = format!("0x{}", "11".repeat(19));
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::NetworkProfile {
            field: NetworkProfileField::EntitlementContract,
            refusal: CanonicalValueRefusal::Hex(HexRefusal::WrongWidth {
                expected: 20,
                actual: 19
            })
        })
    );
    // admit_statements_refuses_a_fixed_width_text_of_the_wrong_width
}

/// Contract: Decimal → an empty text, a sign, and a character outside the
///   digits are `CanonicalValueRefusal::MalformedDecimal { text }`.
/// Arrange: the default payload with `chain_identifier` of kind `Unsigned64`
///   and text `""`, then `"+1"`, then `"1a"`, then `"-1"`.
/// Act:     `admit_statements`, once per text.
/// Assert:  each result's error equals the `NetworkProfile` refusal at
///   `ChainIdentifier` with `MalformedDecimal { text }` carrying that text.
#[test]
fn admit_statements_refuses_a_decimal_text_that_is_empty_or_not_digits() {
    // Arrange — the deps and params are invariant across every text.
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    for text in ["", "+1", "1a", "-1"] {
        let mut payload = build_admit_statements_payload(Default::default());
        payload.network_profile.chain_identifier.kind =
            declared_name(&CanonicalFieldKind::Unsigned64);
        payload.network_profile.chain_identifier.value = text.to_string();
        // Act
        let result = admit_statements(&deps, params, payload);
        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitStatementsErrorReturn::NetworkProfile {
                field: NetworkProfileField::ChainIdentifier,
                refusal: CanonicalValueRefusal::MalformedDecimal {
                    text: text.to_string()
                }
            })
        );
    }
    // admit_statements_refuses_a_decimal_text_that_is_empty_or_not_digits
}

/// Contract: Range → a text above the maximum is
///   `CanonicalValueRefusal::OutOfRange { text }`.
/// Arrange: the default payload with `chain_identifier` of kind `Unsigned16`
///   and `"65536"`, then `Unsigned32` and `"4294967296"`, then `Unsigned64`
///   and `"18446744073709551616"`, then `Unsigned256` and the 256-bit
///   maximum's decimal text plus one.
/// Act:     `admit_statements`, once per payload.
/// Assert:  each result's error equals the `NetworkProfile` refusal with
///   `OutOfRange { text }` carrying that text.
#[test]
fn admit_statements_refuses_a_decimal_above_the_kinds_maximum() {
    // Arrange — each case is the kind and the text one above its maximum.
    let cases: Vec<(CanonicalFieldKind, &str)> = vec![
        (CanonicalFieldKind::Unsigned16, "65536"),
        (CanonicalFieldKind::Unsigned32, "4294967296"),
        (CanonicalFieldKind::Unsigned64, "18446744073709551616"),
        (
            CanonicalFieldKind::Unsigned256,
            "115792089237316195423570985008687907853269984665640564039457584007913129639936",
        ),
    ];
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    for (kind, text) in cases {
        let mut payload = build_admit_statements_payload(Default::default());
        payload.network_profile.chain_identifier.kind = declared_name(&kind);
        payload.network_profile.chain_identifier.value = text.to_string();
        // Act
        let result = admit_statements(&deps, params, payload);
        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitStatementsErrorReturn::NetworkProfile {
                field: NetworkProfileField::ChainIdentifier,
                refusal: CanonicalValueRefusal::OutOfRange {
                    text: text.to_string()
                }
            })
        );
    }
    // admit_statements_refuses_a_decimal_above_the_kinds_maximum
}

/// Contract: Context → a refused canonical value names the relation and the
///   key that held it.
/// Arrange: the default payload with the `transfer` table's `old_interval` of
///   kind `"unknown"`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals `AdmitStatementsErrorReturn::Context {
///   relation: ContextRelation::Transfer, field: ContextField::OldInterval,
///   refusal: ContextRefusal::Value(CanonicalValueRefusal::UnknownKind
///   { text: "unknown".to_string() }) }`.
#[test]
fn admit_statements_refuses_a_context_value_with_its_relation_and_field() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.statements.transfer.old_interval.kind = "unknown".to_string();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Transfer,
            field: ContextField::OldInterval,
            refusal: ContextRefusal::Value(CanonicalValueRefusal::UnknownKind {
                text: "unknown".to_string()
            })
        })
    );
    // admit_statements_refuses_a_context_value_with_its_relation_and_field
}

/// Contract: Context → each relation's table names itself in the refusal.
/// Arrange: the default payload with the `mint` table's `buyer` kind
///   `"unknown"`, then the `transfer` table's.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the results' errors equal the `Context` refusal at
///   `ContextField::Buyer` with `ContextRelation::Mint` and
///   `ContextRelation::Transfer`.
#[test]
fn admit_statements_names_the_relation_that_held_the_refused_value() {
    // Arrange — the deps and params are invariant; each case is the relation
    // whose `buyer` holds the unknown kind.
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    let cases = [ContextRelation::Mint, ContextRelation::Transfer];
    for relation in cases {
        let mut payload = build_admit_statements_payload(Default::default());
        match relation {
            ContextRelation::Mint => {
                payload.statements.mint.buyer.kind = "unknown".to_string();
            }
            ContextRelation::Transfer => {
                payload.statements.transfer.buyer.kind = "unknown".to_string();
            }
        }
        // Act
        let result = admit_statements(&deps, params, payload);
        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitStatementsErrorReturn::Context {
                relation,
                field: ContextField::Buyer,
                refusal: ContextRefusal::Value(CanonicalValueRefusal::UnknownKind {
                    text: "unknown".to_string()
                })
            })
        );
    }
    // admit_statements_names_the_relation_that_held_the_refused_value
}

/// Contract: Context → a suite identifier text that is not hex of the
///   declared width is `ContextRefusal::Hex(refusal)` at
///   `ContextField::SuiteIdentifier`.
/// Arrange: the default payload with the `mint` table's `suite_identifier`
///   `0x` followed by 31 bytes.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `Context` refusal at
///   `ContextRelation::Mint`, `ContextField::SuiteIdentifier` with
///   `Hex(HexRefusal::WrongWidth { expected: 32, actual: 31 })`.
#[test]
fn admit_statements_refuses_a_malformed_suite_identifier_text() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.statements.mint.suite_identifier = format!("0x{}", "22".repeat(31));
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Mint,
            field: ContextField::SuiteIdentifier,
            refusal: ContextRefusal::Hex(HexRefusal::WrongWidth {
                expected: 32,
                actual: 31
            })
        })
    );
    // admit_statements_refuses_a_malformed_suite_identifier_text
}

/// Contract: Context → a version above `u16::MAX` is
///   `ContextRefusal::SuiteVersion { version }`.
/// Arrange: the default payload with the `mint` table's `suite_version`
///   `65536`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `Context` refusal at
///   `ContextRelation::Mint`, `ContextField::SuiteVersion` with
///   `SuiteVersion { version: 65536 }`.
#[test]
fn admit_statements_refuses_a_suite_version_above_sixteen_bits() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.statements.mint.suite_version = 65536;
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Mint,
            field: ContextField::SuiteVersion,
            refusal: ContextRefusal::SuiteVersion { version: 65536 }
        })
    );
    // admit_statements_refuses_a_suite_version_above_sixteen_bits
}

/// Contract: Context → the domain constructor's refusal is carried unchanged.
/// Arrange: the default payload with the `mint` table's `suite_identifier` of
///   32 zero bytes, then `suite_version` `0`.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the results' errors equal the `Context` refusal at
///   `ContextField::SuiteIdentifier` with
///   `Suite(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)` and
///   `Suite(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`.
#[test]
fn admit_statements_returns_the_suite_identifiers_refusals_whole() {
    // Arrange — a suite identifier of 32 zero bytes.
    let mut all_zero = build_admit_statements_payload(Default::default());
    all_zero.statements.mint.suite_identifier = format!("0x{}", "00".repeat(32));
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        all_zero,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Mint,
            field: ContextField::SuiteIdentifier,
            refusal: ContextRefusal::Suite(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)
        })
    );

    // Arrange — a suite version of zero.
    let mut zero_version = build_admit_statements_payload(Default::default());
    zero_version.statements.mint.suite_version = 0;
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        zero_version,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Mint,
            field: ContextField::SuiteIdentifier,
            refusal: ContextRefusal::Suite(SuiteIdentifierTryNewErrorReturn::ZeroVersion)
        })
    );
    // admit_statements_returns_the_suite_identifiers_refusals_whole
}

/// Contract: Context → an asset root of the wrong width is a hex refusal and
///   an all-zero root is the root type's refusal, both at
///   `ContextField::AssetRoot`.
/// Arrange: the default payload with the `transfer` table's `asset_root` 31
///   bytes, then 32 zero bytes.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the results' errors equal the `Context` refusal at
///   `ContextRelation::Transfer`, `ContextField::AssetRoot` with
///   `Hex(HexRefusal::WrongWidth { expected: 32, actual: 31 })` and
///   `Root(RootTryNewErrorReturn::AllZero)`.
#[test]
fn admit_statements_returns_the_roots_refusals_whole() {
    // Arrange — an asset root of 31 bytes.
    let mut wrong_width = build_admit_statements_payload(Default::default());
    wrong_width.statements.transfer.asset_root = format!("0x{}", "44".repeat(31));
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        wrong_width,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Transfer,
            field: ContextField::AssetRoot,
            refusal: ContextRefusal::Hex(HexRefusal::WrongWidth {
                expected: 32,
                actual: 31
            })
        })
    );

    // Arrange — an asset root of 32 zero bytes.
    let mut all_zero = build_admit_statements_payload(Default::default());
    all_zero.statements.transfer.asset_root = format!("0x{}", "00".repeat(32));
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        all_zero,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Transfer,
            field: ContextField::AssetRoot,
            refusal: ContextRefusal::Root(RootTryNewErrorReturn::AllZero)
        })
    );
    // admit_statements_returns_the_roots_refusals_whole
}

/// Contract: Context → a parameter-set identifier of the wrong width is a
///   hex refusal and an all-zero one is the domain constructor's refusal.
/// Arrange: the default payload with the `transfer` table's
///   `parameter_set_identifier` 31 bytes, then 32 zero bytes.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the results' errors equal the `Context` refusal at
///   `ContextRelation::Transfer`, `ContextField::ParameterSetIdentifier` with
///   `Hex(HexRefusal::WrongWidth { expected: 32, actual: 31 })` and
///   `ParameterSet(ParameterSetIdentifierTryNewErrorReturn::AllZero)`.
#[test]
fn admit_statements_returns_the_parameter_set_identifiers_refusals_whole() {
    // Arrange — a parameter-set identifier of 31 bytes.
    let mut wrong_width = build_admit_statements_payload(Default::default());
    wrong_width.statements.transfer.parameter_set_identifier = format!("0x{}", "33".repeat(31));
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        wrong_width,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Transfer,
            field: ContextField::ParameterSetIdentifier,
            refusal: ContextRefusal::Hex(HexRefusal::WrongWidth {
                expected: 32,
                actual: 31
            })
        })
    );

    // Arrange — a parameter-set identifier of 32 zero bytes.
    let mut all_zero = build_admit_statements_payload(Default::default());
    all_zero.statements.transfer.parameter_set_identifier = format!("0x{}", "00".repeat(32));
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        all_zero,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Transfer,
            field: ContextField::ParameterSetIdentifier,
            refusal: ContextRefusal::ParameterSet(ParameterSetIdentifierTryNewErrorReturn::AllZero)
        })
    );
    // admit_statements_returns_the_parameter_set_identifiers_refusals_whole
}

/// Contract: Purposes → a mint context refuses a transfer code and a
///   transfer context refuses a mint code, each with the code.
/// Arrange: the default payload with the `mint` table's `purpose` `2`, then
///   the `transfer` table's `purpose` `1`, then the `transfer` table's
///   `purpose` `4`, then the `mint` table's `purpose` `0`.
/// Act:     `admit_statements`, once per payload.
/// Assert:  each result's error equals the `Context` refusal at
///   `ContextField::Purpose` with `Purpose(UndeclaredDeliveryPurposeCode
///   { code })` carrying that code, and the relation of the refused table.
#[test]
fn admit_statements_refuses_a_purpose_code_of_the_other_relation() {
    // Arrange — each case is the relation, the refused code, and whether the
    // `mint` or the `transfer` table holds it.
    let cases: Vec<(ContextRelation, u16)> = vec![
        (ContextRelation::Mint, 2),
        (ContextRelation::Transfer, 1),
        (ContextRelation::Transfer, 4),
        (ContextRelation::Mint, 0),
    ];
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    for (relation, code) in cases {
        let mut payload = build_admit_statements_payload(Default::default());
        match relation {
            ContextRelation::Mint => {
                payload.statements.mint.purpose = code;
            }
            ContextRelation::Transfer => {
                payload.statements.transfer.purpose = code;
            }
        }
        // Act
        let result = admit_statements(&deps, params, payload);
        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitStatementsErrorReturn::Context {
                relation,
                field: ContextField::Purpose,
                refusal: ContextRefusal::Purpose(UndeclaredDeliveryPurposeCode { code })
            })
        );
    }
    // admit_statements_refuses_a_purpose_code_of_the_other_relation
}

/// Contract: Identities → under `IdentityScope::Asset` a length other than
///   32 bytes is refused with that length.
/// Arrange: the default payload with `scope` `IdentityScope::Asset`, `proof`
///   32 bytes, and `mapping` of 32 bytes then 31 bytes.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals `AdmitStatementsErrorReturn::Identity {
///   identity: IdentityField::Mapping { index: 1 }, refusal:
///   IdentityRefusal::Hex(HexRefusal::WrongWidth { expected: 32, actual: 31 })
///   }`.
#[test]
fn admit_statements_refuses_an_asset_scope_identity_of_the_wrong_width() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.scope = IdentityScope::Asset;
    payload.identities.proof = format!("0x{}", "71".repeat(32));
    payload.identities.mapping = vec![
        format!("0x{}", "72".repeat(32)),
        format!("0x{}", "73".repeat(31)),
    ];
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Identity {
            identity: IdentityField::Mapping { index: 1 },
            refusal: IdentityRefusal::Hex(HexRefusal::WrongWidth {
                expected: 32,
                actual: 31
            })
        })
    );
    // admit_statements_refuses_an_asset_scope_identity_of_the_wrong_width
}

/// Contract: Identities → an all-zero asset identity is
///   `IdentityRefusal::Root(RootTryNewErrorReturn::AllZero)`, carried
///   unchanged.
/// Arrange: the default payload with `scope` `IdentityScope::Asset`, a valid
///   `mapping`, and `proof` 32 zero bytes.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `Identity` refusal at
///   `IdentityField::Proof` with `Root(RootTryNewErrorReturn::AllZero)`.
#[test]
fn admit_statements_refuses_an_all_zero_asset_scope_identity_with_the_root_types_refusal() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.scope = IdentityScope::Asset;
    payload.identities.proof = format!("0x{}", "00".repeat(32));
    payload.identities.mapping = vec![format!("0x{}", "72".repeat(32))];
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Identity {
            identity: IdentityField::Proof,
            refusal: IdentityRefusal::Root(RootTryNewErrorReturn::AllZero)
        })
    );
    // admit_statements_refuses_an_all_zero_asset_scope_identity_with_the_root_types_refusal
}

/// Contract: Identities → under `IdentityScope::Entitlement` a text that is
///   not `0x` and an even count of digits is
///   `IdentityRefusal::Hex(HexRefusal::Malformed { text })`.
/// Arrange: the default payload with `proof` `"aa01"`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `Identity` refusal at
///   `IdentityField::Proof` with `Hex(HexRefusal::Malformed { text: "aa01"
///   .to_string() })`.
#[test]
fn admit_statements_refuses_a_malformed_entitlement_scope_identity() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.identities.proof = "aa01".to_string();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Identity {
            identity: IdentityField::Proof,
            refusal: IdentityRefusal::Hex(HexRefusal::Malformed {
                text: "aa01".to_string()
            })
        })
    );
    // admit_statements_refuses_a_malformed_entitlement_scope_identity
}

/// Contract: Ordering → the first refused text in the stated order returns
///   and later texts are unread.
/// Arrange: the default payload with an unknown deployment record kind under
///   a curve label, a zero `mint.suite_identifier`, and a malformed `proof`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `DeploymentRecord` refusal carrying
///   that label.
#[test]
fn admit_statements_returns_the_first_refusal_in_the_stated_order() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload
        .network_profile
        .deployment_records
        .get_mut("curve-a")
        .expect("the default holds a curve-a record")
        .kind = "unknown".to_string();
    payload.statements.mint.suite_identifier = format!("0x{}", "00".repeat(32));
    payload.identities.proof = "aa01".to_string();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::DeploymentRecord {
            label: "curve-a".to_string(),
            refusal: CanonicalValueRefusal::UnknownKind {
                text: "unknown".to_string()
            }
        })
    );
    // admit_statements_returns_the_first_refusal_in_the_stated_order
}

/// Contract: Ordering → a refused context returns before a refused identity.
/// Arrange: the default payload with the `transfer` table's `asset_root` of
///   32 zero bytes and a malformed `proof`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals the `Context` refusal at
///   `ContextRelation::Transfer`, `ContextField::AssetRoot`.
#[test]
fn admit_statements_admits_the_network_profile_before_the_statements_and_the_statements_before_the_identities()
 {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.statements.transfer.asset_root = format!("0x{}", "00".repeat(32));
    payload.identities.proof = "aa01".to_string();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::Context {
            relation: ContextRelation::Transfer,
            field: ContextField::AssetRoot,
            refusal: ContextRefusal::Root(RootTryNewErrorReturn::AllZero)
        })
    );
    // admit_statements_admits_the_network_profile_before_the_statements_and_the_statements_before_the_identities
}

/// Contract: Deployment records → the refusal names the label of the curve
///   whose record is refused, `AdmitStatementsErrorReturn::DeploymentRecord
///   { label, refusal }`.
/// Arrange: the default payload with two curve labels and the record under
///   the second label of kind `"unknown"`.
/// Act:     `admit_statements`, once.
/// Assert:  the result's error equals `DeploymentRecord { label, refusal:
///   CanonicalValueRefusal::UnknownKind { text } }` with the second label and
///   the text `"unknown"`.
#[test]
fn admit_statements_refuses_a_deployment_record_naming_its_curve_label() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload
        .network_profile
        .deployment_records
        .get_mut("curve-b")
        .expect("the default holds a curve-b record")
        .kind = "unknown".to_string();
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::DeploymentRecord {
            label: "curve-b".to_string(),
            refusal: CanonicalValueRefusal::UnknownKind {
                text: "unknown".to_string()
            }
        })
    );
    // admit_statements_refuses_a_deployment_record_naming_its_curve_label
}

/// Contract: Deployment records → the admitted records are one per curve
///   label, in curve order.
/// Arrange: the default payload with two curve labels and a distinct
///   `FixedBytes20` record under each.
/// Act:     `admit_statements`, once.
/// Assert:  `network_profile.deployment_records` equals the two
///   `CanonicalFieldValue::FixedBytes20` values in curve order.
#[test]
fn admit_statements_admits_each_curves_record_in_curve_order() {
    // Arrange — the labels in the reverse of the records' key order, so a
    //   key-order reading differs from the curve order the contract states.
    let mut payload = build_admit_statements_payload(Default::default());
    payload.curve_labels = vec!["curve-b".to_string(), "curve-a".to_string()];
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(
        admitted.network_profile.deployment_records,
        vec![
            CanonicalFieldValue::FixedBytes20([0x24; 20]),
            CanonicalFieldValue::FixedBytes20([0x22; 20]),
        ]
    );
    // admit_statements_admits_each_curves_record_in_curve_order
}

/// Contract: Deployment records → `MissingDeploymentRecord { label }` for a
///   curve label with no record and `UndeclaredDeploymentRecord { label }`
///   for a key naming no curve label.
/// Arrange: the default payload with a curve label whose record is absent,
///   then with a record key no curve label names.
/// Act:     `admit_statements`, once per payload.
/// Assert:  each result's error equals the refusal carrying that label.
#[test]
fn admit_statements_refuses_a_curve_without_a_record_and_a_record_without_a_curve() {
    // Arrange — a curve label whose record is absent.
    let mut missing = build_admit_statements_payload(Default::default());
    missing.curve_labels = vec!["curve-a".to_string(), "curve-c".to_string()];
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        missing,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::MissingDeploymentRecord {
            label: "curve-c".to_string()
        })
    );

    // Arrange — a record key no curve label names.
    let mut undeclared = build_admit_statements_payload(Default::default());
    undeclared.network_profile.deployment_records.insert(
        "curve-c".to_string(),
        undeclared.network_profile.entitlement_contract.clone(),
    );
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        undeclared,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::UndeclaredDeploymentRecord {
            label: "curve-c".to_string()
        })
    );
    // admit_statements_refuses_a_curve_without_a_record_and_a_record_without_a_curve
}
