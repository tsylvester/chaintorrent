#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_base_chain_settings;
use super::interface::*;
use super::mock::*;
use crate::factory::provides::{
    ChainDeclarationOverrides, ChainFormsIdentifier, ChainMemberIdentifier, FeeEntryDeclaration,
    MOCK_PREFIXED_CHAIN_IDENTIFIER_SETTING_STATEMENTS, build_chain_declaration,
};
use pairing::{PRECOMPILE_ENCODINGS, PrecompileEncoding};
use url::ParseError;

/// Contract: the declaration states the member's identity, versions,
///   forms identifier, and fee entries.
/// Arrange: `BASE_CHAIN_DECLARATION`.
/// Act:     read its fields.
/// Assert:  `identifier` equals `ChainMemberIdentifier::Base`, `name`
///   equals `"Base"`, `adapter_version` equals `1`, `interface_version`
///   equals `1`, `forms_identifier` equals `ChainFormsIdentifier::EvmV1`,
///   and `fee_entries` equals the names `execution_gas` and
///   `l1_data_fee` with the units `gas` and `wei`, each stated as an
///   independent literal.
#[test]
fn the_declaration_names_the_base_member_and_its_bound_forms() {
    // Arrange / Act
    let declaration = &BASE_CHAIN_DECLARATION;

    // Assert
    assert_eq!(declaration.identifier, ChainMemberIdentifier::Base);
    assert_eq!(declaration.name, "Base");
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, 1);
    assert_eq!(declaration.forms_identifier, ChainFormsIdentifier::EvmV1);
    assert_eq!(
        declaration.fee_entries,
        &[
            FeeEntryDeclaration {
                name: "execution_gas",
                unit: "gas",
            },
            FeeEntryDeclaration {
                name: "l1_data_fee",
                unit: "wei",
            },
        ]
    );
}
// the_declaration_names_the_base_member_and_its_bound_forms

/// Contract: the declaration holds one statement per setting the
///   admission reads, in reading order.
/// Arrange: `BASE_CHAIN_DECLARATION`.
/// Act:     read the names of its `settings`.
/// Assert:  the names equal `endpoint`, `settlement_endpoint`,
///   `chain_identifier`, `registry`, `entitlement_contract`,
///   `gas_price_oracle`, `anchor_state_registry`, and `batch_page_limit`
///   in that order.
#[test]
fn the_declaration_states_a_statement_for_every_setting_in_reading_order() {
    // Arrange / Act
    let names: Vec<&'static str> = BASE_CHAIN_DECLARATION
        .settings
        .iter()
        .map(|statement| statement.name)
        .collect();

    // Assert
    assert_eq!(
        names,
        [
            "endpoint",
            "settlement_endpoint",
            "chain_identifier",
            "registry",
            "entitlement_contract",
            "gas_price_oracle",
            "anchor_state_registry",
            "batch_page_limit",
        ]
    );
}
// the_declaration_states_a_statement_for_every_setting_in_reading_order

/// Contract: every encoding the declaration states is a member of the
///   pairing family's declared set.
/// Arrange: `BASE_CHAIN_DECLARATION` and `PRECOMPILE_ENCODINGS`.
/// Act:     test each declared encoding for membership.
/// Assert:  `Eip196Eip197` and `Eip2537` are each a member of
///   `PRECOMPILE_ENCODINGS`, and the declaration's encodings equal
///   `Eip196Eip197` then `Eip2537`.
#[test]
fn the_declared_precompile_encodings_are_members_of_the_pairing_declared_set() {
    // Arrange / Act / Assert
    assert!(PRECOMPILE_ENCODINGS.contains(&PrecompileEncoding::Eip196Eip197));
    assert!(PRECOMPILE_ENCODINGS.contains(&PrecompileEncoding::Eip2537));
    assert_eq!(
        BASE_CHAIN_DECLARATION.precompile_encodings,
        &[
            PrecompileEncoding::Eip196Eip197,
            PrecompileEncoding::Eip2537,
        ]
    );
}
// the_declared_precompile_encodings_are_members_of_the_pairing_declared_set

