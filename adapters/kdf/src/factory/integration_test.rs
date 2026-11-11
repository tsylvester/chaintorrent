#![allow(clippy::expect_used)]

use super::provides::{
    CreateKeyDerivationErrorReturn, CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS,
    DeriveKeyErrorReturn, DeriveKeyParamsOverrides, DeriveKeyPayloadOverrides,
    IKeyDerivationAdapter, KDF_INTERFACE_VERSION, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload,
    build_derive_key_params, build_derive_key_payload, create_key_derivation,
    reference_key_material_bytes,
};
use domain::build_secret;

/// Contract: `create_key_derivation` is called with the selection of a
///   concrete and the identifier that concrete declares →
///   `Ok(CreateKeyDerivationSuccessReturn { adapter })` with
///   `adapter.declaration()` reporting the identifier required and the
///   interface version `KDF_INTERFACE_VERSION`.
/// Arrange: for each case, `build_create_key_derivation_deps`,
///   `build_create_key_derivation_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_key_derivation_payload`; the cases declare different
///   identifiers, so a factory that constructs a concrete other than the
///   selected one reports another identifier.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`;
///   `adapter.declaration().identifier` equals the case's identifier by
///   `assert!`, and `adapter.declaration().interface_version` equals
///   `KDF_INTERFACE_VERSION` by `assert_eq!`.
/// Boundary: the crate-internal path `create_key_derivation` through the
///   selected concrete's `try_new`, with the declaration read through the
///   returned adapter.
/// Mocked:  nothing; the declared concretes and the mock concrete run as
///   themselves, so this block does not prove a derivation.
#[test]
fn every_concrete_is_constructed_under_the_identifier_it_declares() {
    // Arrange
    let mock_selection = build_create_key_derivation_params(Default::default());
    for selection in DECLARED_KDF_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_key_derivation_deps(Default::default());
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_key_derivation_payload(Default::default());

        // Act
        let result = create_key_derivation(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the adapter");
        let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
        assert!(adapter.declaration().identifier == selection.identifier);
        assert_eq!(
            adapter.declaration().interface_version,
            KDF_INTERFACE_VERSION
        );
    }
}

/// Contract: `create_key_derivation` is called with the selection of a
///   concrete and an identifier that concrete does not declare →
///   `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`.
/// Arrange: for each pair of cases whose declared identifiers differ,
///   `build_create_key_derivation_deps`,
///   `build_create_key_derivation_params` overriding `concrete` with the
///   first case's selection and `identifier` with the second case's
///   identifier, and `build_create_key_derivation_payload`; every pairing
///   carries an identifier the selected concrete does not declare, so an
///   admission that compares against a constant or ignores the selection
///   admits a pairing it must refuse.
/// Act:     `create_key_derivation(&deps, params, payload)` for the pairing.
/// Assert:  `result.err()` equals
///   `Some(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)` by
///   `assert_eq!`.
/// Boundary: the crate-internal path `create_key_derivation`, whose
///   admission reads the selected concrete's declaration.
/// Mocked:  nothing.
#[test]
fn every_concrete_is_refused_under_an_identifier_it_does_not_declare() {
    // Arrange
    let mock_selection = build_create_key_derivation_params(Default::default());
    for case in DECLARED_KDF_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        for other in DECLARED_KDF_SELECTIONS
            .iter()
            .chain(core::iter::once(&mock_selection))
        {
            if other.identifier == case.identifier {
                continue;
            }
            let deps = build_create_key_derivation_deps(Default::default());
            let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
                concrete: Some(case.concrete),
                identifier: Some(other.identifier),
            });
            let payload = build_create_key_derivation_payload(Default::default());

            // Act
            let result = create_key_derivation(&deps, params, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)
            );
        }
    }
}

/// Contract: the adapter `create_key_derivation` returns derives with a
///   `params.length` → `Ok(DeriveKeySuccessReturn { key })` with the key's
///   byte length equal to `params.length`.
/// Arrange: for each case, the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`, the case's params, and
///   `build_create_key_derivation_payload`, bound through `expect`; for
///   each length of `REFERENCE_LENGTH`, `REFERENCE_PREFIX_LENGTH`, and
///   zero, `build_derive_key_params` with the length override and
///   `build_derive_key_payload` with no override; the lengths differ, so a
///   concrete that returns the reference length whatever is requested
///   fails.
/// Act:     `adapter.derive_key(params, payload)` for the case and the
///   length.
/// Assert:  the success arm is bound through `expect`; the key's
///   `expose().len()` equals the length override by `assert_eq!`.
/// Boundary: the crate-internal path `create_key_derivation` through the
///   selected concrete's `try_new`, then the returned adapter's
///   `derive_key`.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_concrete_derives_a_key_of_the_requested_length() {
    // Arrange
    let mock_selection = build_create_key_derivation_params(Default::default());
    for selection in DECLARED_KDF_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_key_derivation_deps(Default::default());
        let payload = build_create_key_derivation_payload(Default::default());
        let success = create_key_derivation(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
        for length in [REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, 0] {
            let params = build_derive_key_params(DeriveKeyParamsOverrides {
                length: Some(length),
                ..Default::default()
            });
            let payload = build_derive_key_payload(Default::default());

            // Act
            let result = adapter.derive_key(params, payload);

            // Assert
            let success = result.expect("the requested length derives a key");
            assert_eq!(success.key.expose().len(), length);
        }
    }
}

