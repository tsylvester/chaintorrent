#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, ParameterSetIdentifierTryNewErrorReturn,
    SuiteIdentifierConstructorParamsOverrides, SuiteIdentifierTryNewErrorReturn,
    build_parameter_set_identifier, build_secret, build_suite_identifier,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, IDecoderAdapter,
    IEncoderAdapter, MockIEncodingContract, build_canonical_fields, build_create_encoding_deps,
    build_create_encoding_payload, build_mock_i_encoding_consumer, create_encoding,
};
use envelope::{
    CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS, GenerateKeysErrorReturn,
    IKeyAgreementAdapter, KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params, build_domain_tag, expected_uniform_bytes,
};
use hashing::{RootConstructorParamsOverrides, RootTryNewErrorReturn, build_root};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, KemIdentity, SetupErrorReturn, SetupParamsOverrides,
    SetupPayloadOverrides, build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MockIPairingAdapterFailureMode, PAIRING_CONCRETES,
    PairingConcrete, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, DeliveryProofConcrete, DeliveryRelation,
    IDeliveryProofAdapter, MintPurpose, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, IStatementMutationsCaseConsumer, MINT_MUTATION_FIELDS, MintErrorReturn,
    StatementMutationsCaseParamsOverrides, StatementMutationsErrorReturn,
    StatementMutationsParamsOverrides, StatementMutationsPayloadOverrides,
    TRANSFER_MUTATION_FIELDS, VectorCountConstructorParamsOverrides,
    build_statement_mutations_case_deps, build_statement_mutations_case_params,
    build_statement_mutations_case_payload, build_statement_mutations_deps,
    build_statement_mutations_params, build_statement_mutations_payload, build_vector_count,
    run_statement_mutations_case, statement_mutations,
};

const CANONICAL: &[u8] = b"pkg@1.0.0#1";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

fn os_source() -> Option<random::CreateRandomSourceParams> {
    Some(build_create_random_source_params(
        CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        },
    ))
}

fn base_params() -> StatementMutationsCaseParamsOverrides {
    StatementMutationsCaseParamsOverrides {
        random_source: os_source(),
        ..Default::default()
    }
}

