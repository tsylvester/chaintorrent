#![allow(clippy::expect_used)]

use super::provides::{
    ConsumeKeyDerivationParams, ConsumeKeyDerivationPayload, CreateKeyDerivationDepsOverrides,
    CreateKeyDerivationParamsOverrides, DeriveKeyErrorReturn, DeriveKeyParamsOverrides,
    DeriveKeyPayloadOverrides, DeriveKeyReturn, DeriveKeySuccessReturn, IKeyDerivationAdapter,
    IKeyDerivationConsumer, KDF_CONCRETES, KdfConcrete, KdfConcreteOverrides,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload,
    build_derive_key_params, build_derive_key_payload, build_kdf_concrete, create_key_derivation,
    reference_key_material_bytes,
};
use crate::admit_kdf_selection::provides::{
    AdmitKdfSelectionDeps, AdmitKdfSelectionParamsOverrides, admit_kdf_selection,
    build_admit_kdf_selection_params,
};
use crate::lookup_kdf_declaration::provides::lookup_kdf_declaration;
use domain::build_secret;

/// Contract: `create_key_derivation` called with a selection
///   `admit_kdf_selection` returned and a consumer that derives with a
///   `params.length` → `Ok(CreateKeyDerivationSuccessReturn { output })`
///   whose `output` observes `Ok(DeriveKeySuccessReturn { key })` with the
///   key's byte length equal to `params.length`.
/// Arrange: `Consumer` with `type Output = Vec<DeriveKeyReturn>;` and
///   `consume_key_derivation` calling `payload.adapter.derive_key` once
///   for each length of `REFERENCE_LENGTH`, `REFERENCE_PREFIX_LENGTH`,
///   and zero, with `build_derive_key_params` at the length override and
///   `build_derive_key_payload` at no override, collecting the three
///   returns in order; the lengths differ, so a concrete that returns the
///   reference length whatever is requested fails; per case, the case's
///   params and the deps the Cases bullet states.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`;
///   `success.output.len()` equals `3` and each return's `Ok` arm, bound
///   through `expect`, has `key.expose().len()` equal to that position's
///   length override by `assert_eq!`.
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through `deps.lookup` and the selected
///   concrete's `try_new`, then `consume_key_derivation` calling
///   `derive_key` on the adapter it receives.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_concrete_derives_a_key_of_the_requested_length() {
    // Arrange
    struct Consumer;

    impl IKeyDerivationConsumer for Consumer {
        type Output = Vec<DeriveKeyReturn>;

        fn consume_key_derivation<A: IKeyDerivationAdapter>(
            &self,
            _params: ConsumeKeyDerivationParams,
            payload: ConsumeKeyDerivationPayload<A>,
        ) -> Self::Output {
            [REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, 0]
                .into_iter()
                .map(|length| {
                    payload.adapter.derive_key(
                        build_derive_key_params(DeriveKeyParamsOverrides {
                            length: Some(length),
                            ..Default::default()
                        }),
                        build_derive_key_payload(Default::default()),
                    )
                })
                .collect()
        }
    }

    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
        .chain(core::iter::once(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )))
    {
        let selection = admit_kdf_selection(
            &AdmitKdfSelectionDeps,
            build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
                concrete: Some(concrete),
            }),
            serde_json::json!({}),
        )
        .expect("an empty document supplies no setting")
        .selection;
        let deps = build_create_key_derivation_deps(
            Consumer,
            CreateKeyDerivationDepsOverrides {
                lookup: Some(lookup_kdf_declaration),
            },
        );
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            selection: Some(selection),
        });

        // Act
        let success = create_key_derivation(
            &deps,
            params,
            build_create_key_derivation_payload(Default::default()),
        )
        .expect("an admitted selection returns the adapter");

        // Assert
        assert_eq!(success.output.len(), 3);
        for (result, length) in
            success
                .output
                .iter()
                .zip([REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, 0])
        {
            let derived = result.as_ref().expect("the requested length derives a key");
            assert_eq!(derived.key.expose().len(), length);
        }
    }
}

