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
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter,
    build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DerivationPurpose,
    DeriveKeyErrorReturn, IKeyDerivationAdapter, KdfConcrete, KdfIdentifier,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_params, expected_derived_key, reference_key_material_bytes,
};
use kem::{
    CreateKemParamsOverrides, DECLARED_KEM_SELECTIONS, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, EncapsulatedValue, EncapsulatedValueOverrides,
    ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides, KemIdentity, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_create_kem_params, build_decapsulate_params,
    build_decapsulate_payload, build_derive_identity_params, build_derive_identity_payload,
    build_encapsulate_params, build_encapsulate_payload, build_encapsulated_value,
    build_issue_params, build_issue_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingAdapter, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    build_fill_bytes_payload,
};
use workflows::{
    DerivationContext, DerivationContextConstructorParamsOverrides,
    IUnwrapPieceGroupKeyCaseConsumer, PieceGroupKey, PieceGroupKeyConstructorParamsOverrides,
    UnwrapPieceGroupKeyCaseParamsOverrides, UnwrapPieceGroupKeyCasePayload,
    UnwrapPieceGroupKeyErrorReturn, WrappedPieceGroupKey,
    WrappedPieceGroupKeyConstructorParamsOverrides, build_derivation_context,
    build_derivation_context_description, build_piece_group_key,
    build_unwrap_piece_group_key_case_deps, build_unwrap_piece_group_key_case_params,
    build_unwrap_piece_group_key_case_payload, build_unwrap_piece_group_key_deps,
    build_unwrap_piece_group_key_params, build_unwrap_piece_group_key_payload,
    build_wrap_piece_group_key_deps, build_wrap_piece_group_key_params,
    build_wrap_piece_group_key_payload, build_wrapped_piece_group_key,
    run_unwrap_piece_group_key_case, unwrap_piece_group_key, wrap_piece_group_key,
};

/// Contract: `unwrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter of a case of
///   `DECLARED_KDF_SELECTIONS` that declares `KdfIdentifier::Blake3KeyedV1`,
///   the reference key material as the encapsulated value's key material, a
///   derivation context, and a wrapped key of the 32 bytes
///   `expected_derived_key` returns for `DerivationPurpose::WrappingKey`, the
///   bytes the encoder returns for that context, and the length `32` →
///   `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to 32 zero bytes.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over `reference_key_material_bytes()`; the contexts
///   `build_derivation_context` at its defaults and `build_derivation_context`
///   overriding every component with its builder at a value that differs from
///   its default, the asset by `build_root` over bytes other than the
///   default's, each handed through its own case run; the expected bytes are
///   computed in the consumer's method as `expected_derived_key` over
///   `DerivationPurpose::WrappingKey`, the `bytes` of the success arm of
///   `encoder.encode(EncodeParams { description:
///   &build_derivation_context_description(Default::default()) },
///   &self.context)`, bound irrefutably, and `32`, and the `wrapped` key built
///   inside by `build_wrapped_piece_group_key` over those bytes; the two
///   contexts differ in every component, so a derivation that binds a fixed or
///   partial context fails.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)` for the
///   case, whose consumer calls `unwrap_piece_group_key` once.
/// Assert:  the consumer's method asserts `piece_group_key.expose()` equals
///   `&[0u8; 32]` by `assert_eq!`; the block asserts `result.is_ok()`.
/// Boundary: the public call `run_unwrap_piece_group_key_case` through the
///   consumer's `unwrap_piece_group_key`, with `create_encoding`, the selected
///   encoding concrete's `encode`, `create_key_derivation`, and the selected
///   KDF concrete's `derive_key` running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, so this block does not prove the KEM's encapsulated bytes.
#[test]
fn unwrapping_a_wrapped_key_equal_to_the_wrapping_key_the_oracle_states_yields_a_zero_piece_group_key()
 {
    // Arrange
    struct Consumer<'a> {
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_unwrap_piece_group_key_case<
            E: IEncoderAdapter + IDecoderAdapter,
            P: IPairingArithmetic + IPairingReference,
            K1: ICredentialKemAdapter<Pairing = P>,
            K2: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            _pairing: &P,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _kem_first: &K1,
            _kem_second: &K2,
            _random: &dyn IRandomSourceAdapter,
            _scope_first: IdentityScope,
            _scope_second: IdentityScope,
            _payload: &UnwrapPieceGroupKeyCasePayload,
        ) -> Self::Output {
            let context_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &build_derivation_context_description(Default::default()),
                    },
                    self.context,
                )
                .expect("the selected concrete encodes the derivation context")
                .bytes;
            let expected =
                expected_derived_key(DerivationPurpose::WrappingKey, context_bytes.as_slice(), 32);
            let wrapped =
                build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
                    bytes: Some(expected),
                });
            let result = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    &wrapped,
                    Default::default(),
                ),
            );
            let piece_group_key = result
                .expect("the unwrap admits the borrowed inputs")
                .piece_group_key;
            assert_eq!(piece_group_key.expose(), &[0u8; 32]);
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
        for kdf_selection in DECLARED_KDF_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == KdfIdentifier::Blake3KeyedV1)
        {
            for context in &contexts {
                let consumer = Consumer {
                    encapsulated: &encapsulated,
                    context,
                };
                let deps = build_unwrap_piece_group_key_case_deps(consumer, Default::default());
                let params = build_unwrap_piece_group_key_case_params(
                    UnwrapPieceGroupKeyCaseParamsOverrides {
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
                        ..Default::default()
                    },
                );
                let payload = build_unwrap_piece_group_key_case_payload(Default::default());

                // Act
                let result = run_unwrap_piece_group_key_case(&deps, params, payload);

                // Assert
                assert!(result.is_ok());
            }
        }
    }
}
// unwrapping_a_wrapped_key_equal_to_the_wrapping_key_the_oracle_states_yields_a_zero_piece_group_key

