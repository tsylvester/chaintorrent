#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{ChainFormsIdentifier, ChainMemberIdentifier};
use encoding::EncodingIdentifier;
use pairing::VerifierGroupArithmetic;
use proof::DELIVERY_STATEMENT_VERSION_ONE;
use serde_json;

use super::super::super::factory::provides::VerifierConcreteKind;
use super::admit_evm_verifier_settings;
use super::interface::*;
use super::mock::*;

/// Contract: the declaration states the member's identity and versions.
/// Arrange: `EVM_VERIFIER_DECLARATION`.
/// Act:     read its fields.
/// Assert:  `identifier` equals `VerifierConcreteKind::Evm`, `name` equals
///   `evm`, `adapter_version` equals `1`, and `interface_version` equals `1`,
///   each stated as an independent literal.
#[test]
fn the_declaration_names_the_evm_member_and_its_versions() {
    // Act
    let declaration = EVM_VERIFIER_DECLARATION;

    // Assert
    assert_eq!(declaration.identifier, VerifierConcreteKind::Evm);
    assert_eq!(declaration.name, "evm");
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, 1);
}
// the_declaration_names_the_evm_member_and_its_versions

/// Contract: the declaration states the chain forms, chain member, encoding,
///   and contract suite the member requires.
/// Arrange: `EVM_VERIFIER_DECLARATION`.
/// Act:     read its fields.
/// Assert:  `chain_forms_identifier` equals `ChainFormsIdentifier::EvmV1`,
///   `chain_member` equals `ChainMemberIdentifier::Base`,
///   `encoding_identifier` equals `EncodingIdentifier::EthereumAbiV1`, and
///   `contract_suite` equals `evm`.
#[test]
fn the_declaration_states_what_it_requires_of_the_adapters_it_is_composed_with() {
    // Act
    let declaration = EVM_VERIFIER_DECLARATION;

    // Assert
    assert_eq!(
        declaration.chain_forms_identifier,
        ChainFormsIdentifier::EvmV1
    );
    assert_eq!(declaration.chain_member, ChainMemberIdentifier::Base);
    assert_eq!(
        declaration.encoding_identifier,
        EncodingIdentifier::EthereumAbiV1
    );
    assert_eq!(declaration.contract_suite, "evm");
}
// the_declaration_states_what_it_requires_of_the_adapters_it_is_composed_with

/// Contract: the declaration lists the verifier forms and statement versions
///   the member serves.
/// Arrange: `EVM_VERIFIER_DECLARATION`.
/// Act:     read its fields.
/// Assert:  `verifier_forms` equals `FirstGroupOnly` then `BothGroups` and
///   `statement_versions` equals `1`, each stated as an independent literal.
#[test]
fn the_declaration_serves_both_verifier_forms_and_the_delivery_statement_version() {
    // Act
    let declaration = EVM_VERIFIER_DECLARATION;

    // Assert
    assert_eq!(
        declaration.verifier_forms,
        [
            VerifierGroupArithmetic::FirstGroupOnly,
            VerifierGroupArithmetic::BothGroups
        ]
        .as_slice()
    );
    assert_eq!(
        declaration.statement_versions,
        [DELIVERY_STATEMENT_VERSION_ONE].as_slice()
    );
}
// the_declaration_serves_both_verifier_forms_and_the_delivery_statement_version

/// Contract: the declaration lists a statement for every field of the
///   concrete's settings type, which has none.
/// Arrange: `EVM_VERIFIER_DECLARATION`.
/// Act:     read `settings`.
/// Assert:  `settings` equals the empty list.
#[test]
fn the_declaration_states_no_setting_statement() {
    // Act
    let declaration = EVM_VERIFIER_DECLARATION;

    // Assert
    assert_eq!(declaration.settings, &[]);
}
// the_declaration_states_no_setting_statement

