#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{Secret, SecretConstructorParams, build_secret};
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
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, KemIdentity, SetupErrorReturn, SetupParamsOverrides,
    SetupPayloadOverrides, build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
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
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, DeliveryProofConcrete,
    IDeliveryProofAdapter, MintPurpose, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, IReplayCaseConsumer, MintErrorReturn, NextSettlementField,
    ReplayCaseParamsOverrides, ReplayErrorReturn, ReplayParamsOverrides, ReplayPayloadOverrides,
    VectorCountConstructorParamsOverrides, build_replay_case_deps, build_replay_case_params,
    build_replay_case_payload, build_replay_deps, build_replay_params, build_replay_payload,
    build_vector_count, replay, run_replay_case,
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

fn base_params() -> ReplayCaseParamsOverrides {
    ReplayCaseParamsOverrides {
        random_source: os_source(),
        ..Default::default()
    }
}

fn selections(
    pairing_concrete: PairingConcrete,
    h2s_selection: &hash_to_scalar::CreateHashToScalarParams,
    encoding_selection: &encoding::CreateEncodingParams,
    forms_concrete: &chain::ChainFormsConcrete,
    kem_selection: &kem::CreateKemParams,
    key_agreement_selection: &envelope::CreateKeyAgreementParams,
    proof_concrete: &proof::DeliveryProofConcrete,
) -> harness_crypto::ReplayCaseParams {
    build_replay_case_params(ReplayCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        })),
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
        random_source: os_source(),
    })
}

