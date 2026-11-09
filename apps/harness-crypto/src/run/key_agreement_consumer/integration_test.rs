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
use envelope::{
    CreateKeyAgreementParams, CreateKeyAgreementParamsOverrides, CreateKeyAgreementReturn,
    CreateKeyAgreementSuccessReturn, DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier, IHashToScalarAdapter,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{DECLARED_KEM_SELECTIONS, ICredentialKemAdapter};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofErrorReturn, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryProofConcrete, DeliveryRelation, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::interface::{KeyAgreementConsumerErrorReturn, KeyAgreementConsumerReturn};
use super::mock::{KeyAgreementConsumerConstructorParamsOverrides, build_key_agreement_consumer};
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};
use crate::generate::provides::{
    ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    CreateGenerateCaseParamsOverrides, EmitErrorReturn, GENERATE_CONCRETES, GenerateConcrete,
    GenerateConcreteKind, GenerateConcreteOverrides, GenerateKindConfigurationsOverrides,
    ICreateGenerateCaseConsumer, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, VectorKind, VectorKindConfiguration,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, build_create_generate_params, build_generate_concrete,
    build_generate_kind_configurations, run_create_generate_case,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::run::provides::DeliveryProofConsumerErrorReturn;
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyCost, VerifyOnChainErrorReturn,
};