/// Contract: an object → `Ok` of the document with every field `None`.
/// Arrange: `serde_json::json!({})`.
/// Act:     `BaseChainSettingsDocument::try_from`.
/// Assert:  the result equals
///   `Ok(build_base_chain_settings_document(Default::default()))`.
#[test]
fn the_document_guard_admits_an_object_without_keys() {
    // Arrange
    let value = serde_json::json!({});

    // Act
    let result = BaseChainSettingsDocument::try_from(value);

    // Assert
    assert_eq!(
        result,
        Ok(build_base_chain_settings_document(Default::default()))
    );
}
// the_document_guard_admits_an_object_without_keys

/// Contract: every declared key with a text value → `Ok` holding each.
/// Arrange: `invalidate_base_chain_settings_document` with each key
///   set to a distinct text.
/// Act:     `BaseChainSettingsDocument::try_from` over it.
/// Assert:  each field of the document equals `Some` of the text set for
///   its key.
#[test]
fn the_document_guard_admits_every_key_it_declares() {
    // Arrange
    let value = invalidate_base_chain_settings_document(BaseChainSettingsDocumentCorruptions {
        endpoint: Some(serde_json::json!("endpoint-text")),
        settlement_endpoint: Some(serde_json::json!("settlement-endpoint-text")),
        chain_identifier: Some(serde_json::json!("chain-identifier-text")),
        registry: Some(serde_json::json!("registry-text")),
        entitlement_contract: Some(serde_json::json!("entitlement-contract-text")),
        gas_price_oracle: Some(serde_json::json!("gas-price-oracle-text")),
        anchor_state_registry: Some(serde_json::json!("anchor-state-registry-text")),
        batch_page_limit: Some(serde_json::json!("batch-page-limit-text")),
    });

    // Act
    let document = BaseChainSettingsDocument::try_from(value).expect("the object narrows");

    // Assert
    assert_eq!(
        document,
        build_base_chain_settings_document(BaseChainSettingsDocumentOverrides {
            endpoint: Some(Some("endpoint-text".to_string())),
            settlement_endpoint: Some(Some("settlement-endpoint-text".to_string())),
            chain_identifier: Some(Some("chain-identifier-text".to_string())),
            registry: Some(Some("registry-text".to_string())),
            entitlement_contract: Some(Some("entitlement-contract-text".to_string())),
            gas_price_oracle: Some(Some("gas-price-oracle-text".to_string())),
            anchor_state_registry: Some(Some("anchor-state-registry-text".to_string())),
            batch_page_limit: Some(Some("batch-page-limit-text".to_string())),
        })
    );
}
// the_document_guard_admits_every_key_it_declares

/// Contract: not an object → `Err(NotATable { path: "" })`.
/// Arrange: `serde_json::json!(7)`.
/// Act:     `BaseChainSettingsDocument::try_from`.
/// Assert:  `error` equals `NotATable { path: String::new() }`.
#[test]
fn the_document_guard_refuses_a_value_that_is_not_a_table() {
    // Arrange
    let value = serde_json::json!(7);

    // Act
    let error = BaseChainSettingsDocument::try_from(value).err();

    // Assert
    assert_eq!(
        error,
        Some(BaseChainSettingsDocumentErrorReturn::NotATable {
            path: String::new()
        })
    );
}
// the_document_guard_refuses_a_value_that_is_not_a_table

