#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, ENCODING_CONCRETES, IDecoderAdapter, IEncoderAdapter,
    build_create_encoding_params, build_encoding_concrete,
};
use envelope::{
    AdmitEnvelopeSelectionParamsOverrides, ConsumeCreateKeyAgreementCaseParams,
    ConsumeCreateKeyAgreementCasePayload, ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload,
    CreateKeyAgreementCaseParamsOverrides, CreateKeyAgreementParamsOverrides,
    GenerateKeysErrorReturn, GenerateKeysPayloadOverrides, ICreateKeyAgreementCaseConsumer,
    IKeyAgreementAdapter, IKeyAgreementConsumer, KEY_AGREEMENT_CONCRETES, KeyAgreementConcrete,
    LookupEnvelopeDeclarationParamsOverrides, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    PublicKeysFromComponentsPayloadOverrides, WrapToErrorReturn, WrapToPayloadOverrides,
    admit_envelope_selection, build_admit_envelope_selection_deps,
    build_admit_envelope_selection_params, build_create_key_agreement_case_deps,
    build_create_key_agreement_case_params, build_create_key_agreement_case_payload,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, build_generate_keys_params, build_generate_keys_payload,
    build_key_agreement_concrete, build_key_pair_public_keys_params,
    build_key_pair_public_keys_payload, build_lookup_envelope_declaration_deps,
    build_lookup_envelope_declaration_params, build_lookup_envelope_declaration_payload,
    build_possession_components_params, build_possession_components_payload,
    build_public_keys_from_components_params, build_public_keys_from_components_payload,
    build_unwrap_params, build_unwrap_payload, build_wrap_to_params, build_wrap_to_payload,
    create_key_agreement, lookup_envelope_declaration, run_create_key_agreement_case,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HASH_TO_SCALAR_CONCRETES, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, build_hash_to_scalar_concrete,
};
use hashing::build_root;
use kem::{
    CreateKemParamsOverrides, CredentialComponents, CredentialComponentsOverrides,
    CredentialFromComponentsPayloadOverrides, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, KEM_CONCRETES, KemIdentity,
    LookupKemDeclarationParamsOverrides, SetupParamsOverrides, SetupScope, build_create_kem_params,
    build_credential_components, build_credential_components_params,
    build_credential_components_payload, build_credential_from_components_params,
    build_credential_from_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_is_valid_params, build_is_valid_payload,
    build_issue_params, build_issue_payload, build_kem_concrete, build_lookup_kem_declaration_deps,
    build_lookup_kem_declaration_params, build_lookup_kem_declaration_payload, build_setup_params,
    build_setup_payload, lookup_kem_declaration,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params,
    EncodeG2PayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, LookupPairingDeclarationParamsOverrides,
    PAIRING_CONCRETES, build_create_pairing_params, build_encode_g1_payload,
    build_encode_g2_payload, build_encode_scalar_payload, build_lookup_pairing_declaration_deps,
    build_lookup_pairing_declaration_params, build_lookup_pairing_declaration_payload,
    lookup_pairing_declaration,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RANDOM_SOURCE_KINDS, RandomSourceKind, build_create_random_source_params,
};
use serde_json::Value;

