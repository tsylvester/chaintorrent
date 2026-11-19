#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_chain_selection;
use super::interface::*;
use super::mock::*;
use crate::base::provides::{
    AdmitBaseChainSettingsErrorReturn, BaseChainSettingsDocumentCorruptions,
    BaseChainSettingsOverrides, build_base_chain_settings, invalidate_base_chain_settings_document,
};
use crate::factory::provides::{
    ChainEndpointOverrides, ChainMemberIdentifier, ChainSelection, build_chain_endpoint,
};

/// Contract: the call over `ChainMemberIdentifier::Base` with a document
///   that supplies nothing returns `Ok` whose selection is
///   `ChainSelection::Base` holding the declared defaults.
/// Arrange: `invalidate_base_chain_settings_document` with no corruption.
/// Act:    `admit_chain_selection` over `Base`, the deps
///   `&ADMIT_CHAIN_SELECTION_DEPS`.
/// Assert: the selection equals `ChainSelection::Base` holding
///   `build_base_chain_settings` with every field overridden by an
///   independent literal: `endpoint` from `build_chain_endpoint` over the
///   text `https://mainnet.base.org`, `settlement_endpoint` over
///   `https://ethereum-rpc.publicnode.com`, `chain_identifier` the 32
///   big-endian bytes of `8453`, `registry` twenty bytes each `0xdd`,
///   `entitlement_contract` twenty bytes each `0x00`, `gas_price_oracle`
///   the twenty bytes of `0x420000000000000000000000000000000000000f`,
///   `anchor_state_registry` the twenty bytes of
///   `0x909f6cf47ed12f010a796527f562bfc26c7f4e72`, and `batch_page_limit`
///   `64`.
/// Boundary: `admit_chain_selection` over `ADMIT_CHAIN_SELECTION_DEPS`,
///   the document guard, the real `lookup_chain_declaration`, and the
///   real `admit_base_chain_settings`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_document_supplying_nothing_selects_the_declared_defaults_admitted_by_the_real_constructors() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = invalidate_base_chain_settings_document(Default::default());

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    let mut chain_identifier = [0u8; 32];
    chain_identifier[30] = 0x21;
    chain_identifier[31] = 0x05;
    assert_eq!(
        result,
        Ok(build_admit_chain_selection_success_return(
            AdmitChainSelectionSuccessReturnOverrides {
                selection: Some(ChainSelection::Base {
                    settings: build_base_chain_settings(BaseChainSettingsOverrides {
                        endpoint: Some(build_chain_endpoint(ChainEndpointOverrides {
                            text: Some("https://mainnet.base.org".into()),
                        })),
                        settlement_endpoint: Some(build_chain_endpoint(ChainEndpointOverrides {
                            text: Some("https://ethereum-rpc.publicnode.com".into()),
                        })),
                        chain_identifier: Some(chain_identifier),
                        registry: Some([0xdd; 20]),
                        entitlement_contract: Some([0x00; 20]),
                        gas_price_oracle: Some([
                            0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f,
                        ]),
                        anchor_state_registry: Some([
                            0x90, 0x9f, 0x6c, 0xf4, 0x7e, 0xd1, 0x2f, 0x01, 0x0a, 0x79, 0x65, 0x27,
                            0xf5, 0x62, 0xbf, 0xc2, 0x6c, 0x7f, 0x4e, 0x72,
                        ]),
                        batch_page_limit: Some(64),
                    }),
                }),
            },
        ))
    );
}
// a_document_supplying_nothing_selects_the_declared_defaults_admitted_by_the_real_constructors

/// Contract: a document supplying `endpoint`, `chain_identifier`,
///   `entitlement_contract`, and `batch_page_limit` → `Ok` whose
///   selection holds them and the declared defaults of every other
///   setting.
/// Arrange: `invalidate_base_chain_settings_document` with `endpoint`
///   `http://localhost:8545`, `chain_identifier` `84532`,
///   `entitlement_contract` `0x` followed by forty `a` digits, and
///   `batch_page_limit` `128`, each differing from its declared default.
/// Act:    `admit_chain_selection` over `Base`, the deps
///   `&ADMIT_CHAIN_SELECTION_DEPS`.
/// Assert: the selection equals `ChainSelection::Base` holding
///   `build_base_chain_settings` with every field overridden by an
///   independent literal: `endpoint` from `build_chain_endpoint` over the
///   text `http://localhost:8545`, `settlement_endpoint` over
///   `https://ethereum-rpc.publicnode.com`, `chain_identifier` the 32
///   big-endian bytes of `84532`, `registry` twenty bytes each `0xdd`,
///   `entitlement_contract` twenty bytes each `0xaa`, `gas_price_oracle`
///   the twenty bytes of `0x420000000000000000000000000000000000000f`,
///   `anchor_state_registry` the twenty bytes of
///   `0x909f6cf47ed12f010a796527f562bfc26c7f4e72`, and `batch_page_limit`
///   `128`.
/// Boundary: `admit_chain_selection` over `ADMIT_CHAIN_SELECTION_DEPS`,
///   the document guard, the real `lookup_chain_declaration`, and the
///   real `admit_base_chain_settings`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = invalidate_base_chain_settings_document(BaseChainSettingsDocumentCorruptions {
        endpoint: Some(serde_json::json!("http://localhost:8545")),
        chain_identifier: Some(serde_json::json!("84532")),
        entitlement_contract: Some(serde_json::json!(
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        )),
        batch_page_limit: Some(serde_json::json!("128")),
        ..Default::default()
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    let mut chain_identifier = [0u8; 32];
    chain_identifier[29] = 0x01;
    chain_identifier[30] = 0x4a;
    chain_identifier[31] = 0x34;
    assert_eq!(
        result,
        Ok(build_admit_chain_selection_success_return(
            AdmitChainSelectionSuccessReturnOverrides {
                selection: Some(ChainSelection::Base {
                    settings: build_base_chain_settings(BaseChainSettingsOverrides {
                        endpoint: Some(build_chain_endpoint(ChainEndpointOverrides {
                            text: Some("http://localhost:8545".into()),
                        })),
                        settlement_endpoint: Some(build_chain_endpoint(ChainEndpointOverrides {
                            text: Some("https://ethereum-rpc.publicnode.com".into()),
                        })),
                        chain_identifier: Some(chain_identifier),
                        registry: Some([0xdd; 20]),
                        entitlement_contract: Some([0xaa; 20]),
                        gas_price_oracle: Some([
                            0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f,
                        ]),
                        anchor_state_registry: Some([
                            0x90, 0x9f, 0x6c, 0xf4, 0x7e, 0xd1, 0x2f, 0x01, 0x0a, 0x79, 0x65, 0x27,
                            0xf5, 0x62, 0xbf, 0xc2, 0x6c, 0x7f, 0x4e, 0x72,
                        ]),
                        batch_page_limit: Some(128),
                    }),
                }),
            },
        ))
    );
}
// a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings

