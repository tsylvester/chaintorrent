use super::provides::{
    LookupPairingDeclarationParamsOverrides, build_lookup_pairing_declaration_deps,
    build_lookup_pairing_declaration_params, build_lookup_pairing_declaration_payload,
    lookup_pairing_declaration,
};
use crate::factory::provides::{
    MockIPairingAdapterFailureMode, PairingConcrete, PairingCurve, PrecompileEncoding,
    TargetGroupEncodingIdentifier, VerifierGroupArithmetic,
};

/// Contract: `params.concrete` is `PairingConcrete::Bn254Arkworks` → `Ok` holding the
///   declaration `PairingDeclaration { concrete: PairingConcrete::Bn254Arkworks,
///   curve: PairingCurve::Bn254,
///   verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly,
///   precompile_encoding: PrecompileEncoding::Eip196Eip197,
///   target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1,
///   interface_version: 1 }`.
/// Collaborators: the member's `DECLARATION` constant, read for real by the function;
///   fixtures `build_lookup_pairing_declaration_deps`,
///   `build_lookup_pairing_declaration_params`, and
///   `build_lookup_pairing_declaration_payload`.
/// Arrange: `build_lookup_pairing_declaration_params` with the concrete override
///   `PairingConcrete::Bn254Arkworks`, a member whose declaration differs from every
///   other member's in `concrete`, so a lookup that returns another member's
///   declaration fails.
/// Act:     `lookup_pairing_declaration(&deps, params, payload)`.
/// Assert:  the result binds through the irrefutable `let Ok(success) = …;`; `matches!`
///   holds for `success.declaration.concrete`, `curve`, `verifier_group_arithmetic`,
///   `precompile_encoding`, and `target_group_encoding` against the values in the
///   contract, and `adapter_version` and `interface_version` each equal the literal `1`.
#[test]
fn lookup_returns_the_bn254_arkworks_declaration() {
    // Arrange
    let deps = build_lookup_pairing_declaration_deps(Default::default());
    let params = build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
    });
    let payload = build_lookup_pairing_declaration_payload(Default::default());

    // Act
    let result = lookup_pairing_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        PairingConcrete::Bn254Arkworks
    ));
    assert!(matches!(success.declaration.curve, PairingCurve::Bn254));
    assert!(matches!(
        success.declaration.verifier_group_arithmetic,
        VerifierGroupArithmetic::FirstGroupOnly
    ));
    assert!(matches!(
        success.declaration.precompile_encoding,
        PrecompileEncoding::Eip196Eip197
    ));
    assert!(matches!(
        success.declaration.target_group_encoding,
        TargetGroupEncodingIdentifier::Bn254V1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}

/// Contract: `params.concrete` is `PairingConcrete::Bn254Halo2curves` → `Ok` holding
///   the declaration `PairingDeclaration { concrete: PairingConcrete::Bn254Halo2curves,
///   curve: PairingCurve::Bn254,
///   verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly,
///   precompile_encoding: PrecompileEncoding::Eip196Eip197,
///   target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1,
///   interface_version: 1 }`.
/// Collaborators: the member's `DECLARATION` constant, read for real by the function;
///   the fixtures of the previous block.
/// Arrange: `build_lookup_pairing_declaration_params` with the concrete override
///   `PairingConcrete::Bn254Halo2curves`, whose declaration differs from the arkworks
///   member's in `concrete` alone, so a lookup that returns the arkworks member's
///   declaration fails.
/// Act:     `lookup_pairing_declaration(&deps, params, payload)`.
/// Assert:  the result binds through the irrefutable `let Ok(success) = …;`; `matches!`
///   holds for each enum field of `success.declaration` against the values in the
///   contract, and `adapter_version` and `interface_version` each equal the literal `1`.
#[test]
fn lookup_returns_the_bn254_halo2curves_declaration() {
    // Arrange
    let deps = build_lookup_pairing_declaration_deps(Default::default());
    let params = build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
    });
    let payload = build_lookup_pairing_declaration_payload(Default::default());

    // Act
    let result = lookup_pairing_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        PairingConcrete::Bn254Halo2curves
    ));
    assert!(matches!(success.declaration.curve, PairingCurve::Bn254));
    assert!(matches!(
        success.declaration.verifier_group_arithmetic,
        VerifierGroupArithmetic::FirstGroupOnly
    ));
    assert!(matches!(
        success.declaration.precompile_encoding,
        PrecompileEncoding::Eip196Eip197
    ));
    assert!(matches!(
        success.declaration.target_group_encoding,
        TargetGroupEncodingIdentifier::Bn254V1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}

/// Contract: `params.concrete` is `PairingConcrete::Bls12381Arkworks` → `Ok` holding
///   the declaration `PairingDeclaration { concrete: PairingConcrete::Bls12381Arkworks,
///   curve: PairingCurve::Bls12381,
///   verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups,
///   precompile_encoding: PrecompileEncoding::Eip2537,
///   target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1,
///   adapter_version: 1, interface_version: 1 }`.
/// Collaborators: the member's `DECLARATION` constant, read for real by the function;
///   the fixtures of the previous blocks.
/// Arrange: `build_lookup_pairing_declaration_params` with the concrete override
///   `PairingConcrete::Bls12381Arkworks`, whose curve, arithmetic, precompile
///   encoding, and target-group encoding differ from the BN254 members', so a lookup
///   that returns a BN254 declaration fails.
/// Act:     `lookup_pairing_declaration(&deps, params, payload)`.
/// Assert:  the result binds through the irrefutable `let Ok(success) = …;`; `matches!`
///   holds for each enum field of `success.declaration` against the values in the
///   contract, and `adapter_version` and `interface_version` each equal the literal `1`.
#[test]
fn lookup_returns_the_bls12_381_arkworks_declaration() {
    // Arrange
    let deps = build_lookup_pairing_declaration_deps(Default::default());
    let params = build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
    });
    let payload = build_lookup_pairing_declaration_payload(Default::default());

    // Act
    let result = lookup_pairing_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        PairingConcrete::Bls12381Arkworks
    ));
    assert!(matches!(success.declaration.curve, PairingCurve::Bls12381));
    assert!(matches!(
        success.declaration.verifier_group_arithmetic,
        VerifierGroupArithmetic::BothGroups
    ));
    assert!(matches!(
        success.declaration.precompile_encoding,
        PrecompileEncoding::Eip2537
    ));
    assert!(matches!(
        success.declaration.target_group_encoding,
        TargetGroupEncodingIdentifier::Bls12381V1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}

/// Contract: `params.concrete` is `PairingConcrete::Bls12381Halo2curves` → `Ok`
///   holding the declaration `PairingDeclaration { concrete:
///   PairingConcrete::Bls12381Halo2curves, curve: PairingCurve::Bls12381,
///   verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups,
///   precompile_encoding: PrecompileEncoding::Eip2537,
///   target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1,
///   adapter_version: 1, interface_version: 1 }`.
/// Collaborators: the member's `DECLARATION` constant, read for real by the function;
///   the fixtures of the previous blocks.
/// Arrange: `build_lookup_pairing_declaration_params` with the concrete override
///   `PairingConcrete::Bls12381Halo2curves`, whose declaration differs from the
///   arkworks BLS12-381 member's in `concrete` alone, so a lookup that returns the
///   arkworks member's declaration fails.
/// Act:     `lookup_pairing_declaration(&deps, params, payload)`.
/// Assert:  the result binds through the irrefutable `let Ok(success) = …;`; `matches!`
///   holds for each enum field of `success.declaration` against the values in the
///   contract, and `adapter_version` and `interface_version` each equal the literal `1`.
#[test]
fn lookup_returns_the_bls12_381_halo2curves_declaration() {
    // Arrange
    let deps = build_lookup_pairing_declaration_deps(Default::default());
    let params = build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
    });
    let payload = build_lookup_pairing_declaration_payload(Default::default());

    // Act
    let result = lookup_pairing_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        PairingConcrete::Bls12381Halo2curves
    ));
    assert!(matches!(success.declaration.curve, PairingCurve::Bls12381));
    assert!(matches!(
        success.declaration.verifier_group_arithmetic,
        VerifierGroupArithmetic::BothGroups
    ));
    assert!(matches!(
        success.declaration.precompile_encoding,
        PrecompileEncoding::Eip2537
    ));
    assert!(matches!(
        success.declaration.target_group_encoding,
        TargetGroupEncodingIdentifier::Bls12381V1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}

/// Contract: `params.concrete` is `PairingConcrete::Mock(mode)` → `Ok` holding the
///   declaration of `MOCK_I_PAIRING_ADAPTER_DECLARATION` with `concrete` equal to
///   `PairingConcrete::Mock(mode)`, whose provided values are `PairingCurve::Bn254`,
///   `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`,
///   `TargetGroupEncodingIdentifier::Bn254V1`, `adapter_version: 1`, and
///   `interface_version: 1`.
/// Collaborators: `MOCK_I_PAIRING_ADAPTER_DECLARATION`, read for real by the function;
///   the fixtures of the previous blocks.
/// Arrange: `build_lookup_pairing_declaration_params` with the concrete override
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a
///   mode other than the constant's `Succeeds`, so a lookup that returns the
///   constant's own `concrete` fails.
/// Act:     `lookup_pairing_declaration(&deps, params, payload)`.
/// Assert:  the result binds through the irrefutable `let Ok(success) = …;`; `matches!`
///   holds for `success.declaration.concrete` against
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)` and
///   for each enum field against the values in the contract, and `adapter_version`
///   and `interface_version` each equal the literal `1`.
#[test]
fn lookup_returns_the_mock_declaration_carrying_the_identifier_looked_up() {
    // Arrange
    let deps = build_lookup_pairing_declaration_deps(Default::default());
    let params = build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
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
    assert!(matches!(success.declaration.curve, PairingCurve::Bn254));
    assert!(matches!(
        success.declaration.verifier_group_arithmetic,
        VerifierGroupArithmetic::FirstGroupOnly
    ));
    assert!(matches!(
        success.declaration.precompile_encoding,
        PrecompileEncoding::Eip196Eip197
    ));
    assert!(matches!(
        success.declaration.target_group_encoding,
        TargetGroupEncodingIdentifier::Bn254V1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}
