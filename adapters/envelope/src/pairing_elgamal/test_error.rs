#![allow(clippy::expect_used)]

use crate::factory::provides::{
    GenerateKeysErrorReturn, GenerateKeysPayloadOverrides, IKeyAgreementAdapter,
    KeyPairComponentsOverrides, KeyPairFromComponentsErrorReturn,
    KeyPairFromComponentsPayloadOverrides, PublicKeysComponentsOverrides,
    PublicKeysFromComponentsErrorReturn, PublicKeysFromComponentsPayloadOverrides,
    WrapToErrorReturn, build_generate_keys_params, build_generate_keys_payload,
    build_key_pair_components, build_key_pair_from_components_params,
    build_key_pair_from_components_payload, build_public_keys_components,
    build_public_keys_from_components_params, build_public_keys_from_components_payload,
    build_wrap_to_params, build_wrap_to_payload,
};
use crate::pairing_elgamal::provides::{
    PairingElGamalGenerateKeysErrorReturn, PairingElGamalKeyPairOverrides,
    PairingElGamalPossessionOverrides, PairingElGamalPublicKeysFromComponentsErrorReturn,
    PairingElGamalWrapToErrorReturn, build_pairing_elgamal_key_agreement,
    build_pairing_elgamal_key_pair, build_pairing_elgamal_possession,
    build_pairing_elgamal_public_keys,
};
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, MulG1Params, MulG1PayloadOverrides,
    MulG2Params, MulG2PayloadOverrides, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayloadOverrides, build_create_pairing_deps, build_create_pairing_params,
    build_mul_g1_payload, build_mul_g2_payload, build_sample_uniform_scalar_payload,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_params, create_random_source,
};

/// Contract: an `x` draw the scalar type refuses returns
///   `Err(GenerateKeysErrorReturn::XSampling(error))` with the refusal
///   unchanged.
/// Arrange: `build_generate_keys_payload` with `x_uniform` overridden to
///   `build_secret` over one byte fewer than `<P::Scalar as
///   ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`, read in the consumer.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(), Some(GenerateKeysErrorReturn::XSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual })))` with
///   `expected` the bound and `actual` the bound less one.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_returns_the_x_sampling_refusal_unchanged() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0x0Du8; bound - 1],
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(GenerateKeysErrorReturn::XSampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: bound,
                                actual: bound - 1,
                            }
                        ))
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_returns_the_x_sampling_refusal_unchanged

/// Contract: a `y` draw the scalar type refuses returns
///   `Err(GenerateKeysErrorReturn::YSampling(error))` with the refusal
///   unchanged.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden as
///   `x_uniform` is in `generate_keys_returns_the_x_sampling_refusal_unchanged`.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(), Some(GenerateKeysErrorReturn::YSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual })))`.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_returns_the_y_sampling_refusal_unchanged() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            y_uniform: Some(build_secret(
                                vec![0x0Eu8; bound - 1],
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(GenerateKeysErrorReturn::YSampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: bound,
                                actual: bound - 1,
                            }
                        ))
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_returns_the_y_sampling_refusal_unchanged

/// Contract: with both draws refused, the `x` draw is sampled first and its
///   refusal returned.
/// Arrange: `build_generate_keys_payload` with `x_uniform` one byte short and
///   `y_uniform` two bytes short, so the refusals differ.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(), Some(GenerateKeysErrorReturn::XSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual })))` with
///   `actual` the bound less one.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_refuses_the_x_draw_before_the_y_draw() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0x0Du8; bound - 1],
                                SecretConstructorParamsOverrides,
                            )),
                            y_uniform: Some(build_secret(
                                vec![0x0Eu8; bound - 2],
                                SecretConstructorParamsOverrides,
                            )),
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(GenerateKeysErrorReturn::XSampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: bound,
                                actual: bound - 1,
                            }
                        ))
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_refuses_the_x_draw_before_the_y_draw

