#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use chain::{
    ChainFormsConcrete, CreateChainFormsErrorReturn, CreateChainFormsParams,
    DECLARED_CHAIN_FORMS_SELECTIONS, IChainForms, build_create_chain_forms_params,
};
use core::num::NonZeroU32;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    CreateEncodingParamsOverrides, CreateEncodingReturn, CreateEncodingSuccessReturn,
    DECLARED_ENCODING_SELECTIONS, DecodeParams, EncodingConcrete, EncodingIdentifier,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{
    CreateKeyAgreementParams, CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS,
    IKeyAgreementAdapter, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarIdentifier, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParams, DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, ICredentialKemAdapter,
    KemIdentity, SetupErrorReturn, SetupScope, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    PAIRING_CONCRETES, SampleUniformScalarErrorReturn, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use harness_crypto::{
    AdmittedChainForms, AdmittedIdentities, AdmittedIdentitiesOverrides, AdmittedIdentity,
    AdmittedKem, AdmittedKemIdentity, AdmittedKeyAgreement, AdmittedStatementsOverrides,
    ChainFormsConsumerErrorReturn, ChallengeVectorDescription,
    ChallengeVectorDescriptionConstructorParams, CreateGenerateCaseParamsOverrides,
    CreateGenerateErrorReturn, DeliveryProofConsumerErrorReturn, EncodingConsumer,
    EncodingConsumerConstructorParams, EncodingConsumerErrorReturn, EncodingConsumerReturn,
    GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteKind, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, ICreateGenerateCaseConsumer, KemConsumerErrorReturn,
    KeyAgreementConsumerErrorReturn, L1DataFee, MeasureErrorReturn, MintErrorReturn,
    MockIVerifierAdapterFailureMode, VectorCountConstructorParamsOverrides, VectorKind,
    VectorKindConfiguration, VectorKindConfigurationOverrides, VectorKindQuantitiesOverrides,
    VerifierConcrete, VerifyCost, build_admitted_identities, build_admitted_statements,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, build_create_generate_params, build_emit_success_return,
    build_generate_concrete, build_generate_kind_configurations, build_vector_count,
    build_vector_kind_configuration, build_vector_kind_quantities, run_create_generate_case,
};

fn admitted_identities_of(identities: &[AdmittedKemIdentity<'_>]) -> AdmittedIdentities {
    let converted: Vec<AdmittedIdentity> = identities
        .iter()
        .map(|admitted| match admitted.identity {
            KemIdentity::Entitlement { canonical } => AdmittedIdentity::Entitlement {
                canonical: canonical.to_vec(),
            },
            KemIdentity::Asset { root } => AdmittedIdentity::Asset { root: *root },
        })
        .collect();
    build_admitted_identities(AdmittedIdentitiesOverrides {
        proof: Some(converted[0].clone()),
        mapping: Some(converted),
    })
}

fn encoding_for(forms: CreateChainFormsParams) -> CreateEncodingParams {
    if matches!(forms.concrete, ChainFormsConcrete::Mock) {
        CreateEncodingParams {
            concrete: EncodingConcrete::Mock,
            identifier: EncodingIdentifier::Mock,
        }
    } else {
        CreateEncodingParams {
            concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
            identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
        }
    }
}

fn case_chain_forms(forms: CreateChainFormsParams) -> Option<CreateChainFormsParams> {
    if matches!(forms.concrete, ChainFormsConcrete::Mock) {
        None
    } else {
        Some(forms)
    }
}

fn keccak256_selection() -> hash_to_scalar::CreateHashToScalarParams {
    *DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .find(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
        .expect("a keccak256 selection is declared")
}

/// Contract: own entry
///   `the_files_and_the_measurement_reach_the_caller_of_create_encoding_whole`,
///   as an outside caller observes it — `create_encoding` over the mock
///   selection and each member of `DECLARED_CHAIN_FORMS_SELECTIONS` under the
///   encoding selection the member requires, with an `EncodingConsumer`
///   constructed through `EncodingConsumer::try_new` from
///   `EncodingConsumerConstructorParams` → `Ok` whose output is
///   `Ok(DeliveryProofConsumerSuccessReturn)` whose `files` name no path
///   twice, include for a real generate concrete files tagged
///   `Some(VectorKind::IdentityMapping)`, `Some(VectorKind::Challenge)`,
///   `Some(VectorKind::MintProof)`, and `Some(VectorKind::TransferProof)` each
///   of whose paths exists on the filesystem, and whose
///   `measurement.iterations` equals the held count and
///   `measurement.costs.mint` and `measurement.costs.transfer` each equal
///   the literal cost the verifier family's mock concrete returns.
/// Arrange: every member of `GENERATE_CONCRETES` and the mock generate
///   selection, each paired with the forms selections its declared encoding
///   admits — a real concrete with each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS` and the mock selection with the mock
///   forms — crossed with every member of `PAIRING_CONCRETES`, every KEM
///   selection, every delivery proof selection, and every key agreement
///   selection; the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the other families' selections
///   the declared sets' first members, the random source
///   `RandomSourceKind::OperatingSystem`, an iteration count of three, and
///   the expected cost the literal gas `100_000` and fee `L1DataFee::Charged
///   { wei: 1_000 }`.
/// Act:     `create_encoding` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four
///   tagged files and each of its paths exists, `measurement.iterations`
///   equals the held count, and the two costs equal the expected cost.
/// Boundary: the public calls `EncodingConsumer::try_new` and `create_encoding`
///   through the crate root, inside the consumer the generate factory's
///   `run_create_generate_case` hands it under the `mocks` feature, over the
///   route `run_create_generate_case`, the factories it composes,
///   `create_encoding`, the selected encoding concrete's constructor,
///   `consume_encoding`, `create_chain_forms`, the selected forms concrete's
///   producer, `consume_chain_forms`, `admit_contexts`, `create_kem`,
///   `consume_kem`, `create_key_agreement`, `consume_key_agreement`,
///   `create_delivery_proof`, `consume_delivery_proof`, `create_generate`,
///   `consume_generate`, `emit`, `create_verifier`, `consume_verifier`, and
///   `measure`.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_files_and_the_measurement_reach_the_caller_of_create_encoding_whole() {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        kem: CreateKemParams,
        key_agreement: Option<CreateKeyAgreementParams>,
        delivery_proof: DeliveryProofConcrete,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
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
            let key_agreement_params = match self.key_agreement {
                Some(selection) => {
                    build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                        algebra: Some(selection.algebra),
                    })
                }
                None => build_create_key_agreement_params(Default::default()),
            };
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: self.chain_forms,
                kem: AdmittedKem {
                    concrete: self.kem.concrete,
                    identifier: self.kem.identifier,
                    scope: self.kem.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: self.delivery_proof,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::new(3).expect("three is nonzero"),
            });
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.encoding.concrete),
                    identifier: Some(self.encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
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
    let forms_selections: Vec<CreateChainFormsParams> = DECLARED_CHAIN_FORMS_SELECTIONS
        .iter()
        .copied()
        .chain(core::iter::once(build_create_chain_forms_params(
            Default::default(),
        )))
        .collect();
    let generate_selections: Vec<Option<GenerateConcreteKind>> = GENERATE_CONCRETES
        .iter()
        .map(|kind| Some(*kind))
        .chain(core::iter::once(None))
        .collect();
    let kem_selections: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS
        .iter()
        .copied()
        .chain(core::iter::once(
            build_create_kem_params(Default::default()),
        ))
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

    for forms in &forms_selections {
        let forms_mock = matches!(forms.concrete, ChainFormsConcrete::Mock);
        for selection in &generate_selections {
            if forms_mock != selection.is_none() {
                continue;
            }
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
                for kem_selection in &kem_selections {
                    for delivery_proof in &delivery_proofs {
                        for key_agreement in &key_agreements {
                            let consumer = Consumer {
                                chain_forms: AdmittedChainForms {
                                    concrete: forms.concrete,
                                    identifier: forms.identifier,
                                },
                                encoding: encoding_for(*forms),
                                kem: *kem_selection,
                                key_agreement: *key_agreement,
                                delivery_proof: *delivery_proof,
                                generate: generate.clone(),
                                kinds: kinds.clone(),
                            };
                            let case_params = build_create_generate_case_params(
                                CreateGenerateCaseParamsOverrides {
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*pairing_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(keccak256_selection()),
                                    encoding: Some(encoding_for(*forms)),
                                    chain_forms: case_chain_forms(*forms),
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
                                },
                            );

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
    }
    // the_files_and_the_measurement_reach_the_caller_of_create_encoding_whole
}

/// Contract: own entry
///   `the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with`,
///   as an outside caller observes it — `create_encoding` over the encoding
///   selection the case names → the lines of the file tagged
///   `Some(VectorKind::Challenge)` decode to `ChallengeVector`s under the
///   decoder of the selection passed to `create_encoding` and are refused by
///   the decoder of the other encoding selection.
/// Arrange: each member of `DECLARED_CHAIN_FORMS_SELECTIONS` under the
///   encoding selection it requires, with a member of `GENERATE_CONCRETES`,
///   the mock KEM, key agreement, and delivery proof selections, and
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`,
///   over every member of `PAIRING_CONCRETES`; the other selection's decoder
///   constructed by a second `create_encoding` over the mock encoding
///   selection.
/// Act:     `create_encoding` through the crate root, once per case, inside
///   the case consumer, then `create_encoding` once for the other selection's
///   decoder.
/// Assert:  each line of the file tagged `Some(VectorKind::Challenge)`, its
///   hex after `0x` read back into bytes, decodes through the received
///   decoder's `decode` over `DecodeParams { description: &description }` with
///   `description` the `ChallengeVectorDescription` built by its `try_new`
///   unpacked irrefutably, and the same bytes decoded through the decoder of
///   the other encoding selection are refused.
/// Boundary: the public calls `EncodingConsumer::try_new` and `create_encoding`
///   through the crate root and the route the own entry states.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with() {
    // Arrange
    struct DecodeConsumer<'a> {
        lines: &'a [Vec<u8>],
        description: &'a ChallengeVectorDescription,
    }

    impl IEncodingConsumer for DecodeConsumer<'_> {
        type Output = Vec<bool>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            self.lines
                .iter()
                .map(|bytes| {
                    payload
                        .adapter
                        .decode(
                            DecodeParams {
                                description: self.description,
                            },
                            bytes,
                        )
                        .is_err()
                })
                .collect()
        }
    }

    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        other_encoding: CreateEncodingParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            decoder: &E,
            _kem: &K,
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
            let kem_params = build_create_kem_params(Default::default());
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: self.chain_forms,
                kem: AdmittedKem {
                    concrete: kem_params.concrete,
                    identifier: kem_params.identifier,
                    scope: kem_params.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            let result = create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.encoding.concrete),
                    identifier: Some(self.encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(created) = &result else {
                panic!("the factory admits the concrete")
            };
            let Ok(success) = &created.output else {
                panic!("the consumer emits and measures")
            };
            let challenge = success
                .files
                .iter()
                .find(|file| file.kind == Some(VectorKind::Challenge))
                .expect("the emit returns the challenge file");
            let text = std::fs::read_to_string(&challenge.path).expect("the emitted file reads");
            let Ok(description) =
                ChallengeVectorDescription::try_new(ChallengeVectorDescriptionConstructorParams);
            let lines: Vec<Vec<u8>> = text
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
                        .expect("each line decodes under the received decoder");
                    bytes
                })
                .collect();
            assert!(!lines.is_empty(), "the challenge file holds lines");
            let probe = DecodeConsumer {
                lines: &lines,
                description: &description,
            };
            let refused = create_encoding(
                &build_create_encoding_deps(probe),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.other_encoding.concrete),
                    identifier: Some(self.other_encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the factory admits the other selection")
            .output;
            assert!(
                refused.iter().all(|outcome| *outcome),
                "the other selection's decoder refuses the bytes"
            );
            result
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

    for forms in DECLARED_CHAIN_FORMS_SELECTIONS {
        for kind in GENERATE_CONCRETES {
            for pairing_concrete in PAIRING_CONCRETES {
                let consumer = Consumer {
                    chain_forms: AdmittedChainForms {
                        concrete: forms.concrete,
                        identifier: forms.identifier,
                    },
                    encoding: encoding_for(*forms),
                    other_encoding: build_create_encoding_params(Default::default()),
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
                        hash_to_scalar: Some(keccak256_selection()),
                        encoding: Some(encoding_for(*forms)),
                        chain_forms: case_chain_forms(*forms),
                        kem: Some(build_create_kem_params(Default::default())),
                        key_agreement: Some(build_create_key_agreement_params(Default::default())),
                        delivery_proof: Some(
                            build_create_delivery_proof_params(Default::default()),
                        ),
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
                assert!(matches!(
                    output,
                    Ok(CreateEncodingSuccessReturn { output: Ok(_) })
                ));
            }
        }
    }
    // the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with
}

/// Contract: own entry
///   `the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs`,
///   as an outside caller observes it — `create_encoding` and a consumer
///   constructed with an `AdmittedChainForms` built from a member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, and the same selection with the
///   `identifier` the mock selection declares → `Ok` whose output is `Ok` for
///   the declared member and `Err(EncodingConsumerErrorReturn::
///   CreateChainForms(CreateChainFormsErrorReturn::
///   UnsupportedChainFormsIdentifier))` for the mixed selection.
/// Arrange: the two selections over every member of `PAIRING_CONCRETES`, under
///   the encoding selection the member requires; `generate` and `kinds` the
///   member of `GENERATE_CONCRETES` and its declared kind configurations, the
///   other families' selections the mock defaults, the hash-to-scalar
///   selection a member declaring `HashToScalarIdentifier::Keccak256V1`, the
///   verifier family's mock concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_encoding` through the crate root, once per selection,
///   inside the case consumer.
/// Assert:  the output under the declared selection holds `Ok` and the
///   output under the mixed selection matches `Ok(CreateEncodingSuccessReturn
///   { output: Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier)) })`.
/// Boundary: the public calls `EncodingConsumer::try_new` and `create_encoding`
///   through the crate root and the route `run_create_generate_case`,
///   `create_encoding`, `consume_encoding`, and `create_chain_forms`; Mocked
///   the encoding family's concrete the case selects.
#[test]
fn the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs() {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
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
            let kem_params = build_create_kem_params(Default::default());
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: self.chain_forms,
                kem: AdmittedKem {
                    concrete: kem_params.concrete,
                    identifier: kem_params.identifier,
                    scope: kem_params.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.encoding.concrete),
                    identifier: Some(self.encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let mock_identifier = build_create_chain_forms_params(Default::default()).identifier;
    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for forms in DECLARED_CHAIN_FORMS_SELECTIONS {
        for pairing_concrete in PAIRING_CONCRETES {
            for identifier in [forms.identifier, mock_identifier] {
                let consumer = Consumer {
                    chain_forms: AdmittedChainForms {
                        concrete: forms.concrete,
                        identifier,
                    },
                    encoding: encoding_for(*forms),
                    generate: build_generate_concrete(
                        GENERATE_CONCRETES[0],
                        GenerateConcreteOverrides,
                    ),
                    kinds: build_generate_kind_configurations(
                        GENERATE_CONCRETES[0],
                        GenerateKindConfigurationsOverrides,
                    ),
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(keccak256_selection()),
                        encoding: Some(encoding_for(*forms)),
                        chain_forms: case_chain_forms(*forms),
                        kem: Some(build_create_kem_params(Default::default())),
                        key_agreement: Some(build_create_key_agreement_params(Default::default())),
                        delivery_proof: Some(
                            build_create_delivery_proof_params(Default::default()),
                        ),
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
                if identifier == forms.identifier {
                    assert!(matches!(
                        output,
                        Ok(CreateEncodingSuccessReturn { output: Ok(_) })
                    ));
                } else {
                    assert!(matches!(
                        output,
                        Ok(CreateEncodingSuccessReturn {
                            output: Err(EncodingConsumerErrorReturn::CreateChainForms(
                                CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier
                            )),
                        })
                    ));
                }
            }
        }
    }
    // the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
}

/// Contract: own entry
///   `an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs`,
///   as an outside caller observes it — `create_encoding` over the mock
///   encoding selection with a member of `DECLARED_CHAIN_FORMS_SELECTIONS`,
///   and over the Ethereum ABI selection with the mock forms selection → `Ok`
///   whose output is `Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier))` for each
///   pairing, no file written.
/// Arrange: the two arrangements over every member of `PAIRING_CONCRETES`, the
///   consumer's `chain_forms` the member of `DECLARED_CHAIN_FORMS_SELECTIONS`
///   under the mock encoding selection and the mock default under the
///   Ethereum ABI selection, with the mock generate selection and the verifier
///   family's mock concrete under `NoFailure`; the other families' selections
///   the mock defaults, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_encoding` through the crate root, once per pairing,
///   inside the case consumer.
/// Assert:  each output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)) })` and no
///   path the emit would name exists.
/// Boundary: the public calls `EncodingConsumer::try_new` and `create_encoding`
///   through the crate root and the route `run_create_generate_case`,
///   `create_encoding`, `consume_encoding`, and `create_chain_forms`; Mocked
///   the encoding family's concrete the case selects.
#[test]
fn an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs()
 {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
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
            let kem_params = build_create_kem_params(Default::default());
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let generate_params = build_create_generate_params(Default::default());
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: self.chain_forms,
                kem: AdmittedKem {
                    concrete: kem_params.concrete,
                    identifier: kem_params.identifier,
                    scope: kem_params.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.encoding.concrete),
                    identifier: Some(self.encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let mock_encoding = build_create_encoding_params(Default::default());
    let declared_forms = DECLARED_CHAIN_FORMS_SELECTIONS[0];
    let mock_forms = build_create_chain_forms_params(Default::default());
    let abi_encoding = encoding_for(declared_forms);
    let arrangements: Vec<(
        EncodingConcrete,
        EncodingIdentifier,
        Option<CreateChainFormsParams>,
        AdmittedChainForms,
    )> = vec![
        (
            mock_encoding.concrete,
            mock_encoding.identifier,
            None,
            AdmittedChainForms {
                concrete: declared_forms.concrete,
                identifier: declared_forms.identifier,
            },
        ),
        (
            abi_encoding.concrete,
            abi_encoding.identifier,
            Some(declared_forms),
            AdmittedChainForms {
                concrete: mock_forms.concrete,
                identifier: mock_forms.identifier,
            },
        ),
    ];
    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for (concrete, identifier, case_forms, chain_forms) in &arrangements {
        for pairing_concrete in PAIRING_CONCRETES {
            let consumer = Consumer {
                chain_forms: *chain_forms,
                encoding: build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(*concrete),
                    identifier: Some(*identifier),
                }),
            };
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(keccak256_selection()),
                    encoding: Some(build_create_encoding_params(
                        CreateEncodingParamsOverrides {
                            concrete: Some(*concrete),
                            identifier: Some(*identifier),
                        },
                    )),
                    chain_forms: *case_forms,
                    kem: Some(build_create_kem_params(Default::default())),
                    key_agreement: Some(build_create_key_agreement_params(Default::default())),
                    delivery_proof: Some(build_create_delivery_proof_params(Default::default())),
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
            assert!(matches!(
                output,
                Ok(CreateEncodingSuccessReturn {
                    output: Err(EncodingConsumerErrorReturn::CreateChainForms(
                        CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier
                    )),
                })
            ));
            for file in build_emit_success_return(Default::default()).files {
                assert!(
                    !file.path.exists(),
                    "the refusal returns before the emit writes"
                );
            }
        }
    }
    // an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs
}

/// Contract: carried entry
///   `the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold`
///   of `harness-crypto/run/chain_forms_consumer`, as an outside caller
///   observes it through this consumer — the statements and identities this
///   consumer holds reach the forms consumer it constructs, and through it
///   the files, as records.
/// Arrange: a mapping of one identity and of three, a case payload of that
///   many canonicals and that many roots, under each scope of
///   `DECLARED_KEM_SELECTIONS`, over a member of `GENERATE_CONCRETES`, each
///   member of `DECLARED_CHAIN_FORMS_SELECTIONS`, and every member of
///   `PAIRING_CONCRETES`; `AdmittedStatements` whose mint purpose is
///   `MintPurpose::Replacement` and whose transfer purpose is
///   `TransferPurpose::Grant`; the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_encoding` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  in place inside the case consumer — the file tagged
///   `Some(VectorKind::IdentityMapping)` holds that many lines and each line
///   of the file tagged `Some(VectorKind::Challenge)`, its hex after `0x`
///   read back into bytes, decodes through the received decoder's `decode`
///   over `DecodeParams { description: &description }` with `description`
///   the `ChallengeVectorDescription` built by its `try_new` unpacked
///   irrefutably, and the `described` `ChallengeVector`s' `purpose` is `4`
///   in as many lines as it is `3`, that number positive, and neither `1`
///   nor `2` in any line; the block asserts the output is `Ok` holding `Ok`.
/// Boundary: the public call `create_encoding` with this consumer through the
///   crate root and the route the disposition states.
/// Mocked:  the verifier family's mock concrete.
#[test]
fn the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold()
 {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        kem: CreateKemParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            decoder: &E,
            _kem: &K,
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
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let admitted_statements = build_admitted_statements(AdmittedStatementsOverrides {
                mint_purpose: Some(MintPurpose::Replacement),
                transfer_purpose: Some(TransferPurpose::Grant),
                ..Default::default()
            });
            let admitted_identities = admitted_identities_of(&identities);
            let mapping_count = admitted_identities.mapping.len();
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: self.chain_forms,
                kem: AdmittedKem {
                    concrete: self.kem.concrete,
                    identifier: self.kem.identifier,
                    scope: self.kem.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            let result = create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.encoding.concrete),
                    identifier: Some(self.encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
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
            assert_eq!(text.lines().count(), mapping_count);
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
        for forms in DECLARED_CHAIN_FORMS_SELECTIONS {
            for pairing_concrete in PAIRING_CONCRETES {
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    for count in [1usize, 3usize] {
                        let consumer = Consumer {
                            chain_forms: AdmittedChainForms {
                                concrete: forms.concrete,
                                identifier: forms.identifier,
                            },
                            encoding: encoding_for(*forms),
                            kem: *kem_selection,
                            generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                            kinds: build_generate_kind_configurations(
                                *kind,
                                GenerateKindConfigurationsOverrides,
                            ),
                        };
                        let case_params =
                            build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(keccak256_selection()),
                                encoding: Some(encoding_for(*forms)),
                                chain_forms: case_chain_forms(*forms),
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
                            Ok(CreateEncodingSuccessReturn { output: Ok(_) })
                        ));
                    }
                }
            }
        }
    }
    // the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
}

/// Contract: carried entry
///   `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`
///   of `harness-crypto/run/chain_forms_consumer`, as an outside caller
///   observes it through this consumer — `consume_encoding` passes the held
///   per-kind configuration to the forms consumer it constructs, so the file
///   tagged `Some(VectorKind::Possession)` holds the configured count of
///   lines.
/// Arrange: the generate selection and the per-kind configuration
///   `build_generate_kind_configurations` returns reversed, the possession
///   entry's count overridden through `build_vector_kind_configuration` and
///   `build_vector_kind_quantities` to a `build_vector_count` value other
///   than the default and than every other entry's count, and
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`,
///   over each member of `DECLARED_CHAIN_FORMS_SELECTIONS` and every member
///   of `PAIRING_CONCRETES`; the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the other families' selections
///   the declared sets' first members, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_encoding` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  the file whose `kind` is `Some(VectorKind::Possession)` is read
///   by `std::fs::read_to_string` and its line count equals the configured
///   count.
/// Boundary: the public call `create_encoding` with this consumer through the
///   crate root and the route the disposition states.
/// Mocked:  the verifier family's mock concrete.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
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
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: self.chain_forms,
                kem: AdmittedKem {
                    concrete: DECLARED_KEM_SELECTIONS[0].concrete,
                    identifier: DECLARED_KEM_SELECTIONS[0].identifier,
                    scope: DECLARED_KEM_SELECTIONS[0].scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.encoding.concrete),
                    identifier: Some(self.encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
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
        let mut kinds =
            build_generate_kind_configurations(*kind, GenerateKindConfigurationsOverrides);
        kinds.reverse();
        let expected: u32 = kinds
            .iter()
            .filter_map(|configuration| configuration.quantities.count.map(|count| count.get()))
            .max()
            .unwrap_or(0)
            .max(2)
            + 1;
        for configuration in kinds.iter_mut() {
            if configuration.kind == VectorKind::Possession {
                *configuration =
                    build_vector_kind_configuration(VectorKindConfigurationOverrides {
                        kind: Some(VectorKind::Possession),
                        quantities: Some(build_vector_kind_quantities(
                            VectorKindQuantitiesOverrides {
                                count: Some(Some(build_vector_count(
                                    VectorCountConstructorParamsOverrides {
                                        count: Some(expected),
                                    },
                                ))),
                                terms: Some(configuration.quantities.terms),
                                message_length: Some(configuration.quantities.message_length),
                            },
                        )),
                    });
            }
        }
        for forms in DECLARED_CHAIN_FORMS_SELECTIONS {
            for pairing_concrete in PAIRING_CONCRETES {
                let consumer = Consumer {
                    chain_forms: AdmittedChainForms {
                        concrete: forms.concrete,
                        identifier: forms.identifier,
                    },
                    encoding: encoding_for(*forms),
                    generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                    kinds: kinds.clone(),
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(keccak256_selection()),
                        encoding: Some(encoding_for(*forms)),
                        chain_forms: case_chain_forms(*forms),
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
                let success = result
                    .expect("the case composes")
                    .output
                    .expect("the factory admits the concrete and runs the consumer")
                    .output
                    .expect("the consumer emits and measures");
                let file = success
                    .files
                    .iter()
                    .find(|file| file.kind == Some(VectorKind::Possession))
                    .expect("the emit returns the possession file");
                let text = std::fs::read_to_string(&file.path).expect("the emitted file reads");
                assert_eq!(
                    text.lines().count(),
                    usize::try_from(expected).expect("the configured count fits in usize")
                );
            }
        }
    }
    // a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
}

/// Contract: carried entry
///   `create_generate_refuses_an_encoder_declaring_another_encoding_identifier`
///   of `harness-crypto/run/chain_forms_consumer`, as an outside caller
///   observes it through this consumer — the encoder the generate factory is
///   handed is the encoder this consumer received, and the refusal returns as
///   `EncodingConsumerErrorReturn::ChainForms(ChainFormsConsumerErrorReturn::
///   Kem(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::CreateGenerate(
///   CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)))))`.
/// Arrange: every member of `GENERATE_CONCRETES` over `EncodingConcrete::Mock`,
///   whose declared identifier is `EncodingIdentifier::Mock`, with the mock
///   forms selection, and the mock generate selection over every member of
///   `DECLARED_ENCODING_SELECTIONS` with the declared forms selection, as the
///   entry of the same name of `harness-crypto/generate/factory` arranges the
///   encoder, over every member of `PAIRING_CONCRETES`.
/// Act:     `create_encoding` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::CreateGenerate(
///   CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)))))) })`, the
///   pattern naming every field.
/// Boundary: the public call `create_encoding` with this consumer through the
///   crate root and the route the disposition states.
/// Mocked:  the encoding family's mock concrete as the encoder for the real
///   generate concretes, and the generate family's mock concrete as the
///   selection for the real encoders, selected through their factories.
#[test]
fn create_generate_refuses_an_encoder_declaring_another_encoding_identifier() {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let kem_params = build_create_kem_params(Default::default());
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: self.chain_forms,
                kem: AdmittedKem {
                    concrete: kem_params.concrete,
                    identifier: kem_params.identifier,
                    scope: kem_params.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(self.encoding.concrete),
                    identifier: Some(self.encoding.identifier),
                }),
                build_create_encoding_payload(Default::default()),
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
    let mock_forms = build_create_chain_forms_params(Default::default());

    for kind in GENERATE_CONCRETES {
        for pairing_concrete in PAIRING_CONCRETES {
            let consumer = Consumer {
                chain_forms: AdmittedChainForms {
                    concrete: mock_forms.concrete,
                    identifier: mock_forms.identifier,
                },
                encoding: build_create_encoding_params(Default::default()),
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
                    encoding: Some(CreateEncodingParams {
                        concrete: EncodingConcrete::Mock,
                        identifier: EncodingIdentifier::Mock,
                    }),
                    chain_forms: None,
                    ..Default::default()
                });

            // Act
            let result = run_create_generate_case(
                &build_create_generate_case_deps(consumer, Default::default()),
                case_params,
                build_create_generate_case_payload(&canonicals, &roots, Default::default()),
            );

            // Assert
            let output = result.expect("the case composes").output;
            assert!(matches!(
                output,
                Ok(CreateEncodingSuccessReturn {
                    output: Err(EncodingConsumerErrorReturn::ChainForms(
                        ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::KeyAgreement(
                            KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::CreateGenerate(
                                    CreateGenerateErrorReturn::UnsupportedEncodingIdentifier
                                )
                            )
                        ))
                    )),
                })
            ));
        }
    }

    for encoding in DECLARED_ENCODING_SELECTIONS {
        for pairing_concrete in PAIRING_CONCRETES {
            let generate_params = build_create_generate_params(Default::default());
            let consumer = Consumer {
                chain_forms: AdmittedChainForms {
                    concrete: DECLARED_CHAIN_FORMS_SELECTIONS[0].concrete,
                    identifier: DECLARED_CHAIN_FORMS_SELECTIONS[0].identifier,
                },
                encoding: CreateEncodingParams {
                    concrete: encoding.concrete,
                    identifier: encoding.identifier,
                },
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
            };
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    encoding: Some(CreateEncodingParams {
                        concrete: encoding.concrete,
                        identifier: encoding.identifier,
                    }),
                    chain_forms: Some(DECLARED_CHAIN_FORMS_SELECTIONS[0]),
                    ..Default::default()
                });

            // Act
            let result = run_create_generate_case(
                &build_create_generate_case_deps(consumer, Default::default()),
                case_params,
                build_create_generate_case_payload(&canonicals, &roots, Default::default()),
            );

            // Assert
            let output = result.expect("the case composes").output;
            assert!(matches!(
                output,
                Ok(CreateEncodingSuccessReturn {
                    output: Err(EncodingConsumerErrorReturn::ChainForms(
                        ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::KeyAgreement(
                            KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::CreateGenerate(
                                    CreateGenerateErrorReturn::UnsupportedEncodingIdentifier
                                )
                            )
                        ))
                    )),
                })
            ));
        }
    }
    // create_generate_refuses_an_encoder_declaring_another_encoding_identifier
}

