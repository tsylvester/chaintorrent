#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use chain::{
    ChainFormsConcrete, CreateChainFormsErrorReturn, CreateChainFormsParams,
    DECLARED_CHAIN_FORMS_SELECTIONS, IChainForms, build_create_chain_forms_params,
};
use core::num::NonZeroU32;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParams, CreateEncodingParamsOverrides, CreateEncodingReturn,
    CreateEncodingSuccessReturn, DECLARED_ENCODING_SELECTIONS, DecodeParams, EncodingConcrete,
    EncodingIdentifier, FromFieldErrorReturn, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use envelope::{
    CreateKeyAgreementParams, CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS,
    IKeyAgreementAdapter, build_create_key_agreement_params,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier, IHashToScalarAdapter,
};
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
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DeliveryProofConcrete,
    IDeliveryProofAdapter, build_create_delivery_proof_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::interface::{EncodingConsumerErrorReturn, EncodingConsumerReturn};
use super::mock::{EncodingConsumerConstructorParamsOverrides, build_encoding_consumer};
use crate::config::provides::{
    AdmittedChainForms, AdmittedIdentities, AdmittedIdentitiesOverrides, AdmittedIdentity,
    AdmittedKem, AdmittedKeyAgreement, AdmittedStatements, AdmittedStatementsOverrides,
    CanonicalDeliveryContextOverrides, build_admitted_identities, build_admitted_statements,
    build_canonical_delivery_context,
};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    CreateGenerateCaseParamsOverrides, GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteKind,
    GenerateConcreteOverrides, GenerateKindConfigurationsOverrides, ICreateGenerateCaseConsumer,
    VectorKind, VectorKindConfiguration, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    build_create_generate_params, build_emit_success_return, build_generate_concrete,
    build_generate_kind_configurations, run_create_generate_case,
};
use crate::run::provides::{
    AdmitContextsErrorReturn, ChainFormsConsumerErrorReturn, DeliveryContextField,
};
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, VerifierConcrete, VerifyCost,
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
///   `the_files_and_the_measurement_reach_the_caller_of_create_encoding_whole`
///   — `create_encoding` over the mock selection and each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS` under the encoding selection the member
///   requires, with a consumer constructed from the adapters the case runner
///   hands, statements and identities of the variant the held KEM selection's
///   scope names, a member of `DECLARED_KEM_SELECTIONS` or the mock KEM
///   selection, a member of `DECLARED_KEY_AGREEMENT_SELECTIONS` or the mock
///   selection, a member of `DELIVERY_PROOF_CONCRETES` or the mock selection,
///   a member of `GENERATE_CONCRETES` or the mock selection under `NoFailure`,
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
/// Act:     `create_encoding`, once per case, inside the case consumer.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four
///   tagged files and each of its paths exists, `measurement.iterations`
///   equals the held count, and the two costs equal the expected cost.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_encoding`, the selected encoding concrete's constructor,
///   `EncodingConsumer::try_new`, `consume_encoding`,
///   `ChainFormsConsumer::try_new`, `create_chain_forms`, the selected forms
///   concrete's producer, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, `KemConsumer::try_new`, `consume_kem`,
///   `create_key_agreement`, `KeyAgreementConsumer::try_new`,
///   `consume_key_agreement`, `create_delivery_proof`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, `consume_generate`, `emit`, `create_verifier`,
///   `consume_verifier`, and `measure`.
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
            let consumer = build_encoding_consumer(
                pairing,
                hash_to_scalar,
                random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(self.chain_forms),
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
///   `the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with`
///   and the absorbed `encoding/factory` entry `every concrete is handed to
///   the consumer under the identifier it declares` — `create_encoding` over
///   the encoding selection the case names → the lines of the file tagged
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
/// Act:     `create_encoding`, once per case, inside the case consumer, then
///   `create_encoding` once for the other selection's decoder.
/// Assert:  each line of the file tagged `Some(VectorKind::Challenge)`, its
///   hex after `0x` read back into bytes, decodes through the received
///   decoder's `decode` over `DecodeParams { description: &description }` with
///   `description` the `ChallengeVectorDescription` built by its `try_new`
///   unpacked irrefutably, and the same bytes decoded through the decoder of
///   the other encoding selection are refused.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_encoding`, the selected encoding concrete's constructor,
///   `EncodingConsumer::try_new`, `consume_encoding`,
///   `ChainFormsConsumer::try_new`, `create_chain_forms`, the selected forms
///   concrete's producer, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, `KemConsumer::try_new`, `consume_kem`,
///   `create_key_agreement`, `KeyAgreementConsumer::try_new`,
///   `consume_key_agreement`, `create_delivery_proof`,
///   `DeliveryProofConsumer::try_new`, `consume_delivery_proof`,
///   `create_generate`, `consume_generate`, `emit`, `create_verifier`,
///   `consume_verifier`, `measure`, and the decoder's `decode`.
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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let consumer = build_encoding_consumer(
                pairing,
                hash_to_scalar,
                random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(self.chain_forms),
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    ..Default::default()
                },
            );
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
///   `the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs`
///   — `create_encoding` and a consumer constructed with an
///   `AdmittedChainForms` built from a member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, and the same selection with the
///   `identifier` the mock selection declares → `Ok` whose output is `Ok` for
///   the declared member and `Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier))` for the
///   mixed selection, the forms consumer not reached.
/// Arrange: the two selections over every member of `PAIRING_CONCRETES`, under
///   the encoding selection the member requires; `generate` and `kinds` the
///   member of `GENERATE_CONCRETES` and its declared kind configurations, since
///   the mock generate concrete's declaration requires
///   `EncodingIdentifier::Mock` and would refuse the ABI encoder before
///   `emit`; the other families' selections the mock defaults, the
///   hash-to-scalar selection a member declaring
///   `HashToScalarIdentifier::Keccak256V1`, the verifier family's mock
///   concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_encoding`, once per selection, inside the case consumer.
/// Assert:  the output under the declared selection holds `Ok` and the
///   output under the mixed selection matches `Ok(CreateEncodingSuccessReturn
///   { output: Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier)) })`.
/// Boundary: `run_create_generate_case`, `create_encoding`, `consume_encoding`,
///   and `create_chain_forms`; Mocked the encoding family's concrete the case
///   selects.
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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let consumer = build_encoding_consumer(
                pairing,
                hash_to_scalar,
                random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(self.chain_forms),
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    ..Default::default()
                },
            );
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
///   `an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs`
///   and the absorbed `chain/create_chain_forms` entry
///   `chain_forms_refuse_an_encoder_declaring_another_encoding_identifier` —
///   `create_encoding` over the mock encoding selection with a member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, and over the Ethereum ABI selection
///   with the mock forms selection → `Ok` whose output is
///   `Err(EncodingConsumerErrorReturn::CreateChainForms(
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
/// Act:     `create_encoding`, once per pairing, inside the case consumer.
/// Assert:  each output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)) })` and no
///   path the emit would name exists.
/// Boundary: `run_create_generate_case`, `create_encoding`, `consume_encoding`,
///   and `create_chain_forms`; Mocked the encoding family's concrete the case
///   selects.
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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = admitted_identities_of(&identities);
            let consumer = build_encoding_consumer(
                pairing,
                hash_to_scalar,
                random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(self.chain_forms),
                    ..Default::default()
                },
            );
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

