#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use core::num::NonZeroU32;
use domain::{Secret, SecretConstructorParams};
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
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
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};
use serde_json::{Value, json};
use std::env::temp_dir;
use std::fs;

use harness_crypto::{
    AdmittedKemIdentity, DeliveryMeasurement, IMeasureCaseConsumer, IVerifierAdapter, L1DataFee,
    MeasureCaseParamsOverrides, MeasureErrorReturn, MeasureParamsOverrides,
    MeasurePayloadOverrides, ReportDestination, ReportParamsOverrides, ReportPayloadOverrides,
    ReportReturn, build_measure_case_deps, build_measure_case_params, build_measure_case_payload,
    build_measure_deps, build_measure_params, build_measure_payload, build_report_deps,
    build_report_params, build_report_payload, measure, report, run_measure_case,
};

/// Contract: own entry `the_document_records_every_value_a_measurement_holds`
///   — `report` over the measurement the real `measure` returned for a case
///   with an iteration count of one, every adapter real, the operating-
///   system source real, and the verifier family's mock concrete under
///   `NoFailure`, and one file destination, returns `Ok` whose document the
///   destination file's text equals and whose parsed `iterations`, five
///   `timings`, four `sizes_bytes`, and two `costs` equal the measurement's
///   values, each timing as the whole seconds and subsecond nanoseconds of
///   its `Duration`.
/// Arrange: each member of `PAIRING_CONCRETES`, the operating-system source,
///   the verifier family's mock concrete under `NoFailure`, an iteration
///   count of one, and a file destination beneath a directory removed before
///   the act.
/// Act:     `measure` then `report`, once per member, in the case consumer's
///   method.
/// Assert:  the result is `Ok`, the file equals the returned document, and
///   the parsed document equals the `json!` object built from the
///   measurement's values field by field.
/// Boundary: the public call `report` over the measurement the public call
///   `measure` returned, invoked through the crate root over the factories'
///   concretes the route names — `create_random_source` for the
///   operating-system kind, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, `create_delivery_proof`, and `create_verifier`.
/// Mocked:  the verifier family's mock concrete by configuration; the
///   operating-system generator and monotonic clock run real; the
///   filesystem is the outer edge.
#[test]
fn the_document_records_every_value_a_measurement_holds() {
    struct Outcome {
        report: ReportReturn,
        file: String,
        measurement: DeliveryMeasurement,
    }

    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = Result<Outcome, MeasureErrorReturn>;

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
            let measured = measure::<P, K, A, D, V>(
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
                    iterations: Some(NonZeroU32::MIN),
                }),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = match measured {
                Ok(success) => success,
                Err(error) => return Err(error),
            };
            let directory = temp_dir().join(format!(
                "chaintorrent-report-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&directory);
            fs::create_dir_all(&directory).expect("the directory is created");
            let file = directory.join("report.json");
            // Act
            let reported = report(
                &build_report_deps(Default::default()),
                build_report_params(ReportParamsOverrides {
                    destinations: Some(vec![ReportDestination::File { path: file.clone() }]),
                }),
                build_report_payload(ReportPayloadOverrides {
                    measurement: Some(success.measurement),
                    ..Default::default()
                }),
            );
            let text = match &reported {
                Ok(_) => fs::read_to_string(&file).expect("the destination file reads"),
                Err(_) => String::new(),
            };
            Ok(Outcome {
                report: reported,
                file: text,
                measurement: success.measurement,
            })
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
        let outcome = returned.output.expect("the subject's calls succeed");
        let success = outcome.report.expect("the report call succeeds");
        assert_eq!(outcome.file, success.document);
        let parsed: Value = serde_json::from_str(&success.document).expect("the document parses");
        let measurement = outcome.measurement;
        let timings = &measurement.timings;
        let sizes = &measurement.sizes;
        let costs = &measurement.costs;
        let l1_data_fee = |fee: L1DataFee| -> Value {
            match fee {
                L1DataFee::Charged { wei } => {
                    json!({ "state": "charged", "wei": wei.to_string() })
                }
                L1DataFee::NotApplicable => {
                    json!({ "state": "not_applicable" })
                }
            }
        };
        let expected = json!({
            "curve": "curve",
            "conditions": "curve",
            "iterations": measurement.iterations.get(),
            "timings": {
                "prove_mint": {
                    "seconds": timings.prove_mint.as_secs(),
                    "subsec_nanos": timings.prove_mint.subsec_nanos()
                },
                "prove_transfer": {
                    "seconds": timings.prove_transfer.as_secs(),
                    "subsec_nanos": timings.prove_transfer.subsec_nanos()
                },
                "verify_mint": {
                    "seconds": timings.verify_mint.as_secs(),
                    "subsec_nanos": timings.verify_mint.subsec_nanos()
                },
                "verify_transfer": {
                    "seconds": timings.verify_transfer.as_secs(),
                    "subsec_nanos": timings.verify_transfer.subsec_nanos()
                },
                "decapsulate": {
                    "seconds": timings.decapsulate.as_secs(),
                    "subsec_nanos": timings.decapsulate.subsec_nanos()
                }
            },
            "sizes_bytes": {
                "capsule": sizes.capsule,
                "envelope": sizes.envelope,
                "mint_proof": sizes.mint_proof,
                "transfer_proof": sizes.transfer_proof
            },
            "costs": {
                "mint": {
                    "l2_execution_gas": costs.mint.l2_execution_gas,
                    "l1_data_fee": l1_data_fee(costs.mint.l1_data_fee)
                },
                "transfer": {
                    "l2_execution_gas": costs.transfer.l2_execution_gas,
                    "l1_data_fee": l1_data_fee(costs.transfer.l1_data_fee)
                }
            },
            "defaults_in_force": {
                "piece_group_size_bytes": 1,
                "primary_curve": "curve"
            },
            "adjustment": null
        });
        assert_eq!(parsed, expected);
    }
    // the_document_records_every_value_a_measurement_holds
}

