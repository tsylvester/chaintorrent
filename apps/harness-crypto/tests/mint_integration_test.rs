#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::build_secret;
use encoding::{CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier};
use envelope::{
    DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeCoins, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, KeyPairPublicKeysParams, KeyPairPublicKeysPayload, UnwrapParams,
    UnwrapPayload,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides,
    HashToScalarIdentifier, build_domain_tag, expected_uniform_bytes,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsPayload,
    DECLARED_KEM_SELECTIONS, DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter,
    IdentityElementComponentsParams, IdentityElementComponentsPayload, IdentityScope, KemIdentity,
    MasterScalarComponents, ParameterSetComponentsParams, ParameterSetComponentsPayload,
    SetupParams, SetupPayload, SetupScope, build_credential_from_components_params,
    build_is_valid_params, build_is_valid_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    IDeliveryProofAdapter, MintPurpose, ProveMintParams, ProveMintPayload, VerifyParams,
    VerifyPayload, build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, AdmittedKemIdentityOverrides, IMintCaseConsumer, MintCaseParamsOverrides,
    MintErrorReturn, MintPayloadOverrides, build_admitted_kem_identity, build_mint_case_deps,
    build_mint_case_params, build_mint_case_payload, build_mint_deps, build_mint_params,
    build_mint_payload, mint, run_mint_case,
};

