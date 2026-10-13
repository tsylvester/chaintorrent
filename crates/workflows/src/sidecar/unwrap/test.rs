#![allow(clippy::expect_used)]

use super::provides::{
    UnwrapPieceGroupKeyErrorReturn, UnwrapPieceGroupKeyReturn, build_unwrap_piece_group_key_deps,
    build_unwrap_piece_group_key_params, build_unwrap_piece_group_key_payload,
    unwrap_piece_group_key,
};
use crate::sidecar::wrap::provides::{
    WrappedPieceGroupKey, WrappedPieceGroupKeyConstructorParamsOverrides,
    build_wrapped_piece_group_key,
};
use domain::{DerivationContext, build_derivation_context, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_derivation_context_description,
    build_mock_i_encoding_consumer, create_encoding,
};
use kdf::{
    CreateKeyDerivationParamsOverrides, DerivationPurpose, DeriveKeyErrorReturn,
    DeriveKeyParamsOverrides, DeriveKeyPayloadOverrides, IKeyDerivationAdapter, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_deps, build_create_key_derivation_params,
    build_create_key_derivation_payload, build_derive_key_params, build_derive_key_payload,
    create_key_derivation,
};
use kem::{EncapsulatedValue, EncapsulatedValueOverrides, build_encapsulated_value};

/// Contract: `unwrap_piece_group_key`, key derivation refused:
///   `deps.kdf.derive_key(...)`, called once, returns `Err(error)` →
///   `Err(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(error))`, the refusal
///   unchanged.
/// Arrange: `build_create_key_derivation_params` overriding `concrete` with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`; `build_encapsulated_value` overriding
///   `key_material` with `build_secret` over a literal vector of 48 bytes, a
///   length that differs from the wrapped key's, so a derivation fed other
///   material reports another length; `build_derivation_context` and
///   `build_wrapped_piece_group_key` at their defaults.
/// Act:     `create_encoding(&deps, params, payload)`, whose consumer calls
///   `unwrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`;
///   its `output.err()` equals
///   `Some(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length: 48 })))` by `assert_eq!`, the
///   length a literal.
/// Collaborators: the encoder is the encoding family's mock concrete, reached
///   through `create_encoding` by `build_create_encoding_params` at its
///   defaults; the KDF is the key-derivation family's mock concrete, reached
///   through `create_key_derivation` under the failure mode named; no concrete
///   is named but the mock selections the builders carry.
#[test]
fn unwrap_returns_the_key_derivation_refusal_unchanged() {
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
    let consumer = Consumer {
        kdf: &*kdf,
        encapsulated: &encapsulated,
        context: &context,
        wrapped: &wrapped,
    };

    // Act
    let output = create_encoding(
        &build_create_encoding_deps(consumer),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the encoding factory hands the mock encoder to the consumer")
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

/// Contract: `unwrap_piece_group_key`, unwrapped: the derived length is
///   `PIECE_GROUP_KEY_LENGTH` and the admission returns `Ok(piece_group_key)`
///   → `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })`,
///   `piece_group_key` the byte-wise XOR of the wrapped key and the wrapping
///   key.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over a literal nonempty byte vector; the context bytes are
///   the `encoded` of the `output` of `create_encoding` over
///   `build_mock_i_encoding_consumer` carrying
///   `build_derivation_context_description`, `build_derivation_context` at its
///   defaults, and no bytes; the wrapping key is the `key` of the adapter's
///   `derive_key` called with `build_derive_key_params` overriding `purpose`
///   with `DerivationPurpose::WrappingKey` and `length` with `32`, and
///   `build_derive_key_payload` overriding `key_material` with the
///   encapsulated value's key material and `context` with those bytes;
///   `build_wrapped_piece_group_key` overriding `bytes` with the bytes of
///   that wrapping key, so the XOR cancels, and a wrapping key derived under
///   another purpose, length, key material, or context differs from the
///   wrapped key and leaves a nonzero result.
/// Act:     `create_encoding(&deps, params, payload)`, whose consumer calls
///   `unwrap_piece_group_key` once over `build_derivation_context` at its
///   defaults.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`;
///   `piece_group_key.expose()` equals `&[0u8; 32]` by `assert_eq!`, the
///   expectation a literal and not read from the arrangement.
/// Collaborators: as `unwrap_returns_the_key_derivation_refusal_unchanged`,
///   with the KDF under its default `NoFailure` mode.
#[test]
fn unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_wrapping_key() {
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
            vec![0x19, 0x2b, 0x3d, 0x4f, 0x51],
            Default::default(),
        )),
    });
    let context = build_derivation_context(Default::default());
    let description = build_derivation_context_description(Default::default());
    let context_bytes = create_encoding(
        &build_create_encoding_deps(build_mock_i_encoding_consumer(
            &description,
            build_derivation_context(Default::default()),
            Vec::new(),
        )),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the encoding factory hands the mock encoder to the consumer")
    .output
    .encoded
    .expect("the mock encoder encodes the context")
    .bytes;
    let kdf = create_key_derivation(
        &build_create_key_derivation_deps(Default::default()),
        build_create_key_derivation_params(Default::default()),
        build_create_key_derivation_payload(Default::default()),
    )
    .expect("the mock KDF concrete declares the identifier the params require")
    .adapter;
    let wrapping_key = kdf
        .derive_key(
            build_derive_key_params(DeriveKeyParamsOverrides {
                purpose: Some(DerivationPurpose::WrappingKey),
                length: Some(32),
            }),
            build_derive_key_payload(DeriveKeyPayloadOverrides {
                key_material: Some(encapsulated.key_material()),
                context: Some(context_bytes.as_slice()),
            }),
        )
        .expect("the mock KDF derives under the wrapping-key purpose")
        .key;
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
        bytes: Some(wrapping_key.expose().clone()),
    });
    let consumer = Consumer {
        kdf: &*kdf,
        encapsulated: &encapsulated,
        context: &context,
        wrapped: &wrapped,
    };

    // Act
    let output = create_encoding(
        &build_create_encoding_deps(consumer),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the encoding factory hands the mock encoder to the consumer")
    .output;

    // Assert
    let piece_group_key = output
        .expect("the derivation admits the borrowed inputs")
        .piece_group_key;
    assert_eq!(piece_group_key.expose(), &[0u8; 32]);
}

