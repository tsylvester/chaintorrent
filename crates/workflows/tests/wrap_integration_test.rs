#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    DeploymentIdentityConstructorParamsOverrides, GroupIndexConstructorParamsOverrides,
    ParameterSetIdentifierConstructorParamsOverrides, PieceGeometryConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_deployment_identity, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_secret, build_suite_identifier,
};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodeParams, IDecoderAdapter,
    IEncoderAdapter, build_create_encoding_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DerivationPurpose,
    DeriveKeyErrorReturn, IKeyDerivationAdapter, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_params, expected_derived_key, reference_key_material_bytes,
};
use kem::{EncapsulatedValueOverrides, build_encapsulated_value};
use workflows::{
    DerivationContextConstructorParamsOverrides, IWrapPieceGroupKeyCaseConsumer, PieceGroupKey,
    PieceGroupKeyConstructorParamsOverrides, WrapPieceGroupKeyCaseParamsOverrides,
    WrapPieceGroupKeyCasePayload, WrapPieceGroupKeyErrorReturn, build_derivation_context,
    build_derivation_context_description, build_piece_group_key,
    build_wrap_piece_group_key_case_deps, build_wrap_piece_group_key_case_params,
    build_wrap_piece_group_key_case_payload, build_wrap_piece_group_key_deps,
    build_wrap_piece_group_key_params, build_wrap_piece_group_key_payload,
    run_wrap_piece_group_key_case, wrap_piece_group_key,
};

/// Contract: `wrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter of a case of
///   `DECLARED_KDF_SELECTIONS`, the reference key material as the encapsulated
///   value's key material, a derivation context, and a piece-group key of 32
///   zero bytes → `Ok(WrapPieceGroupKeySuccessReturn { wrapped })` with
///   `wrapped.as_bytes()` equal to the bytes `expected_derived_key` returns
///   for `DerivationPurpose::WrappingKey`, the bytes the encoder returns for
///   that context, and the length `32`.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over `reference_key_material_bytes()`; the contexts
///   `build_derivation_context` at its defaults and `build_derivation_context`
///   overriding every component with its builder at a value that differs from
///   its default, the asset by `build_root` over bytes other than the
///   default's; the consumer holds `build_piece_group_key` overriding `secret`
///   with `build_secret` over 32 zero bytes; the expected bytes are computed
///   in the consumer's method as `expected_derived_key` over
///   `DerivationPurpose::WrappingKey`, the `bytes` of the success arm of
///   `encoder.encode(EncodeParams { description:
///   &build_derivation_context_description(Default::default()) },
///   payload.context)`, and `32`; the two contexts differ in every component,
///   so a derivation that binds a fixed or partial context fails.
/// Act:     `run_wrap_piece_group_key_case(&deps, params, payload)` for the
///   case, whose consumer calls `wrap_piece_group_key` once.
/// Assert:  the consumer's method asserts `wrapped.as_bytes()` equals the
///   expected bytes by `assert_eq!`; the block asserts `result.is_ok()`.
/// Boundary: the public call `run_wrap_piece_group_key_case` through the
///   consumer's `wrap_piece_group_key`, with `create_encoding`, the selected
///   encoding concrete's `encode`, `create_key_derivation`, and the selected
///   KDF concrete's `derive_key` running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, so this block does not prove the KEM's encapsulated bytes.
#[test]
fn wrapping_a_zero_piece_group_key_yields_the_wrapping_key_the_oracle_states() {
    // Arrange
    struct Consumer {
        piece_group_key: PieceGroupKey,
    }

    impl IWrapPieceGroupKeyCaseConsumer for Consumer {
        type Output = ();

        fn consume_wrap_piece_group_key_case<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            payload: &WrapPieceGroupKeyCasePayload<'_>,
        ) -> Self::Output {
            let context_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &build_derivation_context_description(Default::default()),
                    },
                    payload.context,
                )
                .expect("the selected concrete encodes the derivation context")
                .bytes;
            let expected =
                expected_derived_key(DerivationPurpose::WrappingKey, context_bytes.as_slice(), 32);
            let result = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    payload.encapsulated,
                    payload.context,
                    &self.piece_group_key,
                    Default::default(),
                ),
            );
            let wrapped = result.expect("the wrap admits the borrowed inputs").wrapped;
            assert_eq!(wrapped.as_bytes().as_slice(), expected.as_slice());
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
            asset: Some(build_root(RootConstructorParamsOverrides {
                bytes: Some([0xEE; 32]),
            })),
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
        for kdf_selection in DECLARED_KDF_SELECTIONS {
            for context in &contexts {
                let consumer = Consumer {
                    piece_group_key: build_piece_group_key(
                        PieceGroupKeyConstructorParamsOverrides {
                            secret: Some(build_secret(vec![0u8; 32], Default::default())),
                        },
                    ),
                };
                let deps = build_wrap_piece_group_key_case_deps(consumer, Default::default());
                let params =
                    build_wrap_piece_group_key_case_params(WrapPieceGroupKeyCaseParamsOverrides {
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding_selection.concrete),
                                identifier: Some(encoding_selection.identifier),
                            },
                        )),
                        key_derivation: Some(build_create_key_derivation_params(
                            CreateKeyDerivationParamsOverrides {
                                concrete: Some(kdf_selection.concrete),
                                identifier: Some(kdf_selection.identifier),
                            },
                        )),
                    });
                let payload = build_wrap_piece_group_key_case_payload(
                    &encapsulated,
                    context,
                    Default::default(),
                );

                // Act
                let result = run_wrap_piece_group_key_case(&deps, params, payload);

                // Assert
                assert!(result.is_ok());
            }
        }
    }
}
// wrapping_a_zero_piece_group_key_yields_the_wrapping_key_the_oracle_states

