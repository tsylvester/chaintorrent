#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    AssetIdentityConstructorParamsOverrides, DeploymentIdentityConstructorParamsOverrides,
    DerivationContext, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    PieceGeometryConstructorParamsOverrides, Secret, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity, build_asset_identity_hash, build_deployment_identity,
    build_derivation_context, build_group_index, build_parameter_set_identifier,
    build_piece_geometry, build_secret, build_suite_identifier,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_mock_i_encoding_consumer, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DerivationPurpose,
    DeriveKeyErrorReturn, IKeyDerivationAdapter, KdfConcrete, KdfIdentifier,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_deps, build_create_key_derivation_params,
    build_create_key_derivation_payload, create_key_derivation, expected_derived_key,
    reference_key_material_bytes,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, EncapsulatedValue, EncapsulatedValueOverrides,
    ICredentialKemAdapter, IKemConsumer, IdentityScope, IssuePayloadOverrides, KemIdentity,
    SetupParamsOverrides, SetupPayloadOverrides, SetupScope, build_create_kem_deps,
    build_create_kem_payload, build_decapsulate_params, build_decapsulate_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_encapsulated_value, build_issue_params, build_issue_payload,
    build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, ISampleUniformScalar,
    PAIRING_CONCRETES, build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload, create_random_source,
};
use workflows::{
    PieceGroupKey, PieceGroupKeyConstructorParamsOverrides, UnwrapPieceGroupKeyErrorReturn,
    UnwrapPieceGroupKeyReturn, WrappedPieceGroupKey,
    WrappedPieceGroupKeyConstructorParamsOverrides, build_piece_group_key,
    build_unwrap_piece_group_key_deps, build_unwrap_piece_group_key_params,
    build_unwrap_piece_group_key_payload, build_wrap_piece_group_key_deps,
    build_wrap_piece_group_key_params, build_wrap_piece_group_key_payload,
    build_wrapped_piece_group_key, unwrap_piece_group_key, wrap_piece_group_key,
};

/// Contract: `unwrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter of a case of
///   `DECLARED_KDF_SELECTIONS` that declares `KdfIdentifier::Blake3KeyedV1`,
///   the reference key material as the encapsulated value's key material, a
///   derivation context, and a wrapped key of the 32 bytes
///   `expected_derived_key` returns for `DerivationPurpose::WrappingKey`, the
///   bytes `create_encoding` returns for that context, and the length `32` →
///   `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to 32 zero bytes.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over `reference_key_material_bytes()`; the contexts
///   `build_derivation_context` at its defaults and `build_derivation_context`
///   overriding every component with its builder at a value that differs from
///   its default; the expected bytes are `expected_derived_key` over
///   `DerivationPurpose::WrappingKey`, the `encoded` bytes of the `output` of
///   `create_encoding` over `build_mock_i_encoding_consumer` carrying
///   `build_derivation_context_description`, the context, and no bytes, for
///   the same encoding case, and `32`; `build_wrapped_piece_group_key`
///   overriding `bytes` with those bytes; the two contexts differ in every
///   component, so a derivation that binds a fixed or partial context fails.
/// Act:     `create_encoding(&deps, params, payload)` for the case, whose
///   consumer calls `unwrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`;
///   `piece_group_key.expose()` equals `&[0u8; 32]` by `assert_eq!`.
/// Boundary: the public call `create_encoding` through the consumer's
///   `unwrap_piece_group_key`, with the selected encoding concrete's `encode`,
///   `create_key_derivation`, and the selected KDF concrete's `derive_key`
///   running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, so this block does not prove the KEM's encapsulated bytes.
#[test]
fn unwrapping_a_wrapped_key_equal_to_the_wrapping_key_the_oracle_states_yields_a_zero_piece_group_key()
 {
    // Arrange
    struct Consumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        wrapped: &'a WrappedPieceGroupKey,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = UnwrapPieceGroupKeyReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.wrapped,
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
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == KdfIdentifier::Blake3KeyedV1)
        {
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
                let wrapped =
                    build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
                        bytes: Some(expected_derived_key(
                            DerivationPurpose::WrappingKey,
                            context_bytes.as_slice(),
                            32,
                        )),
                    });
                let consumer = Consumer {
                    kdf: &*kdf,
                    encapsulated: &encapsulated,
                    context,
                    wrapped: &wrapped,
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
                let piece_group_key = output
                    .expect("the unwrap admits the borrowed inputs")
                    .piece_group_key;
                assert_eq!(piece_group_key.expose(), &[0u8; 32]);
            }
        }
    }
}

