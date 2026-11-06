#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use core::num::NonZeroU32;
use core::time::Duration;
use domain::{Secret, SecretConstructorParams};
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, ICredentialKemAdapter, SetupErrorReturn,
    SetupScope,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, VerifierGroupArithmetic,
    build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryProofConcrete, DeliveryRelation, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, CreateVerifierParams, CreateVerifierParamsOverrides, CreateVerifierReturn,
    CreateVerifierSuccessReturn, IMeasureCaseConsumer, IVerifierAdapter, L1DataFee,
    MeasureCaseParamsOverrides, MeasureErrorReturn, MintErrorReturn,
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifierConsumer, VerifierConsumerConstructorParamsOverrides,
    VerifierConsumerReturn, VerifyCost, VerifyOnChainErrorReturn, build_create_verifier_deps,
    build_create_verifier_params, build_create_verifier_payload, build_measure_case_deps,
    build_measure_case_params, build_measure_case_payload, build_verifier_consumer,
    create_verifier, run_measure_case,
};

/// Contract: own entry
///   `the_iteration_count_the_consumer_carries_is_the_count_the_measurement_
///   reports`, as an outside caller observes it — `create_verifier` over the
///   verifier family's mock concrete under `NoFailure` and a `VerifierConsumer`
///   constructed through `VerifierConsumer::try_new` from
///   `VerifierConsumerConstructorParams` with an iteration count other than
///   the builder's default → `Ok` whose output is `Ok(MeasureSuccessReturn)`
///   with `measurement.iterations` equal to the count the consumer was
///   constructed with.
/// Arrange: an iteration count of three and an iteration count of one, over
///   every member of `PAIRING_CONCRETES` and each scope of
///   `DECLARED_KEM_SELECTIONS`, the operating-system source.
/// Act:     `create_verifier` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  the output is `Ok(CreateVerifierSuccessReturn { output:
///   Ok(MeasureSuccessReturn { measurement }) })` and
///   `measurement.iterations.get()` equals the count.
/// Boundary: the public calls `VerifierConsumer::try_new` and `create_verifier`
///   through the crate root, inside the consumer the measure module's
///   `run_measure_case` hands it under the `mocks` feature, over the route
///   `run_measure_case`, the factories it composes, `create_verifier`, the
///   mock concrete's `try_new`, `VerifierConsumer::try_new`,
///   `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete, the operating system's
///   generator and monotonic clock running real.
#[test]
fn the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports() {
    // Arrange
    struct Consumer {
        iterations: NonZeroU32,
    }

    impl IMeasureCaseConsumer for Consumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                VerifierConsumerConstructorParamsOverrides {
                    iterations: Some(self.iterations),
                },
            );
            create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(Default::default()),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for count in [3u32, 1] {
                let iterations = NonZeroU32::new(count).expect("the count is nonzero");
                // Act
                let returned = run_measure_case(
                    &build_measure_case_deps(Consumer { iterations }, Default::default()),
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
                let success = returned
                    .output
                    .expect("the factory admits the mock concrete and runs the consumer")
                    .output
                    .expect("measure admits the case");
                assert_eq!(success.measurement.iterations, iterations);
            }
        }
    }
    // the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports
}

