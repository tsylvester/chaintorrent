#![cfg(feature = "mocks")]
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
    CreateKemParams, CreateKemParamsOverrides, CreateKemReturn, CreateKemSuccessReturn,
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, ICredentialKemAdapter, KemIdentity,
    SetupErrorReturn, SetupScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    PAIRING_CONCRETES, SampleUniformScalarErrorReturn, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete,
    IDeliveryProofAdapter, MintPurpose, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use harness_crypto::{
    AdmittedIdentities, AdmittedIdentitiesOverrides, AdmittedIdentity, AdmittedKemIdentity,
    AdmittedKeyAgreement, ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    CreateGenerateCaseParamsOverrides, CreateGenerateErrorReturn, DeliveryProofConsumerErrorReturn,
    GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteKind, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, ICreateGenerateCaseConsumer, KemConsumer,
    KemConsumerConstructorParams, KemConsumerErrorReturn, KemConsumerReturn,
    KeyAgreementConsumerErrorReturn, L1DataFee, MeasureErrorReturn, MintErrorReturn,
    MockIVerifierAdapterFailureMode, VectorCountConstructorParamsOverrides, VectorKind,
    VectorKindConfiguration, VectorKindConfigurationOverrides, VectorKindQuantitiesOverrides,
    VerifierConcrete, VerifyCost, build_admitted_identities, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    build_create_generate_params, build_generate_concrete, build_generate_kind_configurations,
    build_vector_count, build_vector_kind_configuration, build_vector_kind_quantities,
    run_create_generate_case,
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

/// Contract: own entry
///   `the_files_and_the_measurement_reach_the_caller_of_create_kem_whole`,
///   as an outside caller observes it — `create_kem` over the mock selection
///   and each member of `DECLARED_KEM_SELECTIONS`, with a `KemConsumer`
///   constructed through `KemConsumer::try_new` from
///   `KemConsumerConstructorParams` → `Ok` whose output is
///   `Ok(DeliveryProofConsumerSuccessReturn)` whose `files` name no path
///   twice, include for a real generate concrete files tagged
///   `Some(VectorKind::IdentityMapping)`, `Some(VectorKind::Challenge)`,
///   `Some(VectorKind::MintProof)`, and `Some(VectorKind::TransferProof)`
///   each of whose paths exists on the filesystem, and whose
///   `measurement.iterations` equals the held count and
///   `measurement.costs.mint` and `measurement.costs.transfer` each equal
///   the literal cost the verifier family's mock concrete returns.
/// Arrange: every member of `GENERATE_CONCRETES` and the mock generate
///   selection, crossed with every member of `PAIRING_CONCRETES`, every KEM
///   selection, every delivery proof selection, and every key agreement
///   selection; the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the other families' selections
///   the declared sets' first members, the random source
///   `RandomSourceKind::OperatingSystem`, an iteration count of three, and
///   the expected cost the literal gas `100_000` and fee `L1DataFee::Charged
///   { wei: 1_000 }`.
/// Act:     `create_kem` through the crate root, once per case, inside the
///   case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four
///   tagged files and each of its paths exists, `measurement.iterations`
///   equals the held count, and the two costs equal the expected cost.
/// Boundary: the public calls `KemConsumer::try_new` and `create_kem`
///   through the crate root, inside the consumer the generate factory's
///   `run_create_generate_case` hands it under the `mocks` feature, over the
///   route `run_create_generate_case`, the factories it composes,
///   `create_kem`, the selected concrete's `try_new`, `consume_kem`,
///   `create_key_agreement`, the selected key agreement concrete's
///   `try_new`, `consume_key_agreement`, `create_delivery_proof`, the
///   selected delivery proof concrete's `try_new`, `consume_delivery_proof`,
///   `create_generate`, the selected generate concrete's `try_new`,
///   `consume_generate`, `emit`, `create_verifier`, `consume_verifier`, and
///   `measure`.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_files_and_the_measurement_reach_the_caller_of_create_kem_whole() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
        key_agreement: Option<CreateKeyAgreementParams>,
        delivery_proof: DeliveryProofConcrete,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
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
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = KemConsumer::try_new(KemConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                mint: build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    Default::default(),
                ),
                transfer: build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities: &admitted_identities,
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
            create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(CreateKemParamsOverrides {
                    concrete: Some(self.kem.concrete),
                    identifier: Some(self.kem.identifier),
                    scope: Some(self.kem.scope),
                }),
                build_create_kem_payload(Default::default()),
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
            for kem_selection in &kem_selections {
                for delivery_proof in &delivery_proofs {
                    for key_agreement in &key_agreements {
                        let consumer = Consumer {
                            kem: *kem_selection,
                            key_agreement: *key_agreement,
                            delivery_proof: *delivery_proof,
                            generate: generate.clone(),
                            kinds: kinds.clone(),
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
    // the_files_and_the_measurement_reach_the_caller_of_create_kem_whole
}

/// Contract: own entry
///   `the_identities_the_consumer_pairs_reach_the_delivery_proof_consumer_as_the_records_the_files_hold`,
///   as an outside caller observes it — `create_kem` and a consumer
///   constructed with `AdmittedIdentities` whose `mapping` holds a count of
///   identities other than the drawn kinds' configured count, of the variant
///   the selection's scope names, a mint context over
///   `MintPurpose::Replacement`, and a transfer context over
///   `TransferPurpose::Grant` → `Ok` whose output is `Ok` and whose file
///   tagged `Some(VectorKind::IdentityMapping)` holds one line for each
///   mapping identity the consumer was constructed with and whose file
///   tagged `Some(VectorKind::Challenge)` holds lines that each decode to a
///   `ChallengeVector` whose `purpose` is `4` for as many lines as it is
///   `3`, that number positive, and is neither `1` nor `2` for any line.
/// Arrange: a mapping of one identity and of three, a case payload of that
///   many canonicals and that many roots, under each scope of
///   `DECLARED_KEM_SELECTIONS`, over a member of `GENERATE_CONCRETES` and
///   every member of `PAIRING_CONCRETES`; the other families' selections the
///   declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_kem` through the crate root, once per case, inside the
///   case consumer.
/// Assert:  in place inside the case consumer — the file tagged
///   `Some(VectorKind::IdentityMapping)` holds that many lines and each line
///   of the file tagged `Some(VectorKind::Challenge)`, its hex after `0x`
///   read back into bytes, decodes through the received decoder's `decode`
///   over `DecodeParams { description: &description }` with `description`
///   the `ChallengeVectorDescription` built by its `try_new` unpacked
///   irrefutably, and the `described` `ChallengeVector`s' `purpose` is `4`
///   in as many lines as it is `3`, that number positive, and neither `1`
///   nor `2` in any line; the block asserts the output is `Ok` holding `Ok`.
/// Boundary: the public calls `KemConsumer::try_new` and `create_kem`
///   through the crate root, over the route `run_create_generate_case`, the
///   factories it composes, `create_kem`, the selected concrete's `try_new`,
///   `consume_kem`, `create_key_agreement`, the selected key agreement
///   concrete's `try_new`, `consume_key_agreement`, `create_delivery_proof`,
///   the selected delivery proof concrete's `try_new`,
///   `consume_delivery_proof`, `create_generate`, the selected generate
///   concrete's `try_new`, `consume_generate`, and `emit`.
/// Mocked:  nothing; the filesystem and the operating system's generator are
///   the outer edges.
#[test]
fn the_identities_the_consumer_pairs_reach_the_delivery_proof_consumer_as_the_records_the_files_hold()
 {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
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
            let admitted_identities = admitted_identities_of(&identities);
            let mapping_count = admitted_identities.mapping.len();
            let Ok(consumer) = KemConsumer::try_new(KemConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                mint: build_delivery_context::<F, MintPurpose>(
                    MintPurpose::Replacement,
                    Default::default(),
                ),
                transfer: build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Grant,
                    Default::default(),
                ),
                identities: &admitted_identities,
                key_agreement: {
                    let params = build_create_key_agreement_params(Default::default());
                    AdmittedKeyAgreement {
                        concrete: params.concrete,
                        identifier: params.identifier,
                        algebra: params.algebra,
                    }
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            let result = create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(CreateKemParamsOverrides {
                    concrete: Some(self.kem.concrete),
                    identifier: Some(self.kem.identifier),
                    scope: Some(self.kem.scope),
                }),
                build_create_kem_payload(Default::default()),
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
        for pairing_concrete in PAIRING_CONCRETES {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                for count in [1usize, 3usize] {
                    let consumer = Consumer {
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
                        Ok(CreateKemSuccessReturn { output: Ok(_) })
                    ));
                }
            }
        }
    }
    // the_identities_the_consumer_pairs_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
}

/// Contract: carried entry
///   `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`
///   of `harness-crypto/run/delivery_proof_consumer`, as an outside caller
///   observes it through this consumer — `create_kem` with a consumer
///   holding the per-kind configuration the block builds, the possession
///   entry's count a `build_vector_count` value other than the default and
///   than every other entry's count → `Ok` whose output's file tagged
///   `Some(VectorKind::Possession)` holds the configured count of lines.
/// Arrange: the generate selection and the per-kind configuration
///   `build_generate_kind_configurations` returns reversed, the possession
///   entry's count overridden through `build_vector_kind_configuration` and
///   `build_vector_kind_quantities`, and `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::NoFailure)`, over every member of
///   `GENERATE_CONCRETES` and every member of `PAIRING_CONCRETES`; the other
///   families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_kem` through the crate root, once per case, inside the
///   case consumer.
/// Assert:  the output is `Ok` holding `Ok`, and the file whose `kind` is
///   `Some(VectorKind::Possession)` is read by `std::fs::read_to_string` and
///   its line count equals the configured count.
/// Boundary: the public call `create_kem` with this consumer through the
///   crate root and the route the disposition states —
///   `run_create_generate_case`, `create_kem`, the selected concrete's
///   `try_new`, `consume_kem`, `create_key_agreement`, the selected key
///   agreement concrete's `try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, the selected delivery proof concrete's
///   `try_new`, `consume_delivery_proof`, `create_generate`, the selected
///   generate concrete's `try_new`, `consume_generate`, and `emit`.
/// Mocked:  the verifier family's mock concrete.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
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
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = KemConsumer::try_new(KemConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                mint: build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    Default::default(),
                ),
                transfer: build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities: &admitted_identities,
                key_agreement: {
                    let params = build_create_key_agreement_params(Default::default());
                    AdmittedKeyAgreement {
                        concrete: params.concrete,
                        identifier: params.identifier,
                        algebra: params.algebra,
                    }
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(CreateKemParamsOverrides {
                    concrete: Some(DECLARED_KEM_SELECTIONS[0].concrete),
                    identifier: Some(DECLARED_KEM_SELECTIONS[0].identifier),
                    scope: Some(DECLARED_KEM_SELECTIONS[0].scope),
                }),
                build_create_kem_payload(Default::default()),
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
        for pairing_concrete in PAIRING_CONCRETES {
            let consumer = Consumer {
                generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                kinds: kinds.clone(),
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
    // a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
}

/// Contract: carried entry
///   `create_generate_refuses_an_encoder_declaring_another_encoding_identifier`
///   of `harness-crypto/run/delivery_proof_consumer`, as an outside caller
///   observes it through this consumer — the refusal of `create_generate`
///   returns from `consume_kem` as `KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::CreateGenerate(
///   CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)))`.
/// Arrange: every member of `GENERATE_CONCRETES` over `EncodingConcrete::Mock`,
///   whose declared identifier is `EncodingIdentifier::Mock`, and the mock
///   selection over every member of `DECLARED_ENCODING_SELECTIONS` under
///   `CHAIN_FORMS_CONCRETES`' selection, as the entry of the same name of
///   `harness-crypto/generate/factory` arranges the encoder, over every
///   member of `PAIRING_CONCRETES`.
/// Act:     `create_kem` through the crate root, once per case, inside the
///   case consumer.
/// Assert:  the output matches `Ok(CreateKemSuccessReturn { output:
///   Err(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::CreateGenerate(
///   CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)))) })`, the
///   pattern naming every field.
/// Boundary: the public call `create_kem` with this consumer through the
///   crate root and the route the disposition states —
///   `run_create_generate_case`, `create_kem`, the selected concrete's
///   `try_new`, `consume_kem`, `create_key_agreement`, the selected key
///   agreement concrete's `try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, the selected delivery proof concrete's
///   `try_new`, `consume_delivery_proof`, and `create_generate`.
/// Mocked:  the encoding family's mock concrete as the encoder for the real
///   generate concretes, and the generate family's mock concrete as the
///   selection for the real encoders, selected through their factories.
#[test]
fn create_generate_refuses_an_encoder_declaring_another_encoding_identifier() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
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
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = KemConsumer::try_new(KemConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                mint: build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    Default::default(),
                ),
                transfer: build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities: &admitted_identities,
                key_agreement: {
                    let params = build_create_key_agreement_params(Default::default());
                    AdmittedKeyAgreement {
                        concrete: params.concrete,
                        identifier: params.identifier,
                        algebra: params.algebra,
                    }
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: self.generate.clone(),
                kinds: self.kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
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
                    encoding: Some(CreateEncodingParams {
                        concrete: EncodingConcrete::Mock,
                        identifier: EncodingIdentifier::Mock,
                    }),
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
                Ok(CreateKemSuccessReturn {
                    output: Err(KemConsumerErrorReturn::KeyAgreement(
                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                            DeliveryProofConsumerErrorReturn::CreateGenerate(
                                CreateGenerateErrorReturn::UnsupportedEncodingIdentifier
                            )
                        )
                    )),
                })
            ));
        }
    }

    for encoding in DECLARED_ENCODING_SELECTIONS {
        for pairing_concrete in PAIRING_CONCRETES {
            let generate_params = build_create_generate_params(Default::default());
            let consumer = Consumer {
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
                    chain_forms: Some(build_create_chain_forms_params(
                        CreateChainFormsParamsOverrides {
                            concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
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
                Ok(CreateKemSuccessReturn {
                    output: Err(KemConsumerErrorReturn::KeyAgreement(
                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                            DeliveryProofConsumerErrorReturn::CreateGenerate(
                                CreateGenerateErrorReturn::UnsupportedEncodingIdentifier
                            )
                        )
                    )),
                })
            ));
        }
    }
    // create_generate_refuses_an_encoder_declaring_another_encoding_identifier
}

