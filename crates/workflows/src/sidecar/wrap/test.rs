#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    PIECE_GROUP_KEY_LENGTH, PieceGroupKey, PieceGroupKeyErrorReturn, WrapPieceGroupKeyDeps,
    WrapPieceGroupKeyErrorReturn, WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload,
    WrappedPieceGroupKey, WrappedPieceGroupKeyErrorReturn,
};
use super::mock::{PieceGroupKeyOverrides, build_piece_group_key};
use super::wrap_piece_group_key;
use core::cell::RefCell;
use domain::{
    DerivationContextConstructorParamsOverrides, SecretConstructorParamsOverrides,
    build_derivation_context, build_secret,
};
use encoding::{
    ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodeParams, EncodeReturn, EncodeSuccessReturn,
    EncodingDeclaration, EncodingIdentifier, IEncoderAdapter, IEncodingContract,
};
use kdf::{
    DerivationPurpose, DeriveKeyParams, DeriveKeyPayload, DeriveKeyReturn, DeriveKeySuccessReturn,
    IKeyDerivationAdapter, MockIKeyDerivationAdapter, build_kdf_declaration,
};
use kem::{EncapsulatedValueOverrides, build_encapsulated_value};

struct FixedEncoder;

impl IEncoderAdapter for FixedEncoder {
    const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        identifier: EncodingIdentifier::EthereumAbiV1,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
    };

    fn encode<D: IEncodingContract>(
        &self,
        _params: EncodeParams<'_, D>,
        _payload: &D::Described,
    ) -> EncodeReturn {
        Ok(EncodeSuccessReturn {
            bytes: b"encoded-context".to_vec(),
        })
    }
}

struct RefusingEncoder;

impl IEncoderAdapter for RefusingEncoder {
    const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        identifier: EncodingIdentifier::EthereumAbiV1,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
    };

    fn encode<D: IEncodingContract>(
        &self,
        _params: EncodeParams<'_, D>,
        _payload: &D::Described,
    ) -> EncodeReturn {
        Err(EncodeErrorReturn::FieldCount {
            expected: 10,
            actual: 9,
        })
    }
}

struct FixedKdf;

impl IKeyDerivationAdapter for FixedKdf {
    fn declaration(&self) -> kdf::KdfDeclaration {
        build_kdf_declaration(Default::default())
    }

    fn derive_key(
        &self,
        params: DeriveKeyParams,
        _payload: DeriveKeyPayload<'_>,
    ) -> DeriveKeyReturn {
        Ok(DeriveKeySuccessReturn {
            key: build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x0f; params.length]),
            }),
        })
    }
}

struct RecordedDerivation {
    is_wrapping_key: bool,
    length: usize,
    key_material: Vec<u8>,
    context: Vec<u8>,
}

struct RecordingKdf {
    calls: RefCell<Vec<RecordedDerivation>>,
}

impl IKeyDerivationAdapter for RecordingKdf {
    fn declaration(&self) -> kdf::KdfDeclaration {
        build_kdf_declaration(Default::default())
    }

    fn derive_key(
        &self,
        params: DeriveKeyParams,
        payload: DeriveKeyPayload<'_>,
    ) -> DeriveKeyReturn {
        self.calls.borrow_mut().push(RecordedDerivation {
            is_wrapping_key: matches!(params.purpose, DerivationPurpose::WrappingKey),
            length: params.length,
            key_material: payload.key_material.expose().clone(),
            context: payload.context.to_vec(),
        });
        Ok(DeriveKeySuccessReturn {
            key: build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x0f; params.length]),
            }),
        })
    }
}

/// Contract: the wrapped key is the key XORed with the wrapping key byte by
///   byte.
/// Arrange: `FixedKdf` deriving `0x0f`-bytes, and the piece-group key
///   `vec![0x33; 32]`.
/// Act:     `wrap_piece_group_key` over the payload.
/// Assert:  `success.wrapped.as_bytes()` equals `[0x3c; 32]`.
#[test]
fn wrap_xors_the_piece_group_key_with_the_derived_wrapping_key() {
    // Arrange
    let kdf = FixedKdf;
    let encoder = FixedEncoder;
    let deps = WrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let piece_group_key = build_piece_group_key(PieceGroupKeyOverrides {
        bytes: Some(vec![0x33; 32]),
    });

    // Act
    let Ok(success) = wrap_piece_group_key(
        &deps,
        WrapPieceGroupKeyParams,
        WrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            piece_group_key: &piece_group_key,
        },
    ) else {
        panic!("the wrap succeeds")
    };

    // Assert
    assert_eq!(success.wrapped.as_bytes(), &[0x3c; 32]);
}