/// Contract: a key outside the declared keys →
///   `Err(UnknownKey { path: "", key })`.
/// Arrange: an object holding the key `network`.
/// Act:     `BaseChainSettingsDocument::try_from`.
/// Assert:  `error` equals
///   `UnknownKey { path: String::new(), key: "network".into() }`.
#[test]
fn the_document_guard_refuses_an_unknown_key() {
    // Arrange
    let value = serde_json::json!({ "network": "unused" });

    // Act
    let error = BaseChainSettingsDocument::try_from(value).err();

    // Assert
    assert_eq!(
        error,
        Some(BaseChainSettingsDocumentErrorReturn::UnknownKey {
            path: String::new(),
            key: "network".into(),
        })
    );
}
// the_document_guard_refuses_an_unknown_key

/// Contract: a present value that is not a string → `Err(NotText { path })`.
/// Arrange: `invalidate_base_chain_settings_document` with `endpoint`
///   `7`.
/// Act:     `BaseChainSettingsDocument::try_from`.
/// Assert:  `error` equals `NotText { path: "endpoint".into() }`.
#[test]
fn the_document_guard_refuses_a_value_that_is_not_text() {
    // Arrange
    let value = invalidate_base_chain_settings_document(BaseChainSettingsDocumentCorruptions {
        endpoint: Some(serde_json::json!(7)),
        ..Default::default()
    });

    // Act
    let error = BaseChainSettingsDocument::try_from(value).err();

    // Assert
    assert_eq!(
        error,
        Some(BaseChainSettingsDocumentErrorReturn::NotText {
            path: "endpoint".into()
        })
    );
}
// the_document_guard_refuses_a_value_that_is_not_text

/// Contract: of several keys outside the declared keys the lowest in key
///   order is reported.
/// Arrange: an object holding the keys `network` and `alpha`.
/// Act:     `BaseChainSettingsDocument::try_from`.
/// Assert:  `error` equals
///   `UnknownKey { path: String::new(), key: "alpha".into() }`.
#[test]
fn the_document_guard_reports_the_lowest_unknown_key() {
    // Arrange
    let value = serde_json::json!({ "network": "unused", "alpha": "unused" });

    // Act
    let error = BaseChainSettingsDocument::try_from(value).err();

    // Assert
    assert_eq!(
        error,
        Some(BaseChainSettingsDocumentErrorReturn::UnknownKey {
            path: String::new(),
            key: "alpha".into(),
        })
    );
}
// the_document_guard_reports_the_lowest_unknown_key

/// Contract: the unknown-key refusal is decided before any value is
///   read.
/// Arrange: `invalidate_base_chain_settings_document` with `endpoint`
///   `7` and the key `network` added.
/// Act:     `BaseChainSettingsDocument::try_from`.
/// Assert:  `error` equals
///   `UnknownKey { path: String::new(), key: "network".into() }`.
#[test]
fn the_document_guard_refuses_an_unknown_key_before_a_value_that_is_not_text() {
    // Arrange
    let mut value = invalidate_base_chain_settings_document(BaseChainSettingsDocumentCorruptions {
        endpoint: Some(serde_json::json!(7)),
        ..Default::default()
    });
    value
        .as_object_mut()
        .expect("the invalidator renders an object")
        .insert("network".to_string(), serde_json::json!("unused"));

    // Act
    let error = BaseChainSettingsDocument::try_from(value).err();

    // Assert
    assert_eq!(
        error,
        Some(BaseChainSettingsDocumentErrorReturn::UnknownKey {
            path: String::new(),
            key: "network".into(),
        })
    );
}
// the_document_guard_refuses_an_unknown_key_before_a_value_that_is_not_text

/// Contract: of several keys whose values are not text the lowest in key
///   order is reported.
/// Arrange: `invalidate_base_chain_settings_document` with
///   `entitlement_contract` `7` and `chain_identifier` `7`.
/// Act:     `BaseChainSettingsDocument::try_from`.
/// Assert:  `error` equals `NotText { path: "chain_identifier".into() }`.
#[test]
fn the_document_guard_reports_the_lowest_key_holding_a_value_that_is_not_text() {
    // Arrange
    let value = invalidate_base_chain_settings_document(BaseChainSettingsDocumentCorruptions {
        chain_identifier: Some(serde_json::json!(7)),
        entitlement_contract: Some(serde_json::json!(7)),
        ..Default::default()
    });

    // Act
    let error = BaseChainSettingsDocument::try_from(value).err();

    // Assert
    assert_eq!(
        error,
        Some(BaseChainSettingsDocumentErrorReturn::NotText {
            path: "chain_identifier".into()
        })
    );
}
// the_document_guard_reports_the_lowest_key_holding_a_value_that_is_not_text

