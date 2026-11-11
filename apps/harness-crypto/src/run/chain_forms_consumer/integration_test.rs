#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use chain::{
    ChainFormsConcrete, CreateChainFormsParams, CreateChainFormsReturn,
    CreateChainFormsSuccessReturn, DECLARED_CHAIN_FORMS_SELECTIONS, IChainForms,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use core::num::NonZeroU32;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CreateEncodingParams, DECLARED_ENCODING_SELECTIONS,
    DecodeParams, EncodingConcrete, EncodingIdentifier, FromFieldErrorReturn, IDecoderAdapter,
    IEncoderAdapter,
};
use envelope::{
    CreateKeyAgreementErrorReturn, CreateKeyAgreementParams, CreateKeyAgreementParamsOverrides,
    DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter, KeyAgreementConcrete,
    build_create_key_agreement_params,
};
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemErrorReturn, CreateKemParams, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    KemConcrete, KemIdentity, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofErrorReturn, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete, DeliveryRelation, IDeliveryProofAdapter,
    MintPurpose, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn, TransferPurpose,
    build_create_delivery_proof_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params,
};

use super::interface::{ChainFormsConsumerErrorReturn, ChainFormsConsumerReturn};
use super::mock::{ChainFormsConsumerConstructorParamsOverrides, build_chain_forms_consumer};
use crate::config::provides::{
    AdmittedIdentities, AdmittedIdentitiesOverrides, AdmittedIdentity, AdmittedKem,
    AdmittedKeyAgreement, AdmittedStatements, AdmittedStatementsOverrides,
    CanonicalDeliveryContextOverrides, build_admitted_identities, build_admitted_statements,
    build_canonical_delivery_context,
};
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};
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
use crate::run::provides::{
    AdmitContextsErrorReturn, DeliveryContextField, DeliveryProofConsumerErrorReturn,
    KemConsumerErrorReturn, KeyAgreementConsumerErrorReturn,
};
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyCost, VerifyOnChainErrorReturn,
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
///   `the_files_and_the_measurement_reach_the_caller_of_create_chain_forms_whole`
///   — `create_chain_forms` over the mock selection and each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, with a consumer constructed from the
///   adapters the case runner hands, statements and identities of the variant
///   the held KEM selection's scope names, a member of `DECLARED_KEM_SELECTIONS`
///   or the mock KEM selection, the key agreement selection of the mock
///   concrete or a member of `DECLARED_KEY_AGREEMENT_SELECTIONS`, a member of
///   `DELIVERY_PROOF_CONCRETES` or the mock delivery proof selection, a member
///   of `GENERATE_CONCRETES` or the mock generate selection under `NoFailure`,
///   and `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`
///   → `Ok` whose output is `Ok(DeliveryProofConsumerSuccessReturn)` whose
///   `files` name no path twice, include for a real generate concrete files
///   tagged `Some(VectorKind::IdentityMapping)`, `Some(VectorKind::Challenge)`,
///   `Some(VectorKind::MintProof)`, and `Some(VectorKind::TransferProof)` each
///   of whose paths exists on the filesystem, and whose
///   `measurement.iterations` equals the held count and
///   `measurement.costs.mint` and `measurement.costs.transfer` each equal the
///   literal cost the verifier family's mock concrete returns.
/// Arrange: every member of `GENERATE_CONCRETES` and the mock generate
///   selection, each paired with the forms selections its declared encoding
///   admits — a real concrete with each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS` and the mock selection with the mock
///   forms — crossed with every member of `PAIRING_CONCRETES`, every KEM
///   selection, every delivery proof selection, and every key agreement
///   selection; the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the other families' selections the
///   declared sets' first members, the random source
///   `RandomSourceKind::OperatingSystem`, an iteration count of three, and the
///   expected cost the literal gas `100_000` and fee `L1DataFee::Charged {
///   wei: 1_000 }`.
/// Act:     `create_chain_forms`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four
///   tagged files and each of its paths exists, `measurement.iterations`
///   equals the held count, and the two costs equal the expected cost.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, the selected forms concrete's producer,
///   `ChainFormsConsumer::try_new`, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, `KemConsumer::try_new`, `consume_kem`,
///   `create_key_agreement`, `KeyAgreementConsumer::try_new`,
///   `consume_key_agreement`, `create_delivery_proof`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, `consume_generate`, `emit`, `create_verifier`,
///   `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_files_and_the_measurement_reach_the_caller_of_create_chain_forms_whole() {
    // Arrange
    struct Consumer {
        forms: CreateChainFormsParams,
        kem: CreateKemParams,
        key_agreement: Option<CreateKeyAgreementParams>,
        delivery_proof: DeliveryProofConcrete,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: self.kem.concrete,
                        identifier: self.kem.identifier,
                        scope: self.kem.scope,
                    }),
                    key_agreement: Some(AdmittedKeyAgreement {
                        concrete: key_agreement_params.concrete,
                        identifier: key_agreement_params.identifier,
                        algebra: key_agreement_params.algebra,
                    }),
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
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                self.forms,
                build_create_chain_forms_payload(Default::default()),
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
                                forms: *forms,
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
    // the_files_and_the_measurement_reach_the_caller_of_create_chain_forms_whole
}