/// Contract: `unwrap_piece_group_key` is called as the block above with a
///   wrapped key of 32 bytes each `0xFF` →
///   `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to the bitwise complement, byte by byte,
///   of the bytes `expected_derived_key` returns for the same purpose, bytes,
///   and length.
/// Arrange: as the block above, with the consumer holding
///   `build_wrapped_piece_group_key` overriding `bytes` with 32 bytes each
///   `0xFF`; the expected bytes are the complement of the oracle's bytes, so
///   an unwrap that returns the wrapping key whatever the wrapped key
///   differs from them.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)` for the
///   case, whose consumer calls `unwrap_piece_group_key` once.
/// Assert:  the consumer's method asserts `piece_group_key.expose()` equals
///   the expected bytes by `assert_eq!`; the block asserts `result.is_ok()`.
/// Boundary: as the block above.
/// Mocked:  as the block above.
#[test]
fn unwrapping_an_all_ones_wrapped_key_yields_the_complement_of_the_wrapping_key_the_oracle_states()
{
    // Arrange
    struct Consumer<'a> {
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        wrapped: &'a WrappedPieceGroupKey,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_unwrap_piece_group_key_case<
            E: IEncoderAdapter + IDecoderAdapter,
            P: IPairingArithmetic + IPairingReference,
            K1: ICredentialKemAdapter<Pairing = P>,
            K2: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            _pairing: &P,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _kem_first: &K1,
            _kem_second: &K2,
            _random: &dyn IRandomSourceAdapter,
            _scope_first: IdentityScope,
            _scope_second: IdentityScope,
            _payload: &UnwrapPieceGroupKeyCasePayload,
        ) -> Self::Output {
            let context_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &build_derivation_context_description(Default::default()),
                    },
                    self.context,
                )
                .expect("the selected concrete encodes the derivation context")
                .bytes;
            let mut expected =
                expected_derived_key(DerivationPurpose::WrappingKey, context_bytes.as_slice(), 32);
            for byte in expected.iter_mut() {
                *byte = !*byte;
            }
            let result = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.wrapped,
                    Default::default(),
                ),
            );
            let piece_group_key = result
                .expect("the unwrap admits the borrowed inputs")
                .piece_group_key;
            assert_eq!(piece_group_key.expose(), expected.as_slice());
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
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
        bytes: Some(vec![0xFF; 32]),
    });
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        for kdf_selection in DECLARED_KDF_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == KdfIdentifier::Blake3KeyedV1)
        {
            for context in &contexts {
                let consumer = Consumer {
                    encapsulated: &encapsulated,
                    context,
                    wrapped: &wrapped,
                };
                let deps = build_unwrap_piece_group_key_case_deps(consumer, Default::default());
                let params = build_unwrap_piece_group_key_case_params(
                    UnwrapPieceGroupKeyCaseParamsOverrides {
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
                        ..Default::default()
                    },
                );
                let payload = build_unwrap_piece_group_key_case_payload(Default::default());

                // Act
                let result = run_unwrap_piece_group_key_case(&deps, params, payload);

                // Assert
                assert!(result.is_ok());
            }
        }
    }
}
// unwrapping_an_all_ones_wrapped_key_yields_the_complement_of_the_wrapping_key_the_oracle_states