/// Contract: absorbed entry
///   `the_kem_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`
///   of `harness-crypto/run/chain_forms_consumer` — the KEM selection this
///   consumer holds reaches the forms consumer unaltered, and the refusal of
///   an undeclared identifier is the output inside
///   `EncodingConsumerErrorReturn::ChainForms`.
/// Arrange: the `AdmittedKem` built from the `Bb1DepthOne` member of
///   `DECLARED_KEM_SELECTIONS`, then the same selection with the `identifier`
///   the mock selection declares as the counterfactual, over every member of
///   `PAIRING_CONCRETES` and each forms selection the generate selection's
///   declared encoding admits — the mock generate selection under the mock
///   forms and a member of `GENERATE_CONCRETES` under each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`; the other families' selections the
///   declared sets' first members, the hash-to-scalar selection a member
///   declaring `HashToScalarIdentifier::Keccak256V1`, the verifier family's
///   mock concrete under `NoFailure`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_encoding`, once per selection, inside the case consumer.
/// Assert:  the output under the declared selection holds `Ok` and the
///   output under the mixed selection matches `Ok(CreateEncodingSuccessReturn
///   { output: Err(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::CreateKem(
///   CreateKemErrorReturn::UnsupportedKemIdentifier))) })`.
/// Boundary: `run_create_generate_case`, the factories it composes,
///   `create_encoding`, `consume_encoding`, `create_chain_forms`, the selected
///   forms concrete's producer, `consume_chain_forms`, `admit_contexts`,
///   `create_kem`, and the selected KEM concrete's `try_new`.
/// Mocked:  the generate and verifier families' mock concretes where the case
///   reaches them.
#[test]
fn the_kem_selection_the_consumer_carries_is_the_concrete_the_factory_constructs() {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        kem: AdmittedKem,
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
            let consumer = build_encoding_consumer(
                pairing,
                hash_to_scalar,
                random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(self.chain_forms),
                    kem: Some(self.kem),
                    generate: Some(self.generate.clone()),
                    kinds: Some(self.kinds.clone()),
                    ..Default::default()
                },
            );
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
                    chain_forms: AdmittedChainForms {
                        concrete: forms.concrete,
                        identifier: forms.identifier,
                    },
                    encoding: encoding_for(*forms),
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
                        Ok(CreateEncodingSuccessReturn { output: Ok(_) })
                    ));
                } else {
                    assert!(matches!(
                        output,
                        Ok(CreateEncodingSuccessReturn {
                            output: Err(EncodingConsumerErrorReturn::ChainForms(
                                ChainFormsConsumerErrorReturn::CreateKem(
                                    CreateKemErrorReturn::UnsupportedKemIdentifier
                                )
                            )),
                        })
                    ));
                }
            }
        }
    }
    // the_kem_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
}