/// Contract: own entry `every_baseline_statement_verifies_and_every_mutation_is_refused`,
///   as an outside caller observes it.
/// Arrange: `count` of `2`; every case, the canonical bytes `b"pkg@1.0.0#1"`
///   and the root `build_root` builds from `[0x44; 32]`.
/// Act:     `statement_mutations`.
/// Assert:  both baselines verify, the `field`s equal `MINT_MUTATION_FIELDS`
///   and `TRANSFER_MUTATION_FIELDS` in order, and every mutation is refused.
/// Boundary: the public call `statement_mutations`, with the route
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`, each selected
///   concrete through its `try_new`, `statement_mutations`, and every
///   adapter method it calls.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_baseline_statement_verifies_and_every_mutation_is_refused() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.mint_is_valid);
                assert!(record.transfer_is_valid);
                assert_eq!(
                    record
                        .mint_mutations
                        .iter()
                        .map(|mutation| mutation.field)
                        .collect::<Vec<_>>(),
                    MINT_MUTATION_FIELDS.to_vec()
                );
                assert_eq!(
                    record
                        .transfer_mutations
                        .iter()
                        .map(|mutation| mutation.field)
                        .collect::<Vec<_>>(),
                    TRANSFER_MUTATION_FIELDS.to_vec()
                );
                for mutation in &record.mint_mutations {
                    assert!(!mutation.is_valid);
                }
                for mutation in &record.transfer_mutations {
                    assert!(!mutation.is_valid);
                }
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for ka_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_statement_mutations_case(
                                    &build_statement_mutations_case_deps(
                                        Consumer,
                                        Default::default(),
                                    ),
                                    build_statement_mutations_case_params(
                                        StatementMutationsCaseParamsOverrides {
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
                                            key_agreement: Some(*ka_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            ..base_params()
                                        },
                                    ),
                                    build_statement_mutations_case_payload(
                                        CANONICAL,
                                        &asset_root(),
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`.
/// Act:     `statement_mutations`.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `10` times
///   `UNIFORM_BYTES_LENGTH` bytes, and no two records share `draws`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   block above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(3),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 3);
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            for (index, record) in success.vectors.iter().enumerate() {
                assert_eq!(record.draws.len(), 10 * length);
                for other in &success.vectors[index + 1..] {
                    assert_ne!(record.draws, other.draws);
                }
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(Consumer, Default::default()),
                build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    ..base_params()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole`, and the
///   absorbed `delivery/mint` entry `a_refused_draw_reaches_the_caller_whole`.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`; the
///   source for `RandomSourceKind::Mock(FillBytesRefused)`, and the
///   operating-system source as the counterfactual.
/// Act:     `statement_mutations`, once per source.
/// Assert:  the result under the refusing source equals
///   `Err(StatementMutationsErrorReturn::Mint(MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)))`
///   whole, and the result under the operating-system source `is_ok()`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)
                    ))
                );
            } else {
                returned.expect("the subject's calls succeed");
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for source in [
                RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                RandomSourceKind::OperatingSystem,
            ] {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: matches!(source, RandomSourceKind::Mock(_)),
                        },
                        Default::default(),
                    ),
                    build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides { kind: Some(source) },
                        )),
                        ..base_params()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_setup_sampling_refusal_reaches_the_caller_whole`.
/// Arrange: `count` of `1`; `create_pairing` over the pairing mock under
///   `SampleScalarWrongLength`, with each selection and scope of
///   `DECLARED_KEM_SELECTIONS`, and each member of `PAIRING_CONCRETES` as the
///   counterfactual; the expected `error` the literal `WrongLength
///   { expected: 16, actual: 16 }`.
/// Act:     `statement_mutations`, once per pairing selection.
/// Assert:  the result under the refusing selection equals
///   `Err(StatementMutationsErrorReturn::Mint(MintErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 16 }))))`
///   whole, and the result under each member of `PAIRING_CONCRETES` `is_ok()`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   blocks above, through `mint` and `setup`.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(MintErrorReturn::Setup(
                        SetupErrorReturn::MasterScalarSampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: 16,
                                actual: 16,
                            }
                        )
                    )))
                );
            } else {
                returned.expect("the subject's calls succeed");
            }
        }
    }
    for kem_selection in DECLARED_KEM_SELECTIONS {
        for pairing_concrete in [PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
        )]
        .into_iter()
        .chain(PAIRING_CONCRETES.iter().copied())
        {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: matches!(pairing_concrete, PairingConcrete::Mock(_)),
                    },
                    Default::default(),
                ),
                build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    ..base_params()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: carried `delivery/mint` and `delivery/transfer` entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`,
///   restated by the spec.
/// Arrange: each entitlement-scope selection of `DECLARED_KEM_SELECTIONS`
///   over every member of `PAIRING_CONCRETES`, each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring `Keccak256V1`, and each
///   case of `DECLARED_ENCODING_SELECTIONS` declaring `EthereumAbiV1`; the
///   canonical bytes are the bytes `create_encoding` returns for
///   `MockIEncodingContract` over `build_canonical_fields`, through
///   `build_mock_i_encoding_consumer`; `count` of `1`.
/// Act:     `statement_mutations`, once, the case payload's canonical bytes
///   those bytes.
/// Assert:  `encode_scalar` of the `scalar` of the replayed identity element
///   equals `encode_scalar` of the scalar `sample_from_uniform_bytes`
///   returns for `expected_uniform_bytes` over the tag `build_domain_tag`
///   builds from `K::DECLARATION.identity_tag`, those bytes, and
///   `UNIFORM_BYTES_LENGTH`.
/// Boundary: the public call `statement_mutations`, with the route the spec
///   states for the entry.
/// Mocked:  the encoding family's official consumer mock, which hands the
///   bytes the encoding returned to the block.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    // Arrange
    let description = MockIEncodingContract;
    let mut encodings: Vec<Vec<u8>> = Vec::new();
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        if encoding_selection.identifier == EncodingIdentifier::EthereumAbiV1 {
            let consumer = build_mock_i_encoding_consumer(
                &description,
                build_canonical_fields(Default::default()),
                Vec::new(),
            );
            let outcome = create_encoding(
                &build_create_encoding_deps(consumer),
                CreateEncodingParams {
                    concrete: encoding_selection.concrete,
                    identifier: encoding_selection.identifier,
                },
                build_create_encoding_payload(Default::default()),
            )
            .expect("the declared encoding selection constructs");
            encodings.push(
                outcome
                    .output
                    .encoded
                    .expect("the real encoder encodes the description's value")
                    .bytes,
            );
        }
    }

    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let uniforms: Vec<domain::Secret<Vec<u8>>> = record
                .draws
                .chunks_exact(length)
                .map(|chunk| {
                    domain::Secret::try_new(domain::SecretConstructorParams {
                        value: chunk.to_vec(),
                    })
                    .expect("a draw of the declared length is a valid uniform")
                })
                .collect();
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(build_secret(
                            uniforms[0].expose().clone(),
                            Default::default(),
                        )),
                        u0_uniform: Some(build_secret(
                            uniforms[1].expose().clone(),
                            Default::default(),
                        )),
                        u1_uniform: Some(build_secret(
                            uniforms[2].expose().clone(),
                            Default::default(),
                        )),
                    }),
                )
                .expect("setup over the replayed uniforms succeeds");
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(identity.identity),
                        },
                    ),
                )
                .expect("derive_identity over the admitted identity succeeds");
            let identity_components = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: scalar.clone(),
                        },
                    )
                    .expect("the scalar encoding is infallible")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            if let KemIdentity::Entitlement { canonical } = identity.identity {
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                });
                let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(
                                &tag,
                                canonical,
                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            ),
                            Default::default(),
                        ),
                    },
                )
                .expect("the expected uniform bytes sample")
                .scalar
                .expose()
                .clone();
                assert_eq!(
                    encode_scalar(&identity_components.scalar),
                    encode_scalar(&expected)
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if h2s_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for kem_selection in DECLARED_KEM_SELECTIONS {
                if kem_selection.scope != IdentityScope::Entitlement {
                    continue;
                }
                for canonical in &encodings {
                    let result = run_statement_mutations_case(
                        &build_statement_mutations_case_deps(Consumer, Default::default()),
                        build_statement_mutations_case_params(
                            StatementMutationsCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*h2s_selection),
                                kem: Some(*kem_selection),
                                ..base_params()
                            },
                        ),
                        build_statement_mutations_case_payload(
                            canonical.as_slice(),
                            &asset_root(),
                            Default::default(),
                        ),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, the
///   hash-to-scalar mock under `SamplingRefused`, and each member of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` as the counterfactual.
/// Act:     `statement_mutations`, once per hash-to-scalar selection.
/// Assert:  the result under the refusing selection equals the whole `Err`
///   under each scope, and under each declared selection `is_ok()`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   blocks above.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let error =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                let expected = match identity.setup_scope {
                    kem::SetupScope::Entitlement => {
                        StatementMutationsErrorReturn::Mint(MintErrorReturn::DeriveIdentity(
                            DeriveIdentityErrorReturn::HashToScalar(error),
                        ))
                    }
                    kem::SetupScope::Asset { .. } => StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)),
                    ),
                };
                assert_eq!(returned.err(), Some(expected));
            } else {
                returned.expect("the subject's calls succeed");
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for refusing in [true, false] {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: refusing,
                        },
                        Default::default(),
                    ),
                    build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        hash_to_scalar: if refusing {
                            Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(HashToScalarConcrete::Mock(
                                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                                    )),
                                    ..Default::default()
                                },
                            ))
                        } else {
                            None
                        },
                        ..base_params()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        hash_to_scalar: Some(*h2s_selection),
                        ..base_params()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_key_generation_nonce_draw_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it.