/// Contract: the adapter `create_key_derivation` returns for the mock
///   selection under the failure mode `KeyMaterialLengthUnrepresentable`
///   derives →
///   `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))`
///   with `length` the key material's length.
/// Arrange: the case is the mock selection alone, which the declared set
///   cannot reach; `build_create_key_derivation_params` with the
///   `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`;
///   the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`, those params, and
///   `build_create_key_derivation_payload`, bound through `expect`;
///   `build_derive_key_params` and `build_derive_key_payload` with no
///   override, whose key material is the reference key material.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))`
///   by `assert_eq!`, with `length` the length of
///   `reference_key_material_bytes()`.
/// Boundary: the crate-internal path `create_key_derivation` through the
///   mock concrete's `try_new` with the configured failure mode, then the
///   returned adapter's `derive_key`.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_its_own_error_when_the_key_material_length_cannot_be_framed() {
    // Arrange
    let deps = build_create_key_derivation_deps(Default::default());
    let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        concrete: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
        ..Default::default()
    });
    let payload = build_create_key_derivation_payload(Default::default());
    let success =
        create_key_derivation(&deps, params, payload).expect("the mock selection is admitted");
    let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
    let params = build_derive_key_params(Default::default());
    let payload = build_derive_key_payload(Default::default());

    // Act
    let result = adapter.derive_key(params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DeriveKeyErrorReturn::Mock(
            MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                length: reference_key_material_bytes().len(),
            }
        ))
    );
}

/// Contract: the absorbed entry `kdf/blake3_keyed`
///   `derive_key_leaves_the_borrowed_key_material_intact`, in this node's
///   terms: `derive_key` is called on the adapter `create_key_derivation`
///   returns with a living `Secret<Vec<u8>>` borrowed in the payload →
///   after the call the secret's `expose()` equals the bytes it held
///   before the call.
/// Arrange: `build_secret` with the value `reference_key_material_bytes()`,
///   whose bytes are not all zero, so a zeroized secret differs from an
///   intact one; for each case, the adapter from `create_key_derivation`
///   over `build_create_key_derivation_deps`, the case's params, and
///   `build_create_key_derivation_payload`, bound through `expect`;
///   `build_derive_key_params` with no override;
///   `build_derive_key_payload` with the key material override set to a
///   borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the secret's
///   `expose()` equals `reference_key_material_bytes()` by `assert_eq!`.
/// Boundary: the crate-internal path `create_key_derivation` through the
///   selected concrete's `try_new`, then the returned adapter's
///   `derive_key`, with `Secret::expose` reading the key material.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real, so
///   this block does not prove the zeroization of a vendor's hasher and
///   reader themselves, which no caller can observe.
#[test]
fn every_concrete_leaves_the_borrowed_key_material_intact() {
    // Arrange
    let mock_selection = build_create_key_derivation_params(Default::default());
    for selection in DECLARED_KDF_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let key_material = build_secret(reference_key_material_bytes(), Default::default());
        let deps = build_create_key_derivation_deps(Default::default());
        let payload = build_create_key_derivation_payload(Default::default());
        let success = create_key_derivation(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
        let params = build_derive_key_params(Default::default());
        let payload = build_derive_key_payload(DeriveKeyPayloadOverrides {
            key_material: Some(&key_material),
            ..Default::default()
        });

        // Act
        let result = adapter.derive_key(params, payload);

        // Assert
        let _success = result.expect("the derivation returns a key");
        assert_eq!(
            key_material.expose().as_slice(),
            reference_key_material_bytes().as_slice()
        );
    }
}

/// Contract: the absorbed entry `kdf/blake3_keyed`
///   `derive_key_returns_a_key_that_outlives_the_key_material`, in this
///   node's terms: `derive_key` returns `Ok` on the adapter
///   `create_key_derivation` returns and the key material's secret is then
///   dropped, its drop zeroizing its value → the returned key's `expose()`
///   still equals the bytes it held before the drop.
/// Arrange: `build_secret` with the value `reference_key_material_bytes()`,
///   so the dropped secret holds non-zero bytes that its drop overwrites;
///   for each case, the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`, the case's params, and
///   `build_create_key_derivation_payload`, bound through `expect`;
///   `build_derive_key_params` with no override;
///   `build_derive_key_payload` with the key material override set to a
///   borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; a copy of the
///   key's `expose()` bytes is read before `drop` of the key material's
///   secret, and the key's `expose()` after the drop equals that copy by
///   `assert_eq!`, so a key that shares storage with the key material
///   reads as zeros.
/// Boundary: the crate-internal path `create_key_derivation` through the
///   selected concrete's `try_new`, then the returned adapter's
///   `derive_key`, with the key material's `Secret` drop running for real.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_concrete_returns_a_key_that_outlives_the_key_material() {
    // Arrange
    let mock_selection = build_create_key_derivation_params(Default::default());
    for selection in DECLARED_KDF_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let key_material = build_secret(reference_key_material_bytes(), Default::default());
        let deps = build_create_key_derivation_deps(Default::default());
        let payload = build_create_key_derivation_payload(Default::default());
        let success = create_key_derivation(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
        let params = build_derive_key_params(Default::default());
        let payload = build_derive_key_payload(DeriveKeyPayloadOverrides {
            key_material: Some(&key_material),
            ..Default::default()
        });

        // Act
        let result = adapter.derive_key(params, payload);

        // Assert
        let success = result.expect("the derivation returns a key");
        let derived = success.key.expose().clone();
        drop(key_material);
        assert_eq!(success.key.expose().as_slice(), derived.as_slice());
    }
}
