#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, IEncoderAdapter,
    build_create_encoding_params,
};
use envelope::{
    DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeAlgebra, EnvelopeCoins, EnvelopeComponents,
    GenerateKeysPayloadOverrides, IKeyAgreementAdapter, KeyPairComponents, PublicKeysComponents,
    PublicKeysFromComponentsPayloadOverrides, WrapToPayloadOverrides,
    build_envelope_components_params, build_envelope_components_payload,
    build_generate_keys_params, build_generate_keys_payload, build_key_pair_components_params,
    build_key_pair_components_payload, build_key_pair_public_keys_params,
    build_key_pair_public_keys_payload, build_public_keys_components_params,
    build_public_keys_components_payload, build_public_keys_from_components_params,
    build_public_keys_from_components_payload, build_unwrap_params, build_unwrap_payload,
    build_wrap_to_params, build_wrap_to_payload,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter,
    build_create_hash_to_scalar_params,
};
use hashing::build_root;
use kem::{
    CredentialComponents, CredentialFromComponentsPayloadOverrides, DECLARED_KEM_SELECTIONS,
    DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides,
    KemIdentity, MasterScalarComponents, RerandomizePayloadOverrides, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_credential_components_params,
    build_credential_components_payload, build_credential_from_components_params,
    build_credential_from_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_issue_params, build_issue_payload, build_master_scalar_components_params,
    build_master_scalar_components_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_rerandomize_params, build_rerandomize_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingAdapter, IPairingArithmetic, ISampleUniformScalar,
    PAIRING_CONCRETES, build_create_pairing_params,
};
use proof::{
    AlgebraicMintStatementOverrides, AlgebraicStatement, AlgebraicTransferStatementOverrides,
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload,
    CreateDeliveryProofCaseParamsOverrides, CreateDeliveryProofParamsOverrides,
    DELIVERY_PROOF_CONCRETES, DELIVERY_PROOF_INTERFACE_VERSION, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryContextOverrides, DeliveryProofDeclaration, ICreateDeliveryProofCaseConsumer,
    IDeliveryProofAdapter, IDeliveryProofConsumer, ProveMintParams, ProveMintPayloadOverrides,
    ProveTransferParams, ProveTransferPayloadOverrides, TransferPurpose, VerifyParams,
    build_algebraic_mint_statement, build_algebraic_transfer_statement,
    build_create_delivery_proof_case_deps, build_create_delivery_proof_case_params,
    build_create_delivery_proof_case_payload, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload,
    build_delivery_context, build_prove_mint_payload, build_prove_transfer_payload,
    build_verify_payload, create_delivery_proof, run_create_delivery_proof_case,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayload, IRandomSourceAdapter,
    RandomSourceKind, build_create_random_source_params,
};

/// Contract: entry
///   `an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version`;
///   `create_delivery_proof` called through the public surface returns
///   `Ok(CreateDeliveryProofSuccessReturn { output })` with `output` the
///   `D::DECLARATION` the consumer read, whose `algebras`, `verifier_forms`,
///   and `statement_versions` contain the params' values and whose
///   `interface_version` equals `DELIVERY_PROOF_INTERFACE_VERSION`.
/// Arrange: every member of `PAIRING_CONCRETES`, the verifier form read from
///   the `PairingDeclaration` the received pairing declares, nested over every
///   member of `DECLARED_ENCODING_SELECTIONS`, every member of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every member of `DECLARED_KEM_SELECTIONS`, and
///   every member of `DECLARED_KEY_AGREEMENT_SELECTIONS`, each reached through
///   its factory, and the randomness source `create_random_source` returns for
///   `RandomSourceKind::OperatingSystem`; `create_delivery_proof` once per
///   member of `DELIVERY_PROOF_CONCRETES` with params carrying the selection,
///   `EnvelopeAlgebra::PairingElGamal`, the form the pairing declares, and
///   `DELIVERY_STATEMENT_VERSION_ONE`; `DeclarationConsumer` returns
///   `D::DECLARATION`.
/// Act:     `create_delivery_proof` through the public surface.
/// Assert:  the output's `algebras` contains the params' algebra, its
///   `verifier_forms` the params' form, its `statement_versions` the params'
///   version, and its `interface_version` equals
///   `DELIVERY_PROOF_INTERFACE_VERSION`; every factory result `is_ok()` in
///   place.
/// Boundary: the public call `create_delivery_proof`, along `create_pairing`,
///   `create_delivery_proof`, the concrete's `try_new`, and
///   `consume_delivery_proof`.
/// Mocked:  nothing; the block proves the declaration of each real concrete
///   and the consumer's output reaching the return.
#[test]
fn an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version() {
    struct DeclarationConsumer;

    impl<P: IPairingAdapter, F: IChainForms> IDeliveryProofConsumer<P, F> for DeclarationConsumer {
        type Output = DeliveryProofDeclaration;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            _payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            D::DECLARATION
        }
    }

    struct CaseConsumer;

    impl ICreateDeliveryProofCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            _kem: &K,
            _key_agreement: &A,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for concrete in DELIVERY_PROOF_CONCRETES {
                // Act
                let created = create_delivery_proof::<P, E, F, DeclarationConsumer>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        DeclarationConsumer,
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
                let declaration = created
                    .expect("the admitted selection returns the consumer's output")
                    .output;
                assert!(
                    declaration
                        .algebras
                        .contains(&EnvelopeAlgebra::PairingElGamal)
                );
                assert!(declaration.verifier_forms.contains(&verifier_form));
                assert!(
                    declaration
                        .statement_versions
                        .contains(&DELIVERY_STATEMENT_VERSION_ONE)
                );
                assert_eq!(
                    declaration.interface_version,
                    DELIVERY_PROOF_INTERFACE_VERSION
                );
            }
        }
    }

    for pairing_case in PAIRING_CONCRETES {
        for selection in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_case in DECLARED_KEM_SELECTIONS {
                        for key_agreement_case in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            // Act
                            let result = run_create_delivery_proof_case(
                                &build_create_delivery_proof_case_deps(
                                    CaseConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_case_params(
                                    CreateDeliveryProofCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_case),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(hash_case.concrete),
                                                identifier: Some(hash_case.identifier),
                                            },
                                        )),
                                        encoding: Some(build_create_encoding_params(
                                            CreateEncodingParamsOverrides {
                                                concrete: Some(selection.concrete),
                                                identifier: Some(selection.identifier),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_case),
                                        key_agreement: Some(*key_agreement_case),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    },
                                ),
                                build_create_delivery_proof_case_payload(Default::default()),
                            );

                            // Assert
                            assert!(result.is_ok());
                        }
                    }
                }
            }
        }
    }
    // an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version
}