/// Contract: `unwrap_piece_group_key` is called as the block above with a
///   wrapped key of 32 bytes each `0xFF` → `Ok(UnwrapPieceGroupKeySuccessReturn
///   { piece_group_key })` with `piece_group_key.expose()` equal to the
///   bitwise complement, byte by byte, of the bytes `expected_derived_key`
///   returns for the same purpose, bytes, and length.
/// Arrange: as the block above, with `build_wrapped_piece_group_key`
///   overriding `bytes` with 32 bytes each `0xFF`; the expected bytes are the
///   complement of the oracle's bytes, so an unwrap that returns the wrapping
///   key whatever the wrapped key differs from them.
/// Act:     `create_encoding(&deps, params, payload)` for the case, whose
///   consumer calls `unwrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`;
///   `piece_group_key.expose()` equals the expected bytes by `assert_eq!`.
/// Boundary: as the block above.
/// Mocked:  as the block above.
#[test]
fn unwrapping_an_all_ones_wrapped_key_yields_the_complement_of_the_wrapping_key_the_oracle_states()
{
    // Arrange
    struct Consumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        wrapped: &'a WrappedPieceGroupKey,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = UnwrapPieceGroupKeyReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.wrapped,
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
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == KdfIdentifier::Blake3KeyedV1)
        {
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
                let wrapped =
                    build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
                        bytes: Some(vec![0xFF; 32]),
                    });
                let mut expected = expected_derived_key(
                    DerivationPurpose::WrappingKey,
                    context_bytes.as_slice(),
                    32,
                );
                for byte in expected.iter_mut() {
                    *byte = !*byte;
                }
                let consumer = Consumer {
                    kdf: &*kdf,
                    encapsulated: &encapsulated,
                    context,
                    wrapped: &wrapped,
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
                let piece_group_key = output
                    .expect("the unwrap admits the borrowed inputs")
                    .piece_group_key;
                assert_eq!(piece_group_key.expose().as_slice(), expected.as_slice());
            }
        }
    }
}

/// Contract: `unwrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter `create_key_derivation`
///   returns for `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`, and an encapsulated value whose key
///   material is 48 bytes long → `Err(UnwrapPieceGroupKeyErrorReturn::
///   KeyDerivation(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapter
///   DeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 48 })))`.
/// Arrange: `build_create_key_derivation_params` overriding `concrete` with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`; `build_encapsulated_value` overriding
///   `key_material` with `build_secret` over a literal vector of 48 bytes, a
///   length that differs from the wrapped key's and from the encoded
///   context's, so a derivation fed other material reports another length;
///   `build_derivation_context` and `build_wrapped_piece_group_key` at their
///   defaults.
/// Act:     `create_encoding(&deps, params, payload)` for the case, whose
///   consumer calls `unwrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`;
///   its `output.err()` equals `Some(UnwrapPieceGroupKeyErrorReturn::
///   KeyDerivation(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapter
///   DeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 48 })))`
///   by `assert_eq!`, the length a literal.
/// Boundary: the public call `create_encoding` through the consumer's
///   `unwrap_piece_group_key`, with the selected encoding concrete's `encode`
///   running as itself.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, and the KDF concrete, the family's mock concrete under its
///   failure mode, so this block proves the refusal arrives whole and proves
///   no derivation.
#[test]
fn unwrapping_returns_a_key_derivation_refusal_whole() {
    // Arrange
    struct Consumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        wrapped: &'a WrappedPieceGroupKey,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = UnwrapPieceGroupKeyReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.wrapped,
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
    let wrapped = build_wrapped_piece_group_key(Default::default());
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        let consumer = Consumer {
            kdf: &*kdf,
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrapped,
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
            Some(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(
                DeriveKeyErrorReturn::Mock(
                    MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                        length: 48
                    }
                )
            ))
        );
    }
}

