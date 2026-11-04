#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, ParameterSetIdentifierTryNewErrorReturn,
    Secret, SecretConstructorParams, SuiteIdentifierConstructorParamsOverrides,
    SuiteIdentifierTryNewErrorReturn, build_parameter_set_identifier, build_secret,
    build_suite_identifier,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, IDecoderAdapter,
    IEncoderAdapter, build_create_encoding_deps, build_create_encoding_payload,
    build_mock_i_encoding_consumer, create_encoding,
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
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DeriveKeyErrorReturn,
    IKeyDerivationAdapter, KdfConcrete, MockIKeyDerivationAdapterDeriveKeyErrorReturn,
    MockIKeyDerivationAdapterFailureMode, build_create_key_derivation_params,
};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, KemIdentity, SetupErrorReturn, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_derive_identity_params, build_derive_identity_payload,
    build_encapsulate_params, build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PAIRING_CONCRETES, PairingConcrete,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_params,
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
use workflows::{
    PIECE_GROUP_KEY_LENGTH, WrapPieceGroupKeyErrorReturn, build_derivation_context,
    build_derivation_context_description,
};

use harness_crypto::{
    AdmittedKemIdentity, CrossHolderErrorReturn, CrossSetErrorReturn, IVectorsCaseConsumer,
    MintErrorReturn, NextSettlementField, ReplayErrorReturn, StatementMutationsErrorReturn,
    VectorCountConstructorParamsOverrides, VectorsCaseParamsOverrides, VectorsErrorReturn,
    VectorsParamsOverrides, VectorsPayloadOverrides, build_vector_count, build_vectors_case_deps,
    build_vectors_case_params, build_vectors_case_payload, build_vectors_deps,
    build_vectors_params, build_vectors_payload, run_vectors_case, vectors,
};

const CANONICAL: &[u8] = b"pkg@1.0.0#1";
const OTHER_CANONICAL: &[u8] = b"pkg@2.0.0#7";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

fn other_asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x45; 32]),
    })
}

fn os_source() -> Option<random::CreateRandomSourceParams> {
    Some(build_create_random_source_params(
        CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        },
    ))
}

fn case_payload<'a>(
    canonical: &'a [u8],
    root: &'a hashing::Root,
    other_root: &'a hashing::Root,
) -> harness_crypto::VectorsCasePayload<'a> {
    build_vectors_case_payload(
        canonical,
        OTHER_CANONICAL,
        root,
        other_root,
        Default::default(),
    )
}

fn default_contexts<F: IChainForms>() -> (
    proof::DeliveryContext<F, MintPurpose>,
    proof::DeliveryContext<F, TransferPurpose>,
    proof::DeliveryContext<F, TransferPurpose>,
) {
    let mint = build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
    let transfer =
        build_delivery_context::<F, TransferPurpose>(DELIVERY_PURPOSE_TRANSFER, Default::default());
    let next = build_delivery_context::<F, TransferPurpose>(
        DELIVERY_PURPOSE_TRANSFER,
        DeliveryContextOverrides {
            chain: Some(transfer.chain.clone()),
            entitlement_contract: Some(transfer.entitlement_contract.clone()),
            asset_root: Some(transfer.asset_root),
            parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
            source_entitlement: Some(transfer.source_entitlement.clone()),
            target_entitlement: Some(transfer.target_entitlement.clone()),
            old_interval: Some(transfer.new_interval.clone()),
            seller: Some(transfer.buyer.clone()),
            ..Default::default()
        },
    );
    (mint, transfer, next)
}

fn selections(
    pairing_concrete: PairingConcrete,
    h2s_selection: &hash_to_scalar::CreateHashToScalarParams,
    encoding_selection: &encoding::CreateEncodingParams,
    forms_concrete: &chain::ChainFormsConcrete,
    kem_selection: &kem::CreateKemParams,
    key_agreement_selection: &envelope::CreateKeyAgreementParams,
    proof_concrete: &proof::DeliveryProofConcrete,
    kdf_selection: &kdf::CreateKeyDerivationParams,
) -> harness_crypto::VectorsCaseParams {
    build_vectors_case_params(VectorsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        })),
        hash_to_scalar: Some(*h2s_selection),
        encoding: Some(CreateEncodingParams {
            concrete: encoding_selection.concrete,
            identifier: encoding_selection.identifier,
        }),
        key_derivation: Some(*kdf_selection),
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
        random_source: os_source(),
    })
}

