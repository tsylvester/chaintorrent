#![allow(clippy::expect_used)]

use crate::factory::provides::{
    EnvelopeAlgebra, EnvelopeFromComponentsPayloadOverrides, GenerateKeysPayloadOverrides,
    IKeyAgreementAdapter, KEY_AGREEMENT_INTERFACE_VERSION, KeyAgreementIdentifier,
    KeyPairComponentsOverrides, KeyPairFromComponentsPayloadOverrides,
    PossessionFromComponentsPayloadOverrides, PublicKeysFromComponentsPayloadOverrides,
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
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, EncodeParams, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
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

fn uniform_bytes<P: IPairingAdapter>(fill: u8) -> Secret<Vec<u8>> {
    build_secret(
        vec![fill; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
        SecretConstructorParamsOverrides,
    )
}

fn sampled_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(uniform_bytes::<P>(fill)),
        }),
    )
    .expect("the family's sampler admits uniform bytes of the exact length")
    .scalar
    .expose()
    .clone()
}

fn sampled_secret<P: IPairingAdapter>(fill: u8) -> Secret<P::Scalar> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(uniform_bytes::<P>(fill)),
        }),
    )
    .expect("the family's sampler admits uniform bytes of the exact length")
    .scalar
}

fn generator_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
        .expect("the first-group generator never refuses")
        .point
}

fn generator_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
        .expect("the second-group generator never refuses")
        .point
}

fn mul_g1<P: IPairingAdapter>(pairing: &P, point: P::G1, scalar: P::Scalar) -> P::G1 {
    pairing
        .mul_g1(
            MulG1Params,
            build_mul_g1_payload(
                pairing,
                MulG1PayloadOverrides {
                    point: Some(point),
                    scalar: Some(scalar),
                },
            ),
        )
        .expect("first-group scalar multiplication never refuses")
        .product
}

fn mul_g2<P: IPairingAdapter>(pairing: &P, point: P::G2, scalar: P::Scalar) -> P::G2 {
    pairing
        .mul_g2(
            MulG2Params,
            build_mul_g2_payload(
                pairing,
                MulG2PayloadOverrides {
                    point: Some(point),
                    scalar: Some(scalar),
                },
            ),
        )
        .expect("second-group scalar multiplication never refuses")
        .product
}

fn add_g1<P: IPairingAdapter>(pairing: &P, left: P::G1, right: P::G1) -> P::G1 {
    pairing
        .add_g1(
            AddG1Params,
            build_add_g1_payload(
                pairing,
                AddG1PayloadOverrides {
                    left: Some(left),
                    right: Some(right),
                },
            ),
        )
        .expect("first-group addition never refuses")
        .sum
}

fn add_g2<P: IPairingAdapter>(pairing: &P, left: P::G2, right: P::G2) -> P::G2 {
    pairing
        .add_g2(
            AddG2Params,
            build_add_g2_payload(
                pairing,
                AddG2PayloadOverrides {
                    left: Some(left),
                    right: Some(right),
                },
            ),
        )
        .expect("second-group addition never refuses")
        .sum
}

fn encoded_g1<P: IPairingAdapter>(pairing: &P, point: P::G1) -> Vec<u8> {
    pairing
        .encode_g1(
            EncodeG1Params,
            build_encode_g1_payload(pairing, EncodeG1PayloadOverrides { point: Some(point) }),
        )
        .expect("encoding a first-group point never refuses")
        .bytes
        .as_ref()
        .to_vec()
}

fn encoded_g2<P: IPairingAdapter>(pairing: &P, point: P::G2) -> Vec<u8> {
    pairing
        .encode_g2(
            EncodeG2Params,
            build_encode_g2_payload(pairing, EncodeG2PayloadOverrides { point: Some(point) }),
        )
        .expect("encoding a second-group point never refuses")
        .bytes
        .as_ref()
        .to_vec()
}

fn encoded_scalar<P: IPairingAdapter>(pairing: &P, scalar: P::Scalar) -> Vec<u8> {
    pairing
        .encode_scalar(
            EncodeScalarParams,
            build_encode_scalar_payload(
                pairing,
                EncodeScalarPayloadOverrides {
                    scalar: Some(scalar),
                },
            ),
        )
        .expect("encoding a scalar never refuses")
        .bytes
        .expose()
        .as_ref()
        .to_vec()
}

