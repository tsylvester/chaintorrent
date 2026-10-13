#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    AssetIdentityConstructorParamsOverrides, DeploymentIdentityConstructorParamsOverrides,
    DerivationContext, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    PieceGeometryConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity, build_deployment_identity, build_derivation_context, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_secret, build_suite_identifier,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_mock_i_encoding_consumer, create_encoding,
};
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DerivationPurpose,
    DeriveKeyErrorReturn, IKeyDerivationAdapter, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_deps, build_create_key_derivation_params,
    build_create_key_derivation_payload, create_key_derivation, expected_derived_key,
    reference_key_material_bytes,
};
use kem::{EncapsulatedValue, EncapsulatedValueOverrides, build_encapsulated_value};
use workflows::{
    PieceGroupKey, PieceGroupKeyConstructorParamsOverrides, WrapPieceGroupKeyErrorReturn,
    WrapPieceGroupKeyReturn, build_piece_group_key, build_wrap_piece_group_key_deps,
    build_wrap_piece_group_key_params, build_wrap_piece_group_key_payload, wrap_piece_group_key,
};

/// Contract: `wrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter of a case of
///   `DECLARED_KDF_SELECTIONS`, the reference key material as the encapsulated
///   value's key material, a derivation context, and a piece-group key of 32
///   zero bytes → `Ok(WrapPieceGroupKeySuccessReturn { wrapped })` with
///   `wrapped.as_bytes()` equal to the bytes `expected_derived_key` returns
///   for `DerivationPurpose::WrappingKey`, the bytes `create_encoding` returns
///   for that context, and the length `32`.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over `reference_key_material_bytes()`; the contexts
///   `build_derivation_context` at its defaults and `build_derivation_context`
///   overriding every component with its builder at a value that differs from
///   its default; `build_piece_group_key` overriding `secret` with
///   `build_secret` over 32 zero bytes; the expected bytes are
///   `expected_derived_key` over `DerivationPurpose::WrappingKey`, the
///   `encoded` bytes of the `output` of `create_encoding` over
///   `build_mock_i_encoding_consumer` carrying
///   `build_derivation_context_description`, the context, and no bytes, for
///   the same encoding case, and `32`; the two contexts differ in every
///   component, so a derivation that binds a fixed or partial context fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case, whose
///   consumer calls `wrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`; `wrapped.as_bytes()`
///   equals the expected bytes by `assert_eq!`.
/// Boundary: the public call `create_encoding` through the consumer's
///   `wrap_piece_group_key`, with the selected encoding concrete's `encode`,
///   `create_key_derivation`, and the selected KDF concrete's `derive_key`
///   running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, so this block does not prove the KEM's encapsulated bytes.
#[test]
fn wrapping_a_zero_piece_group_key_yields_the_wrapping_key_the_oracle_states() {
    // Arrange
    struct Consumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = WrapPieceGroupKeyReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.piece_group_key,
                    Default::default(),
                ),
            )
        }
    }

    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        key_material: Some(build_secret(
            reference_key_material_bytes(),
            Default::default(),
        )),
    });
    let contexts = [
        build_derivation_context(Default::default()),
        build_derivation_context(DerivationContextConstructorParamsOverrides {
            asset: Some(build_asset_identity(
                AssetIdentityConstructorParamsOverrides {
                    name: Some("other-package".to_string()),
                    version: Some("9.9.9".to_string()),
                },
            )),
            deployment: Some(build_deployment_identity(
                DeploymentIdentityConstructorParamsOverrides {
                    bytes: Some([0xEE; 32]),
                },
            )),
            suite: Some(build_suite_identifier(
                SuiteIdentifierConstructorParamsOverrides {
                    identifier: Some([0xEE; 32]),
                    version: Some(2),
                },
            )),
            parameter_set: Some(build_parameter_set_identifier(
                ParameterSetIdentifierConstructorParamsOverrides {
                    bytes: Some([0xEE; 32]),
                },
            )),
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(3),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(32768),
                    piece_group_size: Some(65536),
                    total_extent: Some(262144),
                },
            )),
        }),
    ];
    let piece_group_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
        secret: Some(build_secret(vec![0u8; 32], Default::default())),
    });
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS {
            for context in &contexts {
                let description = build_derivation_context_description(Default::default());
                let context_bytes = create_encoding(
                    &build_create_encoding_deps(build_mock_i_encoding_consumer(
                        &description,
                        context.clone(),
                        Vec::new(),
                    )),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_selection.concrete),
                        identifier: Some(encoding_selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                )
                .expect("an admitted encoding selection returns the consumer's output")
                .output
                .encoded
                .expect("the derivation context encodes")
                .bytes;
                let kdf = create_key_derivation(
                    &build_create_key_derivation_deps(Default::default()),
                    build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
                        concrete: Some(kdf_selection.concrete),
                        identifier: Some(kdf_selection.identifier),
                    }),
                    build_create_key_derivation_payload(Default::default()),
                )
                .expect("an admitted selection returns the adapter")
                .adapter;
                let consumer = Consumer {
                    kdf: &*kdf,
                    encapsulated: &encapsulated,
                    context,
                    piece_group_key: &piece_group_key,
                };
                let expected = expected_derived_key(
                    DerivationPurpose::WrappingKey,
                    context_bytes.as_slice(),
                    32,
                );

                // Act
                let output = create_encoding(
                    &build_create_encoding_deps(consumer),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_selection.concrete),
                        identifier: Some(encoding_selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                )
                .expect("an admitted encoding selection returns the consumer's output")
                .output;

                // Assert
                let wrapped = output.expect("the wrap admits the borrowed inputs").wrapped;
                assert_eq!(wrapped.as_bytes().as_slice(), expected.as_slice());
            }
        }
    }
}