/// Contract: `unwrap_piece_group_key` is called with the encoder of a case of
///   `DECLARED_ENCODING_SELECTIONS`, the adapter `create_key_derivation`
///   returns for `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`, and an encapsulated value whose key
///   material is 48 bytes long →
///   `Err(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length: 48 })))`.
/// Arrange: `params` from `build_unwrap_piece_group_key_case_params` with the
///   `key_derivation` override set to `build_create_key_derivation_params`
///   overriding `concrete` with `KdfConcrete::Mock(
///   MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`;
///   `build_encapsulated_value` overriding `key_material` with `build_secret`
///   over a literal vector of 48 bytes, a length that differs from the
///   wrapped key's and from the encoded context's, so a derivation fed other
///   material reports another length; `build_derivation_context` and
///   `build_wrapped_piece_group_key` at their defaults, held by the block's
///   consumer.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)` for the
///   case, whose consumer calls `unwrap_piece_group_key` once.
/// Assert:  the consumer's method asserts the call's `err()` equals
///   `Some(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length: 48 })))` by `assert_eq!`, the
///   length a literal; the block asserts `result.is_ok()`.
/// Boundary: the public call `run_unwrap_piece_group_key_case` through the
///   consumer's `unwrap_piece_group_key`, with `create_encoding` and the
///   selected encoding concrete's `encode` running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, and the KDF concrete, the family's mock concrete under its
///   failure mode, so this block proves the refusal arrives whole and proves
///   no derivation.
#[test]
fn unwrapping_returns_a_key_derivation_refusal_whole() {
    // Arrange
    struct Consumer<'a> {
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        wrapped: &'a WrappedPieceGroupKey,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_unwrap_piece_group_key_case<
            E: IEncoderAdapter + IDecoderAdapter,
            P: IPairingArithmetic + IPairingReference,
            K1: ICredentialKemAdapter<Pairing = P>,
            K2: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            _pairing: &P,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _kem_first: &K1,
            _kem_second: &K2,
            _random: &dyn IRandomSourceAdapter,
            _scope_first: IdentityScope,
            _scope_second: IdentityScope,
            _payload: &UnwrapPieceGroupKeyCasePayload,
        ) -> Self::Output {
            let result = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.wrapped,
                    Default::default(),
                ),
            );
            assert_eq!(
                result.err(),
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

    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        key_material: Some(build_secret(vec![0xa7; 48], Default::default())),
    });
    let context = build_derivation_context(Default::default());
    let wrapped = build_wrapped_piece_group_key(Default::default());
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        let consumer = Consumer {
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrapped,
        };
        let deps = build_unwrap_piece_group_key_case_deps(consumer, Default::default());
        let params =
            build_unwrap_piece_group_key_case_params(UnwrapPieceGroupKeyCaseParamsOverrides {
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
                ..Default::default()
            });
        let payload = build_unwrap_piece_group_key_case_payload(Default::default());

        // Act
        let result = run_unwrap_piece_group_key_case(&deps, params, payload);

        // Assert
        assert!(result.is_ok());
    }
}
// unwrapping_returns_a_key_derivation_refusal_whole

