#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use core::num::NonZeroU32;
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, DecodeParams, EncodingConcrete,
    EncodingIdentifier, IDecoderAdapter, IEncoderAdapter,
};
use envelope::{DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter};
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{DECLARED_KEM_SELECTIONS, ICredentialKemAdapter};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, CreateDeliveryProofReturn,
    CreateDeliveryProofSuccessReturn, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete,
    DeliveryRelation, IDeliveryProofAdapter, MintPurpose, TransferPurpose,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, build_delivery_context, create_delivery_proof,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params,
};

use super::interface::{DeliveryProofConsumerErrorReturn, DeliveryProofConsumerReturn};
use super::mock::{DeliveryProofConsumerConstructorParamsOverrides, build_delivery_proof_consumer};
use crate::delivery::provides::MintErrorReturn;
use crate::generate::provides::{
    ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload,
    CreateGenerateCaseParamsOverrides, EmitErrorReturn, GENERATE_CONCRETES, GenerateConcrete,
    GenerateConcreteKind, GenerateConcreteOverrides, GenerateKindConfigurationsOverrides,
    ICreateGenerateCaseConsumer, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, VectorKind, VectorKindConfiguration,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, build_create_generate_params, build_generate_concrete,
    build_generate_kind_configurations, run_create_generate_case,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::verifier::provides::{
    CreateVerifierErrorReturn, L1DataFee, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VerifierConcrete, VerifyCost,
    VerifyOnChainErrorReturn,
};

/// Contract: own entry
///   `the_files_and_the_measurement_reach_the_caller_of_create_delivery_proof_whole` —
///   `create_delivery_proof` over a member of `DELIVERY_PROOF_CONCRETES` and
///   the mock selection, with a consumer constructed from the adapters, typed
///   contexts, and identities the case runner hands, a member of
///   `GENERATE_CONCRETES` or the mock generate selection under `NoFailure`,
///   and `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)` →
///   `Ok` whose output is `Ok(DeliveryProofConsumerSuccessReturn)` whose
///   `files` name no path twice, include for a real generate concrete files
///   tagged `Some(VectorKind::IdentityMapping)`, `Some(VectorKind::Challenge)`,
///   `Some(VectorKind::MintProof)`, and `Some(VectorKind::TransferProof)` each
///   of whose paths exists on the filesystem, and whose `measurement.iterations`
///   equals the held count and `measurement.costs.mint` and
///   `measurement.costs.transfer` each equal the literal cost the verifier
///   family's mock concrete returns.
/// Arrange: every member of `GENERATE_CONCRETES` and the mock generate
///   selection, crossed with every member of `PAIRING_CONCRETES`, each scope
///   of `DECLARED_KEM_SELECTIONS`, and every member of
///   `DELIVERY_PROOF_CONCRETES`; the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the other families'
///   selections the declared sets' first members, the random source
///   `RandomSourceKind::OperatingSystem`, an iteration count of three, and the
///   expected cost the literal gas `100_000` and fee `L1DataFee::Charged {
///   wei: 1_000 }`.
/// Act:     `create_delivery_proof`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four tagged
///   files and each of its paths exists, `measurement.iterations` equals the
///   held count, and the two costs equal the expected cost.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_delivery_proof`, the selected concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, the selected generate concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, `emit`,
///   `create_verifier`, `VerifierConsumer::try_new`, `consume_verifier`, and
///   `measure`.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_files_and_the_measurement_reach_the_caller_of_create_delivery_proof_whole() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
        concrete: DeliveryProofConcrete,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    iterations: NonZeroU32::new(3),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                    concrete: Some(self.concrete),
                    ..Default::default()
                }),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let expected_cost = VerifyCost {
        l2_execution_gas: 100_000,
        l1_data_fee: L1DataFee::Charged { wei: 1_000 },
    };
    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];
    let selections: Vec<Option<GenerateConcreteKind>> = GENERATE_CONCRETES
        .iter()
        .map(|kind| Some(*kind))
        .chain(core::iter::once(None))
        .collect();

    for selection in &selections {
        let (generate, kinds) = match selection {
            Some(kind) => (
                build_generate_concrete(*kind, GenerateConcreteOverrides),
                build_generate_kind_configurations(*kind, GenerateKindConfigurationsOverrides),
            ),
            None => {
                let params = build_create_generate_params(Default::default());
                (params.concrete, params.kinds)
            }
        };
        for pairing_concrete in PAIRING_CONCRETES {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                    let consumer = Consumer {
                        generate: generate.clone(),
                        kinds: kinds.clone(),
                        concrete: *delivery_proof_concrete,
                    };
                    let case_params =
                        build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing_concrete),
                                    ..Default::default()
                                },
                            )),
                            hash_to_scalar: Some(
                                *DECLARED_HASH_TO_SCALAR_SELECTIONS
                                    .iter()
                                    .find(|selection| {
                                        selection.identifier == HashToScalarIdentifier::Keccak256V1
                                    })
                                    .expect("a keccak256 selection is declared"),
                            ),
                            encoding: Some(match selection {
                                Some(_) => CreateEncodingParams {
                                    concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                                    identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                                },
                                None => CreateEncodingParams {
                                    concrete: EncodingConcrete::Mock,
                                    identifier: EncodingIdentifier::Mock,
                                },
                            }),
                            chain_forms: selection.as_ref().map(|_| {
                                build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                    concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                                    ..Default::default()
                                })
                            }),
                            kem: Some(*kem_selection),
                            key_agreement: Some(DECLARED_KEY_AGREEMENT_SELECTIONS[0]),
                            delivery_proof: Some(build_create_delivery_proof_params(
                                CreateDeliveryProofParamsOverrides {
                                    concrete: Some(*delivery_proof_concrete),
                                    ..Default::default()
                                },
                            )),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        });

                    // Act
                    let result = run_create_generate_case(
                        &build_create_generate_case_deps(consumer, Default::default()),
                        case_params,
                        build_create_generate_case_payload(&canonicals, &roots, Default::default()),
                    );

                    // Assert
                    let success = result
                        .expect("the case composes")
                        .output
                        .expect("the factory admits the concrete and runs the consumer")
                        .output
                        .expect("the consumer emits and measures");
                    let mut paths = HashSet::new();
                    for file in &success.files {
                        assert!(
                            paths.insert(file.path.clone()),
                            "no file path is named twice"
                        );
                    }
                    if selection.is_some() {
                        for tag in [
                            VectorKind::IdentityMapping,
                            VectorKind::Challenge,
                            VectorKind::MintProof,
                            VectorKind::TransferProof,
                        ] {
                            assert!(
                                success.files.iter().any(|file| file.kind == Some(tag)),
                                "a real concrete emits the tagged file"
                            );
                        }
                        for file in &success.files {
                            assert!(file.path.exists(), "a real concrete's path exists");
                        }
                    }
                    assert_eq!(success.measurement.iterations.get(), 3);
                    assert_eq!(success.measurement.costs.mint, expected_cost);
                    assert_eq!(success.measurement.costs.transfer, expected_cost);
                }
            }
        }
    }
    // the_files_and_the_measurement_reach_the_caller_of_create_delivery_proof_whole
}