fn encode_g1<P: IPairingArithmetic>(pairing: &P, point: &P::G1) -> Vec<u8> {
    pairing
        .encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: point.clone(),
            },
        )
        .expect("the first-group encoding is infallible")
        .bytes
        .as_ref()
        .to_vec()
}

fn encode_scalar<P: IPairingArithmetic>(pairing: &P, scalar: &P::Scalar) -> Vec<u8> {
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
}

/// Contract: own entry `every_set_holds_its_configured_count_of_records`, as
///   an outside caller observes it.
/// Arrange: as the private integration test's block of the same name, the
///   counts `1` through `7` in set order; every member of `PAIRING_CONCRETES`
///   with each selection and scope of `DECLARED_KEM_SELECTIONS`.
/// Act:     `vectors`, once per case, in the case consumer's method.
/// Assert:  each set's `vectors` holds its count, `non_convertibility`'s
///   holding `3` under the entitlement scope and empty under the asset scope;
///   each case's `Result` `is_ok()`.
/// Boundary: the public call `vectors`, with the route `create_random_source`,
///   `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_key_derivation`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`, each selected
///   concrete through its `try_new`, `vectors`, each set function, and every
///   adapter method they call.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_set_holds_its_configured_count_of_records() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            let count = |count: u32| {
                build_vector_count(VectorCountConstructorParamsOverrides { count: Some(count) })
            };
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(VectorsParamsOverrides {
                    cross_holder: Some(count(1)),
                    rerandomization: Some(count(2)),
                    non_convertibility: Some(count(3)),
                    malformed_capsules: Some(count(4)),
                    cross_set: Some(count(5)),
                    statement_mutations: Some(count(6)),
                    replay: Some(count(7)),
                }),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.cross_holder.vectors.len(), 1);
            assert_eq!(success.rerandomization.vectors.len(), 2);
            match identity.setup_scope {
                SetupScope::Entitlement => {
                    assert_eq!(success.non_convertibility.vectors.len(), 3);
                }
                SetupScope::Asset { .. } => {
                    assert!(success.non_convertibility.vectors.is_empty());
                }
            }
            assert_eq!(success.malformed_capsules.vectors.len(), 4);
            assert_eq!(success.cross_set.vectors.len(), 5);
            assert_eq!(success.statement_mutations.vectors.len(), 6);
            assert_eq!(success.replay.vectors.len(), 7);
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer, Default::default()),
                build_vectors_case_params(VectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    random_source: os_source(),
                    ..Default::default()
                }),
                case_payload(CANONICAL, &asset_root(), &other_asset_root()),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: own entry `every_set_holds_the_verdicts_its_function_computes`,
