#![allow(clippy::expect_used)]

use super::provides::{
    PieceGroupKey, PieceGroupKeyConstructorParamsOverrides, PieceGroupKeyErrorReturn,
    WrapPieceGroupKeyErrorReturn, WrapPieceGroupKeyReturn, WrappedPieceGroupKey,
    WrappedPieceGroupKeyConstructorParamsOverrides, WrappedPieceGroupKeyErrorReturn,
    build_piece_group_key, build_piece_group_key_constructor_params,
    build_wrap_piece_group_key_deps, build_wrap_piece_group_key_params,
    build_wrap_piece_group_key_payload, build_wrapped_piece_group_key_constructor_params,
    wrap_piece_group_key,
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

/// Contract: `wrap_piece_group_key`, key derivation refused:
///   `deps.kdf.derive_key(...)`, called once, returns `Err(error)` →
///   `Err(WrapPieceGroupKeyErrorReturn::KeyDerivation(error))`, the refusal
///   unchanged.
/// Arrange: `build_create_key_derivation_params` overriding `concrete` with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`; `build_encapsulated_value` overriding
///   `key_material` with `build_secret` over a literal vector of 48 bytes, a
///   length that differs from the piece-group key's, so a derivation fed other
///   material reports another length; `build_derivation_context` and
///   `build_piece_group_key` at their defaults.
/// Act:     `create_encoding(&deps, params, payload)`, whose consumer calls
///   `wrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`;
///   its `output.err()` equals
///   `Some(WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length: 48 })))` by `assert_eq!`, the
///   length a literal.
/// Collaborators: the encoder is the encoding family's mock concrete, reached
///   through `create_encoding` by `build_create_encoding_params` at its
///   defaults; the KDF is the key-derivation family's mock concrete, reached
///   through `create_key_derivation` under the failure mode named; no concrete
///   is named but the mock selections the builders carry.
#[test]
fn wrap_returns_the_key_derivation_refusal_unchanged() {
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
    let consumer = Consumer {
        kdf: &*kdf,
        encapsulated: &encapsulated,
        context: &context,
        piece_group_key: &piece_group_key,
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
        Some(WrapPieceGroupKeyErrorReturn::KeyDerivation(
            DeriveKeyErrorReturn::Mock(
                MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                    length: 48
                }
            )
        ))
    );
}

/// Contract: `wrap_piece_group_key`, wrapped: the derived length is
///   `PIECE_GROUP_KEY_LENGTH` and the admission returns `Ok(wrapped)` →
///   `Ok(WrapPieceGroupKeySuccessReturn { wrapped })`, `wrapped` the byte-wise
///   XOR of the piece-group key and the wrapping key.
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
///   `build_piece_group_key` overriding `secret` with `build_secret` over the
///   bytes of that wrapping key, so the XOR cancels, and a wrapping key
///   derived under another purpose, length, key material, or context differs
///   from the piece-group key and leaves a nonzero result.
/// Act:     `create_encoding(&deps, params, payload)`, whose consumer calls
///   `wrap_piece_group_key` once over `build_derivation_context` at its
///   defaults.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`; `wrapped.as_bytes()`
///   equals `&[0u8; 32]` by `assert_eq!`, the expectation a literal and not
///   read from the arrangement.
/// Collaborators: as `wrap_returns_the_key_derivation_refusal_unchanged`, with
///   the KDF under its default `NoFailure` mode.
#[test]
fn wrap_returns_a_zero_wrapped_key_when_the_piece_group_key_equals_the_wrapping_key() {
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
    let piece_group_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
        secret: Some(build_secret(
            wrapping_key.expose().clone(),
            Default::default(),
        )),
    });
    let consumer = Consumer {
        kdf: &*kdf,
        encapsulated: &encapsulated,
        context: &context,
        piece_group_key: &piece_group_key,
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
    let wrapped = output
        .expect("the derivation admits the borrowed inputs")
        .wrapped;
    assert_eq!(wrapped.as_bytes(), &[0u8; 32]);
}