/// Contract: own entry
///   `the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold`
///   — `create_chain_forms` and a consumer constructed with
///   `AdmittedIdentities` whose `mapping` holds a count of identities other
///   than the drawn kinds' configured count, of the variant the held KEM
///   selection's scope names, and `AdmittedStatements` whose mint purpose is
///   `MintPurpose::Replacement` and whose transfer purpose is
///   `TransferPurpose::Grant`, each context in the forms the selection
///   constructs → `Ok` whose output is `Ok` and whose file tagged
///   `Some(VectorKind::IdentityMapping)` holds one line for each mapping
///   identity the consumer was constructed with and whose file tagged
///   `Some(VectorKind::Challenge)` holds lines that each decode to a
///   `ChallengeVector` whose `purpose` is `4` for as many lines as it is `3`,
///   that number positive, and is neither `1` nor `2` for any line.
/// Arrange: a mapping of one identity and of three, a case payload of that
///   many canonicals and that many roots, under each scope of
///   `DECLARED_KEM_SELECTIONS`, over a member of `GENERATE_CONCRETES`, each
///   member of `DECLARED_CHAIN_FORMS_SELECTIONS`, and every member of
///   `PAIRING_CONCRETES`; the other families' selections the declared sets'
///   first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per case, inside the case consumer.
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
///   `create_chain_forms`, the selected forms concrete's producer,
///   `ChainFormsConsumer::try_new`, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, `KemConsumer::try_new`, `consume_kem`,
///   `create_key_agreement`, `KeyAgreementConsumer::try_new`,
///   `consume_key_agreement`, `create_delivery_proof`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, `consume_generate`, and `emit`.
/// Mocked:  nothing; the filesystem and the operating system's generator are
///   the outer edges.
#[test]
fn the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold()
 {
    // Arrange
    struct Consumer {
        forms: CreateChainFormsParams,
        kem: CreateKemParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(AdmittedStatementsOverrides {
                mint_purpose: Some(MintPurpose::Replacement),
                transfer_purpose: Some(TransferPurpose::Grant),
                ..Default::default()
            });
            let admitted_identities = admitted_identities_of(&payload.identities);
            let mapping_count = admitted_identities.mapping.len();
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: self.kem.concrete,
                        identifier: self.kem.identifier,
                        scope: self.kem.scope,
                    }),
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    ..Default::default()
                },
            );
            let result = create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                self.forms,
                build_create_chain_forms_payload(Default::default()),
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
        for forms in DECLARED_CHAIN_FORMS_SELECTIONS {
            for pairing_concrete in PAIRING_CONCRETES {
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    for count in [1usize, 3usize] {
                        let consumer = Consumer {
                            forms: *forms,
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
                            Ok(CreateChainFormsSuccessReturn { output: Ok(_) })
                        ));
                    }
                }
            }
        }
    }
    // the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
}