///   as an outside caller observes it.
/// Arrange: every count `1`; every case under the entitlement scope.
/// Act:     `vectors`, once per case, in the case consumer's method.
/// Assert:  as the private integration test's block of the same name, through
///   the crate's public surface.
/// Boundary: the public call `vectors`, with the route of the block above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_set_holds_the_verdicts_its_function_computes() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            let cross_holder = &success.cross_holder.vectors[0];
            assert_eq!(
                cross_holder.issued_decapsulated.key_material().expose(),
                cross_holder.encapsulated.key_material().expose()
            );
            assert_eq!(
                cross_holder.successor_decapsulated.key_material().expose(),
                cross_holder.encapsulated.key_material().expose()
            );
            let rerandomization = &success.rerandomization.vectors[0];
            assert!(rerandomization.issued_is_valid);
            assert!(rerandomization.rerandomized_is_valid);
            let non_convertibility = &success.non_convertibility.vectors[0];
            assert!(non_convertibility.is_valid);
            assert!(!non_convertibility.other_is_valid);
            let malformed_capsules = &success.malformed_capsules.vectors[0];
            assert!(malformed_capsules.is_well_formed);
            assert!(!malformed_capsules.displaced_is_well_formed);
            let cross_set = &success.cross_set.vectors[0];
            assert_eq!(
                cross_set.first.unwrapped.expose(),
                cross_set.piece_group_key.expose()
            );
            assert_eq!(
                cross_set.second.unwrapped.expose(),
                cross_set.piece_group_key.expose()
            );
            let statement_mutations = &success.statement_mutations.vectors[0];
            assert!(statement_mutations.mint_is_valid);
            assert!(statement_mutations.transfer_is_valid);
            for mutation in &statement_mutations.mint_mutations {
                assert!(!mutation.is_valid);
            }
            for mutation in &statement_mutations.transfer_mutations {
                assert!(!mutation.is_valid);
            }
            let replay = &success.replay.vectors[0];
            assert!(replay.transfer_is_valid);
            assert!(replay.next_is_valid);
            assert!(!replay.replay_is_valid);
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        if kem_selection.scope != IdentityScope::Entitlement {
                            continue;
                        }
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                for kdf_selection in DECLARED_KDF_SELECTIONS {
                                    let result = run_vectors_case(
                                        &build_vectors_case_deps(Consumer, Default::default()),
                                        selections(
                                            *pairing_concrete,
                                            h2s_selection,
                                            encoding_selection,
                                            forms_concrete,
                                            kem_selection,
                                            key_agreement_selection,
                                            proof_concrete,
                                            kdf_selection,
                                        ),
                                        case_payload(CANONICAL, &asset_root(), &other_asset_root()),
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
}

/// Contract: own entry
///   `a_draw_refusal_reaches_the_caller_inside_the_cross_holder_variant`.
/// Arrange: every count `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`; the
///   source `create_random_source` returns for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, and, as the counterfactual, the source for
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `vectors`, once per source, in the case consumer's method.
/// Assert:  the result under the refusing source equals
///   `Err(VectorsErrorReturn::CrossHolder(CrossHolderErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter)))` whole, and the result
///   under the operating-system source `is_ok()`.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_inside_the_cross_holder_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::CrossHolder(
                        CrossHolderErrorReturn::FillBytes(
                            FillBytesErrorReturn::MockIRandomSourceAdapter
                        )
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for (kind, refuse) in [
                (
                    RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                    true,
                ),
                (RandomSourceKind::OperatingSystem, false),
            ] {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides { kind: Some(kind) },
                        )),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_setup_sampling_refusal_reaches_the_caller_inside_the_cross_holder_variant`.
/// Arrange: every count `1`; `create_pairing` over
///   `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`, with each selection and scope of
///   `DECLARED_KEM_SELECTIONS`, and, as the counterfactual, the pairing's
///   default selection.
/// Act:     `vectors`, once per pairing selection, in the case consumer's
///   method.
/// Assert:  the result under the refusing selection equals
///   `Err(VectorsErrorReturn::CrossHolder(CrossHolderErrorReturn::Setup(
///   SetupErrorReturn::MasterScalarSampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: 16, actual: 16 }))))` whole, and the result
///   under the default selection `is_ok()`.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_inside_the_cross_holder_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::CrossHolder(
                        CrossHolderErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: 16,
                                actual: 16,
                            }
                        ))
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for kem_selection in DECLARED_KEM_SELECTIONS {
        for refuse in [true, false] {
            let pairing = if refuse {
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                })
            } else {
                build_create_pairing_params(Default::default())
            };
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse }, Default::default()),
                build_vectors_case_params(VectorsCaseParamsOverrides {
                    pairing: Some(pairing),
                    kem: Some(*kem_selection),
                    random_source: os_source(),
                    ..Default::default()
                }),
                case_payload(CANONICAL, &asset_root(), &other_asset_root()),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: own entry
