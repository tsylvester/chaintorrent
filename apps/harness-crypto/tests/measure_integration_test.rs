#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use core::time::Duration;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{DECLARED_KEM_SELECTIONS, ICredentialKemAdapter};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_params,
};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryRelation, IDeliveryProofAdapter,
    MintPurpose, TransferPurpose, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    ConsumeMeasureCaseParams, ConsumeMeasureCasePayload, CreateVerifierParamsOverrides,
    IMeasureCaseConsumer, IVerifierAdapter, MeasureCaseParamsOverrides, MeasureErrorReturn,
    MeasurePayloadOverrides, MeasureReturn, MintErrorReturn, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VerifierConcrete, VerifyOnChainErrorReturn,
    build_create_verifier_params, build_measure_case_deps, build_measure_case_params,
    build_measure_case_payload, build_measure_deps, build_measure_params, build_measure_payload,
    measure, run_measure_case,
};

/// Contract: own entry `every_operation_is_timed_on_every_concrete` —
///   `measure` over a case with an iteration count of one, every adapter
///   real, the operating-system source real, and the verifier family's mock
///   concrete under `NoFailure`, returns `Ok` whose timings for `prove_mint`,
///   `prove_transfer`, `verify_mint`, `verify_transfer`, and `decapsulate`
///   are each greater than `Duration::ZERO`.
/// Arrange: each member of `PAIRING_CONCRETES`, the operating-system source,
///   and the verifier family's mock concrete under `NoFailure`, an iteration
///   count of one.
/// Act:     `measure`, once per member, in the case consumer's method.
/// Assert:  the result is `Ok` and each of the five timings is greater than
///   `Duration::ZERO`.
/// Boundary: the public call `measure`, invoked through the crate root over
///   the factories' concretes its route names — `create_random_source` for
///   the operating-system kind, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, `create_delivery_proof`, and `create_verifier`.
/// Mocked:  the verifier family's mock concrete, the outer edge, by
///   configuration; the operating-system generator and monotonic clock run
///   real.
#[test]
fn every_operation_is_timed_on_every_concrete() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            _params: ConsumeMeasureCaseParams,
            payload: ConsumeMeasureCasePayload<'_, P, H, E, F, K, A, D, V>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let verifier = payload.verifier;
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
            measure::<P, K, A, D, V>(
                &build_measure_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    verifier,
                    random,
                    Default::default(),
                ),
                build_measure_params(Default::default()),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let returned = run_measure_case(
            &build_measure_case_deps(Consumer, Default::default()),
            build_measure_case_params(MeasureCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    },
                )),
                ..Default::default()
            }),
            build_measure_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
        )
        .expect("the case runs");
        // Assert
        let success = returned.output.expect("the subject's calls succeed");
        let timings = success.measurement.timings;
        assert!(timings.prove_mint > Duration::ZERO);
        assert!(timings.prove_transfer > Duration::ZERO);
        assert!(timings.verify_mint > Duration::ZERO);
        assert!(timings.verify_transfer > Duration::ZERO);
        assert!(timings.decapsulate > Duration::ZERO);
    }
    // every_operation_is_timed_on_every_concrete
}

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole` and the
///   absorbed `delivery/mint` entry `a_refused_draw_reaches_the_caller_whole`
///   — `measure` with the source `create_random_source` returns for
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)` returns `Err(MeasureErrorReturn::Mint(
///   MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSource
///   Adapter)))`, the refusal reaching the caller inside the mint delivery's
///   own variant at its master uniform's draw, before any timing.
/// Arrange: each selection and scope of `DECLARED_KEM_SELECTIONS` over every
///   member of `PAIRING_CONCRETES`, the source `create_random_source` returns
///   for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)` and, as the counterfactual, the source for
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `measure`, once per source, in the case consumer's method.
/// Assert:  the result under the refusing source matches
///   `Err(MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter)))` and the result under
///   the operating-system source `is_ok()`.
/// Boundary: the public call `measure`, invoked through the crate root over
///   the factories' concretes its route names, the source selected through
///   the random-source params.
/// Mocked:  the randomness source, by the randomness family's mock concrete
///   selected by configuration, and the verifier family's mock concrete
///   under `NoFailure`; the operating-system generator and monotonic clock
///   run real in the counterfactual run.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            _params: ConsumeMeasureCaseParams,
            payload: ConsumeMeasureCasePayload<'_, P, H, E, F, K, A, D, V>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let verifier = payload.verifier;
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
            measure::<P, K, A, D, V>(
                &build_measure_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    verifier,
                    random,
                    Default::default(),
                ),
                build_measure_params(Default::default()),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let refused = run_measure_case(
                &build_measure_case_deps(Consumer, Default::default()),
                build_measure_case_params(MeasureCaseParamsOverrides {
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
                build_measure_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            )
            .expect("the case runs");
            let accepted = run_measure_case(
                &build_measure_case_deps(Consumer, Default::default()),
                build_measure_case_params(MeasureCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_measure_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            )
            .expect("the case runs");
            // Assert
            assert!(matches!(
                refused.output,
                Err(MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                )))
            ));
            assert!(accepted.output.is_ok());
        }
    }
    // a_draw_refusal_reaches_the_caller_whole
}

