#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, DecodeParams, IDecoderAdapter,
    IEncoderAdapter,
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
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, IDeliveryProofAdapter,
    MintPurpose, TransferPurpose, build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, RandomSourceKind, build_create_random_source_params,
};

use super::interface::GenerateConsumerReturn;
use super::mock::build_generate_consumer;
use crate::generate::provides::{
    ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload,
    CreateGenerateCaseParamsOverrides, CreateGenerateParams, CreateGenerateParamsOverrides,
    CreateGenerateReturn, EmitErrorReturn, GENERATE_CONCRETES, GenerateConcrete,
    GenerateConcreteOverrides, GenerateKindConfigurationsOverrides, ICreateGenerateCaseConsumer,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode, VectorKind,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, build_create_generate_deps, build_create_generate_params,
    build_create_generate_payload, build_generate_concrete, build_generate_kind_configurations,
    create_generate, run_create_generate_case,
};

/// Contract: own entry
///   `the_identities_the_consumer_carries_reach_the_concrete_as_the_records_its_identity_mapping_file_holds` —
///   `create_generate` over a member of `GENERATE_CONCRETES` and a consumer
///   constructed from the typed contexts and the admitted identities the case
///   runner composes, the identities a count other than the drawn kinds'
///   configured count → `Ok` whose output holds the `EmittedFile` tagged
///   `Some(VectorKind::IdentityMapping)` whose path's file holds one line for
///   each identity the consumer was constructed with.
/// Arrange: a count of identities of one and a count of identities of three,
///   a case payload of that many canonicals and that many roots, over every
///   member of `GENERATE_CONCRETES` crossed with every member of
///   `PAIRING_CONCRETES` and each scope of `DECLARED_KEM_SELECTIONS`; the
///   hash-to-scalar selection a member of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   declaring `HashToScalarIdentifier::Keccak256V1`, the encoding and
///   chain-forms, key-agreement, and delivery-proof selections the declared
///   sets' first members, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_generate`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` and the file tagged
///   `Some(VectorKind::IdentityMapping)` holds that many lines.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_generate`, the selected concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, and `emit`.
/// Mocked:  nothing; the filesystem and the operating system's generator are
///   the outer edges.
#[test]
fn the_identities_the_consumer_carries_reach_the_concrete_as_the_records_its_identity_mapping_file_holds()
 {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<GenerateConsumerReturn>;

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
            let proof_identity = payload.identities[0];
            let consumer = build_generate_consumer(
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Transfer,
                    Default::default(),
                ),
                payload.identities,
                proof_identity,
                Default::default(),
            );
            create_generate(
                &build_create_generate_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    payload.kem,
                    payload.key_agreement,
                    payload.delivery_proof,
                    consumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
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
        for pairing in PAIRING_CONCRETES {
            for kem in DECLARED_KEM_SELECTIONS {
                for count in [1usize, 3usize] {
                    let consumer = Consumer {
                        params: build_create_generate_params(CreateGenerateParamsOverrides {
                            concrete: Some(build_generate_concrete(
                                *kind,
                                GenerateConcreteOverrides,
                            )),
                            kinds: Some(build_generate_kind_configurations(
                                *kind,
                                GenerateKindConfigurationsOverrides,
                            )),
                            ..Default::default()
                        }),
                    };
                    let case_params =
                        build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing),
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
                            kem: Some(*kem),
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
                    let success = result
                        .expect("the case composes")
                        .output
                        .expect("create_generate admits the case")
                        .output
                        .expect("emit admits the case");
                    let file = success
                        .files
                        .iter()
                        .find(|file| file.kind == Some(VectorKind::IdentityMapping))
                        .expect("the emit returns the identity-mapping file");
                    let text = std::fs::read_to_string(&file.path).expect("the emitted file reads");
                    assert_eq!(text.lines().count(), count);
                }
            }
        }
    }
    // the_identities_the_consumer_carries_reach_the_concrete_as_the_records_its_identity_mapping_file_holds
}