/// Contract: own entry
///   `every_mint_delivery_is_a_statement_and_proof_the_delivery_proof_verifies`;
///   `mint` over a case, every adapter real and the operating-system source
///   real, returns a delivery `verify` accepts over `AlgebraicStatement::
///   Mint(&delivery.statement)` and `&delivery.proof`.
/// Arrange: every case of the public surface — every member of
///   `PAIRING_CONCRETES` crossed with the selections
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, `DECLARED_ENCODING_SELECTIONS`,
///   `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS` with each scope,
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES`, so
///   both curves, both verifier forms, and both scopes run; the identity
///   `b"pkg@1.0.0#1"` under the entitlement scope and `vec![0x44; 32]`
///   admitted as a `Root` under the asset scope.
/// Act:     `mint`, then `verify` on the handed delivery proof, once per
///   case, in the case consumer's method.
/// Assert:  the verdict is `is_valid` `true`; each case's `run_mint_case`
///   result `is_ok()`, proving the consumers ran.
/// Boundary: `mint` invoked through the crate root and the chain of real
///   functions the public surface names — `create_random_source` for the
///   operating-system kind, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, `create_delivery_proof`, and every adapter
///   method the block calls, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_mint_delivery_is_a_statement_and_proof_the_delivery_proof_verifies() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Mint(&delivery.statement),
                        proof: &delivery.proof,
                    },
                )
                .expect("a verification verdict is returned");
            assert!(verdict.is_valid);
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(*h2s_selection),
                                        encoding: Some(CreateEncodingParams {
                                            concrete: encoding_selection.concrete,
                                            identifier: encoding_selection.identifier,
                                        }),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
                                        b"pkg@1.0.0#1",
                                        &root,
                                        Default::default(),
                                    ),
                                );
                                // Assert
                                assert!(returned.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
    // every_mint_delivery_is_a_statement_and_proof_the_delivery_proof_verifies
}

/// Contract: own entry `the_witnesses_prove_the_statement_again`; `mint`
///   over a case, then `prove_mint` called on the case's delivery proof over
///   the delivery's witnesses and statement, returns a second proof `verify`
///   accepts against the statement.
/// Arrange: every case of the public surface; the witnesses are copies
///   independent of the values the delivery's own proof consumed.
/// Act:     `mint`, then `prove_mint` over `delivery.witnesses` and
///   `&delivery.statement`, then `verify` over the second proof, in the case
///   consumer's method.
/// Assert:  the second proof's verdict is `is_valid` `true`.
/// Boundary: `mint` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn the_witnesses_prove_the_statement_again() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            let proved = delivery_proof
                .prove_mint(
                    ProveMintParams,
                    ProveMintPayload {
                        master_scalar: MasterScalarComponents {
                            value: build_secret(
                                delivery.witnesses.master_scalar.value.expose().clone(),
                                Default::default(),
                            ),
                        },
                        credential_randomness: build_secret(
                            delivery.witnesses.credential_randomness.expose().clone(),
                            Default::default(),
                        ),
                        coins: EnvelopeCoins {
                            rho: build_secret(
                                delivery.witnesses.coins.rho.expose().clone(),
                                Default::default(),
                            ),
                            sigma: build_secret(
                                delivery.witnesses.coins.sigma.expose().clone(),
                                Default::default(),
                            ),
                        },
                        statement: &delivery.statement,
                    },
                )
                .expect("the declared selection proves the mint statement");
            // Assert
            let verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Mint(&delivery.statement),
                        proof: &proved.proof,
                    },
                )
                .expect("a verification verdict is returned");
            assert!(verdict.is_valid);
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(*h2s_selection),
                                        encoding: Some(CreateEncodingParams {
                                            concrete: encoding_selection.concrete,
                                            identifier: encoding_selection.identifier,
                                        }),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
                                        b"pkg@1.0.0#1",
                                        &root,
                                        Default::default(),
                                    ),
                                );
                                // Assert
                                assert!(returned.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
    // the_witnesses_prove_the_statement_again
}

/// Contract: own entry
///   `the_recipient_opens_its_envelope_to_a_credential_valid_for_the_identity`;
///   `unwrap` of `delivery.recipient.envelope` under
///   `delivery.recipient.key_pair`, `credential_from_components` over the
///   result, and `is_valid` under the delivery's parameter set and identity
///   element returns `is_valid` `true`, and the opened credential's
///   components equal those of `delivery.credential`.
/// Arrange: every case of the public surface; the envelope is the one the
///   statement names.
/// Act:     `mint`, then `unwrap`, `credential_from_components`,
///   `is_valid`, and `credential_components`, in the case consumer's method.
/// Assert:  `is_valid` returns `true`; the opened `a` and `b` encode equal
///   to the delivered credential's components.
/// Boundary: `mint` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn the_recipient_opens_its_envelope_to_a_credential_valid_for_the_identity() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let opened = key_agreement
                .unwrap(
                    UnwrapParams,
                    UnwrapPayload {
                        key_pair: &delivery.recipient.key_pair,
                        envelope: &delivery.recipient.envelope,
                    },
                )
                .expect("an unwrap is infallible");
            let rebuilt = kem
                .credential_from_components(
                    build_credential_from_components_params(Default::default()),
                    CredentialFromComponentsPayload {
                        components: opened.credential,
                    },
                )
                .expect("a credential's reassembly is infallible");
            let verdict = kem
                .is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &delivery.parameter_set,
                        &delivery.identity_element,
                        &rebuilt.credential,
                        Default::default(),
                    ),
                )
                .expect("a validity verdict is infallible");
            assert!(verdict.is_valid);
            let wrapped = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &delivery.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let rebuilt_components = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &rebuilt.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let Ok(opened_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: rebuilt_components.components.a,
                },
            );
            let Ok(wrapped_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: wrapped.components.a,
                },
            );
            assert_eq!(opened_a.bytes.as_ref(), wrapped_a.bytes.as_ref());
            let Ok(opened_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: rebuilt_components.components.b,
                },
            );
            let Ok(wrapped_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: wrapped.components.b,
                },
            );
            assert_eq!(opened_b.bytes.as_ref(), wrapped_b.bytes.as_ref());
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(*h2s_selection),
                                        encoding: Some(CreateEncodingParams {
                                            concrete: encoding_selection.concrete,
                                            identifier: encoding_selection.identifier,
                                        }),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
                                        b"pkg@1.0.0#1",
                                        &root,
                                        Default::default(),
                                    ),
                                );
                                // Assert
                                assert!(returned.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
    // the_recipient_opens_its_envelope_to_a_credential_valid_for_the_identity
}

/// Contract: own entry `every_draw_replays_to_the_objects_it_produced`;
///   `mint` over a case, the draws split at `UNIFORM_BYTES_LENGTH` into the
///   uniforms of its calls in draw order, and `setup`, `derive_identity`,
///   and `generate_keys` run on the case's adapters over those uniforms and
///   the admitted identity, reproduce the objects the statement holds: the
///   replayed parameter set's `hpub`, the replayed identity element's
///   `element`, and the replayed key pair's `pk1` and `pk2`, each encoded
///   through the case's pairing, equal those the delivery's statement holds,
///   and the draws are six uniforms long.
/// Arrange: every case of the public surface, each case run under two
///   identities of its scope — the runner's and a second one the consumer
///   builds inside `consume_mint_case` with `build_admitted_kem_identity`;
///   draws in another order would replay to other objects.
/// Act:     `mint` twice, then `setup`, `derive_identity`, `generate_keys`,
///   and the components methods, in the case consumer's method.
/// Assert:  each delivery's `draws` is `6 * UNIFORM_BYTES_LENGTH` long; each
///   replayed `hpub`, `element`, `pk1`, and `pk2` encodes equal to the
///   statement's.
/// Boundary: `mint` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_draw_replays_to_the_objects_it_produced() {
    struct Consumer<'a> {
        other_canonical: &'a [u8],
        other_root: &'a Root,
    }

    impl IMintCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let second = build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                setup_scope: Some(identity.setup_scope),
                identity: Some(match identity.setup_scope {
                    SetupScope::Entitlement => KemIdentity::Entitlement {
                        canonical: self.other_canonical,
                    },
                    SetupScope::Asset { .. } => KemIdentity::Asset {
                        root: self.other_root,
                    },
                }),
            });
            // Act
            for admitted in [identity, second] {
                let returned = mint(
                    &build_mint_deps(
                        kem,
                        key_agreement,
                        delivery_proof,
                        random,
                        Default::default(),
                    ),
                    build_mint_params(Default::default()),
                    build_mint_payload(
                        &context,
                        MintPayloadOverrides {
                            identity: Some(admitted),
                        },
                    ),
                );
                let success = returned.expect("the subject's calls succeed");
                let delivery = success.delivery;
                // Assert
                let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                assert_eq!(delivery.draws.len(), 6 * length);
                let uniform = |index: usize| {
                    build_secret(
                        delivery.draws[index * length..(index + 1) * length].to_vec(),
                        Default::default(),
                    )
                };
                let setup = kem
                    .setup(
                        SetupParams {
                            scope: admitted.setup_scope,
                        },
                        SetupPayload {
                            master_uniform: uniform(0),
                            u0_uniform: uniform(1),
                            u1_uniform: uniform(2),
                        },
                    )
                    .expect("the replay's setup succeeds");
                let derived = kem
                    .derive_identity(
                        DeriveIdentityParams,
                        DeriveIdentityPayload {
                            parameter_set: &setup.parameter_set,
                            identity: admitted.identity,
                        },
                    )
                    .expect("the replay's identity derives");
                let keys = key_agreement
                    .generate_keys(
                        GenerateKeysParams,
                        GenerateKeysPayload {
                            x_uniform: uniform(4),
                            y_uniform: uniform(5),
                        },
                    )
                    .expect("the replay's key generation succeeds");
                let parameter_set = kem
                    .parameter_set_components(
                        ParameterSetComponentsParams,
                        ParameterSetComponentsPayload {
                            parameter_set: &setup.parameter_set,
                        },
                    )
                    .expect("the parameter set's components are infallible");
                let identity_element = kem
                    .identity_element_components(
                        IdentityElementComponentsParams,
                        IdentityElementComponentsPayload {
                            identity_element: &derived.identity_element,
                        },
                    )
                    .expect("an identity element's components are infallible");
                let public = key_agreement
                    .key_pair_public_keys(
                        KeyPairPublicKeysParams,
                        KeyPairPublicKeysPayload {
                            key_pair: &keys.key_pair,
                        },
                    )
                    .expect("a key pair's public keys are infallible");
                let encode_g1 = |point: &P::G1| -> Vec<u8> {
                    pairing
                        .encode_g1(
                            EncodeG1Params,
                            EncodeG1Payload {
                                point: point.clone(),
                            },
                        )
                        .expect("a first-group encoding is infallible")
                        .bytes
                        .as_ref()
                        .to_vec()
                };
                let encode_g2 = |point: &P::G2| -> Vec<u8> {
                    pairing
                        .encode_g2(
                            EncodeG2Params,
                            EncodeG2Payload {
                                point: point.clone(),
                            },
                        )
                        .expect("a second-group encoding is infallible")
                        .bytes
                        .as_ref()
                        .to_vec()
                };
                assert_eq!(
                    encode_g2(&parameter_set.components.hpub),
                    encode_g2(&delivery.statement.hpub)
                );
                assert_eq!(
                    encode_g1(&identity_element.components.element),
                    encode_g1(&delivery.statement.identity_element)
                );
                assert_eq!(
                    encode_g1(&public.components.pk1),
                    encode_g1(&delivery.statement.buyer_keys.pk1)
                );
                assert_eq!(
                    encode_g2(&public.components.pk2),
                    encode_g2(&delivery.statement.buyer_keys.pk2)
                );
            }
        }
    }

    // Arrange
    let other_canonical: &[u8] = b"pkg@1.0.0#2";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    let other_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x45; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(
                                        Consumer {
                                            other_canonical,
                                            other_root: &other_root,
                                        },
                                        Default::default(),
                                    ),
                                    build_mint_case_params(MintCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(*h2s_selection),
                                        encoding: Some(CreateEncodingParams {
                                            concrete: encoding_selection.concrete,
                                            identifier: encoding_selection.identifier,
                                        }),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
                                        b"pkg@1.0.0#1",
                                        &root,
                                        Default::default(),
                                    ),
                                );
                                // Assert
                                assert!(returned.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
    // every_draw_replays_to_the_objects_it_produced
}

/// Contract: own entry `a_refused_draw_reaches_the_caller_whole`; `mint`
///   over a case whose source is the randomness family's mock concrete
///   under `MockIRandomSourceAdapterFailureMode::FillBytesRefused`, selected
///   by configuration through `create_random_source`, returns
///   `Err(MintErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` — the refusal of the first draw reaches the
///   caller through the function's own variant before any adapter is called.
/// Arrange: the randomness mock selection under `FillBytesRefused`; every
///   member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS`.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  the result equals the error arm whole.
/// Boundary: `mint` invoked through the crate root and the chain of real
///   functions the public surface names, running for real up to the
///   outer-edge collaborator.
/// Mocked:  the randomness family's mock concrete under `FillBytesRefused`,
///   selected by the public configuration in this block alone.
#[test]
fn a_refused_draw_reaches_the_caller_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_mint_case(
                &build_mint_case_deps(Consumer, Default::default()),
                build_mint_case_params(MintCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::Mock(
                                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                            )),
                        },
                    )),
                    ..Default::default()
                }),
                build_mint_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_refused_draw_reaches_the_caller_whole
}