/// Contract: absorbed entry
///   `a_refused_context_names_the_field_that_held_its_value_and_returns_before_the_kem_factory_is_called`
///   of `harness-crypto/run/chain_forms_consumer` — the refusal of a context
///   returns from `consume_encoding` with the field that held the value,
///   inside `EncodingConsumerErrorReturn::ChainForms`.
/// Arrange: `AdmittedStatements` whose mint context holds
///   `CanonicalFieldValue::Text` as its `seller`, then whose transfer context
///   holds it as its `buyer` with the mint context at its defaults, then both,
///   and the mixed `AdmittedKem` of the block above, over each member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS` and the mock forms selection under the
///   encoding selection each requires, with the mock generate selection and
///   the verifier family's mock concrete under `NoFailure`; the hash-to-scalar
///   selection a member declaring `HashToScalarIdentifier::Keccak256V1`, the
///   random source `RandomSourceKind::OperatingSystem`.
/// Act:     `create_encoding`, once per arrangement and per pairing, inside
///   the case consumer.
/// Assert:  the outputs match `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::AdmitMint(AdmitContextsErrorReturn::Field
///   { field: DeliveryContextField::Seller, error:
///   FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } }))) })` for the first and the third
///   arrangements and the same shape under
///   `ChainFormsConsumerErrorReturn::AdmitTransfer` with
///   `DeliveryContextField::Buyer` for the second.
/// Boundary: `run_create_generate_case`, `create_encoding`, `consume_encoding`,
///   `create_chain_forms`, `consume_chain_forms`, and `admit_contexts`; Mocked
///   the encoding family's concrete the case selects.
#[test]
fn a_refused_context_names_the_field_that_held_its_value_and_returns_before_the_kem_factory_is_called()
 {
    // Arrange
    struct Consumer {
        chain_forms: AdmittedChainForms,
        encoding: CreateEncodingParams,
        statements: AdmittedStatements,
        kem: AdmittedKem,
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
            let admitted_identities = admitted_identities_of(&identities);
            let consumer = build_encoding_consumer(
                pairing,
                hash_to_scalar,
                random,
                &self.statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(self.chain_forms),
                    kem: Some(self.kem),
                    ..Default::default()
                },
            );
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
                chain_forms: AdmittedChainForms {
                    concrete: forms.concrete,
                    identifier: forms.identifier,
                },
                encoding: encoding_for(*forms),
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
                    Ok(CreateEncodingSuccessReturn {
                        output: Err(EncodingConsumerErrorReturn::ChainForms(
                            ChainFormsConsumerErrorReturn::AdmitMint(
                                AdmitContextsErrorReturn::Field {
                                    field: DeliveryContextField::Seller,
                                    error: FromFieldErrorReturn::WrongKind {
                                        expected: CanonicalFieldKind::FixedBytes20
                                    }
                                }
                            )
                        )),
                    })
                ));
            } else {
                assert!(matches!(
                    output,
                    Ok(CreateEncodingSuccessReturn {
                        output: Err(EncodingConsumerErrorReturn::ChainForms(
                            ChainFormsConsumerErrorReturn::AdmitTransfer(
                                AdmitContextsErrorReturn::Field {
                                    field: DeliveryContextField::Buyer,
                                    error: FromFieldErrorReturn::WrongKind {
                                        expected: CanonicalFieldKind::FixedBytes20
                                    }
                                }
                            )
                        )),
                    })
                ));
            }
        }
    }
    // a_refused_context_names_the_field_that_held_its_value_and_returns_before_the_kem_factory_is_called
}