///   `a_next_settlement_refusal_reaches_the_caller_inside_the_replay_variant`.
/// Arrange: every count `1`; every member of `CHAIN_FORMS_CONCRETES` over
///   every member of `PAIRING_CONCRETES`; the private surface's contexts with
///   the next context's `old_interval` overridden with a clone of the
///   transfer context's `old_interval`, and, as the counterfactual, the
///   private surface's contexts.
/// Act:     `vectors`, once per context set, in the case consumer's method.
/// Assert:  the result over the overridden next context equals
///   `Err(VectorsErrorReturn::Replay(ReplayErrorReturn::NotTheNextSettlement {
///   field: NextSettlementField::OldInterval }))` whole, and the result over
///   the private surface's contexts `is_ok()`.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_next_settlement_refusal_reaches_the_caller_inside_the_replay_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let old_interval = if self.refuse {
                Some(transfer.old_interval.clone())
            } else {
                Some(transfer.new_interval.clone())
            };
            let next = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer.chain.clone()),
                    entitlement_contract: Some(transfer.entitlement_contract.clone()),
                    asset_root: Some(transfer.asset_root),
                    parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
                    source_entitlement: Some(transfer.source_entitlement.clone()),
                    target_entitlement: Some(transfer.target_entitlement.clone()),
                    old_interval,
                    seller: Some(transfer.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::Replay(
                        ReplayErrorReturn::NotTheNextSettlement {
                            field: NextSettlementField::OldInterval,
                        }
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for forms_concrete in CHAIN_FORMS_CONCRETES {
            for refuse in [true, false] {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        chain_forms: Some(build_create_chain_forms_params(
                            CreateChainFormsParamsOverrides {
                                concrete: Some(*forms_concrete),
                                ..Default::default()
                            },
                        )),
                        random_source: os_source(),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_refused_mutated_domain_value_reaches_the_caller_inside_the_statement_mutations_variant`.
/// Arrange: every count `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`; the private surface's
///   contexts with, in turn, the mint context's `suite_identifier` an
///   identifier of thirty-one `0x00` bytes and a last byte `0x01`, its
///   `asset_root` the root `build_root` returns over those bytes, and its
///   `parameter_set_identifier` the identifier
///   `build_parameter_set_identifier` returns over them, and, as the
///   counterfactual, the private surface's contexts.
/// Act:     `vectors`, once per context set, in the case consumer's method.
/// Assert:  the results equal each named `Mutated*` `Err` whole with
///   `relation: DeliveryRelation::Mint`, and the result over the private
///   surface's contexts `is_ok()`.
/// Boundary: the public call `vectors`, with the route of the blocks above
///   and the domain constructors.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_refused_mutated_domain_value_reaches_the_caller_inside_the_statement_mutations_variant() {
    #[derive(Clone, Copy)]
    enum Mutated {
        SuiteIdentifier,
        AssetRoot,
        ParameterSetIdentifier,
        Counterfactual,
    }

    struct Consumer {
        mutated: Mutated,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let mut bytes = [0x00u8; 32];
            bytes[31] = 0x01;
            let mut overrides = DeliveryContextOverrides::<F>::default();
            match self.mutated {
                Mutated::SuiteIdentifier => {
                    overrides.suite_identifier = Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some(bytes),
                            version: Some(1),
                        },
                    ));
                }
                Mutated::AssetRoot => {
                    overrides.asset_root = Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some(bytes),
                    }));
                }
                Mutated::ParameterSetIdentifier => {
                    overrides.parameter_set_identifier = Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides { bytes: Some(bytes) },
                    ));
                }
                Mutated::Counterfactual => {}
            }
            let mint = build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, overrides);
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer.chain.clone()),
                    entitlement_contract: Some(transfer.entitlement_contract.clone()),
                    asset_root: Some(transfer.asset_root),
                    parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
                    source_entitlement: Some(transfer.source_entitlement.clone()),
                    target_entitlement: Some(transfer.target_entitlement.clone()),
                    old_interval: Some(transfer.new_interval.clone()),
                    seller: Some(transfer.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            let expected = match self.mutated {
                Mutated::SuiteIdentifier => Some(VectorsErrorReturn::StatementMutations(
                    StatementMutationsErrorReturn::MutatedSuiteIdentifier {
                        relation: DeliveryRelation::Mint,
                        error: SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier,
                    },
                )),
                Mutated::AssetRoot => Some(VectorsErrorReturn::StatementMutations(
                    StatementMutationsErrorReturn::MutatedAssetRoot {
                        relation: DeliveryRelation::Mint,
                        error: RootTryNewErrorReturn::AllZero,
                    },
                )),
                Mutated::ParameterSetIdentifier => Some(VectorsErrorReturn::StatementMutations(
                    StatementMutationsErrorReturn::MutatedParameterSetIdentifier {
                        relation: DeliveryRelation::Mint,
                        error: ParameterSetIdentifierTryNewErrorReturn::AllZero,
                    },
                )),
                Mutated::Counterfactual => None,
            };
            assert_eq!(returned.err(), expected);
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for mutated in [
                Mutated::SuiteIdentifier,
                Mutated::AssetRoot,
                Mutated::ParameterSetIdentifier,
                Mutated::Counterfactual,
            ] {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { mutated }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        random_source: os_source(),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_hash_to_scalar_refusal_reaches_the_caller_inside_the_cross_holder_variant`,
///   as an outside caller observes it.
/// Arrange: as the private integration test's block of the same name, and, as
///   the counterfactual, each member of `DECLARED_HASH_TO_SCALAR_SELECTIONS`.
/// Act:     `vectors`, once per hash-to-scalar selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection as the private block
///   states it, whole, and the result under each declared selection `is_ok()`.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  nothing at the outer edge; the hash-to-scalar family's mock
///   concrete is selected by configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_inside_the_cross_holder_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                let expected = HashToScalarErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        actual: 0,
                    },
                );
                let expected = match identity.setup_scope {
                    SetupScope::Entitlement => {
                        VectorsErrorReturn::CrossHolder(CrossHolderErrorReturn::DeriveIdentity(
                            DeriveIdentityErrorReturn::HashToScalar(expected),
                        ))
                    }
                    SetupScope::Asset { .. } => VectorsErrorReturn::CrossHolder(
                        CrossHolderErrorReturn::Setup(SetupErrorReturn::HashToScalar(expected)),
                    ),
                };
                assert_eq!(returned.err(), Some(expected));
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                hash_to_scalar: Some(build_create_hash_to_scalar_params(
                    CreateHashToScalarParamsOverrides {
                        concrete: Some(HashToScalarConcrete::Mock(
                            MockIHashToScalarAdapterFailureMode::SamplingRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: os_source(),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                case_payload(CANONICAL, &asset_root(), &other_asset_root()),
            );
            assert!(result.is_ok());
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(*h2s_selection),
                        kem: Some(*kem_selection),
                        random_source: os_source(),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_key_derivation_refusal_reaches_the_caller_inside_the_cross_set_variant`,
///   as an outside caller observes it.
/// Arrange: as the private integration test's block of the same name.
/// Act:     `vectors`, once per key-derivation selection, in the case
///   consumer's method.
/// Assert:  as the private integration test's block of the same name, through
///   the crate's public surface.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  nothing at the outer edge; the key-derivation family's mock
///   concrete is selected by configuration.
#[test]
fn a_key_derivation_refusal_reaches_the_caller_inside_the_cross_set_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(identity.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(Default::default()),
                    )
                    .expect("the replay's setup succeeds");
                let encapsulated = kem
                    .encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            Default::default(),
                        ),
                    )
                    .expect("the replay's encapsulation succeeds");
                let length = encapsulated.encapsulated.key_material().expose().len();
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::CrossSet(CrossSetErrorReturn::Wrap(
                        WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::Mock(
                            MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                                length,
                            },
                        ))
                    )))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                key_derivation: Some(build_create_key_derivation_params(
                    CreateKeyDerivationParamsOverrides {
                        concrete: Some(KdfConcrete::Mock(
                            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: os_source(),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                case_payload(CANONICAL, &asset_root(), &other_asset_root()),
            );
            assert!(result.is_ok());
            for kdf_selection in DECLARED_KDF_SELECTIONS {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        key_derivation: Some(*kdf_selection),
                        kem: Some(*kem_selection),
                        random_source: os_source(),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_key_generation_nonce_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant`,
///   as an outside caller observes it.
/// Arrange: as the private integration test's block of the same name.
/// Act:     `vectors`, once per key-agreement selection, in the case
///   consumer's method.
/// Assert:  as the private integration test's block of the same name, through
///   the crate's public surface.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  nothing at the outer edge; the key-agreement family's mock
///   concrete is selected by configuration.
#[test]
fn a_key_generation_nonce_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::StatementMutations(
                        StatementMutationsErrorReturn::Mint(MintErrorReturn::GenerateKeys(
                            GenerateKeysErrorReturn::Mock(
                                MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                            )
                        ))
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: os_source(),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                case_payload(CANONICAL, &asset_root(), &other_asset_root()),
            );
            assert!(result.is_ok());
            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        key_agreement: Some(*key_agreement_selection),
                        kem: Some(*kem_selection),
                        random_source: os_source(),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_wrap_coin_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant`,
///   as an outside caller observes it.
/// Arrange: as the private integration test's block of the same name.
/// Act:     `vectors`, once per key-agreement selection, in the case
///   consumer's method.
/// Assert:  as the private integration test's block of the same name, through
///   the crate's public surface.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  nothing at the outer edge; the key-agreement family's mock
///   concrete is selected by configuration.
#[test]
fn a_wrap_coin_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::StatementMutations(
                        StatementMutationsErrorReturn::Mint(MintErrorReturn::WrapTo(
                            WrapToErrorReturn::Mock(
                                MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused,
                            )
                        ))
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: os_source(),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                case_payload(CANONICAL, &asset_root(), &other_asset_root()),
            );
            assert!(result.is_ok());
            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        key_agreement: Some(*key_agreement_selection),
                        kem: Some(*kem_selection),
                        random_source: os_source(),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_mint_nonce_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant`,
///   as an outside caller observes it.
/// Arrange: as the private integration test's block of the same name.
/// Act:     `vectors`, once per delivery-proof selection, in the case
///   consumer's method.
/// Assert:  as the private integration test's block of the same name, through
///   the crate's public surface.
/// Boundary: the public call `vectors`, with the route of the blocks above.
/// Mocked:  nothing at the outer edge; the delivery-proof family's mock
///   concrete is selected by configuration.
#[test]
fn a_mint_nonce_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant() {
    struct Consumer {
        refuse: bool,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::StatementMutations(
                        StatementMutationsErrorReturn::Mint(MintErrorReturn::ProveMint(
                            ProveMintErrorReturn::Mock(
                                MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                            )
                        ))
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                delivery_proof: Some(build_create_delivery_proof_params(
                    CreateDeliveryProofParamsOverrides {
                        concrete: Some(DeliveryProofConcrete::Mock(
                            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: os_source(),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                case_payload(CANONICAL, &asset_root(), &other_asset_root()),
            );
            assert!(result.is_ok());
            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        delivery_proof: Some(build_create_delivery_proof_params(
                            CreateDeliveryProofParamsOverrides {
                                concrete: Some(*proof_concrete),
                                ..Default::default()
                            },
                        )),
                        kem: Some(*kem_selection),
                        random_source: os_source(),
                        ..Default::default()
                    }),
                    case_payload(CANONICAL, &asset_root(), &other_asset_root()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: carried entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`
///   of `vectors/cross_holder`, `vectors/rerandomization`,
///   `vectors/non_convertibility`, `vectors/cross_set`,
///   `vectors/statement_mutations`, and `vectors/replay`, restated by the
///   spec.
/// Arrange: every count `1`; each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, each
///   case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring `Keccak256V1`,
///   and each case of `DECLARED_ENCODING_SELECTIONS` declaring
///   `EthereumAbiV1`; the canonical bytes the bytes `create_encoding` returns
///   for the derivation context `build_derivation_context(Default::default())`
///   returns, under `DerivationContextDescription`, through
///   `build_mock_i_encoding_consumer`; for each set that derives an identity,
///   its parameter set replayed from its record's master, `u0`, and `u1`
///   uniforms through the received KEM's `setup` and the element
///   `derive_identity` returns for those bytes under it.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the `encode_scalar` of each replayed element's `scalar` equals the
///   `encode_scalar` of the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds from
///   `K::DECLARATION.identity_tag`, those bytes, and `UNIFORM_BYTES_LENGTH`;
///   the `encode_g1` of each replayed element's `element` equals that of the
///   identity element the set's record holds; and `is_valid` over
///   `non_convertibility`'s record credential returns `true`.
/// Boundary: the public call `vectors`, with the route the spec states for
///   the carried entry.
/// Mocked:  the encoding family's official consumer mock, which hands the
///   bytes the encoding returned to the block.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    // Arrange
    let description = build_derivation_context_description(Default::default());
    let mut encodings: Vec<(&encoding::CreateEncodingParams, Vec<u8>)> = Vec::new();
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        if encoding_selection.identifier == EncodingIdentifier::EthereumAbiV1 {
            let consumer = build_mock_i_encoding_consumer(
                &description,
                build_derivation_context(Default::default()),
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
            encodings.push((
                encoding_selection,
                outcome
                    .output
                    .encoded
                    .expect("the real encoder encodes the description's value")
                    .bytes,
            ));
        }
    }

    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let uniforms = |draws: &[u8], offset: usize| -> Secret<Vec<u8>> {
                Secret::try_new(SecretConstructorParams {
                    value: draws[offset * length..(offset + 1) * length].to_vec(),
                })
                .expect("a draw of the declared length is a valid uniform")
            };
            let replayed = |draws: &[u8]| {
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(identity.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(uniforms(draws, 0)),
                            u0_uniform: Some(uniforms(draws, 1)),
                            u1_uniform: Some(uniforms(draws, 2)),
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
                let components = kem
                    .identity_element_components(
                        build_identity_element_components_params(Default::default()),
                        build_identity_element_components_payload(
                            &derived.identity_element,
                            Default::default(),
                        ),
                    )
                    .expect("the identity element's components are infallible")
                    .components;
                (setup.parameter_set, derived.identity_element, components)
            };
            if let KemIdentity::Entitlement { canonical } = identity.identity {
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                });
                let expected_scalar =
                    <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        SampleUniformScalarPayload {
                            uniform: build_secret(
                                expected_uniform_bytes(&tag, canonical, length),
                                Default::default(),
                            ),
                        },
                    )
                    .expect("the expected uniform bytes sample")
                    .scalar
                    .expose()
                    .clone();
                let identity_element_bytes = |element: &K::IdentityElement| -> Vec<u8> {
                    encode_g1(
                        pairing,
                        &kem.identity_element_components(
                            build_identity_element_components_params(Default::default()),
                            build_identity_element_components_payload(element, Default::default()),
                        )
                        .expect("the identity element's components are infallible")
                        .components
                        .element,
                    )
                };
                let check = |record_draws: &[u8], element_bytes: Vec<u8>| {
                    let (_set, _element, components) = replayed(record_draws);
                    assert_eq!(
                        encode_scalar(pairing, &components.scalar),
                        encode_scalar(pairing, &expected_scalar)
                    );
                    assert_eq!(encode_g1(pairing, &components.element), element_bytes);
                };
                check(
                    &success.cross_holder.vectors[0].draws,
                    identity_element_bytes(&success.cross_holder.vectors[0].identity_element),
                );
                check(
                    &success.rerandomization.vectors[0].draws,
                    identity_element_bytes(&success.rerandomization.vectors[0].identity_element),
                );
                {
                    let (set, element, _components) =
                        replayed(&success.non_convertibility.vectors[0].draws);
                    assert!(
                        kem.is_valid(
                            build_is_valid_params(Default::default()),
                            build_is_valid_payload(
                                &set,
                                &element,
                                &success.non_convertibility.vectors[0].credential,
                                Default::default(),
                            ),
                        )
                        .expect("the credential check is infallible")
                        .is_valid
                    );
                }
                check(
                    &success.cross_set.vectors[0].draws[PIECE_GROUP_KEY_LENGTH..],
                    identity_element_bytes(&success.cross_set.vectors[0].first.identity_element),
                );
                check(
                    &success.cross_set.vectors[0].draws[PIECE_GROUP_KEY_LENGTH + 5 * length..],
                    identity_element_bytes(&success.cross_set.vectors[0].second.identity_element),
                );
                check(
                    &success.statement_mutations.vectors[0].draws,
                    encode_g1(
                        pairing,
                        &success.statement_mutations.vectors[0]
                            .mint_statement
                            .identity_element,
                    ),
                );
                check(
                    &success.statement_mutations.vectors[0].draws,
                    encode_g1(
                        pairing,
                        &success.statement_mutations.vectors[0]
                            .transfer_statement
                            .identity_element,
                    ),
                );
                check(
                    &success.replay.vectors[0].draws,
                    encode_g1(
                        pairing,
                        &success.replay.vectors[0]
                            .transfer_statement
                            .identity_element,
                    ),
                );
                check(
                    &success.replay.vectors[0].draws,
                    encode_g1(
                        pairing,
                        &success.replay.vectors[0].next_statement.identity_element,
                    ),
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
                for (encoding_selection, canonical) in &encodings {
                    let result = run_vectors_case(
                        &build_vectors_case_deps(Consumer, Default::default()),
                        build_vectors_case_params(VectorsCaseParamsOverrides {
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
                            kem: Some(*kem_selection),
                            random_source: os_source(),
                            ..Default::default()
                        }),
                        case_payload(canonical.as_slice(), &asset_root(), &other_asset_root()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
