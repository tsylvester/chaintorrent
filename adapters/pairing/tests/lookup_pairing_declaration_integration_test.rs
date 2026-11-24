#![cfg(feature = "mocks")]

use pairing::{
    LookupPairingDeclarationParamsOverrides, MockIPairingAdapterFailureMode, PAIRING_CONCRETES,
    PairingConcrete, build_lookup_pairing_declaration_deps,
    build_lookup_pairing_declaration_params, build_lookup_pairing_declaration_payload,
    lookup_pairing_declaration,
};

/// Contract: entry `declaration_names_the_member_looked_up`; each member of
///   `PAIRING_CONCRETES` → `Ok` whose `declaration.concrete` equals that member.
/// Boundary: the public call `lookup_pairing_declaration`, the route reading the
///   member's `DECLARATION` constant.
/// Mocked:   nothing; the route has no outer-edge collaborator, so the test does
///   not prove the mock member.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES`, params built by `build_lookup_pairing_declaration_params`
///   with the member as the concrete override.
/// Act:      `lookup_pairing_declaration(&deps, params, payload)` per member.
/// Assert:   each result binds through the irrefutable `let Ok(success) = …;`;
///   `success.declaration.concrete` equals the member it was looked up by.
#[test]
fn declaration_names_the_member_looked_up() {
    for concrete in PAIRING_CONCRETES {
        // Arrange
        let deps = build_lookup_pairing_declaration_deps(Default::default());
        let params =
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            });
        let payload = build_lookup_pairing_declaration_payload(Default::default());

        // Act
        let result = lookup_pairing_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert_eq!(success.declaration.concrete, *concrete);
    }
}

/// Contract: entry `mock_declaration_names_the_identifier_looked_up`;
///   `PairingConcrete::Mock(mode)` → `Ok` whose `declaration.concrete` is
///   `PairingConcrete::Mock(mode)`.
/// Boundary: the public call `lookup_pairing_declaration`, the route reading
///   `MOCK_I_PAIRING_ADAPTER_DECLARATION`.
/// Mocked:   the mock member itself, so the test proves the identifier is
///   carried and no real member's declaration.
/// Arrange:  `build_lookup_pairing_declaration_params` with the concrete
///   override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`,
///   a mode other than `Succeeds`.
/// Act:      `lookup_pairing_declaration(&deps, params, payload)`.
/// Assert:   the result binds through the irrefutable `let Ok(success) = …;`;
///   `matches!` holds for `success.declaration.concrete` against
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`.
#[cfg(feature = "mocks")]
#[test]
fn mock_declaration_names_the_identifier_looked_up() {
    // Arrange
    let deps = build_lookup_pairing_declaration_deps(Default::default());
    let params = build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
        ..Default::default()
    });
    let payload = build_lookup_pairing_declaration_payload(Default::default());

    // Act
    let result = lookup_pairing_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)
    ));
}
// mock_declaration_names_the_identifier_looked_up

// Public-surface enumeration: the own entries
//   `declaration_names_the_member_looked_up`,
//   proven by `declaration_names_the_member_looked_up`,
//   and `mock_declaration_names_the_identifier_looked_up`,
//   proven by `mock_declaration_names_the_identifier_looked_up`.