fn all_cases(
    mut params: impl FnMut(
        PairingConcrete,
        &hash_to_scalar::CreateHashToScalarParams,
        &encoding::CreateEncodingParams,
        &chain::ChainFormsConcrete,
        &kem::CreateKemParams,
        &envelope::CreateKeyAgreementParams,
        &proof::DeliveryProofConcrete,
    ),
) {
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                params(
                                    *pairing_concrete,
                                    h2s_selection,
                                    encoding_selection,
                                    forms_concrete,
                                    kem_selection,
                                    key_agreement_selection,
                                    proof_concrete,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Contract: own entry `each_settlement_verifies_and_the_replayed_proof_is_refused`,
///   as an outside caller observes it.
/// Arrange: `count` of `2`; every case, the canonical bytes `b"pkg@1.0.0#1"`
///   and the root `build_root` builds from `[0x44; 32]`.
/// Act:     `replay`.
/// Assert:  `vectors` holds `2` records; for each, `transfer_is_valid` and
///   `next_is_valid` are `true` and `replay_is_valid` is `false`.
/// Boundary: the public call `replay`, with the route `create_random_source`,
///   `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_chain_forms`, `create_kem`, `create_key_agreement`, and
///   `create_delivery_proof`, each selected concrete through its `try_new`,
///   `replay`, and every adapter method it calls.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn each_settlement_verifies_and_the_replayed_proof_is_refused() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.transfer_is_valid);
                assert!(record.next_is_valid);
                assert!(!record.replay_is_valid);
            }
        }
    }
    all_cases(
        |pairing_concrete,
         h2s_selection,
         encoding_selection,
         forms_concrete,
         kem_selection,
         key_agreement_selection,
         proof_concrete| {
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    pairing_concrete,
                    h2s_selection,
                    encoding_selection,
                    forms_concrete,
                    kem_selection,
                    key_agreement_selection,
                    proof_concrete,
                ),
                build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        },
    );
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`.
/// Act:     `replay`.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `12` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and no
///   two records share `draws`.
/// Boundary: the public call `replay`, with the route of the block above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(3),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 3);
            for record in &success.vectors {
                assert_eq!(
                    record.draws.len(),
                    12 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                );
            }
            assert_ne!(success.vectors[0].draws, success.vectors[1].draws);
            assert_ne!(success.vectors[0].draws, success.vectors[2].draws);
            assert_ne!(success.vectors[1].draws, success.vectors[2].draws);
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    *pairing_concrete,
                    &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                    &DECLARED_ENCODING_SELECTIONS[0],
                    &CHAIN_FORMS_CONCRETES[0],
                    kem_selection,
                    &DECLARED_KEY_AGREEMENT_SELECTIONS[0],
                    &DELIVERY_PROOF_CONCRETES[0],
                ),
                build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: own entry
///   `a_context_that_is_not_the_next_settlement_is_refused_whole_with_its_field`.
/// Arrange: `count` of `1`; every member of `CHAIN_FORMS_CONCRETES` over every
///   member of `PAIRING_CONCRETES` with each selection and scope of
///   `DECLARED_KEM_SELECTIONS`; next contexts over the private surface's
///   next-context overrides with, in turn, `target_entitlement` overridden with
///   a clone of the transfer context's `source_entitlement`, `old_interval`
///   with a clone of the transfer context's `old_interval`, and `seller` with
///   a clone of the transfer context's `seller`, and, as the counterfactual,
///   the private surface's next context.
/// Act:     `replay`, once per next context.
/// Assert:  the results equal the whole `Err` of each named
///   `NextSettlementField`, and the result over the private surface's next
///   context `is_ok()`.
/// Boundary: the public call `replay`, with the route of the blocks above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_context_that_is_not_the_next_settlement_is_refused_whole_with_its_field() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_overrides = |field: NextSettlementField| -> DeliveryContextOverrides<F> {
                match field {
                    NextSettlementField::TargetEntitlement => DeliveryContextOverrides {
                        chain: Some(transfer_context.chain.clone()),
                        entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                        asset_root: Some(transfer_context.asset_root),
                        parameter_set_identifier: Some(
                            transfer_context.parameter_set_identifier.clone(),
                        ),
                        source_entitlement: Some(transfer_context.source_entitlement.clone()),
                        target_entitlement: Some(transfer_context.source_entitlement.clone()),
                        old_interval: Some(transfer_context.new_interval.clone()),
                        seller: Some(transfer_context.buyer.clone()),
                        ..Default::default()
                    },
                    NextSettlementField::OldInterval => DeliveryContextOverrides {
                        chain: Some(transfer_context.chain.clone()),
                        entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                        asset_root: Some(transfer_context.asset_root),
                        parameter_set_identifier: Some(
                            transfer_context.parameter_set_identifier.clone(),
                        ),
                        source_entitlement: Some(transfer_context.source_entitlement.clone()),
                        target_entitlement: Some(transfer_context.target_entitlement.clone()),
                        old_interval: Some(transfer_context.old_interval.clone()),
                        seller: Some(transfer_context.buyer.clone()),
                        ..Default::default()
                    },
                    _ => DeliveryContextOverrides {
                        chain: Some(transfer_context.chain.clone()),
                        entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                        asset_root: Some(transfer_context.asset_root),
                        parameter_set_identifier: Some(
                            transfer_context.parameter_set_identifier.clone(),
                        ),
                        source_entitlement: Some(transfer_context.source_entitlement.clone()),
                        target_entitlement: Some(transfer_context.target_entitlement.clone()),
                        old_interval: Some(transfer_context.new_interval.clone()),
                        seller: Some(transfer_context.seller.clone()),
                        ..Default::default()
                    },
                }
            };
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let build_next = |field: NextSettlementField| {
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    next_overrides(field),
                )
            };
            let deps = build_replay_deps(
                kem,
                key_agreement,
                delivery_proof,
                random,
                Default::default(),
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act + Assert
            for field in [
                NextSettlementField::TargetEntitlement,
                NextSettlementField::OldInterval,
                NextSettlementField::Seller,
            ] {
                let refused_next = build_next(field);
                let returned = replay(
                    &deps,
                    params,
                    build_replay_payload(
                        &mint_context,
                        &transfer_context,
                        &refused_next,
                        ReplayPayloadOverrides {
                            identity: Some(identity),
                        },
                    ),
                );
                assert_eq!(
                    returned.err(),
                    Some(ReplayErrorReturn::NotTheNextSettlement { field })
                );
            }
            let returned = replay(
                &deps,
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            assert!(returned.is_ok());
        }
    }
    for forms_concrete in CHAIN_FORMS_CONCRETES {
        for pairing_concrete in PAIRING_CONCRETES {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                let result = run_replay_case(
                    &build_replay_case_deps(Consumer, Default::default()),
                    selections(
                        *pairing_concrete,
                        &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                        &DECLARED_ENCODING_SELECTIONS[0],
                        forms_concrete,
                        kem_selection,
                        &DECLARED_KEY_AGREEMENT_SELECTIONS[0],
                        &DELIVERY_PROOF_CONCRETES[0],
                    ),
                    build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole`, and the