/// Contract: own entry
///   `the_contexts_the_consumer_carries_reach_the_concrete_as_the_purposes_its_challenge_file_holds` —
///   `create_generate` over a member of `GENERATE_CONCRETES` and a consumer
///   constructed from a mint context whose purpose is
///   `MintPurpose::Replacement` and a transfer context whose purpose is
///   `TransferPurpose::Grant` → `Ok` whose output holds the `EmittedFile`
///   tagged `Some(VectorKind::Challenge)` whose path's file holds lines that
///   each decode to a `ChallengeVector` whose `purpose` is `4` for as many
///   lines as it is `3`, that number positive, and is neither `1` nor `2` for
///   any line.
/// Arrange: the mint context over `MintPurpose::Replacement` and the
///   transfer context over `TransferPurpose::Grant`, purposes other than the
///   builder defaults, over every member of `GENERATE_CONCRETES` crossed with
///   every member of `PAIRING_CONCRETES`; the other families' selections the
///   declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_generate`, once per case, inside the case consumer.
/// Assert:  in place inside the case consumer — each line of the file
///   tagged `Some(VectorKind::Challenge)`, its hex after `0x` read back into
///   bytes, decodes through the received decoder's `decode` over
///   `DecodeParams { description: &description }` with `description` the
///   `ChallengeVectorDescription` built by its `try_new` unpacked
///   irrefutably, and the `described` `ChallengeVector`s' `purpose` is `4` in
///   as many lines as it is `3`, that number positive, and is neither `1`
///   nor `2` for any line; the block asserts the output is `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_generate`, the selected concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, and `emit`.
/// Mocked:  nothing; the filesystem and the operating system's generator are
///   the outer edges.
#[test]
fn the_contexts_the_consumer_carries_reach_the_concrete_as_the_purposes_its_challenge_file_holds() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<GenerateConsumerReturn>;

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
            let proof_identity = payload.identities[0];
            let consumer = build_generate_consumer(
                build_delivery_context::<F, MintPurpose>(
                    MintPurpose::Replacement,
                    Default::default(),
                ),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Grant,
                    Default::default(),
                ),
                payload.identities,
                proof_identity,
                Default::default(),
            );
            let result = create_generate(
                &build_create_generate_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    payload.kem,
                    payload.key_agreement,
                    payload.delivery_proof,
                    consumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            );
            let Ok(created) = &result else {
                panic!("the factory admits the concrete")
            };
            let Ok(emitted) = &created.output else {
                panic!("emit admits the case")
            };
            let file = emitted
                .files
                .iter()
                .find(|file| file.kind == Some(VectorKind::Challenge))
                .expect("the emit returns the challenge file");
            let text = std::fs::read_to_string(&file.path).expect("the emitted file reads");
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
        for pairing in PAIRING_CONCRETES {
            let consumer = Consumer {
                params: build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(build_generate_concrete(*kind, GenerateConcreteOverrides)),
                    kinds: Some(build_generate_kind_configurations(
                        *kind,
                        GenerateKindConfigurationsOverrides,
                    )),
                    ..Default::default()
                }),
            };
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing),
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
            assert!(result.expect("the case composes").output.is_ok());
        }
    }
    // the_contexts_the_consumer_carries_reach_the_concrete_as_the_purposes_its_challenge_file_holds
}

/// Contract: own entry
///   `the_consumer_returns_the_files_the_concrete_wrote_whole_over_every_concrete` —
///   `create_generate` over a member of `GENERATE_CONCRETES` with the
///   per-kind configuration `build_generate_kind_configurations` returns for
///   it, and the mock selection under `NoFailure`, with a consumer
///   constructed from the typed contexts and the admitted identities the
///   case runner composes → `Ok` whose output is an
///   `Ok(EmitSuccessReturn)` whose `files` name no path twice, include
///   files tagged `Some(VectorKind::IdentityMapping)`,
///   `Some(VectorKind::Challenge)`, `Some(VectorKind::MintProof)`, and
///   `Some(VectorKind::TransferProof)` for a real concrete, and, for a real
///   concrete, each of whose paths exists on the filesystem.
/// Arrange: every member of `GENERATE_CONCRETES` crossed with every member
///   of `PAIRING_CONCRETES` and each scope of `DECLARED_KEM_SELECTIONS`, and
///   the mock selection under `NoFailure`; the other families' selections
///   the declared sets' first members for a real concrete, the hash-to-scalar
///   selection a member declaring `HashToScalarIdentifier::Keccak256V1`, the
///   random source `RandomSourceKind::OperatingSystem`, the mock selection
///   over the mock encoding and chain-forms defaults its declaration admits.
/// Act:     `create_generate`, once per case, inside the case consumer.
/// Assert:  the output is `Ok`, its `files` name no path twice by
///   `assert_eq!` on the unique count, a real concrete's include the files
///   tagged `Some(VectorKind::IdentityMapping)`,
///   `Some(VectorKind::Challenge)`, `Some(VectorKind::MintProof)`, and
///   `Some(VectorKind::TransferProof)`, and each path of a real concrete
///   exists.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_generate`, the selected concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, and `emit`; the mock
///   selection running no filesystem.
/// Mocked:  the family's mock concrete as the mock selection; the
///   filesystem and the operating system's generator the outer edges.
#[test]
fn the_consumer_returns_the_files_the_concrete_wrote_whole_over_every_concrete() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<GenerateConsumerReturn>;

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
            let proof_identity = payload.identities[0];
            let consumer = build_generate_consumer(
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Transfer,
                    Default::default(),
                ),
                payload.identities,
                proof_identity,
                Default::default(),
            );
            create_generate(
                &build_create_generate_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    payload.kem,
                    payload.key_agreement,
                    payload.delivery_proof,
                    consumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
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
        for pairing in PAIRING_CONCRETES {
            for kem in DECLARED_KEM_SELECTIONS {
                let consumer = Consumer {
                    params: build_create_generate_params(CreateGenerateParamsOverrides {
                        concrete: Some(build_generate_concrete(*kind, GenerateConcreteOverrides)),
                        kinds: Some(build_generate_kind_configurations(
                            *kind,
                            GenerateKindConfigurationsOverrides,
                        )),
                        ..Default::default()
                    }),
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing),
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
                        kem: Some(*kem),
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
                    .expect("create_generate admits the case")
                    .output
                    .expect("emit admits the case");
                let paths: Vec<_> = success.files.iter().map(|file| &file.path).collect();
                let unique: HashSet<_> = paths.iter().collect();
                assert_eq!(unique.len(), paths.len());
                for kind in [
                    VectorKind::IdentityMapping,
                    VectorKind::Challenge,
                    VectorKind::MintProof,
                    VectorKind::TransferProof,
                ] {
                    assert!(success.files.iter().any(|file| file.kind == Some(kind)));
                }
                assert!(success.files.iter().all(|file| file.path.exists()));
            }
        }
    }
    for pairing in PAIRING_CONCRETES {
        for kem in DECLARED_KEM_SELECTIONS {
            let consumer = Consumer {
                params: build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::NoFailure,
                    )),
                    ..Default::default()
                }),
            };
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing),
                        ..Default::default()
                    })),
                    kem: Some(*kem),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
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
            let success = result
                .expect("the case composes")
                .output
                .expect("create_generate admits the case")
                .output
                .expect("emit admits the case");
            let paths: Vec<_> = success.files.iter().map(|file| &file.path).collect();
            let unique: HashSet<_> = paths.iter().collect();
            assert_eq!(unique.len(), paths.len());
        }
    }
    // the_consumer_returns_the_files_the_concrete_wrote_whole_over_every_concrete
}