/// Contract: `create_key_derivation` called with the mock selection
///   `admit_kdf_selection` returned under the failure mode
///   `KeyMaterialLengthUnrepresentable` →
///   `Ok(CreateKeyDerivationSuccessReturn { output })` whose `output` is
///   `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))`
///   with `length` the key material's length, `32`; the absorbed entry
///   `kdf/admit_kdf_selection`
///   `admit_kdf_selection_carries_the_selected_mock_member_through_the_crate_surface`,
///   in this node's terms: the failure mode the admitted selection
///   carries is the failure mode of the concrete the factory constructs.
/// Arrange: `Consumer` with `type Output = DeriveKeyReturn;` and
///   `consume_key_derivation` calling `payload.adapter.derive_key` with
///   `build_derive_key_params` and `build_derive_key_payload` at no
///   override, whose key material is the reference key material,
///   returning its return; the case is the mock selection alone, which
///   `KDF_CONCRETES` cannot reach, the `selection` of
///   `admit_kdf_selection` with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the failure mode differs from the `NoFailure` member the mock
///   constant names.
/// Act:     `create_key_derivation(&deps, params, payload)`.
/// Assert:  `assert_eq!(result.ok().map(|success| success.output.err()),
///   Some(Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))))`,
///   the length the literal `32`, the length the reference key material
///   is stated to have.
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through `deps.lookup` and the mock
///   concrete's `try_new` with the failure mode the selection carries,
///   then `consume_key_derivation` calling `derive_key` on the adapter it
///   receives.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_its_own_error_when_the_key_material_length_cannot_be_framed() {
    // Arrange
    struct Consumer;

    impl IKeyDerivationConsumer for Consumer {
        type Output = DeriveKeyReturn;

        fn consume_key_derivation<A: IKeyDerivationAdapter>(
            &self,
            _params: ConsumeKeyDerivationParams,
            payload: ConsumeKeyDerivationPayload<A>,
        ) -> Self::Output {
            payload.adapter.derive_key(
                build_derive_key_params(Default::default()),
                build_derive_key_payload(Default::default()),
            )
        }
    }

    let selection = admit_kdf_selection(
        &AdmitKdfSelectionDeps,
        build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
            concrete: Some(KdfConcrete::Mock(
                MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
            )),
        }),
        serde_json::json!({}),
    )
    .expect("an empty document supplies no setting")
    .selection;
    let deps = build_create_key_derivation_deps(
        Consumer,
        CreateKeyDerivationDepsOverrides {
            lookup: Some(lookup_kdf_declaration),
        },
    );
    let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        selection: Some(selection),
    });

    // Act
    let result = create_key_derivation(
        &deps,
        params,
        build_create_key_derivation_payload(Default::default()),
    );

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output.err()),
        Some(Some(DeriveKeyErrorReturn::Mock(
            MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                length: 32
            }
        )))
    );
}

/// Contract: the absorbed entry `kdf/blake3_keyed`
///   `derive_key_leaves_the_borrowed_key_material_intact`, in this node's
///   terms: `create_key_derivation` is called with a consumer whose
///   `consume_key_derivation` calls `derive_key` on the adapter it
///   receives with a living `Secret<Vec<u8>>` borrowed in the derive
///   payload → after `create_key_derivation` returns, the secret's
///   `expose()` equals the bytes it held before the call.
/// Arrange: `build_secret` with the value override
///   `reference_key_material_bytes()`, whose bytes are not all zero, so a
///   zeroized secret differs from an intact one; `Consumer<'a>` holding
///   `key_material: &'a Secret<Vec<u8>>`, with
///   `type Output = DeriveKeyReturn;` and `consume_key_derivation`
///   calling `payload.adapter.derive_key` with `build_derive_key_params`
///   at no override and `build_derive_key_payload` with the key material
///   override set to a borrow of `self.key_material`; per case, deps
///   carrying that consumer.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the secret's
///   `expose()` equals `reference_key_material_bytes()` by `assert_eq!`.
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through `deps.lookup` and the selected
///   concrete's `try_new`, then `consume_key_derivation` calling
///   `derive_key` on the adapter it receives, with `Secret::expose`
///   reading the key material.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real, so
///   this block does not prove the zeroization of a vendor's hasher and
///   reader themselves, which no caller can observe.
#[test]
fn every_concrete_leaves_the_borrowed_key_material_intact() {
    // Arrange
    struct Consumer<'a> {
        key_material: &'a domain::Secret<Vec<u8>>,
    }

    impl IKeyDerivationConsumer for Consumer<'_> {
        type Output = DeriveKeyReturn;

        fn consume_key_derivation<A: IKeyDerivationAdapter>(
            &self,
            _params: ConsumeKeyDerivationParams,
            payload: ConsumeKeyDerivationPayload<A>,
        ) -> Self::Output {
            payload.adapter.derive_key(
                build_derive_key_params(Default::default()),
                build_derive_key_payload(DeriveKeyPayloadOverrides {
                    key_material: Some(self.key_material),
                    ..Default::default()
                }),
            )
        }
    }

    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
        .chain(core::iter::once(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )))
    {
        let key_material = build_secret(reference_key_material_bytes(), Default::default());
        let selection = admit_kdf_selection(
            &AdmitKdfSelectionDeps,
            build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
                concrete: Some(concrete),
            }),
            serde_json::json!({}),
        )
        .expect("an empty document supplies no setting")
        .selection;
        let deps = build_create_key_derivation_deps(
            Consumer {
                key_material: &key_material,
            },
            CreateKeyDerivationDepsOverrides {
                lookup: Some(lookup_kdf_declaration),
            },
        );
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            selection: Some(selection),
        });

        // Act
        let _success = create_key_derivation(
            &deps,
            params,
            build_create_key_derivation_payload(Default::default()),
        )
        .expect("an admitted selection returns the adapter");

        // Assert
        assert_eq!(
            key_material.expose().as_slice(),
            reference_key_material_bytes().as_slice()
        );
    }
}