/// Contract: carried `measure` entry `every_operation_is_timed_on_every_
///   concrete` — each of the five timings the document records is greater
///   than zero seconds or greater than zero subsecond nanoseconds.
/// Arrange: each member of `PAIRING_CONCRETES`, the operating-system source,
///   the verifier family's mock concrete under `NoFailure`, an iteration
///   count of one.
/// Act:     `measure` then `report`, once per member, in the case consumer's
///   method.
/// Assert:  the result is `Ok` and each of the five timings recorded in the
///   parsed document has seconds or subsecond nanoseconds greater than zero.
/// Boundary: the public call `report` over the measurement the public call
///   `measure` returned, invoked through the crate root over the factories'
///   concretes the route names.
/// Mocked:  the verifier family's mock concrete by configuration; the
///   operating-system generator and monotonic clock run real.
#[test]
fn every_operation_is_timed_on_every_concrete() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = Result<ReportReturn, MeasureErrorReturn>;

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
            let measured = measure::<P, K, A, D, V>(
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
                    iterations: Some(NonZeroU32::MIN),
                }),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = match measured {
                Ok(success) => success,
                Err(error) => return Err(error),
            };
            let directory = temp_dir().join(format!(
                "chaintorrent-report-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&directory);
            fs::create_dir_all(&directory).expect("the directory is created");
            let file = directory.join("report.json");
            // Act
            Ok(report(
                &build_report_deps(Default::default()),
                build_report_params(ReportParamsOverrides {
                    destinations: Some(vec![ReportDestination::File { path: file }]),
                }),
                build_report_payload(ReportPayloadOverrides {
                    measurement: Some(success.measurement),
                    ..Default::default()
                }),
            ))
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
        let reported = returned.output.expect("the subject's calls succeed");
        let success = reported.expect("the report call succeeds");
        let parsed: Value = serde_json::from_str(&success.document).expect("the document parses");
        for timing in [
            "prove_mint",
            "prove_transfer",
            "verify_mint",
            "verify_transfer",
            "decapsulate",
        ] {
            let seconds = parsed["timings"][timing]["seconds"]
                .as_u64()
                .expect("a timing's seconds is a number");
            let nanos = parsed["timings"][timing]["subsec_nanos"]
                .as_u64()
                .expect("a timing's nanos is a number");
            assert!(seconds > 0 || nanos > 0);
        }
    }
    // every_operation_is_timed_on_every_concrete
}

