#![allow(clippy::expect_used)]

use core::marker::PhantomData;

use crate::factory::provides::{
    EnvelopeAlgebra, IKeyAgreementAdapter, KEY_AGREEMENT_INTERFACE_VERSION, KeyAgreementIdentifier,
    WrapToPayloadOverrides, build_envelope_components_params, build_envelope_components_payload,
    build_envelope_from_components_params, build_envelope_from_components_payload,
    build_generate_keys_params, build_generate_keys_payload, build_key_pair_components_params,
    build_key_pair_components_payload, build_key_pair_from_components_params,
    build_key_pair_from_components_payload, build_key_pair_public_keys_params,
    build_key_pair_public_keys_payload, build_possession_components_params,
    build_possession_components_payload, build_possession_from_components_params,
    build_possession_from_components_payload, build_public_keys_components_params,
    build_public_keys_components_payload, build_public_keys_from_components_params,
    build_public_keys_from_components_payload, build_unwrap_params, build_unwrap_payload,
    build_wrap_to_params, build_wrap_to_payload,
};
use crate::pairing_elgamal::provides::{
    PairingElGamalEnvelopeOverrides, PairingElGamalKeyAgreement, PairingElGamalKeyPairOverrides,
    build_pairing_elgamal_envelope, build_pairing_elgamal_key_agreement,
    build_pairing_elgamal_key_agreement_constructor_params, build_pairing_elgamal_key_pair,
    build_pairing_elgamal_possession, build_pairing_elgamal_public_keys,
};
use crate::possession_statement::provides::{
    PossessionG1StatementOverrides, PossessionG2StatementOverrides, build_possession_g1_statement,
    build_possession_g1_statement_description, build_possession_g2_statement,
    build_possession_g2_statement_description,
};
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, EncodeParams, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{CredentialComponentsOverrides, build_credential_components};
use pairing::{
    AddG1Params, AddG1PayloadOverrides, AddG2Params, AddG2PayloadOverrides, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingPayload, EncodeG1Params, EncodeG1PayloadOverrides,
    EncodeG2Params, EncodeG2PayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides,
    G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1PayloadOverrides, MulG2Params, MulG2PayloadOverrides, SampleUniformScalarParams,
    SampleUniformScalarPayloadOverrides, build_add_g1_payload, build_add_g2_payload,
    build_create_pairing_deps, build_create_pairing_params, build_encode_g1_payload,
    build_encode_g2_payload, build_encode_scalar_payload, build_mul_g1_payload,
    build_mul_g2_payload, build_sample_uniform_scalar_payload, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, build_create_random_source_params,
    create_random_source,
};

/// Contract: `try_new` over admitted params returns `Ok` whose adapter holds
///   the tags `DomainTag::try_new` builds from
///   `PAIRING_ELGAMAL_POSSESSION_G1_TAG` and
///   `PAIRING_ELGAMAL_POSSESSION_G2_TAG`.
/// Arrange: `build_pairing_elgamal_key_agreement_constructor_params` over the
///   received pairing, hash-to-scalar adapter, encoder, and randomness source,
///   with no override.
/// Act:     `PairingElGamalKeyAgreement::try_new`.
/// Assert:  the success arm extracted with `expect`; `g1_tag.as_bytes()` equals
///   the literal `b"ChainTorrent-v1-envelope-possession-g1"` and
///   `g2_tag.as_bytes()` the literal `b"ChainTorrent-v1-envelope-possession-g2"`.
/// Boundary: the private surface's `try_new`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn try_new_holds_the_tags_built_from_the_possession_constants() {
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

                    // Act
                    let created = PairingElGamalKeyAgreement::try_new(
                        build_pairing_elgamal_key_agreement_constructor_params(
                            self.pairing,
                            &*hash_to_scalar.adapter,
                            &encoded.adapter,
                            &*random.adapter,
                            Default::default(),
                        ),
                    );

                    // Assert
                    let adapter = created.expect("admitted constructor params admit the adapter");
                    assert_eq!(
                        adapter.g1_tag.as_bytes(),
                        b"ChainTorrent-v1-envelope-possession-g1"
                    );
                    assert_eq!(
                        adapter.g2_tag.as_bytes(),
                        b"ChainTorrent-v1-envelope-possession-g2"
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

/// Contract: given draws of the scalar type's length, `generate_keys` returns
///   `Ok` whose key pair holds `x` and `y` as the `Secret`s the scalar type
///   samples from `x_uniform` and `y_uniform`, moved without copy.
/// Arrange: `build_generate_keys_payload` at its defaults, whose draws differ
///   from each other; the expected scalars sampled by
///   `P::Scalar::sample_from_uniform_bytes` from bytes cloned off the payload
///   before the call.
/// Act:     `adapter.generate_keys`.
/// Assert:  the success arm extracted with `expect`; `key_pair.x` and
///   `key_pair.y`, read through `Secret::expose` and `encode_scalar`, encode
///   equal to the expected scalars' encodings, and the two encodings differ.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_returns_the_secrets_its_draws_sample() {
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
                    let draws = build_generate_keys_payload(pairing, Default::default());
                    let expected_x = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                draws.x_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length x draw samples a scalar")
                    .scalar;
                    let expected_y = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                draws.y_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length y draw samples a scalar")
                    .scalar;

                    // Act
                    let generated = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    let generated = generated.expect("differing draws admit a key pair");
                    let x_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(generated.key_pair.x.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the first secret succeeds")
                        .bytes;
                    let y_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(generated.key_pair.y.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the second secret succeeds")
                        .bytes;
                    let expected_x_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(expected_x.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the first expected scalar succeeds")
                        .bytes;
                    let expected_y_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(expected_y.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the second expected scalar succeeds")
                        .bytes;
                    assert_eq!(
                        x_encoded.expose().as_ref(),
                        expected_x_encoded.expose().as_ref()
                    );
                    assert_eq!(
                        y_encoded.expose().as_ref(),
                        expected_y_encoded.expose().as_ref()
                    );
                    assert_ne!(x_encoded.expose().as_ref(), y_encoded.expose().as_ref());
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

/// Contract: `generate_keys` returns `Ok` whose key pair's public keys are
///   `pk1 = mul_g1(g1, x)` and `pk2 = mul_g2(g2, y)` for the sampled `x` and
///   `y`.
/// Arrange: `build_generate_keys_payload` at its defaults; the expected scalars
///   sampled from bytes cloned off the payload; the expected points from the
///   received pairing's `mul_g1` over `g1_generator` and `mul_g2` over
///   `g2_generator` by the expected scalars.
/// Act:     `adapter.generate_keys`.
/// Assert:  `key_pair.pk1` encodes through `encode_g1` equal to the expected
///   first-group point's encoding, `key_pair.pk2` through `encode_g2` equal to
///   the expected second-group point's, and `key_pair.pk1` encodes differently
///   from `g1_generator`.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_returns_the_public_keys_its_secrets_form() {
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
                    let draws = build_generate_keys_payload(pairing, Default::default());
                    let expected_x = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                draws.x_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length x draw samples a scalar")
                    .scalar;
                    let expected_y = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                draws.y_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length y draw samples a scalar")
                    .scalar;
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let expected_pk1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1.clone()),
                                    scalar: Some(expected_x.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the first generator succeeds")
                        .product;
                    let expected_pk2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(expected_y.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the second generator succeeds")
                        .product;

                    // Act
                    let generated = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    let generated = generated.expect("differing draws admit a key pair");
                    let pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(generated.key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first public key succeeds")
                        .bytes;
                    let pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(generated.key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second public key succeeds")
                        .bytes;
                    let expected_pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(expected_pk1),
                                },
                            ),
                        )
                        .expect("encoding the first expected point succeeds")
                        .bytes;
                    let expected_pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(expected_pk2),
                                },
                            ),
                        )
                        .expect("encoding the second expected point succeeds")
                        .bytes;
                    let g1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(g1) },
                            ),
                        )
                        .expect("encoding the first generator succeeds")
                        .bytes;
                    assert_eq!(pk1_encoded.as_ref(), expected_pk1_encoded.as_ref());
                    assert_eq!(pk2_encoded.as_ref(), expected_pk2_encoded.as_ref());
                    assert_ne!(pk1_encoded.as_ref(), g1_encoded.as_ref());
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