/// Contract: `wrap_piece_group_key` and then `unwrap_piece_group_key` are
///   called with the encoder of a case of `DECLARED_ENCODING_SELECTIONS`, the
///   adapter of a case of `DECLARED_KDF_SELECTIONS`, one encapsulated value
///   and one derivation context, and a piece-group key of 32 pairwise distinct
///   bytes → `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to those 32 bytes.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over `reference_key_material_bytes()`;
///   `build_derivation_context` at its defaults; the consumer holds
///   `build_piece_group_key` overriding `secret` with `build_secret` over a
///   literal vector of 32 pairwise distinct bytes, so a truncation, a
///   reordering, or a mask the wrap and the unwrap do not share differs from
///   the bytes given.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)` for the
///   case, whose consumer's method calls `wrap_piece_group_key`, binds its
///   success through `expect`, and calls `unwrap_piece_group_key` once over
///   the wrapped key returned.
/// Assert:  the consumer's method asserts `piece_group_key.expose()` equals
///   the literal array of those 32 bytes by `assert_eq!`; the block asserts
///   `result.is_ok()`.
/// Boundary: the public call `run_unwrap_piece_group_key_case` through the
///   consumer's `wrap_piece_group_key` and `unwrap_piece_group_key`, with
///   `create_encoding`, the selected encoding concrete's `encode`,
///   `create_key_derivation`, and the selected KDF concrete's `derive_key`
///   running as themselves.
/// Mocked:  the credential KEM, whose output is the `build_encapsulated_value`
///   fixture, so this block does not prove the KEM's encapsulated bytes.
#[test]
fn unwrapping_what_the_wrap_produced_returns_the_piece_group_key() {
    // Arrange
    struct Consumer<'a> {
        encapsulated: &'a EncapsulatedValue,
        context: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_unwrap_piece_group_key_case<
            E: IEncoderAdapter + IDecoderAdapter,
            P: IPairingArithmetic + IPairingReference,
            K1: ICredentialKemAdapter<Pairing = P>,
            K2: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            _pairing: &P,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _kem_first: &K1,
            _kem_second: &K2,
            _random: &dyn IRandomSourceAdapter,
            _scope_first: IdentityScope,
            _scope_second: IdentityScope,
            _payload: &UnwrapPieceGroupKeyCasePayload,
        ) -> Self::Output {
            let wrapped = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    self.piece_group_key,
                    Default::default(),
                ),
            )
            .expect("the wrap admits the piece-group key")
            .wrapped;
            let result = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    self.encapsulated,
                    self.context,
                    &wrapped,
                    Default::default(),
                ),
            );
            let piece_group_key = result
                .expect("the unwrap admits the wrapped key")
                .piece_group_key;
            assert_eq!(
                piece_group_key.expose(),
                &[
                    0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89,
                    0xab, 0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21,
                    0x02, 0xe3, 0xc4, 0xa5, 0x86, 0x67,
                ]
            );
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
            let consumer = Consumer {
                encapsulated: &encapsulated,
                context: &context,
                piece_group_key: &piece_group_key,
            };
            let deps = build_unwrap_piece_group_key_case_deps(consumer, Default::default());
            let params =
                build_unwrap_piece_group_key_case_params(UnwrapPieceGroupKeyCaseParamsOverrides {
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
                    ..Default::default()
                });
            let payload = build_unwrap_piece_group_key_case_payload(Default::default());

            // Act
            let result = run_unwrap_piece_group_key_case(&deps, params, payload);

            // Assert
            assert!(result.is_ok());
        }
    }
}
// unwrapping_what_the_wrap_produced_returns_the_piece_group_key

