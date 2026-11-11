#![allow(clippy::expect_used)]

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
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params,
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
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryProofConcrete, DeliveryRelation, IDeliveryProofAdapter,
    MintPurpose, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, RandomSourceKind, build_create_random_source_params,
};

use super::interface::{
    ConsumeMeasureCaseParams, ConsumeMeasureCasePayload, IMeasureCaseConsumer, MeasureErrorReturn,
    MeasureReturn,
};
use super::measure;
use super::mock::{
    MeasureCaseParamsOverrides, MeasureParamsOverrides, MeasurePayloadOverrides,
    build_measure_case_deps, build_measure_case_params, build_measure_case_payload,
    build_measure_deps, build_measure_params, build_measure_payload, run_measure_case,
};
use crate::delivery::provides::MintErrorReturn;
use crate::verifier::provides::{
    CreateVerifierParamsOverrides, IVerifierAdapter, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VerifierConcrete, VerifyOnChainErrorReturn,
    build_create_verifier_params, build_verify_cost,
};

/// Contract: own entry `every_operation_is_timed_on_every_concrete` — `measure`
///   over a case with an iteration count of one, every adapter real, the
///   operating-system source real, and the verifier family's mock concrete
///   under `NoFailure`, returns `Ok` whose timings for `prove_mint`,
///   `prove_transfer`, `verify_mint`, `verify_transfer`, and `decapsulate`
///   are each greater than `Duration::ZERO`.
/// Arrange: every member of `PAIRING_CONCRETES` crossed with every selection
///   of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, `DECLARED_ENCODING_SELECTIONS`,
///   `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS`,
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES`, so
///   both curves and both verifier forms run.
/// Act:     `measure`, once per case, in the case consumer's method.
/// Assert:  every timing of `measurement.timings` exceeds `Duration::ZERO`.
/// Boundary: `measure`, `mint`, `transfer`, and the chain of real functions
///   the private surface names — `create_random_source` for the
///   operating-system kind, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, `create_delivery_proof`, `create_verifier`, and
///   every adapter method the block calls, all running for real.
/// Mocked:  the verifier family's mock concrete under `NoFailure`, since the
///   deployed contract is a runtime input and no case reaches a chain; the
///   operating system's generator and monotonic clock run real.
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
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_measure_case(
                                    &build_measure_case_deps(Consumer, Default::default()),
                                    build_measure_case_params(MeasureCaseParamsOverrides {
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
                                        ..Default::default()
                                    }),
                                    build_measure_case_payload(
                                        b"pkg@1.0.0#1",
                                        &root,
                                        Default::default(),
                                    ),
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
                        }
                    }
                }
            }
        }
    }
    // every_operation_is_timed_on_every_concrete
}

/// Contract: own entry
///   `the_measurement_carries_the_iteration_count_its_means_were_computed_from`
///   — `measure` over a case with an iteration count of three and a case with
///   an iteration count of one returns `measurement.iterations` equal to each
///   count given.
/// Arrange: every case of the private surface, run once with an iteration
///   count of three and once with an iteration count of one.
/// Act:     `measure`, once per case and count, in the case consumer's method.
/// Assert:  `measurement.iterations` equals the count given.
/// Boundary: the chain of real functions the private surface names, all
///   running for real.
/// Mocked:  the verifier family's mock concrete under `NoFailure`; the
///   operating system's generator and monotonic clock run real.
#[test]
fn the_measurement_carries_the_iteration_count_its_means_were_computed_from() {
    struct Consumer {
        iterations: NonZeroU32,
    }

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
                build_measure_params(MeasureParamsOverrides {
                    iterations: Some(self.iterations),
                }),
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
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                for count in [3u32, 1] {
                                    let iterations =
                                        NonZeroU32::new(count).expect("the count is nonzero");
                                    // Act
                                    let returned = run_measure_case(
                                        &build_measure_case_deps(
                                            Consumer { iterations },
                                            Default::default(),
                                        ),
                                        build_measure_case_params(MeasureCaseParamsOverrides {
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
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                            ..Default::default()
                                        }),
                                        build_measure_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    )
                                    .expect("the case runs");
                                    // Assert
                                    let success =
                                        returned.output.expect("the subject's calls succeed");
                                    assert_eq!(success.measurement.iterations, iterations);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // the_measurement_carries_the_iteration_count_its_means_were_computed_from
}

/// Contract: own entry `every_object_is_sized_by_the_encodings_of_its_elements`
///   — `sizes.capsule` equals the `encode_g2` length of `g2_generator` plus
///   that of `g1_generator` through `encode_g1`, once under the asset scope
///   and twice under the entitlement scope; `sizes.envelope` equals twice the
///   `encode_g1` length plus twice the `encode_g2` length; `sizes.mint_proof`
///   equals five `encode_scalar` lengths and `sizes.transfer_proof` six, each
///   plus three `encode_g2` lengths under the verifier form `FirstGroupOnly`;
///   each expected length computed in Arrange through the received pairing's
///   encoders over its generators and a scalar its sampling bound samples.
/// Arrange: every case of the private surface, each member of
///   `PAIRING_CONCRETES` crossed with both identity scopes and the verifier
///   form each pairing declares.
/// Act:     `measure`, once per case, in the case consumer's method.
/// Assert:  every field of `measurement.sizes` equals its expected length.
/// Boundary: the chain of real functions the private surface names, all
///   running for real.
/// Mocked:  the verifier family's mock concrete under `NoFailure`; the
///   operating system's generator and monotonic clock run real.
#[test]
fn every_object_is_sized_by_the_encodings_of_its_elements() {
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
            // Act
            let returned = measure::<P, K, A, D, V>(
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
            );
            // Assert
            let success = returned.as_ref().expect("the subject's calls succeed");
            assert_eq!(success.measurement.sizes.capsule, expected_capsule);
            assert_eq!(success.measurement.sizes.envelope, expected_envelope);
            assert_eq!(success.measurement.sizes.mint_proof, expected_mint);
            assert_eq!(success.measurement.sizes.transfer_proof, expected_transfer);
            returned
        }
    }

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
                                let returned = run_measure_case(
                                    &build_measure_case_deps(Consumer, Default::default()),
                                    build_measure_case_params(MeasureCaseParamsOverrides {
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
                                        ..Default::default()
                                    }),
                                    build_measure_case_payload(
                                        b"pkg@1.0.0#1",
                                        &root,
                                        Default::default(),
                                    ),
                                )
                                .expect("the case runs");
                                // Assert
                                assert!(returned.output.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
    // every_object_is_sized_by_the_encodings_of_its_elements
}

/// Contract: own entry `every_accepted_delivery_reports_the_cost_the_verifier
///   _returned` — `measure` over a case with an iteration count of one and
///   the verifier family's mock concrete under `NoFailure` returns
///   `costs.mint` and `costs.transfer` each equal
///   `build_verify_cost(Default::default())`.
/// Arrange: every member of `PAIRING_CONCRETES`.
/// Act:     `measure`, once per case, in the case consumer's method.
/// Assert:  `measurement.costs.mint` and `measurement.costs.transfer` each
///   equal `build_verify_cost(Default::default())`.
/// Boundary: the chain of real functions the private surface names, all
///   running for real.
/// Mocked:  the verifier family's mock concrete under `NoFailure`; the
///   operating system's generator and monotonic clock run real.
#[test]
fn every_accepted_delivery_reports_the_cost_the_verifier_returned() {
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
        let expected = build_verify_cost(Default::default());
        assert_eq!(success.measurement.costs.mint, expected);
        assert_eq!(success.measurement.costs.transfer, expected);
    }
    // every_accepted_delivery_reports_the_cost_the_verifier_returned
}