/// Contract: `generate_keys` returns `Ok` whose possession `(R1, z1, R2, z2)`
///   satisfies `z·g = R + c·pk` in each group, `c` the hash-to-scalar mapping,
///   under the group's tag, of the encoding of the statement holding the key's
///   and `R`'s precompile encodings.
/// Arrange: `build_generate_keys_payload` at its defaults; the tags built by
///   `DomainTag::try_new` over the literal bytes
///   `b"ChainTorrent-v1-envelope-possession-g1"` and
///   `b"ChainTorrent-v1-envelope-possession-g2"`.
/// Act:     `adapter.generate_keys`.
/// Assert:  for each group the statement is built by the official statement
///   builder from the returned key and commitment, encoded by the received
///   encoder under the official description, and hashed by the received
///   hash-to-scalar adapter under the group's tag to `c`; `mul_g1(g1, z1)`
///   encodes equal to `add_g1(R1, mul_g1(pk1, c1))` and `mul_g2(g2, z2)` equal
///   to `add_g2(R2, mul_g2(pk2, c2))`.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_returns_proofs_that_satisfy_the_verification_equation() {
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
                    let draws = build_generate_keys_payload(pairing, Default::default());
                    let g1_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g1".to_vec(),
                    })
                    .expect("the first group's possession tag is admitted");
                    let g2_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g2".to_vec(),
                    })
                    .expect("the second group's possession tag is admitted");

                    // Act
                    let generated = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    let generated = generated.expect("differing draws admit a key pair");
                    let g1_statement = build_possession_g1_statement(
                        pairing,
                        PossessionG1StatementOverrides {
                            key: Some(generated.key_pair.pk1.clone()),
                            commitment: Some(generated.possession.r1.clone()),
                        },
                    );
                    let encoded_g1_statement = encoded
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &build_possession_g1_statement_description(
                                    pairing,
                                    Default::default(),
                                ),
                            },
                            &g1_statement,
                        )
                        .expect("encoding the first group's statement succeeds")
                        .bytes;
                    let c1 = hash_to_scalar
                        .adapter
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g1_tag },
                            HashToScalarPayload {
                                message: &encoded_g1_statement,
                            },
                        )
                        .expect("hashing the first group's challenge succeeds")
                        .scalar;
                    let g2_statement = build_possession_g2_statement(
                        pairing,
                        PossessionG2StatementOverrides {
                            key: Some(generated.key_pair.pk2.clone()),
                            commitment: Some(generated.possession.r2.clone()),
                        },
                    );
                    let encoded_g2_statement = encoded
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &build_possession_g2_statement_description(
                                    pairing,
                                    Default::default(),
                                ),
                            },
                            &g2_statement,
                        )
                        .expect("encoding the second group's statement succeeds")
                        .bytes;
                    let c2 = hash_to_scalar
                        .adapter
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g2_tag },
                            HashToScalarPayload {
                                message: &encoded_g2_statement,
                            },
                        )
                        .expect("hashing the second group's challenge succeeds")
                        .scalar;
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let lhs1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(generated.possession.z1.clone()),
                                },
                            ),
                        )
                        .expect("multiplying the first generator by z1 succeeds")
                        .product;
                    let rhs1 = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(generated.possession.r1.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g1(
                                                MulG1Params,
                                                build_mul_g1_payload(
                                                    pairing,
                                                    MulG1PayloadOverrides {
                                                        point: Some(generated.key_pair.pk1.clone()),
                                                        scalar: Some(c1),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk1 by c1 succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let lhs2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(generated.possession.z2.clone()),
                                },
                            ),
                        )
                        .expect("multiplying the second generator by z2 succeeds")
                        .product;
                    let rhs2 = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(generated.possession.r2.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g2(
                                                MulG2Params,
                                                build_mul_g2_payload(
                                                    pairing,
                                                    MulG2PayloadOverrides {
                                                        point: Some(generated.key_pair.pk2.clone()),
                                                        scalar: Some(c2),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk2 by c2 succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let lhs1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(lhs1) },
                            ),
                        )
                        .expect("encoding the first group's left side succeeds")
                        .bytes;
                    let rhs1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(rhs1) },
                            ),
                        )
                        .expect("encoding the first group's right side succeeds")
                        .bytes;
                    let lhs2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(lhs2) },
                            ),
                        )
                        .expect("encoding the second group's left side succeeds")
                        .bytes;
                    let rhs2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(rhs2) },
                            ),
                        )
                        .expect("encoding the second group's right side succeeds")
                        .bytes;
                    assert_eq!(lhs1_encoded.as_ref(), rhs1_encoded.as_ref());
                    assert_eq!(lhs2_encoded.as_ref(), rhs2_encoded.as_ref());
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

/// Contract: `generate_keys` returns `Ok` whose possession's first-group
///   challenge is hashed under the first-group tag and whose second-group
///   challenge is hashed under the second-group tag.
/// Arrange: `build_generate_keys_payload` at its defaults; the tags built by
///   `DomainTag::try_new` over the literal possession tag bytes.
/// Act:     `adapter.generate_keys`.
/// Assert:  for each group the two sides of `z·g = R + c·pk` encode differently
///   when `c` is hashed under the other group's tag, asserted by `assert_ne!`.
/// Boundary: the private surface's `generate_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn generate_keys_hashes_each_challenge_under_its_groups_tag() {
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
                    let draws = build_generate_keys_payload(pairing, Default::default());
                    let g1_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g1".to_vec(),
                    })
                    .expect("the first group's possession tag is admitted");
                    let g2_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g2".to_vec(),
                    })
                    .expect("the second group's possession tag is admitted");

                    // Act
                    let generated = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws);

                    // Assert
                    let generated = generated.expect("differing draws admit a key pair");
                    let g1_statement = build_possession_g1_statement(
                        pairing,
                        PossessionG1StatementOverrides {
                            key: Some(generated.key_pair.pk1.clone()),
                            commitment: Some(generated.possession.r1.clone()),
                        },
                    );
                    let encoded_g1_statement = encoded
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &build_possession_g1_statement_description(
                                    pairing,
                                    Default::default(),
                                ),
                            },
                            &g1_statement,
                        )
                        .expect("encoding the first group's statement succeeds")
                        .bytes;
                    let wrong_c1 = hash_to_scalar
                        .adapter
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g2_tag },
                            HashToScalarPayload {
                                message: &encoded_g1_statement,
                            },
                        )
                        .expect("hashing under the other tag succeeds")
                        .scalar;
                    let g2_statement = build_possession_g2_statement(
                        pairing,
                        PossessionG2StatementOverrides {
                            key: Some(generated.key_pair.pk2.clone()),
                            commitment: Some(generated.possession.r2.clone()),
                        },
                    );
                    let encoded_g2_statement = encoded
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &build_possession_g2_statement_description(
                                    pairing,
                                    Default::default(),
                                ),
                            },
                            &g2_statement,
                        )
                        .expect("encoding the second group's statement succeeds")
                        .bytes;
                    let wrong_c2 = hash_to_scalar
                        .adapter
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g1_tag },
                            HashToScalarPayload {
                                message: &encoded_g2_statement,
                            },
                        )
                        .expect("hashing under the other tag succeeds")
                        .scalar;
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let lhs1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(generated.possession.z1.clone()),
                                },
                            ),
                        )
                        .expect("multiplying the first generator by z1 succeeds")
                        .product;
                    let wrong_rhs1 = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(generated.possession.r1.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g1(
                                                MulG1Params,
                                                build_mul_g1_payload(
                                                    pairing,
                                                    MulG1PayloadOverrides {
                                                        point: Some(generated.key_pair.pk1.clone()),
                                                        scalar: Some(wrong_c1),
                                                    },
                                                ),
                                            )
                                            .expect(
                                                "multiplying pk1 by the wrong challenge succeeds",
                                            )
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let lhs2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(generated.possession.z2.clone()),
                                },
                            ),
                        )
                        .expect("multiplying the second generator by z2 succeeds")
                        .product;
                    let wrong_rhs2 = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(generated.possession.r2.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g2(
                                                MulG2Params,
                                                build_mul_g2_payload(
                                                    pairing,
                                                    MulG2PayloadOverrides {
                                                        point: Some(generated.key_pair.pk2.clone()),
                                                        scalar: Some(wrong_c2),
                                                    },
                                                ),
                                            )
                                            .expect(
                                                "multiplying pk2 by the wrong challenge succeeds",
                                            )
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let lhs1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(lhs1) },
                            ),
                        )
                        .expect("encoding the first group's left side succeeds")
                        .bytes;
                    let wrong_rhs1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(wrong_rhs1),
                                },
                            ),
                        )
                        .expect("encoding the first group's wrong right side succeeds")
                        .bytes;
                    let lhs2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(lhs2) },
                            ),
                        )
                        .expect("encoding the second group's left side succeeds")
                        .bytes;
                    let wrong_rhs2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(wrong_rhs2),
                                },
                            ),
                        )
                        .expect("encoding the second group's wrong right side succeeds")
                        .bytes;
                    assert_ne!(lhs1_encoded.as_ref(), wrong_rhs1_encoded.as_ref());
                    assert_ne!(lhs2_encoded.as_ref(), wrong_rhs2_encoded.as_ref());
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