/// Contract: carried entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`
///   of `harness-crypto/run/chain_forms_consumer`, as an outside caller
///   observes it through this consumer — `consume_encoding` returns the
///   refusal the forms consumer returns, so the refusal of the hash-to-scalar
///   adapter reaches the caller of `create_encoding` inside
///   `EncodingConsumerErrorReturn::ChainForms`, unconverted.
/// Arrange: the hash-to-scalar adapter the family's mock concrete under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused`, selected through
///   the case params, each scope of `DECLARED_KEM_SELECTIONS` over every
///   member of `PAIRING_CONCRETES`, the mock forms selection, the mock
///   generate selection, the other selections at their defaults, the
///   operating-system source.
/// Act:     `create_encoding` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  in place inside the case consumer — the output matches the
///   scope's `Ok(CreateEncodingSuccessReturn { output: Err(
///   EncodingConsumerErrorReturn::ChainForms(…)) })` with `expected`
///   `UNIFORM_BYTES_LENGTH` and `actual` `0`; the block asserts the output is
///   `Ok` holding `Err(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(_)))))))`.
/// Boundary: the public call `create_encoding` with this consumer through the
///   crate root and the route the disposition states.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
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
            let scope = identities[0].setup_scope;
            let generate_params = build_create_generate_params(Default::default());
            let chain_forms_params = build_create_chain_forms_params(Default::default());
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: AdmittedChainForms {
                    concrete: chain_forms_params.concrete,
                    identifier: chain_forms_params.identifier,
                },
                kem: AdmittedKem {
                    concrete: self.kem.concrete,
                    identifier: self.kem.identifier,
                    scope: self.kem.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            let returned = create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            match scope {
                SetupScope::Entitlement => {
                    assert!(matches!(
                        &returned,
                        Ok(CreateEncodingSuccessReturn {
                            output: Err(EncodingConsumerErrorReturn::ChainForms(
                                ChainFormsConsumerErrorReturn::Kem(
                                    KemConsumerErrorReturn::KeyAgreement(
                                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                                            DeliveryProofConsumerErrorReturn::Measure(
                                                MeasureErrorReturn::Mint(
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
                                                )
                                            )
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
                        Ok(CreateEncodingSuccessReturn {
                            output: Err(EncodingConsumerErrorReturn::ChainForms(
                                ChainFormsConsumerErrorReturn::Kem(
                                    KemConsumerErrorReturn::KeyAgreement(
                                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                                            DeliveryProofConsumerErrorReturn::Measure(
                                                MeasureErrorReturn::Mint(MintErrorReturn::Setup(
                                                    SetupErrorReturn::HashToScalar(
                                                        HashToScalarErrorReturn::Sampling(
                                                            SampleUniformScalarErrorReturn::WrongLength {
                                                                expected,
                                                                actual: 0,
                                                            }
                                                        )
                                                    )
                                                ))
                                            )
                                        )
                                    )
                                )
                            ))
                        }) if *expected
                            == <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                    ));
                }
            }
            returned
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
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                });

            // Act
            let result = run_create_generate_case(
                &build_create_generate_case_deps(
                    Consumer {
                        kem: *kem_selection,
                    },
                    Default::default(),
                ),
                case_params,
                build_create_generate_case_payload(&canonicals, &roots, Default::default()),
            );

            // Assert
            let output = result.expect("the case composes").output;
            assert!(matches!(
                output,
                Ok(CreateEncodingSuccessReturn {
                    output: Err(EncodingConsumerErrorReturn::ChainForms(
                        ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::KeyAgreement(
                            KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::Measure(
                                    MeasureErrorReturn::Mint(_)
                                )
                            )
                        ))
                    )),
                })
            ));
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: carried entry `a_nonce_draw_refusal_reaches_the_caller_whole` of
///   `harness-crypto/run/chain_forms_consumer`, as an outside caller observes
///   it through this consumer — `consume_encoding` returns the refusal the
///   forms consumer returns, so the refusal of the delivery proof's nonce
///   draw reaches the caller of `create_encoding` inside
///   `EncodingConsumerErrorReturn::ChainForms`, unconverted.
/// Arrange: the delivery proof the family's mock concrete under
///   `MockIDeliveryProofAdapterFailureMode::NonceDrawRefused`, selected
///   through the case params and the consumer's held selection, each scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, the
///   mock forms selection, the mock generate selection, the other selections
///   at their defaults, the operating-system source.
/// Act:     `create_encoding` through the crate root, once per case, inside
///   the case consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)))))))))) })`,
///   the pattern naming every field.
/// Boundary: the public call `create_encoding` with this consumer through the
///   crate root and the route the disposition states.
/// Mocked:  the delivery-proof family's mock concrete, selected by
///   configuration.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
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
            let generate_params = build_create_generate_params(Default::default());
            let chain_forms_params = build_create_chain_forms_params(Default::default());
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                random,
                statements: &admitted_statements,
                identities: &admitted_identities,
                chain_forms: AdmittedChainForms {
                    concrete: chain_forms_params.concrete,
                    identifier: chain_forms_params.identifier,
                },
                kem: AdmittedKem {
                    concrete: self.kem.concrete,
                    identifier: self.kem.identifier,
                    scope: self.kem.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
                delivery_proof: DeliveryProofConcrete::Mock(
                    MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                ),
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
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
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
                });

            // Act
            let result = run_create_generate_case(
                &build_create_generate_case_deps(
                    Consumer {
                        kem: *kem_selection,
                    },
                    Default::default(),
                ),
                case_params,
                build_create_generate_case_payload(&canonicals, &roots, Default::default()),
            );

            // Assert
            let output = result.expect("the case composes").output;
            assert!(matches!(
                output,
                Ok(CreateEncodingSuccessReturn {
                    output: Err(EncodingConsumerErrorReturn::ChainForms(
                        ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::KeyAgreement(
                            KeyAgreementConsumerErrorReturn::DeliveryProof(
                                DeliveryProofConsumerErrorReturn::Measure(
                                    MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
                                        ProveMintErrorReturn::Mock(
                                            MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                                        )
                                    ))
                                )
                            )
                        ))
                    )),
                })
            ));
        }
    }
    // a_nonce_draw_refusal_reaches_the_caller_whole
}

// Public surface enumeration — every entry of the spec's public surface and
// the block that proves it:
//   the_files_and_the_measurement_reach_the_caller_of_create_encoding_whole
//     → the_files_and_the_measurement_reach_the_caller_of_create_encoding_whole
//   the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with
//     → the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with
//   the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
//     → the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
//   an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs
//     → an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs
//   the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold (carried)
//     → the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
//   a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds (carried)
//     → a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
//   create_generate_refuses_an_encoder_declaring_another_encoding_identifier (carried)
//     → create_generate_refuses_an_encoder_declaring_another_encoding_identifier
//   a_hash_to_scalar_refusal_reaches_the_caller_whole (carried)
//     → a_hash_to_scalar_refusal_reaches_the_caller_whole
//   a_nonce_draw_refusal_reaches_the_caller_whole (carried)
//     → a_nonce_draw_refusal_reaches_the_caller_whole