/// Contract: own entry
///   `a_refused_chain_read_reaches_the_caller_whole_with_the_relation_it_sub
///   mitted` — `measure` over a case whose verifier is `VerifierConcrete::
///   Mock(MockIVerifierAdapterFailureMode::RpcRefused)`, selected by
///   configuration, returns `Err(MeasureErrorReturn::OnChain { relation:
///   DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })`; proves
///   the `harness-crypto/verifier/factory` disposition `a_refused_read_
///   returns_the_mock_concretes_own_error`, the mock concrete's own error
///   carried unchanged inside `MeasureErrorReturn::OnChain` with the relation
///   it concerned.
/// Arrange: every member of `PAIRING_CONCRETES`, the verifier selected by
///   configuration through the verifier params.
/// Act:     `measure`, once per case, in the case consumer's method.
/// Assert:  the `MeasureReturn` matches `Err(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })`, asserted
///   by a pattern naming every field because `MeasureErrorReturn` omits
///   `PartialEq`.
/// Boundary: the chain of real functions the private surface names, all
///   running for real; the transfer is not submitted.
/// Mocked:  the verifier family's mock concrete under `RpcRefused`, selected
///   by configuration; the operating system's generator and monotonic clock
///   run real.
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
        let returned = run_measure_case(
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
        // Assert
        assert!(matches!(
            returned.output,
            Err(MeasureErrorReturn::OnChain {
                relation: DeliveryRelation::Mint,
                error: VerifyOnChainErrorReturn::Mock(
                    MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                )
            })
        ));
    }
    // a_refused_chain_read_reaches_the_caller_whole_with_the_relation_it_submitted
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole` —
///   `measure` over a case whose hash-to-scalar adapter is the family's mock
///   concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`,
///   selected by configuration, returns `Err(MeasureErrorReturn::Mint(
///   MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(
///   error))))` under the entitlement scope and `Err(MeasureErrorReturn::
///   Mint(MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(error))))`
///   under the asset scope, `error` `HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   UNIFORM_BYTES_LENGTH, actual: 0 })`, unconverted.
/// Arrange: each selection and scope of `DECLARED_KEM_SELECTIONS` over every
///   member of `PAIRING_CONCRETES`, the hash-to-scalar adapter selected by
///   configuration through the hash-to-scalar params.
/// Act:     `measure`, once per case, in the case consumer's method.
/// Assert:  the `MeasureReturn` matches the scope's error with `expected`
///   `UNIFORM_BYTES_LENGTH` and `actual` `0`; the refusal reaches the caller
///   inside the mint delivery's own variant, before any timing.
/// Boundary: the chain of real functions the private surface names, all
///   running for real.
/// Mocked:  the hash-to-scalar family's mock concrete under
///   `SamplingRefused` and the verifier family's mock concrete under
///   `NoFailure`, each a selection by configuration; the operating system's
///   generator and monotonic clock run real.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
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
            let returned = measure::<P, K, A, D, V>(
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
            );
            // Assert
            match identity.setup_scope {
                SetupScope::Entitlement => {
                    assert!(matches!(
                        &returned,
                        Err(MeasureErrorReturn::Mint(MintErrorReturn::DeriveIdentity(
                            DeriveIdentityErrorReturn::HashToScalar(
                                HashToScalarErrorReturn::Sampling(
                                    SampleUniformScalarErrorReturn::WrongLength {
                                        expected,
                                        actual: 0,
                                    }
                                )
                            )
                        ))) if *expected
                            == <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                    ));
                }
                SetupScope::Asset { .. } => {
                    assert!(matches!(
                        &returned,
                        Err(MeasureErrorReturn::Mint(MintErrorReturn::Setup(
                            SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                                SampleUniformScalarErrorReturn::WrongLength {
                                    expected,
                                    actual: 0,
                                }
                            ))
                        ))) if *expected
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
            assert!(matches!(returned.output, Err(MeasureErrorReturn::Mint(_))));
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_nonce_draw_refusal_reaches_the_caller_whole` —
///   `measure` over a case whose delivery proof is the family's mock concrete
///   under `MockIDeliveryProofAdapterFailureMode::NonceDrawRefused`,
///   selected by configuration, returns `Err(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))`,
///   the refusal arriving whole from the mint delivery's prover, before any
///   timed call.
/// Arrange: every member of `PAIRING_CONCRETES` with each selection and scope
///   of `DECLARED_KEM_SELECTIONS`, the delivery proof selected by
///   configuration through the delivery-proof params, against
///   `DELIVERY_PROOF_CONCRETES`, under which the call returns `Ok`.
/// Act:     `measure`, once per case, in the case consumer's method.
/// Assert:  the `MeasureReturn` matches `Err(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))`.
/// Boundary: the chain of real functions the private surface names, all
///   running for real.
/// Mocked:  the delivery-proof family's mock concrete under
///   `NonceDrawRefused` and the verifier family's mock concrete under
///   `NoFailure`, each a selection by configuration; the operating system's
///   generator and monotonic clock run real.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
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
                Err(MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
                    ProveMintErrorReturn::Mock(
                        MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                    )
                )))
            ));
        }
    }
    // a_nonce_draw_refusal_reaches_the_caller_whole
}

// Private surface enumeration — every own entry of the spec's private surface
// and the block that proves it:
//   every_operation_is_timed_on_every_concrete
//     → every_operation_is_timed_on_every_concrete
//   the_measurement_carries_the_iteration_count_its_means_were_computed_from
//     → the_measurement_carries_the_iteration_count_its_means_were_computed_from
//   every_object_is_sized_by_the_encodings_of_its_elements
//     → every_object_is_sized_by_the_encodings_of_its_elements
//   every_accepted_delivery_reports_the_cost_the_verifier_returned
//     → every_accepted_delivery_reports_the_cost_the_verifier_returned
//   a_refused_chain_read_reaches_the_caller_whole_with_the_relation_it_submitted
//     → a_refused_chain_read_reaches_the_caller_whole_with_the_relation_it_submitted
//       (also proves the `harness-crypto/verifier/factory` disposition
//        `a_refused_read_returns_the_mock_concretes_own_error`)
//   a_hash_to_scalar_refusal_reaches_the_caller_whole
//     → a_hash_to_scalar_refusal_reaches_the_caller_whole
//   a_nonce_draw_refusal_reaches_the_caller_whole
//     → a_nonce_draw_refusal_reaches_the_caller_whole