/// Contract: `wrap_to` returns `Ok` whose envelope is
///   `(C1, C2, D1, D2) = (ρ·g1, A + ρ·pk1, σ·g2, B + σ·pk2)` for the returned
///   coins.
/// Arrange: `build_pairing_elgamal_public_keys` at its defaults;
///   `build_wrap_to_payload` with the public keys handed and `credential`
///   overridden to `build_credential_components` whose `a` and `b` are points
///   from the received pairing that differ from each other, from the
///   generators, and from the public keys.
/// Act:     `adapter.wrap_to`.
/// Assert:  with `rho` and `sigma` read from `coins` through `Secret::expose`,
///   `envelope.c1` encodes equal to `mul_g1(g1, rho)`, `envelope.c2` to
///   `add_g1(A, mul_g1(pk1, rho))`, `envelope.d1` to `mul_g2(g2, sigma)`, and
///   `envelope.d2` to `add_g2(B, mul_g2(pk2, sigma))`.
/// Boundary: the private surface's `wrap_to`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn wrap_to_returns_the_envelope_its_coins_form() {
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
                    let public_keys =
                        build_pairing_elgamal_public_keys(pairing, Default::default());
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let a = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(g1.clone()),
                                    right: Some(public_keys.pk1.clone()),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let b = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(g2.clone()),
                                    right: Some(public_keys.pk2.clone()),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let credential = build_credential_components(
                        pairing,
                        CredentialComponentsOverrides {
                            a: Some(a.clone()),
                            b: Some(b.clone()),
                        },
                    );
                    let payload = build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(credential),
                        },
                    );

                    // Act
                    let wrapped =
                        adapter.wrap_to(build_wrap_to_params(Default::default()), payload);

                    // Assert
                    let wrapped = wrapped.expect("the admitted public keys admit a wrap");
                    let rho = wrapped.coins.rho.expose().clone();
                    let sigma = wrapped.coins.sigma.expose().clone();
                    let expected_c1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1.clone()),
                                    scalar: Some(rho.clone()),
                                },
                            ),
                        )
                        .expect("multiplying the first generator by rho succeeds")
                        .product;
                    let expected_c2 = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(a),
                                    right: Some(
                                        pairing
                                            .mul_g1(
                                                MulG1Params,
                                                build_mul_g1_payload(
                                                    pairing,
                                                    MulG1PayloadOverrides {
                                                        point: Some(public_keys.pk1.clone()),
                                                        scalar: Some(rho),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk1 by rho succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let expected_d1 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(sigma.clone()),
                                },
                            ),
                        )
                        .expect("multiplying the second generator by sigma succeeds")
                        .product;
                    let expected_d2 = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(b),
                                    right: Some(
                                        pairing
                                            .mul_g2(
                                                MulG2Params,
                                                build_mul_g2_payload(
                                                    pairing,
                                                    MulG2PayloadOverrides {
                                                        point: Some(public_keys.pk2.clone()),
                                                        scalar: Some(sigma),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk2 by sigma succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let c1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(wrapped.envelope.c1.clone()),
                                },
                            ),
                        )
                        .expect("encoding c1 succeeds")
                        .bytes;
                    let c2_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(wrapped.envelope.c2.clone()),
                                },
                            ),
                        )
                        .expect("encoding c2 succeeds")
                        .bytes;
                    let d1_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(wrapped.envelope.d1.clone()),
                                },
                            ),
                        )
                        .expect("encoding d1 succeeds")
                        .bytes;
                    let d2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(wrapped.envelope.d2.clone()),
                                },
                            ),
                        )
                        .expect("encoding d2 succeeds")
                        .bytes;
                    let expected_c1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(expected_c1),
                                },
                            ),
                        )
                        .expect("encoding the expected c1 succeeds")
                        .bytes;
                    let expected_c2_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(expected_c2),
                                },
                            ),
                        )
                        .expect("encoding the expected c2 succeeds")
                        .bytes;
                    let expected_d1_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(expected_d1),
                                },
                            ),
                        )
                        .expect("encoding the expected d1 succeeds")
                        .bytes;
                    let expected_d2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(expected_d2),
                                },
                            ),
                        )
                        .expect("encoding the expected d2 succeeds")
                        .bytes;
                    assert_eq!(c1_encoded.as_ref(), expected_c1_encoded.as_ref());
                    assert_eq!(c2_encoded.as_ref(), expected_c2_encoded.as_ref());
                    assert_eq!(d1_encoded.as_ref(), expected_d1_encoded.as_ref());
                    assert_eq!(d2_encoded.as_ref(), expected_d2_encoded.as_ref());
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