/// Contract: `wrap_piece_group_key` is called as the block above with a
///   piece-group key of 32 bytes each `0xFF` →
///   `Ok(WrapPieceGroupKeySuccessReturn { wrapped })` with
///   `wrapped.as_bytes()` equal to the bitwise complement, byte by byte, of
///   the bytes `expected_derived_key` returns for the same purpose, bytes,
///   and length.
/// Arrange: as the block above, with the consumer holding
///   `build_piece_group_key` overriding `secret` with `build_secret` over 32
///   bytes each `0xFF`; the expected bytes are the complement of the oracle's
///   bytes, so a wrap that copies the wrapping key whatever the key differs
///   from them.
/// Act:     `run_wrap_piece_group_key_case(&deps, params, payload)` for the
///   case, whose consumer calls `wrap_piece_group_key` once.
/// Assert:  the consumer's method asserts `wrapped.as_bytes()` equals the
///   expected bytes by `assert_eq!`; the block asserts `result.is_ok()`.
/// Boundary: as the block above.
/// Mocked:  as the block above.
#[test]
fn wrapping_an_all_ones_piece_group_key_yields_the_complement_of_the_wrapping_key_the_oracle_states()
 {
    // Arrange
    struct Consumer {
        piece_group_key: PieceGroupKey,
    }

    impl IWrapPieceGroupKeyCaseConsumer for Consumer {
        type Output = ();

        fn consume_wrap_piece_group_key_case<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            payload: &WrapPieceGroupKeyCasePayload<'_>,
        ) -> Self::Output {
            let context_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &build_derivation_context_description(Default::default()),
                    },
                    payload.context,
                )
                .expect("the selected concrete encodes the derivation context")
                .bytes;
            let mut expected =
                expected_derived_key(DerivationPurpose::WrappingKey, context_bytes.as_slice(), 32);
            for byte in expected.iter_mut() {
                *byte = !*byte;
            }
            let result = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    payload.encapsulated,
                    payload.context,
                    &self.piece_group_key,
                    Default::default(),
                ),
            );
            let wrapped = result.expect("the wrap admits the borrowed inputs").wrapped;
            assert_eq!(wrapped.as_bytes().as_slice(), expected.as_slice());
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
            asset: Some(build_root(RootConstructorParamsOverrides {
                bytes: Some([0xEE; 32]),
            })),
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
        for kdf_selection in DECLARED_KDF_SELECTIONS {
            for context in &contexts {
                let consumer = Consumer {
                    piece_group_key: build_piece_group_key(
                        PieceGroupKeyConstructorParamsOverrides {
                            secret: Some(build_secret(vec![0xFF; 32], Default::default())),
                        },
                    ),
                };
                let deps = build_wrap_piece_group_key_case_deps(consumer, Default::default());
                let params =
                    build_wrap_piece_group_key_case_params(WrapPieceGroupKeyCaseParamsOverrides {
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding_selection.concrete),
                                identifier: Some(encoding_selection.identifier),
                            },
                        )),
                        key_derivation: Some(build_create_key_derivation_params(
                            CreateKeyDerivationParamsOverrides {
                                concrete: Some(kdf_selection.concrete),
                                identifier: Some(kdf_selection.identifier),
                            },
                        )),
                    });
                let payload = build_wrap_piece_group_key_case_payload(
                    &encapsulated,
                    context,
                    Default::default(),
                );

                // Act
                let result = run_wrap_piece_group_key_case(&deps, params, payload);

                // Assert
                assert!(result.is_ok());
            }
        }
    }
}
// wrapping_an_all_ones_piece_group_key_yields_the_complement_of_the_wrapping_key_the_oracle_states