/// Contract: an object that holds no key → `Ok` of the document.
/// Arrange: `serde_json::json!({})`.
/// Act:     `EvmVerifierSettingsDocument::try_from`.
/// Assert:  the result equals `Ok(EvmVerifierSettingsDocument)`.
#[test]
fn the_document_guard_admits_an_object_without_keys() {
    // Arrange
    let value = serde_json::json!({});

    // Act
    let result = EvmVerifierSettingsDocument::try_from(value);

    // Assert
    assert_eq!(result, Ok(EvmVerifierSettingsDocument));
}
// the_document_guard_admits_an_object_without_keys

/// Contract: not an object → `Err(NotATable { path: "" })`.
/// Arrange: `serde_json::json!(7)`.
/// Act:     `try_from`.
/// Assert:  `error` equals `NotATable { path: String::new() }`.
#[test]
fn the_document_guard_refuses_a_value_that_is_not_a_table() {
    // Arrange
    let value = serde_json::json!(7);

    // Act
    let result = EvmVerifierSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(EvmVerifierSettingsDocumentErrorReturn::NotATable {
            path: String::new()
        })
    );
}
// the_document_guard_refuses_a_value_that_is_not_a_table

/// Contract: an array is not an object → `Err(NotATable { path: "" })`.
/// Arrange: `serde_json::json!([])`.
/// Act:     `try_from`.
/// Assert:  `error` equals `NotATable { path: String::new() }`.
#[test]
fn the_document_guard_refuses_an_array_as_not_a_table() {
    // Arrange
    let value = serde_json::json!([]);

    // Act
    let result = EvmVerifierSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(EvmVerifierSettingsDocumentErrorReturn::NotATable {
            path: String::new()
        })
    );
}
// the_document_guard_refuses_an_array_as_not_a_table

/// Contract: an object that holds a key → `Err(UnknownKey { path: "", key })`.
/// Arrange: an object holding the key `endpoint`.
/// Act:     `try_from`.
/// Assert:  `error` equals `UnknownKey { path: String::new(), key: "endpoint"
///   .into() }`.
#[test]
fn the_document_guard_refuses_a_key_the_concrete_does_not_declare() {
    // Arrange
    let value = serde_json::json!({ "endpoint": "wss://example.invalid" });

    // Act
    let result = EvmVerifierSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(EvmVerifierSettingsDocumentErrorReturn::UnknownKey {
            path: String::new(),
            key: "endpoint".into()
        })
    );
}
// the_document_guard_refuses_a_key_the_concrete_does_not_declare

/// Contract: of several keys the lowest in key order is reported.
/// Arrange: an object holding the keys `network` and `alpha`.
/// Act:     `try_from`.
/// Assert:  `error` equals `UnknownKey { path: String::new(), key: "alpha"
///   .into() }`.
#[test]
fn the_document_guard_reports_the_lowest_key() {
    // Arrange
    let value = serde_json::json!({ "network": 1, "alpha": 2 });

    // Act
    let result = EvmVerifierSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Err(EvmVerifierSettingsDocumentErrorReturn::UnknownKey {
            path: String::new(),
            key: "alpha".into()
        })
    );
}
// the_document_guard_reports_the_lowest_key

/// Contract: the narrowed document → `Ok` holding the concrete's settings.
/// Arrange: deps from `build_admit_evm_verifier_settings_deps`, params from
///   `build_admit_evm_verifier_settings_params`, and the payload from
///   `build_admit_evm_verifier_settings_payload`.
/// Act:     `admit_evm_verifier_settings`.
/// Assert:  the result equals `Ok(AdmitEvmVerifierSettingsSuccessReturn {
///   settings: EvmVerifierSettings })`, stated as an independent literal.
#[test]
fn admission_returns_the_concrete_settings_for_the_narrowed_document() {
    // Arrange
    let deps = build_admit_evm_verifier_settings_deps(Default::default());
    let params = build_admit_evm_verifier_settings_params(Default::default());
    let payload = build_admit_evm_verifier_settings_payload(Default::default());

    // Act
    let result = admit_evm_verifier_settings(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitEvmVerifierSettingsSuccessReturn {
            settings: EvmVerifierSettings
        })
    );
}
// admission_returns_the_concrete_settings_for_the_narrowed_document