/// Contract: `kem/factory` carried entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`;
///   `mint` hands the admitted identity to `derive_identity` unchanged, so
///   the delivery's identity element is the one `derive_identity` returns
///   for it, and under the entitlement scope its `scalar` encodes to the
///   encoding of the scalar the pairing's scalar type samples from
///   `expected_uniform_bytes` over the tag built from
///   `K::DECLARATION.identity_tag` and the identity's canonical bytes.
/// Arrange: every member of `PAIRING_CONCRETES` crossed with each selection
///   of `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, each selection of
///   `DECLARED_ENCODING_SELECTIONS` declaring `EncodingIdentifier::
///   EthereumAbiV1`, each selection of `CHAIN_FORMS_CONCRETES`,
///   `DECLARED_KEM_SELECTIONS` under the entitlement scope,
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES`.
/// Act:     `mint`, then `identity_element_components` on the handed KEM, in
///   the case consumer's method.
/// Assert:  `encode_scalar` of the identity element's `scalar` equals
///   `encode_scalar` of the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes(&tag, canonical, UNIFORM_BYTES_LENGTH)`.
/// Boundary: `mint` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            assert!(matches!(identity.identity, KemIdentity::Entitlement { .. }));
            let KemIdentity::Entitlement { canonical } = identity.identity else {
                return;
            };
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let uniform = build_secret(
                expected_uniform_bytes(
                    &tag,
                    canonical,
                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                ),
                Default::default(),
            );
            let expected_scalar = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("the expected uniform bytes are of the declared length")
            .scalar;
            let identity_element = kem
                .identity_element_components(
                    IdentityElementComponentsParams,
                    IdentityElementComponentsPayload {
                        identity_element: &delivery.identity_element,
                    },
                )
                .expect("an identity element's components are infallible");
            let Ok(expected_encoding) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: expected_scalar.expose().clone(),
                },
            );
            let Ok(actual_encoding) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: identity_element.components.scalar.clone(),
                },
            );
            assert_eq!(
                actual_encoding.bytes.expose().as_ref(),
                expected_encoding.bytes.expose().as_ref()
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
        {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
            {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS
                        .iter()
                        .filter(|selection| selection.scope == IdentityScope::Entitlement)
                    {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(*h2s_selection),
                                        encoding: Some(CreateEncodingParams {
                                            concrete: encoding_selection.concrete,
                                            identifier: encoding_selection.identifier,
                                        }),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
                                        b"pkg@1.0.0#1",
                                        &root,
                                        Default::default(),
                                    ),
                                );
                                // Assert
                                assert!(returned.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
    // every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle
}