/// Contract: own entry
///   `a_refused_chain_read_reaches_the_caller_whole_with_the_relation_it_sub
///   mitted` and the absorbed `harness-crypto/verifier/factory` entry
///   `a_refused_read_returns_the_mock_concretes_own_error` — `measure` over a
///   case whose verifier is `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)`, selected by
///   configuration, returns `Err(MeasureErrorReturn::OnChain { relation:
///   DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })`, the mock
///   concrete's own error carried unchanged with the relation it concerned,
///   reached only after every timing was taken, and the transfer is not
///   submitted.
/// Arrange: every member of `PAIRING_CONCRETES`, the verifier
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)`
///   and, as the counterfactual, `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::NoFailure)`, each selected through the
///   verifier params.
/// Act:     `measure`, once per mode, in the case consumer's method.
/// Assert:  the result under the refusing mode matches `Err(
///   MeasureErrorReturn::OnChain { relation: DeliveryRelation::Mint, error:
///   VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })` and the
///   result under `NoFailure` `is_ok()`.
/// Boundary: the public call `measure`, invoked through the crate root over
///   the factories' concretes its route names, the verifier selected through
///   the verifier params.
/// Mocked:  the verifier family's mock concrete, selected by configuration;
///   the operating-system generator and monotonic clock run real.
#[test]
fn a_refused_chain_read_reaches_the_caller_whole_with_the_relation_it_submitted() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            _params: ConsumeMeasureCaseParams,
            payload: ConsumeMeasureCasePayload<'_, P, H, E, F, K, A, D, V>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let verifier = payload.verifier;
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
            measure::<P, K, A, D, V>(
                &build_measure_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    verifier,
                    random,
                    Default::default(),
                ),
                build_measure_params(Default::default()),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let refused = run_measure_case(
            &build_measure_case_deps(Consumer, Default::default()),
            build_measure_case_params(MeasureCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                verifier: Some(build_create_verifier_params(
                    CreateVerifierParamsOverrides {
                        concrete: Some(VerifierConcrete::Mock(
                            MockIVerifierAdapterFailureMode::RpcRefused,
                        )),
                        ..Default::default()
                    },
                )),
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    },
                )),
                ..Default::default()
            }),
            build_measure_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
        )
        .expect("the case runs");
        let accepted = run_measure_case(
            &build_measure_case_deps(Consumer, Default::default()),
            build_measure_case_params(MeasureCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                verifier: Some(build_create_verifier_params(
                    CreateVerifierParamsOverrides {
                        concrete: Some(VerifierConcrete::Mock(
                            MockIVerifierAdapterFailureMode::NoFailure,
                        )),
                        ..Default::default()
                    },
                )),
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    },
                )),
                ..Default::default()
            }),
            build_measure_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
        )
        .expect("the case runs");
        // Assert
        assert!(matches!(
            refused.output,
            Err(MeasureErrorReturn::OnChain {
                relation: DeliveryRelation::Mint,
                error: VerifyOnChainErrorReturn::Mock(
                    MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                )
            })
        ));
        assert!(accepted.output.is_ok());
    }
    // a_refused_chain_read_reaches_the_caller_whole_with_the_relation_it_submitted
}