/// Contract: `wrap_piece_group_key` is called as the block above with a
///   piece-group key of 32 bytes each `0xFF` → `Ok(WrapPieceGroupKeySuccessReturn
///   { wrapped })` with `wrapped.as_bytes()` equal to the bitwise complement,
///   byte by byte, of the bytes `expected_derived_key` returns for the same
///   purpose, bytes, and length.
/// Arrange: as the block above, with `build_piece_group_key` overriding
///   `secret` with `build_secret` over 32 bytes each `0xFF`; the expected
///   bytes are the complement of the oracle's bytes, so a wrap that copies the
///   wrapping key whatever the key differs from them.
/// Act:     `create_encoding(&deps, params, payload)` for the case, whose
///   consumer calls `wrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`; `wrapped.as_bytes()`
///   equals the expected bytes by `assert_eq!`.
/// Boundary: as the block above.
/// Mocked:  as the block above.
#[test]
fn wrapping_an_all_ones_piece_group_key_yields_the_complement_of_the_wrapping_key_the_oracle_states()
 {
    // Arrange
    struct Consumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = WrapPieceGroupKeyReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.piece_group_key,
                    Default::default(),
                ),
            )
        }
    }

    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        key_material: Some(build_secret(
            reference_key_material_bytes(),
            Default::default(),
        )),
    });
    let contexts = [
        build_derivation_context(Default::default()),
        build_derivation_context(DerivationContextConstructorParamsOverrides {
            asset: Some(build_asset_identity(
                AssetIdentityConstructorParamsOverrides {
                    name: Some("other-package".to_string()),
                    version: Some("9.9.9".to_string()),
                },
            )),
            deployment: Some(build_deployment_identity(
                DeploymentIdentityConstructorParamsOverrides {
                    bytes: Some([0xEE; 32]),
                },
            )),
            suite: Some(build_suite_identifier(
                SuiteIdentifierConstructorParamsOverrides {
                    identifier: Some([0xEE; 32]),
                    version: Some(2),
                },
            )),
            parameter_set: Some(build_parameter_set_identifier(
                ParameterSetIdentifierConstructorParamsOverrides {
                    bytes: Some([0xEE; 32]),
                },
            )),
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(3),
            })),
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_size: Some(32768),
                    piece_group_size: Some(65536),
                    total_extent: Some(262144),
                },
            )),
        }),
    ];
    let piece_group_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
        secret: Some(build_secret(vec![0xFF; 32], Default::default())),
    });
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS {
            for context in &contexts {
                let description = build_derivation_context_description(Default::default());
                let context_bytes = create_encoding(
                    &build_create_encoding_deps(build_mock_i_encoding_consumer(
                        &description,
                        context.clone(),
                        Vec::new(),
                    )),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_selection.concrete),
                        identifier: Some(encoding_selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                )
                .expect("an admitted encoding selection returns the consumer's output")
                .output
                .encoded
                .expect("the derivation context encodes")
                .bytes;
                let kdf = create_key_derivation(
                    &build_create_key_derivation_deps(Default::default()),
                    build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
                        concrete: Some(kdf_selection.concrete),
                        identifier: Some(kdf_selection.identifier),
                    }),
                    build_create_key_derivation_payload(Default::default()),
                )
                .expect("an admitted selection returns the adapter")
                .adapter;
                let consumer = Consumer {
                    kdf: &*kdf,
                    encapsulated: &encapsulated,
                    context,
                    piece_group_key: &piece_group_key,
                };
                let mut expected = expected_derived_key(
                    DerivationPurpose::WrappingKey,
                    context_bytes.as_slice(),
                    32,
                );
                for byte in expected.iter_mut() {
                    *byte = !*byte;
                }

                // Act
                let output = create_encoding(
                    &build_create_encoding_deps(consumer),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_selection.concrete),
                        identifier: Some(encoding_selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                )
                .expect("an admitted encoding selection returns the consumer's output")
                .output;

                // Assert
                let wrapped = output.expect("the wrap admits the borrowed inputs").wrapped;
                assert_eq!(wrapped.as_bytes().as_slice(), expected.as_slice());
            }
        }
    }
}

