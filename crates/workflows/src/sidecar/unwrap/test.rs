#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    UnwrapPieceGroupKeyDeps, UnwrapPieceGroupKeyErrorReturn, UnwrapPieceGroupKeyParams,
    UnwrapPieceGroupKeyPayload,
};
use super::unwrap_piece_group_key;
use crate::sidecar::wrap::provides::{
    PIECE_GROUP_KEY_LENGTH, PieceGroupKeyOverrides, WrapPieceGroupKeyDeps, WrapPieceGroupKeyParams,
    WrapPieceGroupKeyPayload, WrappedPieceGroupKeyOverrides, build_piece_group_key,
    build_wrapped_piece_group_key, wrap_piece_group_key,
};
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

/// Contract: the recovered key is the wrapped key XORed with the wrapping
///   key.
/// Arrange: `FixedKdf` deriving `0x0f`-bytes, and the wrapped key
///   `vec![0x3c; 32]`.
/// Act:     `unwrap_piece_group_key` over the payload.
/// Assert:  `success.piece_group_key.expose()` equals `[0x33; 32]`.
#[test]
fn unwrap_xors_the_wrapped_key_with_the_derived_wrapping_key() {
    // Arrange
    let kdf = FixedKdf;
    let encoder = FixedEncoder;
    let deps = UnwrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyOverrides {
        bytes: Some(vec![0x3c; 32]),
    });

    // Act
    let Ok(success) = unwrap_piece_group_key(
        &deps,
        UnwrapPieceGroupKeyParams,
        UnwrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrapped,
        },
    ) else {
        panic!("the unwrap succeeds")
    };

    // Assert
    assert_eq!(success.piece_group_key.expose(), &[0x33; 32]);
}

/// Contract: the derivation is the wrapping-key purpose at the wrapped
///   key's length.
/// Arrange: `RecordingKdf` and the admitted wrapped key `vec![0x3c; 32]`.
/// Act:     `unwrap_piece_group_key` over the payload.
/// Assert:  the one recorded call's `is_wrapping_key` is `true` and its
///   `length` equals `PIECE_GROUP_KEY_LENGTH`.
#[test]
fn unwrap_derives_the_wrapping_key_under_its_purpose_at_the_wrapped_keys_length() {
    // Arrange
    let kdf = RecordingKdf {
        calls: RefCell::new(Vec::new()),
    };
    let encoder = FixedEncoder;
    let deps = UnwrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyOverrides {
        bytes: Some(vec![0x3c; 32]),
    });

    // Act
    let Ok(_success) = unwrap_piece_group_key(
        &deps,
        UnwrapPieceGroupKeyParams,
        UnwrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrapped,
        },
    ) else {
        panic!("the unwrap succeeds")
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
///   wrapped key `vec![0x3c; 32]`.
/// Act:     `unwrap_piece_group_key` over the payload.
/// Assert:  the one recorded call's `key_material` equals `vec![0x42; 32]`
///   and its `context` equals `b"encoded-context".to_vec()`.
#[test]
fn unwrap_derives_from_the_encapsulated_value_and_the_encoded_context() {
    // Arrange
    let kdf = RecordingKdf {
        calls: RefCell::new(Vec::new()),
    };
    let encoder = FixedEncoder;
    let deps = UnwrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyOverrides {
        bytes: Some(vec![0x3c; 32]),
    });

    // Act
    let Ok(_success) = unwrap_piece_group_key(
        &deps,
        UnwrapPieceGroupKeyParams,
        UnwrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrapped,
        },
    ) else {
        panic!("the unwrap succeeds")
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
///   admitted wrapped key `vec![0x3c; 32]`.
/// Act:     `unwrap_piece_group_key` over the payload.
/// Assert:  `error` matches `UnwrapPieceGroupKeyErrorReturn::
///   WrappingKeyLength { expected: 32, actual: 0 }`.
#[test]
fn unwrap_refuses_a_wrapping_key_of_another_length() {
    // Arrange
    let kdf = MockIKeyDerivationAdapter;
    let encoder = FixedEncoder;
    let deps = UnwrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyOverrides {
        bytes: Some(vec![0x3c; 32]),
    });

    // Act
    let Err(error) = unwrap_piece_group_key(
        &deps,
        UnwrapPieceGroupKeyParams,
        UnwrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrapped,
        },
    ) else {
        panic!("the short derivation is refused")
    };

    // Assert
    let UnwrapPieceGroupKeyErrorReturn::WrappingKeyLength { expected, actual } = error else {
        panic!("the refusal is the wrapping-key length")
    };
    assert_eq!(expected, PIECE_GROUP_KEY_LENGTH);
    assert_eq!(actual, 0);
}