/// Contract: the derivation is the wrapping-key purpose at the admitted
///   key's length.
/// Arrange: `RecordingKdf` and a 32-byte piece-group key.
/// Act:     `wrap_piece_group_key` over the payload.
/// Assert:  the one recorded call's `is_wrapping_key` is `true` and its
///   `length` equals `PIECE_GROUP_KEY_LENGTH`.
#[test]
fn wrap_derives_the_wrapping_key_under_its_purpose_at_the_piece_group_keys_length() {
    // Arrange
    let kdf = RecordingKdf {
        calls: RefCell::new(Vec::new()),
    };
    let encoder = FixedEncoder;
    let deps = WrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let piece_group_key = build_piece_group_key(PieceGroupKeyOverrides {
        bytes: Some(vec![0x33; 32]),
    });

    // Act
    let Ok(_success) = wrap_piece_group_key(
        &deps,
        WrapPieceGroupKeyParams,
        WrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            piece_group_key: &piece_group_key,
        },
    ) else {
        panic!("the wrap succeeds")
    };

    // Assert
    let calls = kdf.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert!(calls[0].is_wrapping_key);
    assert_eq!(calls[0].length, PIECE_GROUP_KEY_LENGTH);
}

/// Contract: the key material is the encapsulated value's bytes and the
///   context is the encoder's output.
/// Arrange: `RecordingKdf`, the encapsulated value `0x42`-bytes, and the
///   piece-group key `vec![0x33; 32]`.
/// Act:     `wrap_piece_group_key` over the payload.
/// Assert:  the one recorded call's `key_material` equals `vec![0x42; 32]`
///   and its `context` equals `b"encoded-context".to_vec()`.
#[test]
fn wrap_derives_from_the_encapsulated_value_and_the_encoded_context() {
    // Arrange
    let kdf = RecordingKdf {
        calls: RefCell::new(Vec::new()),
    };
    let encoder = FixedEncoder;
    let deps = WrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let piece_group_key = build_piece_group_key(PieceGroupKeyOverrides {
        bytes: Some(vec![0x33; 32]),
    });

    // Act
    let Ok(_success) = wrap_piece_group_key(
        &deps,
        WrapPieceGroupKeyParams,
        WrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            piece_group_key: &piece_group_key,
        },
    ) else {
        panic!("the wrap succeeds")
    };

    // Assert
    let calls = kdf.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].key_material, vec![0x42; 32]);
    assert_eq!(calls[0].context, b"encoded-context".to_vec());
}

/// Contract: a derivation returning another length is refused before any
///   byte is combined.
/// Arrange: `MockIKeyDerivationAdapter`, whose key is empty, and the
///   admitted piece-group key `[0x33; 32]`.
/// Act:     `wrap_piece_group_key` over the payload.
/// Assert:  `error` matches `WrapPieceGroupKeyErrorReturn::WrappingKeyLength
///   { expected: 32, actual: 0 }`.
#[test]
fn wrap_refuses_a_wrapping_key_of_another_length() {
    // Arrange
    let kdf = MockIKeyDerivationAdapter;
    let encoder = FixedEncoder;
    let deps = WrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let piece_group_key = build_piece_group_key(PieceGroupKeyOverrides {
        bytes: Some(vec![0x33; 32]),
    });

    // Act
    let Err(error) = wrap_piece_group_key(
        &deps,
        WrapPieceGroupKeyParams,
        WrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            piece_group_key: &piece_group_key,
        },
    ) else {
        panic!("the short derivation is refused")
    };

    // Assert
    let WrapPieceGroupKeyErrorReturn::WrappingKeyLength { expected, actual } = error else {
        panic!("the refusal is the wrapping-key length")
    };
    assert_eq!(expected, PIECE_GROUP_KEY_LENGTH);
    assert_eq!(actual, 0);
}