/// Contract: two parameter sets are each set up through `create_kem` under
///   the entitlement scope over one pairing, every uniform input an
///   operating-system draw; each set wraps one piece-group key under its own
///   encapsulated value and derivation context and unwraps it with the value
///   `decapsulate` returns for its credential and capsule → each
///   `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to the piece-group key wrapped, and the
///   wrapped keys of the two sets differ.
/// Arrange: `params` from `build_unwrap_piece_group_key_case_params` with
///   `pairing` set to `build_create_pairing_params` carrying the member of
///   `PAIRING_CONCRETES`, `hash_to_scalar` to
///   `build_create_hash_to_scalar_params` carrying the case, `random_source`
///   to `build_create_random_source_params` carrying
///   `RandomSourceKind::OperatingSystem`, `encoding` and `key_derivation` to
///   `build_create_encoding_params` and `build_create_key_derivation_params`
///   carrying their cases, and `kem` and `kem_second` to
///   `build_create_kem_params` carrying the ordered pair's two cases of
///   `DECLARED_KEM_SELECTIONS` that declare the entitlement scope; the
///   consumer's method runs each set in turn — `setup` under
///   `SetupScope::Entitlement`, `derive_identity` over
///   `KemIdentity::Entitlement { canonical }` over a literal byte string,
///   `issue`, `encapsulate`, `wrap_piece_group_key` over the
///   `EncapsulatedValue` `encapsulate` returned and the set's own derivation
///   context, `decapsulate` with its credential and capsule, and
///   `unwrap_piece_group_key` over the value `decapsulate` returned —
///   `kem_first` and `scope_first` for the first set and `kem_second` and
///   `scope_second` for the second, every uniform input the bytes `random`'s
///   `fill_bytes` returns for `FillBytesPayloadOverrides { length:
///   Some(P::Scalar::UNIFORM_BYTES_LENGTH) }`, one draw per sampler; the
///   contexts are `build_derivation_context` at its defaults for one set and
///   `build_derivation_context` overriding every component with its builder
///   at a value that differs from its default for the other;
///   `build_piece_group_key` overriding `secret` with `build_secret` over a
///   literal vector of 32 pairwise distinct bytes; the sets share no draw, so
///   an unwrap that derives from anything but its own set's value and
///   context returns the key for at most one set.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)` for the
///   case.
/// Assert:  the consumer's method binds each unwrap's `Ok` arm through
///   `expect` and asserts its `piece_group_key.expose()` equals the literal
///   array of the piece-group key wrapped by `assert_eq!`, and asserts the
///   `as_bytes()` of the two sets' wrapped keys differ by `assert_ne!`; the
///   block asserts `result.is_ok()`.
/// Boundary: the public call `run_unwrap_piece_group_key_case` through the
///   consumer's `wrap_piece_group_key` and `unwrap_piece_group_key`, with the
///   route `create_encoding`, the selected encoding concrete's `encode`,
///   `create_key_derivation`, the selected KDF concrete's `derive_key`,
///   `create_pairing`, the arm's concrete through its `try_new`,
///   `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, both `create_kem` calls, the selected KEM concretes'
///   `try_new`s, and every method the consumer calls.
/// Mocked:  nothing; the operating system's generator is the outer edge and
///   runs real, so this block proves the draws reach the samplers and the
///   unwrap of each set returns the key, and not the generator's quality.
#[test]
fn two_independently_generated_entitlement_scope_sets_unwrap_one_piece_group_key() {
    // Arrange
    struct Consumer<'a> {
        context_a: &'a DerivationContext,
        context_b: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_unwrap_piece_group_key_case<
            E: IEncoderAdapter + IDecoderAdapter,
            P: IPairingArithmetic + IPairingReference,
            K1: ICredentialKemAdapter<Pairing = P>,
            K2: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            _pairing: &P,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            kem_first: &K1,
            kem_second: &K2,
            random: &dyn IRandomSourceAdapter,
            scope_first: IdentityScope,
            scope_second: IdentityScope,
            _payload: &UnwrapPieceGroupKeyCasePayload,
        ) -> Self::Output {
            let wrapped_first = run_set(
                kem_first,
                scope_first,
                encoder,
                kdf,
                random,
                self.context_a,
                self.piece_group_key,
            );
            let wrapped_second = run_set(
                kem_second,
                scope_second,
                encoder,
                kdf,
                random,
                self.context_b,
                self.piece_group_key,
            );
            assert_ne!(wrapped_first.as_bytes(), wrapped_second.as_bytes());
        }
    }

    let context_a = build_derivation_context(Default::default());
    let context_b = build_derivation_context(DerivationContextConstructorParamsOverrides {
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
    });
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
    for concrete in PAIRING_CONCRETES {
        for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for kdf_selection in DECLARED_KDF_SELECTIONS {
                    for first_case in DECLARED_KEM_SELECTIONS
                        .iter()
                        .filter(|case| case.scope == IdentityScope::Entitlement)
                    {
                        for second_case in DECLARED_KEM_SELECTIONS
                            .iter()
                            .filter(|case| case.scope == IdentityScope::Entitlement)
                        {
                            let consumer = Consumer {
                                context_a: &context_a,
                                context_b: &context_b,
                                piece_group_key: &piece_group_key,
                            };
                            let deps = build_unwrap_piece_group_key_case_deps(
                                consumer,
                                Default::default(),
                            );
                            let params = build_unwrap_piece_group_key_case_params(
                                UnwrapPieceGroupKeyCaseParamsOverrides {
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
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(h2s_case.concrete),
                                            identifier: Some(h2s_case.identifier),
                                        },
                                    )),
                                    random_source: Some(build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    )),
                                    kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                                        concrete: Some(first_case.concrete),
                                        identifier: Some(first_case.identifier),
                                        scope: Some(first_case.scope),
                                    })),
                                    kem_second: Some(build_create_kem_params(
                                        CreateKemParamsOverrides {
                                            concrete: Some(second_case.concrete),
                                            identifier: Some(second_case.identifier),
                                            scope: Some(second_case.scope),
                                        },
                                    )),
                                },
                            );
                            let payload =
                                build_unwrap_piece_group_key_case_payload(Default::default());

                            // Act
                            let result = run_unwrap_piece_group_key_case(&deps, params, payload);

                            // Assert
                            assert!(result.is_ok());
                        }
                    }
                }
            }
        }
    }
}
// two_independently_generated_entitlement_scope_sets_unwrap_one_piece_group_key