/// Contract: `wrap_piece_group_key` and then `unwrap_piece_group_key` are
///   called with the encoder of a case of `DECLARED_ENCODING_SELECTIONS`, the
///   adapter of a case of `DECLARED_KDF_SELECTIONS`, one encapsulated value
///   and one derivation context, and a piece-group key of 32 pairwise distinct
///   bytes → `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to those 32 bytes.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over `reference_key_material_bytes()`;
///   `build_derivation_context` at its defaults; `build_piece_group_key`
///   overriding `secret` with `build_secret` over a literal vector of 32
///   pairwise distinct bytes, so a truncation, a reordering, or a mask the
///   wrap and the unwrap do not share differs from the bytes given.
/// Act:     `create_encoding(&deps, params, payload)` for the case, whose
///   consumer calls `wrap_piece_group_key`, binds its success through
///   `expect`, and calls `unwrap_piece_group_key` once over the wrapped key
///   returned.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`;
///   `piece_group_key.expose()` equals the literal array of those 32 bytes by
///   `assert_eq!`.
/// Boundary: the public call `create_encoding` through the consumer's
///   `wrap_piece_group_key` and `unwrap_piece_group_key`, with the selected
///   encoding concrete's `encode`, `create_key_derivation`, and the selected
///   KDF concrete's `derive_key` running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, so this block does not prove the KEM's encapsulated bytes.
#[test]
fn unwrapping_what_the_wrap_produced_returns_the_piece_group_key() {
    // Arrange
    struct Consumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IEncodingConsumer for Consumer<'_> {
        type Output = UnwrapPieceGroupKeyReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let wrapped = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.piece_group_key,
                    Default::default(),
                ),
            )
            .expect("the wrap admits the borrowed inputs")
            .wrapped;
            unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(self.kdf, &payload.adapter, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    &wrapped,
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
    let context = build_derivation_context(Default::default());
    let piece_group_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
        secret: Some(build_secret(
            vec![
                0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21, 0x02, 0xe3,
                0xc4, 0xa5, 0x86, 0x67,
            ],
            Default::default(),
        )),
    });
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS {
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
            let unwrapped = output
                .expect("the unwrap admits the wrapped key the wrap returned")
                .piece_group_key;
            assert_eq!(
                unwrapped.expose(),
                &[
                    0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89,
                    0xab, 0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21,
                    0x02, 0xe3, 0xc4, 0xa5, 0x86, 0x67,
                ]
            );
        }
    }
}