/// Contract: `wrap_piece_group_key`, wrapped, as
///   `wrap_returns_a_zero_wrapped_key_when_the_piece_group_key_equals_the_
///   wrapping_key`.
/// Arrange: the encapsulated value, the context bytes, and the wrapping key as
///   the block `wrap_returns_a_zero_wrapped_key_when_the_piece_group_key_
///   equals_the_wrapping_key`; `build_piece_group_key` overriding `secret`
///   with `build_secret` over 32 bytes each `0xFF`, so a wrap that copies the
///   wrapping key, or returns zeros, differs from the complement.
/// Act:     `create_encoding(&deps, params, payload)`, whose consumer calls
///   `wrap_piece_group_key` once.
/// Assert:  the success arm of `create_encoding` is bound through `expect`,
///   and the `Ok` arm of its `output` through `expect`; `wrapped.as_bytes()`
///   equals the array whose byte at each index is the bitwise complement of
///   the wrapping key's byte at that index, by `assert_eq!`.
/// Collaborators: as
///   `wrap_returns_a_zero_wrapped_key_when_the_piece_group_key_equals_the_
///   wrapping_key`.
#[test]
fn wrap_returns_the_complement_of_the_wrapping_key_when_the_piece_group_key_is_all_ones() {
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
    let piece_group_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
        secret: Some(build_secret(vec![0xFF; 32], Default::default())),
    });
    let mut expected = [0u8; 32];
    for (slot, byte) in expected.iter_mut().zip(wrapping_key.expose().iter()) {
        *slot = !byte;
    }
    let consumer = Consumer {
        kdf: &*kdf,
        encapsulated: &encapsulated,
        context: &context,
        piece_group_key: &piece_group_key,
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
    let wrapped = output
        .expect("the derivation admits the borrowed inputs")
        .wrapped;
    assert_eq!(wrapped.as_bytes(), &expected);
}

/// Contract: `PieceGroupKey::try_new`, wrong length: the secret's length is not
///   `PIECE_GROUP_KEY_LENGTH` → `Err(PieceGroupKeyErrorReturn::WrongLength
///   { expected, actual })`.
/// Arrange: `build_piece_group_key_constructor_params` overriding `secret`
///   with `build_secret` over a vector of 31 bytes, one below the key length,
///   so a comparison that admits a shorter secret fails.
/// Act:     `PieceGroupKey::try_new(params)`.
/// Assert:  `result.err()` equals `Some(PieceGroupKeyErrorReturn::WrongLength
///   { expected: 32, actual: 31 })` by `assert_eq!`.
/// Collaborators: none; the fixture is
///   `build_piece_group_key_constructor_params`.
#[test]
fn piece_group_key_try_new_refuses_a_secret_shorter_than_the_key_length() {
    // Arrange
    let params =
        build_piece_group_key_constructor_params(PieceGroupKeyConstructorParamsOverrides {
            secret: Some(build_secret(vec![0x3c; 31], Default::default())),
        });

    // Act
    let result = PieceGroupKey::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(PieceGroupKeyErrorReturn::WrongLength {
            expected: 32,
            actual: 31
        })
    );
}

/// Contract: `PieceGroupKey::try_new`, wrong length, as
///   `piece_group_key_try_new_refuses_a_secret_shorter_than_the_key_length`.
/// Arrange: `build_piece_group_key_constructor_params` overriding `secret`
///   with `build_secret` over a vector of 33 bytes, one above the key length,
///   so a comparison that admits a longer secret fails.
/// Act:     `PieceGroupKey::try_new(params)`.
/// Assert:  `result.err()` equals `Some(PieceGroupKeyErrorReturn::WrongLength
///   { expected: 32, actual: 33 })` by `assert_eq!`.
/// Collaborators: none; the fixture is
///   `build_piece_group_key_constructor_params`.
#[test]
fn piece_group_key_try_new_refuses_a_secret_longer_than_the_key_length() {
    // Arrange
    let params =
        build_piece_group_key_constructor_params(PieceGroupKeyConstructorParamsOverrides {
            secret: Some(build_secret(vec![0x3c; 33], Default::default())),
        });

    // Act
    let result = PieceGroupKey::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(PieceGroupKeyErrorReturn::WrongLength {
            expected: 32,
            actual: 33
        })
    );
}

/// Contract: `PieceGroupKey::try_new`, admitted: the secret's length is
///   `PIECE_GROUP_KEY_LENGTH` → `Ok(PieceGroupKey)` holding the secret's bytes.
/// Arrange: `build_piece_group_key_constructor_params` overriding `secret`
///   with `build_secret` over a literal vector of 32 pairwise distinct bytes,
///   so a truncation or a reordering differs from the bytes given.
/// Act:     `PieceGroupKey::try_new(params)`.
/// Assert:  the success arm is bound through `expect`; `key.expose()` equals
///   the literal array of those 32 bytes by `assert_eq!`.
/// Collaborators: none; the fixture is
///   `build_piece_group_key_constructor_params`.
#[test]
fn piece_group_key_try_new_holds_the_bytes_of_an_admitted_secret() {
    // Arrange
    let params =
        build_piece_group_key_constructor_params(PieceGroupKeyConstructorParamsOverrides {
            secret: Some(build_secret(
                vec![
                    0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89,
                    0xab, 0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21,
                    0x02, 0xe3, 0xc4, 0xa5, 0x86, 0x67,
                ],
                Default::default(),
            )),
        });

    // Act
    let key = PieceGroupKey::try_new(params).expect("a 32-byte secret is admitted");

    // Assert
    assert_eq!(
        key.expose(),
        &[
            0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21, 0x02, 0xe3,
            0xc4, 0xa5, 0x86, 0x67,
        ]
    );
}