/// Contract: an escrow set set up under the asset scope and a claimant's set
///   set up under the entitlement scope, otherwise as the block above → each
///   `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })` with
///   `piece_group_key.expose()` equal to the piece-group key wrapped, and the
///   wrapped keys of the two sets differ.
/// Arrange: as the block above over each ordered pair of cases of
///   `DECLARED_KEM_SELECTIONS` of which the escrow set's case declares the
///   asset scope and the claimant's set's case declares the entitlement
///   scope, `kem` the escrow case and `kem_second` the claimant's; the
///   escrow set's `setup` runs under `SetupScope::Asset { root: &root }` over
///   `build_root` at its defaults and its `derive_identity` over
///   `KemIdentity::Asset { root: &root }`; the claimant's set runs under
///   `SetupScope::Entitlement` with `KemIdentity::Entitlement { canonical }`
///   over a literal byte string; the two scopes' encapsulated values reach
///   the same unwrap, so an unwrap that serves one scope only fails.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)` for the
///   case.
/// Assert:  as the block above.
/// Boundary: as the block above.
/// Mocked:  as the block above.
#[test]
fn an_escrow_set_and_a_claimants_set_unwrap_one_piece_group_key() {
    // Arrange
    struct Consumer<'a> {
        context_a: &'a DerivationContext,
        context_b: &'a DerivationContext,
        piece_group_key: &'a PieceGroupKey,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_unwrap_piece_group_key_case<
            E: IEncoderAdapter + IDecoderAdapter,
            P: IPairingArithmetic + IPairingReference,
            K1: ICredentialKemAdapter<Pairing = P>,
            K2: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            encoder: &E,
            kdf: &dyn IKeyDerivationAdapter,
            _pairing: &P,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            kem_first: &K1,
            kem_second: &K2,
            random: &dyn IRandomSourceAdapter,
            scope_first: IdentityScope,
            scope_second: IdentityScope,
            _payload: &UnwrapPieceGroupKeyCasePayload,
        ) -> Self::Output {
            let wrapped_escrow = run_set(
                kem_first,
                scope_first,
                encoder,
                kdf,
                random,
                self.context_a,
                self.piece_group_key,
            );
            let wrapped_claimant = run_set(
                kem_second,
                scope_second,
                encoder,
                kdf,
                random,
                self.context_b,
                self.piece_group_key,
            );
            assert_ne!(wrapped_escrow.as_bytes(), wrapped_claimant.as_bytes());
        }
    }

    let context_a = build_derivation_context(Default::default());
    let context_b = build_derivation_context(DerivationContextConstructorParamsOverrides {
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
    });
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
    for concrete in PAIRING_CONCRETES {
        for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for kdf_selection in DECLARED_KDF_SELECTIONS {
                    for escrow_case in DECLARED_KEM_SELECTIONS
                        .iter()
                        .filter(|case| case.scope == IdentityScope::Asset)
                    {
                        for claimant_case in DECLARED_KEM_SELECTIONS
                            .iter()
                            .filter(|case| case.scope == IdentityScope::Entitlement)
                        {
                            let consumer = Consumer {
                                context_a: &context_a,
                                context_b: &context_b,
                                piece_group_key: &piece_group_key,
                            };
                            let deps = build_unwrap_piece_group_key_case_deps(
                                consumer,
                                Default::default(),
                            );
                            let params = build_unwrap_piece_group_key_case_params(
                                UnwrapPieceGroupKeyCaseParamsOverrides {
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
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(h2s_case.concrete),
                                            identifier: Some(h2s_case.identifier),
                                        },
                                    )),
                                    random_source: Some(build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    )),
                                    kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                                        concrete: Some(escrow_case.concrete),
                                        identifier: Some(escrow_case.identifier),
                                        scope: Some(escrow_case.scope),
                                    })),
                                    kem_second: Some(build_create_kem_params(
                                        CreateKemParamsOverrides {
                                            concrete: Some(claimant_case.concrete),
                                            identifier: Some(claimant_case.identifier),
                                            scope: Some(claimant_case.scope),
                                        },
                                    )),
                                },
                            );
                            let payload =
                                build_unwrap_piece_group_key_case_payload(Default::default());

                            // Act
                            let result = run_unwrap_piece_group_key_case(&deps, params, payload);

                            // Assert
                            assert!(result.is_ok());
                        }
                    }
                }
            }
        }
    }
}
// an_escrow_set_and_a_claimants_set_unwrap_one_piece_group_key

