#![allow(clippy::expect_used)]

use chain::IChainForms;
use core::num::NonZeroU32;
use domain::{Secret, SecretConstructorParams};
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    SampleUniformScalarParams, SampleUniformScalarPayload, VerifierGroupArithmetic,
};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_delivery_context,
};
use random::IRandomSourceAdapter;

use super::interface::{IMeasureCaseConsumer, MeasureReturn};
use super::measure;
use super::mock::{
    MeasureParamsOverrides, MeasurePayloadOverrides, build_measure_case_deps,
    build_measure_case_params, build_measure_case_payload, build_measure_deps,
    build_measure_params, build_measure_payload, run_measure_case,
};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::verifier::provides::{IVerifierAdapter, L1DataFee, VerifyCost};

/// Contract: Computed → `Ok(MeasureSuccessReturn { measurement })` with
///   `measurement.iterations` equal to `params.iterations`.
/// Arrange: `iterations` of `3`, a count other than the builder's default.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  `measurement.iterations.get()` equals `3`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn measure_returns_the_iteration_count_it_was_given() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
                    iterations: Some(NonZeroU32::new(3).expect("three is nonzero")),
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

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(Default::default()),
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    let success = case.output.expect("the subject's calls succeed");
    assert_eq!(success.measurement.iterations.get(), 3);
    // measure_returns_the_iteration_count_it_was_given
}

/// Contract: Sizes → the capsule's bytes are the encoded lengths of `u`, `v`,
///   and `w`.
/// Arrange: the entitlement scope; the expected size the `encode_g2` length of
///   `g2_generator` plus twice the `encode_g1` length of `g1_generator`.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  `measurement.sizes.capsule` equals the expected size.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn measure_sizes_the_capsule_by_the_encodings_of_its_elements() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
            let expected = g2_encoded.bytes.as_ref().len() + 2 * g1_encoded.bytes.as_ref().len();
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
            assert_eq!(success.measurement.sizes.capsule, expected);
            returned
        }
    }

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(Default::default()),
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    let _ = case.output.expect("the subject's calls succeed");
    // measure_sizes_the_capsule_by_the_encodings_of_its_elements
}

/// Contract: Sizes → the envelope's bytes are the encoded lengths of `c1`,
///   `c2`, `d1`, and `d2`.
/// Arrange: the expected size twice the `encode_g1` length of `g1_generator`
///   plus twice the `encode_g2` length of `g2_generator`.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  `measurement.sizes.envelope` equals the expected size.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn measure_sizes_the_envelope_by_the_encodings_of_its_elements() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
            let expected =
                2 * g1_encoded.bytes.as_ref().len() + 2 * g2_encoded.bytes.as_ref().len();
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
            assert_eq!(success.measurement.sizes.envelope, expected);
            returned
        }
    }

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(Default::default()),
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    let _ = case.output.expect("the subject's calls succeed");
    // measure_sizes_the_envelope_by_the_encodings_of_its_elements
}

/// Contract: Sizes → a proof's bytes are the encoded lengths of its challenge,
///   its responses, and, under `FirstGroupOnly`, its three second-group first
///   messages.
/// Arrange: the expected mint size the `encode_scalar` length of the sampled
///   scalar times five, and the expected transfer size times six, each plus
///   three times the `encode_g2` length of `g2_generator` when
///   `P::DECLARATION.verifier_group_arithmetic` is
///   `VerifierGroupArithmetic::FirstGroupOnly`.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  `measurement.sizes.mint_proof` and
///   `measurement.sizes.transfer_proof` equal the expected sizes.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn measure_sizes_each_proof_by_the_encodings_of_its_components() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
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
            let scalar_length = scalar_encoded.bytes.expose().as_ref().len();
            let first_messages = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::FirstGroupOnly => 3 * g2_encoded.bytes.as_ref().len(),
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
            assert_eq!(success.measurement.sizes.mint_proof, expected_mint);
            assert_eq!(success.measurement.sizes.transfer_proof, expected_transfer);
            returned
        }
    }

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(Default::default()),
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    let _ = case.output.expect("the subject's calls succeed");
    // measure_sizes_each_proof_by_the_encodings_of_its_components
}

/// Contract: Costed pair → an accepted delivery's cost is the verifier's
///   `Accepted { cost }`.
/// Arrange: the verifier family's mock concrete under `NoFailure`, the
///   default; the expected cost `VerifyCost { l2_execution_gas: 100_000,
///   l1_data_fee: L1DataFee::Charged { wei: 1_000 } }` for each relation.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  `measurement.costs.mint` and `measurement.costs.transfer` each
///   equal the expected cost.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn measure_reports_the_cost_the_verifier_accepted_for_each_delivery() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let expected = VerifyCost {
                l2_execution_gas: 100_000,
                l1_data_fee: L1DataFee::Charged { wei: 1_000 },
            };
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
            assert_eq!(success.measurement.costs.mint, expected);
            assert_eq!(success.measurement.costs.transfer, expected);
            returned
        }
    }

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(Default::default()),
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    let _ = case.output.expect("the subject's calls succeed");
    // measure_reports_the_cost_the_verifier_accepted_for_each_delivery
}
