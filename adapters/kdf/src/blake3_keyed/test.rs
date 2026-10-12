#![allow(clippy::expect_used)]

use super::provides::{
    Blake3KeyedKdf, build_blake3_keyed_kdf, build_blake3_keyed_kdf_constructor_params,
    expected_derived_key,
};
use crate::factory::provides::{
    DerivationPurpose, DeriveKeyParamsOverrides, IKeyDerivationAdapter, KDF_INTERFACE_VERSION,
    KdfIdentifier, REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, build_derive_key_params,
    build_derive_key_payload, reference_context,
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
    assert!(result.is_ok());
}

/// Contract: `declaration()`, one branch: any adapter →
///   `Blake3KeyedKdf::DECLARATION`.
/// Arrange: the adapter from `build_blake3_keyed_kdf`.
/// Act:     `adapter.declaration()`.
/// Assert:  the identifier equals `KdfIdentifier::Blake3KeyedV1`, the adapter
///   version equals the literal `1` the interaction spec's `DECLARATION`
///   entry states, and the interface version equals `KDF_INTERFACE_VERSION`.
#[test]
fn declaration_returns_the_blake3_keyed_declaration() {
    // Arrange
    let adapter = build_blake3_keyed_kdf(Default::default());

    // Act
    let declaration = adapter.declaration();

    // Assert
    assert!(declaration.identifier == KdfIdentifier::Blake3KeyedV1);
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, KDF_INTERFACE_VERSION);
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
