#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_chain_selection;
use super::interface::*;
use super::mock::*;
use crate::base::provides::{
    AdmitBaseChainSettingsDeps, AdmitBaseChainSettingsErrorReturn, AdmitBaseChainSettingsParams,
    AdmitBaseChainSettingsPayload, AdmitBaseChainSettingsReturn,
    BaseChainSettingsDocumentErrorReturn, build_base_chain_settings,
    mock_admit_base_chain_settings,
};
use crate::factory::provides::{
    ChainMemberIdentifier, ChainSelection, MockIChainAdapterFailureMode,
};
use crate::lookup_chain_declaration::provides::mock_lookup_chain_declaration;

/// Contract: the identifier `ChainMemberIdentifier::Mock` → `Ok` holding
///   `ChainSelection::Mock { failure_mode: MockIChainAdapterFailureMode::
///   NoFailure }`.
/// Arrange: the identifier `Mock` and a payload `serde_json::json!(3)`, a
///   value the Base guard refuses, so a branch that read the payload
///   would refuse.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Ok` of the success return built over the
///   selection `ChainSelection::Mock { failure_mode: MockIChainAdapter
///   FailureMode::NoFailure }`.
#[test]
fn the_mock_member_selects_the_mock_concrete_under_no_failure_without_reading_the_payload() {
    // Arrange
    let deps = build_admit_chain_selection_deps(
        mock_lookup_chain_declaration,
        mock_admit_base_chain_settings,
        Default::default(),
    );
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Mock),
    });
    let payload = serde_json::json!(3);

    // Act
    let result = admit_chain_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_chain_selection_success_return(
            AdmitChainSelectionSuccessReturnOverrides {
                selection: Some(ChainSelection::Mock {
                    failure_mode: MockIChainAdapterFailureMode::NoFailure,
                }),
            },
        ))
    );
}
// the_mock_member_selects_the_mock_concrete_under_no_failure_without_reading_the_payload

/// Contract: the Base narrowing branch → `Err(BaseDocument(NotATable {
///   path }))`, the guard's error unchanged.
/// Arrange: the identifier `Base`, the default handles, and the payload
///   `serde_json::json!(3)`.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Err(AdmitChainSelectionErrorReturn::
///   BaseDocument(BaseChainSettingsDocumentErrorReturn::NotATable {
///   path: String::new() }))`.
#[test]
fn a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant() {
    // Arrange
    let deps = build_admit_chain_selection_deps(
        mock_lookup_chain_declaration,
        mock_admit_base_chain_settings,
        Default::default(),
    );
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!(3);

    // Act
    let result = admit_chain_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitChainSelectionErrorReturn::BaseDocument(
            BaseChainSettingsDocumentErrorReturn::NotATable {
                path: String::new()
            },
        ))
    );
}
// a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant

/// Contract: the Base narrowing branch → `Err(BaseDocument(UnknownKey {
///   path, key }))`, the guard's error unchanged.
/// Arrange: the identifier `Base`, the default handles, and the payload
///   `serde_json::json!({ "endpoints": "https://mainnet.base.org" })`, a
///   key one letter from the declared key `endpoint`.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Err(AdmitChainSelectionErrorReturn::
///   BaseDocument(BaseChainSettingsDocumentErrorReturn::UnknownKey {
///   path: String::new(), key: "endpoints".into() }))`.
#[test]
fn an_unknown_key_is_refused_by_the_guard_in_the_document_variant() {
    // Arrange
    let deps = build_admit_chain_selection_deps(
        mock_lookup_chain_declaration,
        mock_admit_base_chain_settings,
        Default::default(),
    );
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({ "endpoints": "https://mainnet.base.org" });

    // Act
    let result = admit_chain_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitChainSelectionErrorReturn::BaseDocument(
            BaseChainSettingsDocumentErrorReturn::UnknownKey {
                path: String::new(),
                key: "endpoints".into(),
            },
        ))
    );
}
// an_unknown_key_is_refused_by_the_guard_in_the_document_variant

/// Contract: the Base narrowing branch → `Err(BaseDocument(NotText {
///   path }))`, the guard's error unchanged.
/// Arrange: the identifier `Base`, the default handles, and the payload
///   `serde_json::json!({ "batch_page_limit": 64 })`, a present value of
///   the wrong kind.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Err(AdmitChainSelectionErrorReturn::
///   BaseDocument(BaseChainSettingsDocumentErrorReturn::NotText {
///   path: "batch_page_limit".into() }))`.
#[test]
fn a_value_that_is_not_text_is_refused_by_the_guard_in_the_document_variant() {
    // Arrange
    let deps = build_admit_chain_selection_deps(
        mock_lookup_chain_declaration,
        mock_admit_base_chain_settings,
        Default::default(),
    );
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({ "batch_page_limit": 64 });

    // Act
    let result = admit_chain_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitChainSelectionErrorReturn::BaseDocument(
            BaseChainSettingsDocumentErrorReturn::NotText {
                path: "batch_page_limit".into(),
            },
        ))
    );
}
// a_value_that_is_not_text_is_refused_by_the_guard_in_the_document_variant

