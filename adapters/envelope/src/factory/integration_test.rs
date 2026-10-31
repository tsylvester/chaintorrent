#![allow(clippy::expect_used)]

use super::create_key_agreement;
use super::provides::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementCaseParamsOverrides,
    CreateKeyAgreementCasePayload, CreateKeyAgreementParams, CreateKeyAgreementParamsOverrides,
    DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeFromComponentsPayloadOverrides,
    GenerateKeysErrorReturn, GenerateKeysPayloadOverrides, ICreateKeyAgreementCaseConsumer,
    IKeyAgreementAdapter, IKeyAgreementConsumer, KeyAgreementConcrete, KeyPairComponentsOverrides,
    KeyPairFromComponentsErrorReturn, KeyPairFromComponentsPayloadOverrides,
    MockIKeyAgreementAdapterFailureMode, MockIKeyAgreementAdapterGenerateKeysErrorReturn,
    MockIKeyAgreementAdapterWrapToErrorReturn, PossessionComponentsOverrides,
    PossessionFromComponentsPayloadOverrides, PublicKeysComponentsOverrides,
    PublicKeysFromComponentsErrorReturn, PublicKeysFromComponentsPayloadOverrides,
    WrapToErrorReturn, WrapToPayloadOverrides, build_create_key_agreement_case_deps,
    build_create_key_agreement_case_params, build_create_key_agreement_case_payload,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, build_envelope_components_params,
    build_envelope_components_payload, build_envelope_from_components_params,
    build_envelope_from_components_payload, build_generate_keys_params,
    build_generate_keys_payload, build_key_pair_components, build_key_pair_components_params,
    build_key_pair_components_payload, build_key_pair_from_components_params,
    build_key_pair_from_components_payload, build_key_pair_public_keys_params,
    build_key_pair_public_keys_payload, build_possession_components,
    build_possession_components_params, build_possession_components_payload,
    build_possession_from_components_params, build_possession_from_components_payload,
    build_public_keys_components, build_public_keys_components_params,
    build_public_keys_components_payload, build_public_keys_from_components_params,
    build_public_keys_from_components_payload, build_unwrap_params, build_unwrap_payload,
    build_wrap_to_params, build_wrap_to_payload, run_create_key_agreement_case,
};
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter,
    build_create_encoding_params,
};
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter};
use kem::{
    CredentialComponentsOverrides, ICredentialKemAdapter, build_create_kem_params,
    build_credential_components,
};
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