/// Contract: own entry
///   `the_files_and_the_measurement_reach_the_caller_of_create_key_agreement_whole` —
///   `create_key_agreement` over the mock selection and each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, with a consumer constructed from
///   the adapters, typed contexts, and identities the case runner hands, a
///   member of `DELIVERY_PROOF_CONCRETES` or the mock delivery proof
///   selection, a member of `GENERATE_CONCRETES` or the mock generate
///   selection under `NoFailure`, and `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::NoFailure)` → `Ok` whose output is
///   `Ok(DeliveryProofConsumerSuccessReturn)` whose `files` name no path
///   twice, include for a real generate concrete files tagged
///   `Some(VectorKind::IdentityMapping)`, `Some(VectorKind::Challenge)`,
///   `Some(VectorKind::MintProof)`, and `Some(VectorKind::TransferProof)`
///   each of whose paths exists on the filesystem, and whose
///   `measurement.iterations` equals the held count and
///   `measurement.costs.mint` and `measurement.costs.transfer` each equal
///   the literal cost the verifier family's mock concrete returns.
/// Arrange: every member of `GENERATE_CONCRETES` and the mock generate
///   selection, crossed with every member of `PAIRING_CONCRETES`, each scope
///   of `DECLARED_KEM_SELECTIONS`, every delivery proof selection, and every
///   key agreement selection; the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the other families'
///   selections the declared sets' first members, the random source
///   `RandomSourceKind::OperatingSystem`, an iteration count of three, and
///   the expected cost the literal gas `100_000` and fee `L1DataFee::Charged
///   { wei: 1_000 }`.
/// Act:     `create_key_agreement`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four
///   tagged files and each of its paths exists, `measurement.iterations`
///   equals the held count, and the two costs equal the expected cost.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, the selected generate
///   concrete's `try_new`, `GenerateConsumer::try_new`, `consume_generate`,
///   `emit`, `create_verifier`, `VerifierConsumer::try_new`,
///   `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_files_and_the_measurement_reach_the_caller_of_create_key_agreement_whole() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
        delivery_proof: DeliveryProofConcrete,
        key_agreement: Option<CreateKeyAgreementParams>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    delivery_proof: Some(self.delivery_proof),
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    iterations: NonZeroU32::new(3),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                match self.key_agreement {
                    Some(selection) => {
                        build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                            concrete: Some(selection.concrete),
                            identifier: Some(selection.identifier),
                            algebra: Some(selection.algebra),
                        })
                    }
                    None => build_create_key_agreement_params(Default::default()),
                },
                build_create_key_agreement_payload(Default::default()),
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
    let generate_selections: Vec<Option<GenerateConcreteKind>> = GENERATE_CONCRETES
        .iter()
        .map(|kind| Some(*kind))
        .chain(core::iter::once(None))
        .collect();
    let delivery_proofs: Vec<DeliveryProofConcrete> = DELIVERY_PROOF_CONCRETES
        .iter()
        .copied()
        .chain(core::iter::once(
            build_create_delivery_proof_params(Default::default()).concrete,
        ))
        .collect();
    let key_agreements: Vec<Option<CreateKeyAgreementParams>> = DECLARED_KEY_AGREEMENT_SELECTIONS
        .iter()
        .map(|selection| Some(*selection))
        .chain(core::iter::once(None))
        .collect();

    for selection in &generate_selections {
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
                for delivery_proof in &delivery_proofs {
                    for key_agreement in &key_agreements {
                        let consumer = Consumer {
                            generate: generate.clone(),
                            kinds: kinds.clone(),
                            delivery_proof: *delivery_proof,
                            key_agreement: *key_agreement,
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
                                            selection.identifier
                                                == HashToScalarIdentifier::Keccak256V1
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
                                    build_create_chain_forms_params(
                                        CreateChainFormsParamsOverrides {
                                            concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                                            ..Default::default()
                                        },
                                    )
                                }),
                                kem: Some(*kem_selection),
                                key_agreement: Some(DECLARED_KEY_AGREEMENT_SELECTIONS[0]),
                                delivery_proof: Some(build_create_delivery_proof_params(
                                    CreateDeliveryProofParamsOverrides {
                                        concrete: Some(*delivery_proof),
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
                                &canonicals,
                                &roots,
                                Default::default(),
                            ),
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
    }
    // the_files_and_the_measurement_reach_the_caller_of_create_key_agreement_whole
}

/// Contract: own entry
///   `the_values_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold` —
///   `create_key_agreement` and a consumer constructed with a count of
///   identities other than the drawn kinds' configured count, a mint context
///   over `MintPurpose::Replacement`, and a transfer context over
///   `TransferPurpose::Grant` → `Ok` whose output is `Ok` and whose file
///   tagged `Some(VectorKind::IdentityMapping)` holds one line for each
///   identity the consumer was constructed with and whose file tagged
///   `Some(VectorKind::Challenge)` holds lines that each decode to a
///   `ChallengeVector` whose `purpose` is `4` for as many lines as it is
///   `3`, that number positive, and is neither `1` nor `2` for any line.
/// Arrange: a count of identities of one and of three, a case payload of
///   that many canonicals and that many roots, over a member of
///   `GENERATE_CONCRETES` and every member of `PAIRING_CONCRETES`; the other
///   families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_key_agreement`, once per case, inside the case consumer.
/// Assert:  in place inside the case consumer — the file tagged
///   `Some(VectorKind::IdentityMapping)` holds that many lines and each line
///   of the file tagged `Some(VectorKind::Challenge)`, its hex after `0x`
///   read back into bytes, decodes through the received decoder's `decode`
///   over `DecodeParams { description: &description }` with `description`
///   the `ChallengeVectorDescription` built by its `try_new` unpacked
///   irrefutably, and the `described` `ChallengeVector`s' `purpose` is `4`
///   in as many lines as it is `3`, that number positive, and neither `1`
///   nor `2` in any line; the block asserts the output is `Ok` holding `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, the selected generate
///   concrete's `try_new`, `GenerateConsumer::try_new`, `consume_generate`,
///   and `emit`.
/// Mocked:  nothing; the filesystem and the operating system's generator are
///   the outer edges.
#[test]
fn the_values_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold()
{
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(
                    MintPurpose::Replacement,
                    Default::default(),
                ),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Grant,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    ..Default::default()
                },
            );
            let result = create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
            assert_eq!(text.lines().count(), identities.len());
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
                    decoder
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
                    Ok(CreateKeyAgreementSuccessReturn { output: Ok(_) })
                ));
            }
        }
    }
    // the_values_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
}