///   absorbed `delivery/mint` entry `a_refused_draw_reaches_the_caller_whole`.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`; the
///   source `create_random_source` returns for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`,
///   and, as the counterfactual, the source for
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `replay`, once per source.
/// Assert:  the result under the refusing source equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)))`
///   whole, and the result under the operating-system source `is_ok()`.
/// Boundary: the public call `replay`, with the route of the blocks above.
/// Mocked:  the randomness source, the outer edge, by the randomness family's
///   mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(ReplayErrorReturn::Mint(MintErrorReturn::FillBytes(
                        FillBytesErrorReturn::MockIRandomSourceAdapter,
                    )))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    let refusing = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for (source, expects_refusal) in [(Some(&refusing), true), (None, false)] {
                let result = run_replay_case(
                    &build_replay_case_deps(Consumer { expects_refusal }, Default::default()),
                    build_replay_case_params(ReplayCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        random_source: source
                            .map(|params| random::CreateRandomSourceParams { kind: params.kind })
                            .or_else(os_source),
                        ..base_params()
                    }),
                    build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_setup_sampling_refusal_reaches_the_caller_whole`.
/// Arrange: `count` of `1`; `create_pairing` over
///   `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`,
///   with each selection and scope of `DECLARED_KEM_SELECTIONS`, and, as the
///   counterfactual, the pairing's default selection; the expected `error` the
///   literal `SampleUniformScalarErrorReturn::WrongLength { expected: 16,
///   actual: 16 }`.
/// Act:     `replay`, once per pairing selection.
/// Assert:  the result under the refusing selection equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 16 }))))`
///   whole, and the result under the default selection `is_ok()`.
/// Boundary: the public call `replay`, with the route of the blocks above,
///   through `mint` and `setup`.
/// Mocked:  the pairing family's mock concrete, selected by configuration; the
///   operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(ReplayErrorReturn::Mint(MintErrorReturn::Setup(
                        SetupErrorReturn::MasterScalarSampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: 16,
                                actual: 16,
                            },
                        ),
                    )))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for expects_refusal in [true, false] {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let pairing = if expects_refusal {
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                })
            } else {
                build_create_pairing_params(Default::default())
            };
            let result = run_replay_case(
                &build_replay_case_deps(Consumer { expects_refusal }, Default::default()),
                build_replay_case_params(ReplayCaseParamsOverrides {
                    pairing: Some(pairing),
                    kem: Some(*kem_selection),
                    ..base_params()
                }),
                build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
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
/// Act:     `replay`, once, the case payload's canonical bytes those bytes.
/// Assert:  `encode_scalar` of the `scalar` of the identity element of the
///   record's transfer statement, read through `identity_element_components`
///   after the KEM's identity mapping inside the mint delivery, equals
///   `encode_scalar` of the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds from
///   `K::DECLARATION.identity_tag`, those bytes, and `UNIFORM_BYTES_LENGTH`.
/// Boundary: the public call `replay`, with the route the spec states for the
///   entry.
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

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let uniforms: Vec<Secret<Vec<u8>>> = record
                .draws
                .chunks_exact(length)
                .map(|chunk| {
                    Secret::try_new(SecretConstructorParams {
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
            let element = pairing
                .encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: record.transfer_statement.identity_element.clone(),
                    },
                )
                .expect("the first-group encoding is infallible");
            let replayed_element = pairing
                .encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: identity_components.element.clone(),
                    },
                )
                .expect("the first-group encoding is infallible");
            assert_eq!(
                element.bytes.as_ref().to_vec(),
                replayed_element.bytes.as_ref().to_vec()
            );
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
                    let result = run_replay_case(
                        &build_replay_case_deps(Consumer, Default::default()),
                        build_replay_case_params(ReplayCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing_concrete),
                                    ..Default::default()
                                },
                            )),
                            hash_to_scalar: Some(*h2s_selection),
                            kem: Some(*kem_selection),
                            ..base_params()
                        }),
                        build_replay_case_payload(
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
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, with
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`,
///   and, as the counterfactual, each member of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
///   actual: 0 })`.
/// Act:     `replay`, once per hash-to-scalar selection.
/// Assert:  the result under the refusing selection equals the whole `Err`
///   under each scope, and under each declared selection `is_ok()`.
/// Boundary: the public call `replay`, with the route of the blocks above.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                let error = HashToScalarErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        actual: 0,
                    },
                );
                let expected = match identity.identity {
                    KemIdentity::Entitlement { .. } => {
                        ReplayErrorReturn::Mint(MintErrorReturn::DeriveIdentity(
                            DeriveIdentityErrorReturn::HashToScalar(error),
                        ))
                    }
                    KemIdentity::Asset { .. } => ReplayErrorReturn::Mint(MintErrorReturn::Setup(
                        SetupErrorReturn::HashToScalar(error),
                    )),
                };
                assert_eq!(returned.err(), Some(expected));
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    let refusing = build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
        concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        ..Default::default()
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for (h2s_params, expects_refusal) in [(&refusing, true)].into_iter().chain(
                DECLARED_HASH_TO_SCALAR_SELECTIONS
                    .iter()
                    .map(|selection| (selection, false)),
            ) {
                let result = run_replay_case(
                    &build_replay_case_deps(Consumer { expects_refusal }, Default::default()),
                    build_replay_case_params(ReplayCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(*h2s_params),
                        kem: Some(*kem_selection),
                        ..base_params()
                    }),
                    build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry
///   `a_key_generation_nonce_draw_refusal_reaches_the_caller_whole`, as an
///   outside caller observes it.
/// Arrange: as the private integration test's block of the same name, through
///   the crate's public surface.
/// Act:     `replay`, once per key-agreement selection.
/// Assert:  the result under the refusing selection equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: the public call `replay`, with the route of the blocks above.
/// Mocked:  the key-agreement family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_key_generation_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(ReplayErrorReturn::Mint(MintErrorReturn::GenerateKeys(
                        GenerateKeysErrorReturn::Mock(
                            MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                        ),
                    )))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    let refusing = build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
        )),
        ..Default::default()
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for (key_agreement_params, expects_refusal) in [(&refusing, true)].into_iter().chain(
                DECLARED_KEY_AGREEMENT_SELECTIONS
                    .iter()
                    .map(|selection| (selection, false)),
            ) {
                let result = run_replay_case(
                    &build_replay_case_deps(Consumer { expects_refusal }, Default::default()),
                    build_replay_case_params(ReplayCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        key_agreement: Some(*key_agreement_params),
                        ..base_params()
                    }),
                    build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_wrap_coin_draw_refusal_reaches_the_caller_whole`, as
///   an outside caller observes it.
/// Arrange: as the private integration test's block of the same name, through
///   the crate's public surface.
/// Act:     `replay`, once per key-agreement selection.
/// Assert:  the result under the refusing selection equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: the public call `replay`, with the route of the blocks above.
/// Mocked:  the key-agreement family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_wrap_coin_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(ReplayErrorReturn::Mint(MintErrorReturn::WrapTo(
                        WrapToErrorReturn::Mock(
                            MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused,
                        ),
                    )))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    let refusing = build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
        )),
        ..Default::default()
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for (key_agreement_params, expects_refusal) in [(&refusing, true)].into_iter().chain(
                DECLARED_KEY_AGREEMENT_SELECTIONS
                    .iter()
                    .map(|selection| (selection, false)),
            ) {
                let result = run_replay_case(
                    &build_replay_case_deps(Consumer { expects_refusal }, Default::default()),
                    build_replay_case_params(ReplayCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        key_agreement: Some(*key_agreement_params),
                        ..base_params()
                    }),
                    build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: own entry `a_nonce_draw_refusal_reaches_the_caller_whole`, as an
///   outside caller observes it.
/// Arrange: as the private integration test's block of the same name, through
///   the crate's public surface.
/// Act:     `replay`, once per delivery-proof selection.
/// Assert:  the result under the refusing selection equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: the public call `replay`, with the route of the blocks above.
/// Mocked:  the delivery-proof family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(ReplayErrorReturn::Mint(MintErrorReturn::ProveMint(
                        ProveMintErrorReturn::Mock(
                            MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                        ),
                    )))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for (proof_concrete, expects_refusal) in [(
                DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NonceDrawRefused),
                true,
            )]
            .into_iter()
            .chain(
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .map(|concrete| (*concrete, false)),
            ) {
                let result = run_replay_case(
                    &build_replay_case_deps(Consumer { expects_refusal }, Default::default()),
                    build_replay_case_params(ReplayCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        delivery_proof: Some(build_create_delivery_proof_params(
                            CreateDeliveryProofParamsOverrides {
                                concrete: Some(proof_concrete),
                                ..Default::default()
                            },
                        )),
                        ..base_params()
                    }),
                    build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}