fn run_set<E: IEncoderAdapter, K: ICredentialKemAdapter>(
    kem: &K,
    scope: IdentityScope,
    encoder: &E,
    kdf: &dyn IKeyDerivationAdapter,
    random: &dyn IRandomSourceAdapter,
    context: &DerivationContext,
    piece_group_key: &PieceGroupKey,
) -> WrappedPieceGroupKey {
    let draw = || {
        random
            .fill_bytes(
                FillBytesParams,
                build_fill_bytes_payload(FillBytesPayloadOverrides {
                    length: Some(<<K as ICredentialKemAdapter>::Pairing as IPairingAdapter>::Scalar::UNIFORM_BYTES_LENGTH),
                }),
            )
            .expect("the operating-system draw fills the uniform input")
            .bytes
    };
    let root = build_root(Default::default());
    let setup = kem
        .setup(
            build_setup_params(SetupParamsOverrides {
                scope: Some(match scope {
                    IdentityScope::Entitlement => SetupScope::Entitlement,
                    IdentityScope::Asset => SetupScope::Asset { root: &root },
                }),
            }),
            build_setup_payload::<<<K as ICredentialKemAdapter>::Pairing as IPairingAdapter>::Scalar>(SetupPayloadOverrides {
                master_uniform: Some(draw()),
                u0_uniform: Some(draw()),
                u1_uniform: Some(draw()),
            }),
        )
        .expect("setup admits the scope the case selects and the drawn inputs");
    let derived = kem
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
                        identity: Some(KemIdentity::Asset { root: &root }),
                    },
                },
            ),
        )
        .expect("the identity the set fixes derives");
    let issued = kem
        .issue(
            build_issue_params(Default::default()),
            build_issue_payload::<
                <<K as ICredentialKemAdapter>::Pairing as IPairingAdapter>::Scalar,
                _,
                _,
                _,
            >(
                &setup.parameter_set,
                &setup.master_scalar,
                &derived.identity_element,
                IssuePayloadOverrides {
                    uniform: Some(draw()),
                },
            ),
        )
        .expect("issue admits the credential inputs");
    let encapsulated = kem
        .encapsulate(
            build_encapsulate_params(Default::default()),
            build_encapsulate_payload::<
                <<K as ICredentialKemAdapter>::Pairing as IPairingAdapter>::Scalar,
                _,
            >(
                &setup.parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(draw()),
                },
            ),
        )
        .expect("encapsulate admits the drawn input");
    let wrapped = wrap_piece_group_key(
        &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
        build_wrap_piece_group_key_params(Default::default()),
        build_wrap_piece_group_key_payload(
            &encapsulated.encapsulated,
            context,
            piece_group_key,
            Default::default(),
        ),
    )
    .expect("the wrap admits the encapsulated value and context")
    .wrapped;
    let Ok(decapsulated) = kem.decapsulate(
        build_decapsulate_params(Default::default()),
        build_decapsulate_payload(
            &derived.identity_element,
            &issued.credential,
            &encapsulated.capsule,
            Default::default(),
        ),
    );
    let unwrapped = unwrap_piece_group_key(
        &build_unwrap_piece_group_key_deps(kdf, encoder, Default::default()),
        build_unwrap_piece_group_key_params(Default::default()),
        build_unwrap_piece_group_key_payload(
            &decapsulated.encapsulated,
            context,
            &wrapped,
            Default::default(),
        ),
    )
    .expect("the unwrap admits the decapsulated value")
    .piece_group_key;
    assert_eq!(unwrapped.expose(), piece_group_key.expose());
    wrapped
}