/// Contract: no supplied value → each setting's declared default.
/// Arrange: the empty document and `BASE_CHAIN_DECLARATION`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  the endpoint text `https://mainnet.base.org`, the settlement
///   endpoint text `https://ethereum-rpc.publicnode.com`, the chain
///   identifier bytes holding `8453` big-endian, the batch page limit
///   `64`, the registry's twenty bytes each `0xdd`, the entitlement
///   contract's twenty bytes each `0x00`, the oracle's twenty bytes the
///   literal bytes of the fixed text
///   `0x420000000000000000000000000000000000000f`, and the anchor state
///   registry's twenty bytes the literal bytes of
///   `0x909f6cf47ed12f010a796527f562bfc26c7f4e72`, each stated as an
///   independent literal.
#[test]
fn admission_takes_every_declared_default_when_the_document_supplies_nothing() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(Default::default());

    // Act
    let success = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .expect("the declared defaults admit");
    let settings = success.settings;

    // Assert
    assert_eq!(settings.endpoint.text, "https://mainnet.base.org");
    assert_eq!(
        settings.settlement_endpoint.text,
        "https://ethereum-rpc.publicnode.com"
    );
    let mut chain_identifier = [0u8; 32];
    chain_identifier[30] = 0x21;
    chain_identifier[31] = 0x05;
    assert_eq!(settings.chain_identifier, chain_identifier);
    assert_eq!(settings.registry, [0xdd; 20]);
    assert_eq!(settings.entitlement_contract, [0x00; 20]);
    assert_eq!(
        settings.gas_price_oracle,
        [
            0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x0f,
        ]
    );
    assert_eq!(
        settings.anchor_state_registry,
        [
            0x90, 0x9f, 0x6c, 0xf4, 0x7e, 0xd1, 0x2f, 0x01, 0x0a, 0x79, 0x65, 0x27, 0xf5, 0x62,
            0xbf, 0xc2, 0x6c, 0x7f, 0x4e, 0x72,
        ]
    );
    assert_eq!(settings.batch_page_limit, 64);
}
// admission_takes_every_declared_default_when_the_document_supplies_nothing