/// Contract: `wrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter `create_key_derivation`
///   returns for `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`, and an encapsulated value whose key
///   material is 48 bytes long → `Err(WrapPieceGroupKeyErrorReturn::
///   KeyDerivation(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapter
///   DeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 48 })))`.
/// Arrange: `build_create_key_derivation_params` overriding `concrete` with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`; `build_encapsulated_value` overriding
///   `key_material` with `build_secret` over a literal vector of 48 bytes, a
///   length that differs from the piece-group key's and from the encoded
///   context's, so a derivation fed other material reports another length;
///   `build_derivation_context` and `build_piece_group_key` at their defaults.
/// Act:     `create_encoding(&deps, params, payload)` for the case, whose
///   consumer calls `wrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`;
///   its `output.err()` equals `Some(WrapPieceGroupKeyErrorReturn::
///   KeyDerivation(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapter
///   DeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 48 })))`
///   by `assert_eq!`, the length a literal.
/// Boundary: the public call `create_encoding` through the consumer's
///   `wrap_piece_group_key`, with the selected encoding concrete's `encode`
///   running as itself.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, and the KDF concrete, the family's mock concrete under its
///   failure mode, so this block proves the refusal arrives whole and proves
///   no derivation.
#[test]
fn a_key_derivation_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = WrapPieceGroupKeyReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.piece_group_key,
                    Default::default(),
                ),
            )
        }
    }

    let kdf = create_key_derivation(
        &build_create_key_derivation_deps(Default::default()),
        build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            concrete: Some(KdfConcrete::Mock(
                MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
            )),
            ..Default::default()
        }),
        build_create_key_derivation_payload(Default::default()),
    )
    .expect("the mock KDF concrete declares the identifier the params require")
    .adapter;
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        key_material: Some(build_secret(vec![0xa7; 48], Default::default())),
    });
    let context = build_derivation_context(Default::default());
    let piece_group_key = build_piece_group_key(Default::default());
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        let consumer = Consumer {
            kdf: &*kdf,
            encapsulated: &encapsulated,
            context: &context,
            piece_group_key: &piece_group_key,
        };

        // Act
        let output = create_encoding(
            &build_create_encoding_deps(consumer),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        )
        .expect("an admitted encoding selection returns the consumer's output")
        .output;

        // Assert
        assert_eq!(
            output.err(),
            Some(WrapPieceGroupKeyErrorReturn::KeyDerivation(
                DeriveKeyErrorReturn::Mock(
                    MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                        length: 48
                    }
                )
            ))
        );
    }
}