/// Arrange: as the private integration test's block of the same name.
/// Act:     `statement_mutations`, once per key-agreement selection.
/// Assert:  the result under the refusing selection equals the whole `Err`,
///   and under each declared selection `is_ok()`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   blocks above.
/// Mocked:  the key-agreement family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_key_generation_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(
                            MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
                        ))
                    ))
                );
            } else {
                returned.expect("the subject's calls succeed");
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..base_params()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for ka_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        key_agreement: Some(*ka_selection),
                        ..base_params()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_wrap_coin_draw_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it.
/// Arrange: as the private integration test's block of the same name.
/// Act:     `statement_mutations`, once per key-agreement selection.
/// Assert:  the result under the refusing selection equals the whole `Err`,
///   and under each declared selection `is_ok()`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   blocks above.
/// Mocked:  the key-agreement family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_wrap_coin_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(
                            MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused
                        ))
                    ))
                );
            } else {
                returned.expect("the subject's calls succeed");
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..base_params()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for ka_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        key_agreement: Some(*ka_selection),
                        ..base_params()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_nonce_draw_refusal_reaches_the_caller_whole`, as an
///   outside caller observes it.
/// Arrange: as the private integration test's block of the same name.
/// Act:     `statement_mutations`, once per delivery-proof selection.
/// Assert:  the result under the refusing selection equals the whole `Err`,
///   and under each declared selection `is_ok()`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   blocks above.
/// Mocked:  the delivery-proof family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
                            MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                        ))
                    ))
                );
            } else {
                returned.expect("the subject's calls succeed");
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    delivery_proof: Some(build_create_delivery_proof_params(
                        CreateDeliveryProofParamsOverrides {
                            concrete: Some(DeliveryProofConcrete::Mock(
                                MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..base_params()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        delivery_proof: Some(build_create_delivery_proof_params(
                            CreateDeliveryProofParamsOverrides {
                                concrete: Some(*proof_concrete),
                                ..Default::default()
                            },
                        )),
                        ..base_params()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_refused_mutated_domain_value_reaches_the_caller_whole`.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`; typed mint contexts
///   overriding, in turn, `suite_identifier` with an identifier of
///   thirty-one `0x00` bytes and a last byte `0x01` and version `1`,
///   `asset_root` with `build_root` over those bytes, and
///   `parameter_set_identifier` with `build_parameter_set_identifier` over
///   them, and, as the counterfactual, the default contexts.
/// Act:     `statement_mutations`, once per mint context.
/// Assert:  the results equal the three whole `Err`s, and the result over
///   the default contexts `is_ok()`.
/// Boundary: the public call `statement_mutations`, with the route of the
///   blocks above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_refused_mutated_domain_value_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mut bytes = [0x00u8; 32];
            bytes[31] = 0x01;
            let expected = |index: usize| match index {
                0 => StatementMutationsErrorReturn::MutatedSuiteIdentifier {
                    relation: DeliveryRelation::Mint,
                    error: SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier,
                },
                1 => StatementMutationsErrorReturn::MutatedAssetRoot {
                    relation: DeliveryRelation::Mint,
                    error: RootTryNewErrorReturn::AllZero,
                },
                _ => StatementMutationsErrorReturn::MutatedParameterSetIdentifier {
                    relation: DeliveryRelation::Mint,
                    error: ParameterSetIdentifierTryNewErrorReturn::AllZero,
                },
            };
            let mint_contexts = [
                build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    DeliveryContextOverrides {
                        suite_identifier: Some(build_suite_identifier(
                            SuiteIdentifierConstructorParamsOverrides {
                                identifier: Some(bytes),
                                version: Some(1),
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    DeliveryContextOverrides {
                        asset_root: Some(build_root(RootConstructorParamsOverrides {
                            bytes: Some(bytes),
                        })),
                        ..Default::default()
                    },
                ),
                build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    DeliveryContextOverrides {
                        parameter_set_identifier: Some(build_parameter_set_identifier(
                            ParameterSetIdentifierConstructorParamsOverrides { bytes: Some(bytes) },
                        )),
                        ..Default::default()
                    },
                ),
            ];
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            for (index, mint_context) in mint_contexts.iter().enumerate() {
                let returned = statement_mutations(
                    &build_statement_mutations_deps(
                        pairing,
                        kem,
                        key_agreement,
                        delivery_proof,
                        random,
                        Default::default(),
                    ),
                    build_statement_mutations_params(StatementMutationsParamsOverrides {
                        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                            count: Some(1),
                        })),
                    }),
                    build_statement_mutations_payload(
                        mint_context,
                        &transfer_context,
                        StatementMutationsPayloadOverrides {
                            identity: Some(identity),
                        },
                    ),
                );
                // Assert
                if self.expects_refusal {
                    assert_eq!(returned.err(), Some(expected(index)));
                } else {
                    returned.expect("the subject's calls succeed");
                }
            }
            let default_mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &default_mint,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            returned.expect("the default contexts' calls succeed");
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    ..base_params()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
        }
    }
}
