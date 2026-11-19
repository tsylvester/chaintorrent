#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_base_chain_settings;
use super::interface::*;
use super::mock::*;

/// Contract: the call over `BASE_CHAIN_DECLARATION` with the empty
///   document returns `Ok` holding each default admitted by its real
///   constructor.
/// Arrange: `build_admit_base_chain_settings_deps` over
///   `&BASE_CHAIN_DECLARATION`; the payload's document from
///   `build_base_chain_settings_document(Default::default())`, which
///   supplies nothing.
/// Act:     `admit_base_chain_settings`.
/// Assert:  the endpoint text `https://mainnet.base.org`, the settlement
///   endpoint text `https://ethereum-rpc.publicnode.com`, the chain
///   identifier bytes holding `8453` big-endian, the batch page limit
///   `64`, the registry's twenty bytes each `0xdd`, the entitlement
///   contract's twenty bytes each `0x00`, the oracle's twenty bytes the
///   literal bytes of `0x420000000000000000000000000000000000000f`, and
///   the anchor state registry's twenty bytes the literal bytes of
///   `0x909f6cf47ed12f010a796527f562bfc26c7f4e72`, each stated as an
///   independent literal and each differing from every other setting's.
/// Boundary: `admit_base_chain_settings` and the real `Url::parse`.
/// Mocked:   nothing, because the chain has no outer-edge collaborator.
#[test]
fn the_declared_settings_are_admitted_through_the_real_constructors() {
    // Arrange
    let deps = build_admit_base_chain_settings_deps(&BASE_CHAIN_DECLARATION, Default::default());
    let payload = build_admit_base_chain_settings_payload(AdmitBaseChainSettingsPayloadOverrides {
        document: Some(build_base_chain_settings_document(Default::default())),
    });

    // Act
    let success = admit_base_chain_settings(
        &deps,
        build_admit_base_chain_settings_params(Default::default()),
        payload,
    )
    .expect("the declared defaults admit through the real constructors");
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
// the_declared_settings_are_admitted_through_the_real_constructors