/// Contract: own entry
///   `a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair`;
///   the credential the KEM issues for an identity, its components wrapped to
///   a recipient's admitted public keys, opens under the recipient's key
///   pair to components the KEM rebuilds into a valid credential, and under
///   a key pair generated for other draws to components it rebuilds into an
///   invalid one.
/// Arrange: inside the case consumer's method with `type Output = ()`, the
///   KEM's `setup`, `derive_identity`, and `issue` from `build_setup_payload`
///   and the KEM family's builders over the KEM `create_kem` hands
///   `IKemConsumer`, read as `payload.kem`, created from each member of
///   `KEM_CONCRETES` whose declaration, read through `lookup_kem_declaration`
///   before the runner, provides `IdentityScope::Asset`, its selection from
///   `build_kem_concrete` with `scope` `IdentityScope::Asset`, the identity
///   each asset-scope selection maps formed over a `Root` from `build_root`,
///   the same root handed to `setup` and to `derive_identity`; two key pairs
///   from `build_generate_keys_payload` with draws that differ in every
///   byte; `random_source` overridden to each member of `RANDOM_SOURCE_KINDS`
///   in turn;
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
/// Mocked:  none; every draw is each declared member's generator, so this
///   block does not prove a draw a fixture chose.
#[test]
fn a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair() {
    // Arrange
    struct Consumer {
        key_agreement: KeyAgreementConcrete,
        scope: IdentityScope,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let pairing = payload.pairing;
            let hash_to_scalar = payload.hash_to_scalar;
            let encoder = payload.encoder;
            let random = payload.random;
            let kem = payload.kem;
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
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.key_agreement),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let cases: Vec<KeyAgreementConcrete> = KEY_AGREEMENT_CONCRETES
        .iter()
        .map(|member| {
            admit_envelope_selection(
                &build_admit_envelope_selection_deps(Default::default()),
                build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
                    concrete: Some(build_key_agreement_concrete(*member, Default::default())),
                }),
                Value::Null,
            )
            .expect("a member of the declared set is admitted")
            .selection
        })
        .collect();

    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_case in ENCODING_CONCRETES {
            for hash_case in HASH_TO_SCALAR_CONCRETES {
                for kem_concrete in KEM_CONCRETES {
                    let looked_up = lookup_kem_declaration(
                        &build_lookup_kem_declaration_deps(Default::default()),
                        build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                            concrete: Some(build_kem_concrete(*kem_concrete, Default::default())),
                        }),
                        build_lookup_kem_declaration_payload(Default::default()),
                    )
                    .expect("the real lookup returns the member's declaration");
                    if !looked_up
                        .declaration
                        .identity_scopes
                        .contains(&IdentityScope::Asset)
                    {
                        continue;
                    }
                    for case in &cases {
                        for kind in RANDOM_SOURCE_KINDS {
                            // Act
                            let run = run_create_key_agreement_case(
                                &build_create_key_agreement_case_deps(
                                    Consumer {
                                        key_agreement: *case,
                                        scope: IdentityScope::Asset,
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
                                                selection: Some(build_encoding_concrete(
                                                    *encoding_case,
                                                    Default::default(),
                                                )),
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(build_hash_to_scalar_concrete(
                                                    *hash_case,
                                                    Default::default(),
                                                )),
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides { kind: Some(*kind) },
                                        )),
                                        kem: Some(build_create_kem_params(
                                            CreateKemParamsOverrides {
                                                concrete: Some(build_kem_concrete(
                                                    *kem_concrete,
                                                    Default::default(),
                                                )),
                                                scope: Some(IdentityScope::Asset),
                                            },
                                        )),
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
}
// end a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair

/// Contract: own entry
///   `libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields`;
///   `generate_keys` over equal draws and equal nonce draws on any two
///   pairing concretes whose declarations carry the same
///   `target_group_encoding` returns possession components whose
///   first-group fields encode to equal bytes.
/// Arrange: a pair of members of `PAIRING_CONCRETES` whose declarations,
///   read through `lookup_pairing_declaration` before the runner, carry
///   equal `target_group_encoding`, the block calling
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
///   block does not prove a declared member's generator.
#[test]
fn libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields() {
    // Arrange
    struct Consumer {
        key_agreement: KeyAgreementConcrete,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let pairing = payload.pairing;
            let hash_to_scalar = payload.hash_to_scalar;
            let encoder = payload.encoder;
            let random = payload.random;
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
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.key_agreement),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names")
            .output
        }
    }

    let cases: Vec<KeyAgreementConcrete> = KEY_AGREEMENT_CONCRETES
        .iter()
        .map(|member| {
            admit_envelope_selection(
                &build_admit_envelope_selection_deps(Default::default()),
                build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
                    concrete: Some(build_key_agreement_concrete(*member, Default::default())),
                }),
                Value::Null,
            )
            .expect("a member of the declared set is admitted")
            .selection
        })
        .collect();

    let mut outputs = Vec::new();
    for pairing_concrete in PAIRING_CONCRETES {
        let looked_up = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(*pairing_concrete),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        )
        .expect("the real lookup returns the concrete's declaration");
        let mut flattened = Vec::new();
        for encoding_case in ENCODING_CONCRETES {
            for hash_case in HASH_TO_SCALAR_CONCRETES {
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
                                        selection: Some(build_encoding_concrete(
                                            *encoding_case,
                                            Default::default(),
                                        )),
                                    },
                                )),
                                hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(build_hash_to_scalar_concrete(
                                            *hash_case,
                                            Default::default(),
                                        )),
                                    },
                                )),
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
        outputs.push((looked_up.declaration.target_group_encoding, flattened));
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
///   block does not prove a declared member's generator.
#[test]
fn libraries_sharing_an_identifier_flatten_a_second_group_statement_to_the_same_fields() {
    // Arrange
    struct Consumer {
        key_agreement: KeyAgreementConcrete,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let pairing = payload.pairing;
            let hash_to_scalar = payload.hash_to_scalar;
            let encoder = payload.encoder;
            let random = payload.random;
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
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.key_agreement),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names")
            .output
        }
    }

    let cases: Vec<KeyAgreementConcrete> = KEY_AGREEMENT_CONCRETES
        .iter()
        .map(|member| {
            admit_envelope_selection(
                &build_admit_envelope_selection_deps(Default::default()),
                build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
                    concrete: Some(build_key_agreement_concrete(*member, Default::default())),
                }),
                Value::Null,
            )
            .expect("a member of the declared set is admitted")
            .selection
        })
        .collect();

    let mut outputs = Vec::new();
    for pairing_concrete in PAIRING_CONCRETES {
        let looked_up = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(*pairing_concrete),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        )
        .expect("the real lookup returns the concrete's declaration");
        let mut flattened = Vec::new();
        for encoding_case in ENCODING_CONCRETES {
            for hash_case in HASH_TO_SCALAR_CONCRETES {
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
                                        selection: Some(build_encoding_concrete(
                                            *encoding_case,
                                            Default::default(),
                                        )),
                                    },
                                )),
                                hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(build_hash_to_scalar_concrete(
                                            *hash_case,
                                            Default::default(),
                                        )),
                                    },
                                )),
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
        outputs.push((looked_up.declaration.target_group_encoding, flattened));
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

