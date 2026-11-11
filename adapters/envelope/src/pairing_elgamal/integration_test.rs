#![allow(clippy::expect_used)]

use super::provides::{
    IPairingElGamalKeyAgreementCaseConsumer, PairingElGamalKeyAgreement,
    PairingElGamalKeyAgreementCaseParamsOverrides, PairingElGamalKeyAgreementCasePayload,
    build_pairing_elgamal_key_agreement, build_pairing_elgamal_key_agreement_case_deps,
    build_pairing_elgamal_key_agreement_case_params,
    build_pairing_elgamal_key_agreement_case_payload, run_pairing_elgamal_key_agreement_case,
};
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
    build_public_keys_components_params, build_public_keys_components_payload,
    build_public_keys_from_components_params, build_public_keys_from_components_payload,
    build_unwrap_params, build_unwrap_payload, build_wrap_to_params, build_wrap_to_payload,
};
use crate::possession_statement::provides::{
    PossessionG1StatementOverrides, PossessionG2StatementOverrides, build_possession_g1_statement,
    build_possession_g1_statement_description, build_possession_g2_statement,
    build_possession_g2_statement_description,
};
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodeParams, IDecoderAdapter,
    IEncoderAdapter, build_create_encoding_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTag,
    DomainTagConstructorParams, HashToScalarIdentifier, HashToScalarParams, HashToScalarPayload,
    IHashToScalarAdapter, build_create_hash_to_scalar_params, expected_uniform_bytes,
};
use kem::{CredentialComponentsOverrides, build_credential_components};
use pairing::{
    AddG1Params, AddG1PayloadOverrides, AddG2Params, AddG2PayloadOverrides,
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params,
    EncodeG2PayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MulG1Params, MulG1PayloadOverrides, MulG2Params,
    MulG2PayloadOverrides, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayloadOverrides, build_add_g1_payload,
    build_add_g2_payload, build_create_pairing_params, build_encode_g1_payload,
    build_encode_g2_payload, build_encode_scalar_payload, build_mul_g1_payload,
    build_mul_g2_payload, build_sample_uniform_scalar_payload,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

/// Contract: entry `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   an envelope `wrap_to` returns to a key pair's admitted public keys opens
///   under that key pair to the components wrapped.
/// Arrange: `generate_keys` over `build_generate_keys_payload` at its
///   defaults; `public_keys_from_components` over the components
///   `key_pair_public_keys` reads and the possession `generate_keys` returned;
///   `A` and `B` as `build_credential_components` points overridden to differ
///   from each other and from the generators; `wrap_to` of `(A, B)` to the
///   admitted public keys; every member of `PAIRING_CONCRETES`, each over
///   every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `unwrap` of the envelope under the key pair.
/// Assert:  `credential.a` and `credential.b` encode, through `encode_g1` and
///   `encode_g2`, equal to the encodings of `A` and `B`, and `credential.a`
///   encodes differently from `credential.b`.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_public_keys`, `public_keys_from_components`, `wrap_to`, and
///   `unwrap`.
/// Mocked:  none; the operating system's generator runs real, so the block
///   proves the opening for coins and nonces no fixture chose and does not
///   prove the generator's quality.
#[test]
fn every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let credential = build_credential_components(
                pairing,
                CredentialComponentsOverrides {
                    a: Some(
                        pairing
                            .add_g1(
                                AddG1Params,
                                build_add_g1_payload(
                                    pairing,
                                    AddG1PayloadOverrides {
                                        left: Some(g1.clone()),
                                        right: Some(g1.clone()),
                                    },
                                ),
                            )
                            .expect("the first-group generator adds")
                            .sum,
                    ),
                    b: Some(
                        pairing
                            .add_g2(
                                AddG2Params,
                                build_add_g2_payload(
                                    pairing,
                                    AddG2PayloadOverrides {
                                        left: Some(twice_g2),
                                        right: Some(g2),
                                    },
                                ),
                            )
                            .expect("second-group addition succeeds")
                            .sum,
                    ),
                },
            );
            let expected_a = credential.a.clone();
            let expected_b = credential.b.clone();
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
                .expect("wrapping to admitted public keys succeeds");

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
                .expect("unwrapping under the key pair is infallible");

            // Assert
            let encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(unwrapped.credential.a),
                        },
                    ),
                )
                .expect("the unwrapped first component encodes")
                .bytes;
            let encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(unwrapped.credential.b),
                        },
                    ),
                )
                .expect("the unwrapped second component encodes")
                .bytes;
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_a),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_b),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_eq!(encoded_a.as_ref(), expected_encoded_a.as_ref());
            assert_eq!(encoded_b.as_ref(), expected_encoded_b.as_ref());
            assert_ne!(encoded_a.as_ref(), encoded_b.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `independent_coins_change_every_envelope_element_and_still_open`;
///   two wraps of the same components to the same admitted public keys differ
///   in every element and each opens to the components.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`,
///   with `wrap_to` called twice and `envelope_components` read for each
///   envelope; every member of `PAIRING_CONCRETES`, each over every case of
///   `DECLARED_ENCODING_SELECTIONS` and of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   that declares `HashToScalarIdentifier::Keccak256V1`.
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
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let point_a = pairing
                .add_g1(
                    AddG1Params,
                    build_add_g1_payload(
                        pairing,
                        AddG1PayloadOverrides {
                            left: Some(g1.clone()),
                            right: Some(g1.clone()),
                        },
                    ),
                )
                .expect("the first-group generator adds")
                .sum;
            let point_b = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(twice_g2),
                            right: Some(g2),
                        },
                    ),
                )
                .expect("second-group addition succeeds")
                .sum;
            let first = adapter
                .wrap_to(
                    build_wrap_to_params(Default::default()),
                    build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(point_a.clone()),
                                    b: Some(point_b.clone()),
                                },
                            )),
                        },
                    ),
                )
                .expect("wrapping to admitted public keys succeeds");
            let second = adapter
                .wrap_to(
                    build_wrap_to_params(Default::default()),
                    build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(point_a.clone()),
                                    b: Some(point_b.clone()),
                                },
                            )),
                        },
                    ),
                )
                .expect("wrapping to admitted public keys succeeds");
            let first_elements = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&first.envelope, Default::default()),
                )
                .expect("an envelope's components read")
                .components;
            let second_elements = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&second.envelope, Default::default()),
                )
                .expect("an envelope's components read")
                .components;

            // Act
            let first_unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(&generated.key_pair, &first.envelope, Default::default()),
                )
                .expect("unwrapping under the key pair is infallible");
            let second_unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(&generated.key_pair, &second.envelope, Default::default()),
                )
                .expect("unwrapping under the key pair is infallible");

            // Assert
            let first_c1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(first_elements.c1),
                        },
                    ),
                )
                .expect("an envelope's first element encodes")
                .bytes;
            let first_c2 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(first_elements.c2),
                        },
                    ),
                )
                .expect("an envelope's second element encodes")
                .bytes;
            let first_d1 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(first_elements.d1),
                        },
                    ),
                )
                .expect("an envelope's third element encodes")
                .bytes;
            let first_d2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(first_elements.d2),
                        },
                    ),
                )
                .expect("an envelope's fourth element encodes")
                .bytes;
            let second_c1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(second_elements.c1),
                        },
                    ),
                )
                .expect("an envelope's first element encodes")
                .bytes;
            let second_c2 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(second_elements.c2),
                        },
                    ),
                )
                .expect("an envelope's second element encodes")
                .bytes;
            let second_d1 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(second_elements.d1),
                        },
                    ),
                )
                .expect("an envelope's third element encodes")
                .bytes;
            let second_d2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(second_elements.d2),
                        },
                    ),
                )
                .expect("an envelope's fourth element encodes")
                .bytes;
            assert_ne!(first_c1.as_ref(), second_c1.as_ref());
            assert_ne!(first_c2.as_ref(), second_c2.as_ref());
            assert_ne!(first_d1.as_ref(), second_d1.as_ref());
            assert_ne!(first_d2.as_ref(), second_d2.as_ref());
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(point_a.clone()),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(point_b.clone()),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_eq!(
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        build_encode_g1_payload(
                            pairing,
                            EncodeG1PayloadOverrides {
                                point: Some(first_unwrapped.credential.a),
                            },
                        ),
                    )
                    .expect("the unwrapped first component encodes")
                    .bytes
                    .as_ref(),
                expected_encoded_a.as_ref()
            );
            assert_eq!(
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        build_encode_g2_payload(
                            pairing,
                            EncodeG2PayloadOverrides {
                                point: Some(first_unwrapped.credential.b),
                            },
                        ),
                    )
                    .expect("the unwrapped second component encodes")
                    .bytes
                    .as_ref(),
                expected_encoded_b.as_ref()
            );
            assert_eq!(
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        build_encode_g1_payload(
                            pairing,
                            EncodeG1PayloadOverrides {
                                point: Some(second_unwrapped.credential.a),
                            },
                        ),
                    )
                    .expect("the unwrapped first component encodes")
                    .bytes
                    .as_ref(),
                expected_encoded_a.as_ref()
            );
            assert_eq!(
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        build_encode_g2_payload(
                            pairing,
                            EncodeG2PayloadOverrides {
                                point: Some(second_unwrapped.credential.b),
                            },
                        ),
                    )
                    .expect("the unwrapped second component encodes")
                    .bytes
                    .as_ref(),
                expected_encoded_b.as_ref()
            );
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `another_key_pair_opens_an_envelope_to_a_different_pair`;
///   an envelope unwrapped under a key pair other than the one it was wrapped
///   to returns components other than the wrapped ones.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`,
///   and another key pair from `generate_keys` over a payload whose draws
///   differ from the first's in every byte; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `unwrap` of the envelope under the other key pair.
/// Assert:  neither returned component encodes equal to the encoding of `A`
///   or of `B`.
/// Boundary: the private surface's chain ending at `generate_keys`, `wrap_to`,
///   and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn another_key_pair_opens_an_envelope_to_a_different_pair() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let credential = build_credential_components(
                pairing,
                CredentialComponentsOverrides {
                    a: Some(
                        pairing
                            .add_g1(
                                AddG1Params,
                                build_add_g1_payload(
                                    pairing,
                                    AddG1PayloadOverrides {
                                        left: Some(g1.clone()),
                                        right: Some(g1.clone()),
                                    },
                                ),
                            )
                            .expect("the first-group generator adds")
                            .sum,
                    ),
                    b: Some(
                        pairing
                            .add_g2(
                                AddG2Params,
                                build_add_g2_payload(
                                    pairing,
                                    AddG2PayloadOverrides {
                                        left: Some(twice_g2),
                                        right: Some(g2),
                                    },
                                ),
                            )
                            .expect("second-group addition succeeds")
                            .sum,
                    ),
                },
            );
            let expected_a = credential.a.clone();
            let expected_b = credential.b.clone();
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
                .expect("wrapping to admitted public keys succeeds");
            let other = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0x1Du8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            y_uniform: Some(build_secret(
                                vec![0x1Eu8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        },
                    ),
                )
                .expect("the other key pair's differing draws admit");

            // Act
            let unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(&other.key_pair, &wrapped.envelope, Default::default()),
                )
                .expect("unwrapping under the other key pair is infallible");

            // Assert
            let encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(unwrapped.credential.a),
                        },
                    ),
                )
                .expect("the unwrapped first component encodes")
                .bytes;
            let encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(unwrapped.credential.b),
                        },
                    ),
                )
                .expect("the unwrapped second component encodes")
                .bytes;
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_a),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_b),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_ne!(encoded_a.as_ref(), expected_encoded_a.as_ref());
            assert_ne!(encoded_a.as_ref(), expected_encoded_b.as_ref());
            assert_ne!(encoded_b.as_ref(), expected_encoded_a.as_ref());
            assert_ne!(encoded_b.as_ref(), expected_encoded_b.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `exchanged_secrets_open_an_envelope_to_a_different_pair`;
///   an envelope unwrapped under a key pair rebuilt from the original's
///   secrets exchanged returns components other than the wrapped ones.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   the secrets read by `key_pair_components`, exchanged, and rebuilt by
///   `key_pair_from_components`; every member of `PAIRING_CONCRETES`, each over
///   every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `unwrap` of the envelope under the rebuilt key pair.
/// Assert:  the constructor returns `Ok`, and neither returned component
///   encodes equal to the encoding of `A` or of `B`.
/// Boundary: the private surface's chain ending at `key_pair_components`,
///   `key_pair_from_components`, `wrap_to`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn exchanged_secrets_open_an_envelope_to_a_different_pair() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let credential = build_credential_components(
                pairing,
                CredentialComponentsOverrides {
                    a: Some(
                        pairing
                            .add_g1(
                                AddG1Params,
                                build_add_g1_payload(
                                    pairing,
                                    AddG1PayloadOverrides {
                                        left: Some(g1.clone()),
                                        right: Some(g1.clone()),
                                    },
                                ),
                            )
                            .expect("the first-group generator adds")
                            .sum,
                    ),
                    b: Some(
                        pairing
                            .add_g2(
                                AddG2Params,
                                build_add_g2_payload(
                                    pairing,
                                    AddG2PayloadOverrides {
                                        left: Some(twice_g2),
                                        right: Some(g2),
                                    },
                                ),
                            )
                            .expect("second-group addition succeeds")
                            .sum,
                    ),
                },
            );
            let expected_a = credential.a.clone();
            let expected_b = credential.b.clone();
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
                .expect("wrapping to admitted public keys succeeds");
            let secrets = adapter
                .key_pair_components(
                    build_key_pair_components_params(Default::default()),
                    build_key_pair_components_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's secret components read")
                .components;

            // Act
            let rebuilt = adapter.key_pair_from_components(
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
            );
            let unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(
                        &rebuilt
                            .as_ref()
                            .expect("exchanged secrets admit a key pair")
                            .key_pair,
                        &wrapped.envelope,
                        Default::default(),
                    ),
                )
                .expect("unwrapping under the rebuilt key pair is infallible");

            // Assert
            assert!(rebuilt.is_ok());
            let encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(unwrapped.credential.a),
                        },
                    ),
                )
                .expect("the unwrapped first component encodes")
                .bytes;
            let encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(unwrapped.credential.b),
                        },
                    ),
                )
                .expect("the unwrapped second component encodes")
                .bytes;
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_a),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_b),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_ne!(encoded_a.as_ref(), expected_encoded_a.as_ref());
            assert_ne!(encoded_a.as_ref(), expected_encoded_b.as_ref());
            assert_ne!(encoded_b.as_ref(), expected_encoded_a.as_ref());
            assert_ne!(encoded_b.as_ref(), expected_encoded_b.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
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
///   other; every member of `PAIRING_CONCRETES`, each over every case of
///   `DECLARED_ENCODING_SELECTIONS` and of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   that declares `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `unwrap` of the rebuilt envelope under the key pair.
/// Assert:  `credential.a` encodes differently from `A`, and `credential.b`
///   encodes equal to `B`.
/// Boundary: the private surface's chain ending at `wrap_to`,
///   `envelope_components`, `envelope_from_components`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn a_first_group_coin_element_from_another_envelope_changes_only_the_first_component() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let wrapped_first = adapter
                .wrap_to(
                    build_wrap_to_params(Default::default()),
                    build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(
                                        pairing
                                            .add_g1(
                                                AddG1Params,
                                                build_add_g1_payload(
                                                    pairing,
                                                    AddG1PayloadOverrides {
                                                        left: Some(g1.clone()),
                                                        right: Some(g1.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("the first-group generator adds")
                                            .sum,
                                    ),
                                    b: Some(
                                        pairing
                                            .add_g2(
                                                AddG2Params,
                                                build_add_g2_payload(
                                                    pairing,
                                                    AddG2PayloadOverrides {
                                                        left: Some(twice_g2.clone()),
                                                        right: Some(g2.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("second-group addition succeeds")
                                            .sum,
                                    ),
                                },
                            )),
                        },
                    ),
                )
                .expect("the first wrap to admitted public keys succeeds");
            let wrapped_second = adapter
                .wrap_to(
                    build_wrap_to_params(Default::default()),
                    build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(
                                        pairing
                                            .add_g1(
                                                AddG1Params,
                                                build_add_g1_payload(
                                                    pairing,
                                                    AddG1PayloadOverrides {
                                                        left: Some(g1.clone()),
                                                        right: Some(g1.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("the first-group generator adds")
                                            .sum,
                                    ),
                                    b: Some(
                                        pairing
                                            .add_g2(
                                                AddG2Params,
                                                build_add_g2_payload(
                                                    pairing,
                                                    AddG2PayloadOverrides {
                                                        left: Some(twice_g2),
                                                        right: Some(g2.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("second-group addition succeeds")
                                            .sum,
                                    ),
                                },
                            )),
                        },
                    ),
                )
                .expect("the second wrap to admitted public keys succeeds");
            let first_components = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&wrapped_first.envelope, Default::default()),
                )
                .expect("the first envelope's components read")
                .components;
            let second_components = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&wrapped_second.envelope, Default::default()),
                )
                .expect("the second envelope's components read")
                .components;

            // Act
            let rebuilt = adapter
                .envelope_from_components(
                    build_envelope_from_components_params(Default::default()),
                    build_envelope_from_components_payload(
                        pairing,
                        EnvelopeFromComponentsPayloadOverrides {
                            components: Some(build_envelope_components(
                                pairing,
                                EnvelopeComponentsOverrides {
                                    c1: Some(second_components.c1),
                                    c2: Some(first_components.c2),
                                    d1: Some(first_components.d1),
                                    d2: Some(first_components.d2),
                                },
                            )),
                        },
                    ),
                )
                .expect("envelope components rebuild an envelope")
                .envelope;
            let unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(&generated.key_pair, &rebuilt, Default::default()),
                )
                .expect("unwrapping the rebuilt envelope is infallible");

            // Assert
            let encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(unwrapped.credential.a),
                        },
                    ),
                )
                .expect("the unwrapped first component encodes")
                .bytes;
            let encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(unwrapped.credential.b),
                        },
                    ),
                )
                .expect("the unwrapped second component encodes")
                .bytes;
            let g1_plus_g1 = pairing
                .add_g1(
                    AddG1Params,
                    build_add_g1_payload(
                        pairing,
                        AddG1PayloadOverrides {
                            left: Some(g1.clone()),
                            right: Some(g1),
                        },
                    ),
                )
                .expect("the first-group generator adds")
                .sum;
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(g1_plus_g1),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let thrice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(
                                pairing
                                    .add_g2(
                                        AddG2Params,
                                        build_add_g2_payload(
                                            pairing,
                                            AddG2PayloadOverrides {
                                                left: Some(g2.clone()),
                                                right: Some(g2),
                                            },
                                        ),
                                    )
                                    .expect("the second-group generator adds")
                                    .sum,
                            ),
                        },
                    ),
                )
                .expect("second-group addition succeeds")
                .sum;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(thrice_g2),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_ne!(encoded_a.as_ref(), expected_encoded_a.as_ref());
            assert_eq!(encoded_b.as_ref(), expected_encoded_b.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry
///   `a_second_group_coin_element_from_another_envelope_changes_only_the_second_component`;
///   an envelope whose `D1` is taken from another envelope wrapped to the same
///   keys unwraps to a second component other than `B` and a first component
///   equal to `A`.
/// Arrange: as
///   `a_first_group_coin_element_from_another_envelope_changes_only_the_first_component`,
///   with `d1` of the other envelope in place of `c1`; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `unwrap` of the rebuilt envelope under the key pair.
/// Assert:  `credential.b` encodes differently from `B`, and `credential.a`
///   encodes equal to `A`.
/// Boundary: as
///   `a_first_group_coin_element_from_another_envelope_changes_only_the_first_component`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn a_second_group_coin_element_from_another_envelope_changes_only_the_second_component() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let wrapped_first = adapter
                .wrap_to(
                    build_wrap_to_params(Default::default()),
                    build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(
                                        pairing
                                            .add_g1(
                                                AddG1Params,
                                                build_add_g1_payload(
                                                    pairing,
                                                    AddG1PayloadOverrides {
                                                        left: Some(g1.clone()),
                                                        right: Some(g1.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("the first-group generator adds")
                                            .sum,
                                    ),
                                    b: Some(
                                        pairing
                                            .add_g2(
                                                AddG2Params,
                                                build_add_g2_payload(
                                                    pairing,
                                                    AddG2PayloadOverrides {
                                                        left: Some(twice_g2.clone()),
                                                        right: Some(g2.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("second-group addition succeeds")
                                            .sum,
                                    ),
                                },
                            )),
                        },
                    ),
                )
                .expect("the first wrap to admitted public keys succeeds");
            let wrapped_second = adapter
                .wrap_to(
                    build_wrap_to_params(Default::default()),
                    build_wrap_to_payload(
                        pairing,
                        &public_keys,
                        WrapToPayloadOverrides {
                            credential: Some(build_credential_components(
                                pairing,
                                CredentialComponentsOverrides {
                                    a: Some(
                                        pairing
                                            .add_g1(
                                                AddG1Params,
                                                build_add_g1_payload(
                                                    pairing,
                                                    AddG1PayloadOverrides {
                                                        left: Some(g1.clone()),
                                                        right: Some(g1.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("the first-group generator adds")
                                            .sum,
                                    ),
                                    b: Some(
                                        pairing
                                            .add_g2(
                                                AddG2Params,
                                                build_add_g2_payload(
                                                    pairing,
                                                    AddG2PayloadOverrides {
                                                        left: Some(twice_g2),
                                                        right: Some(g2.clone()),
                                                    },
                                                ),
                                            )
                                            .expect("second-group addition succeeds")
                                            .sum,
                                    ),
                                },
                            )),
                        },
                    ),
                )
                .expect("the second wrap to admitted public keys succeeds");
            let first_components = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&wrapped_first.envelope, Default::default()),
                )
                .expect("the first envelope's components read")
                .components;
            let second_components = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&wrapped_second.envelope, Default::default()),
                )
                .expect("the second envelope's components read")
                .components;

            // Act
            let rebuilt = adapter
                .envelope_from_components(
                    build_envelope_from_components_params(Default::default()),
                    build_envelope_from_components_payload(
                        pairing,
                        EnvelopeFromComponentsPayloadOverrides {
                            components: Some(build_envelope_components(
                                pairing,
                                EnvelopeComponentsOverrides {
                                    c1: Some(first_components.c1),
                                    c2: Some(first_components.c2),
                                    d1: Some(second_components.d1),
                                    d2: Some(first_components.d2),
                                },
                            )),
                        },
                    ),
                )
                .expect("envelope components rebuild an envelope")
                .envelope;
            let unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(&generated.key_pair, &rebuilt, Default::default()),
                )
                .expect("unwrapping the rebuilt envelope is infallible");

            // Assert
            let encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(unwrapped.credential.a),
                        },
                    ),
                )
                .expect("the unwrapped first component encodes")
                .bytes;
            let encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(unwrapped.credential.b),
                        },
                    ),
                )
                .expect("the unwrapped second component encodes")
                .bytes;
            let g1_plus_g1 = pairing
                .add_g1(
                    AddG1Params,
                    build_add_g1_payload(
                        pairing,
                        AddG1PayloadOverrides {
                            left: Some(g1.clone()),
                            right: Some(g1),
                        },
                    ),
                )
                .expect("the first-group generator adds")
                .sum;
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(g1_plus_g1),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let thrice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(
                                pairing
                                    .add_g2(
                                        AddG2Params,
                                        build_add_g2_payload(
                                            pairing,
                                            AddG2PayloadOverrides {
                                                left: Some(g2.clone()),
                                                right: Some(g2),
                                            },
                                        ),
                                    )
                                    .expect("the second-group generator adds")
                                    .sum,
                            ),
                        },
                    ),
                )
                .expect("second-group addition succeeds")
                .sum;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(thrice_g2),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_eq!(encoded_a.as_ref(), expected_encoded_a.as_ref());
            assert_ne!(encoded_b.as_ref(), expected_encoded_b.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `every_key_pair_holds_the_public_keys_its_secrets_form`;
///   `generate_keys` returns a key pair whose secrets are the scalars its draws
///   sample and whose public keys are those scalars times the generators.
/// Arrange: `build_generate_keys_payload` at its defaults, whose draws differ;
///   the expected scalars sampled by the received scalar type from bytes cloned
///   off the payload; the expected points from `mul_g1` and `mul_g2` over the
///   generators by those scalars; every member of `PAIRING_CONCRETES`, each
///   over every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `key_pair_components` and `key_pair_public_keys` over the key pair
///   `generate_keys` returned.
/// Assert:  the secrets encode, through `encode_scalar`, equal to the expected
///   scalars' encodings and differ from each other, and `pk1` and `pk2` encode
///   equal to the expected points' encodings.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_components`, and `key_pair_public_keys`.
/// Mocked:  none; the draws are caller-supplied, and the operating system's
///   generator runs real for the nonces.
#[test]
fn every_key_pair_holds_the_public_keys_its_secrets_form() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
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
                            point: Some(g1),
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
            let generated = adapter
                .generate_keys(build_generate_keys_params(Default::default()), draws)
                .expect("differing draws admit a key pair");

            // Act
            let secrets = adapter
                .key_pair_components(
                    build_key_pair_components_params(Default::default()),
                    build_key_pair_components_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's secret components read")
                .components;
            let keys = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;

            // Assert
            let encoded_x = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(secrets.x.expose().clone()),
                        },
                    ),
                )
                .expect("the key pair's x secret encodes")
                .bytes;
            let encoded_y = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(secrets.y.expose().clone()),
                        },
                    ),
                )
                .expect("the key pair's y secret encodes")
                .bytes;
            let expected_encoded_x = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(expected_x.expose().clone()),
                        },
                    ),
                )
                .expect("the expected x scalar encodes")
                .bytes;
            let expected_encoded_y = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(expected_y.expose().clone()),
                        },
                    ),
                )
                .expect("the expected y scalar encodes")
                .bytes;
            let encoded_pk1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(keys.pk1),
                        },
                    ),
                )
                .expect("the key pair's pk1 encodes")
                .bytes;
            let encoded_pk2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(keys.pk2),
                        },
                    ),
                )
                .expect("the key pair's pk2 encodes")
                .bytes;
            let expected_encoded_pk1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_pk1),
                        },
                    ),
                )
                .expect("the expected first-group point encodes")
                .bytes;
            let expected_encoded_pk2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_pk2),
                        },
                    ),
                )
                .expect("the expected second-group point encodes")
                .bytes;
            assert_eq!(
                encoded_x.expose().as_ref(),
                expected_encoded_x.expose().as_ref()
            );
            assert_eq!(
                encoded_y.expose().as_ref(),
                expected_encoded_y.expose().as_ref()
            );
            assert_ne!(encoded_x.expose().as_ref(), encoded_y.expose().as_ref());
            assert_eq!(encoded_pk1.as_ref(), expected_encoded_pk1.as_ref());
            assert_eq!(encoded_pk2.as_ref(), expected_encoded_pk2.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `every_wrap_returns_the_coins_its_envelope_was_formed_with`;
///   the elements of the envelope `wrap_to` returns are the coins times the
///   generators and the components masked by the coins times the public keys.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   every member of `PAIRING_CONCRETES`, each over every case of
///   `DECLARED_ENCODING_SELECTIONS` and of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   that declares `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `wrap_to` to the admitted public keys; the coins are read from the
///   return's `coins` through `Secret::expose` and the elements from
///   `envelope_components`.
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
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let pk1 = public_key_components.pk1.clone();
            let pk2 = public_key_components.pk2.clone();
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let expected_a = pairing
                .add_g1(
                    AddG1Params,
                    build_add_g1_payload(
                        pairing,
                        AddG1PayloadOverrides {
                            left: Some(g1.clone()),
                            right: Some(g1.clone()),
                        },
                    ),
                )
                .expect("the first-group generator adds")
                .sum;
            let expected_b = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(twice_g2),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("second-group addition succeeds")
                .sum;
            let credential = build_credential_components(
                pairing,
                CredentialComponentsOverrides {
                    a: Some(expected_a.clone()),
                    b: Some(expected_b.clone()),
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
                .expect("wrapping to admitted public keys succeeds");
            let elements = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&wrapped.envelope, Default::default()),
                )
                .expect("the envelope's components read")
                .components;
            let rho = wrapped.coins.rho.expose().clone();
            let sigma = wrapped.coins.sigma.expose().clone();

            // Assert
            let expected_c1 = pairing
                .mul_g1(
                    MulG1Params,
                    build_mul_g1_payload(
                        pairing,
                        MulG1PayloadOverrides {
                            point: Some(g1),
                            scalar: Some(rho.clone()),
                        },
                    ),
                )
                .expect("the first-group coin times the generator succeeds")
                .product;
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
                .expect("the second-group coin times the generator succeeds")
                .product;
            let rho_pk1 = pairing
                .mul_g1(
                    MulG1Params,
                    build_mul_g1_payload(
                        pairing,
                        MulG1PayloadOverrides {
                            point: Some(pk1),
                            scalar: Some(rho),
                        },
                    ),
                )
                .expect("the first-group coin masks the public key")
                .product;
            let sigma_pk2 = pairing
                .mul_g2(
                    MulG2Params,
                    build_mul_g2_payload(
                        pairing,
                        MulG2PayloadOverrides {
                            point: Some(pk2),
                            scalar: Some(sigma),
                        },
                    ),
                )
                .expect("the second-group coin masks the public key")
                .product;
            let expected_c2 = pairing
                .add_g1(
                    AddG1Params,
                    build_add_g1_payload(
                        pairing,
                        AddG1PayloadOverrides {
                            left: Some(expected_a),
                            right: Some(rho_pk1),
                        },
                    ),
                )
                .expect("the masked first component adds")
                .sum;
            let expected_d2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(expected_b),
                            right: Some(sigma_pk2),
                        },
                    ),
                )
                .expect("the masked second component adds")
                .sum;
            let encoded_c1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(elements.c1),
                        },
                    ),
                )
                .expect("the envelope's c1 encodes")
                .bytes;
            let encoded_c2 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(elements.c2),
                        },
                    ),
                )
                .expect("the envelope's c2 encodes")
                .bytes;
            let encoded_d1 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(elements.d1),
                        },
                    ),
                )
                .expect("the envelope's d1 encodes")
                .bytes;
            let encoded_d2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(elements.d2),
                        },
                    ),
                )
                .expect("the envelope's d2 encodes")
                .bytes;
            let expected_encoded_c1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_c1),
                        },
                    ),
                )
                .expect("the expected c1 encodes")
                .bytes;
            let expected_encoded_c2 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_c2),
                        },
                    ),
                )
                .expect("the expected c2 encodes")
                .bytes;
            let expected_encoded_d1 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_d1),
                        },
                    ),
                )
                .expect("the expected d1 encodes")
                .bytes;
            let expected_encoded_d2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_d2),
                        },
                    ),
                )
                .expect("the expected d2 encodes")
                .bytes;
            assert_eq!(encoded_c1.as_ref(), expected_encoded_c1.as_ref());
            assert_eq!(encoded_c2.as_ref(), expected_encoded_c2.as_ref());
            assert_eq!(encoded_d1.as_ref(), expected_encoded_d1.as_ref());
            assert_eq!(encoded_d2.as_ref(), expected_encoded_d2.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `public_keys_are_admitted_with_their_own_proof_of_possession`;
///   the public keys and the proof of possession `generate_keys` returned are
///   admitted by `public_keys_from_components`.
/// Arrange: `generate_keys` over `build_generate_keys_payload` at its defaults;
///   the components `key_pair_public_keys` reads; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `public_keys_from_components` over those components and the
///   possession `generate_keys` returned.
/// Assert:  the success arm is extracted with `expect`, and the admitted
///   public keys' `pk1` and `pk2` encode equal to the key pair's.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `key_pair_public_keys`, and `public_keys_from_components`, with the
///   possession descriptions' `to_fields`, the encoder, and the hash-to-scalar
///   adapter running for real.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_admitted_with_their_own_proof_of_possession() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let pk1 = public_key_components.pk1.clone();
            let pk2 = public_key_components.pk2.clone();

            // Act
            let admitted = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let admitted_components = adapter
                .public_keys_components(
                    build_public_keys_components_params(Default::default()),
                    build_public_keys_components_payload(&admitted, Default::default()),
                )
                .expect("admitted public keys' components read")
                .components;

            // Assert
            let encoded_pk1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(admitted_components.pk1),
                        },
                    ),
                )
                .expect("the admitted pk1 encodes")
                .bytes;
            let encoded_pk2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(admitted_components.pk2),
                        },
                    ),
                )
                .expect("the admitted pk2 encodes")
                .bytes;
            let expected_encoded_pk1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(pairing, EncodeG1PayloadOverrides { point: Some(pk1) }),
                )
                .expect("the key pair's pk1 encodes")
                .bytes;
            let expected_encoded_pk2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(pairing, EncodeG2PayloadOverrides { point: Some(pk2) }),
                )
                .expect("the key pair's pk2 encodes")
                .bytes;
            assert_eq!(encoded_pk1.as_ref(), expected_encoded_pk1.as_ref());
            assert_eq!(encoded_pk2.as_ref(), expected_encoded_pk2.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry
///   `public_keys_are_admitted_with_a_proof_rebuilt_from_its_components`; a
///   possession rebuilt from its components is admitted with the public keys
///   it proves.
/// Arrange: as `public_keys_are_admitted_with_their_own_proof_of_possession`,
///   with the possession read by `possession_components` and rebuilt by
///   `possession_from_components`; every member of `PAIRING_CONCRETES`, each
///   over every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
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
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let pk1 = public_key_components.pk1.clone();
            let pk2 = public_key_components.pk2.clone();
            let possession_components = adapter
                .possession_components(
                    build_possession_components_params(Default::default()),
                    build_possession_components_payload(&generated.possession, Default::default()),
                )
                .expect("a possession's components read")
                .components;
            let rebuilt_possession = adapter
                .possession_from_components(
                    build_possession_from_components_params(Default::default()),
                    build_possession_from_components_payload(
                        pairing,
                        PossessionFromComponentsPayloadOverrides {
                            components: Some(build_possession_components(
                                pairing,
                                PossessionComponentsOverrides {
                                    r1: Some(possession_components.r1),
                                    z1: Some(possession_components.z1),
                                    r2: Some(possession_components.r2),
                                    z2: Some(possession_components.z2),
                                },
                            )),
                        },
                    ),
                )
                .expect("possession components rebuild a possession")
                .possession;

            // Act
            let admitted = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &rebuilt_possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a rebuilt proof of possession admits")
                .public_keys;
            let admitted_components = adapter
                .public_keys_components(
                    build_public_keys_components_params(Default::default()),
                    build_public_keys_components_payload(&admitted, Default::default()),
                )
                .expect("admitted public keys' components read")
                .components;

            // Assert
            let encoded_pk1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(admitted_components.pk1),
                        },
                    ),
                )
                .expect("the admitted pk1 encodes")
                .bytes;
            let encoded_pk2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(admitted_components.pk2),
                        },
                    ),
                )
                .expect("the admitted pk2 encodes")
                .bytes;
            let expected_encoded_pk1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(pairing, EncodeG1PayloadOverrides { point: Some(pk1) }),
                )
                .expect("the key pair's pk1 encodes")
                .bytes;
            let expected_encoded_pk2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(pairing, EncodeG2PayloadOverrides { point: Some(pk2) }),
                )
                .expect("the key pair's pk2 encodes")
                .bytes;
            assert_eq!(encoded_pk1.as_ref(), expected_encoded_pk1.as_ref());
            assert_eq!(encoded_pk2.as_ref(), expected_encoded_pk2.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_another_key_pairs_proof_of_possession`; one
///   key pair's public keys with another key pair's possession return
///   `Err(PublicKeysFromComponentsErrorReturn::PossessionG1)`.
/// Arrange: two key pairs from `generate_keys` over payloads whose draws differ
///   in every byte; the first key pair's public-key components; the other's
///   possession; every member of `PAIRING_CONCRETES`, each over every case of
///   `DECLARED_ENCODING_SELECTIONS` and of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   that declares `HashToScalarIdentifier::Keccak256V1`.
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
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let other = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0x1Du8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            y_uniform: Some(build_secret(
                                vec![0x1Eu8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        },
                    ),
                )
                .expect("the other key pair's differing draws admit");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;

            // Act
            let result = adapter.public_keys_from_components(
                build_public_keys_from_components_params(Default::default()),
                build_public_keys_from_components_payload(
                    pairing,
                    &other.possession,
                    PublicKeysFromComponentsPayloadOverrides {
                        components: Some(public_key_components),
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

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_a_second_group_proof_from_another_key_pair`;
///   a possession holding one key pair's first-group proof and another key
///   pair's second-group proof returns
///   `Err(PublicKeysFromComponentsErrorReturn::PossessionG2)` with the first
///   key pair's public keys.
/// Arrange: as
///   `public_keys_are_refused_with_another_key_pairs_proof_of_possession`, with
///   a possession rebuilt by `possession_from_components` from the first key
///   pair's `r1` and `z1` and the other's `r2` and `z2`; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `public_keys_from_components` over the first key pair's public-key
///   components and the rebuilt possession.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::PossessionG2))`.
/// Boundary: the private surface's chain ending at `possession_components`,
///   `possession_from_components`, and `public_keys_from_components`.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_refused_with_a_second_group_proof_from_another_key_pair() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let other = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0x1Du8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            y_uniform: Some(build_secret(
                                vec![0x1Eu8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        },
                    ),
                )
                .expect("the other key pair's differing draws admit");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let first_possession_components = adapter
                .possession_components(
                    build_possession_components_params(Default::default()),
                    build_possession_components_payload(&generated.possession, Default::default()),
                )
                .expect("the first key pair's possession components read")
                .components;
            let other_possession_components = adapter
                .possession_components(
                    build_possession_components_params(Default::default()),
                    build_possession_components_payload(&other.possession, Default::default()),
                )
                .expect("the other key pair's possession components read")
                .components;
            let rebuilt_possession = adapter
                .possession_from_components(
                    build_possession_from_components_params(Default::default()),
                    build_possession_from_components_payload(
                        pairing,
                        PossessionFromComponentsPayloadOverrides {
                            components: Some(build_possession_components(
                                pairing,
                                PossessionComponentsOverrides {
                                    r1: Some(first_possession_components.r1),
                                    z1: Some(first_possession_components.z1),
                                    r2: Some(other_possession_components.r2),
                                    z2: Some(other_possession_components.z2),
                                },
                            )),
                        },
                    ),
                )
                .expect("mixed components rebuild a possession")
                .possession;

            // Act
            let result = adapter.public_keys_from_components(
                build_public_keys_from_components_params(Default::default()),
                build_public_keys_from_components_payload(
                    pairing,
                    &rebuilt_possession,
                    PublicKeysFromComponentsPayloadOverrides {
                        components: Some(public_key_components),
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

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `public_keys_are_refused_with_an_identity_first_group_key`;
///   public-key components whose `pk1` is the identity element return
///   `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG1)` before the
///   possession is checked.
/// Arrange: a key pair from `generate_keys` and its possession; the components
///   `key_pair_public_keys` reads with `pk1` replaced by `mul_g1` of
///   `g1_generator` by the scalar sampled from zero bytes; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
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
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let zero = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        vec![0x00u8; bound],
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("a full-length zero draw samples a scalar")
            .scalar;
            let identity_pk1 = pairing
                .mul_g1(
                    MulG1Params,
                    build_mul_g1_payload(
                        pairing,
                        MulG1PayloadOverrides {
                            point: Some(
                                pairing
                                    .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                                    .expect("the first-group generator reads")
                                    .point,
                            ),
                            scalar: Some(zero.expose().clone()),
                        },
                    ),
                )
                .expect("zero times the generator succeeds")
                .product;
            let components = build_public_keys_components(
                pairing,
                PublicKeysComponentsOverrides {
                    pk1: Some(identity_pk1),
                    pk2: Some(public_key_components.pk2),
                },
            );

            // Act
            let result = adapter.public_keys_from_components(
                build_public_keys_from_components_params(Default::default()),
                build_public_keys_from_components_payload(
                    pairing,
                    &generated.possession,
                    PublicKeysFromComponentsPayloadOverrides {
                        components: Some(components),
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

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `public_keys_are_refused_with_an_identity_second_group_key`;
///   public-key components whose `pk2` is the identity element and whose `pk1`
///   is the key pair's return
///   `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG2)` before the
///   possession is checked.
/// Arrange: a key pair from `generate_keys` and its possession; the components
///   `key_pair_public_keys` reads with `pk2` replaced by `mul_g2` of
///   `g2_generator` by the scalar sampled from zero bytes; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
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
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let zero = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        vec![0x00u8; bound],
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("a full-length zero draw samples a scalar")
            .scalar;
            let identity_pk2 = pairing
                .mul_g2(
                    MulG2Params,
                    build_mul_g2_payload(
                        pairing,
                        MulG2PayloadOverrides {
                            point: Some(
                                pairing
                                    .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                                    .expect("the second-group generator reads")
                                    .point,
                            ),
                            scalar: Some(zero.expose().clone()),
                        },
                    ),
                )
                .expect("zero times the generator succeeds")
                .product;
            let components = build_public_keys_components(
                pairing,
                PublicKeysComponentsOverrides {
                    pk1: Some(public_key_components.pk1),
                    pk2: Some(identity_pk2),
                },
            );

            // Act
            let result = adapter.public_keys_from_components(
                build_public_keys_from_components_params(Default::default()),
                build_public_keys_from_components_payload(
                    pairing,
                    &generated.possession,
                    PublicKeysFromComponentsPayloadOverrides {
                        components: Some(components),
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

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry
///   `public_keys_are_refused_with_a_secret_shared_across_the_groups`;
///   public-key components `pk1 = mul_g1(g1, s)` and `pk2 = mul_g2(g2, s)` for
///   one nonzero scalar return
///   `Err(PublicKeysFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: a key pair from `generate_keys` and its possession; components
///   overriding `pk1` and `pk2` to the multiples of the generators by one
///   scalar sampled from nonzero bytes; every member of `PAIRING_CONCRETES`,
///   each over every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `public_keys_from_components` over those components and the
///   possession.
/// Assert:  `assert_eq!(result.err(),
///   Some(PublicKeysFromComponentsErrorReturn::SharedSecret))`.
/// Boundary: the private surface's chain ending at `generate_keys` and
///   `public_keys_from_components`, with the pairing's `pairing_product_is_one`
///   running for real.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn public_keys_are_refused_with_a_secret_shared_across_the_groups() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
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
            .scalar;
            let shared_pk1 = pairing
                .mul_g1(
                    MulG1Params,
                    build_mul_g1_payload(
                        pairing,
                        MulG1PayloadOverrides {
                            point: Some(
                                pairing
                                    .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                                    .expect("the first-group generator reads")
                                    .point,
                            ),
                            scalar: Some(shared.expose().clone()),
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
                            point: Some(
                                pairing
                                    .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                                    .expect("the second-group generator reads")
                                    .point,
                            ),
                            scalar: Some(shared.expose().clone()),
                        },
                    ),
                )
                .expect("the shared scalar times g2 succeeds")
                .product;
            let components = build_public_keys_components(
                pairing,
                PublicKeysComponentsOverrides {
                    pk1: Some(shared_pk1),
                    pk2: Some(shared_pk2),
                },
            );

            // Act
            let result = adapter.public_keys_from_components(
                build_public_keys_from_components_params(Default::default()),
                build_public_keys_from_components_payload(
                    pairing,
                    &generated.possession,
                    PublicKeysFromComponentsPayloadOverrides {
                        components: Some(components),
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

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `a_key_pair_restored_from_its_components_opens_the_envelope`;
///   a key pair rebuilt from its components opens an envelope wrapped to the
///   original's admitted public keys.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   the key pair read by `key_pair_components` and rebuilt by
///   `key_pair_from_components`; every member of `PAIRING_CONCRETES`, each over
///   every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `unwrap` of the envelope under the restored key pair.
/// Assert:  `credential.a` and `credential.b` encode equal to the encodings of
///   `A` and `B`.
/// Boundary: the private surface's chain ending at `key_pair_components`,
///   `key_pair_from_components`, `wrap_to`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn a_key_pair_restored_from_its_components_opens_the_envelope() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let credential = build_credential_components(
                pairing,
                CredentialComponentsOverrides {
                    a: Some(
                        pairing
                            .add_g1(
                                AddG1Params,
                                build_add_g1_payload(
                                    pairing,
                                    AddG1PayloadOverrides {
                                        left: Some(g1.clone()),
                                        right: Some(g1),
                                    },
                                ),
                            )
                            .expect("the first-group generator adds")
                            .sum,
                    ),
                    b: Some(
                        pairing
                            .add_g2(
                                AddG2Params,
                                build_add_g2_payload(
                                    pairing,
                                    AddG2PayloadOverrides {
                                        left: Some(twice_g2),
                                        right: Some(g2),
                                    },
                                ),
                            )
                            .expect("second-group addition succeeds")
                            .sum,
                    ),
                },
            );
            let expected_a = credential.a.clone();
            let expected_b = credential.b.clone();
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
                .expect("wrapping to admitted public keys succeeds");
            let secrets = adapter
                .key_pair_components(
                    build_key_pair_components_params(Default::default()),
                    build_key_pair_components_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's secret components read")
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
                                    x: Some(secrets.x),
                                    y: Some(secrets.y),
                                },
                            )),
                        },
                    ),
                )
                .expect("a key pair's components restore it")
                .key_pair;

            // Act
            let unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(&restored, &wrapped.envelope, Default::default()),
                )
                .expect("unwrapping under the restored key pair is infallible");

            // Assert
            let encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(unwrapped.credential.a),
                        },
                    ),
                )
                .expect("the unwrapped first component encodes")
                .bytes;
            let encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(unwrapped.credential.b),
                        },
                    ),
                )
                .expect("the unwrapped second component encodes")
                .bytes;
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_a),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_b),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_eq!(encoded_a.as_ref(), expected_encoded_a.as_ref());
            assert_eq!(encoded_b.as_ref(), expected_encoded_b.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `an_envelope_rebuilt_from_its_components_opens`; an
///   envelope rebuilt from its components opens under the key pair to the
///   components wrapped.
/// Arrange: the arrangement of
///   `every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped`;
///   the envelope read by `envelope_components` and rebuilt by
///   `envelope_from_components`; every member of `PAIRING_CONCRETES`, each over
///   every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `unwrap` of the rebuilt envelope under the key pair.
/// Assert:  `credential.a` and `credential.b` encode equal to the encodings of
///   `A` and `B`.
/// Boundary: the private surface's chain ending at `envelope_components`,
///   `envelope_from_components`, and `unwrap`.
/// Mocked:  none; the operating system's generator runs real.
#[test]
fn an_envelope_rebuilt_from_its_components_opens() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let public_keys = adapter
                .public_keys_from_components(
                    build_public_keys_from_components_params(Default::default()),
                    build_public_keys_from_components_payload(
                        pairing,
                        &generated.possession,
                        PublicKeysFromComponentsPayloadOverrides {
                            components: Some(public_key_components),
                        },
                    ),
                )
                .expect("a key pair's own proof of possession admits")
                .public_keys;
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator reads")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator reads")
                .point;
            let twice_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(g2.clone()),
                            right: Some(g2.clone()),
                        },
                    ),
                )
                .expect("the second-group generator adds")
                .sum;
            let credential = build_credential_components(
                pairing,
                CredentialComponentsOverrides {
                    a: Some(
                        pairing
                            .add_g1(
                                AddG1Params,
                                build_add_g1_payload(
                                    pairing,
                                    AddG1PayloadOverrides {
                                        left: Some(g1.clone()),
                                        right: Some(g1),
                                    },
                                ),
                            )
                            .expect("the first-group generator adds")
                            .sum,
                    ),
                    b: Some(
                        pairing
                            .add_g2(
                                AddG2Params,
                                build_add_g2_payload(
                                    pairing,
                                    AddG2PayloadOverrides {
                                        left: Some(twice_g2),
                                        right: Some(g2),
                                    },
                                ),
                            )
                            .expect("second-group addition succeeds")
                            .sum,
                    ),
                },
            );
            let expected_a = credential.a.clone();
            let expected_b = credential.b.clone();
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
                .expect("wrapping to admitted public keys succeeds");
            let elements = adapter
                .envelope_components(
                    build_envelope_components_params(Default::default()),
                    build_envelope_components_payload(&wrapped.envelope, Default::default()),
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
                .expect("envelope components rebuild an envelope")
                .envelope;

            // Act
            let unwrapped = adapter
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(&generated.key_pair, &rebuilt, Default::default()),
                )
                .expect("unwrapping the rebuilt envelope is infallible");

            // Assert
            let encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(unwrapped.credential.a),
                        },
                    ),
                )
                .expect("the unwrapped first component encodes")
                .bytes;
            let encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(unwrapped.credential.b),
                        },
                    ),
                )
                .expect("the unwrapped second component encodes")
                .bytes;
            let expected_encoded_a = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(expected_a),
                        },
                    ),
                )
                .expect("the wrapped first component encodes")
                .bytes;
            let expected_encoded_b = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(expected_b),
                        },
                    ),
                )
                .expect("the wrapped second component encodes")
                .bytes;
            assert_eq!(encoded_a.as_ref(), expected_encoded_a.as_ref());
            assert_eq!(encoded_b.as_ref(), expected_encoded_b.as_ref());
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry
///   `each_proof_of_possession_hashes_its_challenge_under_the_declared_tag`;
///   each group's challenge is the hash-to-scalar mapping, under the
///   declaration's tag for that group, of the encoding of the possession
///   statement.
/// Arrange: `build_generate_keys_payload` at its defaults; the tags
///   `DomainTag::try_new` builds from `PairingElGamalKeyAgreement::DECLARATION`'s
///   `possession_g1_tag` and `possession_g2_tag`; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `generate_keys`.
/// Assert:  with the possession read by `possession_components` and the public
///   keys by `key_pair_public_keys`, per group the statement is built by the
///   official statement builder from the key and commitment, encoded by the
///   received encoder under the official description, and `c` is the scalar
///   the received scalar type samples from
///   `expected_uniform_bytes(&tag, bytes, S::UNIFORM_BYTES_LENGTH)`; `z` times
///   the generator encodes equal to `R` added to `c` times the key, and the
///   sides encode differently when `c` is taken under the other group's tag;
///   the consumer returns the six encoded pairs in `type Output` and the
///   block asserts each by `assert_eq!` or `assert_ne!`.
/// Boundary: the private surface's chain ending at `generate_keys`,
///   `possession_components`, and `key_pair_public_keys`, with
///   `DomainTag::try_new`, the possession descriptions' `to_fields`, the
///   encoder's `encode`, and the adapter's `hash_to_scalar` running for real.
/// Mocked:  none; the operating system's generator runs real for the nonces.
#[test]
fn each_proof_of_possession_hashes_its_challenge_under_the_declared_tag() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = (
            (Vec<u8>, Vec<u8>),
            (Vec<u8>, Vec<u8>),
            (Vec<u8>, Vec<u8>),
            (Vec<u8>, Vec<u8>),
            (Vec<u8>, Vec<u8>),
            (Vec<u8>, Vec<u8>),
        );

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let g1_tag = DomainTag::try_new(DomainTagConstructorParams {
                bytes: PairingElGamalKeyAgreement::<P, E>::DECLARATION
                    .possession_g1_tag
                    .to_vec(),
            })
            .expect("the first group's declared tag is admitted");
            let g2_tag = DomainTag::try_new(DomainTagConstructorParams {
                bytes: PairingElGamalKeyAgreement::<P, E>::DECLARATION
                    .possession_g2_tag
                    .to_vec(),
            })
            .expect("the second group's declared tag is admitted");

            // Act
            let generated = adapter
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(pairing, Default::default()),
                )
                .expect("differing draws admit a key pair");

            // Assert
            let possession_components = adapter
                .possession_components(
                    build_possession_components_params(Default::default()),
                    build_possession_components_payload(&generated.possession, Default::default()),
                )
                .expect("a possession's components read")
                .components;
            let public_key_components = adapter
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&generated.key_pair, Default::default()),
                )
                .expect("a key pair's public-key components read")
                .components;
            let g1_statement_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &build_possession_g1_statement_description(
                            pairing,
                            Default::default(),
                        ),
                    },
                    &build_possession_g1_statement(
                        pairing,
                        PossessionG1StatementOverrides {
                            key: Some(public_key_components.pk1.clone()),
                            commitment: Some(possession_components.r1.clone()),
                        },
                    ),
                )
                .expect("the first-group statement encodes")
                .bytes;
            let g2_statement_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &build_possession_g2_statement_description(
                            pairing,
                            Default::default(),
                        ),
                    },
                    &build_possession_g2_statement(
                        pairing,
                        PossessionG2StatementOverrides {
                            key: Some(public_key_components.pk2.clone()),
                            commitment: Some(possession_components.r2.clone()),
                        },
                    ),
                )
                .expect("the second-group statement encodes")
                .bytes;
            let c1 = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        expected_uniform_bytes(&g1_tag, &g1_statement_bytes, bound),
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("the declared tag's uniform input samples a scalar")
            .scalar;
            let c2 = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        expected_uniform_bytes(&g2_tag, &g2_statement_bytes, bound),
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("the declared tag's uniform input samples a scalar")
            .scalar;
            let c1_wrong_tag = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        expected_uniform_bytes(&g2_tag, &g1_statement_bytes, bound),
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("the other tag's uniform input samples a scalar")
            .scalar;
            let c2_wrong_tag = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        expected_uniform_bytes(&g1_tag, &g2_statement_bytes, bound),
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("the other tag's uniform input samples a scalar")
            .scalar;
            let hashed_c1 = hash_to_scalar
                .hash_to_scalar(
                    HashToScalarParams { tag: &g1_tag },
                    HashToScalarPayload {
                        message: &g1_statement_bytes,
                    },
                )
                .expect("the real hash adapter maps the statement")
                .scalar;
            let hashed_c2 = hash_to_scalar
                .hash_to_scalar(
                    HashToScalarParams { tag: &g2_tag },
                    HashToScalarPayload {
                        message: &g2_statement_bytes,
                    },
                )
                .expect("the real hash adapter maps the statement")
                .scalar;
            let lhs_g1 = pairing
                .mul_g1(
                    MulG1Params,
                    build_mul_g1_payload(
                        pairing,
                        MulG1PayloadOverrides {
                            point: Some(
                                pairing
                                    .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                                    .expect("the first-group generator reads")
                                    .point,
                            ),
                            scalar: Some(possession_components.z1.clone()),
                        },
                    ),
                )
                .expect("the first response times the generator succeeds")
                .product;
            let lhs_g2 = pairing
                .mul_g2(
                    MulG2Params,
                    build_mul_g2_payload(
                        pairing,
                        MulG2PayloadOverrides {
                            point: Some(
                                pairing
                                    .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                                    .expect("the second-group generator reads")
                                    .point,
                            ),
                            scalar: Some(possession_components.z2.clone()),
                        },
                    ),
                )
                .expect("the second response times the generator succeeds")
                .product;
            let rhs_g1 = pairing
                .add_g1(
                    AddG1Params,
                    build_add_g1_payload(
                        pairing,
                        AddG1PayloadOverrides {
                            left: Some(possession_components.r1.clone()),
                            right: Some(
                                pairing
                                    .mul_g1(
                                        MulG1Params,
                                        build_mul_g1_payload(
                                            pairing,
                                            MulG1PayloadOverrides {
                                                point: Some(public_key_components.pk1.clone()),
                                                scalar: Some(c1.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("the first challenge masks the key")
                                    .product,
                            ),
                        },
                    ),
                )
                .expect("the first-group verification side adds")
                .sum;
            let rhs_g2 = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(possession_components.r2.clone()),
                            right: Some(
                                pairing
                                    .mul_g2(
                                        MulG2Params,
                                        build_mul_g2_payload(
                                            pairing,
                                            MulG2PayloadOverrides {
                                                point: Some(public_key_components.pk2.clone()),
                                                scalar: Some(c2.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("the second challenge masks the key")
                                    .product,
                            ),
                        },
                    ),
                )
                .expect("the second-group verification side adds")
                .sum;
            let rhs_g1_wrong_tag = pairing
                .add_g1(
                    AddG1Params,
                    build_add_g1_payload(
                        pairing,
                        AddG1PayloadOverrides {
                            left: Some(possession_components.r1),
                            right: Some(
                                pairing
                                    .mul_g1(
                                        MulG1Params,
                                        build_mul_g1_payload(
                                            pairing,
                                            MulG1PayloadOverrides {
                                                point: Some(public_key_components.pk1),
                                                scalar: Some(c1_wrong_tag.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("the wrong-tag challenge masks the key")
                                    .product,
                            ),
                        },
                    ),
                )
                .expect("the wrong-tag first-group side adds")
                .sum;
            let rhs_g2_wrong_tag = pairing
                .add_g2(
                    AddG2Params,
                    build_add_g2_payload(
                        pairing,
                        AddG2PayloadOverrides {
                            left: Some(possession_components.r2),
                            right: Some(
                                pairing
                                    .mul_g2(
                                        MulG2Params,
                                        build_mul_g2_payload(
                                            pairing,
                                            MulG2PayloadOverrides {
                                                point: Some(public_key_components.pk2),
                                                scalar: Some(c2_wrong_tag.expose().clone()),
                                            },
                                        ),
                                    )
                                    .expect("the wrong-tag challenge masks the key")
                                    .product,
                            ),
                        },
                    ),
                )
                .expect("the wrong-tag second-group side adds")
                .sum;
            let encoded_hashed_c1 = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(hashed_c1),
                        },
                    ),
                )
                .expect("the hashed first challenge encodes")
                .bytes;
            let encoded_hashed_c2 = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(hashed_c2),
                        },
                    ),
                )
                .expect("the hashed second challenge encodes")
                .bytes;
            let encoded_c1 = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(c1.expose().clone()),
                        },
                    ),
                )
                .expect("the oracle's first challenge encodes")
                .bytes;
            let encoded_c2 = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    build_encode_scalar_payload(
                        pairing,
                        EncodeScalarPayloadOverrides {
                            scalar: Some(c2.expose().clone()),
                        },
                    ),
                )
                .expect("the oracle's second challenge encodes")
                .bytes;
            let encoded_lhs_g1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(lhs_g1),
                        },
                    ),
                )
                .expect("the first verification left side encodes")
                .bytes;
            let encoded_rhs_g1 = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(rhs_g1),
                        },
                    ),
                )
                .expect("the first verification right side encodes")
                .bytes;
            let encoded_lhs_g2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(lhs_g2),
                        },
                    ),
                )
                .expect("the second verification left side encodes")
                .bytes;
            let encoded_rhs_g2 = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(rhs_g2),
                        },
                    ),
                )
                .expect("the second verification right side encodes")
                .bytes;
            let encoded_rhs_g1_wrong_tag = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(rhs_g1_wrong_tag),
                        },
                    ),
                )
                .expect("the wrong-tag first side encodes")
                .bytes;
            let encoded_rhs_g2_wrong_tag = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(rhs_g2_wrong_tag),
                        },
                    ),
                )
                .expect("the wrong-tag second side encodes")
                .bytes;
            (
                (
                    encoded_hashed_c1.expose().as_ref().to_vec(),
                    encoded_c1.expose().as_ref().to_vec(),
                ),
                (
                    encoded_hashed_c2.expose().as_ref().to_vec(),
                    encoded_c2.expose().as_ref().to_vec(),
                ),
                (
                    encoded_lhs_g1.as_ref().to_vec(),
                    encoded_rhs_g1.as_ref().to_vec(),
                ),
                (
                    encoded_lhs_g2.as_ref().to_vec(),
                    encoded_rhs_g2.as_ref().to_vec(),
                ),
                (
                    encoded_lhs_g1.as_ref().to_vec(),
                    encoded_rhs_g1_wrong_tag.as_ref().to_vec(),
                ),
                (
                    encoded_lhs_g2.as_ref().to_vec(),
                    encoded_rhs_g2_wrong_tag.as_ref().to_vec(),
                ),
            )
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                let (c1_pair, c2_pair, g1_pair, g2_pair, g1_wrong_tag, g2_wrong_tag) =
                    created.expect("the case admits the selection").output;
                assert_eq!(c1_pair.0, c1_pair.1);
                assert_eq!(c2_pair.0, c2_pair.1);
                assert_eq!(g1_pair.0, g1_pair.1);
                assert_eq!(g2_pair.0, g2_pair.1);
                assert_ne!(g1_wrong_tag.0, g1_wrong_tag.1);
                assert_ne!(g2_wrong_tag.0, g2_wrong_tag.1);
            }
        }
    }
}