/// Contract: `wrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter `create_key_derivation`
///   returns for `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`, and an encapsulated value whose key
///   material is 48 bytes long →
///   `Err(WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length: 48 })))`.
/// Arrange: `build_wrap_piece_group_key_case_params` with the
///   `key_derivation` override `build_create_key_derivation_params`
///   overriding `concrete` with `KdfConcrete::Mock(
///   MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`;
///   `build_encapsulated_value` overriding `key_material` with `build_secret`
///   over a literal vector of 48 bytes, a length that differs from the
///   piece-group key's and from the encoded context's, so a derivation fed
///   other material reports another length; `build_derivation_context` and
///   `build_piece_group_key` at their defaults, the key held by the block's
///   consumer.
/// Act:     `run_wrap_piece_group_key_case(&deps, params, payload)` for the
///   case, whose consumer calls `wrap_piece_group_key` once.
/// Assert:  the consumer's method asserts the call's `err()` equals
///   `Some(WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length: 48 })))` by `assert_eq!`, the
///   length a literal; the block asserts `result.is_ok()`.
/// Boundary: the public call `run_wrap_piece_group_key_case` through the
///   consumer's `wrap_piece_group_key`, with `create_encoding` and the
///   selected encoding concrete's `encode` running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, and the KDF concrete, the family's mock concrete under its
///   failure mode, so this block proves the refusal arrives whole and proves
///   no derivation.
#[test]
fn a_key_derivation_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer {
        piece_group_key: PieceGroupKey,
    }

    impl IWrapPieceGroupKeyCaseConsumer for Consumer {
        type Output = ();

        fn consume_wrap_piece_group_key_case<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            payload: &WrapPieceGroupKeyCasePayload<'_>,
        ) -> Self::Output {
            let result = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    payload.encapsulated,
                    payload.context,
                    &self.piece_group_key,
                    Default::default(),
                ),
            );
            assert_eq!(
                result.err(),
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

    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        key_material: Some(build_secret(vec![0xa7; 48], Default::default())),
    });
    let context = build_derivation_context(Default::default());
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        let consumer = Consumer {
            piece_group_key: build_piece_group_key(Default::default()),
        };
        let deps = build_wrap_piece_group_key_case_deps(consumer, Default::default());
        let params = build_wrap_piece_group_key_case_params(WrapPieceGroupKeyCaseParamsOverrides {
            encoding: Some(build_create_encoding_params(
                CreateEncodingParamsOverrides {
                    concrete: Some(encoding_selection.concrete),
                    identifier: Some(encoding_selection.identifier),
                },
            )),
            key_derivation: Some(build_create_key_derivation_params(
                CreateKeyDerivationParamsOverrides {
                    concrete: Some(KdfConcrete::Mock(
                        MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
                    )),
                    ..Default::default()
                },
            )),
        });
        let payload =
            build_wrap_piece_group_key_case_payload(&encapsulated, &context, Default::default());

        // Act
        let result = run_wrap_piece_group_key_case(&deps, params, payload);

        // Assert
        assert!(result.is_ok());
    }
}
// a_key_derivation_refusal_reaches_the_caller_whole