/// Contract: own entry
///   `the_kem_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`
///   — `create_chain_forms` and a consumer constructed with an `AdmittedKem`
///   built from the `Bb1DepthOne` member of `DECLARED_KEM_SELECTIONS`, and the
///   same selection with the `identifier` the mock selection names → `Ok`
///   whose output is `Ok` for the declared member and
///   `Err(ChainFormsConsumerErrorReturn::CreateKem(
///   CreateKemErrorReturn::UnsupportedKemIdentifier))` for the mixed selection.
/// Arrange: the two selections over every member of `PAIRING_CONCRETES` and
///   each forms selection the generate selection's declared encoding admits —
///   the mock generate selection under the mock forms and a member of
///   `GENERATE_CONCRETES` under each member of `DECLARED_CHAIN_FORMS_SELECTIONS`;
///   the other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the verifier family's mock
///   concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per selection, inside the case consumer.
/// Assert:  the output under the declared selection holds `Ok` and the
///   output under the mixed selection matches `Ok(CreateChainFormsSuccessReturn
///   { output: Err(ChainFormsConsumerErrorReturn::CreateKem(
///   CreateKemErrorReturn::UnsupportedKemIdentifier)) })`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, the selected forms concrete's producer,
///   `ChainFormsConsumer::try_new`, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, and the selected KEM concrete's `try_new`.
/// Mocked:  the generate and verifier families' mock concretes where the case
///   reaches them.
#[test]
fn the_kem_selection_the_consumer_carries_is_the_concrete_the_factory_constructs() {
    // Arrange
    struct Consumer {
        forms: CreateChainFormsParams,
        kem: AdmittedKem,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(self.kem),
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                self.forms,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let declared = DECLARED_KEM_SELECTIONS
        .iter()
        .find(|selection| selection.concrete == KemConcrete::Bb1DepthOne)
        .expect("the declared set names the Bb1DepthOne member");
    let declared_admitted = AdmittedKem {
        concrete: declared.concrete,
        identifier: declared.identifier,
        scope: declared.scope,
    };
    let mixed_admitted = AdmittedKem {
        identifier: build_create_kem_params(Default::default()).identifier,
        ..declared_admitted
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
    let cases: Vec<(
        CreateChainFormsParams,
        GenerateConcrete,
        Vec<VectorKindConfiguration>,
    )> = DECLARED_CHAIN_FORMS_SELECTIONS
        .iter()
        .map(|forms| {
            (
                *forms,
                build_generate_concrete(GENERATE_CONCRETES[0], GenerateConcreteOverrides),
                build_generate_kind_configurations(
                    GENERATE_CONCRETES[0],
                    GenerateKindConfigurationsOverrides,
                ),
            )
        })
        .chain(core::iter::once({
            let params = build_create_generate_params(Default::default());
            (
                build_create_chain_forms_params(Default::default()),
                params.concrete,
                params.kinds,
            )
        }))
        .collect();

    for (forms, generate, kinds) in &cases {
        for pairing_concrete in PAIRING_CONCRETES {
            for kem in [declared_admitted, mixed_admitted] {
                let consumer = Consumer {
                    forms: *forms,
                    kem,
                    generate: generate.clone(),
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
                        kem: Some(*declared),
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
                if kem.identifier == declared.identifier {
                    assert!(matches!(
                        output,
                        Ok(CreateChainFormsSuccessReturn { output: Ok(_) })
                    ));
                } else {
                    assert!(matches!(
                        output,
                        Ok(CreateChainFormsSuccessReturn {
                            output: Err(ChainFormsConsumerErrorReturn::CreateKem(
                                CreateKemErrorReturn::UnsupportedKemIdentifier
                            )),
                        })
                    ));
                }
            }
        }
    }
    // the_kem_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
}

/// Contract: own entry
///   `a_refused_context_names_the_field_that_held_its_value_and_returns_before_the_kem_factory_is_called`
///   — `create_chain_forms` and a consumer constructed with
///   `AdmittedStatements` whose mint context holds `CanonicalFieldValue::Text`
///   as its `seller`, a kind no form declares, then whose transfer context
///   holds it as its `buyer` with the mint context at its defaults, then both,
///   and the mixed `AdmittedKem` of the entry above → `Ok` whose output is
///   `Err(ChainFormsConsumerErrorReturn::AdmitMint(
///   AdmitContextsErrorReturn::Field { field: DeliveryContextField::Seller,
///   error: FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } }))` for the first and the third
///   arrangements and `Err(ChainFormsConsumerErrorReturn::AdmitTransfer(
///   AdmitContextsErrorReturn::Field { field: DeliveryContextField::Buyer,
///   error: FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } }))` for the second, none of them the
///   factory's `UnsupportedKemIdentifier`.
/// Arrange: the three arrangements over each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS` and the mock forms selection; the mixed
///   `AdmittedKem`, the other families' selections the declared sets' first
///   members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the mock generate selection, the
///   verifier family's mock concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per arrangement, inside the case
///   consumer.
/// Assert:  the outputs match `Ok(CreateChainFormsSuccessReturn { output:
///   Err(ChainFormsConsumerErrorReturn::AdmitMint(
///   AdmitContextsErrorReturn::Field { field: DeliveryContextField::Seller,
///   error: FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } })) })` for the first and the third
///   arrangements and `Ok(CreateChainFormsSuccessReturn { output:
///   Err(ChainFormsConsumerErrorReturn::AdmitTransfer(
///   AdmitContextsErrorReturn::Field { field: DeliveryContextField::Buyer,
///   error: FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } })) })` for the second.
/// Boundary: `run_create_generate_case`, `create_chain_forms`, the selected
///   forms concrete's producer, `ChainFormsConsumer::try_new`,
///   `consume_chain_forms`, and `admit_contexts`.
/// Mocked:  the encoding family's concrete the case selects.
#[test]
fn a_refused_context_names_the_field_that_held_its_value_and_returns_before_the_kem_factory_is_called()
 {
    // Arrange
    struct Consumer {
        forms: CreateChainFormsParams,
        statements: AdmittedStatements,
        kem: AdmittedKem,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &self.statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(self.kem),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                self.forms,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let declared = DECLARED_KEM_SELECTIONS
        .iter()
        .find(|selection| selection.concrete == KemConcrete::Bb1DepthOne)
        .expect("the declared set names the Bb1DepthOne member");
    let mixed_admitted = AdmittedKem {
        concrete: declared.concrete,
        identifier: build_create_kem_params(Default::default()).identifier,
        scope: declared.scope,
    };
    let refused_text = || CanonicalFieldValue::Text("refused".to_string());
    let arrangements: Vec<(AdmittedStatements, bool)> = vec![
        (
            build_admitted_statements(AdmittedStatementsOverrides {
                mint: Some(build_canonical_delivery_context(
                    CanonicalDeliveryContextOverrides {
                        seller: Some(refused_text()),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            }),
            true,
        ),
        (
            build_admitted_statements(AdmittedStatementsOverrides {
                transfer: Some(build_canonical_delivery_context(
                    CanonicalDeliveryContextOverrides {
                        buyer: Some(refused_text()),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            }),
            false,
        ),
        (
            build_admitted_statements(AdmittedStatementsOverrides {
                mint: Some(build_canonical_delivery_context(
                    CanonicalDeliveryContextOverrides {
                        seller: Some(refused_text()),
                        ..Default::default()
                    },
                )),
                transfer: Some(build_canonical_delivery_context(
                    CanonicalDeliveryContextOverrides {
                        buyer: Some(CanonicalFieldValue::Text("refused".to_string())),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            }),
            true,
        ),
    ];
    let forms_selections: Vec<CreateChainFormsParams> = DECLARED_CHAIN_FORMS_SELECTIONS
        .iter()
        .copied()
        .chain(core::iter::once(build_create_chain_forms_params(
            Default::default(),
        )))
        .collect();
    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for forms in &forms_selections {
        for (statements, mint_refused) in &arrangements {
            let consumer = Consumer {
                forms: *forms,
                statements: statements.clone(),
                kem: mixed_admitted,
            };
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(PAIRING_CONCRETES[0]),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(keccak256_selection()),
                    encoding: Some(encoding_for(*forms)),
                    chain_forms: case_chain_forms(*forms),
                    kem: Some(*declared),
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
            if *mint_refused {
                assert!(matches!(
                    output,
                    Ok(CreateChainFormsSuccessReturn {
                        output: Err(ChainFormsConsumerErrorReturn::AdmitMint(
                            AdmitContextsErrorReturn::Field {
                                field: DeliveryContextField::Seller,
                                error: FromFieldErrorReturn::WrongKind {
                                    expected: CanonicalFieldKind::FixedBytes20
                                }
                            }
                        )),
                    })
                ));
            } else {
                assert!(matches!(
                    output,
                    Ok(CreateChainFormsSuccessReturn {
                        output: Err(ChainFormsConsumerErrorReturn::AdmitTransfer(
                            AdmitContextsErrorReturn::Field {
                                field: DeliveryContextField::Buyer,
                                error: FromFieldErrorReturn::WrongKind {
                                    expected: CanonicalFieldKind::FixedBytes20
                                }
                            }
                        )),
                    })
                ));
            }
        }
    }
    // a_refused_context_names_the_field_that_held_its_value_and_returns_before_the_kem_factory_is_called
}

/// Contract: absorbed entry
///   `the_key_agreement_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`
///   — the selection this consumer holds is the concrete
///   `create_key_agreement` constructs, through the KEM consumer, and the
///   refusal of an undeclared identifier is the output inside
///   `ChainFormsConsumerErrorReturn::Kem`.
/// Arrange: the `AdmittedKeyAgreement` built from the `PairingElGamal` member
///   of `DECLARED_KEY_AGREEMENT_SELECTIONS`, then the same selection with the
///   `identifier` of the mock selection as the counterfactual, over every
///   member of `PAIRING_CONCRETES` and each forms selection the generate
///   selection's declared encoding admits — the mock generate selection under
///   the mock forms and a member of `GENERATE_CONCRETES` under each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`; the other families' selections the
///   declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the verifier family's
///   mock concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per selection, inside the case consumer.
/// Assert:  the output under the declared selection holds `Ok` and the
///   output under the mixed selection matches `Ok(CreateChainFormsSuccessReturn
///   { output: Err(ChainFormsConsumerErrorReturn::Kem(
///   KemConsumerErrorReturn::CreateKeyAgreement(
///   CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier))) })`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, the selected forms concrete's producer,
///   `ChainFormsConsumer::try_new`, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, `KemConsumer::try_new`, `consume_kem`,
///   `create_key_agreement`, and the selected key agreement concrete's
///   `try_new`.
/// Mocked:  the generate and verifier families' mock concretes where the case
///   reaches them.
#[test]
fn the_key_agreement_selection_the_consumer_carries_is_the_concrete_the_factory_constructs() {
    // Arrange
    struct Consumer {
        forms: CreateChainFormsParams,
        key_agreement: AdmittedKeyAgreement,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    key_agreement: Some(self.key_agreement),
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                self.forms,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let declared = DECLARED_KEY_AGREEMENT_SELECTIONS
        .iter()
        .find(|selection| selection.concrete == KeyAgreementConcrete::PairingElGamal)
        .expect("the declared set names the PairingElGamal member");
    let declared_admitted = AdmittedKeyAgreement {
        concrete: declared.concrete,
        identifier: declared.identifier,
        algebra: declared.algebra,
    };
    let mixed_admitted = AdmittedKeyAgreement {
        identifier: build_create_key_agreement_params(Default::default()).identifier,
        ..declared_admitted
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
    let cases: Vec<(
        CreateChainFormsParams,
        GenerateConcrete,
        Vec<VectorKindConfiguration>,
    )> = DECLARED_CHAIN_FORMS_SELECTIONS
        .iter()
        .map(|forms| {
            (
                *forms,
                build_generate_concrete(GENERATE_CONCRETES[0], GenerateConcreteOverrides),
                build_generate_kind_configurations(
                    GENERATE_CONCRETES[0],
                    GenerateKindConfigurationsOverrides,
                ),
            )
        })
        .chain(core::iter::once({
            let params = build_create_generate_params(Default::default());
            (
                build_create_chain_forms_params(Default::default()),
                params.concrete,
                params.kinds,
            )
        }))
        .collect();

    for (forms, generate, kinds) in &cases {
        for pairing_concrete in PAIRING_CONCRETES {
            for key_agreement in [declared_admitted, mixed_admitted] {
                let consumer = Consumer {
                    forms: *forms,
                    key_agreement,
                    generate: generate.clone(),
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
                        key_agreement: Some(*declared),
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
                if key_agreement.identifier == declared.identifier {
                    assert!(matches!(
                        output,
                        Ok(CreateChainFormsSuccessReturn { output: Ok(_) })
                    ));
                } else {
                    assert!(matches!(
                        output,
                        Ok(CreateChainFormsSuccessReturn {
                            output: Err(ChainFormsConsumerErrorReturn::Kem(
                                KemConsumerErrorReturn::CreateKeyAgreement(
                                    CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier
                                )
                            )),
                        })
                    ));
                }
            }
        }
    }
    // the_key_agreement_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
}

/// Contract: absorbed entry
///   `the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports`
///   — the count this consumer holds is the count the measurement reports.
/// Arrange: an iteration count of three and of one, over every member of
///   `PAIRING_CONCRETES` and each KEM selection, the mock forms selection, the
///   other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the mock generate selection, the
///   verifier family's mock concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`
///   and `measurement.iterations.get()` equals the count.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, `ChainFormsConsumer::try_new`,
///   `consume_chain_forms`, `admit_contexts`, `create_kem`,
///   `KemConsumer::try_new`, `consume_kem`, `create_key_agreement`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, `consume_generate`, `emit`,
///   `create_verifier`, `consume_verifier`, and `measure`.
/// Mocked:  the generate and verifier families' mock concretes.
#[test]
fn the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
        iterations: NonZeroU32,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: self.kem.concrete,
                        identifier: self.kem.identifier,
                        scope: self.kem.scope,
                    }),
                    iterations: Some(self.iterations),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
    let kem_selections: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS
        .iter()
        .copied()
        .chain(core::iter::once(
            build_create_kem_params(Default::default()),
        ))
        .collect();

    for count in [3u32, 1u32] {
        for pairing_concrete in PAIRING_CONCRETES {
            for kem_selection in &kem_selections {
                let consumer = Consumer {
                    kem: *kem_selection,
                    iterations: NonZeroU32::new(count).expect("a nonzero count"),
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(keccak256_selection()),
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
                assert_eq!(success.measurement.iterations.get(), count);
            }
        }
    }
    // the_iteration_count_the_consumer_carries_is_the_count_the_measurement_reports
}

/// Contract: absorbed entry
///   `the_statement_version_the_consumer_carries_reaches_the_delivery_proof_factory`
///   — the version this consumer holds reaches `create_delivery_proof` through
///   the KEM consumer, and under `2` the refusal is the output inside
///   `ChainFormsConsumerErrorReturn::Kem`.
/// Arrange: each of `DELIVERY_STATEMENT_VERSION_ONE` and `2`, each member of
///   `DELIVERY_PROOF_CONCRETES` and the mock delivery proof selection, the
///   mock forms selection, every member of `PAIRING_CONCRETES` and each scope
///   of `DECLARED_KEM_SELECTIONS`; the other families' selections the declared
///   sets' first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the mock generate selection, the
///   verifier family's mock concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per case, inside the case consumer.
/// Assert:  the output under version one is `Ok` holding `Ok`, and under `2`
///   matches `Ok(CreateChainFormsSuccessReturn { output:
///   Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::CreateDeliveryProof(
///   CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)))) })`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, `ChainFormsConsumer::try_new`,
///   `consume_chain_forms`, `admit_contexts`, `create_kem`,
///   `KemConsumer::try_new`, `consume_kem`, `create_key_agreement`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`, and
///   `create_delivery_proof`.
/// Mocked:  the generate and verifier families' mock concretes.
#[test]
fn the_statement_version_the_consumer_carries_reaches_the_delivery_proof_factory() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
        delivery_proof: DeliveryProofConcrete,
        statement_version: u16,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: self.kem.concrete,
                        identifier: self.kem.identifier,
                        scope: self.kem.scope,
                    }),
                    delivery_proof: Some(self.delivery_proof),
                    statement_version: Some(self.statement_version),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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

    for statement_version in [DELIVERY_STATEMENT_VERSION_ONE, 2] {
        for pairing_concrete in PAIRING_CONCRETES {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                for delivery_proof in &delivery_proofs {
                    let consumer = Consumer {
                        kem: *kem_selection,
                        delivery_proof: *delivery_proof,
                        statement_version,
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
                        build_create_generate_case_payload(&canonicals, &roots, Default::default()),
                    );

                    // Assert
                    let output = result.expect("the case composes").output;
                    if statement_version == DELIVERY_STATEMENT_VERSION_ONE {
                        assert!(matches!(
                            output,
                            Ok(CreateChainFormsSuccessReturn { output: Ok(_) })
                        ));
                    } else {
                        assert!(matches!(
                            output,
                            Ok(CreateChainFormsSuccessReturn {
                                output: Err(ChainFormsConsumerErrorReturn::Kem(
                                    KemConsumerErrorReturn::KeyAgreement(
                                        KeyAgreementConsumerErrorReturn::CreateDeliveryProof(
                                            CreateDeliveryProofErrorReturn::UnsupportedStatementVersion
                                        )
                                    )
                                )),
                            })
                        ));
                    }
                }
            }
        }
    }
    // the_statement_version_the_consumer_carries_reaches_the_delivery_proof_factory
}

/// Contract: absorbed entry
///   `the_delivery_proof_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`
///   — the selection this consumer holds is the concrete `create_delivery_proof`
///   constructs, through the KEM consumer.
/// Arrange: `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::
///   NonceDrawRefused)`, then `NoFailure` as the counterfactual, the mock forms
///   selection, every member of `PAIRING_CONCRETES` and each scope of
///   `DECLARED_KEM_SELECTIONS`; the other families' selections the declared
///   sets' first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the mock generate selection, the
///   verifier family's mock concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateChainFormsSuccessReturn { output:
///   Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))))))) })`
///   and the output under `NoFailure` holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, `ChainFormsConsumer::try_new`,
///   `consume_chain_forms`, `admit_contexts`, `create_kem`,
///   `KemConsumer::try_new`, `consume_kem`, `create_key_agreement`,
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
        kem: CreateKemParams,
        delivery_proof: DeliveryProofConcrete,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: self.kem.concrete,
                        identifier: self.kem.identifier,
                        scope: self.kem.scope,
                    }),
                    delivery_proof: Some(self.delivery_proof),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
            for delivery_proof in [
                DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NonceDrawRefused),
                DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NoFailure),
            ] {
                let consumer = Consumer {
                    kem: *kem_selection,
                    delivery_proof,
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(keccak256_selection()),
                        encoding: Some(CreateEncodingParams {
                            concrete: EncodingConcrete::Mock,
                            identifier: EncodingIdentifier::Mock,
                        }),
                        chain_forms: None,
                        kem: Some(*kem_selection),
                        key_agreement: Some(DECLARED_KEY_AGREEMENT_SELECTIONS[0]),
                        delivery_proof: Some(build_create_delivery_proof_params(
                            CreateDeliveryProofParamsOverrides {
                                concrete: Some(delivery_proof),
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
                match delivery_proof {
                    DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                    ) => {
                        assert!(matches!(
                            output,
                            Ok(CreateChainFormsSuccessReturn {
                                output: Err(ChainFormsConsumerErrorReturn::Kem(
                                    KemConsumerErrorReturn::KeyAgreement(
                                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                                            DeliveryProofConsumerErrorReturn::Measure(
                                                MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
                                                    ProveMintErrorReturn::Mock(
                                                        MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                                                    )
                                                ))
                                            )
                                        )
                                    )
                                )),
                            })
                        ));
                    }
                    _ => {
                        assert!(matches!(
                            output,
                            Ok(CreateChainFormsSuccessReturn { output: Ok(_) })
                        ));
                    }
                }
            }
        }
    }
    // the_delivery_proof_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
}

/// Contract: absorbed entry
///   `a_refused_write_returns_through_the_consumer_before_the_verifier_is_reached`
///   — the mock concrete's refusal is the output inside
///   `ChainFormsConsumerErrorReturn::Kem`, unchanged.
/// Arrange: `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::
///   WriteRefused)` with `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode
///   ::RpcRefused)`, then the generate mock under `NoFailure` as the
///   counterfactual, the mock forms selection, every member of
///   `PAIRING_CONCRETES`; the other families' selections the declared sets'
///   first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateChainFormsSuccessReturn { output:
///   Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused)))))) })` and the
///   output under `NoFailure` matches `Ok(CreateChainFormsSuccessReturn {
///   output: Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))))) })`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, `ChainFormsConsumer::try_new`,
///   `consume_chain_forms`, `admit_contexts`, `create_kem`,
///   `KemConsumer::try_new`, `consume_kem`, `create_key_agreement`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, the mock concrete's
///   `try_new`, `consume_generate`, and `emit`.
/// Mocked:  the generate and verifier families' mock concretes under their
///   failure modes.
#[test]
fn a_refused_write_returns_through_the_consumer_before_the_verifier_is_reached() {
    // Arrange
    struct Consumer {
        generate: GenerateConcrete,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    generate: Some(self.generate.clone()),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
        for generate in [
            GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused),
            GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure),
        ] {
            let consumer = Consumer {
                generate: generate.clone(),
            };
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(keccak256_selection()),
                    encoding: Some(CreateEncodingParams {
                        concrete: EncodingConcrete::Mock,
                        identifier: EncodingIdentifier::Mock,
                    }),
                    chain_forms: None,
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
            match generate {
                GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused) => {
                    assert!(matches!(
                        output,
                        Ok(CreateChainFormsSuccessReturn {
                            output: Err(ChainFormsConsumerErrorReturn::Kem(
                                KemConsumerErrorReturn::KeyAgreement(
                                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                                        DeliveryProofConsumerErrorReturn::Emit(
                                            EmitErrorReturn::Mock(
                                                MockIGenerateAdapterEmitErrorReturn::WriteRefused
                                            )
                                        )
                                    )
                                )
                            )),
                        })
                    ));
                }
                _ => {
                    assert!(matches!(
                        output,
                        Ok(CreateChainFormsSuccessReturn {
                            output: Err(ChainFormsConsumerErrorReturn::Kem(
                                KemConsumerErrorReturn::KeyAgreement(
                                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                                        DeliveryProofConsumerErrorReturn::Measure(
                                            MeasureErrorReturn::OnChain {
                                                relation: DeliveryRelation::Mint,
                                                error: VerifyOnChainErrorReturn::Mock(
                                                    MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                                ),
                                            }
                                        )
                                    )
                                )
                            )),
                        })
                    ));
                }
            }
        }
    }
    // a_refused_write_returns_through_the_consumer_before_the_verifier_is_reached
}

