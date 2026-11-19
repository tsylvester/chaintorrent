#![cfg(feature = "mocks")]

use chain::{
    ADMIT_CHAIN_SELECTION_DEPS, AdmitChainSelectionParamsOverrides,
    AdmitChainSelectionSuccessReturnOverrides, ChainMemberIdentifier, ChainSelection,
    MockIChainAdapterFailureMode, admit_chain_selection, build_admit_chain_selection_params,
    build_admit_chain_selection_success_return,
};

/// Contract: the absorbed `chain/base/settings` entry
///   `the_declared_settings_are_admitted_through_the_real_constructors`
///   as observed from outside: `admit_chain_selection` over
///   `ChainMemberIdentifier::Base` with `serde_json::json!({})` → `Ok`
///   whose selection is `ChainSelection::Base` holding the declared
///   defaults.
/// Arrange: the identifier `Base`; params built by
///   `build_admit_chain_selection_params` from the crate's public
///   surface; deps `&ADMIT_CHAIN_SELECTION_DEPS`; the payload
///   `serde_json::json!({})` an outside caller holds.
/// Act:     `admit_chain_selection`.
/// Assert:  the result is `Ok`, the selection is the `Base` variant, and
///   its settings read through their public fields equal the independent
///   literals `endpoint` text `https://mainnet.base.org`,
///   `settlement_endpoint` text `https://ethereum-rpc.publicnode.com`,
///   `chain_identifier` the 32 big-endian bytes of `8453`, `registry`
///   twenty bytes each `0xdd`, `entitlement_contract` twenty bytes each
///   `0x00`, `gas_price_oracle` the twenty bytes of
///   `0x420000000000000000000000000000000000000f`,
///   `anchor_state_registry` the twenty bytes of
///   `0x909f6cf47ed12f010a796527f562bfc26c7f4e72`, and `batch_page_limit`
///   `64`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard, the real declaration lookup, and the member's
///   settings admission.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_selects_the_declared_defaults_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert!(result.is_ok());
    let selection = result.ok().map(|success| success.selection);
    assert!(matches!(selection, Some(ChainSelection::Base { .. })));
    let settings = selection.and_then(|selection| match selection {
        ChainSelection::Base { settings } => Some(settings),
        _ => None,
    });
    let mut chain_identifier = [0u8; 32];
    chain_identifier[30] = 0x21;
    chain_identifier[31] = 0x05;
    assert_eq!(
        settings
            .as_ref()
            .map(|settings| settings.endpoint.text.as_str()),
        Some("https://mainnet.base.org")
    );
    assert_eq!(
        settings
            .as_ref()
            .map(|settings| settings.settlement_endpoint.text.as_str()),
        Some("https://ethereum-rpc.publicnode.com")
    );
    assert_eq!(
        settings.as_ref().map(|settings| settings.chain_identifier),
        Some(chain_identifier)
    );
    assert_eq!(
        settings.as_ref().map(|settings| settings.registry),
        Some([0xdd; 20])
    );
    assert_eq!(
        settings
            .as_ref()
            .map(|settings| settings.entitlement_contract),
        Some([0x00; 20])
    );
    assert_eq!(
        settings.as_ref().map(|settings| settings.gas_price_oracle),
        Some([
            0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x0f,
        ])
    );
    assert_eq!(
        settings
            .as_ref()
            .map(|settings| settings.anchor_state_registry),
        Some([
            0x90, 0x9f, 0x6c, 0xf4, 0x7e, 0xd1, 0x2f, 0x01, 0x0a, 0x79, 0x65, 0x27, 0xf5, 0x62,
            0xbf, 0xc2, 0x6c, 0x7f, 0x4e, 0x72,
        ])
    );
    assert_eq!(settings.map(|settings| settings.batch_page_limit), Some(64));
}
// admit_chain_selection_selects_the_declared_defaults_for_an_outside_caller