/// Contract: own entry
///   `the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports` —
///   `create_key_agreement` and a consumer constructed with an iteration
///   count other than the builder's default, the mock generate selection
///   under `NoFailure`, and the verifier family's mock concrete under
///   `NoFailure` → `Ok` whose output is `Ok(DeliveryProofConsumerSuccessReturn)`
///   with `measurement.iterations` equal to the count the consumer was
///   constructed with.
/// Arrange: an iteration count of three and of one, over every member of
///   `PAIRING_CONCRETES` and each scope of `DECLARED_KEM_SELECTIONS`; the
///   other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_key_agreement`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`
///   and `measurement.iterations.get()` equals the count.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, the mock concrete's
///   `try_new`, `GenerateConsumer::try_new`, `consume_generate`, `emit`,
///   `create_verifier`, `VerifierConsumer::try_new`, `consume_verifier`, and
///   `measure`.
/// Mocked:  the generate and verifier families' mock concretes, the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports() {
    // Arrange
    struct Consumer {
        iterations: NonZeroU32,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    iterations: Some(self.iterations),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
///   `the_statement_version_the_consumer_carries_reaches_the_delivery_proof_factory` —
///   `create_key_agreement` and a consumer constructed with
///   `statement_version` `DELIVERY_STATEMENT_VERSION_ONE` and with `2`, over
///   each member of `DELIVERY_PROOF_CONCRETES` and the mock delivery proof
///   selection, the mock generate selection under `NoFailure`, and the
///   verifier family's mock concrete under `NoFailure` → `Ok` whose output
///   is `Ok` for version one and
///   `Err(KeyAgreementConsumerErrorReturn::CreateDeliveryProof(
///   CreateDeliveryProofErrorReturn::UnsupportedStatementVersion))` for `2`.
/// Arrange: the two versions over every member of `PAIRING_CONCRETES`, each
///   delivery proof selection, and the algebra each key agreement selection
///   declares; the other families' selections the declared sets' first
///   members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, each scope of
///   `DECLARED_KEM_SELECTIONS`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_key_agreement`, once per case, inside the case consumer.
/// Assert:  the output under version one is `Ok` holding `Ok`, and under `2`
///   matches `Ok(CreateKeyAgreementSuccessReturn { output:
///   Err(KeyAgreementConsumerErrorReturn::CreateDeliveryProof(
///   CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)) })`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`, and
///   `create_delivery_proof`.
/// Mocked:  the generate and verifier families' mock concretes.
#[test]
fn the_statement_version_the_consumer_carries_reaches_the_delivery_proof_factory() {
    // Arrange
    struct Consumer {
        statement_version: u16,
        delivery_proof: DeliveryProofConcrete,
        key_agreement: Option<CreateKeyAgreementParams>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    delivery_proof: Some(self.delivery_proof),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    statement_version: Some(self.statement_version),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                match self.key_agreement {
                    Some(selection) => {
                        build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                            concrete: Some(selection.concrete),
                            identifier: Some(selection.identifier),
                            algebra: Some(selection.algebra),
                        })
                    }
                    None => build_create_key_agreement_params(Default::default()),
                },
                build_create_key_agreement_payload(Default::default()),
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
    let delivery_proofs: Vec<DeliveryProofConcrete> = DELIVERY_PROOF_CONCRETES
        .iter()
        .copied()
        .chain(core::iter::once(
            build_create_delivery_proof_params(Default::default()).concrete,
        ))
        .collect();
    let key_agreements: Vec<Option<CreateKeyAgreementParams>> = DECLARED_KEY_AGREEMENT_SELECTIONS
        .iter()
        .map(|selection| Some(*selection))
        .chain(core::iter::once(None))
        .collect();

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            for delivery_proof in &delivery_proofs {
                for key_agreement in &key_agreements {
                    for statement_version in [DELIVERY_STATEMENT_VERSION_ONE, 2] {
                        let consumer = Consumer {
                            statement_version,
                            delivery_proof: *delivery_proof,
                            key_agreement: *key_agreement,
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
                                            selection.identifier
                                                == HashToScalarIdentifier::Keccak256V1
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
                                        concrete: Some(*delivery_proof),
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
                                &canonicals,
                                &roots,
                                Default::default(),
                            ),
                        );

                        // Assert
                        let output = result.expect("the case composes").output;
                        match statement_version {
                            DELIVERY_STATEMENT_VERSION_ONE => assert!(matches!(
                                output,
                                Ok(CreateKeyAgreementSuccessReturn { output: Ok(_) })
                            )),
                            _ => assert!(matches!(
                                output,
                                Ok(CreateKeyAgreementSuccessReturn {
                                    output: Err(
                                        KeyAgreementConsumerErrorReturn::CreateDeliveryProof(
                                            CreateDeliveryProofErrorReturn::UnsupportedStatementVersion
                                        )
                                    ),
                                })
                            )),
                        }
                    }
                }
            }
        }
    }
    // the_statement_version_the_consumer_carries_reaches_the_delivery_proof_factory
}