/// Contract: own entry
///   `the_measurement_measure_returns_reaches_the_caller_of_create_verifier_
///   whole`, as an outside caller observes it — `create_verifier` over the
///   verifier family's mock concrete under `NoFailure` and a `VerifierConsumer`
///   constructed with an iteration count of one → `Ok` whose output is
///   `Ok(MeasureSuccessReturn)` whose timings are each greater than
///   `Duration::ZERO`, whose sizes equal the lengths computed in Arrange
///   through the received pairing's encoders, and whose `costs.mint` and
///   `costs.transfer` each equal the literal cost the verifier family's mock
///   concrete returns.
/// Arrange: an iteration count of one, each identity scope, and the expected
///   sizes computed through the received pairing's encoders over its
///   generators and a scalar its sampling bound samples and the expected cost
///   the literal gas `100_000` and fee `L1DataFee::Charged { wei: 1_000 }`,
///   over every member of `PAIRING_CONCRETES` crossed with each scope of
///   `DECLARED_KEM_SELECTIONS`, the operating-system source.
/// Act:     `create_verifier` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  in place inside the case consumer — the five timings are each
///   greater than `Duration::ZERO`, the four sizes equal the expected sizes,
///   and the two costs equal the expected cost; the block asserts the output
///   is `Ok` holding `Ok`.
/// Boundary: the public calls `VerifierConsumer::try_new` and `create_verifier`
///   through the crate root, over the route `run_measure_case`, the factories
///   it composes, `create_verifier`, the mock concrete's `try_new`,
///   `VerifierConsumer::try_new`, `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete, the operating system's
///   generator and monotonic clock running real.
#[test]
fn the_measurement_measure_returns_reaches_the_caller_of_create_verifier_whole() {
    // Arrange
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                VerifierConsumerConstructorParamsOverrides {
                    iterations: NonZeroU32::new(1),
                },
            );
            let expected_cost = VerifyCost {
                l2_execution_gas: 100_000,
                l1_data_fee: L1DataFee::Charged { wei: 1_000 },
            };
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(g1_encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: g1.point.clone(),
                },
            );
            let Ok(g2_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: g2.point.clone(),
                },
            );
            let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                value: vec![0x07; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
            });
            let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("a uniform of the declared length samples");
            let Ok(scalar_encoded) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            let g1_length = g1_encoded.bytes.as_ref().len();
            let g2_length = g2_encoded.bytes.as_ref().len();
            let scalar_length = scalar_encoded.bytes.expose().as_ref().len();
            let expected_capsule = match identity.setup_scope {
                SetupScope::Entitlement => g2_length + 2 * g1_length,
                SetupScope::Asset { .. } => g2_length + g1_length,
            };
            let expected_envelope = 2 * g1_length + 2 * g2_length;
            let first_messages = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::FirstGroupOnly => 3 * g2_length,
                VerifierGroupArithmetic::BothGroups => 0,
            };
            let expected_mint = 5 * scalar_length + first_messages;
            let expected_transfer = 6 * scalar_length + first_messages;
            let returned = create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(Default::default()),
                build_create_verifier_payload(Default::default()),
            );
            let success = returned
                .as_ref()
                .expect("the factory admits the mock concrete and runs the consumer")
                .output
                .as_ref()
                .expect("measure admits the case");
            assert!(success.measurement.timings.prove_mint > Duration::ZERO);
            assert!(success.measurement.timings.prove_transfer > Duration::ZERO);
            assert!(success.measurement.timings.verify_mint > Duration::ZERO);
            assert!(success.measurement.timings.verify_transfer > Duration::ZERO);
            assert!(success.measurement.timings.decapsulate > Duration::ZERO);
            assert_eq!(success.measurement.sizes.capsule, expected_capsule);
            assert_eq!(success.measurement.sizes.envelope, expected_envelope);
            assert_eq!(success.measurement.sizes.mint_proof, expected_mint);
            assert_eq!(success.measurement.sizes.transfer_proof, expected_transfer);
            assert_eq!(success.measurement.costs.mint, expected_cost);
            assert_eq!(success.measurement.costs.transfer, expected_cost);
            returned
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_measure_case(
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
                returned.output,
                Ok(CreateVerifierSuccessReturn { output: Ok(_) })
            ));
        }
    }
    // the_measurement_measure_returns_reaches_the_caller_of_create_verifier_whole
}

/// Contract: own entry
///   `a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_
///   consumer`, as an outside caller observes it — `create_verifier` over the
///   verifier family's mock concrete under `NoFailure` and a `VerifierConsumer`
///   constructed with the source `create_random_source` returns for
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, selected through the case params → `Ok` whose output
///   is `Err(MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter)))`.
/// Arrange: each scope of `DECLARED_KEM_SELECTIONS` over every member of
///   `PAIRING_CONCRETES`, the refusing source selected through the
///   random-source params and the operating-system source as the
///   counterfactual.
/// Act:     `create_verifier` through the crate root, once per source, inside
///   the case consumer.
/// Assert:  the output under the refusing source matches
///   `Ok(CreateVerifierSuccessReturn { output: Err(MeasureErrorReturn::Mint(
///   MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSource
///   Adapter))) })` and the output under the operating-system source is `Ok`
///   holding `Ok`.
/// Boundary: the public calls `VerifierConsumer::try_new` and `create_verifier`
///   through the crate root, over the route `run_measure_case`, the factories
///   it composes, `create_verifier`, the mock concrete's `try_new`,
///   `VerifierConsumer::try_new`, `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the randomness family's mock concrete, selected by configuration;
///   the operating system's generator and monotonic clock run real in the
///   counterfactual run.
#[test]
fn a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_consumer() {
    // Arrange
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                Default::default(),
            );
            create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(Default::default()),
                build_create_verifier_payload(Default::default()),
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
                Ok(CreateVerifierSuccessReturn {
                    output: Err(MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
                        FillBytesErrorReturn::MockIRandomSourceAdapter
                    )))
                })
            ));
            assert!(matches!(
                accepted.output,
                Ok(CreateVerifierSuccessReturn { output: Ok(_) })
            ));
        }
    }
    // a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_consumer
}