/// Contract: the own entry
///   `a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings`
///   as observed from outside: a document supplying `endpoint`,
///   `chain_identifier`, `entitlement_contract`, and `batch_page_limit`
///   → `Ok` whose selection holds them and the declared defaults of
///   every other setting.
/// Arrange: the identifier `Base`; params built by
///   `build_admit_chain_selection_params`; deps
///   `&ADMIT_CHAIN_SELECTION_DEPS`; the payload `serde_json::json!`
///   holding `endpoint` `http://localhost:8545`, `chain_identifier`
///   `84532`, `entitlement_contract` `0x` followed by forty `a` digits,
///   and `batch_page_limit` `128`, each differing from its declared
///   default.
/// Act:     `admit_chain_selection`.
/// Assert:  the result is `Ok`, the selection is the `Base` variant, and
///   its `endpoint` text equals `http://localhost:8545`, its
///   `chain_identifier` the 32 big-endian bytes of `84532`, its
///   `entitlement_contract` twenty bytes each `0xaa`, its
///   `batch_page_limit` `128`, and its `settlement_endpoint` text
///   `https://ethereum-rpc.publicnode.com`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard, the real declaration lookup, and the member's
///   settings admission.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_replaces_a_declared_default_with_the_supplied_value_for_an_outside_caller()
{
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({
        "endpoint": "http://localhost:8545",
        "chain_identifier": "84532",
        "entitlement_contract": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "batch_page_limit": "128",
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert!(result.is_ok());
    let selection = result.ok().map(|success| success.selection);
    assert!(matches!(selection, Some(ChainSelection::Base { .. })));
    let settings = selection.and_then(|selection| match selection {
        ChainSelection::Base { settings } => Some(settings),
        _ => None,
    });
    let mut chain_identifier = [0u8; 32];
    chain_identifier[29] = 0x01;
    chain_identifier[30] = 0x4a;
    chain_identifier[31] = 0x34;
    assert_eq!(
        settings
            .as_ref()
            .map(|settings| settings.endpoint.text.as_str()),
        Some("http://localhost:8545")
    );
    assert_eq!(
        settings
            .as_ref()
            .map(|settings| settings.settlement_endpoint.text.as_str()),
        Some("https://ethereum-rpc.publicnode.com")
    );
    assert_eq!(
        settings.as_ref().map(|settings| settings.chain_identifier),
        Some(chain_identifier)
    );
    assert_eq!(
        settings
            .as_ref()
            .map(|settings| settings.entitlement_contract),
        Some([0xaa; 20])
    );
    assert_eq!(
        settings.map(|settings| settings.batch_page_limit),
        Some(128)
    );
}
// admit_chain_selection_replaces_a_declared_default_with_the_supplied_value_for_an_outside_caller

/// Contract: the own entry
///   `a_value_for_the_fixed_oracle_is_refused_by_the_admission_in_the_base_variant`
///   as observed from outside: a `gas_price_oracle` the declaration
///   fixes → `Err(Base(FixedSetting { setting, supplied }))`.
/// Arrange: the identifier `Base`; deps `&ADMIT_CHAIN_SELECTION_DEPS`;
///   the payload `serde_json::json!` holding `gas_price_oracle` `0x`
///   followed by forty `1` digits.
/// Act:     `admit_chain_selection`.
/// Assert:  the `Debug` rendering of the result equals `Err(Base(
///   FixedSetting { setting: "gas_price_oracle", supplied:
///   "0x1111111111111111111111111111111111111111" }))`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard, the real declaration lookup, and the member's
///   settings admission.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_refuses_a_value_for_the_fixed_oracle_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({
        "gas_price_oracle": "0x1111111111111111111111111111111111111111",
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert_eq!(
        format!("{result:?}"),
        "Err(Base(FixedSetting { setting: \"gas_price_oracle\", supplied: \"0x1111111111111111111111111111111111111111\" }))"
    );
}
// admit_chain_selection_refuses_a_value_for_the_fixed_oracle_for_an_outside_caller

/// Contract: the own entry
///   `an_endpoint_outside_its_declared_bounds_is_refused_by_the_admission_in_the_base_variant`
///   as observed from outside: an `endpoint` beginning with none of the
///   declared prefixes → `Err(Base(OutOfBounds { setting, supplied }))`.
/// Arrange: the identifier `Base`; deps `&ADMIT_CHAIN_SELECTION_DEPS`;
///   the payload `serde_json::json!` holding `endpoint`
///   `ftp://example.test`.
/// Act:     `admit_chain_selection`.
/// Assert:  the `Debug` rendering of the result equals `Err(Base(
///   OutOfBounds { setting: "endpoint", supplied:
///   "ftp://example.test" }))`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard, the real declaration lookup, and the member's
///   settings admission.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_refuses_an_endpoint_outside_its_declared_bounds_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({
        "endpoint": "ftp://example.test",
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert_eq!(
        format!("{result:?}"),
        "Err(Base(OutOfBounds { setting: \"endpoint\", supplied: \"ftp://example.test\" }))"
    );
}
// admit_chain_selection_refuses_an_endpoint_outside_its_declared_bounds_for_an_outside_caller

/// Contract: the own entry
///   `the_guard_refuses_before_the_admission_reads_the_document` as
///   observed from outside: the guard's error, the admission not
///   reached.
/// Arrange: the identifier `Base`; deps `&ADMIT_CHAIN_SELECTION_DEPS`;
///   the payload `serde_json::json!` holding `endpoint`
///   `ftp://example.test`, which only the admission refuses, and
///   `chain_identifier` the number `7`, which only the guard refuses, so
///   an admission that ran first would have returned the `Base` variant.
/// Act:     `admit_chain_selection`.
/// Assert:  the `Debug` rendering of the result equals
///   `Err(BaseDocument(NotText { path: "chain_identifier" }))`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard, the real declaration lookup, and the member's
///   settings admission.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_refuses_before_the_admission_reads_the_document_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({
        "endpoint": "ftp://example.test",
        "chain_identifier": 7,
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert_eq!(
        format!("{result:?}"),
        "Err(BaseDocument(NotText { path: \"chain_identifier\" }))"
    );
}
// admit_chain_selection_refuses_before_the_admission_reads_the_document_for_an_outside_caller

