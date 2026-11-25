#![cfg(feature = "mocks")]

use core::mem::discriminant;
use kdf::{
    KDF_CONCRETES, KDF_IDENTIFIERS, KdfConcrete, KdfConcreteOverrides, KdfIdentifier,
    LookupKdfDeclarationParamsOverrides, MockIKeyDerivationAdapterFailureMode, build_kdf_concrete,
    build_lookup_kdf_declaration_deps, build_lookup_kdf_declaration_params,
    build_lookup_kdf_declaration_payload, lookup_kdf_declaration,
};

/// Contract: `lookup_kdf_declaration` is called through the crate's public
///   surface with the member identifier of each member of `KDF_CONCRETES`
///   and with no key-derivation member constructed anywhere in the call →
///   `Ok` carrying a declaration whose `identifier` is a member of
///   `KDF_IDENTIFIERS`, whose `adapter_version` is `1`, and whose
///   `interface_version` is `1`.
/// Arrange: for each `concrete` in `KDF_CONCRETES`,
///   `build_lookup_kdf_declaration_params` with the `concrete` override
///   `build_kdf_concrete(concrete, KdfConcreteOverrides::default())`;
///   `build_lookup_kdf_declaration_deps` and
///   `build_lookup_kdf_declaration_payload` with no override; the body is
///   written once and runs over the declared set, and the call is made
///   from outside the crate with no member and no factory involved.
/// Act:     `lookup_kdf_declaration(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through the irrefutable pattern
///   `let Ok(success) = result;`; `KDF_IDENTIFIERS` holds a member whose
///   `discriminant` equals `discriminant(&success.declaration.identifier)`,
///   the identifiers compared as identifiers through `discriminant` since
///   `KdfIdentifier` derives no `PartialEq`;
///   `success.declaration.adapter_version` equals the literal `1`;
///   `success.declaration.interface_version` equals the literal `1`.
/// Boundary: the public call `lookup_kdf_declaration` and the route the
///   spec's own entries carry, through the match on the member identifier
///   to the member's declaration constant.
/// Mocked:  nothing; the test constructs no member and does not prove the
///   mock member's arm, which the next block proves.
#[test]
fn lookup_kdf_declaration_is_reachable_through_the_crate_surface_without_a_constructed_member() {
    // Arrange
    for concrete in KDF_CONCRETES {
        let deps = build_lookup_kdf_declaration_deps(Default::default());
        let params = build_lookup_kdf_declaration_params(LookupKdfDeclarationParamsOverrides {
            concrete: Some(build_kdf_concrete(*concrete, KdfConcreteOverrides)),
        });
        let payload = build_lookup_kdf_declaration_payload(Default::default());

        // Act
        let result = lookup_kdf_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert!(
            KDF_IDENTIFIERS
                .iter()
                .any(|identifier| discriminant(identifier)
                    == discriminant(&success.declaration.identifier))
        );
        assert_eq!(success.declaration.adapter_version, 1);
        assert_eq!(success.declaration.interface_version, 1);
    }
}

/// Contract: `lookup_kdf_declaration` is called through the crate's public
///   surface under the `mocks` feature with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`
///   → `Ok` carrying a declaration whose `concrete` is that selected mock
///   member, whose `identifier` is `KdfIdentifier::Blake3KeyedV1`, whose
///   `adapter_version` is `1`, and whose `interface_version` is `1`.
/// Arrange: `build_lookup_kdf_declaration_params` with the `concrete`
///   override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the selected failure mode differs from the `NoFailure` member the
///   mock constant names; `build_lookup_kdf_declaration_deps` and
///   `build_lookup_kdf_declaration_payload` with no override.
/// Act:     `lookup_kdf_declaration(&deps, params, payload)`.
/// Assert:  the success arm is bound through the irrefutable pattern
///   `let Ok(success) = result;`; `success.declaration.concrete` matches
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`;
///   `success.declaration.identifier` matches `KdfIdentifier::Blake3KeyedV1`;
///   `success.declaration.adapter_version` equals the literal `1`;
///   `success.declaration.interface_version` equals the literal `1`.
/// Boundary: the public call `lookup_kdf_declaration` and the route the
///   spec's own entries carry, through the match on the member identifier
///   to `MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION`.
/// Mocked:  nothing; the mock member is the subject's own input.
#[test]
fn lookup_kdf_declaration_carries_the_selected_mock_member_through_the_crate_surface() {
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
    assert!(matches!(
        success.declaration.identifier,
        KdfIdentifier::Blake3KeyedV1
    ));
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(success.declaration.interface_version, 1);
}
