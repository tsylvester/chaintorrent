#![allow(clippy::expect_used)]

use super::provides::{PairingElGamalKeyAgreement, build_pairing_elgamal_key_agreement};
use crate::factory::provides::{
    EnvelopeComponentsOverrides, EnvelopeFromComponentsPayloadOverrides, GenerateKeysErrorReturn,
    GenerateKeysPayloadOverrides, IKeyAgreementAdapter, KeyPairComponentsOverrides,
    KeyPairFromComponentsErrorReturn, KeyPairFromComponentsPayloadOverrides,
    PossessionComponentsOverrides, PossessionFromComponentsPayloadOverrides,
    PublicKeysComponentsOverrides, PublicKeysFromComponentsErrorReturn,
    PublicKeysFromComponentsPayloadOverrides, WrapToPayloadOverrides, build_envelope_components,
    build_envelope_components_params, build_envelope_components_payload,
    build_envelope_from_components_params, build_envelope_from_components_payload,
    build_generate_keys_params, build_generate_keys_payload, build_key_pair_components,
    build_key_pair_components_params, build_key_pair_components_payload,
    build_key_pair_from_components_params, build_key_pair_from_components_payload,
    build_key_pair_public_keys_params, build_key_pair_public_keys_payload,
    build_possession_components, build_possession_components_params,
    build_possession_components_payload, build_possession_from_components_params,
    build_possession_from_components_payload, build_public_keys_components,
    build_public_keys_from_components_params, build_public_keys_from_components_payload,
    build_unwrap_params, build_unwrap_payload, build_wrap_to_params, build_wrap_to_payload,
};
use crate::possession_statement::provides::{
    PossessionG1StatementOverrides, PossessionG2StatementOverrides, build_possession_g1_statement,
    build_possession_g1_statement_description, build_possession_g2_statement,
    build_possession_g2_statement_description,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, EncodeParams, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTag,
    DomainTagConstructorParams, HashToScalarIdentifier, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
    expected_uniform_bytes,
};
use kem::{CredentialComponentsOverrides, build_credential_components};
use pairing::{
    AddG1Params, AddG1PayloadOverrides, AddG2Params, AddG2PayloadOverrides, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingParamsOverrides, CreatePairingPayload, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides, EncodeScalarParams,
    EncodeScalarPayloadOverrides, IPairingAdapter, IPairingArithmetic, IPairingConsumer,
    IPairingReference, ISampleUniformScalar, MulG1Params, MulG1PayloadOverrides, MulG2Params,
    MulG2PayloadOverrides, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayloadOverrides, build_add_g1_payload,
    build_add_g2_payload, build_create_pairing_deps, build_create_pairing_params,
    build_encode_g1_payload, build_encode_g2_payload, build_encode_scalar_payload,
    build_mul_g1_payload, build_mul_g2_payload, build_sample_uniform_scalar_payload,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

fn sampled_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(build_secret(
                vec![fill; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            )),
        }),
    )
    .expect("a full-length uniform samples")
    .scalar
    .expose()
    .clone()
}

fn uniform_bytes_scalar<P: IPairingAdapter>(bytes: Vec<u8>) -> P::Scalar {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(build_secret(bytes, SecretConstructorParamsOverrides)),
        }),
    )
    .expect("a full-length uniform samples")
    .scalar
    .expose()
    .clone()
}

fn mul_g1_point<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> P::G1 {
    pairing
        .mul_g1(
            MulG1Params,
            build_mul_g1_payload(
                pairing,
                MulG1PayloadOverrides {
                    scalar: Some(scalar.clone()),
                    ..Default::default()
                },
            ),
        )
        .expect("scalar multiplication succeeds")
        .product
}

fn uniform_secret(fill: u8, bound: usize) -> Secret<Vec<u8>> {
    build_secret(vec![fill; bound], SecretConstructorParamsOverrides)
}

fn mul_g2_point<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> P::G2 {
    pairing
        .mul_g2(
            MulG2Params,
            build_mul_g2_payload(
                pairing,
                MulG2PayloadOverrides {
                    scalar: Some(scalar.clone()),
                    ..Default::default()
                },
            ),
        )
        .expect("scalar multiplication succeeds")
        .product
}

fn mul_g1_on<P: IPairingAdapter>(pairing: &P, point: &P::G1, scalar: &P::Scalar) -> P::G1 {
    pairing
        .mul_g1(
            MulG1Params,
            build_mul_g1_payload(
                pairing,
                MulG1PayloadOverrides {
                    point: Some(point.clone()),
                    scalar: Some(scalar.clone()),
                },
            ),
        )
        .expect("scalar multiplication succeeds")
        .product
}

fn mul_g2_on<P: IPairingAdapter>(pairing: &P, point: &P::G2, scalar: &P::Scalar) -> P::G2 {
    pairing
        .mul_g2(
            MulG2Params,
            build_mul_g2_payload(
                pairing,
                MulG2PayloadOverrides {
                    point: Some(point.clone()),
                    scalar: Some(scalar.clone()),
                },
            ),
        )
        .expect("scalar multiplication succeeds")
        .product
}

fn add_g1_point<P: IPairingAdapter>(pairing: &P, left: &P::G1, right: &P::G1) -> P::G1 {
    pairing
        .add_g1(
            AddG1Params,
            build_add_g1_payload(
                pairing,
                AddG1PayloadOverrides {
                    left: Some(left.clone()),
                    right: Some(right.clone()),
                },
            ),
        )
        .expect("point addition succeeds")
        .sum
}

fn add_g2_point<P: IPairingAdapter>(pairing: &P, left: &P::G2, right: &P::G2) -> P::G2 {
    pairing
        .add_g2(
            AddG2Params,
            build_add_g2_payload(
                pairing,
                AddG2PayloadOverrides {
                    left: Some(left.clone()),
                    right: Some(right.clone()),
                },
            ),
        )
        .expect("point addition succeeds")
        .sum
}

fn encode_g1_bytes<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Vec<u8> {
    pairing
        .encode_g1(
            EncodeG1Params,
            build_encode_g1_payload(
                pairing,
                EncodeG1PayloadOverrides {
                    point: Some(point.clone()),
                    ..Default::default()
                },
            ),
        )
        .expect("encoding a first-group point succeeds")
        .bytes
        .as_ref()
        .to_vec()
}

fn encode_scalar_bytes<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> Vec<u8> {
    pairing
        .encode_scalar(
            EncodeScalarParams,
            build_encode_scalar_payload(
                pairing,
                EncodeScalarPayloadOverrides {
                    scalar: Some(scalar.clone()),
                    ..Default::default()
                },
            ),
        )
        .expect("encoding a scalar succeeds")
        .bytes
        .expose()
        .as_ref()
        .to_vec()
}

