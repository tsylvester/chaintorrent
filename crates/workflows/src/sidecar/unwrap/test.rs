#![allow(clippy::expect_used)]

use super::provides::{
    IUnwrapPieceGroupKeyCaseConsumer, UnwrapPieceGroupKeyCaseParamsOverrides,
    UnwrapPieceGroupKeyCasePayload, UnwrapPieceGroupKeyErrorReturn,
    build_unwrap_piece_group_key_case_deps, build_unwrap_piece_group_key_case_params,
    build_unwrap_piece_group_key_case_payload, build_unwrap_piece_group_key_deps,
    build_unwrap_piece_group_key_params, build_unwrap_piece_group_key_payload,
    run_unwrap_piece_group_key_case, unwrap_piece_group_key,
};
use crate::sidecar::derivation_context::provides::{
    DerivationContext, build_derivation_context, build_derivation_context_description,
};
use crate::sidecar::wrap::provides::{
    WrappedPieceGroupKey, WrappedPieceGroupKeyConstructorParamsOverrides,
    build_wrapped_piece_group_key,
};
use domain::build_secret;
use encoding::{EncodeParams, IDecoderAdapter, IEncoderAdapter};
use hash_to_scalar::IHashToScalarAdapter;
use kdf::{
    CreateKeyDerivationParamsOverrides, DerivationPurpose, DeriveKeyErrorReturn,
    DeriveKeyParamsOverrides, DeriveKeyPayloadOverrides, IKeyDerivationAdapter, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_params, build_derive_key_params, build_derive_key_payload,
};
use kem::{
    EncapsulatedValue, EncapsulatedValueOverrides, ICredentialKemAdapter, IdentityScope,
    build_encapsulated_value,
};
use pairing::{IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

/// Contract: `unwrap_piece_group_key`, key derivation refused:
///   `deps.kdf.derive_key(...)`, called once, returns `Err(error)` →
///   `Err(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(error))`, the refusal
///   unchanged.
/// Arrange: `params` from `build_unwrap_piece_group_key_case_params` with the
///   `key_derivation` override set to `build_create_key_derivation_params`
///   overriding `concrete` with `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`; `build_encapsulated_value` overriding
///   `key_material` with `build_secret` over a literal vector of 48 bytes, a
///   length that differs from the wrapped key's, so a derivation fed other
///   material reports another length; `build_derivation_context` and
///   `build_wrapped_piece_group_key` at their defaults, held by the block's
///   consumer.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)`, whose
///   consumer calls `unwrap_piece_group_key` once.
/// Assert:  the consumer's method asserts the call's `err()` equals
///   `Some(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length: 48 })))` by `assert_eq!`, the
///   length a literal; the block asserts `result.is_ok()`.
/// Collaborators: the encoder is the encoding family's mock concrete and the
///   KDF is the key-derivation family's mock concrete under the failure mode
///   named, each reached through the case runner by the block's selections;
///   no block names a concrete.
#[test]
fn unwrap_returns_the_key_derivation_refusal_unchanged() {
    // Arrange
    struct Consumer {
        encapsulated: EncapsulatedValue,
        context: DerivationContext,
        wrapped: WrappedPieceGroupKey,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer {
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
                    &self.encapsulated,
                    &self.context,
                    &self.wrapped,
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

    let consumer = Consumer {
        encapsulated: build_encapsulated_value(EncapsulatedValueOverrides {
            key_material: Some(build_secret(vec![0xa7; 48], Default::default())),
        }),
        context: build_derivation_context(Default::default()),
        wrapped: build_wrapped_piece_group_key(Default::default()),
    };
    let deps = build_unwrap_piece_group_key_case_deps(consumer, Default::default());
    let params = build_unwrap_piece_group_key_case_params(UnwrapPieceGroupKeyCaseParamsOverrides {
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

/// Contract: `unwrap_piece_group_key`, unwrapped: the derived length is
///   `PIECE_GROUP_KEY_LENGTH` and the admission returns `Ok(piece_group_key)`
///   → `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })`,
///   `piece_group_key` the byte-wise XOR of the wrapped key and the wrapping
///   key.
/// Arrange: `build_encapsulated_value` overriding `key_material` with
///   `build_secret` over a literal nonempty byte vector;
///   `build_derivation_context` at its defaults; `params` from
///   `build_unwrap_piece_group_key_case_params` with no override; the
///   consumer's method computes the context bytes as the `bytes` of the
///   success arm of `encoder.encode(EncodeParams { description:
///   &build_derivation_context_description(Default::default()) },
///   &self.context)`, bound irrefutably, and the wrapping key as the `key` of
///   `kdf.derive_key` over `build_derive_key_params` overriding `purpose`
///   with `DerivationPurpose::WrappingKey` and `length` with `32` and
///   `build_derive_key_payload` overriding `key_material` with the
///   encapsulated value's key material and `context` with those bytes, and
///   builds the `wrapped` key inside by `build_wrapped_piece_group_key`
///   overriding `bytes` with the wrapping key's exposed bytes, so the XOR
///   cancels and a wrapping key derived under another purpose, length, key
///   material, or context leaves a nonzero result.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)`, whose
///   consumer calls `unwrap_piece_group_key` once.
/// Assert:  the consumer's method asserts `piece_group_key.expose()` equals
///   `&[0u8; 32]` by `assert_eq!`, the expectation a literal and not read
///   from the arrangement; the block asserts `result.is_ok()`.
/// Collaborators: as `unwrap_returns_the_key_derivation_refusal_unchanged`,
///   with the KDF under its default `NoFailure` mode.
#[test]
fn unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_wrapping_key() {
    // Arrange
    struct Consumer {
        encapsulated: EncapsulatedValue,
        context: DerivationContext,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer {
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
                    &self.context,
                )
                .expect("the mock encoder encodes the context")
                .bytes;
            let wrapping_key = kdf
                .derive_key(
                    build_derive_key_params(DeriveKeyParamsOverrides {
                        purpose: Some(DerivationPurpose::WrappingKey),
                        length: Some(32),
                    }),
                    build_derive_key_payload(DeriveKeyPayloadOverrides {
                        key_material: Some(self.encapsulated.key_material()),
                        context: Some(context_bytes.as_slice()),
                    }),
                )
                .expect("the mock KDF derives under the wrapping-key purpose")
                .key;
            let wrapped =
                build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
                    bytes: Some(wrapping_key.expose().clone()),
                });
            let result = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    &self.encapsulated,
                    &self.context,
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

    let consumer = Consumer {
        encapsulated: build_encapsulated_value(EncapsulatedValueOverrides {
            key_material: Some(build_secret(
                vec![0x19, 0x2b, 0x3d, 0x4f, 0x51],
                Default::default(),
            )),
        }),
        context: build_derivation_context(Default::default()),
    };
    let deps = build_unwrap_piece_group_key_case_deps(consumer, Default::default());
    let params = build_unwrap_piece_group_key_case_params(Default::default());
    let payload = build_unwrap_piece_group_key_case_payload(Default::default());

    // Act
    let result = run_unwrap_piece_group_key_case(&deps, params, payload);

    // Assert
    assert!(result.is_ok());
}

/// Contract: `unwrap_piece_group_key`, unwrapped, as
///   `unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_wrapping_key`.
/// Arrange: the encapsulated value, the context, and the params as the block
///   `unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_wrapping_
///   key`, the context bytes and the wrapping key computed in the consumer's
///   method as the block above; the consumer holds
///   `build_wrapped_piece_group_key` overriding `bytes` with 32 bytes each
///   `0xFF`, so an unwrap that returns the wrapping key whatever the wrapped
///   key, or returns zeros, differs from the complement.
/// Act:     `run_unwrap_piece_group_key_case(&deps, params, payload)`, whose
///   consumer calls `unwrap_piece_group_key` once.
/// Assert:  the consumer's method asserts `piece_group_key.expose()` equals
///   the array whose byte at each index is the bitwise complement of the
///   wrapping key's byte at that index, by `assert_eq!`; the block asserts
///   `result.is_ok()`.
/// Collaborators: as
///   `unwrap_returns_a_zero_key_when_the_wrapped_key_equals_the_wrapping_key`.
#[test]
fn unwrap_returns_the_complement_of_the_wrapping_key_when_the_wrapped_key_is_all_ones() {
    // Arrange
    struct Consumer {
        encapsulated: EncapsulatedValue,
        context: DerivationContext,
        wrapped: WrappedPieceGroupKey,
    }

    impl IUnwrapPieceGroupKeyCaseConsumer for Consumer {
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
                    &self.context,
                )
                .expect("the mock encoder encodes the context")
                .bytes;
            let wrapping_key = kdf
                .derive_key(
                    build_derive_key_params(DeriveKeyParamsOverrides {
                        purpose: Some(DerivationPurpose::WrappingKey),
                        length: Some(32),
                    }),
                    build_derive_key_payload(DeriveKeyPayloadOverrides {
                        key_material: Some(self.encapsulated.key_material()),
                        context: Some(context_bytes.as_slice()),
                    }),
                )
                .expect("the mock KDF derives under the wrapping-key purpose")
                .key;
            let mut expected = [0u8; 32];
            for (slot, byte) in expected.iter_mut().zip(wrapping_key.expose().iter()) {
                *slot = !byte;
            }
            let result = unwrap_piece_group_key(
                &build_unwrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_unwrap_piece_group_key_params(Default::default()),
                build_unwrap_piece_group_key_payload(
                    &self.encapsulated,
                    &self.context,
                    &self.wrapped,
                    Default::default(),
                ),
            );
            let piece_group_key = result
                .expect("the unwrap admits the borrowed inputs")
                .piece_group_key;
            assert_eq!(piece_group_key.expose(), &expected);
        }
    }

    let consumer = Consumer {
        encapsulated: build_encapsulated_value(EncapsulatedValueOverrides {
            key_material: Some(build_secret(
                vec![0x19, 0x2b, 0x3d, 0x4f, 0x51],
                Default::default(),
            )),
        }),
        context: build_derivation_context(Default::default()),
        wrapped: build_wrapped_piece_group_key(WrappedPieceGroupKeyConstructorParamsOverrides {
            bytes: Some(vec![0xFF; 32]),
        }),
    };
    let deps = build_unwrap_piece_group_key_case_deps(consumer, Default::default());
    let params = build_unwrap_piece_group_key_case_params(Default::default());
    let payload = build_unwrap_piece_group_key_case_payload(Default::default());

    // Act
    let result = run_unwrap_piece_group_key_case(&deps, params, payload);

    // Assert
    assert!(result.is_ok());
}