/// Contract: the own entry
///   `a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant`
///   as observed from outside: a payload that is not an object →
///   `Err(BaseDocument(NotATable { path }))`.
/// Arrange: the identifier `Base`; deps `&ADMIT_CHAIN_SELECTION_DEPS`;
///   the payload `serde_json::json!(3)`.
/// Act:     `admit_chain_selection`.
/// Assert:  the `Debug` rendering of the result equals
///   `Err(BaseDocument(NotATable { path: "" }))`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_refuses_a_payload_that_is_not_a_table_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!(3);

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert_eq!(
        format!("{result:?}"),
        "Err(BaseDocument(NotATable { path: \"\" }))"
    );
}
// admit_chain_selection_refuses_a_payload_that_is_not_a_table_for_an_outside_caller

/// Contract: the own entry
///   `an_unknown_key_is_refused_by_the_guard_in_the_document_variant` as
///   observed from outside: an object holding a key outside the member's
///   declared keys → `Err(BaseDocument(UnknownKey { path, key }))`.
/// Arrange: the identifier `Base`; deps `&ADMIT_CHAIN_SELECTION_DEPS`;
///   the payload `serde_json::json!` holding the key `endpoints`, one
///   letter from the declared key `endpoint`.
/// Act:     `admit_chain_selection`.
/// Assert:  the `Debug` rendering of the result equals
///   `Err(BaseDocument(UnknownKey { path: "", key: "endpoints" }))`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_refuses_an_unknown_key_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({
        "endpoints": "https://mainnet.base.org",
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert_eq!(
        format!("{result:?}"),
        "Err(BaseDocument(UnknownKey { path: \"\", key: \"endpoints\" }))"
    );
}
// admit_chain_selection_refuses_an_unknown_key_for_an_outside_caller

/// Contract: the own entry
///   `a_value_that_is_not_text_is_refused_by_the_guard_in_the_document_variant`
///   as observed from outside: a present value of the wrong kind inside
///   an otherwise valid table → `Err(BaseDocument(NotText { path }))`.
/// Arrange: the identifier `Base`; deps `&ADMIT_CHAIN_SELECTION_DEPS`;
///   the payload `serde_json::json!` holding `batch_page_limit` the
///   number `64`.
/// Act:     `admit_chain_selection`.
/// Assert:  the `Debug` rendering of the result equals
///   `Err(BaseDocument(NotText { path: "batch_page_limit" }))`.
/// Boundary: the public `admit_chain_selection`; the real chain is the
///   member's guard.
/// Mocked:  nothing.
#[test]
fn admit_chain_selection_refuses_a_value_that_is_not_text_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = serde_json::json!({
        "batch_page_limit": 64,
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert_eq!(
        format!("{result:?}"),
        "Err(BaseDocument(NotText { path: \"batch_page_limit\" }))"
    );
}
// admit_chain_selection_refuses_a_value_that_is_not_text_for_an_outside_caller

/// Contract: the own entry
///   `the_mock_member_is_selected_under_no_failure_whatever_the_payload`
///   as observed from outside: the identifier
///   `ChainMemberIdentifier::Mock` → `Ok` whose selection is
///   `ChainSelection::Mock` under `MockIChainAdapterFailureMode::
///   NoFailure`.
/// Arrange: the identifier `Mock`; deps `&ADMIT_CHAIN_SELECTION_DEPS`;
///   the payload `serde_json::json!(3)`, a value the Base guard refuses,
///   so a branch that read the payload would refuse.
/// Act:     `admit_chain_selection`.
/// Assert:  the result equals `Ok` of the success return built over the
///   selection `ChainSelection::Mock { failure_mode:
///   MockIChainAdapterFailureMode::NoFailure }`.
/// Boundary: the public `admit_chain_selection`.
/// Mocked:  nothing, because the mock member is the family's own mock
///   concrete.
#[test]
fn admit_chain_selection_selects_the_mock_member_whatever_the_payload_for_an_outside_caller() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Mock),
    });
    let payload = serde_json::json!(3);

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

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
// admit_chain_selection_selects_the_mock_member_whatever_the_payload_for_an_outside_caller

// Public-surface enumeration: the absorbed `chain/base/settings` entry
//   `the_declared_settings_are_admitted_through_the_real_constructors`,
//   proven by `admit_chain_selection_selects_the_declared_defaults_for_an_outside_caller`,
//   and the own entries, each proven by the outside-caller block of the
//   same-named spec entry above:
//   `a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings`,
//   `a_value_for_the_fixed_oracle_is_refused_by_the_admission_in_the_base_variant`,
//   `an_endpoint_outside_its_declared_bounds_is_refused_by_the_admission_in_the_base_variant`,
//   `the_guard_refuses_before_the_admission_reads_the_document`,
//   `a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant`,
//   `an_unknown_key_is_refused_by_the_guard_in_the_document_variant`,
//   `a_value_that_is_not_text_is_refused_by_the_guard_in_the_document_variant`,
//   `the_mock_member_is_selected_under_no_failure_whatever_the_payload`.