/// Contract: own entry
///   `the_values_the_consumer_carries_reach_the_generate_step_as_the_records_its_files_hold` —
///   `create_delivery_proof` and a consumer constructed with a count of
///   identities other than the drawn kinds' configured count, a mint context
///   over `MintPurpose::Replacement`, and a transfer context over
///   `TransferPurpose::Grant` → `Ok` whose output is `Ok` and whose file
///   tagged `Some(VectorKind::IdentityMapping)` holds one line for each
///   identity the consumer was constructed with and whose file tagged
///   `Some(VectorKind::Challenge)` holds lines that each decode to a
///   `ChallengeVector` whose `purpose` is `4` for as many lines as it is `3`,
///   that number positive, and is neither `1` nor `2` for any line.
/// Arrange: a count of identities of one and of three, a case payload of that
///   many canonicals and that many roots, over a member of `GENERATE_CONCRETES`
///   and every member of `PAIRING_CONCRETES`; the other families' selections
///   the declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_delivery_proof`, once per case, inside the case consumer.
/// Assert:  in place inside the case consumer — the file tagged
///   `Some(VectorKind::IdentityMapping)` holds that many lines and each line
///   of the file tagged `Some(VectorKind::Challenge)`, its hex after `0x` read
///   back into bytes, decodes through the received decoder's `decode` over
///   `DecodeParams { description: &description }` with `description` the
///   `ChallengeVectorDescription` built by its `try_new` unpacked irrefutably,
///   and the `described` `ChallengeVector`s' `purpose` is `4` in as many lines
///   as it is `3`, that number positive, and neither `1` nor `2` in any line;
///   the block asserts the output is `Ok` holding `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_delivery_proof`, the selected concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, the selected generate concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, and `emit`.
/// Mocked:  nothing; the filesystem and the operating system's generator are
///   the outer edges.
#[test]
fn the_values_the_consumer_carries_reach_the_generate_step_as_the_records_its_files_hold() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(
                    MintPurpose::Replacement,
                    Default::default(),
                ),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Grant,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    ..Default::default()
                },
            );
            let result = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(created) = &result else {
                panic!("the factory admits the concrete")
            };
            let Ok(success) = &created.output else {
                panic!("the consumer emits and measures")
            };
            let mapping = success
                .files
                .iter()
                .find(|file| file.kind == Some(VectorKind::IdentityMapping))
                .expect("the emit returns the identity-mapping file");
            let text = std::fs::read_to_string(&mapping.path).expect("the emitted file reads");
            assert_eq!(text.lines().count(), payload.identities.len());
            let challenge = success
                .files
                .iter()
                .find(|file| file.kind == Some(VectorKind::Challenge))
                .expect("the emit returns the challenge file");
            let text = std::fs::read_to_string(&challenge.path).expect("the emitted file reads");
            let Ok(description) =
                ChallengeVectorDescription::try_new(ChallengeVectorDescriptionConstructorParams);
            let purposes: Vec<u16> = text
                .lines()
                .map(|line| {
                    let hex = line
                        .strip_prefix("0x")
                        .expect("a vector line carries the 0x prefix");
                    let bytes: Vec<u8> = (0..hex.len())
                        .step_by(2)
                        .map(|index| {
                            u8::from_str_radix(&hex[index..index + 2], 16)
                                .expect("every written line is hex")
                        })
                        .collect();
                    payload
                        .decoder
                        .decode(
                            DecodeParams {
                                description: &description,
                            },
                            &bytes,
                        )
                        .expect("each line decodes")
                        .described
                        .purpose
                })
                .collect();
            let replacements = purposes.iter().filter(|purpose| **purpose == 4).count();
            let grants = purposes.iter().filter(|purpose| **purpose == 3).count();
            assert_eq!(replacements, grants);
            assert!(replacements > 0);
            assert!(
                purposes
                    .iter()
                    .all(|purpose| *purpose != 1 && *purpose != 2)
            );
            result
        }
    }

    let canonicals: [&[u8]; 3] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
        b"example-package@3.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let third_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x22; 32]),
    });
    let roots = [&first_root, &second_root, &third_root];

    for kind in GENERATE_CONCRETES {
        for pairing_concrete in PAIRING_CONCRETES {
            for count in [1usize, 3usize] {
                let consumer = Consumer {
                    generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                    kinds: build_generate_kind_configurations(
                        *kind,
                        GenerateKindConfigurationsOverrides,
                    ),
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                        kem: Some(DECLARED_KEM_SELECTIONS[0]),
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
                    });

                // Act
                let result = run_create_generate_case(
                    &build_create_generate_case_deps(consumer, Default::default()),
                    case_params,
                    build_create_generate_case_payload(
                        &canonicals[..count],
                        &roots[..count],
                        Default::default(),
                    ),
                );

                // Assert
                let output = result.expect("the case composes").output;
                assert!(matches!(
                    output,
                    Ok(CreateDeliveryProofSuccessReturn { output: Ok(_) })
                ));
            }
        }
    }
    // the_values_the_consumer_carries_reach_the_generate_step_as_the_records_its_files_hold
}