/// Contract: carried entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`
///   of `harness-crypto/run/key_agreement_consumer`, as an outside caller
///   observes it through this consumer — `create_kem` over a case whose
///   hash-to-scalar adapter is the family's mock concrete under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused`, selected
///   through the case params, with the mock generate selection → `Ok` whose
///   output is `Err(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(
///   error)))))))` under the entitlement scope and `Err(…Mint(
///   MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)))…)` under
///   the asset scope, `error` `HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected, actual: 0 })`
///   with `expected` the scalar's `UNIFORM_BYTES_LENGTH`, unconverted.
/// Arrange: the hash-to-scalar selection through the case params, each scope
///   of `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   the other selections at their defaults, the operating-system source.
/// Act:     `create_kem` through the crate root, once per case, inside the
///   case consumer.
/// Assert:  in place inside the case consumer — the output matches the
///   scope's `Ok(CreateKemSuccessReturn { output: Err(…) })` with `expected`
///   `UNIFORM_BYTES_LENGTH` and `actual` `0`; the block asserts the output
///   is `Ok` holding `Err(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(_)))))`.
/// Boundary: the public call `create_kem` with this consumer through the
///   crate root and the route the disposition states —
///   `run_create_generate_case`, `create_kem`, the selected concrete's
///   `try_new`, `consume_kem`, `create_key_agreement`, the selected key
///   agreement concrete's `try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, the selected delivery proof concrete's
///   `try_new`, `consume_delivery_proof`, `create_generate`,
///   `consume_generate`, `emit`, `create_verifier`, `VerifierConsumer::try_new`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
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
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = KemConsumer::try_new(KemConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                mint: build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    Default::default(),
                ),
                transfer: build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities: &admitted_identities,
                key_agreement: {
                    let params = build_create_key_agreement_params(Default::default());
                    AdmittedKeyAgreement {
                        concrete: params.concrete,
                        identifier: params.identifier,
                        algebra: params.algebra,
                    }
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });
            let returned = create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(CreateKemParamsOverrides {
                    concrete: Some(self.kem.concrete),
                    identifier: Some(self.kem.identifier),
                    scope: Some(self.kem.scope),
                }),
                build_create_kem_payload(Default::default()),
            );
            match scope {
                SetupScope::Entitlement => {
                    assert!(matches!(
                        &returned,
                        Ok(CreateKemSuccessReturn {
                            output: Err(KemConsumerErrorReturn::KeyAgreement(
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
                            ))
                        }) if *expected
                            == <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                    ));
                }
                SetupScope::Asset { .. } => {
                    assert!(matches!(
                        &returned,
                        Ok(CreateKemSuccessReturn {
                            output: Err(KemConsumerErrorReturn::KeyAgreement(
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
                Ok(CreateKemSuccessReturn {
                    output: Err(KemConsumerErrorReturn::KeyAgreement(
                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                            DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(_))
                        )
                    ))
                })
            ));
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: carried entry `a_nonce_draw_refusal_reaches_the_caller_whole` of
///   `harness-crypto/run/key_agreement_consumer`, as an outside caller
///   observes it through this consumer — `create_kem` over the delivery proof
///   family's mock concrete under
///   `MockIDeliveryProofAdapterFailureMode::NonceDrawRefused`, selected
///   through the case params and the consumer's held selection, with the
///   mock generate selection → `Ok` whose output is
///   `Err(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)))))))`,
///   unconverted.
/// Arrange: the delivery-proof selection through the case params and the
///   consumer's held selection, each scope of `DECLARED_KEM_SELECTIONS` over
///   every member of `PAIRING_CONCRETES`, the other selections at their
///   defaults, the operating-system source.
/// Act:     `create_kem` through the crate root, once per case, inside the
///   case consumer.
/// Assert:  the output matches `Ok(CreateKemSuccessReturn { output:
///   Err(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))))) })`,
///   the pattern naming every field.
/// Boundary: the public call `create_kem` with this consumer through the
///   crate root and the route the disposition states —
///   `run_create_generate_case`, `create_kem`, the selected concrete's
///   `try_new`, `consume_kem`, `create_key_agreement`, the selected key
///   agreement concrete's `try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, the selected delivery proof concrete's
///   `try_new`, `consume_delivery_proof`, `create_generate`,
///   `consume_generate`, `emit`, `create_verifier`, `VerifierConsumer::try_new`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the delivery-proof family's mock concrete, selected by
///   configuration.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
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
            let admitted_identities = admitted_identities_of(&identities);
            let Ok(consumer) = KemConsumer::try_new(KemConsumerConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                mint: build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    Default::default(),
                ),
                transfer: build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities: &admitted_identities,
                key_agreement: {
                    let params = build_create_key_agreement_params(Default::default());
                    AdmittedKeyAgreement {
                        concrete: params.concrete,
                        identifier: params.identifier,
                        algebra: params.algebra,
                    }
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
            create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(CreateKemParamsOverrides {
                    concrete: Some(self.kem.concrete),
                    identifier: Some(self.kem.identifier),
                    scope: Some(self.kem.scope),
                }),
                build_create_kem_payload(Default::default()),
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
                Ok(CreateKemSuccessReturn {
                    output: Err(KemConsumerErrorReturn::KeyAgreement(
                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                            DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
                                MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
                                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                                ))
                            ))
                        )
                    ))
                })
            ));
        }
    }
    // a_nonce_draw_refusal_reaches_the_caller_whole
}

// Public surface enumeration — every entry of the spec's public surface and
// the block that proves it:
//   the_files_and_the_measurement_reach_the_caller_of_create_kem_whole
//     → the_files_and_the_measurement_reach_the_caller_of_create_kem_whole
//   the_identities_the_consumer_pairs_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
//     → the_identities_the_consumer_pairs_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
//   a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds (carried)
//     → a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
//   create_generate_refuses_an_encoder_declaring_another_encoding_identifier (carried)
//     → create_generate_refuses_an_encoder_declaring_another_encoding_identifier
//   a_hash_to_scalar_refusal_reaches_the_caller_whole (carried)
//     → a_hash_to_scalar_refusal_reaches_the_caller_whole
//   a_nonce_draw_refusal_reaches_the_caller_whole (carried)
//     → a_nonce_draw_refusal_reaches_the_caller_whole