/// Contract: a `gas_price_oracle` the declaration fixes →
///   `Err(Base(FixedSetting { setting, supplied }))`.
/// Arrange: `invalidate_base_chain_settings_document` with
///   `gas_price_oracle` `0x` followed by forty `1` digits.
/// Act:    `admit_chain_selection` over `Base`, the deps
///   `&ADMIT_CHAIN_SELECTION_DEPS`.
/// Assert: `error` equals `AdmitChainSelectionErrorReturn::Base(
///   AdmitBaseChainSettingsErrorReturn::FixedSetting { setting:
///   "gas_price_oracle".into(), supplied:
///   "0x1111111111111111111111111111111111111111".into() })`.
/// Boundary: `admit_chain_selection` over `ADMIT_CHAIN_SELECTION_DEPS`,
///   the document guard, the real `lookup_chain_declaration`, and the
///   real `admit_base_chain_settings`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_value_for_the_fixed_oracle_is_refused_by_the_admission_in_the_base_variant() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = invalidate_base_chain_settings_document(BaseChainSettingsDocumentCorruptions {
        gas_price_oracle: Some(serde_json::json!(
            "0x1111111111111111111111111111111111111111"
        )),
        ..Default::default()
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

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
// a_value_for_the_fixed_oracle_is_refused_by_the_admission_in_the_base_variant

/// Contract: an `endpoint` beginning with none of the declared prefixes →
///   `Err(Base(OutOfBounds { setting, supplied }))`.
/// Arrange: `invalidate_base_chain_settings_document` with `endpoint`
///   `ftp://example.test`.
/// Act:    `admit_chain_selection` over `Base`, the deps
///   `&ADMIT_CHAIN_SELECTION_DEPS`.
/// Assert: `error` equals `AdmitChainSelectionErrorReturn::Base(
///   AdmitBaseChainSettingsErrorReturn::OutOfBounds { setting:
///   "endpoint".into(), supplied: "ftp://example.test".into() })`.
/// Boundary: `admit_chain_selection` over `ADMIT_CHAIN_SELECTION_DEPS`,
///   the document guard, the real `lookup_chain_declaration`, and the
///   real `admit_base_chain_settings`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn an_endpoint_outside_its_declared_bounds_is_refused_by_the_admission_in_the_base_variant() {
    // Arrange
    let params = build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Base),
    });
    let payload = invalidate_base_chain_settings_document(BaseChainSettingsDocumentCorruptions {
        endpoint: Some(serde_json::json!("ftp://example.test")),
        ..Default::default()
    });

    // Act
    let result = admit_chain_selection(&ADMIT_CHAIN_SELECTION_DEPS, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitChainSelectionErrorReturn::Base(
            AdmitBaseChainSettingsErrorReturn::OutOfBounds {
                setting: "endpoint".into(),
                supplied: "ftp://example.test".into(),
            },
        ))
    );
}
// an_endpoint_outside_its_declared_bounds_is_refused_by_the_admission_in_the_base_variant

// Private-surface enumeration: the absorbed `chain/base/settings` entry
//   `the_declared_settings_are_admitted_through_the_real_constructors`,
//   proven by
//   `a_document_supplying_nothing_selects_the_declared_defaults_admitted_by_the_real_constructors`,
//   and the own entries
//   `a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings`,
//   `a_value_for_the_fixed_oracle_is_refused_by_the_admission_in_the_base_variant`,
//   and
//   `an_endpoint_outside_its_declared_bounds_is_refused_by_the_admission_in_the_base_variant`,
//   each proven by the block of the same name above.