/// Contract: a secret shorter than the piece-group key's length is refused,
///   and an admitted key exposes exactly its bytes.
/// Arrange: a 31-byte secret, and a 32-byte secret of `0x77`.
/// Act:     `PieceGroupKey::try_from_secret_bytes` over each.
/// Assert:  the 31-byte secret yields `WrongLength { expected: 32,
///   actual: 31 }`; the admitted key's `expose()` equals `[0x77; 32]`.
#[test]
fn piece_group_key_rejects_a_short_secret() {
    // Arrange
    let short = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x77; 31]),
    });
    let valid = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x77; 32]),
    });

    // Act
    let Err(error) = PieceGroupKey::try_from_secret_bytes(short) else {
        panic!("the short secret is refused")
    };
    let Ok(key) = PieceGroupKey::try_from_secret_bytes(valid) else {
        panic!("the 32-byte secret is admitted")
    };

    // Assert
    let PieceGroupKeyErrorReturn::WrongLength { expected, actual } = error;
    assert_eq!(expected, PIECE_GROUP_KEY_LENGTH);
    assert_eq!(actual, 31);
    let bytes: &[u8; 32] = key.expose();
    assert_eq!(bytes, &[0x77; 32]);
}

/// Contract: a sidecar value shorter than the piece-group key's length is
///   refused, and an admitted value exposes exactly its bytes.
/// Arrange: 31 sidecar bytes, and 32 sidecar bytes of `0x5a`.
/// Act:     `WrappedPieceGroupKey::try_from_bytes` over each.
/// Assert:  the 31-byte value yields `WrongLength { expected: 32,
///   actual: 31 }`; the admitted value's `as_bytes()` equals `[0x5a; 32]`.
#[test]
fn wrapped_piece_group_key_rejects_a_short_sidecar_value() {
    // Arrange
    let short = vec![0x5a; 31];
    let valid = vec![0x5a; 32];

    // Act
    let Err(error) = WrappedPieceGroupKey::try_from_bytes(short) else {
        panic!("the short sidecar value is refused")
    };
    let Ok(wrapped) = WrappedPieceGroupKey::try_from_bytes(valid) else {
        panic!("the 32-byte sidecar value is admitted")
    };

    // Assert
    let WrappedPieceGroupKeyErrorReturn::WrongLength { expected, actual } = error;
    assert_eq!(expected, PIECE_GROUP_KEY_LENGTH);
    assert_eq!(actual, 31);
    let bytes: &[u8; 32] = wrapped.as_bytes();
    assert_eq!(bytes, &[0x5a; 32]);
}

/// Contract: an encoding refusal returns the error unchanged before any
///   key-derivation call.
/// Arrange: `RefusingEncoder` returning `EncodeErrorReturn::FieldCount
///   { expected: 10, actual: 9 }`, and `RecordingKdf` as the spy.
/// Act:     `wrap_piece_group_key` over the payload.
/// Assert:  the error is `WrapPieceGroupKeyErrorReturn::Encoding` holding
///   the refusal unchanged, and the KDF records no call.
#[test]
fn wrap_returns_an_encoder_contract_refusal() {
    // Arrange
    let kdf = RecordingKdf {
        calls: RefCell::new(Vec::new()),
    };
    let encoder = RefusingEncoder;
    let deps = WrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let piece_group_key = build_piece_group_key(PieceGroupKeyOverrides {
        bytes: Some(vec![0x33; 32]),
    });

    // Act
    let Err(error) = wrap_piece_group_key(
        &deps,
        WrapPieceGroupKeyParams,
        WrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            piece_group_key: &piece_group_key,
        },
    ) else {
        panic!("the encoding refusal propagates")
    };

    // Assert
    let WrapPieceGroupKeyErrorReturn::Encoding(inner) = error else {
        panic!("the refusal is the encoding arm")
    };
    assert_eq!(
        inner,
        EncodeErrorReturn::FieldCount {
            expected: 10,
            actual: 9
        }
    );
    assert!(kdf.calls.borrow().is_empty());
}
