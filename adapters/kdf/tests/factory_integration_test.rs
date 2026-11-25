#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::build_secret;
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, MockIEncodingContract,
    build_canonical_fields, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_mock_i_encoding_consumer, create_encoding,
};
use kdf::{
    AdmitKdfSelectionDeps, AdmitKdfSelectionParamsOverrides, ConsumeKeyDerivationParams,
    ConsumeKeyDerivationPayload, CreateKeyDerivationDepsOverrides,
    CreateKeyDerivationParamsOverrides, DerivationPurpose, DeriveKeyErrorReturn,
    DeriveKeyParamsOverrides, DeriveKeyPayloadOverrides, DeriveKeyReturn, DeriveKeySuccessReturn,
    IKeyDerivationAdapter, IKeyDerivationConsumer, KDF_CONCRETES, KdfConcrete,
    KdfConcreteOverrides, KdfIdentifier, LookupKdfDeclarationParamsOverrides,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    REFERENCE_LENGTH, REFERENCE_PREFIX_LENGTH, admit_kdf_selection,
    build_admit_kdf_selection_params, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload,
    build_derive_key_params, build_derive_key_payload, build_kdf_concrete,
    build_lookup_kdf_declaration_deps, build_lookup_kdf_declaration_params,
    build_lookup_kdf_declaration_payload, create_key_derivation, expected_derived_key,
    lookup_kdf_declaration, reference_context, reference_key_material_bytes,
};

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
///   reference length whatever is requested fails; per case,
///   `build_create_key_derivation_params` overriding `selection` with the
///   case's admitted selection.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`;
///   `success.output.len()` equals `3` and each return's `Ok` arm, bound
///   through `expect`, has `key.expose().len()` equal to that position's
///   length override by `assert_eq!`.
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the consumer's `derive_key` on the
///   adapter `consume_key_derivation` receives.
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
///   `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))`.
/// Arrange: `Consumer` with `type Output = DeriveKeyReturn;` and
///   `consume_key_derivation` calling `payload.adapter.derive_key` with
///   `build_derive_key_params` and `build_derive_key_payload` at no
///   override, returning its return; the selection is the `selection` of
///   `admit_kdf_selection` with the `concrete` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`.
/// Act:     `create_key_derivation(&deps, params, payload)`.
/// Assert:  `assert_eq!(result.ok().map(|success| success.output.err()),
///   Some(Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))))`,
///   the length a literal.
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the consumer's `derive_key` on the
///   adapter `consume_key_derivation` receives.
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

/// Contract: `create_key_derivation` is called with a consumer whose
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
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the consumer's `derive_key` on the
///   adapter `consume_key_derivation` receives, with `Secret::expose`
///   reading the key material.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
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

/// Contract: `create_key_derivation` returns `Ok` whose consumer's
///   `derive_key` succeeded, and the key material's secret is then
///   dropped, its drop zeroizing its value → the returned key in
///   `success.output`'s `expose()` still equals the bytes it held before
///   the drop.
/// Arrange: `build_secret` with the value override
///   `reference_key_material_bytes()`, so the dropped secret holds
///   non-zero bytes that its drop overwrites; `Consumer<'a>` holding
///   `key_material: &'a Secret<Vec<u8>>`, with
///   `type Output = DeriveKeySuccessReturn;` and `consume_key_derivation`
///   calling `payload.adapter.derive_key` with `build_derive_key_params`
///   at no override and `build_derive_key_payload` with the key material
///   override set to a borrow of `self.key_material`, returning the
///   `DeriveKeySuccessReturn` through `expect`; per case, deps carrying
///   that consumer.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; a copy of
///   `success.output.key.expose()` is read before `drop` of the key
///   material's secret, and `success.output.key.expose()` after the drop
///   equals that copy by `assert_eq!`.
/// Boundary: the public calls `admit_kdf_selection`,
///   `create_key_derivation`, and the consumer's `derive_key` on the
///   adapter `consume_key_derivation` receives, with the key material's
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