/// Contract: a supplied text for a settable setting replaces its
///   default.
/// Arrange: a document supplying `endpoint` `http://localhost:8545`,
///   `chain_identifier` `84532`, `entitlement_contract` `0x` followed by
///   forty `a` digits, and `batch_page_limit` `128`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  the endpoint text equals `http://localhost:8545`, the chain
///   identifier bytes hold `84532` big-endian, the entitlement
///   contract's twenty bytes are each `0xaa`, the batch page limit
///   equals `128`, and every other field equals its default.
#[test]
fn admission_replaces_a_default_with_the_supplied_value() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                endpoint: Some(Some("http://localhost:8545".to_string())),
                chain_identifier: Some(Some("84532".to_string())),
                entitlement_contract: Some(Some(
                    "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                )),
                batch_page_limit: Some(Some("128".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let success = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .expect("the supplied values admit");
    let settings = success.settings;

    // Assert
    assert_eq!(settings.endpoint.text, "http://localhost:8545");
    let mut chain_identifier = [0u8; 32];
    chain_identifier[29] = 0x01;
    chain_identifier[30] = 0x4a;
    chain_identifier[31] = 0x34;
    assert_eq!(settings.chain_identifier, chain_identifier);
    assert_eq!(settings.entitlement_contract, [0xaa; 20]);
    assert_eq!(settings.batch_page_limit, 128);
    let defaults = build_base_chain_settings(Default::default());
    assert_eq!(settings.settlement_endpoint, defaults.settlement_endpoint);
    assert_eq!(settings.registry, defaults.registry);
    assert_eq!(settings.gas_price_oracle, defaults.gas_price_oracle);
    assert_eq!(
        settings.anchor_state_registry,
        defaults.anchor_state_registry
    );
}
// admission_replaces_a_default_with_the_supplied_value

/// Contract: a supplied value for the setting the declaration fixes →
///   `Err(FixedSetting { setting, supplied })`.
/// Arrange: a document supplying `gas_price_oracle` as `0x` followed by
///   forty `1` digits.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `FixedSetting` with the setting
///   `gas_price_oracle` and the supplied text equal to that literal.
#[test]
fn admission_refuses_a_value_for_the_fixed_oracle() {
    // Arrange
    let supplied = format!("0x{}", "1".repeat(40));
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                gas_price_oracle: Some(Some(supplied.clone())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::FixedSetting {
            setting: "gas_price_oracle".into(),
            supplied,
        })
    );
}
// admission_refuses_a_value_for_the_fixed_oracle

/// Contract: an endpoint text beginning with none of the declared
///   prefixes → `Err(OutOfBounds)`, decided before the text is parsed.
/// Arrange: a document supplying `endpoint` as `ftp://example.test`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `OutOfBounds` for `endpoint` with the
///   supplied text.
#[test]
fn admission_refuses_an_endpoint_outside_its_bounds() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                endpoint: Some(Some("ftp://example.test".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::OutOfBounds {
            setting: "endpoint".into(),
            supplied: "ftp://example.test".into(),
        })
    );
}
// admission_refuses_an_endpoint_outside_its_bounds

/// Contract: the settlement endpoint is bounded as the endpoint is.
/// Arrange: a document supplying `settlement_endpoint`
///   `ws://example.test`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `OutOfBounds` for `settlement_endpoint` with
///   the supplied text.
#[test]
fn admission_refuses_a_settlement_endpoint_outside_its_bounds() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                settlement_endpoint: Some(Some("ws://example.test".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::OutOfBounds {
            setting: "settlement_endpoint".into(),
            supplied: "ws://example.test".into(),
        })
    );
}
// admission_refuses_a_settlement_endpoint_outside_its_bounds

/// Contract: a settlement endpoint text inside the bounds that does not
///   parse → `Err(Endpoint { setting, error })`, the parser's error
///   unchanged.
/// Arrange: a document supplying `settlement_endpoint` `https://`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `Endpoint` for `settlement_endpoint` with
///   `ParseError::EmptyHost`.
#[test]
fn admission_refuses_an_unparsable_settlement_endpoint() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                settlement_endpoint: Some(Some("https://".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::Endpoint {
            setting: "settlement_endpoint".into(),
            error: ParseError::EmptyHost,
        })
    );
}
// admission_refuses_an_unparsable_settlement_endpoint