/// Contract: own entry
///   `a_refused_write_reaches_the_caller_of_create_generate_whole_through_the_consumer` —
///   `create_generate` over
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`
///   with a consumer constructed from the typed contexts and the admitted
///   identities the case runner composes → `Ok` whose output is
///   `Err(EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::WriteRefused))`,
///   the refusal the concrete's own, neither converted nor summarized.
/// Arrange: the mock selection under `WriteRefused` crossed with every
///   member of `PAIRING_CONCRETES`, of the keccak members of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, of `DECLARED_KEM_SELECTIONS`, of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and of `DELIVERY_PROOF_CONCRETES`,
///   over the mock encoding and chain-forms defaults the mock concrete's
///   declared `encoding_identifier` requires, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_generate`, once per case, inside the case consumer.
/// Assert:  the output matches
///   `Err(EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::WriteRefused))`,
///   the pattern naming every field because `EmitErrorReturn` omits
///   `PartialEq`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_generate`, the mock concrete's `try_new`,
///   `GenerateConsumer::try_new`, `consume_generate`, and `emit`.
/// Mocked:  the family's mock concrete under its failure mode, the declared
///   set unable to reach it.
#[test]
fn a_refused_write_reaches_the_caller_of_create_generate_whole_through_the_consumer() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<GenerateConsumerReturn>;

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
            let proof_identity = payload.identities[0];
            let consumer = build_generate_consumer(
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Transfer,
                    Default::default(),
                ),
                payload.identities,
                proof_identity,
                Default::default(),
            );
            create_generate(
                &build_create_generate_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    payload.kem,
                    payload.key_agreement,
                    payload.delivery_proof,
                    consumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
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

    for pairing in PAIRING_CONCRETES {
        for hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
        {
            for kem in DECLARED_KEM_SELECTIONS {
                for key_agreement in DECLARED_KEY_AGREEMENT_SELECTIONS {
                    for delivery_proof in DELIVERY_PROOF_CONCRETES {
                        let consumer = Consumer {
                            params: build_create_generate_params(CreateGenerateParamsOverrides {
                                concrete: Some(GenerateConcrete::Mock(
                                    MockIGenerateAdapterFailureMode::WriteRefused,
                                )),
                                ..Default::default()
                            }),
                        };
                        let case_params =
                            build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                kem: Some(*kem),
                                key_agreement: Some(*key_agreement),
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
                                ..Default::default()
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
                        let output = result
                            .expect("the case composes")
                            .output
                            .expect("create_generate admits the case")
                            .output;
                        assert!(matches!(
                            output,
                            Err(EmitErrorReturn::Mock(
                                MockIGenerateAdapterEmitErrorReturn::WriteRefused
                            ))
                        ));
                    }
                }
            }
        }
    }
    // a_refused_write_reaches_the_caller_of_create_generate_whole_through_the_consumer
}