/// Contract: `unwrap_piece_group_key`, unwrapped, as
///   `unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_wrapping_key`.
/// Arrange: the encapsulated value, the context bytes, and the wrapping key as
///   the block `unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_
///   wrapping_key`; `build_wrapped_piece_group_key` overriding `bytes` with
///   32 bytes each `0xFF`, so an unwrap that returns the wrapping key
///   whatever the wrapped key, or returns zeros, differs from the
///   complement.
/// Act:     `create_encoding(&deps, params, payload)`, whose consumer calls
///   `unwrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`;
///   `piece_group_key.expose()` equals the array whose byte at each index is
///   the bitwise complement of the wrapping key's byte at that index, by
///   `assert_eq!`.
/// Collaborators: as
///   `unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_wrapping_key`.
#[test]
fn unwrap_returns_the_complement_of_the_wrapping_key_when_the_wrapped_key_is_all_ones() {
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
            vec![0x19, 0x2b, 0x3d, 0x4f, 0x51],
            Default::default(),
        )),
    });
    let context = build_derivation_context(Default::default());
    let description = build_derivation_context_description(Default::default());
    let context_bytes = create_encoding(
        &build_create_encoding_deps(build_mock_i_encoding_consumer(
            &description,
            build_derivation_context(Default::default()),
            Vec::new(),
        )),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the encoding factory hands the mock encoder to the consumer")
    .output
    .encoded
    .expect("the mock encoder encodes the context")
    .bytes;
    let kdf = create_key_derivation(
        &build_create_key_derivation_deps(Default::default()),
        build_create_key_derivation_params(Default::default()),
        build_create_key_derivation_payload(Default::default()),
    )
    .expect("the mock KDF concrete declares the identifier the params require")
    .adapter;
    let wrapping_key = kdf
        .derive_key(
            build_derive_key_params(DeriveKeyParamsOverrides {
                purpose: Some(DerivationPurpose::WrappingKey),
                length: Some(32),
            }),
            build_derive_key_payload(DeriveKeyPayloadOverrides {
                key_material: Some(encapsulated.key_material()),
                context: Some(context_bytes.as_slice()),
            }),
        )
        .expect("the mock KDF derives under the wrapping-key purpose")
        .key;
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
        bytes: Some(vec![0xFF; 32]),
    });
    let mut expected = [0u8; 32];
    for (slot, byte) in expected.iter_mut().zip(wrapping_key.expose().iter()) {
        *slot = !byte;
    }
    let consumer = Consumer {
        kdf: &*kdf,
        encapsulated: &encapsulated,
        context: &context,
        wrapped: &wrapped,
    };

    // Act
    let output = create_encoding(
        &build_create_encoding_deps(consumer),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the encoding factory hands the mock encoder to the consumer")
    .output;

    // Assert
    let piece_group_key = output
        .expect("the derivation admits the borrowed inputs")
        .piece_group_key;
    assert_eq!(piece_group_key.expose(), &expected);
}