/// Contract: the Ordering branch → the guard's error, the admission not
///   reached.
/// Arrange: the identifier `Base`, the admission handle
///   `refusing_admission` declared in the block, and the payload
///   `serde_json::json!({ "chain_identifier": 7 })`, which the guard
///   refuses, so an admission that ran first would have returned the
///   `Base` variant.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Err(AdmitChainSelectionErrorReturn::
///   BaseDocument(BaseChainSettingsDocumentErrorReturn::NotText {
///   path: "chain_identifier".into() }))`.
#[test]
fn the_guard_refuses_before_the_admission_reads_the_document() {
    // Arrange
    fn refusing_admission<'a>(
        _deps: &AdmitBaseChainSettingsDeps<'a>,
        _params: AdmitBaseChainSettingsParams,
        _payload: AdmitBaseChainSettingsPayload,
    ) -> AdmitBaseChainSettingsReturn {
        Err(AdmitBaseChainSettingsErrorReturn::FixedSetting {
            setting: "gas_price_oracle".into(),
            supplied: "0x1111111111111111111111111111111111111111".into(),
        })
    }
    let deps = build_admit_chain_selection_deps(
        mock_lookup_chain_declaration,
        refusing_admission,
        Default::default(),
    );
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({ "chain_identifier": 7 });

    // Act
    let result = admit_chain_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitChainSelectionErrorReturn::BaseDocument(
            BaseChainSettingsDocumentErrorReturn::NotText {
                path: "chain_identifier".into(),
            },
        ))
    );
}
// the_guard_refuses_before_the_admission_reads_the_document

/// Contract: the Base admission branch → `Err(Base(error))` holding the
///   admission's refusal unchanged.
/// Arrange: the identifier `Base`, the admission handle
///   `refusing_admission` declared in the block, and the payload
///   `serde_json::json!({})`, which the guard narrows.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Err(AdmitChainSelectionErrorReturn::
///   Base(AdmitBaseChainSettingsErrorReturn::FixedSetting { setting:
///   "gas_price_oracle".into(), supplied: "0x1111111111111111111111111
///   111111111111111".into() }))`, the literals stated independently in
///   the assertion.
#[test]
fn an_admission_refusal_is_returned_whole_in_the_base_variant() {
    // Arrange
    fn refusing_admission<'a>(
        _deps: &AdmitBaseChainSettingsDeps<'a>,
        _params: AdmitBaseChainSettingsParams,
        _payload: AdmitBaseChainSettingsPayload,
    ) -> AdmitBaseChainSettingsReturn {
        Err(AdmitBaseChainSettingsErrorReturn::FixedSetting {
            setting: "gas_price_oracle".into(),
            supplied: "0x1111111111111111111111111111111111111111".into(),
        })
    }
    let deps = build_admit_chain_selection_deps(
        mock_lookup_chain_declaration,
        refusing_admission,
        Default::default(),
    );
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_chain_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitChainSelectionErrorReturn::Base(
            AdmitBaseChainSettingsErrorReturn::FixedSetting {
                setting: "gas_price_oracle".into(),
                supplied: "0x1111111111111111111111111111111111111111".into(),
            },
        ))
    );
}
// an_admission_refusal_is_returned_whole_in_the_base_variant

/// Contract: the Base admission branch → `Ok` whose selection is
///   `ChainSelection::Base` holding the admitted settings.
/// Arrange: the identifier `Base`, the default handles, and the payload
///   `serde_json::json!({})`, which the guard narrows.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Ok` of the success return built over the
///   selection `ChainSelection::Base` holding
///   `build_base_chain_settings(Default::default())`, the subject's act
///   being the choice of the `Base` variant around the settings the
///   admission returned.
#[test]
fn the_admitted_settings_are_carried_in_the_base_selection() {
    // Arrange
    let deps = build_admit_chain_selection_deps(
        mock_lookup_chain_declaration,
        mock_admit_base_chain_settings,
        Default::default(),
    );
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_chain_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_chain_selection_success_return(
            AdmitChainSelectionSuccessReturnOverrides {
                selection: Some(ChainSelection::Base {
                    settings: build_base_chain_settings(Default::default()),
                }),
            },
        ))
    );
}
// the_admitted_settings_are_carried_in_the_base_selection