/// Contract: carried `measure` entry
///   `the_measurement_carries_the_iteration_count_its_means_were_computed_
///   from` — the document's `iterations` equals the count the measurement
///   was taken with.
/// Arrange: iteration counts of `3` and `1` over each member of
///   `PAIRING_CONCRETES`, the operating-system source, and the verifier
///   family's mock concrete under `NoFailure`.
/// Act:     `measure` then `report`, once per count and member, in the case
///   consumer's method.
/// Assert:  the result is `Ok` and the parsed document's `iterations` equals
///   each count.
/// Boundary: the public call `report` over the measurement the public call
///   `measure` returned, invoked through the crate root over the factories'
///   concretes the route names.
/// Mocked:  the verifier family's mock concrete by configuration; the
///   operating-system generator and monotonic clock run real.
#[test]
fn the_measurement_carries_the_iteration_count_its_means_were_computed_from() {
    struct Consumer {
        iterations: NonZeroU32,
    }

    impl IMeasureCaseConsumer for Consumer {
        type Output = Result<ReportReturn, MeasureErrorReturn>;

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
            let measured = measure::<P, K, A, D, V>(
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
            );
            let success = match measured {
                Ok(success) => success,
                Err(error) => return Err(error),
            };
            let directory = temp_dir().join(format!(
                "chaintorrent-report-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&directory);
            fs::create_dir_all(&directory).expect("the directory is created");
            let file = directory.join("report.json");
            // Act
            Ok(report(
                &build_report_deps(Default::default()),
                build_report_params(ReportParamsOverrides {
                    destinations: Some(vec![ReportDestination::File { path: file }]),
                }),
                build_report_payload(ReportPayloadOverrides {
                    measurement: Some(success.measurement),
                    ..Default::default()
                }),
            ))
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
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
            let reported = returned.output.expect("the subject's calls succeed");
            let success = reported.expect("the report call succeeds");
            let parsed: Value =
                serde_json::from_str(&success.document).expect("the document parses");
            assert_eq!(parsed["iterations"], json!(count));
        }
    }
    // the_measurement_carries_the_iteration_count_its_means_were_computed_from
}

/// Contract: carried `measure` entry
///   `every_object_is_sized_by_the_encodings_of_its_elements` — the
///   document's `sizes_bytes` equal the encoded lengths of the object's
///   elements, each computed in Arrange through the received pairing's
///   encoders over its generators and a scalar its sampling bound samples.
/// Arrange: each member of `PAIRING_CONCRETES` crossed with both identity
///   scopes, the operating-system source, and the verifier family's mock
///   concrete under `NoFailure`.
/// Act:     `measure` then `report`, once per case, in the case consumer's
///   method.
/// Assert:  the result is `Ok` and the parsed document's `sizes_bytes.
///   capsule`, `envelope`, `mint_proof`, and `transfer_proof` equal the
///   expected sizes.
/// Boundary: the public call `report` over the measurement the public call
///   `measure` returned, invoked through the crate root over the factories'
///   concretes the route names.
/// Mocked:  the verifier family's mock concrete by configuration; the
///   operating-system generator and monotonic clock run real.
#[test]
fn every_object_is_sized_by_the_encodings_of_its_elements() {
    struct Outcome {
        report: ReportReturn,
        capsule: usize,
        envelope: usize,
        mint_proof: usize,
        transfer_proof: usize,
    }

    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = Result<Outcome, MeasureErrorReturn>;

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
            let capsule = match identity.setup_scope {
                SetupScope::Entitlement => g2_length + 2 * g1_length,
                SetupScope::Asset { .. } => g2_length + g1_length,
            };
            let envelope = 2 * g1_length + 2 * g2_length;
            let first_messages = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::FirstGroupOnly => 3 * g2_length,
                VerifierGroupArithmetic::BothGroups => 0,
            };
            let mint_proof = 5 * scalar_length + first_messages;
            let transfer_proof = 6 * scalar_length + first_messages;
            let measured = measure::<P, K, A, D, V>(
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
                    iterations: Some(NonZeroU32::MIN),
                }),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = match measured {
                Ok(success) => success,
                Err(error) => return Err(error),
            };
            let directory = temp_dir().join(format!(
                "chaintorrent-report-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&directory);
            fs::create_dir_all(&directory).expect("the directory is created");
            let file = directory.join("report.json");
            // Act
            Ok(Outcome {
                report: report(
                    &build_report_deps(Default::default()),
                    build_report_params(ReportParamsOverrides {
                        destinations: Some(vec![ReportDestination::File { path: file }]),
                    }),
                    build_report_payload(ReportPayloadOverrides {
                        measurement: Some(success.measurement),
                        ..Default::default()
                    }),
                ),
                capsule,
                envelope,
                mint_proof,
                transfer_proof,
            })
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
            let outcome = returned.output.expect("the subject's calls succeed");
            let success = outcome.report.expect("the report call succeeds");
            let parsed: Value =
                serde_json::from_str(&success.document).expect("the document parses");
            assert_eq!(parsed["sizes_bytes"]["capsule"], json!(outcome.capsule));
            assert_eq!(parsed["sizes_bytes"]["envelope"], json!(outcome.envelope));
            assert_eq!(
                parsed["sizes_bytes"]["mint_proof"],
                json!(outcome.mint_proof)
            );
            assert_eq!(
                parsed["sizes_bytes"]["transfer_proof"],
                json!(outcome.transfer_proof)
            );
        }
    }
    // every_object_is_sized_by_the_encodings_of_its_elements
}

