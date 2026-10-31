#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter,
    build_create_encoding_params,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementCaseParamsOverrides,
    CreateKeyAgreementCasePayload, CreateKeyAgreementParams, DECLARED_KEY_AGREEMENT_SELECTIONS,
    GenerateKeysPayloadOverrides, ICreateKeyAgreementCaseConsumer, IKeyAgreementAdapter,
    IKeyAgreementConsumer, PublicKeysFromComponentsPayloadOverrides, WrapToPayloadOverrides,
    build_create_key_agreement_case_deps, build_create_key_agreement_case_params,
    build_create_key_agreement_case_payload, build_create_key_agreement_deps,
    build_create_key_agreement_payload, build_generate_keys_params, build_generate_keys_payload,
    build_key_pair_public_keys_params, build_key_pair_public_keys_payload,
    build_possession_components_params, build_possession_components_payload,
    build_public_keys_from_components_params, build_public_keys_from_components_payload,
    build_unwrap_params, build_unwrap_payload, build_wrap_to_params, build_wrap_to_payload,
    create_key_agreement, run_create_key_agreement_case,
};
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter};
use hashing::build_root;
use kem::{
    CredentialComponents, CredentialFromComponentsPayloadOverrides, DECLARED_KEM_SELECTIONS,
    DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IdentityScope, KemIdentity,
    SetupParamsOverrides, SetupScope, build_create_kem_params, build_credential_components_params,
    build_credential_components_payload, build_credential_from_components_params,
    build_credential_from_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_is_valid_params, build_is_valid_payload,
    build_issue_params, build_issue_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params,
    EncodeG2PayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES, build_create_pairing_params,
    build_encode_g1_payload, build_encode_g2_payload, build_encode_scalar_payload,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params,
};