/// Contract: absorbed entry
///   `a_refused_chain_read_returns_through_the_consumer_after_the_generate_step_succeeded`
///   — the mock concrete's refusal reaches the caller of `create_chain_forms`
///   inside `ChainFormsConsumerErrorReturn::Kem`, unconverted, with the
///   relation it concerned.
/// Arrange: each member of `GENERATE_CONCRETES` with
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)`,
///   then `NoFailure` as the counterfactual, each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, every member of `PAIRING_CONCRETES`;
///   the other families' selections the declared sets' first members, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_chain_forms`, once per mode, inside the case consumer.
/// Assert:  the output under the refusing mode matches
///   `Ok(CreateChainFormsSuccessReturn { output:
///   Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))))) })` and
///   the output under `NoFailure` holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_chain_forms`, `ChainFormsConsumer::try_new`,
///   `consume_chain_forms`, `admit_contexts`, `create_kem`,
///   `KemConsumer::try_new`, `consume_kem`, `create_key_agreement`,
///   `KeyAgreementConsumer::try_new`, `consume_key_agreement`,
///   `create_delivery_proof`, `DeliveryProofConsumer::try_new`,
///   `consume_delivery_proof`, `create_generate`, `consume_generate`, `emit`,
///   `create_verifier`, the mock concrete's `try_new`, `consume_verifier`,
///   `measure`, and `mint`.
/// Mocked:  the verifier family's mock concrete under its failure mode.
#[test]
fn a_refused_chain_read_returns_through_the_consumer_after_the_generate_step_succeeded() {
    // Arrange
    struct Consumer {
        forms: CreateChainFormsParams,
        generate: GenerateConcrete,
        kinds: Vec<VectorKindConfiguration>,
        verifier: VerifierConcrete,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    verifier: Some(self.verifier.clone()),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                self.forms,
                build_create_chain_forms_payload(Default::default()),
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
        for forms in DECLARED_CHAIN_FORMS_SELECTIONS {
            for pairing_concrete in PAIRING_CONCRETES {
                for verifier in [
                    VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused),
                    VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                ] {
                    let consumer = Consumer {
                        forms: *forms,
                        generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                        kinds: build_generate_kind_configurations(
                            *kind,
                            GenerateKindConfigurationsOverrides,
                        ),
                        verifier: verifier.clone(),
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
                    match verifier {
                        VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused) => {
                            assert!(matches!(
                                output,
                                Ok(CreateChainFormsSuccessReturn {
                                    output: Err(ChainFormsConsumerErrorReturn::Kem(
                                        KemConsumerErrorReturn::KeyAgreement(
                                            KeyAgreementConsumerErrorReturn::DeliveryProof(
                                                DeliveryProofConsumerErrorReturn::Measure(
                                                    MeasureErrorReturn::OnChain {
                                                        relation: DeliveryRelation::Mint,
                                                        error: VerifyOnChainErrorReturn::Mock(
                                                            MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                                        ),
                                                    }
                                                )
                                            )
                                        )
                                    )),
                                })
                            ));
                        }
                        _ => {
                            assert!(matches!(
                                output,
                                Ok(CreateChainFormsSuccessReturn { output: Ok(_) })
                            ));
                        }
                    }
                }
            }
        }
    }
    // a_refused_chain_read_returns_through_the_consumer_after_the_generate_step_succeeded
}