/// Contract: the absorbed entry `kdf/blake3_keyed`
///   `derive_key_returns_a_key_that_outlives_the_key_material`, in this
///   node's terms: `create_key_derivation` returns `Ok` whose consumer's
///   `derive_key` succeeded, and the key material's secret is then
///   dropped, its drop zeroizing its value → the returned key in
///   `success.output`'s `expose()` still equals the bytes it held before
///   the drop.
/// Arrange: `build_secret` with the value override
///   `reference_key_material_bytes()`, so the dropped secret holds
///   non-zero bytes that its drop overwrites; `Consumer<'a>` holding
///   `key_material: &'a Secret<Vec<u8>>`, with
///   `type Output = DeriveKeySuccessReturn;` and
///   `consume_key_derivation` calling `payload.adapter.derive_key` with
///   `build_derive_key_params` at no override and
///   `build_derive_key_payload` with the key material override set to a
///   borrow of `self.key_material`, returning the `DeriveKeySuccessReturn`
///   through `expect`; per case, deps carrying that consumer.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; a copy of
///   `success.output.key.expose()` is read before `drop` of the key
///   material's secret, and `success.output.key.expose()` after the drop
///   equals that copy by `assert_eq!`, so a key that shares storage with
///   the key material reads as zeros.
/// Boundary: the crate-internal path `admit_kdf_selection`, then
///   `create_key_derivation` through `deps.lookup` and the selected
///   concrete's `try_new`, then `consume_key_derivation` calling
///   `derive_key` on the adapter it receives, with the key material's
///   `Secret` drop running for real.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_concrete_returns_a_key_that_outlives_the_key_material() {
    // Arrange
    struct Consumer<'a> {
        key_material: &'a domain::Secret<Vec<u8>>,
    }

    impl IKeyDerivationConsumer for Consumer<'_> {
        type Output = DeriveKeySuccessReturn;

        fn consume_key_derivation<A: IKeyDerivationAdapter>(
            &self,
            _params: ConsumeKeyDerivationParams,
            payload: ConsumeKeyDerivationPayload<A>,
        ) -> Self::Output {
            payload
                .adapter
                .derive_key(
                    build_derive_key_params(Default::default()),
                    build_derive_key_payload(DeriveKeyPayloadOverrides {
                        key_material: Some(self.key_material),
                        ..Default::default()
                    }),
                )
                .expect("the derivation returns a key")
        }
    }

    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
        .chain(core::iter::once(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )))
    {
        let key_material = build_secret(reference_key_material_bytes(), Default::default());
        let selection = admit_kdf_selection(
            &AdmitKdfSelectionDeps,
            build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
                concrete: Some(concrete),
            }),
            serde_json::json!({}),
        )
        .expect("an empty document supplies no setting")
        .selection;
        let deps = build_create_key_derivation_deps(
            Consumer {
                key_material: &key_material,
            },
            CreateKeyDerivationDepsOverrides {
                lookup: Some(lookup_kdf_declaration),
            },
        );
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            selection: Some(selection),
        });

        // Act
        let success = create_key_derivation(
            &deps,
            params,
            build_create_key_derivation_payload(Default::default()),
        )
        .expect("an admitted selection returns the adapter");

        // Assert
        let derived = success.output.key.expose().clone();
        drop(key_material);
        assert_eq!(success.output.key.expose().as_slice(), derived.as_slice());
    }
}