fn encode_g2_bytes<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Vec<u8> {
    pairing
        .encode_g2(
            EncodeG2Params,
            build_encode_g2_payload(
                pairing,
                EncodeG2PayloadOverrides {
                    point: Some(point.clone()),
                    ..Default::default()
                },
            ),
        )
        .expect("encoding a second-group point succeeds")
        .bytes
        .as_ref()
        .to_vec()
}

/// Contract: entry `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   an envelope `wrap_to` returns to a key pair's admitted public keys opens
///   under that key pair to the components wrapped.
/// Arrange: per concrete of `PAIRING_CONCRETES`, per case of
///   `DECLARED_ENCODING_SELECTIONS`, and per case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring `Keccak256V1`: the adapter
///   `build_pairing_elgamal_key_agreement` forms over the received borrows,
///   `generate_keys` over `build_generate_keys_payload` at its defaults,
///   `public_keys_from_components` over the components `key_pair_public_keys`
///   reads and the possession `generate_keys` returned, and `wrap_to` of
///   `build_credential_components` points overridden to differ from each other
///   and from the generators.
/// Act:     `unwrap` of the envelope under the key pair.
/// Assert:  `credential.a` and `credential.b` encode, through `encode_g1` and
///   `encode_g2`, equal to the encodings of `A` and `B`, and `credential.a`
///   encodes differently from `credential.b`; each factory call asserts
///   `is_ok` in place.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_public_keys`, `public_keys_from_components`, `wrap_to`, and
///   `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let wrapped = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the wrap draws its coins");

                            // Act
                            let unwrapped = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &generated.key_pair,
                                        &wrapped.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the unwrap is infallible");

                            // Assert
                            let a_encoded = encode_g1_bytes(pairing, &unwrapped.credential.a);
                            let b_encoded = encode_g2_bytes(pairing, &unwrapped.credential.b);
                            assert_eq!(a_encoded, encode_g1_bytes(pairing, &a));
                            assert_eq!(b_encoded, encode_g2_bytes(pairing, &b));
                            assert_ne!(a_encoded, b_encoded);
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `independent_coins_change_every_envelope_element_and_still_open`;
///   two wraps of the same components to the same admitted public keys differ
///   in every element and each opens to the components.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`,
///   with `wrap_to` called twice and `envelope_components` read for each
///   envelope.
/// Act:     `unwrap` of each envelope under the key pair.
/// Assert:  `c1`, `c2`, `d1`, and `d2` of one envelope each encode differently
///   from the same element of the other, and each unwrap's components encode
///   equal to the encodings of `A` and `B`.
/// Boundary: the private surface's chain ending at `wrap_to`,
///   `envelope_components`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn independent_coins_change_every_envelope_element_and_still_open() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let first = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the first wrap draws its coins");
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let second = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the second wrap draws its coins");
                            let first_elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &first.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the first envelope's elements read")
                                .components;
                            let second_elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &second.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the second envelope's elements read")
                                .components;

                            // Act
                            let first_open = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &generated.key_pair,
                                        &first.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the unwrap is infallible");
                            let second_open = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &generated.key_pair,
                                        &second.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the unwrap is infallible");

                            // Assert
                            let a_encoded = encode_g1_bytes(pairing, &a);
                            let b_encoded = encode_g2_bytes(pairing, &b);
                            assert_ne!(
                                encode_g1_bytes(pairing, &first_elements.c1),
                                encode_g1_bytes(pairing, &second_elements.c1)
                            );
                            assert_ne!(
                                encode_g1_bytes(pairing, &first_elements.c2),
                                encode_g1_bytes(pairing, &second_elements.c2)
                            );
                            assert_ne!(
                                encode_g2_bytes(pairing, &first_elements.d1),
                                encode_g2_bytes(pairing, &second_elements.d1)
                            );
                            assert_ne!(
                                encode_g2_bytes(pairing, &first_elements.d2),
                                encode_g2_bytes(pairing, &second_elements.d2)
                            );
                            assert_eq!(
                                encode_g1_bytes(pairing, &first_open.credential.a),
                                a_encoded
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &first_open.credential.b),
                                b_encoded
                            );
                            assert_eq!(
                                encode_g1_bytes(pairing, &second_open.credential.a),
                                a_encoded
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &second_open.credential.b),
                                b_encoded
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `another_key_pair_opens_an_envelope_to_a_different_pair`;
///   an envelope unwrapped under a key pair other than the one it was wrapped
///   to returns components other than the wrapped ones.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`,
///   and another key pair from `generate_keys` over a payload whose draws
///   differ from the first's in every byte.
/// Act:     `unwrap` of the envelope under the other key pair.
/// Assert:  neither returned component encodes equal to the encoding of `A` or
///   of `B`.
/// Boundary: the private surface's chain ending at `generate_keys`, `wrap_to`,
///   and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn another_key_pair_opens_an_envelope_to_a_different_pair() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let other = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(
                                        pairing,
                                        GenerateKeysPayloadOverrides {
                                            x_uniform: Some(uniform_secret(0xA1, bound)),
                                            y_uniform: Some(uniform_secret(0xB2, bound)),
                                        },
                                    ),
                                )
                                .expect("the differing draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let wrapped = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(build_credential_components(
                                                pairing,
                                                CredentialComponentsOverrides {
                                                    a: Some(a.clone()),
                                                    b: Some(b.clone()),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the wrap draws its coins");

                            // Act
                            let unwrapped = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &other.key_pair,
                                        &wrapped.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the unwrap is infallible");

                            // Assert
                            assert_ne!(
                                encode_g1_bytes(pairing, &unwrapped.credential.a),
                                encode_g1_bytes(pairing, &a)
                            );
                            assert_ne!(
                                encode_g2_bytes(pairing, &unwrapped.credential.b),
                                encode_g2_bytes(pairing, &b)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `exchanged_secrets_open_an_envelope_to_a_different_pair`;
///   an envelope unwrapped under a key pair rebuilt from the original's
///   secrets exchanged returns components other than the wrapped ones.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   the secrets read by `key_pair_components`, exchanged, and rebuilt by
///   `key_pair_from_components`.
/// Act:     `unwrap` of the envelope under the rebuilt key pair.
/// Assert:  the constructor returns `Ok`, and neither returned component
///   encodes equal to the encoding of `A` or of `B`.
/// Boundary: the private surface's chain ending at `key_pair_components`,
///   `key_pair_from_components`, `wrap_to`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn exchanged_secrets_open_an_envelope_to_a_different_pair() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let wrapped = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(build_credential_components(
                                                pairing,
                                                CredentialComponentsOverrides {
                                                    a: Some(a.clone()),
                                                    b: Some(b.clone()),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the wrap draws its coins");
                            let secrets = adapter
                                .key_pair_components(
                                    build_key_pair_components_params(Default::default()),
                                    build_key_pair_components_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's secrets read")
                                .components;
                            let restored = adapter
                                .key_pair_from_components(
                                    build_key_pair_from_components_params(Default::default()),
                                    build_key_pair_from_components_payload(
                                        pairing,
                                        KeyPairFromComponentsPayloadOverrides {
                                            components: Some(build_key_pair_components(
                                                pairing,
                                                KeyPairComponentsOverrides {
                                                    x: Some(secrets.y),
                                                    y: Some(secrets.x),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the exchanged secrets are distinct and nonzero")
                                .key_pair;

                            // Act
                            let unwrapped = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &restored,
                                        &wrapped.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the unwrap is infallible");

                            // Assert
                            assert_ne!(
                                encode_g1_bytes(pairing, &unwrapped.credential.a),
                                encode_g1_bytes(pairing, &a)
                            );
                            assert_ne!(
                                encode_g2_bytes(pairing, &unwrapped.credential.b),
                                encode_g2_bytes(pairing, &b)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `a_first_group_coin_element_from_another_envelope_changes_only_the_first_component`;
///   an envelope whose `C1` is taken from another envelope wrapped to the same
///   keys unwraps to a first component other than `A` and a second component
///   equal to `B`.
/// Arrange: two wraps of `(A, B)` to the same admitted public keys;
///   `envelope_components` read for each; an envelope rebuilt by
///   `envelope_from_components` from the elements of one with `c1` of the
///   other.
/// Act:     `unwrap` of the rebuilt envelope under the key pair.
/// Assert:  `credential.a` encodes differently from `A`, and `credential.b`
///   encodes equal to `B`.
/// Boundary: the private surface's chain ending at `wrap_to`,
///   `envelope_components`, `envelope_from_components`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn a_first_group_coin_element_from_another_envelope_changes_only_the_first_component() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let first = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the first wrap draws its coins");
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let second = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the second wrap draws its coins");
                            let first_elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &first.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the first envelope's elements read")
                                .components;
                            let second_elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &second.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the second envelope's elements read")
                                .components;
                            let rebuilt = adapter
                                .envelope_from_components(
                                    build_envelope_from_components_params(Default::default()),
                                    build_envelope_from_components_payload(
                                        pairing,
                                        EnvelopeFromComponentsPayloadOverrides {
                                            components: Some(build_envelope_components(
                                                pairing,
                                                EnvelopeComponentsOverrides {
                                                    c1: Some(second_elements.c1),
                                                    c2: Some(first_elements.c2),
                                                    d1: Some(first_elements.d1),
                                                    d2: Some(first_elements.d2),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the envelope rebuild is infallible")
                                .envelope;

                            // Act
                            let unwrapped = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &generated.key_pair,
                                        &rebuilt,
                                        Default::default(),
                                    ),
                                )
                                .expect("the unwrap is infallible");

                            // Assert
                            assert_ne!(
                                encode_g1_bytes(pairing, &unwrapped.credential.a),
                                encode_g1_bytes(pairing, &a)
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &unwrapped.credential.b),
                                encode_g2_bytes(pairing, &b)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `a_second_group_coin_element_from_another_envelope_changes_only_the_second_component`;
///   an envelope whose `D1` is taken from another envelope wrapped to the same
///   keys unwraps to a second component other than `B` and a first component
///   equal to `A`.
/// Arrange: as
///   `a_first_group_coin_element_from_another_envelope_changes_only_the_first_component`,
///   with `d1` of the other envelope in place of `c1`.
/// Act:     `unwrap` of the rebuilt envelope under the key pair.
/// Assert:  `credential.b` encodes differently from `B`, and `credential.a`
///   encodes equal to `A`.
/// Boundary: as
///   `a_first_group_coin_element_from_another_envelope_changes_only_the_first_component`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn a_second_group_coin_element_from_another_envelope_changes_only_the_second_component() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let first = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the first wrap draws its coins");
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let second = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the second wrap draws its coins");
                            let first_elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &first.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the first envelope's elements read")
                                .components;
                            let second_elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &second.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the second envelope's elements read")
                                .components;
                            let rebuilt = adapter
                                .envelope_from_components(
                                    build_envelope_from_components_params(Default::default()),
                                    build_envelope_from_components_payload(
                                        pairing,
                                        EnvelopeFromComponentsPayloadOverrides {
                                            components: Some(build_envelope_components(
                                                pairing,
                                                EnvelopeComponentsOverrides {
                                                    c1: Some(first_elements.c1),
                                                    c2: Some(first_elements.c2),
                                                    d1: Some(second_elements.d1),
                                                    d2: Some(first_elements.d2),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the envelope rebuild is infallible")
                                .envelope;

                            // Act
                            let unwrapped = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &generated.key_pair,
                                        &rebuilt,
                                        Default::default(),
                                    ),
                                )
                                .expect("the unwrap is infallible");

                            // Assert
                            assert_eq!(
                                encode_g1_bytes(pairing, &unwrapped.credential.a),
                                encode_g1_bytes(pairing, &a)
                            );
                            assert_ne!(
                                encode_g2_bytes(pairing, &unwrapped.credential.b),
                                encode_g2_bytes(pairing, &b)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `every_key_pair_holds_the_public_keys_its_secrets_form`;
///   `generate_keys` returns a key pair whose secrets are the scalars its
///   draws sample and whose public keys are those scalars times the
///   generators.
/// Arrange: `build_generate_keys_payload` at its defaults, whose draws
///   differ; the expected scalars sampled by the received scalar type from
///   bytes cloned off the payload; the expected points from `mul_g1` and
///   `mul_g2` over the generators by those scalars.
/// Act:     `key_pair_components` and `key_pair_public_keys` over the key
///   pair `generate_keys` returned.
/// Assert:  the secrets encode, through `encode_scalar`, equal to the
///   expected scalars' encodings and differ from each other, and `pk1` and
///   `pk2` encode equal to the expected points' encodings.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_components`, and `key_pair_public_keys`.
/// Mocked:  none; the draws are caller-supplied, and the operating system's
///   generator runs real for the nonces.
#[test]
fn every_key_pair_holds_the_public_keys_its_secrets_form() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let payload =
                                build_generate_keys_payload::<P>(pairing, Default::default());
                            let x_uniform = payload.x_uniform.expose().clone();
                            let y_uniform = payload.y_uniform.expose().clone();
                            let expected_x = P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            x_uniform,
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("the x draw samples")
                            .scalar
                            .expose()
                            .clone();
                            let expected_y = P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            y_uniform,
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("the y draw samples")
                            .scalar
                            .expose()
                            .clone();
                            let expected_pk1 = mul_g1_point(pairing, &expected_x);
                            let expected_pk2 = mul_g2_point(pairing, &expected_y);

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    payload,
                                )
                                .expect("the default draws admit a key pair");

                            // Act
                            let secrets = adapter
                                .key_pair_components(
                                    build_key_pair_components_params(Default::default()),
                                    build_key_pair_components_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's secrets read")
                                .components;
                            let keys = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;

                            // Assert
                            let x_encoded = encode_scalar_bytes(pairing, secrets.x.expose());
                            let y_encoded = encode_scalar_bytes(pairing, secrets.y.expose());
                            assert_eq!(x_encoded, encode_scalar_bytes(pairing, &expected_x));
                            assert_eq!(y_encoded, encode_scalar_bytes(pairing, &expected_y));
                            assert_ne!(x_encoded, y_encoded);
                            assert_eq!(
                                encode_g1_bytes(pairing, &keys.pk1),
                                encode_g1_bytes(pairing, &expected_pk1)
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &keys.pk2),
                                encode_g2_bytes(pairing, &expected_pk2)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `every_wrap_returns_the_coins_its_envelope_was_formed_with`;
///   the elements of the envelope `wrap_to` returns are the coins times the
///   generators and the components masked by the coins times the public keys.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`,
///   with the coins read from `coins` through `Secret::expose` and the
///   elements from `envelope_components`.
/// Act:     `wrap_to` to the admitted public keys.
/// Assert:  `c1` encodes equal to `mul_g1` of `g1_generator` by `rho`, `c2` to
///   `add_g1` of `A` and `mul_g1` of `pk1` by `rho`, `d1` to `mul_g2` of
///   `g2_generator` by `sigma`, and `d2` to `add_g2` of `B` and `mul_g2` of
///   `pk2` by `sigma`.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `public_keys_from_components`, `wrap_to`, and `envelope_components`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn every_wrap_returns_the_coins_its_envelope_was_formed_with() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let pk1 = components.pk1.clone();
                            let pk2 = components.pk2.clone();
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );

                            // Act
                            let wrapped = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("the wrap draws its coins");
                            let elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &wrapped.envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the envelope's elements read")
                                .components;
                            let rho = wrapped.coins.rho.expose().clone();
                            let sigma = wrapped.coins.sigma.expose().clone();

                            // Assert
                            assert_eq!(
                                encode_g1_bytes(pairing, &elements.c1),
                                encode_g1_bytes(pairing, &mul_g1_point(pairing, &rho))
                            );
                            assert_eq!(
                                encode_g1_bytes(pairing, &elements.c2),
                                encode_g1_bytes(
                                    pairing,
                                    &add_g1_point(pairing, &a, &mul_g1_on(pairing, &pk1, &rho))
                                )
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &elements.d1),
                                encode_g2_bytes(pairing, &mul_g2_point(pairing, &sigma))
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &elements.d2),
                                encode_g2_bytes(
                                    pairing,
                                    &add_g2_point(pairing, &b, &mul_g2_on(pairing, &pk2, &sigma))
                                )
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `public_keys_are_admitted_with_their_own_proof_of_possession`; the public
///   keys and the proof of possession `generate_keys` returned are admitted
///   by `public_keys_from_components`.
/// Arrange: `generate_keys` over `build_generate_keys_payload` at its
///   defaults; the components `key_pair_public_keys` reads.
/// Act:     `public_keys_from_components` over those components and the
///   possession `generate_keys` returned.
/// Assert:  the success arm is extracted with `expect`, and the admitted
///   public keys' `pk1` and `pk2` encode equal to the key pair's.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_public_keys`, and `public_keys_from_components`, with the
///   possession descriptions' `to_fields`, the encoder, and the
///   hash-to-scalar adapter running for real.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_admitted_with_their_own_proof_of_possession() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;

                            // Act
                            let admitted = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;

                            // Assert
                            assert_eq!(
                                encode_g1_bytes(pairing, &admitted.pk1),
                                encode_g1_bytes(pairing, &generated.key_pair.pk1)
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &admitted.pk2),
                                encode_g2_bytes(pairing, &generated.key_pair.pk2)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `public_keys_are_admitted_with_a_proof_rebuilt_from_its_components`; a
///   possession rebuilt from its components is admitted with the public keys
///   it proves.
/// Arrange: as `public_keys_are_admitted_with_their_own_proof_of_possession`,
///   with the possession read by `possession_components` and rebuilt by
///   `possession_from_components`.
/// Act:     `public_keys_from_components` over the key pair's public-key
///   components and the rebuilt possession.
/// Assert:  the success arm is extracted with `expect`, and the admitted
///   public keys' `pk1` and `pk2` encode equal to the key pair's.
/// Boundary: the private surface's chain ending at `possession_components`,
///   `possession_from_components`, and `public_keys_from_components`.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_admitted_with_a_proof_rebuilt_from_its_components() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let proof = adapter
                                .possession_components(
                                    build_possession_components_params(Default::default()),
                                    build_possession_components_payload(
                                        &generated.possession,
                                        Default::default(),
                                    ),
                                )
                                .expect("the possession's components read")
                                .components;
                            let rebuilt = adapter
                                .possession_from_components(
                                    build_possession_from_components_params(Default::default()),
                                    build_possession_from_components_payload(
                                        pairing,
                                        PossessionFromComponentsPayloadOverrides {
                                            components: Some(build_possession_components(
                                                pairing,
                                                PossessionComponentsOverrides {
                                                    r1: Some(proof.r1),
                                                    z1: Some(proof.z1),
                                                    r2: Some(proof.r2),
                                                    z2: Some(proof.z2),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the possession rebuild is infallible")
                                .possession;

                            // Act
                            let admitted = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &rebuilt,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the rebuilt proof admits the public keys")
                                .public_keys;

                            // Assert
                            assert_eq!(
                                encode_g1_bytes(pairing, &admitted.pk1),
                                encode_g1_bytes(pairing, &generated.key_pair.pk1)
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &admitted.pk2),
                                encode_g2_bytes(pairing, &generated.key_pair.pk2)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_another_key_pairs_proof_of_possession`; one
///   key pair's public keys with another key pair's possession return
///   `Err(PublicKeysFromComponentsErrorReturn::PossessionG1)`.
/// Arrange: two key pairs from `generate_keys` over payloads whose draws
///   differ in every byte; the first key pair's public-key components; the
///   other's possession.
/// Act:     `public_keys_from_components` over those components and that
///   possession.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::PossessionG1))`.
/// Boundary: the private surface's chain ending at `generate_keys` and
///   `public_keys_from_components`.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_refused_with_another_key_pairs_proof_of_possession() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let other = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(
                                        pairing,
                                        GenerateKeysPayloadOverrides {
                                            x_uniform: Some(uniform_secret(0xA1, bound)),
                                            y_uniform: Some(uniform_secret(0xB2, bound)),
                                        },
                                    ),
                                )
                                .expect("the differing draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;

                            // Act
                            let result = adapter.public_keys_from_components(
                                build_public_keys_from_components_params(Default::default()),
                                build_public_keys_from_components_payload(
                                    pairing,
                                    &other.possession,
                                    PublicKeysFromComponentsPayloadOverrides {
                                        components: Some(components),
                                    },
                                ),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(PublicKeysFromComponentsErrorReturn::PossessionG1)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_a_second_group_proof_from_another_key_pair`;
///   a possession holding one key pair's first-group proof and another key
///   pair's second-group proof returns
///   `Err(PublicKeysFromComponentsErrorReturn::PossessionG2)` with the first
///   key pair's public keys.
/// Arrange: as
///   `public_keys_are_refused_with_another_key_pairs_proof_of_possession`,
///   with a possession rebuilt by `possession_from_components` from the first
///   key pair's `r1` and `z1` and the other's `r2` and `z2`.
/// Act:     `public_keys_from_components` over the first key pair's
///   public-key components and the rebuilt possession.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::PossessionG2))`.
/// Boundary: the private surface's chain ending at `possession_components`,
///   `possession_from_components`, and `public_keys_from_components`.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_refused_with_a_second_group_proof_from_another_key_pair() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let other = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(
                                        pairing,
                                        GenerateKeysPayloadOverrides {
                                            x_uniform: Some(uniform_secret(0xA1, bound)),
                                            y_uniform: Some(uniform_secret(0xB2, bound)),
                                        },
                                    ),
                                )
                                .expect("the differing draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let own_proof = adapter
                                .possession_components(
                                    build_possession_components_params(Default::default()),
                                    build_possession_components_payload(
                                        &generated.possession,
                                        Default::default(),
                                    ),
                                )
                                .expect("the possession's components read")
                                .components;
                            let other_proof = adapter
                                .possession_components(
                                    build_possession_components_params(Default::default()),
                                    build_possession_components_payload(
                                        &other.possession,
                                        Default::default(),
                                    ),
                                )
                                .expect("the other's possession components read")
                                .components;
                            let rebuilt = adapter
                                .possession_from_components(
                                    build_possession_from_components_params(Default::default()),
                                    build_possession_from_components_payload(
                                        pairing,
                                        PossessionFromComponentsPayloadOverrides {
                                            components: Some(build_possession_components(
                                                pairing,
                                                PossessionComponentsOverrides {
                                                    r1: Some(own_proof.r1),
                                                    z1: Some(own_proof.z1),
                                                    r2: Some(other_proof.r2),
                                                    z2: Some(other_proof.z2),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the possession rebuild is infallible")
                                .possession;

                            // Act
                            let result = adapter.public_keys_from_components(
                                build_public_keys_from_components_params(Default::default()),
                                build_public_keys_from_components_payload(
                                    pairing,
                                    &rebuilt,
                                    PublicKeysFromComponentsPayloadOverrides {
                                        components: Some(components),
                                    },
                                ),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(PublicKeysFromComponentsErrorReturn::PossessionG2)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_an_identity_first_group_key`; public-key
///   components whose `pk1` is the identity element return
///   `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG1)` before the
///   possession is checked.
/// Arrange: a key pair from `generate_keys` and its possession; the
///   components `key_pair_public_keys` reads with `pk1` replaced by
///   `mul_g1` of `g1_generator` by the scalar sampled from zero bytes.
/// Act:     `public_keys_from_components` over those components and the
///   possession.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG1))`.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_public_keys`, and `public_keys_from_components`.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_refused_with_an_identity_first_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;

                            // Act
                            let result = adapter.public_keys_from_components(
                                build_public_keys_from_components_params(Default::default()),
                                build_public_keys_from_components_payload(
                                    pairing,
                                    &generated.possession,
                                    PublicKeysFromComponentsPayloadOverrides {
                                        components: Some(build_public_keys_components(
                                            pairing,
                                            PublicKeysComponentsOverrides {
                                                pk1: Some(mul_g1_point(
                                                    pairing,
                                                    &sampled_scalar::<P>(0x00),
                                                )),
                                                pk2: Some(components.pk2),
                                            },
                                        )),
                                    },
                                ),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG1)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_an_identity_second_group_key`; public-key
///   components whose `pk2` is the identity element and whose `pk1` is the
///   key pair's return `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG2)`
///   before the possession is checked.
/// Arrange: a key pair from `generate_keys` and its possession; the
///   components `key_pair_public_keys` reads with `pk2` replaced by
///   `mul_g2` of `g2_generator` by the scalar sampled from zero bytes.
/// Act:     `public_keys_from_components` over those components and the
///   possession.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG2))`.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_public_keys`, and `public_keys_from_components`.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_refused_with_an_identity_second_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;

                            // Act
                            let result = adapter.public_keys_from_components(
                                build_public_keys_from_components_params(Default::default()),
                                build_public_keys_from_components_payload(
                                    pairing,
                                    &generated.possession,
                                    PublicKeysFromComponentsPayloadOverrides {
                                        components: Some(build_public_keys_components(
                                            pairing,
                                            PublicKeysComponentsOverrides {
                                                pk1: Some(components.pk1),
                                                pk2: Some(mul_g2_point(
                                                    pairing,
                                                    &sampled_scalar::<P>(0x00),
                                                )),
                                            },
                                        )),
                                    },
                                ),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(PublicKeysFromComponentsErrorReturn::IdentityKeyG2)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_a_secret_shared_across_the_groups`;
///   public-key components `pk1 = mul_g1(g1, s)` and `pk2 = mul_g2(g2, s)`
///   for one nonzero scalar return
///   `Err(PublicKeysFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: a key pair from `generate_keys` and its possession; components
///   overriding `pk1` and `pk2` to the multiples of the generators by one
///   scalar sampled from nonzero bytes.
/// Act:     `public_keys_from_components` over those components and the
///   possession.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::SharedSecret))`.
/// Boundary: the private surface's chain ending at `generate_keys` and
///   `public_keys_from_components`, with the pairing's
///   `pairing_product_is_one` running for real.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_refused_with_a_secret_shared_across_the_groups() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let shared = sampled_scalar::<P>(0x07);

                            // Act
                            let result = adapter.public_keys_from_components(
                                build_public_keys_from_components_params(Default::default()),
                                build_public_keys_from_components_payload(
                                    pairing,
                                    &generated.possession,
                                    PublicKeysFromComponentsPayloadOverrides {
                                        components: Some(build_public_keys_components(
                                            pairing,
                                            PublicKeysComponentsOverrides {
                                                pk1: Some(mul_g1_point(pairing, &shared)),
                                                pk2: Some(mul_g2_point(pairing, &shared)),
                                            },
                                        )),
                                    },
                                ),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(PublicKeysFromComponentsErrorReturn::SharedSecret)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `a_key_pair_restored_from_its_components_opens_the_envelope`; a key pair
///   rebuilt from its components opens an envelope wrapped to the original's
///   admitted public keys.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   the key pair read by `key_pair_components` and rebuilt by
///   `key_pair_from_components`.
/// Act:     `unwrap` of the envelope under the restored key pair.
/// Assert:  `credential.a` and `credential.b` encode equal to the encodings
///   of `A` and `B`.
/// Boundary: the private surface's chain ending at `key_pair_components`,
///   `key_pair_from_components`, `wrap_to`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn a_key_pair_restored_from_its_components_opens_the_envelope() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let envelope = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("a valid wrap to the admitted keys")
                                .envelope;
                            let secrets = adapter
                                .key_pair_components(
                                    build_key_pair_components_params(Default::default()),
                                    build_key_pair_components_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's components read")
                                .components;
                            let restored = adapter
                                .key_pair_from_components(
                                    build_key_pair_from_components_params(Default::default()),
                                    build_key_pair_from_components_payload(
                                        pairing,
                                        KeyPairFromComponentsPayloadOverrides {
                                            components: Some(secrets),
                                        },
                                    ),
                                )
                                .expect("the restored key pair is admitted")
                                .key_pair;

                            // Act
                            let opened = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(&restored, &envelope, Default::default()),
                                )
                                .expect("the envelope opens under the restored pair")
                                .credential;

                            // Assert
                            assert_eq!(
                                encode_g1_bytes(pairing, &opened.a),
                                encode_g1_bytes(pairing, &a)
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &opened.b),
                                encode_g2_bytes(pairing, &b)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `an_envelope_rebuilt_from_its_components_opens`; an
///   envelope rebuilt from its components opens under the key pair to the
///   components wrapped.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   the envelope read by `envelope_components` and rebuilt by
///   `envelope_from_components`.
/// Act:     `unwrap` of the rebuilt envelope under the key pair.
/// Assert:  `credential.a` and `credential.b` encode equal to the encodings
///   of `A` and `B`.
/// Boundary: the private surface's chain ending at `envelope_components`,
///   `envelope_from_components`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn an_envelope_rebuilt_from_its_components_opens() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");
                            let components = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;
                            let public_keys = adapter
                                .public_keys_from_components(
                                    build_public_keys_from_components_params(Default::default()),
                                    build_public_keys_from_components_payload(
                                        pairing,
                                        &generated.possession,
                                        PublicKeysFromComponentsPayloadOverrides {
                                            components: Some(components),
                                        },
                                    ),
                                )
                                .expect("the key pair's own proof admits its public keys")
                                .public_keys;
                            let a = mul_g1_point(pairing, &sampled_scalar::<P>(0x05));
                            let b = mul_g2_point(pairing, &sampled_scalar::<P>(0x06));
                            let credential = build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(a.clone()),
                                    b: Some(b.clone()),
                                },
                            );
                            let envelope = adapter
                                .wrap_to(
                                    build_wrap_to_params(Default::default()),
                                    build_wrap_to_payload(
                                        pairing,
                                        &public_keys,
                                        WrapToPayloadOverrides {
                                            credential: Some(credential),
                                        },
                                    ),
                                )
                                .expect("a valid wrap to the admitted keys")
                                .envelope;
                            let elements = adapter
                                .envelope_components(
                                    build_envelope_components_params(Default::default()),
                                    build_envelope_components_payload(
                                        &envelope,
                                        Default::default(),
                                    ),
                                )
                                .expect("the envelope's components read")
                                .components;
                            let rebuilt = adapter
                                .envelope_from_components(
                                    build_envelope_from_components_params(Default::default()),
                                    build_envelope_from_components_payload(
                                        pairing,
                                        EnvelopeFromComponentsPayloadOverrides {
                                            components: Some(build_envelope_components(
                                                pairing,
                                                EnvelopeComponentsOverrides {
                                                    c1: Some(elements.c1),
                                                    c2: Some(elements.c2),
                                                    d1: Some(elements.d1),
                                                    d2: Some(elements.d2),
                                                },
                                            )),
                                        },
                                    ),
                                )
                                .expect("the envelope rebuild is infallible")
                                .envelope;

                            // Act
                            let opened = adapter
                                .unwrap(
                                    build_unwrap_params(Default::default()),
                                    build_unwrap_payload(
                                        &generated.key_pair,
                                        &rebuilt,
                                        Default::default(),
                                    ),
                                )
                                .expect("the rebuilt envelope opens")
                                .credential;

                            // Assert
                            assert_eq!(
                                encode_g1_bytes(pairing, &opened.a),
                                encode_g1_bytes(pairing, &a)
                            );
                            assert_eq!(
                                encode_g2_bytes(pairing, &opened.b),
                                encode_g2_bytes(pairing, &b)
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `each_proof_of_possession_hashes_its_challenge_under_the_declared_tag`;
///   each group's challenge is the hash-to-scalar mapping, under the
///   declaration's tag for that group, of the encoding of the possession
///   statement.
/// Arrange: `build_generate_keys_payload` at its defaults; the tags
///   `DomainTag::try_new` builds from
///   `PairingElGamalKeyAgreement::DECLARATION`'s `possession_g1_tag` and
///   `possession_g2_tag`.
/// Act:     `generate_keys`.
/// Assert:  with the possession read by `possession_components` and the
///   public keys by `key_pair_public_keys`, per group the statement is built
///   by the official statement builder from the key and commitment, encoded
///   by the received encoder under the official description, and `c` is the
///   scalar the received scalar type samples from
///   `expected_uniform_bytes(&tag, bytes, S::UNIFORM_BYTES_LENGTH)`; `z`
///   times the generator encodes equal to `R` added to `c` times the key,
///   and the sides encode differently when `c` is taken under the other
///   group's tag, asserted by `assert_ne!`.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `possession_components`, and `key_pair_public_keys`, with
///   `DomainTag::try_new`, the possession descriptions' `to_fields`, the
///   encoder's `encode`, and the adapter's `hash_to_scalar` running for
///   real.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn each_proof_of_possession_hashes_its_challenge_under_the_declared_tag() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let declaration = PairingElGamalKeyAgreement::<P, E>::DECLARATION;
                            let g1_tag = DomainTag::try_new(DomainTagConstructorParams {
                                bytes: declaration.possession_g1_tag.to_vec(),
                            })
                            .expect("the declaration's first-group tag is admitted");
                            let g2_tag = DomainTag::try_new(DomainTagConstructorParams {
                                bytes: declaration.possession_g2_tag.to_vec(),
                            })
                            .expect("the declaration's second-group tag is admitted");
                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;

                            // Act
                            let generated = adapter
                                .generate_keys(
                                    build_generate_keys_params(Default::default()),
                                    build_generate_keys_payload(pairing, Default::default()),
                                )
                                .expect("the default draws admit a key pair");

                            // Assert
                            let proof = adapter
                                .possession_components(
                                    build_possession_components_params(Default::default()),
                                    build_possession_components_payload(
                                        &generated.possession,
                                        Default::default(),
                                    ),
                                )
                                .expect("the possession's components read")
                                .components;
                            let keys = adapter
                                .key_pair_public_keys(
                                    build_key_pair_public_keys_params(Default::default()),
                                    build_key_pair_public_keys_payload(
                                        &generated.key_pair,
                                        Default::default(),
                                    ),
                                )
                                .expect("the key pair's public keys read")
                                .components;

                            let g1_description = build_possession_g1_statement_description(
                                pairing,
                                Default::default(),
                            );
                            let statement1 = build_possession_g1_statement(
                                pairing,
                                PossessionG1StatementOverrides {
                                    key: Some(keys.pk1.clone()),
                                    commitment: Some(proof.r1.clone()),
                                },
                            );
                            let bytes1 = encoded
                                .adapter
                                .encode(
                                    EncodeParams {
                                        description: &g1_description,
                                    },
                                    &statement1,
                                )
                                .expect("encoding a possession statement never refuses")
                                .bytes;
                            let c1 = uniform_bytes_scalar::<P>(expected_uniform_bytes(
                                &g1_tag, &bytes1, bound,
                            ));
                            let c1_other = uniform_bytes_scalar::<P>(expected_uniform_bytes(
                                &g2_tag, &bytes1, bound,
                            ));

                            assert_eq!(
                                encode_g1_bytes(pairing, &mul_g1_point(pairing, &proof.z1)),
                                encode_g1_bytes(
                                    pairing,
                                    &add_g1_point(
                                        pairing,
                                        &proof.r1,
                                        &mul_g1_on(pairing, &keys.pk1, &c1)
                                    )
                                )
                            );
                            assert_ne!(
                                encode_g1_bytes(pairing, &mul_g1_point(pairing, &proof.z1)),
                                encode_g1_bytes(
                                    pairing,
                                    &add_g1_point(
                                        pairing,
                                        &proof.r1,
                                        &mul_g1_on(pairing, &keys.pk1, &c1_other)
                                    )
                                )
                            );

                            let g2_description = build_possession_g2_statement_description(
                                pairing,
                                Default::default(),
                            );
                            let statement2 = build_possession_g2_statement(
                                pairing,
                                PossessionG2StatementOverrides {
                                    key: Some(keys.pk2.clone()),
                                    commitment: Some(proof.r2.clone()),
                                },
                            );
                            let bytes2 = encoded
                                .adapter
                                .encode(
                                    EncodeParams {
                                        description: &g2_description,
                                    },
                                    &statement2,
                                )
                                .expect("encoding a possession statement never refuses")
                                .bytes;
                            let c2 = uniform_bytes_scalar::<P>(expected_uniform_bytes(
                                &g2_tag, &bytes2, bound,
                            ));
                            let c2_other = uniform_bytes_scalar::<P>(expected_uniform_bytes(
                                &g1_tag, &bytes2, bound,
                            ));

                            assert_eq!(
                                encode_g2_bytes(pairing, &mul_g2_point(pairing, &proof.z2)),
                                encode_g2_bytes(
                                    pairing,
                                    &add_g2_point(
                                        pairing,
                                        &proof.r2,
                                        &mul_g2_on(pairing, &keys.pk2, &c2)
                                    )
                                )
                            );
                            assert_ne!(
                                encode_g2_bytes(pairing, &mul_g2_point(pairing, &proof.z2)),
                                encode_g2_bytes(
                                    pairing,
                                    &add_g2_point(
                                        pairing,
                                        &proof.r2,
                                        &mul_g2_on(pairing, &keys.pk2, &c2_other)
                                    )
                                )
                            );
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `every_concrete_refuses_equal_secret_draws`; equal draws
///   return `Err(GenerateKeysErrorReturn::SharedSecret)`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to the
///   bytes of the default `x_uniform`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::SharedSecret))`; the default payload
///   returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`, with the
///   pairing's `pairing_product_is_one` running for real.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_equal_secret_draws() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let defaults = build_generate_keys_payload(pairing, Default::default());
                            let mut overrides = GenerateKeysPayloadOverrides::default();
                            overrides.y_uniform = Some(build_secret(
                                defaults.x_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            ));
                            let payload = build_generate_keys_payload(pairing, overrides);

                            // Assert
                            let result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                payload,
                            );
                            assert_eq!(result.err(), Some(GenerateKeysErrorReturn::SharedSecret));

                            // Act
                            let default_result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                defaults,
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_concrete_refuses_a_key_pair_of_equal_secrets`; components whose
///   `x` and `y` are one nonzero scalar return
///   `Err(KeyPairFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: `build_key_pair_from_components_payload` with `y` overridden to
///   a `Secret` of the scalar the default `x` holds.
/// Act:     `key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::SharedSecret))`; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at
///   `key_pair_from_components`, with the pairing's `pairing_product_is_one`
///   running for real.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_key_pair_of_equal_secrets() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let defaults = build_key_pair_components(pairing, Default::default());
                            let mut overrides = KeyPairFromComponentsPayloadOverrides::default();
                            overrides.components = Some(build_key_pair_components(
                                pairing,
                                KeyPairComponentsOverrides {
                                    y: Some(build_secret(
                                        defaults.x.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            ));

                            // Act
                            let result = adapter.key_pair_from_components(
                                build_key_pair_from_components_params(Default::default()),
                                build_key_pair_from_components_payload(pairing, overrides),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(KeyPairFromComponentsErrorReturn::SharedSecret)
                            );

                            // Act
                            let default_result = adapter.key_pair_from_components(
                                build_key_pair_from_components_params(Default::default()),
                                build_key_pair_from_components_payload(
                                    pairing,
                                    KeyPairFromComponentsPayloadOverrides {
                                        components: Some(defaults),
                                    },
                                ),
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_concrete_refuses_a_zero_x_draw_as_an_identity_first_group_key`;
///   a zero `x` draw with a nonzero `y` draw returns
///   `Err(GenerateKeysErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_generate_keys_payload` with `x_uniform` overridden to
///   `build_secret` over zero bytes of the bound's length.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG1))`; the default payload
///   returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_zero_x_draw_as_an_identity_first_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let payload = build_generate_keys_payload(
                                pairing,
                                GenerateKeysPayloadOverrides {
                                    x_uniform: Some(build_secret(
                                        vec![0x00; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            );

                            // Act
                            let result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                payload,
                            );

                            // Assert
                            assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG1));

                            // Act
                            let default_result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                build_generate_keys_payload(pairing, Default::default()),
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_concrete_refuses_a_zero_y_draw_as_an_identity_second_group_key`;
///   a nonzero `x` draw with a zero `y` draw returns
///   `Err(GenerateKeysErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to
///   `build_secret` over zero bytes of the bound's length.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG2))`; the default payload
///   returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_zero_y_draw_as_an_identity_second_group_key() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let payload = build_generate_keys_payload(
                                pairing,
                                GenerateKeysPayloadOverrides {
                                    y_uniform: Some(build_secret(
                                        vec![0x00; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            );

                            // Act
                            let result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                payload,
                            );

                            // Assert
                            assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG2));

                            // Act
                            let default_result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                build_generate_keys_payload(pairing, Default::default()),
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_concrete_refuses_a_key_pair_of_a_zero_x_secret`; components
///   whose `x` is the zero scalar and whose `y` is nonzero return
///   `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_key_pair_from_components_payload` with `x` overridden to
///   the `Secret` the scalar type samples from zero bytes.
/// Act:     `key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1))`; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at
///   `key_pair_from_components`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_key_pair_of_a_zero_x_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let zero_x = P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            vec![0x00; bound],
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("a full-length uniform samples")
                            .scalar;
                            let mut overrides = KeyPairFromComponentsPayloadOverrides::default();
                            overrides.components = Some(build_key_pair_components(
                                pairing,
                                KeyPairComponentsOverrides {
                                    x: Some(zero_x),
                                    ..Default::default()
                                },
                            ));

                            // Act
                            let result = adapter.key_pair_from_components(
                                build_key_pair_from_components_params(Default::default()),
                                build_key_pair_from_components_payload(pairing, overrides),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1)
                            );

                            // Act
                            let default_result = adapter.key_pair_from_components(
                                build_key_pair_from_components_params(Default::default()),
                                build_key_pair_from_components_payload(pairing, Default::default()),
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_concrete_refuses_a_key_pair_of_a_zero_y_secret`; components
///   whose `y` is the zero scalar and whose `x` is nonzero return
///   `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_key_pair_from_components_payload` with `y` overridden to
///   the `Secret` the scalar type samples from zero bytes.
/// Act:     `key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::IdentityKeyG2))`; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at
///   `key_pair_from_components`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_key_pair_of_a_zero_y_secret() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let zero_y = P::Scalar::sample_from_uniform_bytes(
                                SampleUniformScalarParams,
                                build_sample_uniform_scalar_payload(
                                    SampleUniformScalarPayloadOverrides {
                                        uniform: Some(build_secret(
                                            vec![0x00; bound],
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                ),
                            )
                            .expect("a full-length uniform samples")
                            .scalar;
                            let mut overrides = KeyPairFromComponentsPayloadOverrides::default();
                            overrides.components = Some(build_key_pair_components(
                                pairing,
                                KeyPairComponentsOverrides {
                                    y: Some(zero_y),
                                    ..Default::default()
                                },
                            ));

                            // Act
                            let result = adapter.key_pair_from_components(
                                build_key_pair_from_components_params(Default::default()),
                                build_key_pair_from_components_payload(pairing, overrides),
                            );

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(KeyPairFromComponentsErrorReturn::IdentityKeyG2)
                            );

                            // Act
                            let default_result = adapter.key_pair_from_components(
                                build_key_pair_from_components_params(Default::default()),
                                build_key_pair_from_components_payload(pairing, Default::default()),
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_concrete_returns_an_x_draw_sampling_refusal_whole`; an `x` draw
///   one byte shorter than the scalar type's bound returns the sampling
///   refusal whole in `XSampling`.
/// Arrange: `build_generate_keys_payload` with `x_uniform` overridden to
///   `build_secret` over one byte fewer than `S::UNIFORM_BYTES_LENGTH`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::XSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual })))`,
///   `expected` the bound and `actual` the bound less one; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_returns_an_x_draw_sampling_refusal_whole() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let payload = build_generate_keys_payload(
                                pairing,
                                GenerateKeysPayloadOverrides {
                                    x_uniform: Some(build_secret(
                                        vec![0x0D; bound - 1],
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            );

                            // Act
                            let result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                payload,
                            );

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

                            // Act
                            let default_result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                build_generate_keys_payload(pairing, Default::default()),
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_concrete_returns_a_y_draw_sampling_refusal_whole`; a `y` draw
///   one byte shorter than the scalar type's bound returns the sampling
///   refusal whole in `YSampling`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to
///   `build_secret` over one byte fewer than `S::UNIFORM_BYTES_LENGTH`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::YSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual })))`,
///   `expected` the bound and `actual` the bound less one; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_returns_a_y_draw_sampling_refusal_whole() {
    // Arrange
    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for encoding_case in DECLARED_ENCODING_SELECTIONS {
                // Arrange
                struct EncodingConsumer<'a, P> {
                    pairing: &'a P,
                }

                impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                    type Output = ();

                    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                        &self,
                        _params: ConsumeEncodingParams,
                        encoded: ConsumeEncodingPayload<E>,
                    ) -> Self::Output {
                        let pairing = self.pairing;
                        for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                        {
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(hash_case.concrete),
                                        identifier: Some(hash_case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating system's source constructs");
                            let adapter = build_pairing_elgamal_key_agreement(
                                pairing,
                                &*hashed.adapter,
                                &encoded.adapter,
                                &*random.adapter,
                                Default::default(),
                            );

                            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                            let payload = build_generate_keys_payload(
                                pairing,
                                GenerateKeysPayloadOverrides {
                                    y_uniform: Some(build_secret(
                                        vec![0x0E; bound - 1],
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            );

                            // Act
                            let result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                payload,
                            );

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

                            // Act
                            let default_result = adapter.generate_keys(
                                build_generate_keys_params(Default::default()),
                                build_generate_keys_payload(pairing, Default::default()),
                            );

                            // Assert
                            assert!(default_result.is_ok());
                        }
                    }
                }

                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_case.concrete),
                        identifier: Some(encoding_case.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}
