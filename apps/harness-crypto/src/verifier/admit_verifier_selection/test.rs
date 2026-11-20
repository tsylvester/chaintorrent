#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use serde_json;

use super::super::evm::provides::{
    EvmVerifierSettingsDocumentErrorReturn, EvmVerifierSettingsOverrides,
    build_evm_verifier_settings, mock_admit_evm_verifier_settings,
};
use super::super::factory::provides::{
    MockIVerifierAdapterFailureMode, VerifierConcrete, VerifierConcreteKind,
};
use super::super::lookup_verifier_declaration::provides::mock_lookup_verifier_declaration;
use super::admit_verifier_selection;
use super::interface::*;
use super::mock::*;

/// Contract: the identifier `VerifierConcreteKind::Mock` → `Ok` holding
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`.
/// Arrange: the identifier `Mock` and a payload `serde_json::json!(3)`, a
///   value the EVM guard refuses, so a branch that read the payload would
///   refuse.
/// Act:    `admit_verifier_selection`.
/// Assert: the result equals `Ok` of the success return built over the
///   selection `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::
///   NoFailure)`.
#[test]
fn the_mock_member_selects_the_mock_concrete_under_no_failure_without_reading_the_payload() {
    // Arrange
    let deps = build_admit_verifier_selection_deps(
        mock_lookup_verifier_declaration,
        mock_admit_evm_verifier_settings,
        Default::default(),
    );
    let params = build_admit_verifier_selection_params(AdmitVerifierSelectionParamsOverrides {
        identifier: Some(VerifierConcreteKind::Mock),
    });
    let payload = serde_json::json!(3);

    // Act
    let result = admit_verifier_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_verifier_selection_success_return(
            AdmitVerifierSelectionSuccessReturnOverrides {
                selection: Some(VerifierConcrete::Mock(
                    MockIVerifierAdapterFailureMode::NoFailure,
                )),
            },
        ))
    );
}
// the_mock_member_selects_the_mock_concrete_under_no_failure_without_reading_the_payload

/// Contract: the EVM narrowing branch → `Err(EvmDocument(NotATable { path
///   }))`, the guard's error unchanged.
/// Arrange: the identifier `Evm`, the lookup and admission handles as the
///   official mocks, and the payload `serde_json::json!(7)`, a value that is
///   not an object.
/// Act:    `admit_verifier_selection`.
/// Assert: the result equals `Err(AdmitVerifierSelectionErrorReturn::
///   EvmDocument(EvmVerifierSettingsDocumentErrorReturn::NotATable { path:
///   String::new() }))`.
#[test]
fn a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant() {
    // Arrange
    let deps = build_admit_verifier_selection_deps(
        mock_lookup_verifier_declaration,
        mock_admit_evm_verifier_settings,
        Default::default(),
    );
    let params = build_admit_verifier_selection_params(AdmitVerifierSelectionParamsOverrides {
        identifier: Some(VerifierConcreteKind::Evm),
    });
    let payload = serde_json::json!(7);

    // Act
    let result = admit_verifier_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitVerifierSelectionErrorReturn::EvmDocument(
            EvmVerifierSettingsDocumentErrorReturn::NotATable {
                path: String::new(),
            },
        ))
    );
}
// a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant

/// Contract: the EVM narrowing branch → `Err(EvmDocument(UnknownKey { path,
///   key }))` naming that key, the guard's error unchanged.
/// Arrange: the identifier `Evm`, the lookup and admission handles as the
///   official mocks, and the payload `serde_json::json!({ "network": "base"
///   })`, an object holding a key.
/// Act:    `admit_verifier_selection`.
/// Assert: the result equals `Err(AdmitVerifierSelectionErrorReturn::
///   EvmDocument(EvmVerifierSettingsDocumentErrorReturn::UnknownKey { path:
///   String::new(), key: "network".into() }))`.
#[test]
fn a_key_the_member_does_not_declare_is_refused_by_the_guard_in_the_document_variant() {
    // Arrange
    let deps = build_admit_verifier_selection_deps(
        mock_lookup_verifier_declaration,
        mock_admit_evm_verifier_settings,
        Default::default(),
    );
    let params = build_admit_verifier_selection_params(AdmitVerifierSelectionParamsOverrides {
        identifier: Some(VerifierConcreteKind::Evm),
    });
    let payload = serde_json::json!({ "network": "base" });

    // Act
    let result = admit_verifier_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitVerifierSelectionErrorReturn::EvmDocument(
            EvmVerifierSettingsDocumentErrorReturn::UnknownKey {
                path: String::new(),
                key: "network".into(),
            },
        ))
    );
}
// a_key_the_member_does_not_declare_is_refused_by_the_guard_in_the_document_variant

/// Contract: the EVM admission branch → `Ok` whose selection is
///   `VerifierConcrete::Evm` holding the admitted settings boxed.
/// Arrange: the identifier `Evm`, the lookup and admission handles as the
///   official mocks, and the payload `serde_json::json!({})`, which the
///   guard narrows.
/// Act:    `admit_verifier_selection`.
/// Assert: the result equals `Ok` of the success return built over the
///   selection `VerifierConcrete::Evm` boxing the settings the admission
///   handle returned, the transform under test being the member's variant
///   and the box around the handle's settings.
#[test]
fn the_admitted_settings_are_carried_in_the_evm_selection() {
    // Arrange
    let deps = build_admit_verifier_selection_deps(
        mock_lookup_verifier_declaration,
        mock_admit_evm_verifier_settings,
        Default::default(),
    );
    let params = build_admit_verifier_selection_params(AdmitVerifierSelectionParamsOverrides {
        identifier: Some(VerifierConcreteKind::Evm),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_verifier_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_verifier_selection_success_return(
            AdmitVerifierSelectionSuccessReturnOverrides {
                selection: Some(VerifierConcrete::Evm {
                    settings: Box::new(build_evm_verifier_settings(
                        EvmVerifierSettingsOverrides::default(),
                    )),
                }),
            },
        ))
    );
}
// the_admitted_settings_are_carried_in_the_evm_selection