/// Contract: own entry
///   `the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports` —
///   `create_delivery_proof` and a consumer constructed with an iteration
///   count other than the builder's default, the mock generate selection under
///   `NoFailure`, and the verifier family's mock concrete under `NoFailure` →
///   `Ok` whose output is `Ok(DeliveryProofConsumerSuccessReturn)` with
///   `measurement.iterations` equal to the count the consumer was constructed
///   with.
/// Arrange: an iteration count of three and of one, over every member of
///   `PAIRING_CONCRETES` and each scope of `DECLARED_KEM_SELECTIONS`; the
///   other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_delivery_proof`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`
///   and `measurement.iterations.get()` equals the count.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_delivery_proof`, the selected concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, the mock concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, `emit`,
///   `create_verifier`, `VerifierConsumer::try_new`, `consume_verifier`, and
///   `measure`.
/// Mocked:  the generate and verifier families' mock concretes, the operating
///   system's generator and monotonic clock running real.
#[test]
fn the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports() {
    // Arrange
    struct Consumer {
        iterations: NonZeroU32,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    iterations: Some(self.iterations),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for count in [3u32, 1] {
                let iterations = NonZeroU32::new(count).expect("the count is nonzero");
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                            concrete: EncodingConcrete::Mock,
                            identifier: EncodingIdentifier::Mock,
                        }),
                        chain_forms: None,
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
                    });

                // Act
                let result = run_create_generate_case(
                    &build_create_generate_case_deps(Consumer { iterations }, Default::default()),
                    case_params,
                    build_create_generate_case_payload(&canonicals, &roots, Default::default()),
                );

                // Assert
                let success = result
                    .expect("the case composes")
                    .output
                    .expect("the factory admits the concrete and runs the consumer")
                    .output
                    .expect("the consumer emits and measures");
                assert_eq!(success.measurement.iterations, iterations);
            }
        }
    }
    // the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports
}