/// Contract: two parameter sets are each set up through `create_kem` under the
///   entitlement scope over one pairing, every uniform input an
///   operating-system draw; each set wraps one piece-group key under its own
///   encapsulated value and derivation context and unwraps it with the value
///   `decapsulate` returns for its credential and capsule → each
///   `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to the piece-group key wrapped, and the
///   wrapped keys of the two sets differ.
/// Arrange: each member of `PAIRING_CONCRETES`, each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, each case of
///   `DECLARED_ENCODING_SELECTIONS` and of `DECLARED_KDF_SELECTIONS`, and each
///   ordered pair of cases of `DECLARED_KEM_SELECTIONS` that declare the
///   entitlement scope, the pair's cases the two sets; every uniform input is
///   the bytes `fill_bytes` returns for
///   `FillBytesPayloadOverrides { length: Some(P::Scalar::UNIFORM_BYTES_
///   LENGTH) }` from the adapter `create_random_source` returns for
///   `RandomSourceKind::OperatingSystem`, one draw per sampler; the contexts
///   are `build_derivation_context` at its defaults for one set and
///   `build_derivation_context` overriding every component with its builder at
///   a value that differs from its default for the other;
///   `KemIdentity::Entitlement { canonical }` over a literal byte string;
///   `build_piece_group_key` overriding `secret` with `build_secret` over a
///   literal vector of 32 pairwise distinct bytes; the sets share no draw, so
///   an unwrap that derives from anything but its own set's value and context
///   returns the key for at most one set.
/// Act:     in `KemConsumer`, `unwrap_piece_group_key` once for each set, over
///   the wrapped key `wrap_piece_group_key` returned for that set and the
///   value `decapsulate` returned; `setup`, `derive_identity`, `issue`,
///   `encapsulate`, `wrap_piece_group_key`, and `decapsulate` run in the
///   arrangement.
/// Assert:  in `KemConsumer`, each unwrap's `Ok` arm is bound through `expect`
///   and its `piece_group_key.expose()` equals the literal array of the
///   piece-group key wrapped by `assert_eq!`; in `PairingConsumer`, the
///   `as_bytes()` of the wrapped keys the two `create_kem` outputs hold differ
///   by `assert_ne!`; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_encoding` through `create_pairing` and
///   `create_kem` to the consumers' `wrap_piece_group_key` and
///   `unwrap_piece_group_key`, with the route `create_random_source`, the
///   operating-system concrete's `try_new` and `fill_bytes`, the selected
///   encoding concrete's `encode`, `create_key_derivation`, the selected KDF
///   concrete's `derive_key`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected KEM concrete's `try_new`, and every method the
///   consumer calls.
/// Mocked:  nothing; the operating system's generator is the outer edge and
///   runs real, so this block proves the draws reach the samplers and the
///   unwrap of each set returns the key, and not the generator's quality.
#[test]
fn two_independently_generated_entitlement_scope_sets_unwrap_one_piece_group_key() {
    // Arrange
    let random = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the operating-system kind constructs its source")
    .adapter;

    struct KemConsumer<'a, E: IEncoderAdapter> {
        encoder: &'a E,
        kdf: &'a dyn IKeyDerivationAdapter,
        random: &'a dyn IRandomSourceAdapter,
        context: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl<'a, E: IEncoderAdapter, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, E> {
        type Output = WrappedPieceGroupKey;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let draw = || -> Secret<Vec<u8>> {
                self.random
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(P::Scalar::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the operating-system draw fills the uniform input")
                    .bytes
            };
            let scope = payload.scope();
            let hash = build_asset_identity_hash(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset {
                                identity_hash: &hash,
                            },
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("setup admits the scope the case selects and the drawn inputs");
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        match scope {
                            IdentityScope::Entitlement => DeriveIdentityPayloadOverrides {
                                identity: Some(KemIdentity::Entitlement {
                                    canonical: b"entitlement-one",
                                }),
                            },
                            IdentityScope::Asset => DeriveIdentityPayloadOverrides {
                                identity: Some(KemIdentity::Asset {
                                    identity_hash: &hash,
                                }),
                            },
                        },
                    ),
                )
                .expect("the identity the set fixes derives");
            let issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("issue admits the credential inputs");
            let encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("encapsulate admits the drawn input");
            let wrapped = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(self.kdf, self.encoder, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    &encapsulated.encapsulated,
                    self.context,
                    self.piece_group_key,
                    Default::default(),
                ),
            )
            .expect("the wrap admits the encapsulated value and context")
            .wrapped;

            // Act
            let Ok(decapsulated) = payload.adapter.decapsulate(
                build_decapsulate_params(Default::default()),
                build_decapsulate_payload(
                    &derived.identity_element,
                    &issued.credential,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );
            let unwrapped = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(self.kdf, self.encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    &decapsulated.encapsulated,
                    self.context,
                    &wrapped,
                    Default::default(),
                ),
            )
            .expect("the unwrap admits the decapsulated value")
            .piece_group_key;

            // Assert
            assert_eq!(unwrapped.expose(), self.piece_group_key.expose());
            wrapped
        }
    }

    struct PairingConsumer<'a, E: IEncoderAdapter> {
        encoder: &'a E,
        kdf: &'a dyn IKeyDerivationAdapter,
        random: &'a dyn IRandomSourceAdapter,
        context_a: &'a DerivationContext,
        context_b: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl<'a, E: IEncoderAdapter> IPairingConsumer for PairingConsumer<'a, E> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(h2s_case.concrete),
                        identifier: Some(h2s_case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for first_case in DECLARED_KEM_SELECTIONS
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Entitlement)
                {
                    for second_case in DECLARED_KEM_SELECTIONS
                        .iter()
                        .filter(|case| case.scope == IdentityScope::Entitlement)
                    {
                        let first_wrapped = create_kem(
                            &build_create_kem_deps(
                                &consumed.adapter,
                                &*hashed.adapter,
                                KemConsumer {
                                    encoder: self.encoder,
                                    kdf: self.kdf,
                                    random: self.random,
                                    context: self.context_a,
                                    piece_group_key: self.piece_group_key,
                                },
                                Default::default(),
                            ),
                            *first_case,
                            build_create_kem_payload(Default::default()),
                        )
                        .expect("the first set's case constructs and runs its consumer")
                        .output;
                        let second_wrapped = create_kem(
                            &build_create_kem_deps(
                                &consumed.adapter,
                                &*hashed.adapter,
                                KemConsumer {
                                    encoder: self.encoder,
                                    kdf: self.kdf,
                                    random: self.random,
                                    context: self.context_b,
                                    piece_group_key: self.piece_group_key,
                                },
                                Default::default(),
                            ),
                            *second_case,
                            build_create_kem_payload(Default::default()),
                        )
                        .expect("the second set's case constructs and runs its consumer")
                        .output;

                        // Assert
                        assert_ne!(first_wrapped.as_bytes(), second_wrapped.as_bytes());
                    }
                }
            }
        }
    }

    struct EncodingConsumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        random: &'a dyn IRandomSourceAdapter,
        context_a: &'a DerivationContext,
        context_b: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IEncodingConsumer for EncodingConsumer<'_> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in PAIRING_CONCRETES {
                let created = create_pairing(
                    &build_create_pairing_deps(
                        PairingConsumer {
                            encoder: &payload.adapter,
                            kdf: self.kdf,
                            random: self.random,
                            context_a: self.context_a,
                            context_b: self.context_b,
                            piece_group_key: self.piece_group_key,
                        },
                        Default::default(),
                    ),
                    build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    CreatePairingPayload,
                );
                assert!(created.is_ok());
            }
        }
    }

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
        secret: Some(build_secret(
            vec![
                0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21, 0x02, 0xe3,
                0xc4, 0xa5, 0x86, 0x67,
            ],
            Default::default(),
        )),
    });
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS {
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
            let consumer = EncodingConsumer {
                kdf: &*kdf,
                random: &*random,
                context_a: &contexts[0],
                context_b: &contexts[1],
                piece_group_key: &piece_group_key,
            };

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(encoding_selection.concrete),
                    identifier: Some(encoding_selection.identifier),
                }),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: an escrow set set up under the asset scope and a claimant's set
