use super::provides::{
    LookupKdfDeclarationParamsOverrides, build_lookup_kdf_declaration_deps,
    build_lookup_kdf_declaration_params, build_lookup_kdf_declaration_payload,
    lookup_kdf_declaration,
};

use crate::factory::provides::{KdfConcrete, KdfIdentifier, MockIKeyDerivationAdapterFailureMode};

/// Contract: `lookup_kdf_declaration`, real member: `params.concrete` is
///   `KdfConcrete::Blake3Keyed` → `Ok` carrying the declaration whose
///   `concrete` is `KdfConcrete::Blake3Keyed`, `identifier` is
///   `KdfIdentifier::Blake3KeyedV1`, `adapter_version` is `1`, and
///   `interface_version` is `1`.
/// Collaborators: none; `Blake3KeyedKdf::DECLARATION` is read for real.
/// Arrange: `build_lookup_kdf_declaration_deps` and
///   `build_lookup_kdf_declaration_payload` with no override;
///   `build_lookup_kdf_declaration_params` with no override, whose default
///   member is the real `Blake3Keyed`, so the request differs from the mock
///   members the sibling blocks request.
/// Act:     `lookup_kdf_declaration(&deps, params, payload)`.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.concrete` matches
///   `KdfConcrete::Blake3Keyed`; `success.declaration.identifier` matches
///   `KdfIdentifier::Blake3KeyedV1`; `success.declaration.adapter_version`
///   equals the literal `1`; `success.declaration.interface_version` equals
///   the literal `1`.
#[test]
fn lookup_kdf_declaration_returns_the_blake3_keyed_declaration_for_the_real_member() {
    // Arrange
    let deps = build_lookup_kdf_declaration_deps(Default::default());
    let params = build_lookup_kdf_declaration_params(Default::default());
    let payload = build_lookup_kdf_declaration_payload(Default::default());

    // Act
    let result = lookup_kdf_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        KdfConcrete::Blake3Keyed
    ));
    assert!(matches!(
        success.declaration.identifier,
        KdfIdentifier::Blake3KeyedV1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}

/// Contract: `lookup_kdf_declaration`, mock member: `params.concrete` is
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::NoFailure)` →
///   `Ok` carrying the declaration whose `concrete` is that mock member,
///   `identifier` is `KdfIdentifier::Blake3KeyedV1`, `adapter_version` is
///   `1`, and `interface_version` is `1`.
/// Collaborators: none; `MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION` is read
///   for real.
/// Arrange: `build_lookup_kdf_declaration_deps` and
///   `build_lookup_kdf_declaration_payload` with no override;
///   `build_lookup_kdf_declaration_params` with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::NoFailure)`,
///   so the request names the mock member.
/// Act:     `lookup_kdf_declaration(&deps, params, payload)`.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.concrete` matches
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::NoFailure)`;
///   `success.declaration.identifier` matches `KdfIdentifier::Blake3KeyedV1`;
///   `success.declaration.adapter_version` equals the literal `1`;
///   `success.declaration.interface_version` equals the literal `1`.
#[test]
fn lookup_kdf_declaration_returns_the_mock_declaration_for_the_no_failure_mock_member() {
    // Arrange
    let deps = build_lookup_kdf_declaration_deps(Default::default());
    let params = build_lookup_kdf_declaration_params(LookupKdfDeclarationParamsOverrides {
        concrete: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )),
    });
    let payload = build_lookup_kdf_declaration_payload(Default::default());

    // Act
    let result = lookup_kdf_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::NoFailure)
    ));
    assert!(matches!(
        success.declaration.identifier,
        KdfIdentifier::Blake3KeyedV1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}

/// Contract: `lookup_kdf_declaration`, mock member: `params.concrete` is
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`
///   → `Ok` carrying a declaration whose `concrete` is that selected mock
///   member and not the `NoFailure` member the mock constant names.
/// Collaborators: none; `MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION` is read
///   for real.
/// Arrange: `build_lookup_kdf_declaration_deps` and
///   `build_lookup_kdf_declaration_payload` with no override;
///   `build_lookup_kdf_declaration_params` with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the selected failure mode differs from the mock constant's.
/// Act:     `lookup_kdf_declaration(&deps, params, payload)`.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.concrete` matches
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`.
#[test]
fn lookup_kdf_declaration_carries_the_selected_failure_mode_for_the_mock_member() {
    // Arrange
    let deps = build_lookup_kdf_declaration_deps(Default::default());
    let params = build_lookup_kdf_declaration_params(LookupKdfDeclarationParamsOverrides {
        concrete: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
    });
    let payload = build_lookup_kdf_declaration_payload(Default::default());

    // Act
    let result = lookup_kdf_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(matches!(
        success.declaration.concrete,
        KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)
    ));
}