/// Contract: entry
///   `a_credential_issued_and_wrapped_through_the_families_is_proved_by_a_mint_proof_that_verifies`;
///   a credential `issue` returns through the KEM family's concrete, wrapped by
///   `wrap_to` through the key-agreement family's concrete to admitted keys,
///   then `prove_mint` over the statement built from the values the families
///   produced and the witnesses they hold, then `verify`, on the adapter
///   `create_delivery_proof` hands the consumer in the verifier form the
///   resolved pairing declares returns `Ok(VerifySuccessReturn { is_valid:
///   true })`.
/// Arrange: the consumer chain of the declaration block; the credential
///   `issue` returns through the KEM family's concrete over a setup whose
///   uniform inputs are drawn by `fill_bytes`, a key pair from `generate_keys`
///   whose public keys the key-agreement family's concrete admits, the
///   credential wrapped to those keys by `wrap_to` giving the envelope and its
///   coins, every draw from the operating system's generator; the statement
///   from `build_algebraic_mint_statement` overridden with the `hpub` of
///   `parameter_set_components`, the `element` of `identity_element_components`,
///   the admitted keys of `public_keys_components` as the buyer's keys, and
///   the envelope of `envelope_components`; the witnesses from
///   `build_prove_mint_payload` overridden with `master_scalar_components`,
///   the issuance randomness, and the wrap coins.
/// Act:     `prove_mint` on the adapter the consumer receives, then `verify`
///   over the same statement and the proof, inside `MintConsumer`.
/// Assert:  `verify`'s result equals `Ok` with `is_valid` true; every factory
///   result `is_ok()` in place.
/// Boundary: the public call `create_delivery_proof`, along `create_pairing`,
///   `create_encoding`, `create_hash_to_scalar`, `create_random_source`,
///   `create_chain_forms`, `create_kem`, `create_key_agreement`,
///   `create_delivery_proof`, the concrete's `try_new`, `prove_mint`, and
///   `verify`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn a_credential_issued_and_wrapped_through_the_families_is_proved_by_a_mint_proof_that_verifies() {
    struct MintConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hpub: P::G2,
        identity_element: P::G1,
        buyer_keys: PublicKeysComponents<P::G1, P::G2>,
        envelope: EnvelopeComponents<P::G1, P::G2>,
        master_scalar: MasterScalarComponents<P::Scalar>,
        randomness: Secret<P::Scalar>,
        coins: EnvelopeCoins<P::Scalar>,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F> for MintConsumer<'_, P> {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;

            // Arrange
            let statement = build_algebraic_mint_statement::<P, F>(
                self.pairing,
                AlgebraicMintStatementOverrides {
                    hpub: Some(self.hpub.clone()),
                    identity_element: Some(self.identity_element.clone()),
                    buyer_keys: Some(PublicKeysComponents {
                        pk1: self.buyer_keys.pk1.clone(),
                        pk2: self.buyer_keys.pk2.clone(),
                    }),
                    envelope: Some(EnvelopeComponents {
                        c1: self.envelope.c1.clone(),
                        c2: self.envelope.c2.clone(),
                        d1: self.envelope.d1.clone(),
                        d2: self.envelope.d2.clone(),
                    }),
                    ..Default::default()
                },
            );
            let proof = adapter
                .prove_mint(
                    ProveMintParams,
                    build_prove_mint_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveMintPayloadOverrides {
                            master_scalar: Some(MasterScalarComponents {
                                value: build_secret(
                                    self.master_scalar.value.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            credential_randomness: Some(build_secret(
                                self.randomness.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                            coins: Some(EnvelopeCoins {
                                rho: build_secret(
                                    self.coins.rho.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                sigma: build_secret(
                                    self.coins.sigma.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                        },
                    ),
                )
                .expect("the families' witnesses prove")
                .proof;

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    struct CaseConsumer;

    impl ICreateDeliveryProofCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
        ) -> Self::Output {
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw = |length: usize| {
                random
                    .fill_bytes(FillBytesParams, FillBytesPayload { length })
                    .expect("the operating system's generator draws")
                    .bytes
            };
            for &scope in K::DECLARATION.identity_scopes {
                // Arrange
                let root = build_root(Default::default());
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(match scope {
                                IdentityScope::Entitlement => SetupScope::Entitlement,
                                IdentityScope::Asset => SetupScope::Asset { root: &root },
                            }),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(draw(bound)),
                            u0_uniform: Some(draw(bound)),
                            u1_uniform: Some(draw(bound)),
                        }),
                    )
                    .expect("setup admits the scope the case selects and the drawn uniforms");
                let derived = kem
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            match scope {
                                IdentityScope::Entitlement => {
                                    DeriveIdentityPayloadOverrides::default()
                                }
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
                            IssuePayloadOverrides {
                                uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("issue admits the credential inputs");
                let parameter_set = kem
                    .parameter_set_components(
                        build_parameter_set_components_params(Default::default()),
                        build_parameter_set_components_payload(
                            &setup.parameter_set,
                            Default::default(),
                        ),
                    )
                    .expect("the parameter set's components read")
                    .components;
                let identity_element = kem
                    .identity_element_components(
                        build_identity_element_components_params(Default::default()),
                        build_identity_element_components_payload(
                            &derived.identity_element,
                            Default::default(),
                        ),
                    )
                    .expect("the identity element's components read")
                    .components;
                let master_scalar = kem
                    .master_scalar_components(
                        build_master_scalar_components_params(Default::default()),
                        build_master_scalar_components_payload(
                            &setup.master_scalar,
                            Default::default(),
                        ),
                    )
                    .expect("the master scalar's components read")
                    .components;
                let credential = kem
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(&issued.credential, Default::default()),
                    )
                    .expect("the credential's components read")
                    .components;
                let buyer = key_agreement
                    .generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(draw(bound)),
                                y_uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("the drawn keys generate");
                let components = key_agreement
                    .key_pair_public_keys(
                        build_key_pair_public_keys_params(Default::default()),
                        build_key_pair_public_keys_payload(&buyer.key_pair, Default::default()),
                    )
                    .expect("the key pair's public keys read")
                    .components;
                let buyer_public_keys = key_agreement
                    .public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &buyer.possession,
                            PublicKeysFromComponentsPayloadOverrides {
                                components: Some(components),
                            },
                        ),
                    )
                    .expect("the key pair's own proof admits its public keys")
                    .public_keys;
                let buyer_keys = key_agreement
                    .public_keys_components(
                        build_public_keys_components_params(Default::default()),
                        build_public_keys_components_payload(
                            &buyer_public_keys,
                            Default::default(),
                        ),
                    )
                    .expect("the admitted keys' components read")
                    .components;
                let wrapped = key_agreement
                    .wrap_to(
                        build_wrap_to_params(Default::default()),
                        build_wrap_to_payload(
                            pairing,
                            &buyer_public_keys,
                            WrapToPayloadOverrides {
                                credential: Some(CredentialComponents {
                                    a: credential.a.clone(),
                                    b: credential.b.clone(),
                                }),
                            },
                        ),
                    )
                    .expect("the credential wraps to the admitted keys");
                let envelope = key_agreement
                    .envelope_components(
                        build_envelope_components_params(Default::default()),
                        build_envelope_components_payload(&wrapped.envelope, Default::default()),
                    )
                    .expect("the envelope's components read")
                    .components;

                for concrete in DELIVERY_PROOF_CONCRETES {
                    // Act
                    let created = create_delivery_proof::<P, E, F, MintConsumer<'_, P>>(
                        &build_create_delivery_proof_deps(
                            pairing,
                            hash_to_scalar,
                            encoder,
                            random,
                            MintConsumer {
                                pairing,
                                hpub: parameter_set.hpub.clone(),
                                identity_element: identity_element.element.clone(),
                                buyer_keys: PublicKeysComponents {
                                    pk1: buyer_keys.pk1.clone(),
                                    pk2: buyer_keys.pk2.clone(),
                                },
                                envelope: EnvelopeComponents {
                                    c1: envelope.c1.clone(),
                                    c2: envelope.c2.clone(),
                                    d1: envelope.d1.clone(),
                                    d2: envelope.d2.clone(),
                                },
                                master_scalar: MasterScalarComponents {
                                    value: build_secret(
                                        master_scalar.value.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                },
                                randomness: build_secret(
                                    issued.randomness.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                coins: EnvelopeCoins {
                                    rho: build_secret(
                                        wrapped.coins.rho.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                    sigma: build_secret(
                                        wrapped.coins.sigma.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                },
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );

                    // Assert
                    assert!(created.is_ok());
                }
            }
        }
    }

    for pairing_case in PAIRING_CONCRETES {
        for selection in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_case in DECLARED_KEM_SELECTIONS {
                        for key_agreement_case in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            // Act
                            let result = run_create_delivery_proof_case(
                                &build_create_delivery_proof_case_deps(
                                    CaseConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_case_params(
                                    CreateDeliveryProofCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_case),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(hash_case.concrete),
                                                identifier: Some(hash_case.identifier),
                                            },
                                        )),
                                        encoding: Some(build_create_encoding_params(
                                            CreateEncodingParamsOverrides {
                                                concrete: Some(selection.concrete),
                                                identifier: Some(selection.identifier),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_case),
                                        key_agreement: Some(*key_agreement_case),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    },
                                ),
                                build_create_delivery_proof_case_payload(Default::default()),
                            );

                            // Assert
                            assert!(result.is_ok());
                        }
                    }
                }
            }
        }
    }
    // a_credential_issued_and_wrapped_through_the_families_is_proved_by_a_mint_proof_that_verifies
}

