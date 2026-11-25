#![allow(clippy::expect_used)]

use super::provides::{
    Blake3KeyedKdf, build_blake3_keyed_kdf, build_blake3_keyed_kdf_constructor_params,
    expected_derived_key,
};
use crate::factory::provides::{
    DERIVATION_PURPOSES, DerivationPurpose, DeriveKeyParamsOverrides, IKeyDerivationAdapter,
    KDF_CONCRETES, KDF_IDENTIFIERS, KDF_INTERFACE_VERSION, KdfConcrete, KdfConcreteOverrides,
    KdfDeclaration, KdfIdentifier, REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH,
    build_derive_key_params, build_derive_key_payload, build_kdf_concrete, reference_context,
};

/// Contract: `Blake3KeyedKdf::try_new(params)`, one branch: any params →
///   `Ok(Blake3KeyedKdf)`; the error arm has no branch.
/// Arrange: the params from `build_blake3_keyed_kdf_constructor_params`, the
///   params declaring no field.
/// Act:     `Blake3KeyedKdf::try_new(params)`.
/// Assert:  `result.is_ok()` is true.
#[test]
fn try_new_returns_the_adapter() {
    // Arrange
    let params = build_blake3_keyed_kdf_constructor_params(Default::default());

    // Act
    let result = Blake3KeyedKdf::try_new(params);

    // Assert
    assert!(matches!(result, Ok(Blake3KeyedKdf)));
}

/// Contract: a direct read of `Blake3KeyedKdf::DECLARATION` →
///   `KdfDeclaration { concrete: KdfConcrete::Blake3Keyed,
///   identifier: KdfIdentifier::Blake3KeyedV1, adapter_version: 1,
///   interface_version: KDF_INTERFACE_VERSION }`, no adapter constructed.
/// Arrange: none; `Blake3KeyedKdf::DECLARATION` is the interface-held
///   constant.
/// Act:     `Blake3KeyedKdf::DECLARATION` is read.
/// Assert:  the concrete matches `KdfConcrete::Blake3Keyed`, the identifier
///   matches `KdfIdentifier::Blake3KeyedV1`, the adapter version equals the
///   literal `1`, and the interface version equals `KDF_INTERFACE_VERSION`.
#[test]
fn declaration_returns_the_blake3_keyed_declaration() {
    // Arrange
    // Act
    let declaration: KdfDeclaration = Blake3KeyedKdf::DECLARATION;

    // Assert
    assert!(matches!(declaration.concrete, KdfConcrete::Blake3Keyed));
    assert!(matches!(
        declaration.identifier,
        KdfIdentifier::Blake3KeyedV1
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, KDF_INTERFACE_VERSION);
}

/// Contract: `KDF_CONCRETES` contains only `KdfConcrete::Blake3Keyed`;
///   `KDF_IDENTIFIERS` contains only `KdfIdentifier::Blake3KeyedV1`;
///   `DERIVATION_PURPOSES` contains exactly the eight purposes in
///   declaration order; every member and value `name()` spelling is
///   distinct; `Blake3KeyedV1` spells `blake3_keyed_v1` with `uint8` wire
///   code `1`; no `MockV1` or mock member exists in a real set;
///   `build_kdf_concrete` returns the real selection without configuration.
/// Arrange: `KDF_CONCRETES`, `KDF_IDENTIFIERS`, and `DERIVATION_PURPOSES`;
///   `build_kdf_concrete` over `KdfConcrete::Blake3Keyed` with fieldless
///   overrides.
/// Act:     each declared set and each member's and value's `name()` is
///   read; the declared-selection builder is run.
/// Assert:  the concrete set is exactly `Blake3Keyed`, the identifier set
///   exactly `Blake3KeyedV1`, and the purpose set exactly the eight
///   variants; the spellings are pairwise distinct; `Blake3KeyedV1` spells
///   `blake3_keyed_v1` and its `uint8` wire code is `1`; no `MockV1` exists
///   and no mock member appears in a real set; the builder returns
///   `KdfConcrete::Blake3Keyed` carrying no configuration.
#[test]
fn the_declared_sets_and_spellings_are_total_and_real() {
    // Arrange
    // Act
    let selection = build_kdf_concrete(KdfConcrete::Blake3Keyed, KdfConcreteOverrides);

    // Assert
    assert!(matches!(KDF_CONCRETES, [KdfConcrete::Blake3Keyed]));
    assert!(matches!(KDF_IDENTIFIERS, [KdfIdentifier::Blake3KeyedV1]));
    assert!(matches!(
        DERIVATION_PURPOSES,
        [
            DerivationPurpose::WrappingKey,
            DerivationPurpose::PublisherRoot,
            DerivationPurpose::AssetRoot,
            DerivationPurpose::MasterScalar,
            DerivationPurpose::IdentityBases,
            DerivationPurpose::CapsuleRandomness,
            DerivationPurpose::PieceGroupKey,
            DerivationPurpose::PlaintextRootKey,
        ]
    ));
    let spellings = [
        KdfConcrete::Blake3Keyed.name(),
        KdfIdentifier::Blake3KeyedV1.name(),
        DerivationPurpose::WrappingKey.name(),
        DerivationPurpose::PublisherRoot.name(),
        DerivationPurpose::AssetRoot.name(),
        DerivationPurpose::MasterScalar.name(),
        DerivationPurpose::IdentityBases.name(),
        DerivationPurpose::CapsuleRandomness.name(),
        DerivationPurpose::PieceGroupKey.name(),
        DerivationPurpose::PlaintextRootKey.name(),
    ];
    for (index, spelling) in spellings.iter().enumerate() {
        assert!(spellings[..index].iter().all(|earlier| earlier != spelling));
    }
    assert_eq!(KdfIdentifier::Blake3KeyedV1.name(), "blake3_keyed_v1");
    assert_eq!(KdfIdentifier::Blake3KeyedV1.wire_code(), 1);
    assert!(matches!(selection, KdfConcrete::Blake3Keyed));
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the wrapping-key
///   context string over the length prefix, the key material, and the
///   context, in that order.
/// Arrange: the adapter, the params with the purpose `WrappingKey` and the
///   reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `WrappingKey`, the reference context,
///   and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_wrapping_key_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::WrappingKey,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the publisher-root
///   context string over the length prefix, the key material, and the
///   context, in that order.
/// Arrange: the adapter, the params with the purpose `PublisherRoot` and the
///   reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `PublisherRoot`, the reference
///   context, and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_publisher_root_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::PublisherRoot),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::PublisherRoot,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the asset-root
///   context string over the length prefix, the key material, and the
///   context, in that order.
/// Arrange: the adapter, the params with the purpose `AssetRoot` and the
///   reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `AssetRoot`, the reference context,
///   and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_asset_root_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::AssetRoot),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::AssetRoot,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the master-scalar
///   context string over the length prefix, the key material, and the
///   context, in that order.
/// Arrange: the adapter, the params with the purpose `MasterScalar` and the
///   reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `MasterScalar`, the reference
///   context, and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_master_scalar_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::MasterScalar),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::MasterScalar,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the identity-bases
///   context string over the length prefix, the key material, and the
///   context, in that order.
/// Arrange: the adapter, the params with the purpose `IdentityBases` and the
///   reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `IdentityBases`, the reference
///   context, and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_identity_bases_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::IdentityBases),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::IdentityBases,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the
///   capsule-randomness context string over the length prefix, the key
///   material, and the context, in that order.
/// Arrange: the adapter, the params with the purpose `CapsuleRandomness` and
///   the reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `CapsuleRandomness`, the reference
///   context, and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_capsule_randomness_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::CapsuleRandomness),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::CapsuleRandomness,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the piece-group-key
///   context string over the length prefix, the key material, and the
///   context, in that order.
/// Arrange: the adapter, the params with the purpose `PieceGroupKey` and the
///   reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `PieceGroupKey`, the reference
///   context, and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_piece_group_key_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::PieceGroupKey),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::PieceGroupKey,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: the length converts →
///   `Ok(DeriveKeySuccessReturn { key })` derived under the plaintext-root
///   context string over the length prefix, the key material, and the
///   context, in that order.
/// Arrange: the adapter, the params with the purpose `PlaintextRootKey` and
///   the reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals `expected_derived_key` over `PlaintextRootKey`, the reference
///   context, and `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_for_plaintext_root_key_matches_the_expected_derived_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::PlaintextRootKey),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        expected_derived_key(
            DerivationPurpose::PlaintextRootKey,
            reference_context(),
            REFERENCE_LENGTH,
        )
        .as_slice()
    );
}