/// Contract: own entry
///   `the_statement_version_the_consumer_carries_reaches_the_verifier_factory` —
///   `create_delivery_proof` and a consumer constructed with
///   `statement_version` `DELIVERY_STATEMENT_VERSION_ONE` and with `2`, the
///   mock generate selection under `NoFailure`, and the verifier family's mock
///   concrete under `NoFailure` → `Ok` whose output is `Ok` for version one
///   and `Err(DeliveryProofConsumerErrorReturn::CreateVerifier(
///   CreateVerifierErrorReturn::UnsupportedStatementVersion { version: 2 }))`
///   for `2`, asserted by a pattern naming every field because
///   `DeliveryProofConsumerErrorReturn` omits `PartialEq`.
/// Arrange: the two versions over every member of `PAIRING_CONCRETES`; the
///   other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, each scope of
///   `DECLARED_KEM_SELECTIONS`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_delivery_proof`, once per case, inside the case consumer.
/// Assert:  the output under version one is `Ok` holding `Ok`, and under `2`
///   matches `Ok(CreateDeliveryProofSuccessReturn { output:
///   Err(DeliveryProofConsumerErrorReturn::CreateVerifier(
///   CreateVerifierErrorReturn::UnsupportedStatementVersion { version: 2 })) })`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_delivery_proof`, the selected concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, the mock concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, `emit`,
///   `create_verifier`, `VerifierConsumer::try_new`, `consume_verifier`, and
///   `measure`.
/// Mocked:  the generate and verifier families' mock concretes.
#[test]
fn the_statement_version_the_consumer_carries_reaches_the_verifier_factory() {
    // Arrange
    struct Consumer {
        statement_version: u16,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    statement_version: Some(self.statement_version),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for statement_version in [DELIVERY_STATEMENT_VERSION_ONE, 2] {
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                            concrete: EncodingConcrete::Mock,
                            identifier: EncodingIdentifier::Mock,
                        }),
                        chain_forms: None,
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
                    });

                // Act
                let result = run_create_generate_case(
                    &build_create_generate_case_deps(
                        Consumer { statement_version },
                        Default::default(),
                    ),
                    case_params,
                    build_create_generate_case_payload(&canonicals, &roots, Default::default()),
                );

                // Assert
                let output = result.expect("the case composes").output;
                match statement_version {
                    DELIVERY_STATEMENT_VERSION_ONE => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn { output: Ok(_) })
                    )),
                    _ => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn {
                            output: Err(DeliveryProofConsumerErrorReturn::CreateVerifier(
                                CreateVerifierErrorReturn::UnsupportedStatementVersion {
                                    version: 2
                                }
                            )),
                        })
                    )),
                }
            }
        }
    }
    // the_statement_version_the_consumer_carries_reaches_the_verifier_factory
}