/// Contract: entry
///   `a_credential_rerandomized_and_wrapped_to_the_buyer_is_proved_by_a_transfer_proof_that_verifies_and_opens`;
///   the seller's fresh `unwrap` of the envelope a credential was wrapped to
///   the seller in, `rerandomize` by the KEM with a drawn `uniform`, the
///   result wrapped to the buyer's admitted keys, then `prove_transfer` over
///   the statement built from the values the families produced and the
///   seller's secrets and the offset, then `verify`, and the buyer's `unwrap`
///   of the new envelope returns `Ok(VerifySuccessReturn { is_valid: true })`
///   and a credential the buyer's key pair opens that the KEM's `is_valid`
///   accepts.
/// Arrange: the consumer chain of the mint block; the credential `issue`
///   returns through the KEM family's concrete, a seller key pair and a buyer
///   key pair from `generate_keys` over drawn uniforms, each admitted through
///   the key-agreement family's concrete, the credential wrapped to the
///   seller's keys by `wrap_to` giving the old envelope, the seller's fresh
///   `unwrap` of it, `rerandomize` over a drawn `uniform` returning the
///   credential and the private offset, the result wrapped to the buyer's
///   keys by `wrap_to` giving the new envelope and its coins, every draw from
///   the operating system's generator; the statement from
///   `build_algebraic_transfer_statement` overridden with the identity element
///   element, the seller's and buyer's keys, the old envelope, and the new
///   envelope the families produced; the witnesses from
///   `build_prove_transfer_payload` overridden with the seller's key-pair
///   components, the offset, and the new wrap coins.
/// Act:     `prove_transfer` on the adapter the consumer receives, then
///   `verify` over the same statement and the proof, then the buyer's `unwrap`
///   of the new envelope, inside `TransferConsumer`.
/// Assert:  `verify`'s result equals `Ok` with `is_valid` true, the buyer's
///   `unwrap` returns `Ok`, and the KEM's `is_valid` over the credential it
///   returns equals `Ok` with `is_valid` true; every factory result `is_ok()`
///   in place.
/// Boundary: the route of the mint block with `unwrap`, `rerandomize`,
///   `prove_transfer`, and `verify`.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn a_credential_rerandomized_and_wrapped_to_the_buyer_is_proved_by_a_transfer_proof_that_verifies_and_opens()
 {
    struct TransferConsumer<
        'a,
        P: IPairingArithmetic,
        Km: ICredentialKemAdapter<Pairing = P>,
        K: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a Km,
        parameter_set: &'a Km::ParameterSet,
        identity_element: &'a Km::IdentityElement,
        identity_element_element: P::G1,
        key_agreement: &'a K,
        buyer_key_pair: &'a K::KeyPair,
        new_envelope: &'a K::Envelope,
        seller_secrets: KeyPairComponents<P::Scalar>,
        offset: Secret<P::Scalar>,
        coins: EnvelopeCoins<P::Scalar>,
        seller_keys: PublicKeysComponents<P::G1, P::G2>,
        buyer_keys: PublicKeysComponents<P::G1, P::G2>,
        old_envelope: EnvelopeComponents<P::G1, P::G2>,
        new_envelope_components: EnvelopeComponents<P::G1, P::G2>,
    }

    impl<
        P: IPairingArithmetic,
        Km: ICredentialKemAdapter<Pairing = P>,
        K: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    > IDeliveryProofConsumer<P, F> for TransferConsumer<'_, P, Km, K>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;

            // Arrange
            let statement = build_algebraic_transfer_statement::<P, F>(
                self.pairing,
                AlgebraicTransferStatementOverrides {
                    identity_element: Some(self.identity_element_element.clone()),
                    seller_keys: Some(PublicKeysComponents {
                        pk1: self.seller_keys.pk1.clone(),
                        pk2: self.seller_keys.pk2.clone(),
                    }),
                    buyer_keys: Some(PublicKeysComponents {
                        pk1: self.buyer_keys.pk1.clone(),
                        pk2: self.buyer_keys.pk2.clone(),
                    }),
                    old_envelope: Some(EnvelopeComponents {
                        c1: self.old_envelope.c1.clone(),
                        c2: self.old_envelope.c2.clone(),
                        d1: self.old_envelope.d1.clone(),
                        d2: self.old_envelope.d2.clone(),
                    }),
                    new_envelope: Some(EnvelopeComponents {
                        c1: self.new_envelope_components.c1.clone(),
                        c2: self.new_envelope_components.c2.clone(),
                        d1: self.new_envelope_components.d1.clone(),
                        d2: self.new_envelope_components.d2.clone(),
                    }),
                    ..Default::default()
                },
            );
            let proof = adapter
                .prove_transfer(
                    ProveTransferParams,
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveTransferPayloadOverrides {
                            seller_secrets: Some(KeyPairComponents {
                                x: build_secret(
                                    self.seller_secrets.x.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                y: build_secret(
                                    self.seller_secrets.y.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            offset: Some(build_secret(
                                self.offset.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                            coins: Some(EnvelopeCoins {
                                rho: build_secret(
                                    self.coins.rho.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                sigma: build_secret(
                                    self.coins.sigma.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                        },
                    ),
                )
                .expect("the families' witnesses prove")
                .proof;

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));

            // Act
            let opened = self
                .key_agreement
                .unwrap(
                    build_unwrap_params(Default::default()),
                    build_unwrap_payload(
                        self.buyer_key_pair,
                        self.new_envelope,
                        Default::default(),
                    ),
                )
                .expect("the buyer's unwrap is infallible");
            let delivered = self
                .kem
                .credential_from_components(
                    build_credential_from_components_params(Default::default()),
                    build_credential_from_components_payload(
                        self.pairing,
                        CredentialFromComponentsPayloadOverrides {
                            components: Some(opened.credential),
                        },
                    ),
                )
                .expect("the credential rebuild is infallible")
                .credential;
            let valid = self
                .kem
                .is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        self.parameter_set,
                        self.identity_element,
                        &delivered,
                        Default::default(),
                    ),
                )
                .expect("the validity read is infallible");

            // Assert
            assert!(valid.is_valid);
        }
    }

    struct CaseConsumer;

    impl ICreateDeliveryProofCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
        ) -> Self::Output {
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw = |length: usize| {
                random
                    .fill_bytes(FillBytesParams, FillBytesPayload { length })
                    .expect("the operating system's generator draws")
                    .bytes
            };
            for &scope in K::DECLARATION.identity_scopes {
                // Arrange
                let root = build_root(Default::default());
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(match scope {
                                IdentityScope::Entitlement => SetupScope::Entitlement,
                                IdentityScope::Asset => SetupScope::Asset { root: &root },
                            }),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(draw(bound)),
                            u0_uniform: Some(draw(bound)),
                            u1_uniform: Some(draw(bound)),
                        }),
                    )
                    .expect("setup admits the scope the case selects and the drawn uniforms");
                let derived = kem
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            match scope {
                                IdentityScope::Entitlement => {
                                    DeriveIdentityPayloadOverrides::default()
                                }
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
                            IssuePayloadOverrides {
                                uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("issue admits the credential inputs");
                let identity_element = kem
                    .identity_element_components(
                        build_identity_element_components_params(Default::default()),
                        build_identity_element_components_payload(
                            &derived.identity_element,
                            Default::default(),
                        ),
                    )
                    .expect("the identity element's components read")
                    .components;
                let credential = kem
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(&issued.credential, Default::default()),
                    )
                    .expect("the credential's components read")
                    .components;
                let seller = key_agreement
                    .generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(draw(bound)),
                                y_uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("the seller's drawn keys generate");
                let buyer = key_agreement
                    .generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(draw(bound)),
                                y_uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("the buyer's drawn keys generate");
                let seller_components = key_agreement
                    .key_pair_public_keys(
                        build_key_pair_public_keys_params(Default::default()),
                        build_key_pair_public_keys_payload(&seller.key_pair, Default::default()),
                    )
                    .expect("the seller's public keys read")
                    .components;
                let seller_public_keys = key_agreement
                    .public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &seller.possession,
                            PublicKeysFromComponentsPayloadOverrides {
                                components: Some(seller_components),
                            },
                        ),
                    )
                    .expect("the seller's own proof admits its public keys")
                    .public_keys;
                let buyer_components = key_agreement
                    .key_pair_public_keys(
                        build_key_pair_public_keys_params(Default::default()),
                        build_key_pair_public_keys_payload(&buyer.key_pair, Default::default()),
                    )
                    .expect("the buyer's public keys read")
                    .components;
                let buyer_public_keys = key_agreement
                    .public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &buyer.possession,
                            PublicKeysFromComponentsPayloadOverrides {
                                components: Some(buyer_components),
                            },
                        ),
                    )
                    .expect("the buyer's own proof admits its public keys")
                    .public_keys;
                let seller_keys = key_agreement
                    .public_keys_components(
                        build_public_keys_components_params(Default::default()),
                        build_public_keys_components_payload(
                            &seller_public_keys,
                            Default::default(),
                        ),
                    )
                    .expect("the seller's components read")
                    .components;
                let buyer_keys = key_agreement
                    .public_keys_components(
                        build_public_keys_components_params(Default::default()),
                        build_public_keys_components_payload(
                            &buyer_public_keys,
                            Default::default(),
                        ),
                    )
                    .expect("the buyer's components read")
                    .components;
                let seller_secrets = key_agreement
                    .key_pair_components(
                        build_key_pair_components_params(Default::default()),
                        build_key_pair_components_payload(&seller.key_pair, Default::default()),
                    )
                    .expect("the seller's key-pair components read")
                    .components;
                let wrapped_old = key_agreement
                    .wrap_to(
                        build_wrap_to_params(Default::default()),
                        build_wrap_to_payload(
                            pairing,
                            &seller_public_keys,
                            WrapToPayloadOverrides {
                                credential: Some(CredentialComponents {
                                    a: credential.a.clone(),
                                    b: credential.b.clone(),
                                }),
                            },
                        ),
                    )
                    .expect("the credential wraps to the seller's keys");
                let opened = key_agreement
                    .unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(
                            &seller.key_pair,
                            &wrapped_old.envelope,
                            Default::default(),
                        ),
                    )
                    .expect("the seller's unwrap is infallible");
                let delivered = kem
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
                let rerandomized = kem
                    .rerandomize(
                        build_rerandomize_params(Default::default()),
                        build_rerandomize_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &delivered,
                            RerandomizePayloadOverrides {
                                uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("the drawn uniform rerandomizes the credential");
                let rerandomized_components = kem
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(
                            &rerandomized.credential,
                            Default::default(),
                        ),
                    )
                    .expect("the credential's components read")
                    .components;
                let wrapped_new = key_agreement
                    .wrap_to(
                        build_wrap_to_params(Default::default()),
                        build_wrap_to_payload(
                            pairing,
                            &buyer_public_keys,
                            WrapToPayloadOverrides {
                                credential: Some(rerandomized_components),
                            },
                        ),
                    )
                    .expect("the credential wraps to the buyer's keys");
                let old_envelope = key_agreement
                    .envelope_components(
                        build_envelope_components_params(Default::default()),
                        build_envelope_components_payload(
                            &wrapped_old.envelope,
                            Default::default(),
                        ),
                    )
                    .expect("the old envelope's components read")
                    .components;
                let new_envelope_components = key_agreement
                    .envelope_components(
                        build_envelope_components_params(Default::default()),
                        build_envelope_components_payload(
                            &wrapped_new.envelope,
                            Default::default(),
                        ),
                    )
                    .expect("the new envelope's components read")
                    .components;

                for concrete in DELIVERY_PROOF_CONCRETES {
                    // Act
                    let created = create_delivery_proof::<P, E, F, TransferConsumer<'_, P, K, A>>(
                        &build_create_delivery_proof_deps(
                            pairing,
                            hash_to_scalar,
                            encoder,
                            random,
                            TransferConsumer {
                                pairing,
                                kem,
                                parameter_set: &setup.parameter_set,
                                identity_element: &derived.identity_element,
                                identity_element_element: identity_element.element.clone(),
                                key_agreement,
                                buyer_key_pair: &buyer.key_pair,
                                new_envelope: &wrapped_new.envelope,
                                seller_secrets: KeyPairComponents {
                                    x: build_secret(
                                        seller_secrets.x.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                    y: build_secret(
                                        seller_secrets.y.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                },
                                offset: build_secret(
                                    rerandomized.offset.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                coins: EnvelopeCoins {
                                    rho: build_secret(
                                        wrapped_new.coins.rho.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                    sigma: build_secret(
                                        wrapped_new.coins.sigma.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                },
                                seller_keys: PublicKeysComponents {
                                    pk1: seller_keys.pk1.clone(),
                                    pk2: seller_keys.pk2.clone(),
                                },
                                buyer_keys: PublicKeysComponents {
                                    pk1: buyer_keys.pk1.clone(),
                                    pk2: buyer_keys.pk2.clone(),
                                },
                                old_envelope: EnvelopeComponents {
                                    c1: old_envelope.c1.clone(),
                                    c2: old_envelope.c2.clone(),
                                    d1: old_envelope.d1.clone(),
                                    d2: old_envelope.d2.clone(),
                                },
                                new_envelope_components: EnvelopeComponents {
                                    c1: new_envelope_components.c1.clone(),
                                    c2: new_envelope_components.c2.clone(),
                                    d1: new_envelope_components.d1.clone(),
                                    d2: new_envelope_components.d2.clone(),
                                },
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );

                    // Assert
                    assert!(created.is_ok());
                }
            }
        }
    }

    for pairing_case in PAIRING_CONCRETES {
        for selection in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_case in DECLARED_KEM_SELECTIONS {
                        for key_agreement_case in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            // Act
                            let result = run_create_delivery_proof_case(
                                &build_create_delivery_proof_case_deps(
                                    CaseConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_case_params(
                                    CreateDeliveryProofCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_case),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(hash_case.concrete),
                                                identifier: Some(hash_case.identifier),
                                            },
                                        )),
                                        encoding: Some(build_create_encoding_params(
                                            CreateEncodingParamsOverrides {
                                                concrete: Some(selection.concrete),
                                                identifier: Some(selection.identifier),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_case),
                                        key_agreement: Some(*key_agreement_case),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    },
                                ),
                                build_create_delivery_proof_case_payload(Default::default()),
                            );

                            // Assert
                            assert!(result.is_ok());
                        }
                    }
                }
            }
        }
    }
    // a_credential_rerandomized_and_wrapped_to_the_buyer_is_proved_by_a_transfer_proof_that_verifies_and_opens
}

