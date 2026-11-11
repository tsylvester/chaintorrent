#![allow(clippy::expect_used)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{
    CreateKeyAgreementParamsOverrides, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarParamsOverrides,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    build_hash_to_scalar_params, build_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::build_root;
use kem::{
    CreateKemParamsOverrides, DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope,
    SetupErrorReturn, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayloadOverrides, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, build_fill_bytes_payload,
    create_random_source,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::interface::{
    ConsumeProofVectorsCaseParams, ConsumeProofVectorsCasePayload, IProofVectorsCaseConsumer,
    ProofVectorsErrorReturn,
};
use super::mock::{
    ProofVectorsCaseParamsOverrides, ProofVectorsParamsOverrides, ProofVectorsPayloadOverrides,
    build_proof_vectors_case_deps, build_proof_vectors_case_params,
    build_proof_vectors_case_payload, build_proof_vectors_deps, build_proof_vectors_params,
    build_proof_vectors_payload, run_proof_vectors_case,
};
use super::proof_vectors;
use crate::delivery::provides::MintErrorReturn;

/// Contract: Mint delivery, the mint delivery's draw fails →
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::FillBytes(error)))`,
///   `error` unchanged.
/// Arrange: `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; the expected `error` the refusal of the independent
///   source's `fill_bytes`, built over the same kind.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result equals `Err(ProofVectorsErrorReturn::Mint(
///   MintErrorReturn::FillBytes(error)))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory.
#[test]
fn proof_vectors_return_the_draw_refusal_whole() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the refusal selection constructs the source");
            let error = independent
                .adapter
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .err()
                .expect("the selected mode refuses");
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let returned = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            assert_eq!(
                returned,
                Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::FillBytes(
                    error
                )))
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::Mock(
                            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                        )),
                    },
                )),
                ..Default::default()
            }),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_return_the_draw_refusal_whole
}

/// Contract: Mint delivery, its identity derivation fails →
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::DeriveIdentity(error)))`,
///   `error` unchanged.
/// Arrange: `scope` `IdentityScope::Entitlement` and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected sampling refusal the `Err` of
///   `hash_to_scalar` over `build_hash_to_scalar_params` and
///   `build_hash_to_scalar_payload` at their defaults on an adapter built
///   under the same mode.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result equals `Err(ProofVectorsErrorReturn::Mint(
///   MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(
///   error))))` whole, `error` the expected sampling refusal.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn proof_vectors_return_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let tag = build_domain_tag(Default::default());
            let independent = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the refusal selection constructs the adapter");
            let error = independent
                .adapter
                .hash_to_scalar(
                    build_hash_to_scalar_params(HashToScalarParamsOverrides { tag: Some(&tag) }),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the selected mode refuses");
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let returned = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            assert_eq!(
                returned,
                Err(ProofVectorsErrorReturn::Mint(
                    MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error))
                ))
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                hash_to_scalar: Some(build_create_hash_to_scalar_params(
                    CreateHashToScalarParamsOverrides {
                        concrete: Some(HashToScalarConcrete::Mock(
                            MockIHashToScalarAdapterFailureMode::SamplingRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(IdentityScope::Entitlement),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_return_the_hash_to_scalar_refusal_of_identity_derivation_whole
}

/// Contract: Mint delivery, its setup fails →
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::Setup(error)))`,
///   `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the expected `error` the `Err` of
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes` over
///   the independent source's first draw.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result equals `Err(ProofVectorsErrorReturn::Mint(
///   MintErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(error))))`
///   whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection, and the independent
///   source a second concrete of the same selection.
#[test]
fn proof_vectors_return_the_sampling_refusal_of_setup_whole() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the source");
            let uniform = independent
                .adapter
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .expect("the mock source fills")
                .bytes;
            let error = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .err()
            .expect("the selected scalar refuses the uniform");
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let returned = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            assert_eq!(
                returned,
                Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(error)
                )))
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_return_the_sampling_refusal_of_setup_whole
}

/// Contract: Mint delivery, the recipient's key generation fails →
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::GenerateKeys(error)))`,
///   `error` unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   NonceDrawRefused)`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result equals `Err(ProofVectorsErrorReturn::Mint(
///   MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(
///   MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused))))`
///   whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn proof_vectors_return_the_nonce_draw_refusal_of_key_generation_whole() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let returned = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            assert_eq!(
                returned,
                Err(ProofVectorsErrorReturn::Mint(
                    MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(
                        MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
                    ))
                ))
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            }),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_return_the_nonce_draw_refusal_of_key_generation_whole
}

/// Contract: Mint delivery, its wrap fails →
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::WrapTo(error)))`,
///   `error` unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   CoinDrawRefused)`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result equals `Err(ProofVectorsErrorReturn::Mint(
///   MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(
///   MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused))))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn proof_vectors_return_the_coin_draw_refusal_of_wrapping_whole() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let returned = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            assert_eq!(
                returned,
                Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::WrapTo(
                    WrapToErrorReturn::Mock(
                        MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused
                    )
                )))
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            }),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_return_the_coin_draw_refusal_of_wrapping_whole
}

/// Contract: Mint delivery, its proof fails →
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::ProveMint(error)))`,
///   `error` unchanged.
/// Arrange: `build_create_delivery_proof_params` overriding `concrete` with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::
///   NonceDrawRefused)`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result equals `Err(ProofVectorsErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))`
///   whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn proof_vectors_return_the_nonce_draw_refusal_of_the_mint_prover_whole() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let returned = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            assert_eq!(
                returned,
                Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::ProveMint(
                    ProveMintErrorReturn::Mock(
                        MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                    )
                )))
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                delivery_proof: Some(build_create_delivery_proof_params(
                    CreateDeliveryProofParamsOverrides {
                        concrete: Some(DeliveryProofConcrete::Mock(
                            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            }),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_return_the_nonce_draw_refusal_of_the_mint_prover_whole
}