/// Contract: `WrappedPieceGroupKey::try_new`, wrong length: the bytes' length
///   is not `PIECE_GROUP_KEY_LENGTH` → `Err(WrappedPieceGroupKeyErrorReturn::
///   WrongLength { expected, actual })`.
/// Arrange: `build_wrapped_piece_group_key_constructor_params` overriding
///   `bytes` with a vector of 31 bytes, one below the key length, so a
///   comparison that admits shorter bytes fails.
/// Act:     `WrappedPieceGroupKey::try_new(params)`.
/// Assert:  `result.err()` equals
///   `Some(WrappedPieceGroupKeyErrorReturn::WrongLength { expected: 32,
///   actual: 31 })` by `assert_eq!`.
/// Collaborators: none; the fixture is
///   `build_wrapped_piece_group_key_constructor_params`.
#[test]
fn wrapped_piece_group_key_try_new_refuses_bytes_shorter_than_the_key_length() {
    // Arrange
    let params = build_wrapped_piece_group_key_constructor_params(
        WrappedPieceGroupKeyConstructorParamsOverrides {
            bytes: Some(vec![0x3c; 31]),
        },
    );

    // Act
    let result = WrappedPieceGroupKey::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(WrappedPieceGroupKeyErrorReturn::WrongLength {
            expected: 32,
            actual: 31
        })
    );
}

/// Contract: `WrappedPieceGroupKey::try_new`, wrong length, as
///   `wrapped_piece_group_key_try_new_refuses_bytes_shorter_than_the_key_
///   length`.
/// Arrange: `build_wrapped_piece_group_key_constructor_params` overriding
///   `bytes` with a vector of 33 bytes, one above the key length, so a
///   comparison that admits longer bytes fails.
/// Act:     `WrappedPieceGroupKey::try_new(params)`.
/// Assert:  `result.err()` equals
///   `Some(WrappedPieceGroupKeyErrorReturn::WrongLength { expected: 32,
///   actual: 33 })` by `assert_eq!`.
/// Collaborators: none; the fixture is
///   `build_wrapped_piece_group_key_constructor_params`.
#[test]
fn wrapped_piece_group_key_try_new_refuses_bytes_longer_than_the_key_length() {
    // Arrange
    let params = build_wrapped_piece_group_key_constructor_params(
        WrappedPieceGroupKeyConstructorParamsOverrides {
            bytes: Some(vec![0x3c; 33]),
        },
    );

    // Act
    let result = WrappedPieceGroupKey::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(WrappedPieceGroupKeyErrorReturn::WrongLength {
            expected: 32,
            actual: 33
        })
    );
}

/// Contract: `WrappedPieceGroupKey::try_new`, admitted: the bytes' length is
///   `PIECE_GROUP_KEY_LENGTH` → `Ok(WrappedPieceGroupKey)` holding the bytes.
/// Arrange: `build_wrapped_piece_group_key_constructor_params` overriding
///   `bytes` with a literal vector of 32 pairwise distinct bytes, so a
///   truncation or a reordering differs from the bytes given.
/// Act:     `WrappedPieceGroupKey::try_new(params)`.
/// Assert:  the success arm is bound through `expect`; `wrapped.as_bytes()`
///   equals the literal array of those 32 bytes by `assert_eq!`.
/// Collaborators: none; the fixture is
///   `build_wrapped_piece_group_key_constructor_params`.
#[test]
fn wrapped_piece_group_key_try_new_holds_the_admitted_bytes() {
    // Arrange
    let params = build_wrapped_piece_group_key_constructor_params(
        WrappedPieceGroupKeyConstructorParamsOverrides {
            bytes: Some(vec![
                0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21, 0x02, 0xe3,
                0xc4, 0xa5, 0x86, 0x67,
            ]),
        },
    );

    // Act
    let wrapped = WrappedPieceGroupKey::try_new(params).expect("32 bytes are admitted");

    // Assert
    assert_eq!(
        wrapped.as_bytes(),
        &[
            0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0xf0, 0xe2, 0xd4, 0xc6, 0xb8, 0x9a, 0x7c, 0x5e, 0x40, 0x21, 0x02, 0xe3,
            0xc4, 0xa5, 0x86, 0x67,
        ]
    );
}
