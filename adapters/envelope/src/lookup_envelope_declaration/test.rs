use super::lookup_envelope_declaration;
use super::mock::{
    LookupEnvelopeDeclarationParamsOverrides, build_lookup_envelope_declaration_deps,
    build_lookup_envelope_declaration_params, build_lookup_envelope_declaration_payload,
};
use crate::factory::provides::{
    EnvelopeAlgebra, KeyAgreementConcrete, KeyAgreementIdentifier,
    MockIKeyAgreementAdapterFailureMode,
};

/// Contract: given the real member `KeyAgreementConcrete::PairingElGamal`,
///   the lookup returns the declaration that member states.
/// Arrange: the deps and payload from their builders and the params from
///   `build_lookup_envelope_declaration_params` with `concrete` overridden
///   to `KeyAgreementConcrete::PairingElGamal`.
/// Act:     `lookup_envelope_declaration`.
/// Assert:  `concrete` equals `KeyAgreementConcrete::PairingElGamal`,
///   `identifier` equals `KeyAgreementIdentifier::PairingElGamalV1`,
///   `algebra` equals `EnvelopeAlgebra::PairingElGamal`, `possession_g1_tag`
///   equals the bytes of `ChainTorrent-v1-envelope-possession-g1`,
///   `possession_g2_tag` equals the bytes of
///   `ChainTorrent-v1-envelope-possession-g2`, `adapter_version` equals `1`,
///   and `interface_version` equals `1`, each stated as an independent
///   literal.
#[test]
fn lookup_envelope_declaration_returns_the_pairing_elgamal_declaration_for_the_real_member() {
    // Arrange
    let deps = build_lookup_envelope_declaration_deps(Default::default());
    let params =
        build_lookup_envelope_declaration_params(LookupEnvelopeDeclarationParamsOverrides {
            concrete: Some(KeyAgreementConcrete::PairingElGamal),
        });
    let payload = build_lookup_envelope_declaration_payload(Default::default());

    // Act
    let result = lookup_envelope_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(
        success.declaration.concrete,
        KeyAgreementConcrete::PairingElGamal
    );
    assert_eq!(
        success.declaration.identifier,
        KeyAgreementIdentifier::PairingElGamalV1
    );
    assert_eq!(success.declaration.algebra, EnvelopeAlgebra::PairingElGamal);
    assert_eq!(
        success.declaration.possession_g1_tag,
        b"ChainTorrent-v1-envelope-possession-g1".as_slice()
    );
    assert_eq!(
        success.declaration.possession_g2_tag,
        b"ChainTorrent-v1-envelope-possession-g2".as_slice()
    );
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}
// lookup_envelope_declaration_returns_the_pairing_elgamal_declaration_for_the_real_member

/// Contract: given the mock member under a failure mode, the lookup returns
///   the mock declaration whose `concrete` is the selected member and whose
///   other fields are the mock declaration's.
/// Arrange: for each variant of `MockIKeyAgreementAdapterFailureMode`, the
///   params from `build_lookup_envelope_declaration_params` with `concrete`
///   overridden to the `KeyAgreementConcrete::Mock` member carrying that
///   variant; the deps and payload from their builders.
/// Act:     `lookup_envelope_declaration` once per variant.
/// Assert:  `concrete` equals the `KeyAgreementConcrete::Mock` member
///   carrying the selected variant; `identifier` equals
///   `KeyAgreementIdentifier::PairingElGamalV1`, `algebra` equals
///   `EnvelopeAlgebra::PairingElGamal`, `adapter_version` equals `1`, and
///   `interface_version` equals `1`, `possession_g1_tag` equals the bytes of
///   `ChainTorrent-v1-envelope-possession-g1-mock`, and `possession_g2_tag`
///   equals the bytes of `ChainTorrent-v1-envelope-possession-g2-mock`, each
///   an independent literal.
#[test]
fn lookup_envelope_declaration_substitutes_the_selected_failure_mode_into_the_mock_declaration() {
    for failure_mode in [
        MockIKeyAgreementAdapterFailureMode::NoFailure,
        MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
        MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
    ] {
        // Arrange
        let deps = build_lookup_envelope_declaration_deps(Default::default());
        let params =
            build_lookup_envelope_declaration_params(LookupEnvelopeDeclarationParamsOverrides {
                concrete: Some(KeyAgreementConcrete::Mock(failure_mode)),
            });
        let payload = build_lookup_envelope_declaration_payload(Default::default());

        // Act
        let result = lookup_envelope_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert_eq!(
            success.declaration.concrete,
            KeyAgreementConcrete::Mock(failure_mode)
        );
        assert_eq!(
            success.declaration.identifier,
            KeyAgreementIdentifier::PairingElGamalV1
        );
        assert_eq!(success.declaration.algebra, EnvelopeAlgebra::PairingElGamal);
        assert_eq!(success.declaration.adapter_version, 1);
        assert_eq!(success.declaration.interface_version, 1);
        assert_eq!(
            success.declaration.possession_g1_tag,
            b"ChainTorrent-v1-envelope-possession-g1-mock".as_slice()
        );
        assert_eq!(
            success.declaration.possession_g2_tag,
            b"ChainTorrent-v1-envelope-possession-g2-mock".as_slice()
        );
    }
}
// lookup_envelope_declaration_substitutes_the_selected_failure_mode_into_the_mock_declaration