/// Contract: the `Unbounded` arm admits a text the parser admits
///   whatever its prefix.
/// Arrange: a declaration from `build_chain_declaration` with `settings`
///   `MOCK_PREFIXED_CHAIN_IDENTIFIER_SETTING_STATEMENTS`, which leaves
///   `endpoint` unbounded, and a document supplying `endpoint`
///   `ftp://example.test`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  the endpoint text equals `ftp://example.test`.
#[test]
fn admission_admits_an_endpoint_text_under_an_unbounded_statement() {
    // Arrange
    let declaration = build_chain_declaration(ChainDeclarationOverrides {
        settings: Some(MOCK_PREFIXED_CHAIN_IDENTIFIER_SETTING_STATEMENTS),
        ..Default::default()
    });
    let deps = build_admit_base_chain_settings_deps(&declaration, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                endpoint: Some(Some("ftp://example.test".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let success = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .expect("the unbounded statement admits the text");

    // Assert
    assert_eq!(success.settings.endpoint.text, "ftp://example.test");
}
// admission_admits_an_endpoint_text_under_an_unbounded_statement

/// Contract: bounds apply to every settable setting; a text beginning
///   with none of the prefixes → `Err(OutOfBounds)`.
/// Arrange: the same declaration, which bounds `chain_identifier` to the
///   prefix `8`, and a document supplying `chain_identifier` `1`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `OutOfBounds` for `chain_identifier` with the
///   supplied text `1`.
#[test]
fn admission_refuses_a_chain_identifier_outside_its_declared_bounds() {
    // Arrange
    let declaration = build_chain_declaration(ChainDeclarationOverrides {
        settings: Some(MOCK_PREFIXED_CHAIN_IDENTIFIER_SETTING_STATEMENTS),
        ..Default::default()
    });
    let deps = build_admit_base_chain_settings_deps(&declaration, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                chain_identifier: Some(Some("1".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::OutOfBounds {
            setting: "chain_identifier".into(),
            supplied: "1".into(),
        })
    );
}
// admission_refuses_a_chain_identifier_outside_its_declared_bounds

/// Contract: an endpoint text inside the bounds that does not parse →
///   `Err(Endpoint { setting, error })`, the parser's error unchanged.
/// Arrange: a document supplying `endpoint` `http://`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `Endpoint` for `endpoint` with
///   `ParseError::EmptyHost`.
#[test]
fn admission_refuses_an_unparsable_endpoint() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                endpoint: Some(Some("http://".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::Endpoint {
            setting: "endpoint".into(),
            error: ParseError::EmptyHost,
        })
    );
}
// admission_refuses_an_unparsable_endpoint

/// Contract: a chain identifier text holding a non-digit →
///   `Err(ChainIdentifier { setting, text })`.
/// Arrange: a document supplying `chain_identifier` `84x32`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `ChainIdentifier` carrying the supplied text.
#[test]
fn admission_refuses_a_chain_identifier_that_is_not_decimal() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                chain_identifier: Some(Some("84x32".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::ChainIdentifier {
            setting: "chain_identifier".into(),
            text: "84x32".into(),
        })
    );
}
// admission_refuses_a_chain_identifier_that_is_not_decimal

/// Contract: an empty chain identifier text → `Err(ChainIdentifier)`.
/// Arrange: a document supplying `chain_identifier` empty.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `ChainIdentifier` carrying the empty text.
#[test]
fn admission_refuses_an_empty_chain_identifier() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                chain_identifier: Some(Some(String::new())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::ChainIdentifier {
            setting: "chain_identifier".into(),
            text: String::new(),
        })
    );
}
// admission_refuses_an_empty_chain_identifier

/// Contract: a decimal text whose value is two to the 256th power →
///   `Err(ChainIdentifier)`.
/// Arrange: a document supplying `chain_identifier`
///   `115792089237316195423570985008687907853269984665640564039457584007913129639936`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `ChainIdentifier` carrying that text.
#[test]
fn admission_refuses_a_chain_identifier_wider_than_256_bits() {
    // Arrange
    let text = "115792089237316195423570985008687907853269984665640564039457584007913129639936"
        .to_string();
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                chain_identifier: Some(Some(text.clone())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::ChainIdentifier {
            setting: "chain_identifier".into(),
            text,
        })
    );
}
// admission_refuses_a_chain_identifier_wider_than_256_bits