/// Contract: own entry
///   `every_admitted_key_agreement_selection_declares_the_member_it_names`;
///   the selection `admit_envelope_selection` returns for a declared member,
///   read by `lookup_envelope_declaration` as that member's declaration and
///   set in `CreateKeyAgreementParams.concrete`, constructs an adapter whose
///   `generate_keys` returns `Ok`.
/// Arrange: for each member of `KEY_AGREEMENT_CONCRETES`, the member
///   `build_key_agreement_concrete` returns for it set in
///   `AdmitEnvelopeSelectionParamsOverrides.concrete`, the document a
///   `serde_json::Value::Null` written directly; `lookup_envelope_declaration`
///   over the returned selection; the case params' `pairing` overridden to
///   each member of `PAIRING_CONCRETES` and `random_source` to each member
///   of `RANDOM_SOURCE_KINDS`, and the case consumer's
///   `CreateKeyAgreementParams.concrete` set to the returned selection; draws
///   from `build_generate_keys_payload` that differ from each other.
/// Act:     `admit_envelope_selection`, then `lookup_envelope_declaration`,
///   then `run_create_key_agreement_case` whose consumer calls
///   `generate_keys` once.
/// Assert:  the declaration's `concrete` equals the member admitted and the
///   `generate_keys` return is `Ok`.
/// Boundary: the public calls `admit_envelope_selection`,
///   `lookup_envelope_declaration`, and `run_create_key_agreement_case`,
///   then `create_key_agreement` and `generate_keys` as the entry's route
///   states.
/// Mocked:  none.
#[test]
fn every_admitted_key_agreement_selection_declares_the_member_it_names() {
    // Arrange
    struct Consumer {
        key_agreement: KeyAgreementConcrete,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let pairing = payload.pairing;
            let hash_to_scalar = payload.hash_to_scalar;
            let encoder = payload.encoder;
            let random = payload.random;
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
                bound: usize,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    // Act
                    let generated = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            self.pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(build_secret(
                                    vec![0xA1u8; self.bound],
                                    SecretConstructorParamsOverrides,
                                )),
                                y_uniform: Some(build_secret(
                                    vec![0xB2u8; self.bound],
                                    SecretConstructorParamsOverrides,
                                )),
                            },
                        ),
                    );

                    // Assert
                    assert!(generated.is_ok());
                }
            }

            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing, bound },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.key_agreement),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    for member in KEY_AGREEMENT_CONCRETES {
        // Arrange
        let admitted = admit_envelope_selection(
            &build_admit_envelope_selection_deps(Default::default()),
            build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
                concrete: Some(build_key_agreement_concrete(*member, Default::default())),
            }),
            Value::Null,
        )
        .expect("a member of the declared set is admitted");
        let looked_up = lookup_envelope_declaration(
            &build_lookup_envelope_declaration_deps(Default::default()),
            build_lookup_envelope_declaration_params(LookupEnvelopeDeclarationParamsOverrides {
                concrete: Some(admitted.selection),
            }),
            build_lookup_envelope_declaration_payload(Default::default()),
        )
        .expect("the real lookup returns the selection's declaration");

        // Assert
        assert_eq!(looked_up.declaration.concrete, admitted.selection);

        for pairing_concrete in PAIRING_CONCRETES {
            for kind in RANDOM_SOURCE_KINDS {
                // Act
                let run = run_create_key_agreement_case(
                    &build_create_key_agreement_case_deps(
                        Consumer {
                            key_agreement: admitted.selection,
                        },
                        Default::default(),
                    ),
                    build_create_key_agreement_case_params(CreateKeyAgreementCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides { kind: Some(*kind) },
                        )),
                        ..Default::default()
                    }),
                    build_create_key_agreement_case_payload(Default::default()),
                );

                // Assert
                assert!(run.is_ok());
            }
        }
    }
}
// end every_admitted_key_agreement_selection_declares_the_member_it_names