/// Contract: absorbed entry
///   `a_refused_draw_reaches_the_caller_of_create_chain_forms_whole` —
///   the refusal of the consumer's held source reaches the caller of
///   `create_chain_forms` inside `ChainFormsConsumerErrorReturn::Kem`,
///   unconverted.
/// Arrange: the source `create_random_source` returns for
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)` through the case params with the mock generate and
///   mock forms selections, then the operating-system source as the
///   counterfactual, every member of `PAIRING_CONCRETES` and each scope of
///   `DECLARED_KEM_SELECTIONS`; the other families' selections the declared
///   sets' first members, the hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the verifier family's mock
///   concrete under `NoFailure`.
/// Act:     `create_chain_forms`, once per source, inside the case consumer.
/// Assert:  the output under the refusing source matches
///   `Ok(CreateChainFormsSuccessReturn { output:
///   Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)))))))
///   })` and the output under the operating-system source holds `Ok`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_random_source`, `create_chain_forms`,
///   `ChainFormsConsumer::try_new`, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, `KemConsumer::try_new`, `consume_kem`,
///   `create_key_agreement`, `KeyAgreementConsumer::try_new`,
///   `consume_key_agreement`, `create_delivery_proof`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, `consume_generate`, `emit`, `create_verifier`,
///   `consume_verifier`, `measure`, and `mint`.
/// Mocked:  the randomness family's mock concrete, selected by configuration.
#[test]
fn a_refused_draw_reaches_the_caller_of_create_chain_forms_whole() {
    // Arrange
    struct Consumer {
        kem: CreateKemParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&payload.identities);
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: self.kem.concrete,
                        identifier: self.kem.identifier,
                        scope: self.kem.scope,
                    }),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
            for source in [
                RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
                RandomSourceKind::OperatingSystem,
            ] {
                let consumer = Consumer {
                    kem: *kem_selection,
                };
                let case_params =
                    build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(keccak256_selection()),
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
                            CreateRandomSourceParamsOverrides { kind: Some(source) },
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
                match source {
                    RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    ) => {
                        assert!(matches!(
                            output,
                            Ok(CreateChainFormsSuccessReturn {
                                output: Err(ChainFormsConsumerErrorReturn::Kem(
                                    KemConsumerErrorReturn::KeyAgreement(
                                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                                            DeliveryProofConsumerErrorReturn::Measure(
                                                MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
                                                    FillBytesErrorReturn::MockIRandomSourceAdapter
                                                ))
                                            )
                                        )
                                    )
                                )),
                            })
                        ));
                    }
                    _ => {
                        assert!(matches!(
                            output,
                            Ok(CreateChainFormsSuccessReturn { output: Ok(_) })
                        ));
                    }
                }
            }
        }
    }
    // a_refused_draw_reaches_the_caller_of_create_chain_forms_whole
}