///   set up under the entitlement scope, otherwise as the block above → each
///   `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to the piece-group key wrapped, and the
///   wrapped keys of the two sets differ.
/// Arrange: as the block above over each ordered pair of cases of
///   `DECLARED_KEM_SELECTIONS` of which the escrow set's case declares the
///   asset scope and the claimant's set's case declares the entitlement scope;
///   the escrow set's `setup` runs under `SetupScope::Asset { identity_hash }`
///   over `build_asset_identity_hash` at its defaults and its
///   `derive_identity` over `KemIdentity::Asset { identity_hash }`; the
///   claimant's set runs under `SetupScope::Entitlement` with
///   `KemIdentity::Entitlement { canonical }` over a literal byte string; the
///   two scopes' encapsulated values reach the same unwrap, so an unwrap that
///   serves one scope only fails.
/// Act:     in `KemConsumer`, `unwrap_piece_group_key` once for each set, over
///   the wrapped key `wrap_piece_group_key` returned for that set and the
///   value `decapsulate` returned; `setup`, `derive_identity`, `issue`,
///   `encapsulate`, `wrap_piece_group_key`, and `decapsulate` run in the
///   arrangement.
/// Assert:  as the block above.
/// Boundary: as the block above.
/// Mocked:  as the block above.
#[test]
fn an_escrow_set_and_a_claimants_set_unwrap_one_piece_group_key() {
    // Arrange
    let random = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the operating-system kind constructs its source")
    .adapter;

    struct KemConsumer<'a, E: IEncoderAdapter> {
        encoder: &'a E,
        kdf: &'a dyn IKeyDerivationAdapter,
        random: &'a dyn IRandomSourceAdapter,
        context: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl<'a, E: IEncoderAdapter, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, E> {
        type Output = WrappedPieceGroupKey;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let draw = || -> Secret<Vec<u8>> {
                self.random
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(P::Scalar::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the operating-system draw fills the uniform input")
                    .bytes
            };
            let scope = payload.scope();
            let hash = build_asset_identity_hash(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset {
                                identity_hash: &hash,
                            },
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("setup admits the scope the case selects and the drawn inputs");
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        match scope {
                            IdentityScope::Entitlement => DeriveIdentityPayloadOverrides {
                                identity: Some(KemIdentity::Entitlement {
                                    canonical: b"claimant-entitlement",
                                }),
                            },
                            IdentityScope::Asset => DeriveIdentityPayloadOverrides {
                                identity: Some(KemIdentity::Asset {
                                    identity_hash: &hash,
                                }),
                            },
                        },
                    ),
                )
                .expect("the identity the set fixes derives");
            let issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("issue admits the credential inputs");
            let encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("encapsulate admits the drawn input");
            let wrapped = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(self.kdf, self.encoder, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    &encapsulated.encapsulated,
                    self.context,
                    self.piece_group_key,
                    Default::default(),
                ),
            )
            .expect("the wrap admits the encapsulated value and context")
            .wrapped;

            // Act
            let Ok(decapsulated) = payload.adapter.decapsulate(
                build_decapsulate_params(Default::default()),
                build_decapsulate_payload(
                    &derived.identity_element,
                    &issued.credential,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );
            let unwrapped = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(self.kdf, self.encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    &decapsulated.encapsulated,
                    self.context,
                    &wrapped,
                    Default::default(),
                ),
            )
            .expect("the unwrap admits the decapsulated value")
            .piece_group_key;

            // Assert
            assert_eq!(unwrapped.expose(), self.piece_group_key.expose());
            wrapped
        }
    }

    struct PairingConsumer<'a, E: IEncoderAdapter> {
        encoder: &'a E,
        kdf: &'a dyn IKeyDerivationAdapter,
        random: &'a dyn IRandomSourceAdapter,
        context_a: &'a DerivationContext,
        context_b: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl<'a, E: IEncoderAdapter> IPairingConsumer for PairingConsumer<'a, E> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(h2s_case.concrete),
                        identifier: Some(h2s_case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for escrow_case in DECLARED_KEM_SELECTIONS
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Asset)
                {
                    for claimant_case in DECLARED_KEM_SELECTIONS
                        .iter()
                        .filter(|case| case.scope == IdentityScope::Entitlement)
                    {
                        let escrow_wrapped = create_kem(
                            &build_create_kem_deps(
                                &consumed.adapter,
                                &*hashed.adapter,
                                KemConsumer {
                                    encoder: self.encoder,
                                    kdf: self.kdf,
                                    random: self.random,
                                    context: self.context_a,
                                    piece_group_key: self.piece_group_key,
                                },
                                Default::default(),
                            ),
                            *escrow_case,
                            build_create_kem_payload(Default::default()),
                        )
                        .expect("the escrow set's case constructs and runs its consumer")
                        .output;
                        let claimant_wrapped = create_kem(
                            &build_create_kem_deps(
                                &consumed.adapter,
                                &*hashed.adapter,
                                KemConsumer {
                                    encoder: self.encoder,
                                    kdf: self.kdf,
                                    random: self.random,
                                    context: self.context_b,
                                    piece_group_key: self.piece_group_key,
                                },
                                Default::default(),
                            ),
                            *claimant_case,
                            build_create_kem_payload(Default::default()),
                        )
                        .expect("the claimant set's case constructs and runs its consumer")
                        .output;

                        // Assert
                        assert_ne!(escrow_wrapped.as_bytes(), claimant_wrapped.as_bytes());
                    }
                }
            }
        }
    }

    struct EncodingConsumer<'a> {
        kdf: &'a dyn IKeyDerivationAdapter,
        random: &'a dyn IRandomSourceAdapter,
        context_a: &'a DerivationContext,
        context_b: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IEncodingConsumer for EncodingConsumer<'_> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in PAIRING_CONCRETES {
                let created = create_pairing(
                    &build_create_pairing_deps(
                        PairingConsumer {
                            encoder: &payload.adapter,
                            kdf: self.kdf,
                            random: self.random,
                            context_a: self.context_a,
                            context_b: self.context_b,
                            piece_group_key: self.piece_group_key,
                        },
                        Default::default(),
                    ),
                    build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    CreatePairingPayload,
                );
                assert!(created.is_ok());
            }
        }
    }

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
        secret: Some(build_secret(
            vec![
                0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21, 0x02, 0xe3,
                0xc4, 0xa5, 0x86, 0x67,
            ],
            Default::default(),
        )),
    });
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS {
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
            let consumer = EncodingConsumer {
                kdf: &*kdf,
                random: &*random,
                context_a: &contexts[0],
                context_b: &contexts[1],
                piece_group_key: &piece_group_key,
            };

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(encoding_selection.concrete),
                    identifier: Some(encoding_selection.identifier),
                }),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}