/// Contract: `try_new` over admitted params returns
///   `Ok(PairingElGamalKeyAgreement { …, g1_tag, g2_tag })` holding the tags
///   `DomainTag::try_new` builds from the two possession tag constants.
/// Arrange: `build_pairing_elgamal_key_agreement_constructor_params` over the
///   received pairing, hash-to-scalar adapter, encoder, and randomness source.
/// Act:     `PairingElGamalKeyAgreement::try_new(params)`.
/// Assert:  `g1_tag.as_bytes()` and `g2_tag.as_bytes()` equal the two distinct
///   literals.
#[test]
fn try_new_holds_the_tags_built_from_the_possession_constants() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let params = build_pairing_elgamal_key_agreement_constructor_params(
                        self.pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );

                    // Act
                    let result = PairingElGamalKeyAgreement::try_new(params);

                    // Assert
                    let success = result.expect("admitted constructor params build the concrete");
                    assert_eq!(
                        success.g1_tag.as_bytes(),
                        b"ChainTorrent-v1-envelope-possession-g1"
                    );
                    assert_eq!(
                        success.g2_tag.as_bytes(),
                        b"ChainTorrent-v1-envelope-possession-g2"
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION` is
///   `KeyAgreementDeclaration { identifier: PairingElGamalV1,
///   algebra: PairingElGamal,
///   possession_g1_tag: PAIRING_ELGAMAL_POSSESSION_G1_TAG,
///   possession_g2_tag: PAIRING_ELGAMAL_POSSESSION_G2_TAG, adapter_version: 1,
///   interface_version: KEY_AGREEMENT_INTERFACE_VERSION }`.
/// Arrange: the block's consumers only, which supply `P` and `E`.
/// Act:     reading `PairingElGamalKeyAgreement::<P, E>::DECLARATION` in the
///   innermost method.
/// Assert:  `identifier` matches `KeyAgreementIdentifier::PairingElGamalV1`;
///   `algebra` equals `EnvelopeAlgebra::PairingElGamal`; each tag equals its
///   literal; `adapter_version` is `1`; `interface_version` is the constant.
#[test]
fn declaration_names_the_identifier_algebra_tags_and_versions() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    _consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let _ = self.pairing;

                    // Act
                    let declaration = PairingElGamalKeyAgreement::<P, E>::DECLARATION;

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

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: given draws of the scalar type's length, `generate_keys` returns
///   `Ok` whose key pair holds `x` and `y` as the `Secret`s the scalar type
///   samples from `x_uniform` and `y_uniform`.
/// Arrange: `build_generate_keys_payload` at its defaults; the expected scalars
///   sampled from bytes cloned off the payload before the call.
/// Act:     `adapter.generate_keys(build_generate_keys_params(Default::default()), payload)`.
/// Assert:  `key_pair.x` and `key_pair.y` encode equal to the expected scalars'
///   encodings, and the two encodings differ.
#[test]
fn generate_keys_returns_the_secrets_its_draws_sample() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let payload = build_generate_keys_payload::<P>(pairing, Default::default());
                    let expected_x = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                payload.x_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("the family's sampler admits uniform bytes of the exact length")
                    .scalar;
                    let expected_y = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                payload.y_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("the family's sampler admits uniform bytes of the exact length")
                    .scalar;

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), payload);

                    // Assert
                    let success = result.expect("default draws build a key pair");
                    let x = encoded_scalar(pairing, success.key_pair.x.expose().clone());
                    let y = encoded_scalar(pairing, success.key_pair.y.expose().clone());
                    assert_eq!(x, encoded_scalar(pairing, expected_x.expose().clone()));
                    assert_eq!(y, encoded_scalar(pairing, expected_y.expose().clone()));
                    assert_ne!(x, y);
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `pk1 = mul_g1(g1, x)` and `pk2 = mul_g2(g2, y)` for the sampled
///   `x` and `y`.
/// Arrange: as `generate_keys_returns_the_secrets_its_draws_sample`; the
///   expected points from `mul_g1` over `g1_generator` and `mul_g2` over
///   `g2_generator` by the expected scalars.
/// Act:     `generate_keys`.
/// Assert:  `key_pair.pk1` encodes equal to the expected first-group point's
///   encoding, `key_pair.pk2` to the second-group point's, and `key_pair.pk1`
///   encodes differently from `g1_generator`.
#[test]
fn generate_keys_returns_the_public_keys_its_secrets_form() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let payload = build_generate_keys_payload::<P>(pairing, Default::default());
                    let expected_x = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                payload.x_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("the family's sampler admits uniform bytes of the exact length")
                    .scalar;
                    let expected_y = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                payload.y_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("the family's sampler admits uniform bytes of the exact length")
                    .scalar;
                    let g1 = generator_g1(pairing);
                    let g2 = generator_g2(pairing);
                    let expected_pk1 = mul_g1(pairing, g1.clone(), expected_x.expose().clone());
                    let expected_pk2 = mul_g2(pairing, g2, expected_y.expose().clone());

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), payload);

                    // Assert
                    let success = result.expect("default draws build a key pair");
                    let pk1 = encoded_g1(pairing, success.key_pair.pk1.clone());
                    assert_eq!(pk1, encoded_g1(pairing, expected_pk1));
                    assert_eq!(
                        encoded_g2(pairing, success.key_pair.pk2.clone()),
                        encoded_g2(pairing, expected_pk2)
                    );
                    assert_ne!(pk1, encoded_g1(pairing, g1));
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the possession `(R1, z1, R2, z2)` satisfies `z·g = R + c·pk` in
///   each group, `c` the hash-to-scalar mapping, under the group's tag, of the
///   encoding of the statement holding the key's and `R`'s precompile
///   encodings.
/// Arrange: `build_generate_keys_payload` at its defaults; the tags built by
///   `DomainTag::try_new` over the two possession literals.
/// Act:     `generate_keys`.
/// Assert:  for each group, `c` hashed from the encoded statement under the
///   group's tag, `mul_g1(g1, z1)` encodes equal to
///   `add_g1(R1, mul_g1(pk1, c1))` and the second-group equation likewise.
#[test]
fn generate_keys_returns_proofs_that_satisfy_the_verification_equation() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let payload = build_generate_keys_payload::<P>(pairing, Default::default());
                    let g1_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g1".to_vec(),
                    })
                    .expect("the first-group possession tag is admitted");
                    let g2_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g2".to_vec(),
                    })
                    .expect("the second-group possession tag is admitted");
                    let g1_description =
                        build_possession_g1_statement_description(pairing, Default::default());
                    let g2_description =
                        build_possession_g2_statement_description(pairing, Default::default());

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), payload);

                    // Assert
                    let success = result.expect("default draws build a key pair");
                    let statement1 = build_possession_g1_statement(
                        pairing,
                        PossessionG1StatementOverrides {
                            key: Some(success.key_pair.pk1.clone()),
                            commitment: Some(success.possession.r1.clone()),
                        },
                    );
                    let encoded1 = consumed
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &g1_description,
                            },
                            &statement1,
                        )
                        .expect("encoding a possession statement never refuses");
                    let c1 = hash_to_scalar
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g1_tag },
                            HashToScalarPayload {
                                message: &encoded1.bytes,
                            },
                        )
                        .expect("hashing a possession challenge never refuses")
                        .scalar;
                    let statement2 = build_possession_g2_statement(
                        pairing,
                        PossessionG2StatementOverrides {
                            key: Some(success.key_pair.pk2.clone()),
                            commitment: Some(success.possession.r2.clone()),
                        },
                    );
                    let encoded2 = consumed
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &g2_description,
                            },
                            &statement2,
                        )
                        .expect("encoding a possession statement never refuses");
                    let c2 = hash_to_scalar
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g2_tag },
                            HashToScalarPayload {
                                message: &encoded2.bytes,
                            },
                        )
                        .expect("hashing a possession challenge never refuses")
                        .scalar;
                    let lhs1 = mul_g1(
                        pairing,
                        generator_g1(pairing),
                        success.possession.z1.clone(),
                    );
                    let rhs1 = add_g1(
                        pairing,
                        success.possession.r1.clone(),
                        mul_g1(pairing, success.key_pair.pk1.clone(), c1),
                    );
                    assert_eq!(encoded_g1(pairing, lhs1), encoded_g1(pairing, rhs1));
                    let lhs2 = mul_g2(
                        pairing,
                        generator_g2(pairing),
                        success.possession.z2.clone(),
                    );
                    let rhs2 = add_g2(
                        pairing,
                        success.possession.r2.clone(),
                        mul_g2(pairing, success.key_pair.pk2.clone(), c2),
                    );
                    assert_eq!(encoded_g2(pairing, lhs2), encoded_g2(pairing, rhs2));
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the first group's challenge is hashed under the first-group tag
///   and the second group's under the second-group tag.
/// Arrange: as
///   `generate_keys_returns_proofs_that_satisfy_the_verification_equation`.
/// Act:     `generate_keys`.
/// Assert:  for each group the two sides of `z·g = R + c·pk` encode
///   differently when `c` is hashed under the other group's tag.
#[test]
fn generate_keys_hashes_each_challenge_under_its_groups_tag() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let payload = build_generate_keys_payload::<P>(pairing, Default::default());
                    let g1_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g1".to_vec(),
                    })
                    .expect("the first-group possession tag is admitted");
                    let g2_tag = DomainTag::try_new(DomainTagConstructorParams {
                        bytes: b"ChainTorrent-v1-envelope-possession-g2".to_vec(),
                    })
                    .expect("the second-group possession tag is admitted");
                    let g1_description =
                        build_possession_g1_statement_description(pairing, Default::default());
                    let g2_description =
                        build_possession_g2_statement_description(pairing, Default::default());

                    // Act
                    let result = adapter
                        .generate_keys(build_generate_keys_params(Default::default()), payload);

                    // Assert
                    let success = result.expect("default draws build a key pair");
                    let statement1 = build_possession_g1_statement(
                        pairing,
                        PossessionG1StatementOverrides {
                            key: Some(success.key_pair.pk1.clone()),
                            commitment: Some(success.possession.r1.clone()),
                        },
                    );
                    let encoded1 = consumed
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &g1_description,
                            },
                            &statement1,
                        )
                        .expect("encoding a possession statement never refuses");
                    let wrong_c1 = hash_to_scalar
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g2_tag },
                            HashToScalarPayload {
                                message: &encoded1.bytes,
                            },
                        )
                        .expect("hashing a possession challenge never refuses")
                        .scalar;
                    let lhs1 = mul_g1(
                        pairing,
                        generator_g1(pairing),
                        success.possession.z1.clone(),
                    );
                    let wrong_rhs1 = add_g1(
                        pairing,
                        success.possession.r1.clone(),
                        mul_g1(pairing, success.key_pair.pk1.clone(), wrong_c1),
                    );
                    assert_ne!(encoded_g1(pairing, lhs1), encoded_g1(pairing, wrong_rhs1));
                    let statement2 = build_possession_g2_statement(
                        pairing,
                        PossessionG2StatementOverrides {
                            key: Some(success.key_pair.pk2.clone()),
                            commitment: Some(success.possession.r2.clone()),
                        },
                    );
                    let encoded2 = consumed
                        .adapter
                        .encode(
                            EncodeParams {
                                description: &g2_description,
                            },
                            &statement2,
                        )
                        .expect("encoding a possession statement never refuses");
                    let wrong_c2 = hash_to_scalar
                        .hash_to_scalar(
                            HashToScalarParams { tag: &g1_tag },
                            HashToScalarPayload {
                                message: &encoded2.bytes,
                            },
                        )
                        .expect("hashing a possession challenge never refuses")
                        .scalar;
                    let lhs2 = mul_g2(
                        pairing,
                        generator_g2(pairing),
                        success.possession.z2.clone(),
                    );
                    let wrong_rhs2 = add_g2(
                        pairing,
                        success.possession.r2.clone(),
                        mul_g2(pairing, success.key_pair.pk2.clone(), wrong_c2),
                    );
                    assert_ne!(encoded_g2(pairing, lhs2), encoded_g2(pairing, wrong_rhs2));
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `wrap_to` returns `Ok` whose envelope is
///   `(C1, C2, D1, D2) = (ρ·g1, A + ρ·pk1, σ·g2, B + σ·pk2)` for the returned
///   coins.
/// Arrange: `build_pairing_elgamal_public_keys` at its defaults;
///   `build_wrap_to_payload` with the public keys handed and `credential`
///   overridden to `build_credential_components` whose `a` and `b` differ from
///   each other, the generators, and the public keys.
/// Act:     `adapter.wrap_to(build_wrap_to_params(Default::default()), payload)`.
/// Assert:  `envelope.c1` encodes equal to `mul_g1(g1, rho)`, `envelope.c2` to
///   `add_g1(A, mul_g1(pk1, rho))`, `envelope.d1` to `mul_g2(g2, sigma)`, and
///   `envelope.d2` to `add_g2(B, mul_g2(pk2, sigma))`.
#[test]
fn wrap_to_returns_the_envelope_its_coins_form() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let public_keys =
                        build_pairing_elgamal_public_keys(pairing, Default::default());
                    let a = mul_g1(pairing, generator_g1(pairing), sampled_scalar::<P>(0x41));
                    let b = mul_g2(pairing, generator_g2(pairing), sampled_scalar::<P>(0x42));
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
                    let result = adapter.wrap_to(build_wrap_to_params(Default::default()), payload);

                    // Assert
                    let success = result.expect("default public keys admit a wrap");
                    let rho = success.coins.rho.expose().clone();
                    let sigma = success.coins.sigma.expose().clone();
                    assert_eq!(
                        encoded_g1(pairing, success.envelope.c1.clone()),
                        encoded_g1(pairing, mul_g1(pairing, generator_g1(pairing), rho.clone()))
                    );
                    assert_eq!(
                        encoded_g1(pairing, success.envelope.c2.clone()),
                        encoded_g1(
                            pairing,
                            add_g1(pairing, a, mul_g1(pairing, public_keys.pk1.clone(), rho))
                        )
                    );
                    assert_eq!(
                        encoded_g2(pairing, success.envelope.d1.clone()),
                        encoded_g2(
                            pairing,
                            mul_g2(pairing, generator_g2(pairing), sigma.clone())
                        )
                    );
                    assert_eq!(
                        encoded_g2(pairing, success.envelope.d2.clone()),
                        encoded_g2(
                            pairing,
                            add_g2(pairing, b, mul_g2(pairing, public_keys.pk2.clone(), sigma))
                        )
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `ρ` and `σ` are sampled from separate draws of the randomness
///   source.
/// Arrange: as `wrap_to_returns_the_envelope_its_coins_form`.
/// Act:     `wrap_to`.
/// Assert:  `rho` and `sigma` encode differently through `encode_scalar`.
#[test]
fn wrap_to_draws_each_coin_from_its_own_draw() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let public_keys =
                        build_pairing_elgamal_public_keys(pairing, Default::default());
                    let credential = build_credential_components(
                        pairing,
                        CredentialComponentsOverrides {
                            a: Some(mul_g1(
                                pairing,
                                generator_g1(pairing),
                                sampled_scalar::<P>(0x41),
                            )),
                            b: Some(mul_g2(
                                pairing,
                                generator_g2(pairing),
                                sampled_scalar::<P>(0x42),
                            )),
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
                    let result = adapter.wrap_to(build_wrap_to_params(Default::default()), payload);

                    // Assert
                    let success = result.expect("default public keys admit a wrap");
                    assert_ne!(
                        encoded_scalar(pairing, success.coins.rho.expose().clone()),
                        encoded_scalar(pairing, success.coins.sigma.expose().clone())
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `unwrap` returns `Ok` holding `(A, B) = (C2 − x·C1, D2 − y·D1)`.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults; `A` and `B`
///   points that differ from each other and from the generators;
///   `build_pairing_elgamal_envelope` overridden to
///   `(ρ·g1, A + ρ·pk1, σ·g2, B + σ·pk2)` for constant `ρ` and `σ`.
/// Act:     `adapter.unwrap(build_unwrap_params(Default::default()), build_unwrap_payload(&key_pair, &envelope, Default::default()))`.
/// Assert:  `credential.a` encodes equal to `A`, `credential.b` to `B`, and
///   the two differ.
#[test]
fn unwrap_returns_the_components_the_envelope_was_formed_from() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let a = mul_g1(pairing, generator_g1(pairing), sampled_scalar::<P>(0x41));
                    let b = mul_g2(pairing, generator_g2(pairing), sampled_scalar::<P>(0x42));
                    let rho = sampled_scalar::<P>(0x31);
                    let sigma = sampled_scalar::<P>(0x32);
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(mul_g1(pairing, generator_g1(pairing), rho.clone())),
                            c2: Some(add_g1(
                                pairing,
                                a.clone(),
                                mul_g1(pairing, key_pair.pk1.clone(), rho),
                            )),
                            d1: Some(mul_g2(pairing, generator_g2(pairing), sigma.clone())),
                            d2: Some(add_g2(
                                pairing,
                                b.clone(),
                                mul_g2(pairing, key_pair.pk2.clone(), sigma),
                            )),
                        },
                    );

                    // Act
                    let result = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let success = result.expect("a well-formed envelope unwraps");
                    let unwrapped_a = encoded_g1(pairing, success.credential.a.clone());
                    let unwrapped_b = encoded_g2(pairing, success.credential.b.clone());
                    assert_eq!(unwrapped_a, encoded_g1(pairing, a));
                    assert_eq!(unwrapped_b, encoded_g2(pairing, b));
                    assert_ne!(unwrapped_a, unwrapped_b);
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: an envelope formed to one key pair, unwrapped under another key
///   pair, returns components other than the formed ones.
/// Arrange: the envelope of
///   `unwrap_returns_the_components_the_envelope_was_formed_from`; another key
///   pair with `x` and `y` overridden to `Secret`s sampled from bytes that
///   differ from the defaults and from each other.
/// Act:     `unwrap` under the other key pair.
/// Assert:  neither returned component encodes equal to `A` or `B`.
#[test]
fn unwrap_under_another_key_pair_returns_different_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let a = mul_g1(pairing, generator_g1(pairing), sampled_scalar::<P>(0x41));
                    let b = mul_g2(pairing, generator_g2(pairing), sampled_scalar::<P>(0x42));
                    let rho = sampled_scalar::<P>(0x31);
                    let sigma = sampled_scalar::<P>(0x32);
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(mul_g1(pairing, generator_g1(pairing), rho.clone())),
                            c2: Some(add_g1(
                                pairing,
                                a.clone(),
                                mul_g1(pairing, key_pair.pk1.clone(), rho),
                            )),
                            d1: Some(mul_g2(pairing, generator_g2(pairing), sigma.clone())),
                            d2: Some(add_g2(
                                pairing,
                                b.clone(),
                                mul_g2(pairing, key_pair.pk2.clone(), sigma),
                            )),
                        },
                    );
                    let other_key_pair = build_pairing_elgamal_key_pair(
                        pairing,
                        PairingElGamalKeyPairOverrides {
                            x: Some(sampled_secret::<P>(0x51)),
                            y: Some(sampled_secret::<P>(0x52)),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&other_key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let success = result.expect("a well-formed envelope unwraps");
                    assert_ne!(
                        encoded_g1(pairing, success.credential.a.clone()),
                        encoded_g1(pairing, a)
                    );
                    assert_ne!(
                        encoded_g2(pairing, success.credential.b.clone()),
                        encoded_g2(pairing, b)
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: an envelope unwrapped under a key pair whose `x` and `y` are
///   exchanged returns components other than the formed ones.
/// Arrange: the envelope of
///   `unwrap_returns_the_components_the_envelope_was_formed_from`; a key pair
///   with `x` overridden to the default `y` and `y` to the default `x`.
/// Act:     `unwrap` under the exchanged key pair.
/// Assert:  neither returned component encodes equal to `A` or `B`.
#[test]
fn unwrap_with_exchanged_secrets_returns_different_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let a = mul_g1(pairing, generator_g1(pairing), sampled_scalar::<P>(0x41));
                    let b = mul_g2(pairing, generator_g2(pairing), sampled_scalar::<P>(0x42));
                    let rho = sampled_scalar::<P>(0x31);
                    let sigma = sampled_scalar::<P>(0x32);
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(mul_g1(pairing, generator_g1(pairing), rho.clone())),
                            c2: Some(add_g1(
                                pairing,
                                a.clone(),
                                mul_g1(pairing, key_pair.pk1.clone(), rho),
                            )),
                            d1: Some(mul_g2(pairing, generator_g2(pairing), sigma.clone())),
                            d2: Some(add_g2(
                                pairing,
                                b.clone(),
                                mul_g2(pairing, key_pair.pk2.clone(), sigma),
                            )),
                        },
                    );
                    let exchanged_key_pair = build_pairing_elgamal_key_pair(
                        pairing,
                        PairingElGamalKeyPairOverrides {
                            x: Some(key_pair.y),
                            y: Some(key_pair.x),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&exchanged_key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let success = result.expect("a well-formed envelope unwraps");
                    assert_ne!(
                        encoded_g1(pairing, success.credential.a.clone()),
                        encoded_g1(pairing, a)
                    );
                    assert_ne!(
                        encoded_g2(pairing, success.credential.b.clone()),
                        encoded_g2(pairing, b)
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: an envelope whose `C1` is taken from another envelope unwraps to
///   a first component other than `A` and a second component equal to `B`.
/// Arrange: the envelope of
///   `unwrap_returns_the_components_the_envelope_was_formed_from`, and another
///   envelope formed under a different `ρ`, whose `c1` replaces the first
///   envelope's through the builder's `c1` override.
/// Act:     `unwrap` under the key pair.
/// Assert:  `credential.a` encodes differently from `A`, and `credential.b`
///   encodes equal to `B`.
#[test]
fn unwrap_with_a_first_group_coin_element_from_another_envelope_changes_only_the_first_component() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let a = mul_g1(pairing, generator_g1(pairing), sampled_scalar::<P>(0x41));
                    let b = mul_g2(pairing, generator_g2(pairing), sampled_scalar::<P>(0x42));
                    let rho = sampled_scalar::<P>(0x31);
                    let sigma = sampled_scalar::<P>(0x32);
                    let other_rho = sampled_scalar::<P>(0x33);
                    let other_c1 = mul_g1(pairing, generator_g1(pairing), other_rho);
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(other_c1),
                            c2: Some(add_g1(
                                pairing,
                                a.clone(),
                                mul_g1(pairing, key_pair.pk1.clone(), rho),
                            )),
                            d1: Some(mul_g2(pairing, generator_g2(pairing), sigma.clone())),
                            d2: Some(add_g2(
                                pairing,
                                b.clone(),
                                mul_g2(pairing, key_pair.pk2.clone(), sigma),
                            )),
                        },
                    );

                    // Act
                    let result = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let success = result.expect("a well-formed envelope unwraps");
                    assert_ne!(
                        encoded_g1(pairing, success.credential.a.clone()),
                        encoded_g1(pairing, a)
                    );
                    assert_eq!(
                        encoded_g2(pairing, success.credential.b.clone()),
                        encoded_g2(pairing, b)
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: an envelope whose `D1` is taken from another envelope unwraps to
///   a second component other than `B` and a first component equal to `A`.
/// Arrange: the envelope of
///   `unwrap_returns_the_components_the_envelope_was_formed_from`, and another
///   envelope formed under a different `σ`, whose `d1` replaces the first
///   envelope's through the builder's `d1` override.
/// Act:     `unwrap` under the key pair.
/// Assert:  `credential.b` encodes differently from `B`, and `credential.a`
///   encodes equal to `A`.
#[test]
fn unwrap_with_a_second_group_coin_element_from_another_envelope_changes_only_the_second_component()
{
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());
                    let a = mul_g1(pairing, generator_g1(pairing), sampled_scalar::<P>(0x41));
                    let b = mul_g2(pairing, generator_g2(pairing), sampled_scalar::<P>(0x42));
                    let rho = sampled_scalar::<P>(0x31);
                    let sigma = sampled_scalar::<P>(0x32);
                    let other_sigma = sampled_scalar::<P>(0x34);
                    let other_d1 = mul_g2(pairing, generator_g2(pairing), other_sigma);
                    let envelope = build_pairing_elgamal_envelope(
                        pairing,
                        PairingElGamalEnvelopeOverrides {
                            c1: Some(mul_g1(pairing, generator_g1(pairing), rho.clone())),
                            c2: Some(add_g1(
                                pairing,
                                a.clone(),
                                mul_g1(pairing, key_pair.pk1.clone(), rho),
                            )),
                            d1: Some(other_d1),
                            d2: Some(add_g2(
                                pairing,
                                b.clone(),
                                mul_g2(pairing, key_pair.pk2.clone(), sigma),
                            )),
                        },
                    );

                    // Act
                    let result = adapter.unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(&key_pair, &envelope, Default::default()),
                    );

                    // Assert
                    let success = result.expect("a well-formed envelope unwraps");
                    assert_ne!(
                        encoded_g2(pairing, success.credential.b.clone()),
                        encoded_g2(pairing, b)
                    );
                    assert_eq!(
                        encoded_g1(pairing, success.credential.a.clone()),
                        encoded_g1(pairing, a)
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: over components whose secrets are distinct and nonzero,
///   `key_pair_from_components` returns `Ok` whose key pair holds the secrets
///   moved, `pk1 = mul_g1(g1, x)`, and `pk2 = mul_g2(g2, y)`.
/// Arrange: `build_key_pair_from_components_payload` at its defaults, built a
///   second time from the same defaults so the expected secrets survive the
///   move; the expected points from the received pairing's `mul_g1` and
///   `mul_g2` over the generators by the expected scalars.
/// Act:     `key_pair_from_components`.
/// Assert:  `key_pair.x` and `key_pair.y` encode equal to the expected
///   scalars; `key_pair.pk1` and `key_pair.pk2` to the expected points.
#[test]
fn key_pair_from_components_returns_the_key_pair_its_secrets_form() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let expected =
                        build_key_pair_from_components_payload(pairing, Default::default());
                    let expected_x = expected.components.x.expose().clone();
                    let expected_y = expected.components.y.expose().clone();
                    let expected_pk1 = mul_g1(pairing, generator_g1(pairing), expected_x.clone());
                    let expected_pk2 = mul_g2(pairing, generator_g2(pairing), expected_y.clone());
                    let payload =
                        build_key_pair_from_components_payload(pairing, Default::default());

                    // Act
                    let result = adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    let success = result.expect("well-formed components construct a key pair");
                    assert_eq!(
                        encoded_scalar(pairing, success.key_pair.x.expose().clone()),
                        encoded_scalar(pairing, expected_x)
                    );
                    assert_eq!(
                        encoded_scalar(pairing, success.key_pair.y.expose().clone()),
                        encoded_scalar(pairing, expected_y)
                    );
                    assert_eq!(
                        encoded_g1(pairing, success.key_pair.pk1.clone()),
                        encoded_g1(pairing, expected_pk1)
                    );
                    assert_eq!(
                        encoded_g2(pairing, success.key_pair.pk2.clone()),
                        encoded_g2(pairing, expected_pk2)
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: over components that pass the identity, shared-secret, and proof
///   checks, `public_keys_from_components` returns `Ok` whose public keys
///   hold `pk1` and `pk2` moved.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults;
///   `build_pairing_elgamal_possession(&adapter, &key_pair, Default::default())`,
///   the proof valid for that key pair;
///   `build_public_keys_from_components_payload` with the possession handed
///   and `components` at its defaults, which are the default key pair's
///   public keys.
/// Act:     `public_keys_from_components`.
/// Assert:  `public_keys.pk1` and `public_keys.pk2` encode equal to the key
///   pair's `pk1` and `pk2`.
#[test]
fn public_keys_from_components_returns_the_public_keys_the_components_hold() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
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
                    let success = result.expect("valid components construct public keys");
                    assert_eq!(
                        encoded_g1(pairing, success.public_keys.pk1.clone()),
                        encoded_g1(pairing, key_pair.pk1.clone())
                    );
                    assert_eq!(
                        encoded_g2(pairing, success.public_keys.pk2.clone()),
                        encoded_g2(pairing, key_pair.pk2.clone())
                    );
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `key_pair_components` returns `Ok` holding `x` and `y` as new
///   `Secret`s of the key pair's secrets.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults, whose `x` and
///   `y` differ.
/// Act:     `key_pair_components`.
/// Assert:  `components.x` and `components.y` encode through `encode_scalar`
///   equal to the key pair's `x` and `y` respectively, and differ from each
///   other.
#[test]
fn key_pair_components_returns_new_secrets_holding_the_key_pairs_secrets() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());

                    // Act
                    let result = adapter.key_pair_components(
                        build_key_pair_components_params(Default::default()),
                        build_key_pair_components_payload(&key_pair, Default::default()),
                    );

                    // Assert
                    let success = result.expect("component extraction is infallible");
                    let extracted_x =
                        encoded_scalar(pairing, success.components.x.expose().clone());
                    let extracted_y =
                        encoded_scalar(pairing, success.components.y.expose().clone());
                    assert_eq!(
                        extracted_x,
                        encoded_scalar(pairing, key_pair.x.expose().clone())
                    );
                    assert_eq!(
                        extracted_y,
                        encoded_scalar(pairing, key_pair.y.expose().clone())
                    );
                    assert_ne!(extracted_x, extracted_y);
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `key_pair_public_keys` returns `Ok` holding clones of the key
///   pair's `pk1` and `pk2`.
/// Arrange: `build_pairing_elgamal_key_pair` at its defaults, whose public
///   keys differ from the generators.
/// Act:     `key_pair_public_keys`.
/// Assert:  `components.pk1` and `components.pk2` encode equal to the key
///   pair's `pk1` and `pk2` and differ from the generators.
#[test]
fn key_pair_public_keys_returns_the_key_pairs_public_keys() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let key_pair = build_pairing_elgamal_key_pair(pairing, Default::default());

                    // Act
                    let result = adapter.key_pair_public_keys(
                        build_key_pair_public_keys_params(Default::default()),
                        build_key_pair_public_keys_payload(&key_pair, Default::default()),
                    );

                    // Assert
                    let success = result.expect("component extraction is infallible");
                    let extracted_pk1 = encoded_g1(pairing, success.components.pk1.clone());
                    let extracted_pk2 = encoded_g2(pairing, success.components.pk2.clone());
                    assert_eq!(extracted_pk1, encoded_g1(pairing, key_pair.pk1.clone()));
                    assert_eq!(extracted_pk2, encoded_g2(pairing, key_pair.pk2.clone()));
                    assert_ne!(extracted_pk1, encoded_g1(pairing, generator_g1(pairing)));
                    assert_ne!(extracted_pk2, encoded_g2(pairing, generator_g2(pairing)));
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `public_keys_components` returns `Ok` holding clones of `pk1`
///   and `pk2`.
/// Arrange: `build_pairing_elgamal_public_keys` at its defaults, whose points
///   differ from the generators.
/// Act:     `public_keys_components`.
/// Assert:  `components.pk1` and `components.pk2` encode equal to the public
///   keys' `pk1` and `pk2` and differ from the generators.
#[test]
fn public_keys_components_returns_the_public_keys_elements() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let public_keys =
                        build_pairing_elgamal_public_keys(pairing, Default::default());

                    // Act
                    let result = adapter.public_keys_components(
                        build_public_keys_components_params(Default::default()),
                        build_public_keys_components_payload(&public_keys, Default::default()),
                    );

                    // Assert
                    let success = result.expect("component extraction is infallible");
                    let extracted_pk1 = encoded_g1(pairing, success.components.pk1.clone());
                    let extracted_pk2 = encoded_g2(pairing, success.components.pk2.clone());
                    assert_eq!(extracted_pk1, encoded_g1(pairing, public_keys.pk1.clone()));
                    assert_eq!(extracted_pk2, encoded_g2(pairing, public_keys.pk2.clone()));
                    assert_ne!(extracted_pk1, encoded_g1(pairing, generator_g1(pairing)));
                    assert_ne!(extracted_pk2, encoded_g2(pairing, generator_g2(pairing)));
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `possession_components` returns `Ok` holding clones of `r1`,
///   `z1`, `r2`, and `z2`.
/// Arrange: `build_pairing_elgamal_possession` over the adapter and the
///   default key pair at its defaults, whose scalars differ from each other
///   and whose points differ from the generators.
/// Act:     `possession_components`.
/// Assert:  each of `components.r1`, `components.z1`, `components.r2`, and
///   `components.z2` encodes equal to the field it was read from, and
///   `components.z1` encodes differently from `components.z2`.
#[test]
fn possession_components_returns_the_possessions_elements() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
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
                    let success = result.expect("component extraction is infallible");
                    let extracted_r1 = encoded_g1(pairing, success.components.r1.clone());
                    let extracted_z1 = encoded_scalar(pairing, success.components.z1.clone());
                    let extracted_r2 = encoded_g2(pairing, success.components.r2.clone());
                    let extracted_z2 = encoded_scalar(pairing, success.components.z2.clone());
                    assert_eq!(extracted_r1, encoded_g1(pairing, possession.r1.clone()));
                    assert_eq!(extracted_z1, encoded_scalar(pairing, possession.z1.clone()));
                    assert_eq!(extracted_r2, encoded_g2(pairing, possession.r2.clone()));
                    assert_eq!(extracted_z2, encoded_scalar(pairing, possession.z2.clone()));
                    assert_ne!(extracted_z1, extracted_z2);
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `possession_from_components` returns `Ok` whose possession
///   holds the components moved.
/// Arrange: `build_possession_from_components_payload` at its defaults,
///   built a second time so the expected values survive the move.
/// Act:     `possession_from_components`.
/// Assert:  `possession.r1`, `possession.z1`, `possession.r2`, and
///   `possession.z2` encode equal to the expected components' fields, and
///   `possession.z1` encodes differently from `possession.z2`.
#[test]
fn possession_from_components_holds_the_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let expected =
                        build_possession_from_components_payload(pairing, Default::default());
                    let payload =
                        build_possession_from_components_payload(pairing, Default::default());

                    // Act
                    let result = adapter.possession_from_components(
                        build_possession_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    let success = result.expect("component reconstruction is infallible");
                    let reconstructed_r1 = encoded_g1(pairing, success.possession.r1.clone());
                    let reconstructed_z1 = encoded_scalar(pairing, success.possession.z1.clone());
                    let reconstructed_r2 = encoded_g2(pairing, success.possession.r2.clone());
                    let reconstructed_z2 = encoded_scalar(pairing, success.possession.z2.clone());
                    assert_eq!(
                        reconstructed_r1,
                        encoded_g1(pairing, expected.components.r1.clone())
                    );
                    assert_eq!(
                        reconstructed_z1,
                        encoded_scalar(pairing, expected.components.z1.clone())
                    );
                    assert_eq!(
                        reconstructed_r2,
                        encoded_g2(pairing, expected.components.r2.clone())
                    );
                    assert_eq!(
                        reconstructed_z2,
                        encoded_scalar(pairing, expected.components.z2.clone())
                    );
                    assert_ne!(reconstructed_z1, reconstructed_z2);
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `envelope_components` returns `Ok` holding clones of `c1`,
///   `c2`, `d1`, and `d2`.
/// Arrange: `build_pairing_elgamal_envelope` at its defaults, whose
///   first-group elements differ from each other and whose second-group
///   elements differ from each other.
/// Act:     `envelope_components`.
/// Assert:  each of `components.c1`, `components.c2`, `components.d1`, and
///   `components.d2` encodes equal to the field it was read from, and
///   `components.c1` encodes differently from `components.c2`.
#[test]
fn envelope_components_returns_the_envelopes_elements() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let envelope = build_pairing_elgamal_envelope(pairing, Default::default());

                    // Act
                    let result = adapter.envelope_components(
                        build_envelope_components_params(Default::default()),
                        build_envelope_components_payload(&envelope, Default::default()),
                    );

                    // Assert
                    let success = result.expect("component extraction is infallible");
                    let extracted_c1 = encoded_g1(pairing, success.components.c1.clone());
                    let extracted_c2 = encoded_g1(pairing, success.components.c2.clone());
                    let extracted_d1 = encoded_g2(pairing, success.components.d1.clone());
                    let extracted_d2 = encoded_g2(pairing, success.components.d2.clone());
                    assert_eq!(extracted_c1, encoded_g1(pairing, envelope.c1.clone()));
                    assert_eq!(extracted_c2, encoded_g1(pairing, envelope.c2.clone()));
                    assert_eq!(extracted_d1, encoded_g2(pairing, envelope.d1.clone()));
                    assert_eq!(extracted_d2, encoded_g2(pairing, envelope.d2.clone()));
                    assert_ne!(extracted_c1, extracted_c2);
                    assert_ne!(extracted_d1, extracted_d2);
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `envelope_from_components` returns `Ok` whose envelope holds
///   the components moved.
/// Arrange: `build_envelope_from_components_payload` at its defaults, built
///   a second time so the expected values survive the move.
/// Act:     `envelope_from_components`.
/// Assert:  `envelope.c1`, `envelope.c2`, `envelope.d1`, and `envelope.d2`
///   encode equal to the expected components' fields, and `envelope.c1`
///   encodes differently from `envelope.c2`.
#[test]
fn envelope_from_components_holds_the_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
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
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let pairing = self.pairing;
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let adapter = build_pairing_elgamal_key_agreement(
                        pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        Default::default(),
                    );
                    let expected =
                        build_envelope_from_components_payload(pairing, Default::default());
                    let payload =
                        build_envelope_from_components_payload(pairing, Default::default());

                    // Act
                    let result = adapter.envelope_from_components(
                        build_envelope_from_components_params(Default::default()),
                        payload,
                    );

                    // Assert
                    let success = result.expect("component reconstruction is infallible");
                    let reconstructed_c1 = encoded_g1(pairing, success.envelope.c1.clone());
                    let reconstructed_c2 = encoded_g1(pairing, success.envelope.c2.clone());
                    let reconstructed_d1 = encoded_g2(pairing, success.envelope.d1.clone());
                    let reconstructed_d2 = encoded_g2(pairing, success.envelope.d2.clone());
                    assert_eq!(
                        reconstructed_c1,
                        encoded_g1(pairing, expected.components.c1.clone())
                    );
                    assert_eq!(
                        reconstructed_c2,
                        encoded_g1(pairing, expected.components.c2.clone())
                    );
                    assert_eq!(
                        reconstructed_d1,
                        encoded_g2(pairing, expected.components.d1.clone())
                    );
                    assert_eq!(
                        reconstructed_d2,
                        encoded_g2(pairing, expected.components.d2.clone())
                    );
                    assert_ne!(reconstructed_c1, reconstructed_c2);
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}