/// Contract: own entry `a_refused_write_returns_before_the_verifier_is_reached` —
///   `create_delivery_proof` and a consumer constructed with
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`
///   and `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)` →
///   `Ok` whose output is `Err(DeliveryProofConsumerErrorReturn::Emit(
///   EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::WriteRefused)))`,
///   asserted by a pattern naming every field.
/// Arrange: the generate failure mode against `NoFailure`, under which the
///   output is `Err(DeliveryProofConsumerErrorReturn::Measure(
///   MeasureErrorReturn::OnChain { relation: DeliveryRelation::Mint, error:
///   VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))`, over
///   every member of `PAIRING_CONCRETES`; the other families' selections the
///   declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, each scope of
///   `DECLARED_KEM_SELECTIONS`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_delivery_proof`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateDeliveryProofSuccessReturn { output:
///   Err(DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused))) })` and the output
///   under `NoFailure` matches the measurement refusal above.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_delivery_proof`, the selected concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`, and
///   `create_generate`.
/// Mocked:  the generate and verifier families' mock concretes under their
///   failure modes.
#[test]
fn a_refused_write_returns_before_the_verifier_is_reached() {
    // Arrange
    struct Consumer {
        mode: MockIGenerateAdapterFailureMode,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    generate: Some(GenerateConcrete::Mock(self.mode)),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for mode in [
                MockIGenerateAdapterFailureMode::WriteRefused,
                MockIGenerateAdapterFailureMode::NoFailure,
            ] {
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                            concrete: EncodingConcrete::Mock,
                            identifier: EncodingIdentifier::Mock,
                        }),
                        chain_forms: None,
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
                    });

                // Act
                let result = run_create_generate_case(
                    &build_create_generate_case_deps(Consumer { mode }, Default::default()),
                    case_params,
                    build_create_generate_case_payload(&canonicals, &roots, Default::default()),
                );

                // Assert
                let output = result.expect("the case composes").output;
                match mode {
                    MockIGenerateAdapterFailureMode::WriteRefused => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn {
                            output: Err(DeliveryProofConsumerErrorReturn::Emit(
                                EmitErrorReturn::Mock(
                                    MockIGenerateAdapterEmitErrorReturn::WriteRefused
                                )
                            )),
                        })
                    )),
                    MockIGenerateAdapterFailureMode::NoFailure => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn {
                            output: Err(DeliveryProofConsumerErrorReturn::Measure(
                                MeasureErrorReturn::OnChain {
                                    relation: DeliveryRelation::Mint,
                                    error: VerifyOnChainErrorReturn::Mock(
                                        MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                    ),
                                }
                            )),
                        })
                    )),
                }
            }
        }
    }
    // a_refused_write_returns_before_the_verifier_is_reached
}