/// Contract: own entry `every_key_agreement_forms_the_public_keys_its_secrets_form`;
///   `generate_keys` over an `x` draw and a `y` draw that differ from each
///   other, then `key_pair_components` and `key_pair_public_keys`, returns
///   secrets that encode through `encode_scalar` to the encodings of the
///   scalars sampled from the draws and `pk1` and `pk2` that encode through
///   `encode_g1` and `encode_g2` to the encodings of the generators
///   multiplied by those scalars.
/// Arrange: draws from `build_generate_keys_payload` over the pairing,
///   overridden so they differ from each other; each case of the cases above,
///   so swapped secrets or swapped keys fail.
/// Act:     `generate_keys` once on the adapter, then `key_pair_components`
///   and `key_pair_public_keys`.
/// Assert:  each secret's encoding equals the encoding of the scalar the
///   received scalar type samples from its draw, and each public key's
///   encoding equals the encoding of `mul_g1` or `mul_g2` of its generator by
///   that scalar.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement` through the key-agreement consumer's method,
///   then the adapter's `generate_keys`, `key_pair_components`, and
///   `key_pair_public_keys`.
/// Mocked:  none.
#[test]
fn every_key_agreement_forms_the_public_keys_its_secrets_form() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let draws = build_generate_keys_payload(
                        pairing,
                        GenerateKeysPayloadOverrides {
                            x_uniform: Some(build_secret(
                                vec![0x0Du8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                            y_uniform: Some(build_secret(
                                vec![0x0Eu8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        },
                    );
                    let x_scalar = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                draws.x_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length uniform samples")
                    .scalar
                    .expose()
                    .clone();
                    let y_scalar = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                draws.y_uniform.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length uniform samples")
                    .scalar
                    .expose()
                    .clone();
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second generator reads")
                        .point;
                    let expected_pk1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    point: Some(g1),
                                    scalar: Some(x_scalar.clone()),
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let expected_pk2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    point: Some(g2),
                                    scalar: Some(y_scalar.clone()),
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;

                    // Act
                    let generated = payload
                        .adapter
                        .generate_keys(build_generate_keys_params(Default::default()), draws)
                        .expect("differing draws admit a key pair");
                    let secrets = payload
                        .adapter
                        .key_pair_components(
                            build_key_pair_components_params(Default::default()),
                            build_key_pair_components_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's secrets read")
                        .components;
                    let keys = payload
                        .adapter
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
                    let encode_scalar = |scalar: &P::Scalar| {
                        pairing
                            .encode_scalar(
                                EncodeScalarParams,
                                build_encode_scalar_payload(
                                    pairing,
                                    EncodeScalarPayloadOverrides {
                                        scalar: Some(scalar.clone()),
                                    },
                                ),
                            )
                            .expect("encoding a scalar succeeds")
                            .bytes
                            .expose()
                            .as_ref()
                            .to_vec()
                    };
                    let encode_g1 = |point: &P::G1| {
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides {
                                        point: Some(point.clone()),
                                    },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    };
                    let encode_g2 = |point: &P::G2| {
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides {
                                        point: Some(point.clone()),
                                    },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    };
                    assert_eq!(encode_scalar(secrets.x.expose()), encode_scalar(&x_scalar));
                    assert_eq!(encode_scalar(secrets.y.expose()), encode_scalar(&y_scalar));
                    assert_eq!(encode_g1(&keys.pk1), encode_g1(&expected_pk1));
                    assert_eq!(encode_g2(&keys.pk2), encode_g2(&expected_pk2));
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_forms_the_public_keys_its_secrets_form

/// Contract: own entry
///   `every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped`;
///   `generate_keys`, `public_keys_from_components` over the components
///   `key_pair_public_keys` reads and the possession `generate_keys`
///   returned, `wrap_to` of components `(A, B)` to the admitted public keys,
///   then `unwrap` under the key pair, returns components that encode to the
///   encodings of `A` and `B`.
/// Arrange: `A` and `B` from `build_credential_components` over the pairing,
///   its overrides set to `mul_g1` and `mul_g2` of the generators by scalars
///   that differ from each other and from one, so an unwrap that returns a
///   generator, swaps the components, or returns a masked element fails;
///   each case.
/// Act:     `unwrap` once, after the calls the contract names.
/// Assert:  the encoding of each returned component equals the encoding of
///   `A` or `B` respectively.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`, `key_pair_public_keys`,
///   `public_keys_from_components`, `wrap_to`, and `unwrap`.
/// Mocked:  none.
#[test]
fn every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
                        .components;
                    let public_keys = payload
                        .adapter
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
                    let a = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(build_secret(
                                                        vec![
                                                            0x05u8;
                                                            <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                        ],
                                                        SecretConstructorParamsOverrides,
                                                    )),
                                                },
                                            ),
                                        )
                                        .expect("a full-length uniform samples")
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let b = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(build_secret(
                                                        vec![
                                                            0x06u8;
                                                            <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                        ],
                                                        SecretConstructorParamsOverrides,
                                                    )),
                                                },
                                            ),
                                        )
                                        .expect("a full-length uniform samples")
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let wrapped = payload
                        .adapter
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
                    let unwrapped = payload
                        .adapter
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
                    let a_encoded = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(unwrapped.credential.a),
                                },
                            ),
                        )
                        .expect("encoding a first-group point succeeds")
                        .bytes
                        .as_ref()
                        .to_vec();
                    let b_encoded = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(unwrapped.credential.b),
                                },
                            ),
                        )
                        .expect("encoding a second-group point succeeds")
                        .bytes
                        .as_ref()
                        .to_vec();
                    assert_eq!(
                        a_encoded,
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides { point: Some(a) },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                    assert_eq!(
                        b_encoded,
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides { point: Some(b) },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped

/// Contract: own entry
///   `another_key_pair_opens_an_envelope_to_different_components`; an
///   envelope `wrap_to` returned to one key pair's admitted public keys,
///   opened by `unwrap` under a key pair `generate_keys` returned for other
///   draws, returns components that encode equal to neither `A` nor `B`.
/// Arrange: two key pairs from draws that differ in every byte; `A` and `B`
///   as the round-trip block; each case.
/// Act:     `unwrap` once under the other key pair.
/// Assert:  neither returned component's encoding equals the encoding of `A`
///   or of `B`.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys` twice,
///   `public_keys_from_components`, `wrap_to`, and `unwrap`.
/// Mocked:  none.
#[test]
fn another_key_pair_opens_an_envelope_to_different_components() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let other = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(
                                pairing,
                                GenerateKeysPayloadOverrides {
                                    x_uniform: Some(build_secret(
                                        vec![0xA1u8; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                    y_uniform: Some(build_secret(
                                        vec![0xB2u8; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                },
                            ),
                        )
                        .expect("the differing draws admit a key pair");
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
                        .components;
                    let public_keys = payload
                        .adapter
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
                    let a = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x05u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let b = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x06u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let wrapped = payload
                        .adapter
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
                    let unwrapped = payload
                        .adapter
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
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides {
                                        point: Some(unwrapped.credential.a),
                                    },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides { point: Some(a) },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                    assert_ne!(
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides {
                                        point: Some(unwrapped.credential.b),
                                    },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides { point: Some(b) },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end another_key_pair_opens_an_envelope_to_different_components

/// Contract: own entry
///   `every_wrap_returns_the_coins_its_envelope_was_formed_with`; `wrap_to`
///   of components `(A, B)` to admitted public keys and
///   `envelope_components` over the envelope return elements equal to the
///   generators and the public keys combined with the returned coins.
/// Arrange: an admitted key pair and `A` and `B` as the round-trip block;
///   each case.
/// Act:     `wrap_to` once, then `envelope_components`.
/// Assert:  `C1`, `C2`, `D1`, and `D2` encode equal to `mul_g1` of
///   `g1_generator` by `rho`, `add_g1` of `A` and `mul_g1` of `pk1` by
///   `rho`, `mul_g2` of `g2_generator` by `sigma`, and `add_g2` of `B` and
///   `mul_g2` of `pk2` by `sigma`, the coins read through `Secret::expose`.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`,
///   `public_keys_from_components`, `wrap_to`, and `envelope_components`.
/// Mocked:  none.
#[test]
fn every_wrap_returns_the_coins_its_envelope_was_formed_with() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let components = payload
                        .adapter
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
                    let public_keys = payload
                        .adapter
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
                    let a = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x05u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let b = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x06u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;

                    // Act
                    let wrapped = payload
                        .adapter
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
                    let elements = payload
                        .adapter
                        .envelope_components(
                            build_envelope_components_params(Default::default()),
                            build_envelope_components_payload(
                                &wrapped.envelope,
                                Default::default(),
                            ),
                        )
                        .expect("the envelope's elements read")
                        .components;

                    // Assert
                    let g1 = pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                        .expect("the first generator reads")
                        .point;
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("the second generator reads")
                        .point;
                    let rho = wrapped.coins.rho.expose().clone();
                    let sigma = wrapped.coins.sigma.expose().clone();
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
                        .expect("scalar multiplication succeeds")
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
                                                        point: Some(pk1),
                                                        scalar: Some(rho),
                                                    },
                                                ),
                                            )
                                            .expect("scalar multiplication succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("point addition succeeds")
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
                        .expect("scalar multiplication succeeds")
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
                                                        point: Some(pk2),
                                                        scalar: Some(sigma),
                                                    },
                                                ),
                                            )
                                            .expect("scalar multiplication succeeds")
                                            .product,
                                    ),
                                },
                            ),
                        )
                        .expect("point addition succeeds")
                        .sum;
                    let encode_g1 = |point: &P::G1| {
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides {
                                        point: Some(point.clone()),
                                    },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    };
                    let encode_g2 = |point: &P::G2| {
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides {
                                        point: Some(point.clone()),
                                    },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    };
                    assert_eq!(encode_g1(&elements.c1), encode_g1(&expected_c1));
                    assert_eq!(encode_g1(&elements.c2), encode_g1(&expected_c2));
                    assert_eq!(encode_g2(&elements.d1), encode_g2(&expected_d1));
                    assert_eq!(encode_g2(&elements.d2), encode_g2(&expected_d2));
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_wrap_returns_the_coins_its_envelope_was_formed_with