/// Contract: `create_key_derivation` is called with the selection
///   `admit_kdf_selection` returned for a member whose declared identifier
///   is `KdfIdentifier::Blake3KeyedV1`, and a consumer that derives with
///   `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the
///   reference key material and the reference context →
///   `Ok(CreateKeyDerivationSuccessReturn { output })` whose `output`
///   observes a key equal to `expected_derived_key` over the wrapping-key
///   purpose, the reference context, and `REFERENCE_LENGTH`.
/// Arrange: for each case, the declaration read through
///   `lookup_kdf_declaration` over `build_lookup_kdf_declaration_deps`,
///   `build_lookup_kdf_declaration_params` with the `concrete` override
///   the case, and `build_lookup_kdf_declaration_payload`, bound through
///   `let Ok(success) = …;` since its error arm is uninhabited, selecting
///   the cases for which
///   `matches!(success.declaration.identifier, KdfIdentifier::Blake3KeyedV1)`
///   holds, and asserting by `assert!` that the selection is not empty;
///   `Consumer` with `type Output = DeriveKeyReturn;` and
///   `consume_key_derivation` calling `payload.adapter.derive_key` with
///   `build_derive_key_params` overriding the purpose with
///   `DerivationPurpose::WrappingKey` and the length with
///   `REFERENCE_LENGTH` and `build_derive_key_payload` at no override,
///   whose key material and context are the reference key material and
///   the reference context, so the oracle's inputs are the arrangement's
///   inputs and its output is not; per case,
///   `build_create_key_derivation_params` overriding `selection` with the
///   case's admitted selection and deps carrying that consumer.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the `Ok` arm of
///   `success.output`, bound through `expect`, has `key.expose()` equal
///   to `expected_derived_key` over `DerivationPurpose::WrappingKey`,
///   `reference_context()`, and `REFERENCE_LENGTH` by `assert_eq!`, the
///   expectation computed by the independent oracle and not read from the
///   arrangement.
/// Boundary: the public calls `lookup_kdf_declaration`,
///   `admit_kdf_selection`, `create_key_derivation`, and the consumer's
///   `derive_key` on the adapter `consume_key_derivation` receives; the
///   route `admit_kdf_selection`, `create_key_derivation` over the real
///   `lookup_kdf_declaration`, the selected concrete's constructor,
///   `consume_key_derivation`, and `derive_key`.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_declared_concrete_providing_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_reference_inputs()
 {
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
                build_derive_key_params(DeriveKeyParamsOverrides {
                    purpose: Some(DerivationPurpose::WrappingKey),
                    length: Some(REFERENCE_LENGTH),
                }),
                build_derive_key_payload(Default::default()),
            )
        }
    }

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
        let derived = success.output.expect("the derivation returns a key");
        assert_eq!(
            derived.key.expose().as_slice(),
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
///   is `KdfIdentifier::Blake3KeyedV1`, and a consumer that derives with
///   `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the
///   reference key material and the bytes `create_encoding` returns for a
///   value of the family's mock description →
///   `Ok(CreateKeyDerivationSuccessReturn { output })` whose `output`
///   observes a key equal to `expected_derived_key` over the wrapping-key
///   purpose, those bytes, and `REFERENCE_LENGTH`.
/// Arrange: for each case and each entry of
///   `DECLARED_ENCODING_SELECTIONS`, the context bytes obtained from
///   `create_encoding` over `build_create_encoding_deps` taking
///   `build_mock_i_encoding_consumer` over `&MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes,
///   `build_create_encoding_params` overriding `selection` with the
///   encoding entry's selection, and `build_create_encoding_payload`, the
///   `encoded` of the output bound through `expect`; `Consumer<'a>`
///   holding `context: &'a [u8]` those bytes, with
///   `type Output = DeriveKeyReturn;` and `consume_key_derivation` calling
///   `payload.adapter.derive_key` with `build_derive_key_params` as in the
///   block above and `build_derive_key_payload` with the context override
///   set to `self.context`, whose length and content differ from the
///   reference context's, so a derivation that frames the context by the
///   reference context's shape fails; the case selected as in the block
///   above.
/// Act:     `create_key_derivation(&deps, params, payload)` for the case
///   and the encoding entry.
/// Assert:  the success arm is bound through `expect`; the `Ok` arm of
///   `success.output`, bound through `expect`, has `key.expose()` equal
///   to `expected_derived_key` over `DerivationPurpose::WrappingKey`, the
///   context bytes, and `REFERENCE_LENGTH` by `assert_eq!`, the
///   expectation computed by the independent oracle and not read from the
///   arrangement.
/// Boundary: the public calls `create_encoding`,
///   `lookup_kdf_declaration`, `admit_kdf_selection`,
///   `create_key_derivation`, and the consumer's `derive_key` on the
///   adapter `consume_key_derivation` receives; the route
///   `create_encoding`, the selected encoding concrete's `encode`, then
///   `admit_kdf_selection`, `create_key_derivation` over the real
///   `lookup_kdf_declaration`, the selected concrete's constructor,
///   `consume_key_derivation`, and `derive_key`.
/// Mocked:  the encoding consumer, replaced by the official
///   `MockIEncodingConsumer`, and the description it carries, replaced by
///   the official `MockIEncodingContract`, so the test does not prove
///   what a real consumer or a real description does with the concrete;
///   `domain`, `blake3`, and `zeroize` run for real.
#[test]
fn every_declared_concrete_providing_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes()
 {
    // Arrange
    struct Consumer<'a> {
        context: &'a [u8],
    }

    impl IKeyDerivationConsumer for Consumer<'_> {
        type Output = DeriveKeyReturn;

        fn consume_key_derivation<A: IKeyDerivationAdapter>(
            &self,
            _params: ConsumeKeyDerivationParams,
            payload: ConsumeKeyDerivationPayload<A>,
        ) -> Self::Output {
            payload.adapter.derive_key(
                build_derive_key_params(DeriveKeyParamsOverrides {
                    purpose: Some(DerivationPurpose::WrappingKey),
                    length: Some(REFERENCE_LENGTH),
                }),
                build_derive_key_payload(DeriveKeyPayloadOverrides {
                    context: Some(self.context),
                    ..Default::default()
                }),
            )
        }
    }

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
                    context: context.as_slice(),
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
            let derived = success.output.expect("the derivation returns a key");
            assert_eq!(
                derived.key.expose().as_slice(),
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