/// Contract: own entry
///   `a_refused_chain_read_reaches_the_caller_of_create_verifier_whole_with_
///   the_relation_it_submitted`, as an outside caller observes it —
///   `create_verifier` over `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)`, selected through the
///   verifier params, and a `VerifierConsumer` constructed as above → `Ok`
///   whose output is `Err(MeasureErrorReturn::OnChain { relation:
///   DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })`.
/// Arrange: `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::
///   RpcRefused)` selected through `build_create_verifier_params`, then
///   `NoFailure` as the counterfactual, over every member of
///   `PAIRING_CONCRETES` and each scope of `DECLARED_KEM_SELECTIONS`, the
///   operating-system source.
/// Act:     `create_verifier` through the crate root, once per mode, inside
///   the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateVerifierSuccessReturn { output: Err(MeasureErrorReturn::
///   OnChain { relation: DeliveryRelation::Mint, error:
///   VerifyOnChainErrorReturn::Mock(MockIVerifierAdapterVerifyOnChainError
///   Return::RpcRefused) }) })` and the output under `NoFailure` is `Ok`
///   holding `Ok`.
/// Boundary: the public calls `VerifierConsumer::try_new` and `create_verifier`
///   through the crate root, over the route `run_measure_case`, the factories
///   it composes, `create_verifier`, the mock concrete's `try_new`,
///   `VerifierConsumer::try_new`, `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete under its failure mode,
///   selected by configuration.
#[test]
fn a_refused_chain_read_reaches_the_caller_of_create_verifier_whole_with_the_relation_it_submitted()
{
    // Arrange
    struct Consumer {
        params: CreateVerifierParams,
    }

    impl IMeasureCaseConsumer for Consumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                Default::default(),
            );
            create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                self.params.clone(),
                build_create_verifier_payload(Default::default()),
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
                &build_measure_case_deps(
                    Consumer {
                        params: build_create_verifier_params(CreateVerifierParamsOverrides {
                            concrete: Some(VerifierConcrete::Mock(
                                MockIVerifierAdapterFailureMode::RpcRefused,
                            )),
                            ..Default::default()
                        }),
                    },
                    Default::default(),
                ),
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
            let accepted = run_measure_case(
                &build_measure_case_deps(
                    Consumer {
                        params: build_create_verifier_params(Default::default()),
                    },
                    Default::default(),
                ),
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
                Ok(CreateVerifierSuccessReturn {
                    output: Err(MeasureErrorReturn::OnChain {
                        relation: DeliveryRelation::Mint,
                        error: VerifyOnChainErrorReturn::Mock(
                            MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                        ),
                    })
                })
            ));
            assert!(matches!(
                accepted.output,
                Ok(CreateVerifierSuccessReturn { output: Ok(_) })
            ));
        }
    }
    // a_refused_chain_read_reaches_the_caller_of_create_verifier_whole_with_the_relation_it_submitted
}

