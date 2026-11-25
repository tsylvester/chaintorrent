#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::build_secret;
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, MockIEncodingContract,
    build_canonical_fields, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_mock_i_encoding_consumer, create_encoding,
};
use kdf::{
    AdmitKdfSelectionParamsOverrides, AdmitKdfSelectionPayloadCorruptions,
    CreateKeyDerivationParamsOverrides, DerivationPurpose, DeriveKeyErrorReturn,
    DeriveKeyParamsOverrides, DeriveKeyPayloadOverrides, IKeyDerivationAdapter, KDF_CONCRETES,
    KdfConcrete, KdfConcreteOverrides, KdfIdentifier, LookupKdfDeclarationParamsOverrides,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, admit_kdf_selection, build_admit_kdf_selection_deps,
    build_admit_kdf_selection_params, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload,
    build_derive_key_params, build_derive_key_payload, build_kdf_concrete,
    build_lookup_kdf_declaration_deps, build_lookup_kdf_declaration_params,
    build_lookup_kdf_declaration_payload, create_key_derivation, expected_derived_key,
    invalidate_admit_kdf_selection_payload, lookup_kdf_declaration, reference_context,
    reference_key_material_bytes,
};

/// Contract: the adapter `create_key_derivation` returns for a selection
///   `admit_kdf_selection` returned derives with a `params.length` →
///   `Ok(DeriveKeySuccessReturn { key })` with the key's byte length equal
///   to `params.length`.
/// Arrange: for each case, the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`,
///   `build_create_key_derivation_params` overriding `selection` with the
///   case's admitted selection, and `build_create_key_derivation_payload`,
///   bound through `expect`; for each length of `REFERENCE_LENGTH`,
///   `REFERENCE_PREFIX_LENGTH`, and zero, `build_derive_key_params` with
///   the length override and `build_derive_key_payload` with no override;
///   the lengths differ, so a concrete that returns the reference length
///   whatever is requested fails.
/// Act:     `adapter.derive_key(params, payload)` for the case and the
///   length.
/// Assert:  the success arm is bound through `expect`; the key's
///   `expose().len()` equals the length override by `assert_eq!`.
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the returned adapter's `derive_key`.
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
///   `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))`.
/// Arrange: the selection is the `selection` of `admit_kdf_selection`
///   with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`;
///   the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`, `build_create_key_derivation_params`
///   with that selection, and `build_create_key_derivation_payload`, bound
///   through `expect`; `build_derive_key_params` and
///   `build_derive_key_payload` with no override.
/// Act:     `adapter.derive_key(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))`
///   by `assert_eq!`, the length a literal.
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the returned adapter's `derive_key`.
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

/// Contract: `derive_key` is called on the adapter `create_key_derivation`
///   returns with a living `Secret<Vec<u8>>` borrowed in the payload →
///   after the call the secret's `expose()` equals the bytes it held
///   before the call.
/// Arrange: `build_secret` with the value override
///   `reference_key_material_bytes()`, whose bytes are not all zero, so a
///   zeroized secret differs from an intact one; for each case, the
///   adapter as in the length block; `build_derive_key_params` with no
///   override; `build_derive_key_payload` with the key material override
///   set to a borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the secret's
///   `expose()` equals `reference_key_material_bytes()` by `assert_eq!`.
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the returned adapter's `derive_key`,
///   with `Secret::expose` reading the key material.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
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

/// Contract: `derive_key` returns `Ok` on the adapter
///   `create_key_derivation` returns and the key material's secret is then
///   dropped, its drop zeroizing its value → the returned key's `expose()`
///   still equals the bytes it held before the drop.
/// Arrange: `build_secret` with the value override
///   `reference_key_material_bytes()`, so the dropped secret holds
///   non-zero bytes that its drop overwrites; for each case, the adapter
///   as in the length block; `build_derive_key_params` with no override;
///   `build_derive_key_payload` with the key material override set to a
///   borrow of that secret.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; a copy of the
///   key's `expose()` bytes is read before `drop` of the key material's
///   secret, and the key's `expose()` after the drop equals that copy by
///   `assert_eq!`.
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the returned adapter's `derive_key`,
///   with the key material's `Secret` drop running for real.
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