/// Contract: an encoding refusal returns the error unchanged before any
///   key-derivation call.
/// Arrange: `RefusingEncoder` returning `EncodeErrorReturn::FieldCount
///   { expected: 10, actual: 9 }`, and `RecordingKdf` as the spy.
/// Act:     `unwrap_piece_group_key` over the payload.
/// Assert:  the error is `UnwrapPieceGroupKeyErrorReturn::Encoding` holding
///   the refusal unchanged, and the KDF records no call.
#[test]
fn unwrap_returns_an_encoder_contract_refusal() {
    // Arrange
    let kdf = RecordingKdf {
        calls: RefCell::new(Vec::new()),
    };
    let encoder = RefusingEncoder;
    let deps = UnwrapPieceGroupKeyDeps {
        kdf: &kdf,
        encoder: &encoder,
    };
    let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
        bytes: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0x42; 32]),
        })),
    });
    let context = build_derivation_context(DerivationContextConstructorParamsOverrides::default());
    let wrapped = build_wrapped_piece_group_key(WrappedPieceGroupKeyOverrides {
        bytes: Some(vec![0x3c; 32]),
    });

    // Act
    let Err(error) = unwrap_piece_group_key(
        &deps,
        UnwrapPieceGroupKeyParams,
        UnwrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrapped,
        },
    ) else {
        panic!("the encoding refusal propagates")
    };

    // Assert
    let UnwrapPieceGroupKeyErrorReturn::Encoding(inner) = error else {
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

/// Contract: unwrap inverts wrap under the same derivation — unwrapping
///   what the wrap produced under one encapsulated value and context
///   returns the original key.
/// Arrange: `FixedKdf` as both calls' key derivation, an admitted
///   `PieceGroupKey` of `[0x9a; 32]`, and its wrap through
///   `wrap_piece_group_key` under the same encapsulated value and context.
/// Act:     `unwrap_piece_group_key` over the wrapped key.
/// Assert:  `success.piece_group_key.expose()` equals `[0x9a; 32]`.
#[test]
fn unwrap_returns_the_key_the_wrap_wrapped() {
    // Arrange
    let kdf = FixedKdf;
    let encoder = FixedEncoder;
    let deps = UnwrapPieceGroupKeyDeps {
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
        bytes: Some(vec![0x9a; 32]),
    });
    let Ok(wrap_success) = wrap_piece_group_key(
        &WrapPieceGroupKeyDeps {
            kdf: &kdf,
            encoder: &encoder,
        },
        WrapPieceGroupKeyParams,
        WrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            piece_group_key: &piece_group_key,
        },
    ) else {
        panic!("the wrap succeeds")
    };

    // Act
    let Ok(success) = unwrap_piece_group_key(
        &deps,
        UnwrapPieceGroupKeyParams,
        UnwrapPieceGroupKeyPayload {
            encapsulated: &encapsulated,
            context: &context,
            wrapped: &wrap_success.wrapped,
        },
    ) else {
        panic!("the unwrap succeeds")
    };

    // Assert
    assert_eq!(success.piece_group_key.expose(), &[0x9a; 32]);
}