/// Contract: entry `every_concrete_refuses_equal_secret_draws`; equal draws
///   return `Err(GenerateKeysErrorReturn::SharedSecret)`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to the
///   bytes of the default `x_uniform`; every member of `PAIRING_CONCRETES`,
///   each over every case of `DECLARED_ENCODING_SELECTIONS` and of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::SharedSecret))`; the default payload returns
///   `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`, with the
///   pairing's `pairing_product_is_one` running for real.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_equal_secret_draws() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let default = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(pairing, Default::default()),
            );
            let result = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(
                    pairing,
                    GenerateKeysPayloadOverrides {
                        y_uniform: Some(build_secret(
                            vec![0x0Du8; bound],
                            SecretConstructorParamsOverrides,
                        )),
                        ..Default::default()
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
            assert_eq!(result.err(), Some(GenerateKeysErrorReturn::SharedSecret));
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `every_concrete_refuses_a_key_pair_of_equal_secrets`;
///   components whose `x` and `y` are one nonzero scalar return
///   `Err(KeyPairFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: `build_key_pair_from_components_payload` with `y` overridden to a
///   `Secret` of the scalar the default `x` holds; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::SharedSecret))`; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at `key_pair_from_components`,
///   with the pairing's `pairing_product_is_one` running for real.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_key_pair_of_equal_secrets() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let shared = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        vec![0x01u8; bound],
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("a nonzero uniform input samples a scalar")
            .scalar;

            // Act
            let default = adapter.key_pair_from_components(
                build_key_pair_from_components_params(Default::default()),
                build_key_pair_from_components_payload(pairing, Default::default()),
            );
            let result = adapter.key_pair_from_components(
                build_key_pair_from_components_params(Default::default()),
                build_key_pair_from_components_payload(
                    pairing,
                    KeyPairFromComponentsPayloadOverrides {
                        components: Some(build_key_pair_components(
                            pairing,
                            KeyPairComponentsOverrides {
                                x: None,
                                y: Some(shared),
                            },
                        )),
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
            assert_eq!(
                result.err(),
                Some(KeyPairFromComponentsErrorReturn::SharedSecret)
            );
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            // Act
            let created = run_pairing_elgamal_key_agreement_case(
                &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                build_pairing_elgamal_key_agreement_case_params(
                    PairingElGamalKeyAgreementCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        })),
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding_case.concrete),
                                identifier: Some(encoding_case.identifier),
                            },
                        )),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_pairing_elgamal_key_agreement_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: entry