/// Contract: `wrap_to` returns `Ok` whose coins `ρ` and `σ` are sampled from
///   separate draws of the randomness source.
/// Arrange: `build_pairing_elgamal_public_keys` at its defaults;
///   `build_wrap_to_payload` with the public keys handed and `credential`
///   overridden to `build_credential_components` whose `a` and `b` are points
///   from the received pairing that differ from each other, from the
///   generators, and from the public keys.
/// Act:     `adapter.wrap_to`.
/// Assert:  `rho` and `sigma`, read from `coins` through `Secret::expose`,
///   encode differently through `encode_scalar`.
/// Boundary: the private surface's `wrap_to`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn wrap_to_draws_each_coin_from_its_own_draw() {
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
                    let public_keys =
                        build_pairing_elgamal_public_keys(pairing, Default::default());
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let a = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(g1),
                                    right: Some(public_keys.pk1.clone()),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let b = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(g2.clone()),
                                    right: Some(public_keys.pk2.clone()),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let credential = build_credential_components(
                        pairing,
                        CredentialComponentsOverrides {
                            a: Some(a),
                            b: Some(b),
                        },
                    );
                    let payload = build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(credential),
                        },
                    );

                    // Act
                    let wrapped =
                        adapter.wrap_to(build_wrap_to_params(Default::default()), payload);

                    // Assert
                    let wrapped = wrapped.expect("the admitted public keys admit a wrap");
                    let rho_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(wrapped.coins.rho.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding rho succeeds")
                        .bytes;
                    let sigma_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(wrapped.coins.sigma.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding sigma succeeds")
                        .bytes;
                    assert_ne!(
                        rho_encoded.expose().as_ref(),
                        sigma_encoded.expose().as_ref()
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

/// Contract: `unwrap` returns `Ok` holding
///   `(A, B) = (C2 − x·C1, D2 − y·D1)`.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults; `A` and `B`
///   points that differ from each other and from the generators;
///   `build_pairing_elgamal_envelope` with `c1`, `c2`, `d1`, and `d2`
///   overridden to `(ρ·g1, A + ρ·pk1, σ·g2, B + σ·pk2)` for constant `ρ` and
///   `σ` sampled by the received scalar type, the elements formed by the
///   received pairing's operations.
/// Act:     `adapter.unwrap`.
/// Assert:  `credential.a` encodes equal to `A`'s encoding and `credential.b`
///   to `B`'s, and `credential.a` encodes differently from `credential.b`.
/// Boundary: the private surface's `unwrap`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn unwrap_returns_the_components_the_envelope_was_formed_from() {
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
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let a = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(g1.clone()),
                                    right: Some(key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let b = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(g2.clone()),
                                    right: Some(key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let rho = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x21u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length rho draw samples a scalar")
                    .scalar;
                    let sigma = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x22u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length sigma draw samples a scalar")
                    .scalar;
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(
                                pairing
                                    .mul_g1(
                                        MulG1Params,
                                        build_mul_g1_payload(
                                            pairing,
                                            MulG1PayloadOverrides {
                                                point: Some(g1),
                                                scalar: Some(rho.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("multiplying the first generator by rho succeeds")
                                    .product,
                            ),
                            c2: Some(
                                pairing
                                    .add_g1(
                                        AddG1Params,
                                        build_add_g1_payload(
                                            pairing,
                                            AddG1PayloadOverrides {
                                                left: Some(a.clone()),
                                                right: Some(
                                                    pairing
                                                        .mul_g1(
                                                            MulG1Params,
                                                            build_mul_g1_payload(
                                                                pairing,
                                                                MulG1PayloadOverrides {
                                                                    point: Some(
                                                                        key_pair.pk1.clone(),
                                                                    ),
                                                                    scalar: Some(
                                                                        rho.expose().clone(),
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect("multiplying pk1 by rho succeeds")
                                                        .product,
                                                ),
                                            },
                                        ),
                                    )
                                    .expect("adding the first group's terms succeeds")
                                    .sum,
                            ),
                            d1: Some(
                                pairing
                                    .mul_g2(
                                        MulG2Params,
                                        build_mul_g2_payload(
                                            pairing,
                                            MulG2PayloadOverrides {
                                                point: Some(g2),
                                                scalar: Some(sigma.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("multiplying the second generator by sigma succeeds")
                                    .product,
                            ),
                            d2: Some(
                                pairing
                                    .add_g2(
                                        AddG2Params,
                                        build_add_g2_payload(
                                            pairing,
                                            AddG2PayloadOverrides {
                                                left: Some(b.clone()),
                                                right: Some(
                                                    pairing
                                                        .mul_g2(
                                                            MulG2Params,
                                                            build_mul_g2_payload(
                                                                pairing,
                                                                MulG2PayloadOverrides {
                                                                    point: Some(
                                                                        key_pair.pk2.clone(),
                                                                    ),
                                                                    scalar: Some(
                                                                        sigma.expose().clone(),
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect("multiplying pk2 by sigma succeeds")
                                                        .product,
                                                ),
                                            },
                                        ),
                                    )
                                    .expect("adding the second group's terms succeeds")
                                    .sum,
                            ),
                        },
                    );

                    // Act
                    let unwrapped = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let unwrapped = unwrapped.expect("unwrap over an admitted envelope succeeds");
                    let a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(unwrapped.credential.a.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned first component succeeds")
                        .bytes;
                    let b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(unwrapped.credential.b.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned second component succeeds")
                        .bytes;
                    let expected_a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(a) },
                            ),
                        )
                        .expect("encoding A succeeds")
                        .bytes;
                    let expected_b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(b) },
                            ),
                        )
                        .expect("encoding B succeeds")
                        .bytes;
                    assert_eq!(a_encoded.as_ref(), expected_a_encoded.as_ref());
                    assert_eq!(b_encoded.as_ref(), expected_b_encoded.as_ref());
                    assert_ne!(a_encoded.as_ref(), b_encoded.as_ref());
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

/// Contract: an envelope formed to one key pair, unwrapped under another key
///   pair, returns components other than the formed ones.
/// Arrange: the envelope of `unwrap_returns_the_components_the_envelope_was_formed_from`;
///   another key pair from `build_pairing_elgamal_key_pair` with `x` and `y`
///   overridden to `Secret`s sampled from bytes that differ from the defaults
///   and from each other.
/// Act:     `adapter.unwrap` under the other key pair.
/// Assert:  neither returned component encodes equal to the encoding of `A` or
///   of `B`.
/// Boundary: the private surface's `unwrap`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn unwrap_under_another_key_pair_returns_different_components() {
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
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let a = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(g1.clone()),
                                    right: Some(key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let b = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(g2.clone()),
                                    right: Some(key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let rho = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x21u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length rho draw samples a scalar")
                    .scalar;
                    let sigma = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x22u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length sigma draw samples a scalar")
                    .scalar;
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(
                                pairing
                                    .mul_g1(
                                        MulG1Params,
                                        build_mul_g1_payload(
                                            pairing,
                                            MulG1PayloadOverrides {
                                                point: Some(g1),
                                                scalar: Some(rho.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("multiplying the first generator by rho succeeds")
                                    .product,
                            ),
                            c2: Some(
                                pairing
                                    .add_g1(
                                        AddG1Params,
                                        build_add_g1_payload(
                                            pairing,
                                            AddG1PayloadOverrides {
                                                left: Some(a.clone()),
                                                right: Some(
                                                    pairing
                                                        .mul_g1(
                                                            MulG1Params,
                                                            build_mul_g1_payload(
                                                                pairing,
                                                                MulG1PayloadOverrides {
                                                                    point: Some(
                                                                        key_pair.pk1.clone(),
                                                                    ),
                                                                    scalar: Some(
                                                                        rho.expose().clone(),
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect("multiplying pk1 by rho succeeds")
                                                        .product,
                                                ),
                                            },
                                        ),
                                    )
                                    .expect("adding the first group's terms succeeds")
                                    .sum,
                            ),
                            d1: Some(
                                pairing
                                    .mul_g2(
                                        MulG2Params,
                                        build_mul_g2_payload(
                                            pairing,
                                            MulG2PayloadOverrides {
                                                point: Some(g2),
                                                scalar: Some(sigma.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("multiplying the second generator by sigma succeeds")
                                    .product,
                            ),
                            d2: Some(
                                pairing
                                    .add_g2(
                                        AddG2Params,
                                        build_add_g2_payload(
                                            pairing,
                                            AddG2PayloadOverrides {
                                                left: Some(b.clone()),
                                                right: Some(
                                                    pairing
                                                        .mul_g2(
                                                            MulG2Params,
                                                            build_mul_g2_payload(
                                                                pairing,
                                                                MulG2PayloadOverrides {
                                                                    point: Some(
                                                                        key_pair.pk2.clone(),
                                                                    ),
                                                                    scalar: Some(
                                                                        sigma.expose().clone(),
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect("multiplying pk2 by sigma succeeds")
                                                        .product,
                                                ),
                                            },
                                        ),
                                    )
                                    .expect("adding the second group's terms succeeds")
                                    .sum,
                            ),
                        },
                    );
                    let other_key_pair = build_pairing_elgamal_key_pair(
                        pairing,
                        PairingElGamalKeyPairOverrides {
                            x: Some(P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            vec![0x31u8;
                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("a full-length x draw samples a scalar")
                            .scalar),
                            y: Some(P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            vec![0x32u8;
                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("a full-length y draw samples a scalar")
                            .scalar),
                            pk1: None,
                            pk2: None,
                        },
                    );

                    // Act
                    let unwrapped = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&other_key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let unwrapped = unwrapped.expect("unwrap over an admitted envelope succeeds");
                    let a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(unwrapped.credential.a.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned first component succeeds")
                        .bytes;
                    let b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(unwrapped.credential.b.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned second component succeeds")
                        .bytes;
                    let expected_a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(a) },
                            ),
                        )
                        .expect("encoding A succeeds")
                        .bytes;
                    let expected_b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(b) },
                            ),
                        )
                        .expect("encoding B succeeds")
                        .bytes;
                    assert_ne!(a_encoded.as_ref(), expected_a_encoded.as_ref());
                    assert_ne!(a_encoded.as_ref(), expected_b_encoded.as_ref());
                    assert_ne!(b_encoded.as_ref(), expected_a_encoded.as_ref());
                    assert_ne!(b_encoded.as_ref(), expected_b_encoded.as_ref());
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

/// Contract: `PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION` is
///   `KeyAgreementDeclaration { identifier: PairingElGamalV1,
///   algebra: PairingElGamal, possession_g1_tag: PAIRING_ELGAMAL_POSSESSION_G1_TAG,
///   possession_g2_tag: PAIRING_ELGAMAL_POSSESSION_G2_TAG, adapter_version: 1,
///   interface_version: KEY_AGREEMENT_INTERFACE_VERSION }`.
/// Arrange: the block's consumers only, which supply `P` and `E`; no instance.
/// Act:     reading `PairingElGamalKeyAgreement::<P, E>::DECLARATION` in the
///   innermost method.
/// Assert:  `identifier` matches `KeyAgreementIdentifier::PairingElGamalV1`
///   under `matches!`; `algebra` equals `EnvelopeAlgebra::PairingElGamal`; each
///   tag equals the literal written in the assertion; `adapter_version` equals
///   the literal `1`; `interface_version` equals
///   `KEY_AGREEMENT_INTERFACE_VERSION`.
/// Boundary: the private surface's `DECLARATION` constant.
/// Mocked:  the pairing and encoding concretes each family's factory hands out
///   at default params, supplying the block's generic types.
#[test]
fn declaration_names_the_identifier_algebra_tags_and_versions() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            _consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<P>(PhantomData<P>);

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    _encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Act
                    let declaration = PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION;

                    // Assert
                    assert!(matches!(
                        declaration.identifier,
                        KeyAgreementIdentifier::PairingElGamalV1
                    ));
                    assert_eq!(declaration.algebra, EnvelopeAlgebra::PairingElGamal);
                    assert_eq!(
                        declaration.possession_g1_tag,
                        b"ChainTorrent-v1-envelope-possession-g1"
                    );
                    assert_eq!(
                        declaration.possession_g2_tag,
                        b"ChainTorrent-v1-envelope-possession-g2"
                    );
                    assert_eq!(declaration.adapter_version, 1);
                    assert_eq!(
                        declaration.interface_version,
                        KEY_AGREEMENT_INTERFACE_VERSION
                    );
                }
            }

            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer::<P>(PhantomData)),
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
// end declaration_names_the_identifier_algebra_tags_and_versions

/// Contract: an envelope unwrapped under a key pair whose `x` and `y` are
///   exchanged returns components other than the formed ones.
/// Arrange: the envelope of `unwrap_returns_the_components_the_envelope_was_formed_from`;
///   a key pair from `build_pairing_elgamal_key_pair` with `x` overridden to
///   the default `y` and `y` to the default `x`.
/// Act:     `adapter.unwrap` under the exchanged key pair.
/// Assert:  neither returned component encodes equal to the encoding of `A` or
///   of `B`.
/// Boundary: the private surface's `unwrap`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn unwrap_with_exchanged_secrets_returns_different_components() {
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
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let a = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(g1.clone()),
                                    right: Some(key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let b = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(g2.clone()),
                                    right: Some(key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let rho = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x21u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length rho draw samples a scalar")
                    .scalar;
                    let sigma = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x22u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length sigma draw samples a scalar")
                    .scalar;
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(
                                pairing
                                    .mul_g1(
                                        MulG1Params,
                                        build_mul_g1_payload(
                                            pairing,
                                            MulG1PayloadOverrides {
                                                point: Some(g1),
                                                scalar: Some(rho.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("multiplying the first generator by rho succeeds")
                                    .product,
                            ),
                            c2: Some(
                                pairing
                                    .add_g1(
                                        AddG1Params,
                                        build_add_g1_payload(
                                            pairing,
                                            AddG1PayloadOverrides {
                                                left: Some(a.clone()),
                                                right: Some(
                                                    pairing
                                                        .mul_g1(
                                                            MulG1Params,
                                                            build_mul_g1_payload(
                                                                pairing,
                                                                MulG1PayloadOverrides {
                                                                    point: Some(
                                                                        key_pair.pk1.clone(),
                                                                    ),
                                                                    scalar: Some(
                                                                        rho.expose().clone(),
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect("multiplying pk1 by rho succeeds")
                                                        .product,
                                                ),
                                            },
                                        ),
                                    )
                                    .expect("adding the first group's terms succeeds")
                                    .sum,
                            ),
                            d1: Some(
                                pairing
                                    .mul_g2(
                                        MulG2Params,
                                        build_mul_g2_payload(
                                            pairing,
                                            MulG2PayloadOverrides {
                                                point: Some(g2),
                                                scalar: Some(sigma.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("multiplying the second generator by sigma succeeds")
                                    .product,
                            ),
                            d2: Some(
                                pairing
                                    .add_g2(
                                        AddG2Params,
                                        build_add_g2_payload(
                                            pairing,
                                            AddG2PayloadOverrides {
                                                left: Some(b.clone()),
                                                right: Some(
                                                    pairing
                                                        .mul_g2(
                                                            MulG2Params,
                                                            build_mul_g2_payload(
                                                                pairing,
                                                                MulG2PayloadOverrides {
                                                                    point: Some(
                                                                        key_pair.pk2.clone(),
                                                                    ),
                                                                    scalar: Some(
                                                                        sigma.expose().clone(),
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect("multiplying pk2 by sigma succeeds")
                                                        .product,
                                                ),
                                            },
                                        ),
                                    )
                                    .expect("adding the second group's terms succeeds")
                                    .sum,
                            ),
                        },
                    );
                    let exchanged_key_pair = build_pairing_elgamal_key_pair(
                        pairing,
                        PairingElGamalKeyPairOverrides {
                            x: Some(P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            vec![0x02u8;
                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("a full-length x draw samples a scalar")
                            .scalar),
                            y: Some(P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            vec![0x01u8;
                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("a full-length y draw samples a scalar")
                            .scalar),
                            pk1: None,
                            pk2: None,
                        },
                    );

                    // Act
                    let unwrapped = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&exchanged_key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let unwrapped = unwrapped.expect("unwrap over an admitted envelope succeeds");
                    let a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(unwrapped.credential.a.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned first component succeeds")
                        .bytes;
                    let b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(unwrapped.credential.b.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned second component succeeds")
                        .bytes;
                    let expected_a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(a) },
                            ),
                        )
                        .expect("encoding A succeeds")
                        .bytes;
                    let expected_b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(b) },
                            ),
                        )
                        .expect("encoding B succeeds")
                        .bytes;
                    assert_ne!(a_encoded.as_ref(), expected_a_encoded.as_ref());
                    assert_ne!(a_encoded.as_ref(), expected_b_encoded.as_ref());
                    assert_ne!(b_encoded.as_ref(), expected_a_encoded.as_ref());
                    assert_ne!(b_encoded.as_ref(), expected_b_encoded.as_ref());
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
// end unwrap_with_exchanged_secrets_returns_different_components

/// Contract: an envelope whose `C1` is taken from another envelope unwraps to
///   a first component other than `A` and a second component equal to `B`.
/// Arrange: the envelope of `unwrap_returns_the_components_the_envelope_was_formed_from`,
///   and another envelope formed under a different `ρ`, whose `c1` replaces
///   the first envelope's through the builder's `c1` override.
/// Act:     `adapter.unwrap` under the key pair.
/// Assert:  `credential.a` encodes differently from `A`, and `credential.b`
///   encodes equal to `B`.
/// Boundary: the private surface's `unwrap`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn unwrap_with_a_first_group_coin_element_from_another_envelope_changes_only_the_first_component() {
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
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let a = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(g1.clone()),
                                    right: Some(key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let b = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(g2.clone()),
                                    right: Some(key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let rho = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x21u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length rho draw samples a scalar")
                    .scalar;
                    let sigma = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x22u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length sigma draw samples a scalar")
                    .scalar;
                    let c2 = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(a.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g1(
                                                MulG1Params,
                                                build_mul_g1_payload(
                                                    pairing,
                                                    MulG1PayloadOverrides {
                                                        point: Some(key_pair.pk1.clone()),
                                                        scalar: Some(rho.expose().clone()),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk1 by rho succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let d1 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2.clone()),
                                    scalar: Some(sigma.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the second generator by sigma succeeds")
                        .product;
                    let d2 = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(b.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g2(
                                                MulG2Params,
                                                build_mul_g2_payload(
                                                    pairing,
                                                    MulG2PayloadOverrides {
                                                        point: Some(key_pair.pk2.clone()),
                                                        scalar: Some(sigma.expose().clone()),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk2 by sigma succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let other_rho = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x41u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length rho draw samples a scalar")
                    .scalar;
                    let other_c1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(other_rho.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the first generator by the other rho succeeds")
                        .product;
                    let mixed_envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(other_c1),
                            c2: Some(c2),
                            d1: Some(d1),
                            d2: Some(d2),
                        },
                    );

                    // Act
                    let unwrapped = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&key_pair, &mixed_envelope, Default::default()),
                    );

                    // Assert
                    let unwrapped = unwrapped.expect("unwrap over an admitted envelope succeeds");
                    let a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(unwrapped.credential.a.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned first component succeeds")
                        .bytes;
                    let b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(unwrapped.credential.b.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned second component succeeds")
                        .bytes;
                    let expected_a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(a) },
                            ),
                        )
                        .expect("encoding A succeeds")
                        .bytes;
                    let expected_b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(b) },
                            ),
                        )
                        .expect("encoding B succeeds")
                        .bytes;
                    assert_ne!(a_encoded.as_ref(), expected_a_encoded.as_ref());
                    assert_eq!(b_encoded.as_ref(), expected_b_encoded.as_ref());
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
// end unwrap_with_a_first_group_coin_element_from_another_envelope_changes_only_the_first_component

/// Contract: an envelope whose `D1` is taken from another envelope unwraps to
///   a second component other than `B` and a first component equal to `A`.
/// Arrange: the envelope of `unwrap_returns_the_components_the_envelope_was_formed_from`,
///   and another envelope formed under a different `σ`, whose `d1` replaces
///   the first envelope's through the builder's `d1` override.
/// Act:     `adapter.unwrap` under the key pair.
/// Assert:  `credential.b` encodes differently from `B`, and `credential.a`
///   encodes equal to `A`.
/// Boundary: the private surface's `unwrap`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn unwrap_with_a_second_group_coin_element_from_another_envelope_changes_only_the_second_component()
{
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
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let a = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(g1.clone()),
                                    right: Some(key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let b = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(g2.clone()),
                                    right: Some(key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let rho = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x21u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length rho draw samples a scalar")
                    .scalar;
                    let sigma = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x22u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length sigma draw samples a scalar")
                    .scalar;
                    let c1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1.clone()),
                                    scalar: Some(rho.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the first generator by rho succeeds")
                        .product;
                    let c2 = pairing
                        .add_g1(
                            AddG1Params,
                            build_add_g1_payload(
                                pairing,
                                AddG1PayloadOverrides {
                                    left: Some(a.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g1(
                                                MulG1Params,
                                                build_mul_g1_payload(
                                                    pairing,
                                                    MulG1PayloadOverrides {
                                                        point: Some(key_pair.pk1.clone()),
                                                        scalar: Some(rho.expose().clone()),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk1 by rho succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the first group's terms succeeds")
                        .sum;
                    let d2 = pairing
                        .add_g2(
                            AddG2Params,
                            build_add_g2_payload(
                                pairing,
                                AddG2PayloadOverrides {
                                    left: Some(b.clone()),
                                    right: Some(
                                        pairing
                                            .mul_g2(
                                                MulG2Params,
                                                build_mul_g2_payload(
                                                    pairing,
                                                    MulG2PayloadOverrides {
                                                        point: Some(key_pair.pk2.clone()),
                                                        scalar: Some(sigma.expose().clone()),
                                                    },
                                                ),
                                            )
                                            .expect("multiplying pk2 by sigma succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("adding the second group's terms succeeds")
                        .sum;
                    let other_sigma = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![
                                    0x42u8;
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                ],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length sigma draw samples a scalar")
                    .scalar;
                    let other_d1 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(other_sigma.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the second generator by the other sigma succeeds")
                        .product;
                    let mixed_envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(c1),
                            c2: Some(c2),
                            d1: Some(other_d1),
                            d2: Some(d2),
                        },
                    );

                    // Act
                    let unwrapped = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&key_pair, &mixed_envelope, Default::default()),
                    );

                    // Assert
                    let unwrapped = unwrapped.expect("unwrap over an admitted envelope succeeds");
                    let a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(unwrapped.credential.a.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned first component succeeds")
                        .bytes;
                    let b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(unwrapped.credential.b.clone()),
                                },
                            ),
                        )
                        .expect("encoding the returned second component succeeds")
                        .bytes;
                    let expected_a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(a) },
                            ),
                        )
                        .expect("encoding A succeeds")
                        .bytes;
                    let expected_b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(b) },
                            ),
                        )
                        .expect("encoding B succeeds")
                        .bytes;
                    assert_eq!(a_encoded.as_ref(), expected_a_encoded.as_ref());
                    assert_ne!(b_encoded.as_ref(), expected_b_encoded.as_ref());
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
// end unwrap_with_a_second_group_coin_element_from_another_envelope_changes_only_the_second_component

/// Contract: over components whose secrets are distinct and nonzero,
///   `key_pair_from_components` returns `Ok` whose key pair holds the secrets
///   moved, `pk1 = mul_g1(g1, x)`, and `pk2 = mul_g2(g2, y)`.
/// Arrange: `build_key_pair_from_components_payload` at its defaults, built a
///   second time from the same defaults so the expected secrets survive the
///   move; the expected points from the received pairing's `mul_g1` and
///   `mul_g2` over the generators by the expected scalars.
/// Act:     `adapter.key_pair_from_components`.
/// Assert:  the success arm extracted with `expect`; `key_pair.x` and
///   `key_pair.y` encode through `encode_scalar` equal to the expected scalars,
///   and `key_pair.pk1` and `key_pair.pk2` encode equal to the expected points.
/// Boundary: the private surface's `key_pair_from_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn key_pair_from_components_returns_the_key_pair_its_secrets_form() {
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
                    let payload =
                        build_key_pair_from_components_payload(pairing, Default::default());
                    let expected =
                        build_key_pair_from_components_payload(pairing, Default::default())
                            .components;
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;
                    let expected_pk1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(expected.x.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the first generator succeeds")
                        .product;
                    let expected_pk2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(expected.y.expose().clone()),
                                },
                            ),
                        )
                        .expect("multiplying the second generator succeeds")
                        .product;

                    // Act
                    let result = adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    let result = result.expect("distinct nonzero secrets admit a key pair");
                    let x_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.key_pair.x.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the first secret succeeds")
                        .bytes;
                    let y_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.key_pair.y.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the second secret succeeds")
                        .bytes;
                    let pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first public key succeeds")
                        .bytes;
                    let pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second public key succeeds")
                        .bytes;
                    let expected_x_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(expected.x.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the first expected scalar succeeds")
                        .bytes;
                    let expected_y_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(expected.y.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the second expected scalar succeeds")
                        .bytes;
                    let expected_pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(expected_pk1),
                                },
                            ),
                        )
                        .expect("encoding the first expected point succeeds")
                        .bytes;
                    let expected_pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(expected_pk2),
                                },
                            ),
                        )
                        .expect("encoding the second expected point succeeds")
                        .bytes;
                    assert_eq!(
                        x_encoded.expose().as_ref(),
                        expected_x_encoded.expose().as_ref()
                    );
                    assert_eq!(
                        y_encoded.expose().as_ref(),
                        expected_y_encoded.expose().as_ref()
                    );
                    assert_eq!(pk1_encoded.as_ref(), expected_pk1_encoded.as_ref());
                    assert_eq!(pk2_encoded.as_ref(), expected_pk2_encoded.as_ref());
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
// end key_pair_from_components_returns_the_key_pair_its_secrets_form

/// Contract: over components that pass the identity, shared-secret, and proof
///   checks, `public_keys_from_components` returns `Ok` whose public keys hold
///   `pk1` and `pk2` moved.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults;
///   `build_pairing_elgamal_possession(&adapter, &key_pair, Default::default())`,
///   the proof valid for that key pair; `build_public_keys_from_components_payload`
///   with the possession handed and `components` at its defaults, which are the
///   default key pair's public keys.
/// Act:     `adapter.public_keys_from_components`.
/// Assert:  the success arm extracted with `expect`; `public_keys.pk1` and
///   `public_keys.pk2` encode equal to the key pair's `pk1` and `pk2`.
/// Boundary: the private surface's `public_keys_from_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_from_components_returns_the_public_keys_the_components_hold() {
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
                    let payload = build_public_keys_from_components_payload(
                        pairing,
                        &possession,
                        Default::default(),
                    );

                    // Act
                    let result = adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    let result = result.expect("a valid possession admits the public keys");
                    let pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.public_keys.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first public key succeeds")
                        .bytes;
                    let pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.public_keys.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second public key succeeds")
                        .bytes;
                    let key_pair_pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the key pair's first key succeeds")
                        .bytes;
                    let key_pair_pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the key pair's second key succeeds")
                        .bytes;
                    assert_eq!(pk1_encoded.as_ref(), key_pair_pk1_encoded.as_ref());
                    assert_eq!(pk2_encoded.as_ref(), key_pair_pk2_encoded.as_ref());
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
// end public_keys_from_components_returns_the_public_keys_the_components_hold

/// Contract: `key_pair_components` returns `Ok` holding `x` and `y` as new
///   `Secret`s of the key pair's secrets.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults, whose `x` and
///   `y` differ.
/// Act:     `adapter.key_pair_components`.
/// Assert:  `components.x` and `components.y` encode through `encode_scalar`
///   equal to the key pair's `x` and `y` respectively, and differ from each
///   other.
/// Boundary: the private surface's `key_pair_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn key_pair_components_returns_new_secrets_holding_the_key_pairs_secrets() {
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

                    // Act
                    let result = adapter.key_pair_components(
                        build_key_pair_components_params(Default::default()),
                        build_key_pair_components_payload(&key_pair, Default::default()),
                    );

                    // Assert
                    let result = result.expect("reading a key pair's components succeeds");
                    let x_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.components.x.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the first component succeeds")
                        .bytes;
                    let y_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.components.y.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the second component succeeds")
                        .bytes;
                    let key_pair_x_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(key_pair.x.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the key pair's first secret succeeds")
                        .bytes;
                    let key_pair_y_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(key_pair.y.expose().clone()),
                                },
                            ),
                        )
                        .expect("encoding the key pair's second secret succeeds")
                        .bytes;
                    assert_eq!(
                        x_encoded.expose().as_ref(),
                        key_pair_x_encoded.expose().as_ref()
                    );
                    assert_eq!(
                        y_encoded.expose().as_ref(),
                        key_pair_y_encoded.expose().as_ref()
                    );
                    assert_ne!(x_encoded.expose().as_ref(), y_encoded.expose().as_ref());
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
// end key_pair_components_returns_new_secrets_holding_the_key_pairs_secrets

/// Contract: `key_pair_public_keys` returns `Ok` holding clones of the key
///   pair's `pk1` and `pk2`.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults, whose public keys
///   differ from the generators.
/// Act:     `adapter.key_pair_public_keys`.
/// Assert:  `components.pk1` and `components.pk2` encode equal to the key
///   pair's `pk1` and `pk2` and differ from the generators.
/// Boundary: the private surface's `key_pair_public_keys`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn key_pair_public_keys_returns_the_key_pairs_public_keys() {
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
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;

                    // Act
                    let result = adapter.key_pair_public_keys(
                        build_key_pair_public_keys_params(Default::default()),
                        build_key_pair_public_keys_payload(&key_pair, Default::default()),
                    );

                    // Assert
                    let result = result.expect("reading a key pair's public keys succeeds");
                    let pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.components.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first component succeeds")
                        .bytes;
                    let pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.components.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second component succeeds")
                        .bytes;
                    let key_pair_pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(key_pair.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the key pair's first key succeeds")
                        .bytes;
                    let key_pair_pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(key_pair.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the key pair's second key succeeds")
                        .bytes;
                    let g1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(g1) },
                            ),
                        )
                        .expect("encoding the first generator succeeds")
                        .bytes;
                    let g2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(g2) },
                            ),
                        )
                        .expect("encoding the second generator succeeds")
                        .bytes;
                    assert_eq!(pk1_encoded.as_ref(), key_pair_pk1_encoded.as_ref());
                    assert_eq!(pk2_encoded.as_ref(), key_pair_pk2_encoded.as_ref());
                    assert_ne!(pk1_encoded.as_ref(), g1_encoded.as_ref());
                    assert_ne!(pk2_encoded.as_ref(), g2_encoded.as_ref());
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
// end key_pair_public_keys_returns_the_key_pairs_public_keys

/// Contract: `public_keys_components` returns `Ok` holding clones of `pk1` and
///   `pk2`.
/// Arrange: `build_pairing_elgamal_public_keys` at its defaults, whose points
///   differ from the generators.
/// Act:     `adapter.public_keys_components`.
/// Assert:  `components.pk1` and `components.pk2` encode equal to the public
///   keys' `pk1` and `pk2` and differ from the generators.
/// Boundary: the private surface's `public_keys_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn public_keys_components_returns_the_public_keys_elements() {
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
                    let public_keys =
                        build_pairing_elgamal_public_keys(pairing, Default::default());
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first-group generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second-group generator reads")
                        .point;

                    // Act
                    let result = adapter.public_keys_components(
                        build_public_keys_components_params(Default::default()),
                        build_public_keys_components_payload(&public_keys, Default::default()),
                    );

                    // Assert
                    let result = result.expect("reading public keys' components succeeds");
                    let pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.components.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first component succeeds")
                        .bytes;
                    let pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.components.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second component succeeds")
                        .bytes;
                    let public_keys_pk1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(public_keys.pk1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the public keys' first key succeeds")
                        .bytes;
                    let public_keys_pk2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(public_keys.pk2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the public keys' second key succeeds")
                        .bytes;
                    let g1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides { point: Some(g1) },
                            ),
                        )
                        .expect("encoding the first generator succeeds")
                        .bytes;
                    let g2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides { point: Some(g2) },
                            ),
                        )
                        .expect("encoding the second generator succeeds")
                        .bytes;
                    assert_eq!(pk1_encoded.as_ref(), public_keys_pk1_encoded.as_ref());
                    assert_eq!(pk2_encoded.as_ref(), public_keys_pk2_encoded.as_ref());
                    assert_ne!(pk1_encoded.as_ref(), g1_encoded.as_ref());
                    assert_ne!(pk2_encoded.as_ref(), g2_encoded.as_ref());
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
// end public_keys_components_returns_the_public_keys_elements

/// Contract: `possession_components` returns `Ok` holding clones of `r1`,
///   `z1`, `r2`, and `z2`.
/// Arrange: `build_pairing_elgamal_possession` over the adapter and the default
///   key pair at its defaults, whose scalars differ from each other and whose
///   points differ from the generators.
/// Act:     `adapter.possession_components`.
/// Assert:  each of `components.r1`, `components.z1`, `components.r2`, and
///   `components.z2` encodes equal to the field it was read from, and
///   `components.z1` encodes differently from `components.z2`.
/// Boundary: the private surface's `possession_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn possession_components_returns_the_possessions_elements() {
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

                    // Act
                    let result = adapter.possession_components(
                        build_possession_components_params(Default::default()),
                        build_possession_components_payload(&possession, Default::default()),
                    );

                    // Assert
                    let result = result.expect("reading a possession's components succeeds");
                    let r1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.components.r1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first commitment succeeds")
                        .bytes;
                    let z1_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.components.z1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first response succeeds")
                        .bytes;
                    let r2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.components.r2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second commitment succeeds")
                        .bytes;
                    let z2_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.components.z2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second response succeeds")
                        .bytes;
                    let possession_r1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(possession.r1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the possession's first commitment succeeds")
                        .bytes;
                    let possession_z1_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(possession.z1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the possession's first response succeeds")
                        .bytes;
                    let possession_r2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(possession.r2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the possession's second commitment succeeds")
                        .bytes;
                    let possession_z2_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(possession.z2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the possession's second response succeeds")
                        .bytes;
                    assert_eq!(r1_encoded.as_ref(), possession_r1_encoded.as_ref());
                    assert_eq!(
                        z1_encoded.expose().as_ref(),
                        possession_z1_encoded.expose().as_ref()
                    );
                    assert_eq!(r2_encoded.as_ref(), possession_r2_encoded.as_ref());
                    assert_eq!(
                        z2_encoded.expose().as_ref(),
                        possession_z2_encoded.expose().as_ref()
                    );
                    assert_ne!(z1_encoded.expose().as_ref(), z2_encoded.expose().as_ref());
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
// end possession_components_returns_the_possessions_elements

/// Contract: `possession_from_components` returns `Ok` whose possession holds
///   the components moved.
/// Arrange: `build_possession_from_components_payload` at its defaults, built a
///   second time so the expected values survive the move.
/// Act:     `adapter.possession_from_components`.
/// Assert:  `possession.r1`, `possession.z1`, `possession.r2`, and
///   `possession.z2` encode equal to the expected components' fields, and
///   `possession.z1` encodes differently from `possession.z2`.
/// Boundary: the private surface's `possession_from_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn possession_from_components_holds_the_components() {
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
                    let payload =
                        build_possession_from_components_payload(pairing, Default::default());
                    let expected =
                        build_possession_from_components_payload(pairing, Default::default())
                            .components;

                    // Act
                    let result = adapter.possession_from_components(
                        build_possession_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    let result = result.expect("admitted components admit a possession");
                    let r1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.possession.r1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first commitment succeeds")
                        .bytes;
                    let z1_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.possession.z1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first response succeeds")
                        .bytes;
                    let r2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.possession.r2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second commitment succeeds")
                        .bytes;
                    let z2_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(result.possession.z2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second response succeeds")
                        .bytes;
                    let expected_r1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(expected.r1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected first commitment succeeds")
                        .bytes;
                    let expected_z1_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(expected.z1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected first response succeeds")
                        .bytes;
                    let expected_r2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(expected.r2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected second commitment succeeds")
                        .bytes;
                    let expected_z2_encoded = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(expected.z2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected second response succeeds")
                        .bytes;
                    assert_eq!(r1_encoded.as_ref(), expected_r1_encoded.as_ref());
                    assert_eq!(
                        z1_encoded.expose().as_ref(),
                        expected_z1_encoded.expose().as_ref()
                    );
                    assert_eq!(r2_encoded.as_ref(), expected_r2_encoded.as_ref());
                    assert_eq!(
                        z2_encoded.expose().as_ref(),
                        expected_z2_encoded.expose().as_ref()
                    );
                    assert_ne!(z1_encoded.expose().as_ref(), z2_encoded.expose().as_ref());
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
// end possession_from_components_holds_the_components

/// Contract: `envelope_components` returns `Ok` holding clones of `c1`, `c2`,
///   `d1`, and `d2`.
/// Arrange: `build_pairing_elgamal_envelope` at its defaults, whose first-group
///   elements differ from each other and whose second-group elements differ
///   from each other.
/// Act:     `adapter.envelope_components`.
/// Assert:  each of `components.c1`, `components.c2`, `components.d1`, and
///   `components.d2` encodes equal to the field it was read from, and
///   `components.c1` encodes differently from `components.c2`.
/// Boundary: the private surface's `envelope_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn envelope_components_returns_the_envelopes_elements() {
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
                    let envelope = build_pairing_elgamal_envelope(pairing, Default::default());

                    // Act
                    let result = adapter.envelope_components(
                        build_envelope_components_params(Default::default()),
                        build_envelope_components_payload(&envelope, Default::default()),
                    );

                    // Assert
                    let result = result.expect("reading an envelope's components succeeds");
                    let c1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.components.c1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first component succeeds")
                        .bytes;
                    let c2_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.components.c2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second component succeeds")
                        .bytes;
                    let d1_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.components.d1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the third component succeeds")
                        .bytes;
                    let d2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.components.d2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the fourth component succeeds")
                        .bytes;
                    let envelope_c1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(envelope.c1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the envelope's first element succeeds")
                        .bytes;
                    let envelope_c2_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(envelope.c2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the envelope's second element succeeds")
                        .bytes;
                    let envelope_d1_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(envelope.d1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the envelope's third element succeeds")
                        .bytes;
                    let envelope_d2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(envelope.d2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the envelope's fourth element succeeds")
                        .bytes;
                    assert_eq!(c1_encoded.as_ref(), envelope_c1_encoded.as_ref());
                    assert_eq!(c2_encoded.as_ref(), envelope_c2_encoded.as_ref());
                    assert_eq!(d1_encoded.as_ref(), envelope_d1_encoded.as_ref());
                    assert_eq!(d2_encoded.as_ref(), envelope_d2_encoded.as_ref());
                    assert_ne!(c1_encoded.as_ref(), c2_encoded.as_ref());
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
// end envelope_components_returns_the_envelopes_elements

/// Contract: `envelope_from_components` returns `Ok` whose envelope holds the
///   components moved.
/// Arrange: `build_envelope_from_components_payload` at its defaults, built a
///   second time so the expected values survive the move.
/// Act:     `adapter.envelope_from_components`.
/// Assert:  `envelope.c1`, `envelope.c2`, `envelope.d1`, and `envelope.d2`
///   encode equal to the expected components' fields, and `envelope.c1`
///   encodes differently from `envelope.c2`.
/// Boundary: the private surface's `envelope_from_components`.
/// Mocked:  the pairing, encoding, hash-to-scalar, and randomness concretes each
///   family's factory hands out at default params.
#[test]
fn envelope_from_components_holds_the_components() {
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
                    let payload =
                        build_envelope_from_components_payload(pairing, Default::default());
                    let expected =
                        build_envelope_from_components_payload(pairing, Default::default())
                            .components;

                    // Act
                    let result = adapter.envelope_from_components(
                        build_envelope_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    let result = result.expect("admitted components admit an envelope");
                    let c1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.envelope.c1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the first element succeeds")
                        .bytes;
                    let c2_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(result.envelope.c2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the second element succeeds")
                        .bytes;
                    let d1_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.envelope.d1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the third element succeeds")
                        .bytes;
                    let d2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(result.envelope.d2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the fourth element succeeds")
                        .bytes;
                    let expected_c1_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(expected.c1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected first element succeeds")
                        .bytes;
                    let expected_c2_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(expected.c2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected second element succeeds")
                        .bytes;
                    let expected_d1_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(expected.d1.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected third element succeeds")
                        .bytes;
                    let expected_d2_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(expected.d2.clone()),
                                },
                            ),
                        )
                        .expect("encoding the expected fourth element succeeds")
                        .bytes;
                    assert_eq!(c1_encoded.as_ref(), expected_c1_encoded.as_ref());
                    assert_eq!(c2_encoded.as_ref(), expected_c2_encoded.as_ref());
                    assert_eq!(d1_encoded.as_ref(), expected_d1_encoded.as_ref());
                    assert_eq!(d2_encoded.as_ref(), expected_d2_encoded.as_ref());
                    assert_ne!(c1_encoded.as_ref(), c2_encoded.as_ref());
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
// end envelope_from_components_holds_the_components