/// Contract: a decimal text whose value is two to the 256th power less
///   one is admitted as 32 bytes of `0xff`.
/// Arrange: a document supplying `chain_identifier`
///   `115792089237316195423570985008687907853269984665640564039457584007913129639935`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `chain_identifier` equals 32 bytes each `0xff`.
#[test]
fn admission_admits_the_largest_256_bit_chain_identifier() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(
        AdmitBaseChainSettingsPayloadOverrides {
            document: Some(build_base_chain_settings_document(
                BaseChainSettingsDocumentOverrides {
                    chain_identifier: Some(Some(
                        "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                            .to_string(),
                    )),
                    ..Default::default()
                },
            )),
        },
    );

    // Act
    let success = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .expect("the largest 256-bit value admits");

    // Assert
    assert_eq!(success.settings.chain_identifier, [0xff; 32]);
}
// admission_admits_the_largest_256_bit_chain_identifier

/// Contract: an address text of other than forty hexadecimal digits →
///   `Err(Address { setting, text })`.
/// Arrange: a document supplying `entitlement_contract` `0x1234`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `Address` for `entitlement_contract` carrying
///   the supplied text.
#[test]
fn admission_refuses_an_address_of_the_wrong_length() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                entitlement_contract: Some(Some("0x1234".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::Address {
            setting: "entitlement_contract".into(),
            text: "0x1234".into(),
        })
    );
}
// admission_refuses_an_address_of_the_wrong_length

/// Contract: a forty-character body holding a non-hexadecimal digit →
///   `Err(Address)`.
/// Arrange: a document supplying `entitlement_contract` `0x` followed by
///   thirty-nine `a` digits and one `g`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `Address` for `entitlement_contract` carrying
///   the supplied text.
#[test]
fn admission_refuses_an_address_with_a_non_hexadecimal_digit() {
    // Arrange
    let text = format!("0x{}g", "a".repeat(39));
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                entitlement_contract: Some(Some(text.clone())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::Address {
            setting: "entitlement_contract".into(),
            text,
        })
    );
}
// admission_refuses_an_address_with_a_non_hexadecimal_digit

/// Contract: hexadecimal digits of either case are admitted.
/// Arrange: a document supplying `entitlement_contract` `0x` followed by
///   forty `A` digits.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `entitlement_contract` equals twenty bytes each `0xaa`.
#[test]
fn admission_admits_an_address_in_upper_case_digits() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                entitlement_contract: Some(Some(
                    "0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string(),
                )),
                ..Default::default()
            },
        )),
    });

    // Act
    let success = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .expect("upper-case hexadecimal digits admit");

    // Assert
    assert_eq!(success.settings.entitlement_contract, [0xaa; 20]);
}
// admission_admits_an_address_in_upper_case_digits

/// Contract: forty hexadecimal digits without `0x` → `Err(Address)`.
/// Arrange: a document supplying `entitlement_contract` forty `a`
///   digits.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `Address` for `entitlement_contract` carrying
///   the supplied text.
#[test]
fn admission_refuses_an_address_without_the_hexadecimal_prefix() {
    // Arrange
    let text = "a".repeat(40);
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                entitlement_contract: Some(Some(text.clone())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::Address {
            setting: "entitlement_contract".into(),
            text,
        })
    );
}
// admission_refuses_an_address_without_the_hexadecimal_prefix

/// Contract: the registry is admitted as an address is.
/// Arrange: a document supplying `registry` `0x1234`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `Address` for `registry` carrying the
///   supplied text.
#[test]
fn admission_refuses_a_registry_of_the_wrong_length() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                registry: Some(Some("0x1234".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::Address {
            setting: "registry".into(),
            text: "0x1234".into(),
        })
    );
}
// admission_refuses_a_registry_of_the_wrong_length

/// Contract: the anchor state registry is admitted as an address is.
/// Arrange: a document supplying `anchor_state_registry` `0x1234`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `Address` for `anchor_state_registry`
///   carrying the supplied text.
#[test]
fn admission_refuses_an_anchor_state_registry_of_the_wrong_length() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                anchor_state_registry: Some(Some("0x1234".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::Address {
            setting: "anchor_state_registry".into(),
            text: "0x1234".into(),
        })
    );
}
// admission_refuses_an_anchor_state_registry_of_the_wrong_length