/// Contract: `derive_key`, derived: a requested length shorter than the
///   reference length → `Ok` with a key that is the leading bytes of the
///   longer output.
/// Arrange: the adapter, the params with the purpose `WrappingKey` and the
///   length `REFERENCE_PREFIX_LENGTH`, so the requested length differs from
///   the reference length, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals the first `REFERENCE_PREFIX_LENGTH` bytes of
///   `expected_derived_key` over `WrappingKey`, the reference context, and
///   `REFERENCE_LENGTH` by `assert_eq!`.
#[test]
fn derive_key_of_the_prefix_length_is_the_prefix_of_the_reference_length_output() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        length: Some(REFERENCE_PREFIX_LENGTH),
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        success.key.expose().as_slice(),
        &expected_derived_key(
            DerivationPurpose::WrappingKey,
            reference_context(),
            REFERENCE_LENGTH,
        )[..REFERENCE_PREFIX_LENGTH]
    );
}

/// Contract: `derive_key`, derived: `params.length` of zero → `Ok` with an
///   empty key.
/// Arrange: the adapter, the params with the length `0`, so the length
///   differs from `REFERENCE_LENGTH`, and the reference payload.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the key's `expose()`
///   equals the empty byte sequence by `assert_eq!`.
#[test]
fn derive_key_of_zero_bytes_returns_an_empty_key() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        length: Some(0),
        ..Default::default()
    });
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(success.key.expose().as_slice(), b"");
}