/// Contract: own entry
///   `every_key_agreement_returns_an_x_draw_sampling_refusal_whole`;
///   `generate_keys` given an `x` draw one byte shorter than the received
///   scalar type's `UNIFORM_BYTES_LENGTH` returns
///   `Err(GenerateKeysErrorReturn::XSampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected, actual }))`.
/// Arrange: the `x` draw one byte short beside a full-length `y` draw; each
///   case.
/// Act:     `generate_keys` once.
/// Assert:  the error equals `XSampling` carrying `WrongLength` with
///   `expected` the scalar type's bound and `actual` one less, whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`.
/// Mocked:  none.
#[test]
fn every_key_agreement_returns_an_x_draw_sampling_refusal_whole() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;

                    // Act
                    let result = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(build_secret(
                                    vec![0x0Du8; bound - 1],
                                    SecretConstructorParamsOverrides,
                                )),
                                y_uniform: Some(build_secret(
                                    vec![0x0Eu8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                            },
                        ),
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
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_returns_an_x_draw_sampling_refusal_whole

/// Contract: own entry
///   `every_key_agreement_returns_a_y_draw_sampling_refusal_whole`;
///   `generate_keys` given a `y` draw one byte shorter than the received
///   scalar type's `UNIFORM_BYTES_LENGTH` returns
///   `Err(GenerateKeysErrorReturn::YSampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected, actual }))`.
/// Arrange: a full-length `x` draw beside the `y` draw one byte short; each
///   case.
/// Act:     `generate_keys` once.
/// Assert:  the error equals `YSampling` carrying `WrongLength` with
///   `expected` the scalar type's bound and `actual` one less, whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`.
/// Mocked:  none.
#[test]
fn every_key_agreement_returns_a_y_draw_sampling_refusal_whole() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;

                    // Act
                    let result = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(build_secret(
                                    vec![0x0Du8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                                y_uniform: Some(build_secret(
                                    vec![0x0Eu8; bound - 1],
                                    SecretConstructorParamsOverrides,
                                )),
                            },
                        ),
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
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_returns_a_y_draw_sampling_refusal_whole

/// Contract: own entry
///   `every_key_agreement_refuses_a_zero_x_draw_as_an_identity_first_group_key`;
///   `generate_keys` over a zero `x` draw and a nonzero `y` draw returns
///   `Err(GenerateKeysErrorReturn::IdentityKeyG1)`.
/// Arrange: the zero `x` draw beside a nonzero `y` draw; each case.
/// Act:     `generate_keys` once.
/// Assert:  the error equals `GenerateKeysErrorReturn::IdentityKeyG1` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`.
/// Mocked:  none.
#[test]
fn every_key_agreement_refuses_a_zero_x_draw_as_an_identity_first_group_key() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;

                    // Act
                    let result = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(build_secret(
                                    vec![0x00u8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                                y_uniform: Some(build_secret(
                                    vec![0x0Eu8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG1));
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_refuses_a_zero_x_draw_as_an_identity_first_group_key

/// Contract: own entry
///   `every_key_agreement_refuses_a_zero_y_draw_as_an_identity_second_group_key`;
///   `generate_keys` over a nonzero `x` draw and a zero `y` draw returns
///   `Err(GenerateKeysErrorReturn::IdentityKeyG2)`.
/// Arrange: a nonzero `x` draw beside the zero `y` draw; each case.
/// Act:     `generate_keys` once.
/// Assert:  the error equals `GenerateKeysErrorReturn::IdentityKeyG2` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`.
/// Mocked:  none.
#[test]
fn every_key_agreement_refuses_a_zero_y_draw_as_an_identity_second_group_key() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;

                    // Act
                    let result = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(build_secret(
                                    vec![0x0Du8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                                y_uniform: Some(build_secret(
                                    vec![0x00u8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::IdentityKeyG2));
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_refuses_a_zero_y_draw_as_an_identity_second_group_key

/// Contract: own entry `every_key_agreement_refuses_equal_secret_draws`;
///   `generate_keys` over two draws of equal bytes returns
///   `Err(GenerateKeysErrorReturn::SharedSecret)`.
/// Arrange: equal nonzero draws; each case.
/// Act:     `generate_keys` once.
/// Assert:  the error equals `GenerateKeysErrorReturn::SharedSecret` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`.
/// Mocked:  none.
#[test]
fn every_key_agreement_refuses_equal_secret_draws() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;

                    // Act
                    let result = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(build_secret(
                                    vec![0x0Du8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                                y_uniform: Some(build_secret(
                                    vec![0x0Du8; bound],
                                    SecretConstructorParamsOverrides,
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert_eq!(result.err(), Some(GenerateKeysErrorReturn::SharedSecret));
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_refuses_equal_secret_draws

/// Contract: own entry
///   `every_key_agreement_refuses_a_key_pair_of_a_zero_x_secret`;
///   `key_pair_from_components` over components whose `x` is the zero scalar
///   and whose `y` is nonzero returns
///   `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_key_pair_components` over the pairing with `x` the scalar
///   sampled from zero bytes; each case.
/// Act:     `key_pair_from_components` once.
/// Assert:  the error equals `KeyPairFromComponentsErrorReturn::IdentityKeyG1`
///   whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `key_pair_from_components`.
/// Mocked:  none.
#[test]
fn every_key_agreement_refuses_a_key_pair_of_a_zero_x_secret() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let zero = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0x00u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length uniform samples")
                    .scalar;

                    // Act
                    let result = payload.adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        build_key_pair_from_components_payload(
                            pairing,
                            KeyPairFromComponentsPayloadOverrides {
                                components: Some(build_key_pair_components(
                                    pairing,
                                    KeyPairComponentsOverrides {
                                        x: Some(zero),
                                        ..Default::default()
                                    },
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(KeyPairFromComponentsErrorReturn::IdentityKeyG1)
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_refuses_a_key_pair_of_a_zero_x_secret

/// Contract: own entry
///   `every_key_agreement_refuses_a_key_pair_of_a_zero_y_secret`;
///   `key_pair_from_components` over components whose `y` is the zero scalar
///   and whose `x` is nonzero returns
///   `Err(KeyPairFromComponentsErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_key_pair_components` over the pairing with `y` the scalar
///   sampled from zero bytes; each case.
/// Act:     `key_pair_from_components` once.
/// Assert:  the error equals `KeyPairFromComponentsErrorReturn::IdentityKeyG2`
///   whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `key_pair_from_components`.
/// Mocked:  none.
#[test]
fn every_key_agreement_refuses_a_key_pair_of_a_zero_y_secret() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let zero = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0x00u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length uniform samples")
                    .scalar;

                    // Act
                    let result = payload.adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        build_key_pair_from_components_payload(
                            pairing,
                            KeyPairFromComponentsPayloadOverrides {
                                components: Some(build_key_pair_components(
                                    pairing,
                                    KeyPairComponentsOverrides {
                                        y: Some(zero),
                                        ..Default::default()
                                    },
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(KeyPairFromComponentsErrorReturn::IdentityKeyG2)
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_refuses_a_key_pair_of_a_zero_y_secret

/// Contract: own entry
///   `every_key_agreement_refuses_a_key_pair_of_equal_secrets`;
///   `key_pair_from_components` over components whose `x` and `y` are one
///   nonzero scalar returns
///   `Err(KeyPairFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: `build_key_pair_components` over the pairing with `y`
///   overridden by a scalar equal to the nonzero `x`; each case.
/// Act:     `key_pair_from_components` once.
/// Assert:  the error equals `KeyPairFromComponentsErrorReturn::SharedSecret`
///   whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `key_pair_from_components`.
/// Mocked:  none.
#[test]
fn every_key_agreement_refuses_a_key_pair_of_equal_secrets() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let shared = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0x07u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length uniform samples")
                    .scalar
                    .expose()
                    .clone();

                    // Act
                    let result = payload.adapter.key_pair_from_components(
                        build_key_pair_from_components_params(Default::default()),
                        build_key_pair_from_components_payload(
                            pairing,
                            KeyPairFromComponentsPayloadOverrides {
                                components: Some(build_key_pair_components(
                                    pairing,
                                    KeyPairComponentsOverrides {
                                        x: Some(build_secret(
                                            shared.clone(),
                                            SecretConstructorParamsOverrides,
                                        )),
                                        y: Some(build_secret(
                                            shared,
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(KeyPairFromComponentsErrorReturn::SharedSecret)
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end every_key_agreement_refuses_a_key_pair_of_equal_secrets

/// Contract: own entry
///   `a_key_pair_restored_from_its_components_opens_the_envelope`; the key
///   pair read by `key_pair_components` and rebuilt by
///   `key_pair_from_components` opens, under `unwrap`, an envelope wrapped
///   to the original's admitted public keys, to components that encode to
///   the encodings of `A` and `B`.
/// Arrange: `A` and `B` as the round-trip block; a generated key pair and
///   its admitted public keys; each case.
/// Act:     `unwrap` once under the restored key pair.
/// Assert:  the encoding of each returned component equals the encoding of
///   `A` or `B` respectively.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`,
///   `key_pair_components`, `key_pair_from_components`,
///   `public_keys_from_components`, `wrap_to`, and `unwrap`.
/// Mocked:  none.
#[test]
fn a_key_pair_restored_from_its_components_opens_the_envelope() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let secrets = payload
                        .adapter
                        .key_pair_components(
                            build_key_pair_components_params(Default::default()),
                            build_key_pair_components_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's secrets read")
                        .components;
                    let restored = payload
                        .adapter
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
                        .expect("the read secrets rebuild the key pair")
                        .key_pair;
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
                        .components;
                    let public_keys = payload
                        .adapter
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
                    let a = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x05u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let b = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x06u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let wrapped = payload
                        .adapter
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
                    let unwrapped = payload
                        .adapter
                        .unwrap(
                            build_unwrap_params(Default::default()),
                            build_unwrap_payload(&restored, &wrapped.envelope, Default::default()),
                        )
                        .expect("the unwrap is infallible");

                    // Assert
                    assert_eq!(
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides {
                                        point: Some(unwrapped.credential.a),
                                    },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides { point: Some(a) },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                    assert_eq!(
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides {
                                        point: Some(unwrapped.credential.b),
                                    },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides { point: Some(b) },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end a_key_pair_restored_from_its_components_opens_the_envelope

/// Contract: own entry `an_envelope_rebuilt_from_its_components_opens`; the
///   envelope read by `envelope_components` and rebuilt by
///   `envelope_from_components` opens under `unwrap` and the key pair to
///   components that encode to the encodings of `A` and `B`.
/// Arrange: `A` and `B` as the round-trip block; an envelope wrapped to an
///   admitted key pair's public keys; each case.
/// Act:     `unwrap` once over the rebuilt envelope.
/// Assert:  the encoding of each returned component equals the encoding of
///   `A` or `B` respectively.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`,
///   `public_keys_from_components`, `wrap_to`, `envelope_components`,
///   `envelope_from_components`, and `unwrap`.
/// Mocked:  none.
#[test]
fn an_envelope_rebuilt_from_its_components_opens() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
                        .components;
                    let public_keys = payload
                        .adapter
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
                    let a = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x05u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let b = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    scalar: Some(
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            build_sample_uniform_scalar_payload(
                                                SampleUniformScalarPayloadOverrides {
                                                    uniform: Some(
                                                        build_secret(
                                                            vec![
                                                                0x06u8;
                                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                            ],
                                                            SecretConstructorParamsOverrides,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        )
                                        .expect(
                                            "a full-length uniform samples",
                                        )
                                        .scalar
                                        .expose()
                                        .clone(),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let wrapped = payload
                        .adapter
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
                    let elements = payload
                        .adapter
                        .envelope_components(
                            build_envelope_components_params(Default::default()),
                            build_envelope_components_payload(
                                &wrapped.envelope,
                                Default::default(),
                            ),
                        )
                        .expect("the envelope's elements read")
                        .components;
                    let rebuilt = payload
                        .adapter
                        .envelope_from_components(
                            build_envelope_from_components_params(Default::default()),
                            build_envelope_from_components_payload(
                                pairing,
                                EnvelopeFromComponentsPayloadOverrides {
                                    components: Some(elements),
                                },
                            ),
                        )
                        .expect("the envelope rebuild is infallible")
                        .envelope;

                    // Act
                    let unwrapped = payload
                        .adapter
                        .unwrap(
                            build_unwrap_params(Default::default()),
                            build_unwrap_payload(&generated.key_pair, &rebuilt, Default::default()),
                        )
                        .expect("the unwrap is infallible");

                    // Assert
                    assert_eq!(
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides {
                                        point: Some(unwrapped.credential.a),
                                    },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides { point: Some(a) },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                    assert_eq!(
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides {
                                        point: Some(unwrapped.credential.b),
                                    },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides { point: Some(b) },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end an_envelope_rebuilt_from_its_components_opens

/// Contract: own entry
///   `public_keys_are_admitted_with_their_own_proof_of_possession`;
///   `public_keys_from_components` over the components
///   `key_pair_public_keys` reads and the possession `generate_keys`
///   returned returns `Ok`, and the admitted public keys' components equal
///   the key pair's.
/// Arrange: a generated key pair and its possession; each case.
/// Act:     `public_keys_from_components` once, then
///   `public_keys_components`.
/// Assert:  the return is `Ok` and the components' encodings equal the
///   encodings of the key pair's `pk1` and `pk2`.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`,
///   `key_pair_public_keys`, `public_keys_from_components`, and
///   `public_keys_components`.
/// Mocked:  none.
#[test]
fn public_keys_are_admitted_with_their_own_proof_of_possession() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let components = payload
                        .adapter
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

                    // Act
                    let admitted = payload
                        .adapter
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
                    let read = payload
                        .adapter
                        .public_keys_components(
                            build_public_keys_components_params(Default::default()),
                            build_public_keys_components_payload(&admitted, Default::default()),
                        )
                        .expect("the public keys' components read")
                        .components;

                    // Assert
                    assert_eq!(
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides {
                                        point: Some(read.pk1),
                                    },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g1(
                                EncodeG1Params,
                                build_encode_g1_payload(
                                    pairing,
                                    EncodeG1PayloadOverrides { point: Some(pk1) },
                                ),
                            )
                            .expect("encoding a first-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                    assert_eq!(
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides {
                                        point: Some(read.pk2),
                                    },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec(),
                        pairing
                            .encode_g2(
                                EncodeG2Params,
                                build_encode_g2_payload(
                                    pairing,
                                    EncodeG2PayloadOverrides { point: Some(pk2) },
                                ),
                            )
                            .expect("encoding a second-group point succeeds")
                            .bytes
                            .as_ref()
                            .to_vec()
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end public_keys_are_admitted_with_their_own_proof_of_possession

/// Contract: own entry
///   `a_possession_rebuilt_from_its_components_is_admitted`; the possession
///   read by `possession_components` and rebuilt by
///   `possession_from_components`, handed to `public_keys_from_components`
///   with the key pair's public keys, returns `Ok`.
/// Arrange: a generated key pair and its possession; each case.
/// Act:     `public_keys_from_components` once over the rebuilt possession.
/// Assert:  the return is `Ok`.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys`,
///   `possession_components`, `possession_from_components`, and
///   `public_keys_from_components`.
/// Mocked:  none.
#[test]
fn a_possession_rebuilt_from_its_components_is_admitted() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
                        .components;
                    let proof = payload
                        .adapter
                        .possession_components(
                            build_possession_components_params(Default::default()),
                            build_possession_components_payload(
                                &generated.possession,
                                Default::default(),
                            ),
                        )
                        .expect("the possession's components read")
                        .components;
                    let rebuilt = payload
                        .adapter
                        .possession_from_components(
                            build_possession_from_components_params(Default::default()),
                            build_possession_from_components_payload(
                                pairing,
                                PossessionFromComponentsPayloadOverrides {
                                    components: Some(proof),
                                },
                            ),
                        )
                        .expect("the possession rebuild is infallible")
                        .possession;

                    // Act
                    let result = payload.adapter.public_keys_from_components(
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
                    assert!(result.is_ok());
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end a_possession_rebuilt_from_its_components_is_admitted

/// Contract: own entry
///   `public_keys_are_refused_with_another_key_pairs_proof_of_possession`;
///   the public keys of one key pair with the possession `generate_keys`
///   returned for another return
///   `Err(PublicKeysFromComponentsErrorReturn::PossessionG1)`.
/// Arrange: two key pairs from draws that differ in every byte, the first's
///   public keys with the second's possession; each case.
/// Act:     `public_keys_from_components` once.
/// Assert:  the error equals
///   `PublicKeysFromComponentsErrorReturn::PossessionG1` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys` twice and
///   `public_keys_from_components`.
/// Mocked:  none.
#[test]
fn public_keys_are_refused_with_another_key_pairs_proof_of_possession() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let other = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(
                                pairing,
                                GenerateKeysPayloadOverrides {
                                    x_uniform: Some(build_secret(
                                        vec![0xA1u8; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                    y_uniform: Some(build_secret(
                                        vec![0xB2u8; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                },
                            ),
                        )
                        .expect("the differing draws admit a key pair");
                    let components = payload
                        .adapter
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
                    let result = payload.adapter.public_keys_from_components(
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

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end public_keys_are_refused_with_another_key_pairs_proof_of_possession

/// Contract: own entry
///   `public_keys_are_refused_with_a_second_group_proof_from_another_key_pair`;
///   the public keys of one key pair with a possession rebuilt from that key
///   pair's first-group proof and another key pair's second-group proof
///   return `Err(PublicKeysFromComponentsErrorReturn::PossessionG2)`.
/// Arrange: two key pairs from draws that differ in every byte; the
///   possession's components read by `possession_components`, the
///   second-group fields exchanged, and rebuilt by
///   `possession_from_components`; each case.
/// Act:     `public_keys_from_components` once.
/// Assert:  the error equals
///   `PublicKeysFromComponentsErrorReturn::PossessionG2` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys` twice,
///   `possession_components`, `possession_from_components`, and
///   `public_keys_from_components`.
/// Mocked:  none.
#[test]
fn public_keys_are_refused_with_a_second_group_proof_from_another_key_pair() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let other = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(
                                pairing,
                                GenerateKeysPayloadOverrides {
                                    x_uniform: Some(build_secret(
                                        vec![0xA1u8; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                    y_uniform: Some(build_secret(
                                        vec![0xB2u8; bound],
                                        SecretConstructorParamsOverrides,
                                    )),
                                },
                            ),
                        )
                        .expect("the differing draws admit a key pair");
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
                        .components;
                    let own_proof = payload
                        .adapter
                        .possession_components(
                            build_possession_components_params(Default::default()),
                            build_possession_components_payload(
                                &generated.possession,
                                Default::default(),
                            ),
                        )
                        .expect("the possession's components read")
                        .components;
                    let other_proof = payload
                        .adapter
                        .possession_components(
                            build_possession_components_params(Default::default()),
                            build_possession_components_payload(
                                &other.possession,
                                Default::default(),
                            ),
                        )
                        .expect("the other possession's components read")
                        .components;
                    let exchanged = payload
                        .adapter
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
                    let result = payload.adapter.public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &exchanged,
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

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end public_keys_are_refused_with_a_second_group_proof_from_another_key_pair

/// Contract: own entry
///   `public_keys_are_refused_with_an_identity_first_group_key`; public-key
///   components whose `pk1` is the identity element, with the possession of
///   a generated key pair, return
///   `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG1)`.
/// Arrange: `build_public_keys_components` over the pairing with `pk1`
///   overridden by `mul_g1` of `g1_generator` by the scalar sampled from zero
///   bytes; each case.
/// Act:     `public_keys_from_components` once.
/// Assert:  the error equals
///   `PublicKeysFromComponentsErrorReturn::IdentityKeyG1` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys` and
///   `public_keys_from_components`.
/// Mocked:  none.
#[test]
fn public_keys_are_refused_with_an_identity_first_group_key() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let zero = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0x00u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length uniform samples")
                    .scalar
                    .expose()
                    .clone();
                    let identity_g1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    scalar: Some(zero),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let components = build_public_keys_components(
                        pairing,
                        PublicKeysComponentsOverrides {
                            pk1: Some(identity_g1),
                            ..Default::default()
                        },
                    );

                    // Act
                    let result = payload.adapter.public_keys_from_components(
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

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end public_keys_are_refused_with_an_identity_first_group_key

/// Contract: own entry
///   `public_keys_are_refused_with_an_identity_second_group_key`;
///   public-key components whose `pk2` is the identity element and whose
///   `pk1` is the key pair's, with the possession of a generated key pair,
///   return `Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG2)`.
/// Arrange: `build_public_keys_components` over the pairing with `pk1` the
///   key pair's and `pk2` overridden by `mul_g2` of `g2_generator` by the
///   scalar sampled from zero bytes; each case.
/// Act:     `public_keys_from_components` once.
/// Assert:  the error equals
///   `PublicKeysFromComponentsErrorReturn::IdentityKeyG2` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys` and
///   `public_keys_from_components`.
/// Mocked:  none.
#[test]
fn public_keys_are_refused_with_an_identity_second_group_key() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
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
                    .expect("a full-length uniform samples")
                    .scalar
                    .expose()
                    .clone();
                    let identity_g2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    scalar: Some(zero),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let components = build_public_keys_components(
                        pairing,
                        PublicKeysComponentsOverrides {
                            pk1: Some(components.pk1),
                            pk2: Some(identity_g2),
                        },
                    );

                    // Act
                    let result = payload.adapter.public_keys_from_components(
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

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end public_keys_are_refused_with_an_identity_second_group_key

/// Contract: own entry
///   `public_keys_are_refused_with_a_secret_shared_across_the_groups`;
///   public-key components `pk1 = mul_g1(g1, s)` and `pk2 = mul_g2(g2, s)`
///   for one nonzero scalar `s`, with the possession of a generated key
///   pair, return
///   `Err(PublicKeysFromComponentsErrorReturn::SharedSecret)`.
/// Arrange: `build_public_keys_components` over the pairing with both keys
///   formed from one nonzero scalar; each case.
/// Act:     `public_keys_from_components` once.
/// Assert:  the error equals
///   `PublicKeysFromComponentsErrorReturn::SharedSecret` whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement`, then `generate_keys` and
///   `public_keys_from_components`.
/// Mocked:  none.
#[test]
fn public_keys_are_refused_with_a_secret_shared_across_the_groups() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let shared = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                            uniform: Some(build_secret(
                                vec![0x09u8; bound],
                                SecretConstructorParamsOverrides,
                            )),
                        }),
                    )
                    .expect("a full-length uniform samples")
                    .scalar
                    .expose()
                    .clone();
                    let pk1 = pairing
                        .mul_g1(
                            MulG1Params,
                            build_mul_g1_payload(
                                pairing,
                                MulG1PayloadOverrides {
                                    scalar: Some(shared.clone()),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let pk2 = pairing
                        .mul_g2(
                            MulG2Params,
                            build_mul_g2_payload(
                                pairing,
                                MulG2PayloadOverrides {
                                    scalar: Some(shared),
                                    ..Default::default()
                                },
                            ),
                        )
                        .expect("scalar multiplication succeeds")
                        .product;
                    let components = build_public_keys_components(
                        pairing,
                        PublicKeysComponentsOverrides {
                            pk1: Some(pk1),
                            pk2: Some(pk2),
                        },
                    );

                    // Act
                    let result = payload.adapter.public_keys_from_components(
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

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let mut cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS.to_vec();
    cases.push(build_create_key_agreement_params(Default::default()));

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end public_keys_are_refused_with_a_secret_shared_across_the_groups

/// Contract: own entry `a_refused_nonce_draw_returns_the_mock_error_whole`;
///   `generate_keys` over valid draws through the adapter for the mock
///   selection under `NonceDrawRefused` returns
///   `Err(GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused))`.
/// Arrange: `build_create_key_agreement_params` with `concrete` overridden
///   by `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`;
///   the mock selection alone, each over every member of
///   `PAIRING_CONCRETES`.
/// Act:     `generate_keys` once.
/// Assert:  the error equals the `Mock` variant carrying `NonceDrawRefused`,
///   whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement` for the mock selection, then `generate_keys`.
/// Mocked:  the family's mock concrete under its failure mode, which this
///   block selects through configuration; nothing else.
#[test]
fn a_refused_nonce_draw_returns_the_mock_error_whole() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;

                    // Act
                    let result = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(pairing, Default::default()),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(GenerateKeysErrorReturn::Mock(
                            MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                        ))
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let cases = vec![build_create_key_agreement_params(
        CreateKeyAgreementParamsOverrides {
            concrete: Some(KeyAgreementConcrete::Mock(
                MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
            )),
            ..Default::default()
        },
    )];

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end a_refused_nonce_draw_returns_the_mock_error_whole

/// Contract: own entry `a_refused_coin_draw_returns_the_mock_error_whole`;
///   `wrap_to` of components `(A, B)` to admitted public keys through the
///   adapter for the mock selection under `CoinDrawRefused` returns
///   `Err(WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused))`.
/// Arrange: `build_create_key_agreement_params` with `concrete` overridden
///   by `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`;
///   the mock selection alone, each over every member of
///   `PAIRING_CONCRETES`.
/// Act:     `wrap_to` once.
/// Assert:  the error equals the `Mock` variant carrying `CoinDrawRefused`,
///   whole.
/// Boundary: `run_create_key_agreement_case` to the case consumer,
///   `create_key_agreement` for the mock selection, then `generate_keys`,
///   `public_keys_from_components`, and `wrap_to`.
/// Mocked:  the family's mock concrete under its failure mode, which this
///   block selects through configuration; nothing else.
#[test]
fn a_refused_coin_draw_returns_the_mock_error_whole() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let generated = payload
                        .adapter
                        .generate_keys(
                            build_generate_keys_params(Default::default()),
                            build_generate_keys_payload(pairing, Default::default()),
                        )
                        .expect("the default draws admit a key pair");
                    let components = payload
                        .adapter
                        .key_pair_public_keys(
                            build_key_pair_public_keys_params(Default::default()),
                            build_key_pair_public_keys_payload(
                                &generated.key_pair,
                                Default::default(),
                            ),
                        )
                        .expect("the key pair's public keys read")
                        .components;
                    let public_keys = payload
                        .adapter
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

                    // Act
                    let result = payload.adapter.wrap_to(
                        build_wrap_to_params(Default::default()),
                        build_wrap_to_payload(
                            pairing,
                            &public_keys,
                            WrapToPayloadOverrides {
                                credential: Some(build_credential_components(
                                    pairing,
                                    CredentialComponentsOverrides::default(),
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result.err(),
                        Some(WrapToErrorReturn::Mock(
                            MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused,
                        ))
                    );
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let cases = vec![build_create_key_agreement_params(
        CreateKeyAgreementParamsOverrides {
            concrete: Some(KeyAgreementConcrete::Mock(
                MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
            )),
            ..Default::default()
        },
    )];

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in &cases {
                    // Act
                    let run = run_create_key_agreement_case(
                        &build_create_key_agreement_case_deps(
                            Consumer {
                                key_agreement: *case,
                            },
                            Default::default(),
                        ),
                        build_create_key_agreement_case_params(
                            CreateKeyAgreementCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding_case.concrete),
                                        identifier: Some(encoding_case.identifier),
                                    },
                                )),
                                hash_to_scalar: Some(*hash_case),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    );

                    // Assert
                    assert!(run.is_ok());
                }
            }
        }
    }
}
// end a_refused_coin_draw_returns_the_mock_error_whole