/// Contract: own entry
///   `the_delivery_proof_selection_the_consumer_carries_is_the_concrete_the_factory_constructs` —
///   `create_key_agreement` and a consumer constructed with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NoFailure)`
///   and with `DeliveryProofConcrete::Mock(
///   MockIDeliveryProofAdapterFailureMode::NonceDrawRefused)`, the mock
///   generate selection under `NoFailure`, and the verifier family's mock
///   concrete under `NoFailure` → `Ok` whose output is `Ok` under
///   `NoFailure` and `Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))))`
///   under `NonceDrawRefused`, asserted by a pattern naming every field.
/// Arrange: each failure mode over every member of `PAIRING_CONCRETES` and
///   each scope of `DECLARED_KEM_SELECTIONS`; the other families' selections
///   the declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_key_agreement`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)))))) })`
///   and the output under `NoFailure` holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, the mock concrete's `try_new`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, `consume_generate`, `emit`, `create_verifier`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the delivery proof, generate, and verifier families' mock
///   concretes.
#[test]
fn the_delivery_proof_selection_the_consumer_carries_is_the_concrete_the_factory_constructs() {
    // Arrange
    struct Consumer {
        mode: MockIDeliveryProofAdapterFailureMode,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    delivery_proof: Some(DeliveryProofConcrete::Mock(self.mode)),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
                MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                MockIDeliveryProofAdapterFailureMode::NoFailure,
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
                    MockIDeliveryProofAdapterFailureMode::NonceDrawRefused => {
                        assert!(matches!(
                            output,
                            Ok(CreateKeyAgreementSuccessReturn {
                                output: Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
                                    DeliveryProofConsumerErrorReturn::Measure(
                                        MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
                                            ProveMintErrorReturn::Mock(
                                                MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                                            )
                                        ))
                                    )
                                )),
                            })
                        ));
                    }
                    MockIDeliveryProofAdapterFailureMode::NoFailure => assert!(matches!(
                        output,
                        Ok(CreateKeyAgreementSuccessReturn { output: Ok(_) })
                    )),
                }
            }
        }
    }
    // the_delivery_proof_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
}