/// Contract: own entry
///   `a_refused_chain_read_returns_after_the_generate_step_succeeded` —
///   `create_delivery_proof` and a consumer constructed with a member of
///   `GENERATE_CONCRETES` and `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` → `Ok` whose output is
///   `Err(DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))`, asserted
///   by a pattern naming every field.
/// Arrange: every member of `GENERATE_CONCRETES` with the refusing mode, then
///   `NoFailure` as the counterfactual, over every member of
///   `PAIRING_CONCRETES`; the other families' selections the declared sets'
///   first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, each scope of
///   `DECLARED_KEM_SELECTIONS`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_delivery_proof`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateDeliveryProofSuccessReturn { output:
///   Err(DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })) })` and the
///   output under `NoFailure` holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_delivery_proof`, the selected concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, the selected generate concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, `emit`,
///   `create_verifier`, `VerifierConsumer::try_new`, `consume_verifier`, and
///   `measure`.
/// Mocked:  the verifier family's mock concrete under its failure mode.
#[test]
fn a_refused_chain_read_returns_after_the_generate_step_succeeded() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
        mode: MockIVerifierAdapterFailureMode,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    verifier: Some(VerifierConcrete::Mock(self.mode)),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for kind in GENERATE_CONCRETES {
        for pairing_concrete in PAIRING_CONCRETES {
            for mode in [
                MockIVerifierAdapterFailureMode::RpcRefused,
                MockIVerifierAdapterFailureMode::NoFailure,
            ] {
                let consumer = Consumer {
                    generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                    kinds: build_generate_kind_configurations(
                        *kind,
                        GenerateKindConfigurationsOverrides,
                    ),
                    mode,
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                        kem: Some(DECLARED_KEM_SELECTIONS[0]),
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
                    });

                // Act
                let result = run_create_generate_case(
                    &build_create_generate_case_deps(consumer, Default::default()),
                    case_params,
                    build_create_generate_case_payload(&canonicals, &roots, Default::default()),
                );

                // Assert
                let output = result.expect("the case composes").output;
                match mode {
                    MockIVerifierAdapterFailureMode::RpcRefused => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn {
                            output: Err(DeliveryProofConsumerErrorReturn::Measure(
                                MeasureErrorReturn::OnChain {
                                    relation: DeliveryRelation::Mint,
                                    error: VerifyOnChainErrorReturn::Mock(
                                        MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                    ),
                                }
                            )),
                        })
                    )),
                    MockIVerifierAdapterFailureMode::NoFailure => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn { output: Ok(_) })
                    )),
                }
            }
        }
    }
    // a_refused_chain_read_returns_after_the_generate_step_succeeded
}

/// Contract: own entry
///   `a_refused_draw_reaches_the_caller_of_create_delivery_proof_whole_after_the_generate_step_succeeds` —
///   `create_delivery_proof` and a consumer constructed with the source
///   `create_random_source` returns for `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, selected through
///   the case params, the mock generate selection under `NoFailure`, which
///   draws nothing, and the verifier family's mock concrete under `NoFailure` →
///   `Ok` whose output is `Err(DeliveryProofConsumerErrorReturn::Measure(
///   MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))))`, asserted by a pattern
///   naming every field.
/// Arrange: the source the case params select, `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, then the
///   operating-system source as the counterfactual, each scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`; the
///   other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `create_delivery_proof`, once per source, inside the case consumer.
/// Assert:  the output under the refusing source matches
///   `Ok(CreateDeliveryProofSuccessReturn { output:
///   Err(DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)))) })`
///   and the output under the operating-system source holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_delivery_proof`, the selected concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, `GenerateConsumer::try_new`, `consume_generate`,
///   `emit`, `create_verifier`, `VerifierConsumer::try_new`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the randomness family's mock concrete, selected by configuration.
#[test]
fn a_refused_draw_reaches_the_caller_of_create_delivery_proof_whole_after_the_generate_step_succeeds()
 {
    // Arrange
    struct Consumer;

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for kind in [
                RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                RandomSourceKind::OperatingSystem,
            ] {
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                            concrete: EncodingConcrete::Mock,
                            identifier: EncodingIdentifier::Mock,
                        }),
                        chain_forms: None,
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
                    });

                // Act
                let result = run_create_generate_case(
                    &build_create_generate_case_deps(Consumer, Default::default()),
                    case_params,
                    build_create_generate_case_payload(&canonicals, &roots, Default::default()),
                );

                // Assert
                let output = result.expect("the case composes").output;
                match kind {
                    RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    ) => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn {
                            output: Err(DeliveryProofConsumerErrorReturn::Measure(
                                MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
                                    FillBytesErrorReturn::MockIRandomSourceAdapter
                                ))
                            )),
                        })
                    )),
                    RandomSourceKind::OperatingSystem => assert!(matches!(
                        output,
                        Ok(CreateDeliveryProofSuccessReturn { output: Ok(_) })
                    )),
                    _ => panic!("the case names a source the block arranges"),
                }
            }
        }
    }
    // a_refused_draw_reaches_the_caller_of_create_delivery_proof_whole_after_the_generate_step_succeeds
}