///   `every_concrete_refuses_a_zero_x_draw_as_an_identity_first_group_key`;
///   a zero `x` draw with a nonzero `y` draw returns
///   `Err(GenerateKeysErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_generate_keys_payload` with `x_uniform` overridden to
///   `build_secret` over zero bytes of the bound's length; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG1))`; the default payload
///   returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_zero_x_draw_as_an_identity_first_group_key() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let default = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(pairing, Default::default()),
            );
            let result = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(
                    pairing,
                    GenerateKeysPayloadOverrides {
                        x_uniform: Some(build_secret(
                            vec![0u8; bound],
                            SecretConstructorParamsOverrides,
                        )),
                        ..Default::default()
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
            assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG1));
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry
///   `every_concrete_refuses_a_zero_y_draw_as_an_identity_second_group_key`;
///   a nonzero `x` draw with a zero `y` draw returns
///   `Err(GenerateKeysErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to
///   `build_secret` over zero bytes of the bound's length; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::IdentityKeyG2))`; the default payload
///   returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_zero_y_draw_as_an_identity_second_group_key() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let default = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(pairing, Default::default()),
            );
            let result = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(
                    pairing,
                    GenerateKeysPayloadOverrides {
                        y_uniform: Some(build_secret(
                            vec![0u8; bound],
                            SecretConstructorParamsOverrides,
                        )),
                        ..Default::default()
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
            assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG2));
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `every_concrete_refuses_a_key_pair_of_a_zero_x_secret`;
///   components whose `x` is the zero scalar and whose `y` is nonzero return
///   `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_key_pair_from_components_payload` with `x` overridden to
///   the `Secret` the scalar type samples from zero bytes; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1))`; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at `key_pair_from_components`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_key_pair_of_a_zero_x_secret() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let zero = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        vec![0u8; bound],
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("zero bytes sample the zero scalar")
            .scalar;

            // Act
            let default = adapter.key_pair_from_components(
                build_key_pair_from_components_params(Default::default()),
                build_key_pair_from_components_payload(pairing, Default::default()),
            );
            let result = adapter.key_pair_from_components(
                build_key_pair_from_components_params(Default::default()),
                build_key_pair_from_components_payload(
                    pairing,
                    KeyPairFromComponentsPayloadOverrides {
                        components: Some(build_key_pair_components(
                            pairing,
                            KeyPairComponentsOverrides {
                                x: Some(zero),
                                y: None,
                            },
                        )),
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
            assert_eq!(
                result.err(),
                Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1)
            );
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            // Act
            let created = run_pairing_elgamal_key_agreement_case(
                &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                build_pairing_elgamal_key_agreement_case_params(
                    PairingElGamalKeyAgreementCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        })),
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding_case.concrete),
                                identifier: Some(encoding_case.identifier),
                            },
                        )),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_pairing_elgamal_key_agreement_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: entry `every_concrete_refuses_a_key_pair_of_a_zero_y_secret`;
