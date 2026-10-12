#![allow(clippy::expect_used)]

use super::provides::{build_blake3_keyed_kdf, expected_derived_key};
use crate::factory::provides::{
    DerivationPurpose, DeriveKeyParamsOverrides, DeriveKeyPayloadOverrides, IKeyDerivationAdapter,
    REFERENCE_LENGTH, build_derive_key_params, build_derive_key_payload, reference_context,
    reference_key_material_bytes,
};
use domain::build_secret;

/// Contract: the own entry `derive_key_leaves_the_borrowed_key_material_intact`:
///   `derive_key` is called with a living `Secret<Vec<u8>>` borrowed in the
///   payload → after the call the secret's `expose()` equals the bytes it
///   held before the call.
/// Arrange: `build_secret` with the value `reference_key_material_bytes()`,
///   whose bytes are not all zero, so a zeroized secret differs from an
///   intact one; `build_blake3_keyed_kdf` with no override;
///   `build_derive_key_params` with the purpose override `WrappingKey`;
///   `build_derive_key_payload` with the key material override set to a
///   borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; the secret's `expose()`
///   equals `reference_key_material_bytes()` by `assert_eq!`.
/// Boundary: the crate-internal path `Blake3KeyedKdf::derive_key` through
///   `build_blake3_keyed_kdf` and `Blake3KeyedKdf::try_new`, running
///   `blake3::Hasher`, `blake3::OutputReader`, `Zeroize::zeroize`, and
///   `Secret::try_new`, with `Secret::expose` reading the result.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real, so this
///   block does not prove the zeroization of the hasher and the reader
///   themselves, which no caller can observe.
#[test]
fn derive_key_leaves_the_borrowed_key_material_intact() {
    // Arrange
    let key_material = build_secret(reference_key_material_bytes(), Default::default());
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        ..Default::default()
    });
    let payload = build_derive_key_payload(DeriveKeyPayloadOverrides {
        key_material: Some(&key_material),
        ..Default::default()
    });

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let _success =
        result.expect("the key material's length converts, so the derivation returns a key");
    assert_eq!(
        key_material.expose().as_slice(),
        reference_key_material_bytes().as_slice()
    );
}

/// Contract: the own entry `derive_key_returns_a_key_that_outlives_the_key_material`:
///   `derive_key` returns `Ok` and the key material's secret is then dropped,
///   its drop zeroizing its value → the returned key's `expose()` still
///   equals the derived bytes.
/// Arrange: `build_secret` with the value `reference_key_material_bytes()`,
///   so the dropped secret holds non-zero bytes that its drop overwrites;
///   `build_blake3_keyed_kdf` with no override; `build_derive_key_params`
///   with the purpose override `WrappingKey`; `build_derive_key_payload`
///   with the key material override set to a borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  the success arm binds through `expect`; after `drop` of the key
///   material's secret, the returned key's `expose()` equals
///   `expected_derived_key` over `WrappingKey`, the reference context, and
///   `REFERENCE_LENGTH` by `assert_eq!`.
/// Boundary: the crate-internal path `Blake3KeyedKdf::derive_key` through
///   `build_blake3_keyed_kdf` and `Blake3KeyedKdf::try_new`, running
///   `blake3::Hasher`, `blake3::OutputReader`, `Zeroize::zeroize`, and
///   `Secret::try_new`, with the key material's `Secret` drop running for
///   real.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn derive_key_returns_a_key_that_outlives_the_key_material() {
    // Arrange
    let key_material = build_secret(reference_key_material_bytes(), Default::default());
    let adapter = build_blake3_keyed_kdf(Default::default());
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        ..Default::default()
    });
    let payload = build_derive_key_payload(DeriveKeyPayloadOverrides {
        key_material: Some(&key_material),
        ..Default::default()
    });

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    let success =
        result.expect("the key material's length converts, so the derivation returns a key");
    drop(key_material);
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