/// Contract: draws that sample, with `pk1 = mul_g1(g1, x)` the identity
///   element, return `Err(GenerateKeysErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_generate_keys_payload` with `x_uniform` overridden to
///   `build_secret` over zero bytes of the bound's length and `y_uniform` at
///   its default, so only `pk1` is the identity.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG1))`.
/// Boundary: the private surface's `generate_keys`, with the mock pairing's
///   `is_identity_g1` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_refuses_a_zero_x_draw_as_an_identity_first_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG1));
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_refuses_a_zero_x_draw_as_an_identity_first_group_key

/// Contract: draws that sample, with `pk2 = mul_g2(g2, y)` the identity
///   element and `pk1` not, return `Err(GenerateKeysErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to
///   `build_secret` over zero bytes of the bound's length and `x_uniform` at
///   its default.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG2))`.
/// Boundary: the private surface's `generate_keys`, with the mock pairing's
///   `is_identity_g2` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_refuses_a_zero_y_draw_as_an_identity_second_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            y_uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG2));
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_refuses_a_zero_y_draw_as_an_identity_second_group_key

/// Contract: draws that sample to one nonzero scalar, so `e(pk1, g2) =
///   e(g1, pk2)`, return `Err(GenerateKeysErrorReturn::SharedSecret)`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to
///   `build_secret` over the bytes of the default `x_uniform`, which are
///   nonzero.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::SharedSecret))`.
/// Boundary: the private surface's `generate_keys`, with the mock pairing's
///   `pairing_product_is_one` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_refuses_equal_draws_as_a_shared_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            y_uniform: Some(build_secret(
                                vec![0x0Du8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::SharedSecret));
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_refuses_equal_draws_as_a_shared_secret

/// Contract: draws that make both keys the identity element, which also
///   satisfy the shared-secret relation, return the identity refusal of the
///   first group.
/// Arrange: `build_generate_keys_payload` with both draws overridden to
///   `build_secret` over zero bytes of the bound's length.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG1))`.
/// Boundary: the private surface's `generate_keys`, with the mock pairing's
///   identity checks deciding before `pairing_product_is_one` is reached.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_checks_the_identity_keys_before_the_shared_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            y_uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG1));
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_checks_the_identity_keys_before_the_shared_secret

/// Contract: draws whose key fails the identity check return the identity
///   refusal before any nonce is drawn.
/// Arrange: `build_create_random_source_params` with `kind` overridden to
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; `build_generate_keys_payload` with `x_uniform`
///   overridden to `build_secret` over zero bytes of the bound's length.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG1))`, so the refused draw is
///   never reached.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes,
///   the randomness concrete named `FillBytesRefused`.
#[test]
fn generate_keys_checks_the_keys_before_drawing_the_nonces() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::Mock(
                                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                            )),
                        }),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's mock concrete constructs under any failure mode");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG1));
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_checks_the_keys_before_drawing_the_nonces

/// Contract: keys that pass and a refused draw for the first nonce return
///   `Err(GenerateKeysErrorReturn::PairingElGamal(
///   PairingElGamalGenerateKeysErrorReturn::NonceDraw(error)))` with the
///   refusal unchanged.
/// Arrange: `build_create_random_source_params` with `kind` overridden to
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; `build_generate_keys_payload` at its defaults.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(), Some(GenerateKeysErrorReturn::
///   PairingElGamal(PairingElGamalGenerateKeysErrorReturn::NonceDraw(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))))`.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes,
///   the randomness concrete named `FillBytesRefused`.
#[test]
fn generate_keys_returns_the_nonce_draw_refusal_unchanged() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::Mock(
                                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                            )),
                        }),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's mock concrete constructs under any failure mode");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );

                    // Act
                    let result = adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(pairing, Default::default()),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(GenerateKeysErrorReturn::PairingElGamal(
                            PairingElGamalGenerateKeysErrorReturn::NonceDraw(
                                FillBytesErrorReturn::MockIRandomSourceAdapter
                            )
                        ))
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_returns_the_nonce_draw_refusal_unchanged