///   components whose `y` is the zero scalar and whose `x` is nonzero return
///   `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_key_pair_from_components_payload` with `y` overridden to
///   the `Secret` the scalar type samples from zero bytes; every member of
///   `PAIRING_CONCRETES`, each over every case of `DECLARED_ENCODING_SELECTIONS`
///   and of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `key_pair_from_components`.
/// Assert:  `assert_eq!(result.err(),
///   Some(KeyPairFromComponentsErrorReturn::IdentityKeyG2))`; the default
///   payload returns `Ok`.
/// Boundary: the private surface's chain ending at `key_pair_from_components`.
/// Mocked:  none.
#[test]
fn every_concrete_refuses_a_key_pair_of_a_zero_y_secret() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let zero = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                    uniform: Some(build_secret(
                        vec![0u8; bound],
                        SecretConstructorParamsOverrides,
                    )),
                }),
            )
            .expect("zero bytes sample the zero scalar")
            .scalar;

            // Act
            let default = adapter.key_pair_from_components(
                build_key_pair_from_components_params(Default::default()),
                build_key_pair_from_components_payload(pairing, Default::default()),
            );
            let result = adapter.key_pair_from_components(
                build_key_pair_from_components_params(Default::default()),
                build_key_pair_from_components_payload(
                    pairing,
                    KeyPairFromComponentsPayloadOverrides {
                        components: Some(build_key_pair_components(
                            pairing,
                            KeyPairComponentsOverrides {
                                x: None,
                                y: Some(zero),
                            },
                        )),
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
            assert_eq!(
                result.err(),
                Some(KeyPairFromComponentsErrorReturn::IdentityKeyG2)
            );
        }
    }

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            // Act
            let created = run_pairing_elgamal_key_agreement_case(
                &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                build_pairing_elgamal_key_agreement_case_params(
                    PairingElGamalKeyAgreementCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        })),
                        encoding: Some(build_create_encoding_params(
                            CreateEncodingParamsOverrides {
                                concrete: Some(encoding_case.concrete),
                                identifier: Some(encoding_case.identifier),
                            },
                        )),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_pairing_elgamal_key_agreement_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: entry `every_concrete_returns_an_x_draw_sampling_refusal_whole`;