/// Contract: carried entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`
///   of `harness-crypto/measure`, as an outside caller observes it through
///   this consumer — `create_verifier` over a case whose hash-to-scalar
///   adapter is the family's mock concrete under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused`, selected through
///   the case params → `Ok` whose output is `Err(MeasureErrorReturn::Mint(
///   MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(
///   error))))` under the entitlement scope and `Err(MeasureErrorReturn::
///   Mint(MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(error))))`
///   under the asset scope, `error` `HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual: 0 })`
///   with `expected` the scalar's `UNIFORM_BYTES_LENGTH`, unconverted.
/// Arrange: the hash-to-scalar selection through the case params, each scope
///   of `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   the operating-system source.
/// Act:     `create_verifier` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  in place inside the case consumer — the output matches the
///   scope's `Ok(CreateVerifierSuccessReturn { output: Err(...) })` with
///   `expected` `UNIFORM_BYTES_LENGTH` and `actual` `0`; the block asserts the
///   output is `Ok` holding `Err(MeasureErrorReturn::Mint(_))`.
/// Boundary: the public call `create_verifier` with this consumer through the
///   crate root and the route the disposition states — `run_measure_case`,
///   the factories it composes, `create_verifier`, `VerifierConsumer::try_new`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                Default::default(),
            );
            let returned = create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(Default::default()),
                build_create_verifier_payload(Default::default()),
            );
            match identity.setup_scope {
                SetupScope::Entitlement => {
                    assert!(matches!(
                        &returned,
                        Ok(CreateVerifierSuccessReturn {
                            output: Err(MeasureErrorReturn::Mint(
                                MintErrorReturn::DeriveIdentity(
                                    DeriveIdentityErrorReturn::HashToScalar(
                                        HashToScalarErrorReturn::Sampling(
                                            SampleUniformScalarErrorReturn::WrongLength {
                                                expected,
                                                actual: 0,
                                            }
                                        )
                                    )
                                )
                            ))
                        }) if *expected
                            == <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                    ));
                }
                SetupScope::Asset { .. } => {
                    assert!(matches!(
                        &returned,
                        Ok(CreateVerifierSuccessReturn {
                            output: Err(MeasureErrorReturn::Mint(MintErrorReturn::Setup(
                                SetupErrorReturn::HashToScalar(
                                    HashToScalarErrorReturn::Sampling(
                                        SampleUniformScalarErrorReturn::WrongLength {
                                            expected,
                                            actual: 0,
                                        }
                                    )
                                )
                            )))
                        }) if *expected
                            == <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                    ));
                }
            }
            returned
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_measure_case(
                &build_measure_case_deps(Consumer, Default::default()),
                build_measure_case_params(MeasureCaseParamsOverrides {
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
                returned.output,
                Ok(CreateVerifierSuccessReturn {
                    output: Err(MeasureErrorReturn::Mint(_))
                })
            ));
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: carried entry `a_nonce_draw_refusal_reaches_the_caller_whole` of
///   `harness-crypto/measure`, as an outside caller observes it through this
///   consumer — `create_verifier` over a case whose delivery proof is the
///   family's mock concrete under `MockIDeliveryProofAdapterFailureMode::
///   NonceDrawRefused`, selected through the case params → `Ok` whose output
///   is `Err(MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
///   ProveMintErrorReturn::Mock(MockIDeliveryProofAdapterProveMintError
///   Return::NonceDrawRefused))))`, unconverted.
/// Arrange: the delivery-proof selection through the case params, each scope
///   of `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   the operating-system source.
/// Act:     `create_verifier` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  the output matches `Ok(CreateVerifierSuccessReturn { output:
///   Err(MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
///   ProveMintErrorReturn::Mock(MockIDeliveryProofAdapterProveMintError
///   Return::NonceDrawRefused)))) })`, the pattern naming every field.
/// Boundary: the public call `create_verifier` with this consumer through the
///   crate root and the route the disposition states — `run_measure_case`,
///   the factories it composes, `create_verifier`, `VerifierConsumer::try_new`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the delivery-proof family's mock concrete, selected by
///   configuration.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                Default::default(),
            );
            create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(Default::default()),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_measure_case(
                &build_measure_case_deps(Consumer, Default::default()),
                build_measure_case_params(MeasureCaseParamsOverrides {
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
                returned.output,
                Ok(CreateVerifierSuccessReturn {
                    output: Err(MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
                        ProveMintErrorReturn::Mock(
                            MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                        )
                    )))
                })
            ));
        }
    }
    // a_nonce_draw_refusal_reaches_the_caller_whole
}

// Public surface enumeration — every entry of the spec's public surface and
// the block that proves it:
//   the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports
//     → the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports
//   the_measurement_measure_returns_reaches_the_caller_of_create_verifier_whole
//     → the_measurement_measure_returns_reaches_the_caller_of_create_verifier_whole
//   a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_consumer
//     → a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_consumer
//   a_refused_chain_read_reaches_the_caller_of_create_verifier_whole_with_the_relation_it_submitted
//     → a_refused_chain_read_reaches_the_caller_of_create_verifier_whole_with_the_relation_it_submitted
//   a_hash_to_scalar_refusal_reaches_the_caller_whole (carried from `harness-crypto/measure`)
//     → a_hash_to_scalar_refusal_reaches_the_caller_whole
//   a_nonce_draw_refusal_reaches_the_caller_whole (carried from `harness-crypto/measure`)
//     → a_nonce_draw_refusal_reaches_the_caller_whole