/// Contract: own entry
///   `a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair`;
///   the credential the KEM issues for an identity, its components wrapped to
///   a recipient's admitted public keys, opens under the recipient's key
///   pair to components the KEM rebuilds into a valid credential, and under
///   a key pair generated for other draws to components it rebuilds into an
///   invalid one.
/// Arrange: inside the case consumer's method with `type Output = ()`, the
///   KEM's `setup`, `derive_identity`, and `issue` from `build_setup_payload`
///   and the KEM family's builders over the received `kem`, created from
///   each member of `DECLARED_KEM_SELECTIONS`, the identity each
///   asset-scope selection maps formed over a `Root` from `build_root`, the
///   same root handed to `setup` and to `derive_identity`; two key pairs
///   from `build_generate_keys_payload` with draws that differ in every
///   byte; `random_source` overridden to `RandomSourceKind::OperatingSystem`;
///   each case, the other key pair's draws differing so that an unwrap that
///   ignores the key pair fails.
/// Act:     `unwrap` under each key pair, after `credential_components`,
///   `generate_keys`, `public_keys_from_components`, and `wrap_to`; then
///   `credential_from_components` and `is_valid` over each unwrapped pair.
/// Assert:  `is_valid` equals `true` for the credential the recipient's key
///   pair opened and `false` for the credential the other key pair opened.
/// Boundary: the public `run_create_key_agreement_case` to the case
///   consumer, the public call `create_key_agreement` and the route of the
///   delivery entry in the spec's public surface.
/// Mocked:  none; every draw is the operating system's generator, so this
///   block does not prove a draw a fixture chose.
#[test]
fn a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
        scope: IdentityScope,
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
            kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            let root = build_root(Default::default());
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match self.scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset { root: &root },
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("setup admits the scope the case selects and the uniform inputs");
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        match self.scope {
                            IdentityScope::Entitlement => DeriveIdentityPayloadOverrides::default(),
                            IdentityScope::Asset => DeriveIdentityPayloadOverrides {
                                identity: Some(KemIdentity::Asset { root: &root }),
                            },
                        },
                    ),
                )
                .expect("the identity the set fixes derives");
            let issued = kem
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("issue admits the credential inputs");
            let credential = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&issued.credential, Default::default()),
                )
                .expect("the credential's components read")
                .components;

            struct KeyAgreementConsumer<'a, P: IPairingArithmetic, Km: ICredentialKemAdapter> {
                pairing: &'a P,
                kem: &'a Km,
                parameter_set: &'a Km::ParameterSet,
                identity_element: &'a Km::IdentityElement,
                credential: CredentialComponents<P::G1, P::G2>,
            }

            impl<P: IPairingArithmetic, Km: ICredentialKemAdapter<Pairing = P>>
                IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, Km>
            {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    let pairing = self.pairing;
                    let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                    let recipient = payload
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
                                &recipient.key_pair,
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
                                &recipient.possession,
                                PublicKeysFromComponentsPayloadOverrides {
                                    components: Some(components),
                                },
                            ),
                        )
                        .expect("the key pair's own proof admits its public keys")
                        .public_keys;
                    let wrapped = payload
                        .adapter
                        .wrap_to(
                            build_wrap_to_params(Default::default()),
                            build_wrap_to_payload(
                                pairing,
                                &public_keys,
                                WrapToPayloadOverrides {
                                    credential: Some(CredentialComponents {
                                        a: self.credential.a.clone(),
                                        b: self.credential.b.clone(),
                                    }),
                                },
                            ),
                        )
                        .expect("the wrap draws its coins");

                    // Act
                    let opened = payload
                        .adapter
                        .unwrap(
                            build_unwrap_params(Default::default()),
                            build_unwrap_payload(
                                &recipient.key_pair,
                                &wrapped.envelope,
                                Default::default(),
                            ),
                        )
                        .expect("the unwrap is infallible");
                    let delivered = self
                        .kem
                        .credential_from_components(
                            build_credential_from_components_params(Default::default()),
                            build_credential_from_components_payload(
                                pairing,
                                CredentialFromComponentsPayloadOverrides {
                                    components: Some(opened.credential),
                                },
                            ),
                        )
                        .expect("the credential rebuild is infallible")
                        .credential;
                    let wrong = payload
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
                    let foreign = self
                        .kem
                        .credential_from_components(
                            build_credential_from_components_params(Default::default()),
                            build_credential_from_components_payload(
                                pairing,
                                CredentialFromComponentsPayloadOverrides {
                                    components: Some(wrong.credential),
                                },
                            ),
                        )
                        .expect("the credential rebuild is infallible")
                        .credential;

                    // Assert
                    let Ok(valid) = self.kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            self.parameter_set,
                            self.identity_element,
                            &delivered,
                            Default::default(),
                        ),
                    );
                    assert!(valid.is_valid);
                    let Ok(valid) = self.kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            self.parameter_set,
                            self.identity_element,
                            &foreign,
                            Default::default(),
                        ),
                    );
                    assert!(!valid.is_valid);
                }
            }

            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer {
                        pairing,
                        kem,
                        parameter_set: &setup.parameter_set,
                        identity_element: &derived.identity_element,
                        credential: CredentialComponents {
                            a: credential.a.clone(),
                            b: credential.b.clone(),
                        },
                    },
                    Default::default(),
                ),
                self.key_agreement,
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for kem_case in DECLARED_KEM_SELECTIONS {
                    for case in DECLARED_KEY_AGREEMENT_SELECTIONS {
                        // Act
                        let run = run_create_key_agreement_case(
                            &build_create_key_agreement_case_deps(
                                Consumer {
                                    key_agreement: *case,
                                    scope: kem_case.scope,
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
                                    kem: Some(*kem_case),
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
}
// end a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair

/// Contract: own entry
///   `libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields`;
///   `generate_keys` over equal draws and equal nonce draws on any two
///   pairing concretes whose declarations carry the same
///   `target_group_encoding` returns possession components whose
///   first-group fields encode to equal bytes.
/// Arrange: a pair of members of `PAIRING_CONCRETES` whose declarations
///   carry equal `target_group_encoding`, the block calling
///   `run_create_key_agreement_case` once per member with `pairing`
///   overridden to that member's params; equal draws from
///   `build_generate_keys_payload`; `random_source` overridden to
///   `build_create_random_source_params` naming
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`,
///   one fresh source per runner call; the same encoding and hash-to-scalar
///   selection for both; the consumer's `type Output = (Vec<u8>, Vec<u8>)`,
///   its method performing `generate_keys` and `possession_components` and
///   returning the encodings of `r1` and `z1` through `encode_g1` and
///   `encode_scalar`.
/// Act:     `generate_keys` once on each library's adapter, then
///   `possession_components` on each possession, each inside its own runner
///   call.
/// Assert:  the `r1` and `z1` encodings the two runner calls return are
///   equal.
/// Boundary: the public `run_create_key_agreement_case` once per library,
///   then `create_key_agreement`, `generate_keys`, and
///   `possession_components` inside each call.
/// Mocked:  the randomness family's mock concrete under `Succeeds`; this
///   block does not prove the operating system's generator.
#[test]
fn libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

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
                type Output = (Vec<u8>, Vec<u8>);

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
                    let possession = payload
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
                    let r1 = pairing
                        .encode_g1(
                            EncodeG1Params,
                            build_encode_g1_payload(
                                pairing,
                                EncodeG1PayloadOverrides {
                                    point: Some(possession.r1),
                                },
                            ),
                        )
                        .expect("encoding a first-group point succeeds")
                        .bytes
                        .as_ref()
                        .to_vec();
                    let z1 = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(possession.z1),
                                },
                            ),
                        )
                        .expect("encoding a scalar succeeds")
                        .bytes
                        .expose()
                        .as_ref()
                        .to_vec();
                    (r1, z1)
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
            .expect("the selection admits the concrete it names")
            .output
        }
    }

    let mut outputs = Vec::new();
    for pairing_concrete in PAIRING_CONCRETES {
        let pairing_params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*pairing_concrete),
            ..Default::default()
        });
        let mut flattened = Vec::new();
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in DECLARED_KEY_AGREEMENT_SELECTIONS {
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
                                        kind: Some(RandomSourceKind::Mock(
                                            MockIRandomSourceAdapterFailureMode::Succeeds,
                                        )),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    )
                    .expect("the case runner admits the case");

                    flattened.push(run.output.0);
                    flattened.push(run.output.1);
                }
            }
        }
        outputs.push((pairing_params.target_group_encoding, flattened));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for left in &outputs {
        for right in &outputs {
            if left.0 == right.0 {
                assert_eq!(left.1, right.1);
            }
        }
    }
}
// end libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields

/// Contract: own entry
///   `libraries_sharing_an_identifier_flatten_a_second_group_statement_to_the_same_fields`;
///   `generate_keys` over equal draws and equal nonce draws on any two
///   pairing concretes whose declarations carry the same
///   `target_group_encoding` returns possession components whose
///   second-group fields encode to equal bytes.
/// Arrange: as the first-group block, the consumer's method returning the
///   encodings of `r2` and `z2` through `encode_g2` and `encode_scalar`.
/// Act:     `generate_keys` once on each library's adapter, then
///   `possession_components` on each possession, each inside its own runner
///   call.
/// Assert:  the `r2` and `z2` encodings the two runner calls return are
///   equal.
/// Boundary: the public `run_create_key_agreement_case` once per library,
///   then `create_key_agreement`, `generate_keys`, and
///   `possession_components` inside each call.
/// Mocked:  the randomness family's mock concrete under `Succeeds`; this
///   block does not prove the operating system's generator.
#[test]
fn libraries_sharing_an_identifier_flatten_a_second_group_statement_to_the_same_fields() {
    // Arrange
    struct Consumer {
        key_agreement: CreateKeyAgreementParams,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

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
                type Output = (Vec<u8>, Vec<u8>);

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
                    let possession = payload
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
                    let r2 = pairing
                        .encode_g2(
                            EncodeG2Params,
                            build_encode_g2_payload(
                                pairing,
                                EncodeG2PayloadOverrides {
                                    point: Some(possession.r2),
                                },
                            ),
                        )
                        .expect("encoding a second-group point succeeds")
                        .bytes
                        .as_ref()
                        .to_vec();
                    let z2 = pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            build_encode_scalar_payload(
                                pairing,
                                EncodeScalarPayloadOverrides {
                                    scalar: Some(possession.z2),
                                },
                            ),
                        )
                        .expect("encoding a scalar succeeds")
                        .bytes
                        .expose()
                        .as_ref()
                        .to_vec();
                    (r2, z2)
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
            .expect("the selection admits the concrete it names")
            .output
        }
    }

    let mut outputs = Vec::new();
    for pairing_concrete in PAIRING_CONCRETES {
        let pairing_params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*pairing_concrete),
            ..Default::default()
        });
        let mut flattened = Vec::new();
        for encoding_case in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for case in DECLARED_KEY_AGREEMENT_SELECTIONS {
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
                                        kind: Some(RandomSourceKind::Mock(
                                            MockIRandomSourceAdapterFailureMode::Succeeds,
                                        )),
                                    },
                                )),
                                kem: Some(build_create_kem_params(Default::default())),
                            },
                        ),
                        build_create_key_agreement_case_payload(Default::default()),
                    )
                    .expect("the case runner admits the case");

                    flattened.push(run.output.0);
                    flattened.push(run.output.1);
                }
            }
        }
        outputs.push((pairing_params.target_group_encoding, flattened));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for left in &outputs {
        for right in &outputs {
            if left.0 == right.0 {
                assert_eq!(left.1, right.1);
            }
        }
    }
}
// end libraries_sharing_an_identifier_flatten_a_second_group_statement_to_the_same_fields