/// Contract: a page limit of zero → `Err(BatchPageLimit { setting, text })`.
/// Arrange: a document supplying `batch_page_limit` `0`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `BatchPageLimit` carrying the supplied text.
#[test]
fn admission_refuses_a_batch_page_limit_of_zero() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                batch_page_limit: Some(Some("0".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::BatchPageLimit {
            setting: "batch_page_limit".into(),
            text: "0".into(),
        })
    );
}
// admission_refuses_a_batch_page_limit_of_zero

/// Contract: an empty page limit text → `Err(BatchPageLimit)`.
/// Arrange: a document supplying `batch_page_limit` empty.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `BatchPageLimit` carrying the empty text.
#[test]
fn admission_refuses_an_empty_batch_page_limit() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                batch_page_limit: Some(Some(String::new())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::BatchPageLimit {
            setting: "batch_page_limit".into(),
            text: String::new(),
        })
    );
}
// admission_refuses_an_empty_batch_page_limit

/// Contract: a page limit text holding a non-digit →
///   `Err(BatchPageLimit)`.
/// Arrange: a document supplying `batch_page_limit` `sixty`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `BatchPageLimit` carrying the supplied text.
#[test]
fn admission_refuses_a_batch_page_limit_that_is_not_decimal() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                batch_page_limit: Some(Some("sixty".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::BatchPageLimit {
            setting: "batch_page_limit".into(),
            text: "sixty".into(),
        })
    );
}
// admission_refuses_a_batch_page_limit_that_is_not_decimal

/// Contract: a decimal value above the largest 64-bit unsigned value →
///   `Err(BatchPageLimit)`.
/// Arrange: a document supplying `batch_page_limit`
///   `18446744073709551616`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `BatchPageLimit` carrying the supplied text.
#[test]
fn admission_refuses_a_batch_page_limit_wider_than_64_bits() {
    // Arrange
    let text = "18446744073709551616".to_string();
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                batch_page_limit: Some(Some(text.clone())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::BatchPageLimit {
            setting: "batch_page_limit".into(),
            text,
        })
    );
}
// admission_refuses_a_batch_page_limit_wider_than_64_bits

/// Contract: no statement of the setting's name →
///   `Err(UndeclaredSetting)`.
/// Arrange: a declaration from `build_chain_declaration` with `settings`
///   at `&[]` and the empty document.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` equals `UndeclaredSetting` for `endpoint`, the
///   first setting read.
#[test]
fn admission_refuses_a_setting_the_declaration_does_not_state() {
    // Arrange
    let declaration = build_chain_declaration(ChainDeclarationOverrides {
        settings: Some(&[]),
        ..Default::default()
    });
    let deps = build_admit_base_chain_settings_deps(&declaration, Default::default());
    let payload = build_admit_base_chain_settings_payload(Default::default());

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::UndeclaredSetting {
            setting: "endpoint".into(),
        })
    );
}
// admission_refuses_a_setting_the_declaration_does_not_state

/// Contract: the settings are read in the order the spec states and the
///   first refusal returns.
/// Arrange: a document supplying `endpoint` `ftp://example.test` and
///   `chain_identifier` `84x32`.
/// Act:     `admit_base_chain_settings`.
/// Assert:  `error` is the endpoint's `OutOfBounds`, not the chain
///   identifier's refusal.
#[test]
fn admission_returns_the_first_refusal_in_reading_order() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(
            BaseChainSettingsDocumentOverrides {
                endpoint: Some(Some("ftp://example.test".to_string())),
                chain_identifier: Some(Some("84x32".to_string())),
                ..Default::default()
            },
        )),
    });

    // Act
    let error = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .err();

    // Assert
    assert_eq!(
        error,
        Some(AdmitBaseChainSettingsErrorReturn::OutOfBounds {
            setting: "endpoint".into(),
            supplied: "ftp://example.test".into(),
        })
    );
}
// admission_returns_the_first_refusal_in_reading_order