/// Contract: `create_key_derivation` is called with the selection
///   `admit_kdf_selection` returned for a member whose declared identifier
///   is `KdfIdentifier::Blake3KeyedV1`, and the adapter derives with
///   `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the
///   reference key material and the reference context → the key equals
///   `expected_derived_key` over the wrapping-key purpose, the reference
///   context, and `REFERENCE_LENGTH`.
/// Arrange: for each case, the declaration read through
///   `lookup_kdf_declaration` over `build_lookup_kdf_declaration_deps`,
///   `build_lookup_kdf_declaration_params` with the `concrete` override
///   the case, and `build_lookup_kdf_declaration_payload`, bound through
///   `let Ok(success) = …;` since its error arm is uninhabited, selecting
///   the cases for which
///   `matches!(success.declaration.identifier, KdfIdentifier::Blake3KeyedV1)`
///   holds, and asserting by `assert!` that the selection is not empty;
///   the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`,
///   `build_create_key_derivation_params` overriding `selection` with the
///   case's admitted selection, and `build_create_key_derivation_payload`,
///   bound through `expect`; `build_derive_key_params` overriding the
///   purpose with `DerivationPurpose::WrappingKey` and the length with
///   `REFERENCE_LENGTH`; `build_derive_key_payload` with no override,
///   whose key material and context are the reference key material and
///   the reference context, so the oracle's inputs are the arrangement's
///   inputs and its output is not.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the key's
///   `expose()` equals `expected_derived_key` over
///   `DerivationPurpose::WrappingKey`, `reference_context()`, and
///   `REFERENCE_LENGTH` by `assert_eq!`, the expectation computed by the
///   independent oracle and not read from the arrangement.
/// Boundary: the public calls `lookup_kdf_declaration`,
///   `admit_kdf_selection`, `create_key_derivation`, and the returned
///   adapter's `derive_key`; the route `admit_kdf_selection`,
///   `create_key_derivation`, the selected concrete's constructor, and
///   its `derive_key`.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_declared_concrete_providing_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_reference_inputs()
 {
    // Arrange
    let mut selected = 0usize;
    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
    {
        let Ok(success) = lookup_kdf_declaration(
            &build_lookup_kdf_declaration_deps(Default::default()),
            build_lookup_kdf_declaration_params(LookupKdfDeclarationParamsOverrides {
                concrete: Some(concrete),
            }),
            build_lookup_kdf_declaration_payload(Default::default()),
        );
        if !matches!(success.declaration.identifier, KdfIdentifier::Blake3KeyedV1) {
            continue;
        }
        selected += 1;
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
        let params = build_derive_key_params(DeriveKeyParamsOverrides {
            purpose: Some(DerivationPurpose::WrappingKey),
            length: Some(REFERENCE_LENGTH),
        });
        let payload = build_derive_key_payload(Default::default());

        // Act
        let result = adapter.derive_key(params, payload);

        // Assert
        let success = result.expect("the derivation returns a key");
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
    assert!(selected > 0, "at least one case was selected");
}

/// Contract: `create_key_derivation` is called with the selection
///   `admit_kdf_selection` returned for a member whose declared identifier
///   is `KdfIdentifier::Blake3KeyedV1`, and the adapter derives with
///   `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the
///   reference key material and the bytes `create_encoding` returns for a
///   value of the family's mock description → the key equals
///   `expected_derived_key` over the wrapping-key purpose, those bytes,
///   and `REFERENCE_LENGTH`.
/// Arrange: for each case and each entry of `DECLARED_ENCODING_SELECTIONS`,
///   the context bytes obtained from `create_encoding` over
///   `build_create_encoding_deps` taking `build_mock_i_encoding_consumer`
///   over `&MockIEncodingContract`, `build_canonical_fields` at its
///   defaults, and no bytes, `build_create_encoding_params` overriding
///   `selection` with the encoding entry's selection, and
///   `build_create_encoding_payload`, the `encoded` of the output bound
///   through `expect`; the case selected and the adapter built as in the
///   block above; `build_derive_key_params` as in the block above;
///   `build_derive_key_payload` with the context override set to a borrow
///   of those bytes, whose length and content differ from the reference
///   context's, so a derivation that frames the context by the reference
///   context's shape fails.
/// Act:     `adapter.derive_key(params, payload)` for the case and the
///   encoding entry.
/// Assert:  the success arm is bound through `expect`; the key's
///   `expose()` equals `expected_derived_key` over
///   `DerivationPurpose::WrappingKey`, the context bytes, and
///   `REFERENCE_LENGTH` by `assert_eq!`, the expectation computed by the
///   independent oracle and not read from the arrangement.
/// Boundary: the public calls `create_encoding`, `lookup_kdf_declaration`,
///   `admit_kdf_selection`, `create_key_derivation`, and the returned
///   adapter's `derive_key`; the route `create_encoding`, the selected
///   encoding concrete's `encode`, then `admit_kdf_selection`,
///   `create_key_derivation`, the selected concrete's constructor, and
///   its `derive_key`.
/// Mocked:  the encoding consumer, replaced by the official
///   `MockIEncodingConsumer`, and the description it carries, replaced by
///   the official `MockIEncodingContract`, so the test does not prove
///   what a real consumer or a real description does with the concrete;
///   `domain`, `blake3`, and `zeroize` run for real.
#[test]
fn every_declared_concrete_providing_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes()
 {
    // Arrange
    let mut selected = 0usize;
    for concrete in KDF_CONCRETES
        .iter()
        .map(|concrete| build_kdf_concrete(*concrete, KdfConcreteOverrides))
    {
        let Ok(success) = lookup_kdf_declaration(
            &build_lookup_kdf_declaration_deps(Default::default()),
            build_lookup_kdf_declaration_params(LookupKdfDeclarationParamsOverrides {
                concrete: Some(concrete),
            }),
            build_lookup_kdf_declaration_payload(Default::default()),
        );
        if !matches!(success.declaration.identifier, KdfIdentifier::Blake3KeyedV1) {
            continue;
        }
        selected += 1;
        for encoding_selection in DECLARED_ENCODING_SELECTIONS {
            let consumer = build_mock_i_encoding_consumer(
                &MockIEncodingContract,
                build_canonical_fields(Default::default()),
                Vec::new(),
            );
            let encoding_deps = build_create_encoding_deps(consumer, Default::default());
            let encoding_params = build_create_encoding_params(CreateEncodingParamsOverrides {
                selection: Some(encoding_selection.selection),
            });
            let encoding_payload = build_create_encoding_payload(Default::default());
            let context = create_encoding(&encoding_deps, encoding_params, encoding_payload)
                .expect("an admitted encoding selection returns the consumer's output")
                .output
                .encoded
                .expect("the canonical fields encode under the mock description")
                .bytes;
            let admitted = admit_kdf_selection(
                &build_admit_kdf_selection_deps(Default::default()),
                build_admit_kdf_selection_params(AdmitKdfSelectionParamsOverrides {
                    concrete: Some(concrete),
                }),
                invalidate_admit_kdf_selection_payload(
                    AdmitKdfSelectionPayloadCorruptions::default(),
                ),
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
            let params = build_derive_key_params(DeriveKeyParamsOverrides {
                purpose: Some(DerivationPurpose::WrappingKey),
                length: Some(REFERENCE_LENGTH),
            });
            let payload = build_derive_key_payload(DeriveKeyPayloadOverrides {
                context: Some(context.as_slice()),
                ..Default::default()
            });

            // Act
            let result = adapter.derive_key(params, payload);

            // Assert
            let success = result.expect("the derivation returns a key");
            assert_eq!(
                success.key.expose().as_slice(),
                expected_derived_key(
                    DerivationPurpose::WrappingKey,
                    context.as_slice(),
                    REFERENCE_LENGTH,
                )
                .as_slice()
            );
        }
    }
    assert!(selected > 0, "at least one case was selected");
}