/// Contract: the challenge step's hash refusal, with keys that pass and
///   nonces drawn, is returned as `Err(GenerateKeysErrorReturn::PairingElGamal(
///   PairingElGamalGenerateKeysErrorReturn::HashToScalar(error)))` with the
///   refusal unchanged.
/// Arrange: `build_create_hash_to_scalar_params` with `concrete` overridden to
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; `build_generate_keys_payload` at its defaults.
/// Act:     `adapter.generate_keys`.
/// Assert:  `assert_eq!(result.err(), Some(GenerateKeysErrorReturn::
///   PairingElGamal(PairingElGamalGenerateKeysErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected, actual: 0 })))))` with `expected` the bound.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes,
///   the hash-to-scalar concrete named `SamplingRefused`.
#[test]
fn generate_keys_returns_the_hash_to_scalar_refusal_unchanged() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                            concrete: Some(HashToScalarConcrete::Mock(
                                MockIHashToScalarAdapterFailureMode::SamplingRefused,
                            )),
                            ..Default::default()
                        }),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's mock concrete constructs under any failure mode");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );

                    // Act
                    let result = adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(pairing, Default::default()),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(GenerateKeysErrorReturn::PairingElGamal(
                            PairingElGamalGenerateKeysErrorReturn::HashToScalar(
                                HashToScalarErrorReturn::Sampling(
                                    SampleUniformScalarErrorReturn::WrongLength {
                                        expected: bound,
                                        actual: 0,
                                    }
                                )
                            )
                        ))
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end generate_keys_returns_the_hash_to_scalar_refusal_unchanged

/// Contract: a refused draw for `ρ` returns `Err(WrapToErrorReturn::
///   PairingElGamal(PairingElGamalWrapToErrorReturn::CoinDraw(error)))` with
///   the refusal unchanged.
/// Arrange: `build_create_random_source_params` with `kind` overridden to
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; `build_pairing_elgamal_public_keys` and
///   `build_wrap_to_payload` at their defaults.
/// Act:     `adapter.wrap_to`.
/// Assert:  `assert_eq!(result.err(), Some(WrapToErrorReturn::PairingElGamal(
///   PairingElGamalWrapToErrorReturn::CoinDraw(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))))`.
/// Boundary: the private surface's `wrap_to`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes,
///   the randomness concrete named `FillBytesRefused`.
#[test]
fn wrap_to_returns_the_coin_draw_refusal_unchanged() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::Mock(
                                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                            )),
                        }),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's mock concrete constructs under any failure mode");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let public_keys =
                        build_pairing_elgamal_public_keys(pairing, Default::default());
                    let payload = build_wrap_to_payload(pairing, &public_keys, Default::default());

                    // Act
                    let result = adapter.wrap_to(build_wrap_to_params(Default::default()), payload);

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(WrapToErrorReturn::PairingElGamal(
                            PairingElGamalWrapToErrorReturn::CoinDraw(
                                FillBytesErrorReturn::MockIRandomSourceAdapter
                            )
                        ))
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end wrap_to_returns_the_coin_draw_refusal_unchanged

