#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use core::num::NonZeroU32;
use core::time::Duration;
use domain::{Secret, SecretConstructorParams};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter,
};
use envelope::{DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier, IHashToScalarAdapter,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{DECLARED_KEM_SELECTIONS, ICredentialKemAdapter, SetupScope};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryRelation, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params,
};

use super::interface::{VerifierConsumer, VerifierConsumerReturn};
use super::mock::{VerifierConsumerConstructorParamsOverrides, build_verifier_consumer};
use crate::delivery::provides::MintErrorReturn;
use crate::measure::provides::{
    ConsumeMeasureCaseParams, ConsumeMeasureCasePayload, IMeasureCaseConsumer,
    MeasureCaseParamsOverrides, MeasureErrorReturn, build_measure_case_deps,
    build_measure_case_params, build_measure_case_payload, run_measure_case,
};
use crate::verifier::provides::{
    CreateVerifierParams, CreateVerifierParamsOverrides, CreateVerifierReturn,
    CreateVerifierSuccessReturn, IVerifierAdapter, L1DataFee, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VerifierConcrete, VerifyCost,
    VerifyOnChainErrorReturn, build_create_verifier_deps, build_create_verifier_params,
    build_create_verifier_payload, create_verifier,
};

/// Contract: own entry
///   `the_iteration_count_the_consumer_carries_is_the_count_the_measurement_
///   reports` — `create_verifier` over the verifier family's mock concrete
///   under `NoFailure` and a consumer constructed from the adapters, typed
///   contexts, and admitted identity the case runner hands, with an iteration
///   count other than the builder's default → `Ok` whose output is
///   `Ok(MeasureSuccessReturn)` with `measurement.iterations` equal to the
///   count the consumer was constructed with.
/// Arrange: an iteration count of three and an iteration count of one, over
///   every member of `PAIRING_CONCRETES` and each scope of
///   `DECLARED_KEM_SELECTIONS`; the other families' selections the declared
///   sets' first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_verifier`, once per case, inside the case consumer.
/// Assert:  the output is `Ok(CreateVerifierSuccessReturn { output:
///   Ok(MeasureSuccessReturn { measurement }) })` and
///   `measurement.iterations.get()` equals the count.
/// Boundary: `run_measure_case`, the factories it composes, `create_verifier`,
///   the mock concrete's `try_new`, `VerifierConsumer::try_new`,
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
            let random = payload.random;
            let identity = payload.identity;
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

    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for count in [3u32, 1] {
                let iterations = NonZeroU32::new(count).expect("the count is nonzero");
                let deps = build_measure_case_deps(Consumer { iterations }, Default::default());
                let params = build_measure_case_params(MeasureCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(
                        *DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .find(|selection| {
                                selection.identifier == HashToScalarIdentifier::Keccak256V1
                            })
                            .expect("a keccak256 selection is declared"),
                    ),
                    encoding: Some(CreateEncodingParams {
                        concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                        identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                    }),
                    chain_forms: Some(build_create_chain_forms_params(
                        CreateChainFormsParamsOverrides {
                            concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
                    kem: Some(*kem_selection),
                    key_agreement: Some(DECLARED_KEY_AGREEMENT_SELECTIONS[0]),
                    delivery_proof: Some(build_create_delivery_proof_params(
                        CreateDeliveryProofParamsOverrides {
                            concrete: Some(DELIVERY_PROOF_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                });

                // Act
                let result = run_measure_case(
                    &deps,
                    params,
                    build_measure_case_payload(canonical, &root, Default::default()),
                );

                // Assert
                let success = result
                    .expect("the case runs")
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
///   whole` — `create_verifier` over the verifier family's mock concrete
///   under `NoFailure` and a consumer constructed as above with an iteration
///   count of one → `Ok` whose output is `Ok(MeasureSuccessReturn)` whose
///   timings are each greater than `Duration::ZERO`, whose sizes equal the
///   lengths computed in Arrange through the received pairing's encoders as
///   `every_object_is_sized_by_the_encodings_of_its_elements` of
///   `harness-crypto/measure` states, and whose `costs.mint` and
///   `costs.transfer` each equal the literal cost the verifier family's mock
///   concrete returns; proves the absorbed `harness-crypto/measure` entries
///   `every_operation_is_timed_on_every_concrete`,
///   `every_object_is_sized_by_the_encodings_of_its_elements`, and
///   `every_accepted_delivery_reports_the_cost_the_verifier_returned`.
/// Arrange: an iteration count of one, each identity scope, and the expected
///   sizes computed through the received pairing's encoders over its
///   generators and a scalar its sampling bound samples and the expected cost
///   the literal gas `100_000` and fee `L1DataFee::Charged { wei: 1_000 }`,
///   over every member of `PAIRING_CONCRETES` crossed with both identity
///   scopes and the verifier form each pairing declares; the other families'
///   selections the declared sets' first members, the hash-to-scalar
///   selection a member declaring `HashToScalarIdentifier::Keccak256V1`, the
///   random source `RandomSourceKind::OperatingSystem`.
/// Act:     `create_verifier`, once per case, inside the case consumer.
/// Assert:  in place inside the case consumer — the five timings are each
///   greater than `Duration::ZERO`, the four sizes equal the expected sizes,
///   and the two costs equal the expected cost; the block asserts the output
///   is `Ok` holding `Ok`.
/// Boundary: `run_measure_case`, the factories it composes, `create_verifier`,
///   the mock concrete's `try_new`, `VerifierConsumer::try_new`,
///   `consume_verifier`, and `measure`.
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
            let random = payload.random;
            let identity = payload.identity;
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

    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let deps = build_measure_case_deps(Consumer, Default::default());
            let params = build_measure_case_params(MeasureCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                hash_to_scalar: Some(
                    *DECLARED_HASH_TO_SCALAR_SELECTIONS
                        .iter()
                        .find(|selection| {
                            selection.identifier == HashToScalarIdentifier::Keccak256V1
                        })
                        .expect("a keccak256 selection is declared"),
                ),
                encoding: Some(CreateEncodingParams {
                    concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                    identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                }),
                chain_forms: Some(build_create_chain_forms_params(
                    CreateChainFormsParamsOverrides {
                        concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                key_agreement: Some(DECLARED_KEY_AGREEMENT_SELECTIONS[0]),
                delivery_proof: Some(build_create_delivery_proof_params(
                    CreateDeliveryProofParamsOverrides {
                        concrete: Some(DELIVERY_PROOF_CONCRETES[0]),
                        ..Default::default()
                    },
                )),
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    },
                )),
                ..Default::default()
            });

            // Act
            let result = run_measure_case(
                &deps,
                params,
                build_measure_case_payload(canonical, &root, Default::default()),
            );

            // Assert
            let output = result.expect("the case runs").output;
            assert!(matches!(
                output,
                Ok(CreateVerifierSuccessReturn { output: Ok(_) })
            ));
        }
    }
    // the_measurement_measure_returns_reaches_the_caller_of_create_verifier_whole
}

/// Contract: own entry
///   `a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_
///   consumer` — `create_verifier` over the verifier family's mock concrete
///   under `NoFailure` and a consumer constructed with the source
///   `create_random_source` returns for `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, selected
///   through the case params → `Ok` whose output is `Err(MeasureErrorReturn::
///   Mint(MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSource
///   Adapter)))`; proves the absorbed `harness-crypto/measure` entry
///   `a_draw_refusal_reaches_the_caller_whole` and the absorbed
///   `harness-crypto/delivery/mint` entry
///   `a_refused_draw_reaches_the_caller_whole`.
/// Arrange: the source the case params select, `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, then the
///   operating-system source as the counterfactual, each scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`; the
///   other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `create_verifier`, once per source, inside the case consumer.
/// Assert:  the output under the refusing source matches
///   `Ok(CreateVerifierSuccessReturn { output: Err(MeasureErrorReturn::Mint(
///   MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSource
///   Adapter))) })` and the output under the operating-system source is `Ok`
///   holding `Ok`.
/// Boundary: `run_measure_case`, the factories it composes, `create_verifier`,
///   the mock concrete's `try_new`, `VerifierConsumer::try_new`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the randomness family's mock concrete, selected by configuration.
#[test]
fn a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_consumer() {
    // Arrange
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

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
            let random = payload.random;
            let identity = payload.identity;
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

    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for kind in [
                RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                RandomSourceKind::OperatingSystem,
            ] {
                let deps = build_measure_case_deps(Consumer, Default::default());
                let params = build_measure_case_params(MeasureCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(
                        *DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .find(|selection| {
                                selection.identifier == HashToScalarIdentifier::Keccak256V1
                            })
                            .expect("a keccak256 selection is declared"),
                    ),
                    encoding: Some(CreateEncodingParams {
                        concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                        identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                    }),
                    chain_forms: Some(build_create_chain_forms_params(
                        CreateChainFormsParamsOverrides {
                            concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
                    kem: Some(*kem_selection),
                    key_agreement: Some(DECLARED_KEY_AGREEMENT_SELECTIONS[0]),
                    delivery_proof: Some(build_create_delivery_proof_params(
                        CreateDeliveryProofParamsOverrides {
                            concrete: Some(DELIVERY_PROOF_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides { kind: Some(kind) },
                    )),
                    ..Default::default()
                });

                // Act
                let result = run_measure_case(
                    &deps,
                    params,
                    build_measure_case_payload(canonical, &root, Default::default()),
                );

                // Assert
                let output = result.expect("the case runs").output;
                match kind {
                    RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    ) => assert!(matches!(
                        output,
                        Ok(CreateVerifierSuccessReturn {
                            output: Err(MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
                                FillBytesErrorReturn::MockIRandomSourceAdapter
                            )))
                        })
                    )),
                    RandomSourceKind::OperatingSystem => assert!(matches!(
                        output,
                        Ok(CreateVerifierSuccessReturn { output: Ok(_) })
                    )),
                    _ => panic!("the case names a source the block arranges"),
                }
            }
        }
    }
    // a_refused_draw_reaches_the_caller_of_create_verifier_whole_through_the_consumer
}

/// Contract: own entry
///   `a_refused_chain_read_reaches_the_caller_of_create_verifier_whole_with_
///   the_relation_it_submitted` — `create_verifier` over `VerifierConcrete::
///   Mock(MockIVerifierAdapterFailureMode::RpcRefused)`, selected through the
///   verifier params, and a consumer constructed as above → `Ok` whose output
///   is `Err(MeasureErrorReturn::OnChain { relation: DeliveryRelation::Mint,
///   error: VerifyOnChainErrorReturn::Mock(MockIVerifierAdapterVerifyOnChain
///   ErrorReturn::RpcRefused) })`; proves the absorbed `harness-crypto/measure`
///   entry `a_refused_chain_read_reaches_the_caller_whole_with_the_relation_
///   it_submitted` and the absorbed `harness-crypto/verifier/factory` entry
///   `a_refused_read_returns_the_mock_concretes_own_error`.
/// Arrange: `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::
///   RpcRefused)` selected through `build_create_verifier_params`, then
///   `NoFailure` as the counterfactual, over every member of
///   `PAIRING_CONCRETES`; the other families' selections the declared sets'
///   first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, each scope of
///   `DECLARED_KEM_SELECTIONS`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_verifier`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateVerifierSuccessReturn { output: Err(MeasureErrorReturn::
///   OnChain { relation: DeliveryRelation::Mint, error:
///   VerifyOnChainErrorReturn::Mock(MockIVerifierAdapterVerifyOnChainError
///   Return::RpcRefused) }) })` and the output under `NoFailure` holds `Ok`.
/// Boundary: `run_measure_case`, the factories it composes, `create_verifier`,
///   the mock concrete's `try_new`, `VerifierConsumer::try_new`,
///   `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete under its failure mode.
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
            let random = payload.random;
            let identity = payload.identity;
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

    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for mode in [
                MockIVerifierAdapterFailureMode::RpcRefused,
                MockIVerifierAdapterFailureMode::NoFailure,
            ] {
                let consumer = Consumer {
                    params: build_create_verifier_params(CreateVerifierParamsOverrides {
                        concrete: Some(VerifierConcrete::Mock(mode)),
                        ..Default::default()
                    }),
                };
                let deps = build_measure_case_deps(consumer, Default::default());
                let params = build_measure_case_params(MeasureCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(
                        *DECLARED_HASH_TO_SCALAR_SELECTIONS
                            .iter()
                            .find(|selection| {
                                selection.identifier == HashToScalarIdentifier::Keccak256V1
                            })
                            .expect("a keccak256 selection is declared"),
                    ),
                    encoding: Some(CreateEncodingParams {
                        concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                        identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                    }),
                    chain_forms: Some(build_create_chain_forms_params(
                        CreateChainFormsParamsOverrides {
                            concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
                    kem: Some(*kem_selection),
                    key_agreement: Some(DECLARED_KEY_AGREEMENT_SELECTIONS[0]),
                    delivery_proof: Some(build_create_delivery_proof_params(
                        CreateDeliveryProofParamsOverrides {
                            concrete: Some(DELIVERY_PROOF_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                });

                // Act
                let result = run_measure_case(
                    &deps,
                    params,
                    build_measure_case_payload(canonical, &root, Default::default()),
                );

                // Assert
                let output = result.expect("the case runs").output;
                match mode {
                    MockIVerifierAdapterFailureMode::RpcRefused => assert!(matches!(
                        output,
                        Ok(CreateVerifierSuccessReturn {
                            output: Err(MeasureErrorReturn::OnChain {
                                relation: DeliveryRelation::Mint,
                                error: VerifyOnChainErrorReturn::Mock(
                                    MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                ),
                            })
                        })
                    )),
                    MockIVerifierAdapterFailureMode::NoFailure => assert!(matches!(
                        output,
                        Ok(CreateVerifierSuccessReturn { output: Ok(_) })
                    )),
                }
            }
        }
    }
    // a_refused_chain_read_reaches_the_caller_of_create_verifier_whole_with_the_relation_it_submitted
}