/// Contract: own entry
///   `a_refused_write_returns_through_the_consumer_before_the_verifier_is_reached` —
///   `create_key_agreement` and a consumer constructed with
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`
///   and `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)` →
///   `Ok` whose output is `Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused))))`, asserted by a
///   pattern naming every field.
/// Arrange: the generate failure mode against `NoFailure`, under which the
///   output is `Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })))`, over
///   every member of `PAIRING_CONCRETES`; the other families' selections the
///   declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, each scope of
///   `DECLARED_KEM_SELECTIONS`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_key_agreement`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused)))) })` and the
///   output under `NoFailure` matches the measurement refusal above.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, and `create_generate`.
/// Mocked:  the generate and verifier families' mock concretes under their
///   failure modes.
#[test]
fn a_refused_write_returns_through_the_consumer_before_the_verifier_is_reached() {
    // Arrange
    struct Consumer {
        mode: MockIGenerateAdapterFailureMode,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    generate: Some(GenerateConcrete::Mock(self.mode)),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
                        Ok(CreateKeyAgreementSuccessReturn {
                            output: Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
                                    MockIGenerateAdapterEmitErrorReturn::WriteRefused
                                ))
                            )),
                        })
                    )),
                    MockIGenerateAdapterFailureMode::NoFailure => assert!(matches!(
                        output,
                        Ok(CreateKeyAgreementSuccessReturn {
                            output: Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::Measure(
                                    MeasureErrorReturn::OnChain {
                                        relation: DeliveryRelation::Mint,
                                        error: VerifyOnChainErrorReturn::Mock(
                                            MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                        ),
                                    }
                                )
                            )),
                        })
                    )),
                }
            }
        }
    }
    // a_refused_write_returns_through_the_consumer_before_the_verifier_is_reached
}

/// Contract: own entry
///   `a_refused_chain_read_returns_through_the_consumer_after_the_generate_step_succeeded` —
///   `create_key_agreement` and a consumer constructed with a member of
///   `GENERATE_CONCRETES` and `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` → `Ok` whose output is
///   `Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })))`,
///   asserted by a pattern naming every field.
/// Arrange: every member of `GENERATE_CONCRETES` with the refusing mode,
///   then `NoFailure` as the counterfactual, over every member of
///   `PAIRING_CONCRETES`; the other families' selections the declared sets'
///   first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, each scope of
///   `DECLARED_KEM_SELECTIONS`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_key_agreement`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))) })` and
///   the output under `NoFailure` holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, the selected generate
///   concrete's `try_new`, `GenerateConsumer::try_new`, `consume_generate`,
///   `emit`, `create_verifier`, `VerifierConsumer::try_new`,
///   `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete under its failure mode.
#[test]
fn a_refused_chain_read_returns_through_the_consumer_after_the_generate_step_succeeded() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
        mode: MockIVerifierAdapterFailureMode,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    verifier: Some(VerifierConcrete::Mock(self.mode)),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
                        Ok(CreateKeyAgreementSuccessReturn {
                            output: Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::Measure(
                                    MeasureErrorReturn::OnChain {
                                        relation: DeliveryRelation::Mint,
                                        error: VerifyOnChainErrorReturn::Mock(
                                            MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                        ),
                                    }
                                )
                            )),
                        })
                    )),
                    MockIVerifierAdapterFailureMode::NoFailure => assert!(matches!(
                        output,
                        Ok(CreateKeyAgreementSuccessReturn { output: Ok(_) })
                    )),
                }
            }
        }
    }
    // a_refused_chain_read_returns_through_the_consumer_after_the_generate_step_succeeded
}

/// Contract: own entry
///   `a_refused_draw_reaches_the_caller_of_create_key_agreement_whole` —
///   `create_key_agreement` and a consumer constructed with the source
///   `create_random_source` returns for `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, selected
///   through the case params, the mock generate selection under `NoFailure`,
///   and the verifier family's mock concrete under `NoFailure` → `Ok` whose
///   output is `Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)))))`,
///   asserted by a pattern naming every field.
/// Arrange: the source the case params select, `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, then the
///   operating-system source as the counterfactual, each scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`; the
///   other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`.
/// Act:     `create_key_agreement`, once per source, inside the case
///   consumer.
/// Assert:  the output under the refusing source matches
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter))))) })`
///   and the output under the operating-system source holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_key_agreement`, the selected concrete's `try_new`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, `GenerateConsumer::try_new`,
///   `consume_generate`, `emit`, `create_verifier`,
///   `VerifierConsumer::try_new`, `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the randomness family's mock concrete, selected by
///   configuration.
#[test]
fn a_refused_draw_reaches_the_caller_of_create_key_agreement_whole() {
    // Arrange
    struct Consumer;

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
                        Ok(CreateKeyAgreementSuccessReturn {
                            output: Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::Measure(
                                    MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
                                        FillBytesErrorReturn::MockIRandomSourceAdapter
                                    ))
                                )
                            )),
                        })
                    )),
                    RandomSourceKind::OperatingSystem => assert!(matches!(
                        output,
                        Ok(CreateKeyAgreementSuccessReturn { output: Ok(_) })
                    )),
                    _ => panic!("the case names a source the block arranges"),
                }
            }
        }
    }
    // a_refused_draw_reaches_the_caller_of_create_key_agreement_whole
}