/// Contract: components whose `x` is the zero scalar, so `pk1` is the identity
///   element, return `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_key_pair_from_components_payload` with `components` carrying
///   `x` overridden to the `Secret` the scalar type samples from zero bytes of
///   the bound's length and `y` at its default.
/// Act:     `adapter.key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1))`.
/// Boundary: the private surface's `key_pair_from_components`, with the mock
///   pairing's `is_identity_g1` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn key_pair_from_components_refuses_a_zero_x_secret_as_an_identity_first_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let zero_x = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling a zero draw succeeds")
                    .scalar;
                    let payload = build_key_pair_from_components_payload(
                        pairing,
                        KeyPairFromComponentsPayloadOverrides {
                            components: Some(build_key_pair_components(
                                pairing,
                                KeyPairComponentsOverrides {
                                    x: Some(zero_x),
                                    ..Default::default()
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end key_pair_from_components_refuses_a_zero_x_secret_as_an_identity_first_group_key

/// Contract: components whose `y` is the zero scalar and whose `x` is not
///   return `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_key_pair_from_components_payload` with `components` carrying
///   `y` overridden to the `Secret` the scalar type samples from zero bytes of
///   the bound's length and `x` at its default.
/// Act:     `adapter.key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::IdentityKeyG2))`.
/// Boundary: the private surface's `key_pair_from_components`, with the mock
///   pairing's `is_identity_g2` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn key_pair_from_components_refuses_a_zero_y_secret_as_an_identity_second_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let zero_y = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling a zero draw succeeds")
                    .scalar;
                    let payload = build_key_pair_from_components_payload(
                        pairing,
                        KeyPairFromComponentsPayloadOverrides {
                            components: Some(build_key_pair_components(
                                pairing,
                                KeyPairComponentsOverrides {
                                    y: Some(zero_y),
                                    ..Default::default()
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(KeyPairFromComponentsErrorReturn::IdentityKeyG2)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end key_pair_from_components_refuses_a_zero_y_secret_as_an_identity_second_group_key

/// Contract: components whose `x` and `y` are one nonzero scalar return
///   `Err(KeyPairFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: `build_key_pair_from_components_payload` with `components` carrying
///   `y` overridden to a `Secret` of the scalar the default `x` holds.
/// Act:     `adapter.key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::SharedSecret))`.
/// Boundary: the private surface's `key_pair_from_components`, with the mock
///   pairing's `pairing_product_is_one` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn key_pair_from_components_refuses_a_shared_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let shared_y = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0x01u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling the shared scalar succeeds")
                    .scalar;
                    let payload = build_key_pair_from_components_payload(
                        pairing,
                        KeyPairFromComponentsPayloadOverrides {
                            components: Some(build_key_pair_components(
                                pairing,
                                KeyPairComponentsOverrides {
                                    y: Some(shared_y),
                                    ..Default::default()
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(KeyPairFromComponentsErrorReturn::SharedSecret)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end key_pair_from_components_refuses_a_shared_secret

/// Contract: components whose `x` and `y` are both the zero scalar return the
///   identity refusal of the first group.
/// Arrange: `build_key_pair_from_components_payload` with `x` and `y` both
///   overridden to `Secret`s of the zero scalar.
/// Act:     `adapter.key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1))`.
/// Boundary: the private surface's `key_pair_from_components`, with the mock
///   pairing's identity checks deciding before `pairing_product_is_one` is
///   reached.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn key_pair_from_components_checks_the_identity_keys_before_the_shared_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let zero_x = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling a zero draw succeeds")
                    .scalar;
                    let zero_y = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling a zero draw succeeds")
                    .scalar;
                    let payload = build_key_pair_from_components_payload(
                        pairing,
                        KeyPairFromComponentsPayloadOverrides {
                            components: Some(build_key_pair_components(
                                pairing,
                                KeyPairComponentsOverrides {
                                    x: Some(zero_x),
                                    y: Some(zero_y),
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end key_pair_from_components_checks_the_identity_keys_before_the_shared_secret

/// Contract: components whose `pk1` is the identity element and whose `pk2` is
///   not return `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG1)`.
/// Arrange: the adapter, the default key pair, and the default possession valid
///   for it; `build_public_keys_from_components_payload` with `components`
///   carrying `pk1` overridden to `mul_g1` of `g1_generator` by the zero scalar.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG1))`.
/// Boundary: the private surface's `public_keys_from_components`, with the mock
///   pairing's `is_identity_g1` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_refuses_an_identity_first_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let possession =
                        build_pairing_elgamal_possession(&adapter, &key_pair, Default::default());
                    let zero_scalar = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling a zero draw succeeds")
                    .scalar
                    .expose()
                    .clone();
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let identity_pk1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(zero_scalar),
                                },
                            ),
                        )
                        .expect("the identity multiple multiplies")
                        .product;
                    let payload = build_public_keys_from_components_payload(
                        pairing,
                        &possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(build_public_keys_components(
                                pairing,
                                PublicKeysComponentsOverrides {
                                    pk1: Some(identity_pk1),
                                    ..Default::default()
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG1)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_refuses_an_identity_first_group_key

/// Contract: components whose `pk2` is the identity element and whose `pk1` is
///   not return `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG2)`.
/// Arrange: as
///   `public_keys_from_components_refuses_an_identity_first_group_key`, with
///   `pk2` overridden to `mul_g2` of `g2_generator` by the zero scalar.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG2))`.
/// Boundary: the private surface's `public_keys_from_components`, with the mock
///   pairing's `is_identity_g2` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_refuses_an_identity_second_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let possession =
                        build_pairing_elgamal_possession(&adapter, &key_pair, Default::default());
                    let zero_scalar = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling a zero draw succeeds")
                    .scalar
                    .expose()
                    .clone();
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let identity_pk2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(zero_scalar),
                                },
                            ),
                        )
                        .expect("the identity multiple multiplies")
                        .product;
                    let payload = build_public_keys_from_components_payload(
                        pairing,
                        &possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(build_public_keys_components(
                                pairing,
                                PublicKeysComponentsOverrides {
                                    pk2: Some(identity_pk2),
                                    ..Default::default()
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG2)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_refuses_an_identity_second_group_key

/// Contract: components `pk1 = mul_g1(g1, s)` and `pk2 = mul_g2(g2, s)` for one
///   nonzero scalar `s` return
///   `Err(PublicKeysFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: as
///   `public_keys_from_components_refuses_an_identity_first_group_key`, with
///   `pk1` and `pk2` overridden to the multiples of the generators by one scalar
///   the scalar type samples from nonzero bytes.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::SharedSecret))`.
/// Boundary: the private surface's `public_keys_from_components`, with the mock
///   pairing's `pairing_product_is_one` deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_refuses_a_shared_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let possession =
                        build_pairing_elgamal_possession(&adapter, &key_pair, Default::default());
                    let shared = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0x0Fu8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length nonzero draw samples a scalar")
                    .scalar
                    .expose()
                    .clone();
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let shared_pk1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(shared.clone()),
                                },
                            ),
                        )
                        .expect("the shared scalar times g1 succeeds")
                        .product;
                    let shared_pk2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(shared),
                                },
                            ),
                        )
                        .expect("the shared scalar times g2 succeeds")
                        .product;
                    let payload = build_public_keys_from_components_payload(
                        pairing,
                        &possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(build_public_keys_components(
                                pairing,
                                PublicKeysComponentsOverrides {
                                    pk1: Some(shared_pk1),
                                    pk2: Some(shared_pk2),
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::SharedSecret)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_refuses_a_shared_secret

/// Contract: components whose keys are both the identity element, which also
///   satisfy the shared-secret relation, return the identity refusal of the
///   first group.
/// Arrange: as
///   `public_keys_from_components_refuses_an_identity_first_group_key`, with
///   `pk1` and `pk2` both overridden to the multiples of the generators by the
///   zero scalar.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG1))`.
/// Boundary: the private surface's `public_keys_from_components`, with the mock
///   pairing's identity checks deciding before `pairing_product_is_one` is
///   reached.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_checks_the_identity_keys_before_the_shared_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let possession =
                        build_pairing_elgamal_possession(&adapter, &key_pair, Default::default());
                    let zero_scalar = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("sampling a zero draw succeeds")
                    .scalar
                    .expose()
                    .clone();
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let identity_pk1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(zero_scalar.clone()),
                                },
                            ),
                        )
                        .expect("the identity multiple multiplies")
                        .product;
                    let identity_pk2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(zero_scalar),
                                },
                            ),
                        )
                        .expect("the identity multiple multiplies")
                        .product;
                    let payload = build_public_keys_from_components_payload(
                        pairing,
                        &possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(build_public_keys_components(
                                pairing,
                                PublicKeysComponentsOverrides {
                                    pk1: Some(identity_pk1),
                                    pk2: Some(identity_pk2),
                                },
                            )),
                        },
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG1)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_checks_the_identity_keys_before_the_shared_secret

/// Contract: components whose first-group proof does not satisfy `z·g = R +
///   c·pk` return `Err(PublicKeysFromComponentsErrorReturn::PossessionG1)`.
/// Arrange: the adapter and the default key pair; another key pair from
///   `build_pairing_elgamal_key_pair` with `x` and `y` overridden;
///   `build_pairing_elgamal_possession` over the default key pair with `r1` and
///   `z1` overridden to the other key pair's proof's fields, so only the
///   first-group proof differs.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::PossessionG1))`.
/// Boundary: the private surface's `public_keys_from_components`, with the
///   possession check deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_refuses_a_first_group_proof_from_another_key_pair() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let other = build_pairing_elgamal_key_pair(
                        pairing,
                        PairingElGamalKeyPairOverrides {
                            x: Some(
                                P::Scalar::sample_from_uniform_bytes(
                                    SampleUniformScalarParams,
                                    build_sample_uniform_scalar_payload(
                                        SampleUniformScalarPayloadOverrides {
                                            uniform: Some(build_secret(
                                                vec![0x1Du8; bound],
                                                SecretConstructorParamsOverrides,
                                            )),
                                        },
                                    ),
                                )
                                .expect("the other first secret samples")
                                .scalar,
                            ),
                            y: Some(
                                P::Scalar::sample_from_uniform_bytes(
                                    SampleUniformScalarParams,
                                    build_sample_uniform_scalar_payload(
                                        SampleUniformScalarPayloadOverrides {
                                            uniform: Some(build_secret(
                                                vec![0x1Eu8; bound],
                                                SecretConstructorParamsOverrides,
                                            )),
                                        },
                                    ),
                                )
                                .expect("the other second secret samples")
                                .scalar,
                            ),
                            ..Default::default()
                        },
                    );
                    let other_possession =
                        build_pairing_elgamal_possession(&adapter, &other, Default::default());
                    let possession = build_pairing_elgamal_possession(
                        &adapter,
                        &key_pair,
                        PairingElGamalPossessionOverrides {
                            r1: Some(other_possession.r1.clone()),
                            z1: Some(other_possession.z1.clone()),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &possession,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::PossessionG1)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_refuses_a_first_group_proof_from_another_key_pair

/// Contract: components whose first-group proof satisfies the equation and
///   whose second-group proof does not return
///   `Err(PublicKeysFromComponentsErrorReturn::PossessionG2)`.
/// Arrange: as
///   `public_keys_from_components_refuses_a_first_group_proof_from_another_key_pair`,
///   with `r2` and `z2` overridden to the other key pair's proof's fields, so
///   only the second-group proof differs.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::PossessionG2))`.
/// Boundary: the private surface's `public_keys_from_components`, with the
///   possession check deciding.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_refuses_a_second_group_proof_from_another_key_pair() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let other = build_pairing_elgamal_key_pair(
                        pairing,
                        PairingElGamalKeyPairOverrides {
                            x: Some(
                                P::Scalar::sample_from_uniform_bytes(
                                    SampleUniformScalarParams,
                                    build_sample_uniform_scalar_payload(
                                        SampleUniformScalarPayloadOverrides {
                                            uniform: Some(build_secret(
                                                vec![0x1Du8; bound],
                                                SecretConstructorParamsOverrides,
                                            )),
                                        },
                                    ),
                                )
                                .expect("the other first secret samples")
                                .scalar,
                            ),
                            y: Some(
                                P::Scalar::sample_from_uniform_bytes(
                                    SampleUniformScalarParams,
                                    build_sample_uniform_scalar_payload(
                                        SampleUniformScalarPayloadOverrides {
                                            uniform: Some(build_secret(
                                                vec![0x1Eu8; bound],
                                                SecretConstructorParamsOverrides,
                                            )),
                                        },
                                    ),
                                )
                                .expect("the other second secret samples")
                                .scalar,
                            ),
                            ..Default::default()
                        },
                    );
                    let other_possession =
                        build_pairing_elgamal_possession(&adapter, &other, Default::default());
                    let possession = build_pairing_elgamal_possession(
                        &adapter,
                        &key_pair,
                        PairingElGamalPossessionOverrides {
                            r2: Some(other_possession.r2.clone()),
                            z2: Some(other_possession.z2.clone()),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &possession,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::PossessionG2)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_refuses_a_second_group_proof_from_another_key_pair

/// Contract: components whose proofs both fail return the first-group refusal.
/// Arrange: as
///   `public_keys_from_components_refuses_a_first_group_proof_from_another_key_pair`,
///   with the possession built for the other key pair whole.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::PossessionG1))`.
/// Boundary: the private surface's `public_keys_from_components`, with the
///   first-group possession check deciding before the second's is reached.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_checks_the_first_group_proof_before_the_second() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let other = build_pairing_elgamal_key_pair(
                        pairing,
                        PairingElGamalKeyPairOverrides {
                            x: Some(
                                P::Scalar::sample_from_uniform_bytes(
                                    SampleUniformScalarParams,
                                    build_sample_uniform_scalar_payload(
                                        SampleUniformScalarPayloadOverrides {
                                            uniform: Some(build_secret(
                                                vec![0x1Du8; bound],
                                                SecretConstructorParamsOverrides,
                                            )),
                                        },
                                    ),
                                )
                                .expect("the other first secret samples")
                                .scalar,
                            ),
                            y: Some(
                                P::Scalar::sample_from_uniform_bytes(
                                    SampleUniformScalarParams,
                                    build_sample_uniform_scalar_payload(
                                        SampleUniformScalarPayloadOverrides {
                                            uniform: Some(build_secret(
                                                vec![0x1Eu8; bound],
                                                SecretConstructorParamsOverrides,
                                            )),
                                        },
                                    ),
                                )
                                .expect("the other second secret samples")
                                .scalar,
                            ),
                            ..Default::default()
                        },
                    );
                    let possession =
                        build_pairing_elgamal_possession(&adapter, &other, Default::default());

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &possession,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::PossessionG1)
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_checks_the_first_group_proof_before_the_second

/// Contract: components that pass the key checks and a refused challenge hash
///   return `Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
///   PairingElGamalPublicKeysFromComponentsErrorReturn::HashToScalar(error)))`
///   with the refusal unchanged.
/// Arrange: `build_create_hash_to_scalar_params` with `concrete` overridden to
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)` for the adapter under test; a second adapter over the
///   default hash-to-scalar adapter, whose `build_pairing_elgamal_possession`
///   forms the possession for the default key pair, so the builder never meets
///   the refusal.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  `assert_eq!(result.err(), Some(PublicKeysFromComponentsErrorReturn::
///   PairingElGamal(PairingElGamalPublicKeysFromComponentsErrorReturn::
///   HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual: 0 })))))`
///   with `expected` the bound.
/// Boundary: the private surface's `public_keys_from_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes,
///   the subject's hash-to-scalar concrete named `SamplingRefused` and the
///   fixture adapter's at its default.
#[test]
fn public_keys_from_components_returns_the_hash_to_scalar_refusal_unchanged() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                            concrete: Some(HashToScalarConcrete::Mock(
                                MockIHashToScalarAdapterFailureMode::SamplingRefused,
                            )),
                            ..Default::default()
                        }),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's mock concrete constructs under any failure mode");
                    let fixture_hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the family's default params select its mock concrete");
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the family's default params select its mock concrete");
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let fixture_adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*fixture_hash_to_scalar.adapter,
                        &encoded.adapter,
                        &*random.adapter,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let possession = build_pairing_elgamal_possession(
                        &fixture_adapter,
                        &key_pair,
                        Default::default(),
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &possession,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                            PairingElGamalPublicKeysFromComponentsErrorReturn::HashToScalar(
                                HashToScalarErrorReturn::Sampling(
                                    SampleUniformScalarErrorReturn::WrongLength {
                                        expected: bound,
                                        actual: 0,
                                    }
                                )
                            )
                        ))
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    assert!(created.is_ok());
}
// end public_keys_from_components_returns_the_hash_to_scalar_refusal_unchanged