/// Contract: entry `a_transfer_proof_fails_against_a_statement_with_another_expiry`;
///   `verify` over a transfer proof and a statement identical to the one it was
///   formed for but for `expiry` returns
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the consumer chain of the transfer block; a seller key pair and a
///   buyer key pair from `generate_keys`, each admitted through the
///   key-agreement family's concrete; the credential `issue` returns, wrapped
///   to the seller by `wrap_to`, unwrapped by the seller, rerandomized by
///   `rerandomize` over a drawn `uniform`, and wrapped to the buyer by
///   `wrap_to`, every draw from the operating system's generator; the transfer
///   statement from `build_algebraic_transfer_statement` overridden with the
///   keys and envelopes the families produced and the proof from
///   `prove_transfer` over `build_prove_transfer_payload` overridden with the
///   seller's key-pair components, the offset, and the new wrap coins; a
///   second statement from the same builder with the same overrides and a
///   `context` whose `expiry` is another value from the context builder's own
///   override.
/// Act:     `verify` over the second statement and the proof, inside
///   `ExpiryConsumer`.
/// Assert:  `verify` over the second statement equals `Ok` with `is_valid`
///   false, while `verify` over the first statement and the proof returns
///   `is_valid` true, so the refusal comes from the expiry alone; every
///   factory result `is_ok()` in place.
/// Boundary: the route of the transfer block.
/// Mocked:  nothing; the operating system's generator runs real.
#[test]
fn a_transfer_proof_fails_against_a_statement_with_another_expiry() {
    struct ExpiryConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        identity_element: P::G1,
        seller_secrets: KeyPairComponents<P::Scalar>,
        offset: Secret<P::Scalar>,
        coins: EnvelopeCoins<P::Scalar>,
        seller_keys: PublicKeysComponents<P::G1, P::G2>,
        buyer_keys: PublicKeysComponents<P::G1, P::G2>,
        old_envelope: EnvelopeComponents<P::G1, P::G2>,
        new_envelope: EnvelopeComponents<P::G1, P::G2>,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F> for ExpiryConsumer<'_, P> {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;

            // Arrange
            let keys_and_envelopes = |expiry: Option<u64>| AlgebraicTransferStatementOverrides {
                context: Some(build_delivery_context(
                    TransferPurpose::Transfer,
                    DeliveryContextOverrides {
                        expiry,
                        ..Default::default()
                    },
                )),
                identity_element: Some(self.identity_element.clone()),
                seller_keys: Some(PublicKeysComponents {
                    pk1: self.seller_keys.pk1.clone(),
                    pk2: self.seller_keys.pk2.clone(),
                }),
                buyer_keys: Some(PublicKeysComponents {
                    pk1: self.buyer_keys.pk1.clone(),
                    pk2: self.buyer_keys.pk2.clone(),
                }),
                old_envelope: Some(EnvelopeComponents {
                    c1: self.old_envelope.c1.clone(),
                    c2: self.old_envelope.c2.clone(),
                    d1: self.old_envelope.d1.clone(),
                    d2: self.old_envelope.d2.clone(),
                }),
                new_envelope: Some(EnvelopeComponents {
                    c1: self.new_envelope.c1.clone(),
                    c2: self.new_envelope.c2.clone(),
                    d1: self.new_envelope.d1.clone(),
                    d2: self.new_envelope.d2.clone(),
                }),
            };
            let statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, keys_and_envelopes(None));
            let other = build_algebraic_transfer_statement::<P, F>(
                self.pairing,
                keys_and_envelopes(Some(1_700_000_001)),
            );
            let proof = adapter
                .prove_transfer(
                    ProveTransferParams,
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveTransferPayloadOverrides {
                            seller_secrets: Some(KeyPairComponents {
                                x: build_secret(
                                    self.seller_secrets.x.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                y: build_secret(
                                    self.seller_secrets.y.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            offset: Some(build_secret(
                                self.offset.expose().clone(),
                                SecretConstructorParamsOverrides,
                            )),
                            coins: Some(EnvelopeCoins {
                                rho: build_secret(
                                    self.coins.rho.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                sigma: build_secret(
                                    self.coins.sigma.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                        },
                    ),
                )
                .expect("the families' witnesses prove")
                .proof;

            // Act
            let baseline = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );
            let refused = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&other),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(baseline.ok().map(|success| success.is_valid), Some(true));
            assert_eq!(refused.ok().map(|success| success.is_valid), Some(false));
        }
    }

    struct CaseConsumer;

    impl ICreateDeliveryProofCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
        ) -> Self::Output {
            let bound = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw = |length: usize| {
                random
                    .fill_bytes(FillBytesParams, FillBytesPayload { length })
                    .expect("the operating system's generator draws")
                    .bytes
            };
            for &scope in K::DECLARATION.identity_scopes {
                // Arrange
                let root = build_root(Default::default());
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(match scope {
                                IdentityScope::Entitlement => SetupScope::Entitlement,
                                IdentityScope::Asset => SetupScope::Asset { root: &root },
                            }),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(draw(bound)),
                            u0_uniform: Some(draw(bound)),
                            u1_uniform: Some(draw(bound)),
                        }),
                    )
                    .expect("setup admits the scope the case selects and the drawn uniforms");
                let derived = kem
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            match scope {
                                IdentityScope::Entitlement => {
                                    DeriveIdentityPayloadOverrides::default()
                                }
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
                            IssuePayloadOverrides {
                                uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("issue admits the credential inputs");
                let identity_element = kem
                    .identity_element_components(
                        build_identity_element_components_params(Default::default()),
                        build_identity_element_components_payload(
                            &derived.identity_element,
                            Default::default(),
                        ),
                    )
                    .expect("the identity element's components read")
                    .components;
                let credential = kem
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(&issued.credential, Default::default()),
                    )
                    .expect("the credential's components read")
                    .components;
                let seller = key_agreement
                    .generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(draw(bound)),
                                y_uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("the seller's drawn keys generate");
                let buyer = key_agreement
                    .generate_keys(
                        build_generate_keys_params(Default::default()),
                        build_generate_keys_payload(
                            pairing,
                            GenerateKeysPayloadOverrides {
                                x_uniform: Some(draw(bound)),
                                y_uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("the buyer's drawn keys generate");
                let seller_components = key_agreement
                    .key_pair_public_keys(
                        build_key_pair_public_keys_params(Default::default()),
                        build_key_pair_public_keys_payload(&seller.key_pair, Default::default()),
                    )
                    .expect("the seller's public keys read")
                    .components;
                let seller_public_keys = key_agreement
                    .public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &seller.possession,
                            PublicKeysFromComponentsPayloadOverrides {
                                components: Some(seller_components),
                            },
                        ),
                    )
                    .expect("the seller's own proof admits its public keys")
                    .public_keys;
                let buyer_components = key_agreement
                    .key_pair_public_keys(
                        build_key_pair_public_keys_params(Default::default()),
                        build_key_pair_public_keys_payload(&buyer.key_pair, Default::default()),
                    )
                    .expect("the buyer's public keys read")
                    .components;
                let buyer_public_keys = key_agreement
                    .public_keys_from_components(
                        build_public_keys_from_components_params(Default::default()),
                        build_public_keys_from_components_payload(
                            pairing,
                            &buyer.possession,
                            PublicKeysFromComponentsPayloadOverrides {
                                components: Some(buyer_components),
                            },
                        ),
                    )
                    .expect("the buyer's own proof admits its public keys")
                    .public_keys;
                let seller_keys = key_agreement
                    .public_keys_components(
                        build_public_keys_components_params(Default::default()),
                        build_public_keys_components_payload(
                            &seller_public_keys,
                            Default::default(),
                        ),
                    )
                    .expect("the seller's components read")
                    .components;
                let buyer_keys = key_agreement
                    .public_keys_components(
                        build_public_keys_components_params(Default::default()),
                        build_public_keys_components_payload(
                            &buyer_public_keys,
                            Default::default(),
                        ),
                    )
                    .expect("the buyer's components read")
                    .components;
                let seller_secrets = key_agreement
                    .key_pair_components(
                        build_key_pair_components_params(Default::default()),
                        build_key_pair_components_payload(&seller.key_pair, Default::default()),
                    )
                    .expect("the seller's key-pair components read")
                    .components;
                let wrapped_old = key_agreement
                    .wrap_to(
                        build_wrap_to_params(Default::default()),
                        build_wrap_to_payload(
                            pairing,
                            &seller_public_keys,
                            WrapToPayloadOverrides {
                                credential: Some(CredentialComponents {
                                    a: credential.a.clone(),
                                    b: credential.b.clone(),
                                }),
                            },
                        ),
                    )
                    .expect("the credential wraps to the seller's keys");
                let opened = key_agreement
                    .unwrap(
                        build_unwrap_params(Default::default()),
                        build_unwrap_payload(
                            &seller.key_pair,
                            &wrapped_old.envelope,
                            Default::default(),
                        ),
                    )
                    .expect("the seller's unwrap is infallible");
                let delivered = kem
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
                let rerandomized = kem
                    .rerandomize(
                        build_rerandomize_params(Default::default()),
                        build_rerandomize_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &delivered,
                            RerandomizePayloadOverrides {
                                uniform: Some(draw(bound)),
                            },
                        ),
                    )
                    .expect("the drawn uniform rerandomizes the credential");
                let rerandomized_components = kem
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(
                            &rerandomized.credential,
                            Default::default(),
                        ),
                    )
                    .expect("the credential's components read")
                    .components;
                let wrapped_new = key_agreement
                    .wrap_to(
                        build_wrap_to_params(Default::default()),
                        build_wrap_to_payload(
                            pairing,
                            &buyer_public_keys,
                            WrapToPayloadOverrides {
                                credential: Some(rerandomized_components),
                            },
                        ),
                    )
                    .expect("the credential wraps to the buyer's keys");
                let old_envelope = key_agreement
                    .envelope_components(
                        build_envelope_components_params(Default::default()),
                        build_envelope_components_payload(
                            &wrapped_old.envelope,
                            Default::default(),
                        ),
                    )
                    .expect("the old envelope's components read")
                    .components;
                let new_envelope = key_agreement
                    .envelope_components(
                        build_envelope_components_params(Default::default()),
                        build_envelope_components_payload(
                            &wrapped_new.envelope,
                            Default::default(),
                        ),
                    )
                    .expect("the new envelope's components read")
                    .components;

                for concrete in DELIVERY_PROOF_CONCRETES {
                    // Act
                    let created = create_delivery_proof::<P, E, F, ExpiryConsumer<'_, P>>(
                        &build_create_delivery_proof_deps(
                            pairing,
                            hash_to_scalar,
                            encoder,
                            random,
                            ExpiryConsumer {
                                pairing,
                                identity_element: identity_element.element.clone(),
                                seller_secrets: KeyPairComponents {
                                    x: build_secret(
                                        seller_secrets.x.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                    y: build_secret(
                                        seller_secrets.y.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                },
                                offset: build_secret(
                                    rerandomized.offset.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                coins: EnvelopeCoins {
                                    rho: build_secret(
                                        wrapped_new.coins.rho.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                    sigma: build_secret(
                                        wrapped_new.coins.sigma.expose().clone(),
                                        SecretConstructorParamsOverrides,
                                    ),
                                },
                                seller_keys: PublicKeysComponents {
                                    pk1: seller_keys.pk1.clone(),
                                    pk2: seller_keys.pk2.clone(),
                                },
                                buyer_keys: PublicKeysComponents {
                                    pk1: buyer_keys.pk1.clone(),
                                    pk2: buyer_keys.pk2.clone(),
                                },
                                old_envelope: EnvelopeComponents {
                                    c1: old_envelope.c1.clone(),
                                    c2: old_envelope.c2.clone(),
                                    d1: old_envelope.d1.clone(),
                                    d2: old_envelope.d2.clone(),
                                },
                                new_envelope: EnvelopeComponents {
                                    c1: new_envelope.c1.clone(),
                                    c2: new_envelope.c2.clone(),
                                    d1: new_envelope.d1.clone(),
                                    d2: new_envelope.d2.clone(),
                                },
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );

                    // Assert
                    assert!(created.is_ok());
                }
            }
        }
    }

    for pairing_case in PAIRING_CONCRETES {
        for selection in DECLARED_ENCODING_SELECTIONS {
            for hash_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_case in DECLARED_KEM_SELECTIONS {
                        for key_agreement_case in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            // Act
                            let result = run_create_delivery_proof_case(
                                &build_create_delivery_proof_case_deps(
                                    CaseConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_case_params(
                                    CreateDeliveryProofCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_case),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(hash_case.concrete),
                                                identifier: Some(hash_case.identifier),
                                            },
                                        )),
                                        encoding: Some(build_create_encoding_params(
                                            CreateEncodingParamsOverrides {
                                                concrete: Some(selection.concrete),
                                                identifier: Some(selection.identifier),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_case),
                                        key_agreement: Some(*key_agreement_case),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    },
                                ),
                                build_create_delivery_proof_case_payload(Default::default()),
                            );

                            // Assert
                            assert!(result.is_ok());
                        }
                    }
                }
            }
        }
    }
    // a_transfer_proof_fails_against_a_statement_with_another_expiry
}

// Enumeration: the public surface's entries and the block of the same name
//   that proves each — the declaration entry
//   `an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version`,
//   the mint entry
//   `a_credential_issued_and_wrapped_through_the_families_is_proved_by_a_mint_proof_that_verifies`,
//   the transfer entry
//   `a_credential_rerandomized_and_wrapped_to_the_buyer_is_proved_by_a_transfer_proof_that_verifies_and_opens`,
//   and the expiry entry
//   `a_transfer_proof_fails_against_a_statement_with_another_expiry`.