/// Contract: carried `measure` entry
///   `every_accepted_delivery_reports_the_cost_the_verifier_returned` — the
///   document's `costs` equal the cost the verifier family's mock concrete
///   returns for each submission, its gas `100_000` and its fee charged at
///   the wei `"1000"`.
/// Arrange: each member of `PAIRING_CONCRETES`, the operating-system source,
///   and the verifier family's mock concrete under `NoFailure`.
/// Act:     `measure` then `report`, once per member, in the case consumer's
///   method.
/// Assert:  the result is `Ok` and the parsed document's `costs.mint` and
///   `costs.transfer` each equal `json!({ "l2_execution_gas": 100000,
///   "l1_data_fee": { "state": "charged", "wei": "1000" } })`.
/// Boundary: the public call `report` over the measurement the public call
///   `measure` returned, invoked through the crate root over the factories'
///   concretes the route names.
/// Mocked:  the verifier family's mock concrete by configuration; the
///   operating-system generator and monotonic clock run real.
#[test]
fn every_accepted_delivery_reports_the_cost_the_verifier_returned() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = Result<ReportReturn, MeasureErrorReturn>;

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
            let measured = measure::<P, K, A, D, V>(
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
                    iterations: Some(NonZeroU32::MIN),
                }),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = match measured {
                Ok(success) => success,
                Err(error) => return Err(error),
            };
            let directory = temp_dir().join(format!(
                "chaintorrent-report-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&directory);
            fs::create_dir_all(&directory).expect("the directory is created");
            let file = directory.join("report.json");
            // Act
            Ok(report(
                &build_report_deps(Default::default()),
                build_report_params(ReportParamsOverrides {
                    destinations: Some(vec![ReportDestination::File { path: file }]),
                }),
                build_report_payload(ReportPayloadOverrides {
                    measurement: Some(success.measurement),
                    ..Default::default()
                }),
            ))
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
        let reported = returned.output.expect("the subject's calls succeed");
        let success = reported.expect("the report call succeeds");
        let parsed: Value = serde_json::from_str(&success.document).expect("the document parses");
        assert_eq!(
            parsed["costs"]["mint"],
            json!({ "l2_execution_gas": 100000, "l1_data_fee": { "state": "charged", "wei": "1000" } })
        );
        assert_eq!(
            parsed["costs"]["transfer"],
            json!({ "l2_execution_gas": 100000, "l1_data_fee": { "state": "charged", "wei": "1000" } })
        );
    }
    // every_accepted_delivery_reports_the_cost_the_verifier_returned
}