/// Contract: own entry
///   `a_refused_nonce_draw_returns_the_mock_error_whole`; `generate_keys`
///   over valid draws through the adapter for the mock selection under
///   `NonceDrawRefused` returns
///   `Err(GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused))`.
/// Arrange: the consumer's `CreateKeyAgreementParams.concrete` set to
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`;
///   draws from `build_generate_keys_payload` that differ from each other;
///   each member of `PAIRING_CONCRETES`.
/// Act:     `generate_keys` once.
/// Assert:  the error equals the `Mock` variant carrying `NonceDrawRefused`,
///   whole.
/// Boundary: the public `run_create_key_agreement_case`, then
///   `create_key_agreement` for the mock selection and `generate_keys`.
/// Mocked:  the family's mock concrete under its failure mode, which this
///   block selects through configuration; nothing else.
#[test]
fn a_refused_nonce_draw_returns_the_mock_error_whole() {
    // Arrange
    struct Consumer {
        key_agreement: KeyAgreementConcrete,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let pairing = payload.pairing;
            let hash_to_scalar = payload.hash_to_scalar;
            let encoder = payload.encoder;
            let random = payload.random;
            struct KeyAgreementConsumer<'a, P> {
                pairing: &'a P,
                bound: usize,
            }

            impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
                type Output = ();

                fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
                    &self,
                    _params: ConsumeKeyAgreementParams,
                    payload: ConsumeKeyAgreementPayload<A>,
                ) -> Self::Output {
                    // Act
                    let result = payload.adapter.generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            self.pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(build_secret(
                                    vec![0xA1u8; self.bound],
                                    SecretConstructorParamsOverrides,
                                )),
                                y_uniform: Some(build_secret(
                                    vec![0xB2u8; self.bound],
                                    SecretConstructorParamsOverrides,
                                )),
                            },
                        ),
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

            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    KeyAgreementConsumer { pairing, bound },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.key_agreement),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let cases = vec![KeyAgreementConcrete::Mock(
        MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
    )];

    for pairing_concrete in PAIRING_CONCRETES {
        for case in &cases {
            // Act
            let run = run_create_key_agreement_case(
                &build_create_key_agreement_case_deps(
                    Consumer {
                        key_agreement: *case,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_case_params(CreateKeyAgreementCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                build_create_key_agreement_case_payload(Default::default()),
            );

            // Assert
            assert!(run.is_ok());
        }
    }
}
// end a_refused_nonce_draw_returns_the_mock_error_whole

/// Contract: own entry
///   `a_refused_coin_draw_returns_the_mock_error_whole`; `wrap_to` of
///   components `(A, B)` to admitted public keys through the adapter for the
///   mock selection under `CoinDrawRefused` returns
///   `Err(WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused))`.
/// Arrange: the consumer's `CreateKeyAgreementParams.concrete` set to
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`;
///   admitted public keys from `generate_keys` and
///   `public_keys_from_components`; `A` and `B` from
///   `build_credential_components` that differ from each other; each member
///   of `PAIRING_CONCRETES`.
/// Act:     `wrap_to` once.
/// Assert:  the error equals the `Mock` variant carrying `CoinDrawRefused`,
///   whole.
/// Boundary: the public `run_create_key_agreement_case`, then
///   `create_key_agreement` for the mock selection, `generate_keys`,
///   `public_keys_from_components`, and `wrap_to`.
/// Mocked:  the family's mock concrete under its failure mode, which this
///   block selects through configuration; nothing else.
#[test]
fn a_refused_coin_draw_returns_the_mock_error_whole() {
    // Arrange
    struct Consumer {
        key_agreement: KeyAgreementConcrete,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let pairing = payload.pairing;
            let hash_to_scalar = payload.hash_to_scalar;
            let encoder = payload.encoder;
            let random = payload.random;
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
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.key_agreement),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
            .expect("the selection admits the concrete it names");
        }
    }

    let cases = vec![KeyAgreementConcrete::Mock(
        MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
    )];

    for pairing_concrete in PAIRING_CONCRETES {
        for case in &cases {
            // Act
            let run = run_create_key_agreement_case(
                &build_create_key_agreement_case_deps(
                    Consumer {
                        key_agreement: *case,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_case_params(CreateKeyAgreementCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                build_create_key_agreement_case_payload(Default::default()),
            );

            // Assert
            assert!(run.is_ok());
        }
    }
}
// end a_refused_coin_draw_returns_the_mock_error_whole
