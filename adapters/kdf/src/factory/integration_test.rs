#![allow(clippy::expect_used)]

use super::provides::{
    CreateKeyDerivationParamsOverrides, DeriveKeyErrorReturn, DeriveKeyParamsOverrides,
    DeriveKeyPayloadOverrides, IKeyDerivationAdapter, KDF_CONCRETES, KdfConcrete,
    KdfConcreteOverrides, MockIKeyDerivationAdapterDeriveKeyErrorReturn,
    MockIKeyDerivationAdapterFailureMode, REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH,
    build_create_key_derivation_deps, build_create_key_derivation_params,
    build_create_key_derivation_payload, build_derive_key_params, build_derive_key_payload,
    build_kdf_concrete, create_key_derivation, reference_key_material_bytes,
};
use crate::admit_kdf_selection::provides::{
    AdmitKdfSelectionParamsOverrides, AdmitKdfSelectionPayloadCorruptions, admit_kdf_selection,
    build_admit_kdf_selection_deps, build_admit_kdf_selection_params,
    invalidate_admit_kdf_selection_payload,
};
use domain::build_secret;

/// Contract: the adapter `create_key_derivation` returns for a selection
///   `admit_kdf_selection` returned derives with a `params.length` →
///   `Ok(DeriveKeySuccessReturn { key })` with the key's byte length equal
///   to `params.length`.
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
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through the selected concrete's `try_new`,
///   then the returned adapter's `derive_key`.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_concrete_derives_a_key_of_the_requested_length() {
    // Arrange
    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
        .chain(core::iter::once(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )))
    {
        let admitted = admit_kdf_selection(
            &build_admit_kdf_selection_deps(Default::default()),
            build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
                concrete: Some(concrete),
            }),
            invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions::default()),
        )
        .expect("an empty document supplies no setting");
        let deps = build_create_key_derivation_deps(Default::default());
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            selection: Some(admitted.selection),
        });
        let payload = build_create_key_derivation_payload(Default::default());
        let success = create_key_derivation(&deps, params, payload)
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
///   selection `admit_kdf_selection` returned under the failure mode
///   `KeyMaterialLengthUnrepresentable` derives →
///   `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))`
///   with `length` the key material's length, `32`; the absorbed entry
///   `kdf/admit_kdf_selection`
///   `admit_kdf_selection_carries_the_selected_mock_member_through_the_crate_surface`,
///   in this node's terms: the failure mode the admitted selection carries
///   is the failure mode of the concrete the factory constructs.
/// Arrange: the case is the mock selection alone, which `KDF_CONCRETES`
///   cannot reach; the selection is the `selection` of
///   `admit_kdf_selection` with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the failure mode differs from the `NoFailure` member the mock
///   constant names; the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`, `build_create_key_derivation_params`
///   with that selection, and `build_create_key_derivation_payload`, bound
///   through `expect`; `build_derive_key_params` and
///   `build_derive_key_payload` with no override, whose key material is
///   the reference key material.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))`
///   by `assert_eq!`, the length the literal `32`, the length the
///   reference key material is stated to have.
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through the mock concrete's `try_new` with
///   the failure mode the selection carries, then the returned adapter's
///   `derive_key`.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_its_own_error_when_the_key_material_length_cannot_be_framed() {
    // Arrange
    let admitted = admit_kdf_selection(
        &build_admit_kdf_selection_deps(Default::default()),
        build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
            concrete: Some(KdfConcrete::Mock(
                MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
            )),
        }),
        invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions::default()),
    )
    .expect("an empty document supplies no setting");
    let deps = build_create_key_derivation_deps(Default::default());
    let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        selection: Some(admitted.selection),
    });
    let payload = build_create_key_derivation_payload(Default::default());
    let success =
        create_key_derivation(&deps, params, payload).expect("the mock selection is constructed");
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
                length: 32
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
/// Arrange: `build_secret` with the value override
///   `reference_key_material_bytes()`, whose bytes are not all zero, so a
///   zeroized secret differs from an intact one; for each case, the
///   adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`, the case's params, and
///   `build_create_key_derivation_payload`, bound through `expect`;
///   `build_derive_key_params` with no override;
///   `build_derive_key_payload` with the key material override set to a
///   borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the secret's
///   `expose()` equals `reference_key_material_bytes()` by `assert_eq!`.
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through the selected concrete's `try_new`,
///   then the returned adapter's `derive_key`, with `Secret::expose`
///   reading the key material.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real, so
///   this block does not prove the zeroization of a vendor's hasher and
///   reader themselves, which no caller can observe.
#[test]
fn every_concrete_leaves_the_borrowed_key_material_intact() {
    // Arrange
    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
        .chain(core::iter::once(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )))
    {
        let key_material = build_secret(reference_key_material_bytes(), Default::default());
        let admitted = admit_kdf_selection(
            &build_admit_kdf_selection_deps(Default::default()),
            build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
                concrete: Some(concrete),
            }),
            invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions::default()),
        )
        .expect("an empty document supplies no setting");
        let deps = build_create_key_derivation_deps(Default::default());
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            selection: Some(admitted.selection),
        });
        let payload = build_create_key_derivation_payload(Default::default());
        let success = create_key_derivation(&deps, params, payload)
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
/// Arrange: `build_secret` with the value override
///   `reference_key_material_bytes()`, so the dropped secret holds
///   non-zero bytes that its drop overwrites; for each case, the adapter
///   from `create_key_derivation` over `build_create_key_derivation_deps`,
///   the case's params, and `build_create_key_derivation_payload`, bound
///   through `expect`; `build_derive_key_params` with no override;
///   `build_derive_key_payload` with the key material override set to a
///   borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; a copy of the
///   key's `expose()` bytes is read before `drop` of the key material's
///   secret, and the key's `expose()` after the drop equals that copy by
///   `assert_eq!`, so a key that shares storage with the key material
///   reads as zeros.
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through the selected concrete's `try_new`,
///   then the returned adapter's `derive_key`, with the key material's
///   `Secret` drop running for real.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_concrete_returns_a_key_that_outlives_the_key_material() {
    // Arrange
    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
        .chain(core::iter::once(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )))
    {
        let key_material = build_secret(reference_key_material_bytes(), Default::default());
        let admitted = admit_kdf_selection(
            &build_admit_kdf_selection_deps(Default::default()),
            build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
                concrete: Some(concrete),
            }),
            invalidate_admit_kdf_selection_payload(AdmitKdfSelectionPayloadCorruptions::default()),
        )
        .expect("an empty document supplies no setting");
        let deps = build_create_key_derivation_deps(Default::default());
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            selection: Some(admitted.selection),
        });
        let payload = build_create_key_derivation_payload(Default::default());
        let success = create_key_derivation(&deps, params, payload)
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
