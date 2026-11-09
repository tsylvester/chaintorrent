#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use core::num::NonZeroU32;
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter,
};
use envelope::{DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter};
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{DECLARED_KEM_SELECTIONS, ICredentialKemAdapter};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, IDeliveryProofAdapter, MintPurpose, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};
use serde_json::{Value, json};
use std::env::temp_dir;
use std::fs;

use super::interface::{ReportDestination, ReportReturn};
use super::mock::{
    ReportParamsOverrides, ReportPayloadOverrides, build_report_deps, build_report_params,
    build_report_payload,
};
use super::report;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::measure::provides::{
    DeliveryMeasurement, IMeasureCaseConsumer, MeasureCaseParamsOverrides, MeasureErrorReturn,
    MeasureParamsOverrides, MeasurePayloadOverrides, build_measure_case_deps,
    build_measure_case_params, build_measure_case_payload, build_measure_deps,
    build_measure_params, build_measure_payload, measure, run_measure_case,
};
use crate::verifier::provides::{IVerifierAdapter, L1DataFee};

/// Contract: own entry `the_document_records_every_value_a_measurement_holds`
///   — `report` over the measurement the real `measure` returned for a case
///   with an iteration count of one, every adapter real, the operating-
///   system source real, and the verifier family's mock concrete under
///   `NoFailure`, and one file destination, returns `Ok` whose document the
///   destination file's text equals and whose parsed `iterations`, five
///   `timings`, four `sizes_bytes`, and two `costs` equal the measurement's
///   values, each timing as the whole seconds and subsecond nanoseconds of
///   its `Duration`.
/// Arrange: every member of `PAIRING_CONCRETES` crossed with every selection
///   of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, `DECLARED_ENCODING_SELECTIONS`,
///   `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS`,
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES`, so
///   both curves and both verifier forms run; a file destination beneath a
///   directory removed before each act.
/// Act:     `measure` then `report`, once per case, in the case consumer's
///   method.
/// Assert:  the file's text equals the returned document, and the parsed
///   document equals the `json!` object built from the measurement's values
///   field by field.
/// Boundary: `measure`, `report`, `mint`, `transfer`, and the chain of real
///   functions the private surface names — `create_random_source` for the
///   operating-system kind, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, `create_delivery_proof`, `create_verifier`, and
///   every adapter method the block calls, all running for real.
/// Mocked:  the verifier family's mock concrete under `NoFailure`, since the
///   deployed contract is a runtime input and no case reaches a chain; the
///   operating system's generator and monotonic clock and the filesystem run
///   real.
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
            let success = measured?;
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
                                let outcome = returned.output.expect("the subject's calls succeed");
                                let success = outcome.report.expect("the report call succeeds");
                                assert_eq!(outcome.file, success.document);
                                let parsed: Value = serde_json::from_str(&success.document)
                                    .expect("the document parses");
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
                        }
                    }
                }
            }
        }
    }
    // the_document_records_every_value_a_measurement_holds
}