///   an `x` draw one byte shorter than the scalar type's bound returns the
///   sampling refusal whole in `XSampling`.
/// Arrange: `build_generate_keys_payload` with `x_uniform` overridden to
///   `build_secret` over one byte fewer than `S::UNIFORM_BYTES_LENGTH`; every
///   member of `PAIRING_CONCRETES`, each over every case of
///   `DECLARED_ENCODING_SELECTIONS` and of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   that declares `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::XSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual })))`,
///   `expected` the bound and `actual` the bound less one; the default payload
///   returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_returns_an_x_draw_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let default = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(pairing, Default::default()),
            );
            let result = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(
                    pairing,
                    GenerateKeysPayloadOverrides {
                        x_uniform: Some(build_secret(
                            vec![0x0Du8; bound - 1],
                            SecretConstructorParamsOverrides,
                        )),
                        ..Default::default()
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
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

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}

/// Contract: entry `every_concrete_returns_a_y_draw_sampling_refusal_whole`;
///   a `y` draw one byte shorter than the scalar type's bound returns the
///   sampling refusal whole in `YSampling`.
/// Arrange: `build_generate_keys_payload` with `y_uniform` overridden to
///   `build_secret` over one byte fewer than `S::UNIFORM_BYTES_LENGTH`; every
///   member of `PAIRING_CONCRETES`, each over every case of
///   `DECLARED_ENCODING_SELECTIONS` and of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   that declares `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `generate_keys`.
/// Assert:  `assert_eq!(result.err(),
///   Some(GenerateKeysErrorReturn::YSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual })))`,
///   `expected` the bound and `actual` the bound less one; the default payload
///   returns `Ok`.
/// Boundary: the private surface's chain ending at `generate_keys`.
/// Mocked:  none.
#[test]
fn every_concrete_returns_a_y_draw_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingElGamalKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_pairing_elgamal_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _payload: &PairingElGamalKeyAgreementCasePayload,
        ) -> Self::Output {
            // Arrange
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let adapter = build_pairing_elgamal_key_agreement(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let default = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(pairing, Default::default()),
            );
            let result = adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload(
                    pairing,
                    GenerateKeysPayloadOverrides {
                        y_uniform: Some(build_secret(
                            vec![0x0Eu8; bound - 1],
                            SecretConstructorParamsOverrides,
                        )),
                        ..Default::default()
                    },
                ),
            );

            // Assert
            assert!(default.is_ok());
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

    for concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                // Act
                let created = run_pairing_elgamal_key_agreement_case(
                    &build_pairing_elgamal_key_agreement_case_deps(Consumer, Default::default()),
                    build_pairing_elgamal_key_agreement_case_params(
                        PairingElGamalKeyAgreementCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_case.concrete),
                                    identifier: Some(encoding_case.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_case.concrete),
                                    identifier: Some(hash_case.identifier),
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        },
                    ),
                    build_pairing_elgamal_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}
