#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use chain::{
    ChainFormsConcrete, CreateChainFormsErrorReturn, CreateChainFormsParams,
    DECLARED_CHAIN_FORMS_SELECTIONS, build_create_chain_forms_params,
};
use core::num::NonZeroU32;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingErrorReturn, CreateEncodingParams,
    CreateEncodingPayload, DECLARED_ENCODING_SELECTIONS, DecodeParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    create_encoding,
};
use envelope::{
    CreateKeyAgreementParams, DECLARED_KEY_AGREEMENT_SELECTIONS, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarIdentifier, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParams, DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, IdentityScope,
    SetupErrorReturn, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, CreatePairingPayload, CreatePairingSuccessReturn,
    PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::{
    DELIVERY_PROOF_CONCRETES, DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
};
use random::{
    CreateRandomSourceParamsOverrides, RandomSourceKind, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};

use harness_crypto::{
    AdmittedChainForms, AdmittedEncoding, AdmittedHashToScalar, AdmittedIdentities,
    AdmittedIdentitiesOverrides, AdmittedIdentity, AdmittedKem, AdmittedKeyAgreement,
    AdmittedStatementsOverrides, ChainFormsConsumerErrorReturn, ChallengeVectorDescription,
    ChallengeVectorDescriptionConstructorParams, CreateGenerateErrorReturn,
    DeliveryProofConsumerErrorReturn, DeliveryProofConsumerSuccessReturn,
    EncodingConsumerErrorReturn, GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, KemConsumerErrorReturn, KeyAgreementConsumerErrorReturn,
    L1DataFee, MeasureErrorReturn, MintErrorReturn, MockIVerifierAdapterFailureMode,
    PairingConsumer, PairingConsumerConstructorParams, PairingConsumerErrorReturn,
    VectorCountConstructorParamsOverrides, VectorKind, VectorKindConfiguration,
    VectorKindConfigurationOverrides, VectorKindQuantitiesOverrides, VerifierConcrete, VerifyCost,
    build_admitted_identities, build_admitted_statements, build_create_generate_params,
    build_emit_success_return, build_generate_concrete, build_generate_kind_configurations,
    build_vector_count, build_vector_kind_configuration, build_vector_kind_quantities,
};

fn admitted_identities_of(scope: IdentityScope, count: usize) -> AdmittedIdentities {
    let canonicals: [&[u8]; 3] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
        b"example-package@3.0.0".as_slice(),
    ];
    let roots = [
        build_root(Default::default()),
        build_root(RootConstructorParamsOverrides {
            bytes: Some([0x11; 32]),
        }),
        build_root(RootConstructorParamsOverrides {
            bytes: Some([0x22; 32]),
        }),
    ];
    let mapping: Vec<AdmittedIdentity> = (0..count)
        .map(|index| match scope {
            IdentityScope::Entitlement => AdmittedIdentity::Entitlement {
                canonical: canonicals[index].to_vec(),
            },
            IdentityScope::Asset => AdmittedIdentity::Asset { root: roots[index] },
        })
        .collect();
    build_admitted_identities(AdmittedIdentitiesOverrides {
        proof: Some(mapping[0].clone()),
        mapping: Some(mapping),
    })
}

fn encoding_for(forms: CreateChainFormsParams) -> CreateEncodingParams {
    if matches!(forms.concrete, ChainFormsConcrete::Mock) {
        build_create_encoding_params(Default::default())
    } else {
        CreateEncodingParams {
            concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
            identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
        }
    }
}

/// Contract: own entry
///   `the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole`,
///   as an outside caller observes it — `create_pairing` over each member of
///   `PAIRING_CONCRETES` with params naming only the concrete, and over the
///   mock selection, with a consumer constructed through `PairingConsumer::
///   try_new` from `PairingConsumerConstructorParams` → `Ok` whose output is
///   `Ok(DeliveryProofConsumerSuccessReturn)` whose `files` name no path
///   twice, include for a real generate concrete the four tagged files each
///   of whose paths exists on the filesystem, and whose
///   `measurement.iterations` equals the held count and whose costs each
///   equal the literal cost the verifier family's mock concrete returns.
/// Arrange: every member of `GENERATE_CONCRETES` and the mock generate
///   selection, each member of `DELIVERY_PROOF_CONCRETES` and the mock
///   selection, each key agreement selection, each member of
///   `DECLARED_KEM_SELECTIONS` and the mock KEM selection, each
///   hash-to-scalar selection the entry names, and the pairings the entry
///   names, with `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::
///   NoFailure)`, the encoding selection paired with the forms selection
///   that requires its identifier, the random source
///   `RandomSourceKind::OperatingSystem`, an iteration count of three, and
///   the expected cost the literal gas `100_000` and fee `L1DataFee::Charged
///   { wei: 1_000 }`; a real forms selection runs only under a real generate
///   concrete and the mock forms only under the mock, since the mock
///   generate concrete's declaration requires `EncodingIdentifier::Mock`.
/// Act:     `create_pairing` through the crate root, once per case.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four
///   tagged files and each of its paths exists, `measurement.iterations`
///   equals the held count, and the two costs equal the expected cost.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root, over the route `create_pairing`, the selected
///   pairing concrete's `try_new`, `consume_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `consume_encoding`, `create_chain_forms`,
///   `consume_chain_forms`, `admit_contexts`, `create_kem`,
///   `create_key_agreement`, `create_delivery_proof`, `create_generate`,
///   `emit`, `create_verifier`, `consume_verifier`, and `measure`.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let expected_cost = VerifyCost {
        l2_execution_gas: 100_000,
        l1_data_fee: L1DataFee::Charged { wei: 1_000 },
    };
    let forms_cases: Vec<(CreateChainFormsParams, CreateEncodingParams, bool)> =
        DECLARED_CHAIN_FORMS_SELECTIONS
            .iter()
            .map(|forms| (*forms, encoding_for(*forms), true))
            .chain(core::iter::once({
                let forms = build_create_chain_forms_params(Default::default());
                (forms, encoding_for(forms), false)
            }))
            .collect();
    let generate_cases: Vec<(GenerateConcrete, Vec<VectorKindConfiguration>, bool)> =
        GENERATE_CONCRETES
            .iter()
            .map(|kind| {
                (
                    build_generate_concrete(*kind, GenerateConcreteOverrides),
                    build_generate_kind_configurations(*kind, GenerateKindConfigurationsOverrides),
                    true,
                )
            })
            .chain(core::iter::once({
                let params = build_create_generate_params(Default::default());
                (params.concrete, params.kinds, false)
            }))
            .collect();
    let pairing_cases: Vec<Option<PairingConcrete>> = PAIRING_CONCRETES
        .iter()
        .map(|concrete| Some(*concrete))
        .chain(core::iter::once(None))
        .collect();
    let hash_to_scalar_cases: Vec<AdmittedHashToScalar> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .map(|selection| AdmittedHashToScalar {
            concrete: selection.concrete,
            identifier: selection.identifier,
        })
        .chain(core::iter::once({
            let params = build_create_hash_to_scalar_params(Default::default());
            AdmittedHashToScalar {
                concrete: params.concrete,
                identifier: params.identifier,
            }
        }))
        .collect();
    let kem_cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS
        .iter()
        .copied()
        .chain(core::iter::once(
            build_create_kem_params(Default::default()),
        ))
        .collect();
    let key_agreement_cases: Vec<CreateKeyAgreementParams> = DECLARED_KEY_AGREEMENT_SELECTIONS
        .iter()
        .copied()
        .chain(core::iter::once(build_create_key_agreement_params(
            Default::default(),
        )))
        .collect();
    let delivery_cases: Vec<DeliveryProofConcrete> = DELIVERY_PROOF_CONCRETES
        .iter()
        .copied()
        .chain(core::iter::once(
            build_create_delivery_proof_params(Default::default()).concrete,
        ))
        .collect();

    for (forms, encoding, real_forms) in &forms_cases {
        for (generate, kinds, real_generate) in &generate_cases {
            if real_forms != real_generate {
                continue;
            }
            for pairing_concrete in &pairing_cases {
                for hash_to_scalar in &hash_to_scalar_cases {
                    for kem in &kem_cases {
                        for key_agreement in &key_agreement_cases {
                            for delivery_proof in &delivery_cases {
                                let admitted_identities = admitted_identities_of(kem.scope, 2);
                                let Ok(consumer) =
                                    PairingConsumer::try_new(PairingConsumerConstructorParams {
                                        random: &*random_source.adapter,
                                        statements: &admitted_statements,
                                        identities: &admitted_identities,
                                        hash_to_scalar: *hash_to_scalar,
                                        encoding: AdmittedEncoding {
                                            concrete: encoding.concrete,
                                            identifier: encoding.identifier,
                                        },
                                        chain_forms: AdmittedChainForms {
                                            concrete: forms.concrete,
                                            identifier: forms.identifier,
                                        },
                                        kem: AdmittedKem {
                                            concrete: kem.concrete,
                                            identifier: kem.identifier,
                                            scope: kem.scope,
                                        },
                                        key_agreement: AdmittedKeyAgreement {
                                            concrete: key_agreement.concrete,
                                            identifier: key_agreement.identifier,
                                            algebra: key_agreement.algebra,
                                        },
                                        delivery_proof: *delivery_proof,
                                        generate: generate.clone(),
                                        kinds: kinds.clone(),
                                        verifier: VerifierConcrete::Mock(
                                            MockIVerifierAdapterFailureMode::NoFailure,
                                        ),
                                        statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                                        iterations: NonZeroU32::new(3).expect("three is nonzero"),
                                    });

                                // Act
                                let result = create_pairing(
                                    &build_create_pairing_deps(consumer, Default::default()),
                                    build_create_pairing_params(CreatePairingParamsOverrides {
                                        concrete: *pairing_concrete,
                                        ..Default::default()
                                    }),
                                    CreatePairingPayload,
                                );

                                // Assert
                                let success = result
                                    .expect("the factory admits the concrete and runs the consumer")
                                    .output
                                    .expect("the consumer calls the factories and the chain beneath them");
                                let mut paths = HashSet::new();
                                for file in &success.files {
                                    assert!(
                                        paths.insert(file.path.clone()),
                                        "no file path is named twice"
                                    );
                                }
                                if *real_generate {
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
                                        assert!(
                                            file.path.exists(),
                                            "a real concrete's path exists"
                                        );
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
    }
    // the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole
}

/// Contract: own entry
///   `the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`,
///   as an outside caller observes it — `create_pairing` and a consumer
///   constructed with an `AdmittedHashToScalar` built from a member of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, and the same selection with the
///   `identifier` the mock selection declares → `Ok` whose output is `Ok`
///   for the declared member and `Err(PairingConsumerErrorReturn::
///   CreateHashToScalar(CreateHashToScalarErrorReturn::
///   UnsupportedHashToScalarIdentifier))` for the mixed selection.
/// Arrange: the two selections over every member of `PAIRING_CONCRETES`, the
///   mock generate selection and the verifier family's mock concrete under
///   `NoFailure`, and every other selection the mock default.
/// Act:     `create_pairing` through the crate root, once per selection and
///   per pairing.
/// Assert:  the output under the declared selection holds
///   `Ok(DeliveryProofConsumerSuccessReturn { .. })`, and the output under
///   the mixed selection matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateHashToScalar(
///   CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)) })`.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route `create_pairing`, `consume_pairing`,
///   and `create_hash_to_scalar`; Mocked the pairing family's concrete the
///   case selects.
#[test]
fn the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let mock_identifier = build_create_hash_to_scalar_params(Default::default()).identifier;

    for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
        for pairing_concrete in PAIRING_CONCRETES {
            for identifier in [selection.identifier, mock_identifier] {
                let scope = build_create_kem_params(Default::default()).scope;
                let admitted_identities = admitted_identities_of(scope, 2);
                let generate_params = build_create_generate_params(Default::default());
                let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                    random: &*random_source.adapter,
                    statements: &admitted_statements,
                    identities: &admitted_identities,
                    hash_to_scalar: AdmittedHashToScalar {
                        concrete: selection.concrete,
                        identifier,
                    },
                    encoding: AdmittedEncoding {
                        concrete: build_create_encoding_params(Default::default()).concrete,
                        identifier: build_create_encoding_params(Default::default()).identifier,
                    },
                    chain_forms: AdmittedChainForms {
                        concrete: build_create_chain_forms_params(Default::default()).concrete,
                        identifier: build_create_chain_forms_params(Default::default()).identifier,
                    },
                    kem: AdmittedKem {
                        concrete: build_create_kem_params(Default::default()).concrete,
                        identifier: build_create_kem_params(Default::default()).identifier,
                        scope,
                    },
                    key_agreement: AdmittedKeyAgreement {
                        concrete: build_create_key_agreement_params(Default::default()).concrete,
                        identifier: build_create_key_agreement_params(Default::default())
                            .identifier,
                        algebra: build_create_key_agreement_params(Default::default()).algebra,
                    },
                    delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                    generate: generate_params.concrete,
                    kinds: generate_params.kinds,
                    verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                    statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                    iterations: NonZeroU32::MIN,
                });

                // Act
                let result = create_pairing(
                    &build_create_pairing_deps(consumer, Default::default()),
                    build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    }),
                    CreatePairingPayload,
                );

                // Assert
                if identifier == selection.identifier {
                    assert!(matches!(
                        result,
                        Ok(CreatePairingSuccessReturn {
                            output: Ok(DeliveryProofConsumerSuccessReturn { .. })
                        })
                    ));
                } else {
                    assert!(matches!(
                        result,
                        Ok(CreatePairingSuccessReturn {
                            output: Err(PairingConsumerErrorReturn::CreateHashToScalar(
                                CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier
                            ))
                        })
                    ));
                }
            }
        }
    }
    // the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
}

/// Contract: own entry
///   `the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs`,
///   as an outside caller observes it — `create_pairing` and a consumer
///   constructed with an `AdmittedEncoding` built from a member of
///   `DECLARED_ENCODING_SELECTIONS` paired with a member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS` that requires its identifier, and the
///   same selection with the `identifier` the mock selection declares → `Ok`
///   whose output is `Ok` for the declared member and
///   `Err(PairingConsumerErrorReturn::CreateEncoding(
///   CreateEncodingErrorReturn::UnsupportedEncodingIdentifier))` for the
///   mixed selection.
/// Arrange: the two selections over every member of `PAIRING_CONCRETES`, the
///   `AdmittedChainForms` of the member of `DECLARED_CHAIN_FORMS_SELECTIONS`
///   that requires the encoding identifier, `generate` and `kinds` the
///   member of `GENERATE_CONCRETES` and its declared kind configurations
///   since the mock generate concrete's declaration requires
///   `EncodingIdentifier::Mock` and would refuse the ABI encoder before
///   `emit`, the verifier family's mock concrete under `NoFailure`, and
///   every other selection the mock default.
/// Act:     `create_pairing` through the crate root, once per selection and
///   per pairing.
/// Assert:  the output under the declared selection holds
///   `Ok(DeliveryProofConsumerSuccessReturn { .. })`, and the output under
///   the mixed selection matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateEncoding(
///   CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)) })`.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route `create_pairing`, `consume_pairing`,
///   `create_hash_to_scalar`, and `create_encoding`; Mocked the pairing
///   family's concrete the case selects.
#[test]
fn the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let declared_forms = DECLARED_CHAIN_FORMS_SELECTIONS[0];
    let mock_identifier = build_create_encoding_params(Default::default()).identifier;

    for selection in DECLARED_ENCODING_SELECTIONS {
        for pairing_concrete in PAIRING_CONCRETES {
            for identifier in [selection.identifier, mock_identifier] {
                let scope = build_create_kem_params(Default::default()).scope;
                let admitted_identities = admitted_identities_of(scope, 2);
                let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                    random: &*random_source.adapter,
                    statements: &admitted_statements,
                    identities: &admitted_identities,
                    hash_to_scalar: AdmittedHashToScalar {
                        concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                        identifier: build_create_hash_to_scalar_params(Default::default())
                            .identifier,
                    },
                    encoding: AdmittedEncoding {
                        concrete: selection.concrete,
                        identifier,
                    },
                    chain_forms: AdmittedChainForms {
                        concrete: declared_forms.concrete,
                        identifier: declared_forms.identifier,
                    },
                    kem: AdmittedKem {
                        concrete: build_create_kem_params(Default::default()).concrete,
                        identifier: build_create_kem_params(Default::default()).identifier,
                        scope,
                    },
                    key_agreement: AdmittedKeyAgreement {
                        concrete: build_create_key_agreement_params(Default::default()).concrete,
                        identifier: build_create_key_agreement_params(Default::default())
                            .identifier,
                        algebra: build_create_key_agreement_params(Default::default()).algebra,
                    },
                    delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                    generate: build_generate_concrete(
                        GENERATE_CONCRETES[0],
                        GenerateConcreteOverrides,
                    ),
                    kinds: build_generate_kind_configurations(
                        GENERATE_CONCRETES[0],
                        GenerateKindConfigurationsOverrides,
                    ),
                    verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                    statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                    iterations: NonZeroU32::MIN,
                });

                // Act
                let result = create_pairing(
                    &build_create_pairing_deps(consumer, Default::default()),
                    build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    }),
                    CreatePairingPayload,
                );

                // Assert
                if identifier == selection.identifier {
                    assert!(matches!(
                        result,
                        Ok(CreatePairingSuccessReturn {
                            output: Ok(DeliveryProofConsumerSuccessReturn { .. })
                        })
                    ));
                } else {
                    assert!(matches!(
                        result,
                        Ok(CreatePairingSuccessReturn {
                            output: Err(PairingConsumerErrorReturn::CreateEncoding(
                                CreateEncodingErrorReturn::UnsupportedEncodingIdentifier
                            ))
                        })
                    ));
                }
            }
        }
    }
    // the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs
}

/// Contract: own entry `the_hash_to_scalar_refusal_precedes_the_encoding_refusal`,
///   as an outside caller observes it — `create_pairing` and a consumer
///   constructed with a mixed `AdmittedHashToScalar` and a mixed
///   `AdmittedEncoding` together → `Ok` whose output is
///   `Err(PairingConsumerErrorReturn::CreateHashToScalar(
///   CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier))` for
///   every pairing, so the hash-to-scalar adapter is obtained before the
///   encoding family is called.
/// Arrange: the mixed `AdmittedHashToScalar` of the block above together
///   with the mixed `AdmittedEncoding` of the block above, the mock generate
///   selection, and every member of `PAIRING_CONCRETES`.
/// Act:     `create_pairing` through the crate root, once per pairing.
/// Assert:  each output matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateHashToScalar(
///   CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)) })`
///   and no path exists beneath the case's output directory.
/// Boundary and Mocked: as the block above states.
#[test]
fn the_hash_to_scalar_refusal_precedes_the_encoding_refusal() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let mixed_hash_to_scalar = AdmittedHashToScalar {
        concrete: DECLARED_HASH_TO_SCALAR_SELECTIONS[0].concrete,
        identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
    };
    let mixed_encoding = AdmittedEncoding {
        concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
        identifier: build_create_encoding_params(Default::default()).identifier,
    };

    for pairing_concrete in PAIRING_CONCRETES {
        let scope = build_create_kem_params(Default::default()).scope;
        let admitted_identities = admitted_identities_of(scope, 2);
        let generate_params = build_create_generate_params(Default::default());
        let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
            random: &*random_source.adapter,
            statements: &admitted_statements,
            identities: &admitted_identities,
            hash_to_scalar: mixed_hash_to_scalar,
            encoding: mixed_encoding,
            chain_forms: AdmittedChainForms {
                concrete: build_create_chain_forms_params(Default::default()).concrete,
                identifier: build_create_chain_forms_params(Default::default()).identifier,
            },
            kem: AdmittedKem {
                concrete: build_create_kem_params(Default::default()).concrete,
                identifier: build_create_kem_params(Default::default()).identifier,
                scope,
            },
            key_agreement: AdmittedKeyAgreement {
                concrete: build_create_key_agreement_params(Default::default()).concrete,
                identifier: build_create_key_agreement_params(Default::default()).identifier,
                algebra: build_create_key_agreement_params(Default::default()).algebra,
            },
            delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
            generate: generate_params.concrete,
            kinds: generate_params.kinds,
            verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
            statement_version: DELIVERY_STATEMENT_VERSION_ONE,
            iterations: NonZeroU32::MIN,
        });

        // Act
        let result = create_pairing(
            &build_create_pairing_deps(consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );

        // Assert
        assert!(matches!(
            result,
            Ok(CreatePairingSuccessReturn {
                output: Err(PairingConsumerErrorReturn::CreateHashToScalar(
                    CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier
                ))
            })
        ));
        for file in build_emit_success_return(Default::default()).files {
            assert!(
                !file.path.exists(),
                "the refusal returns before the emit writes"
            );
        }
    }
    // the_hash_to_scalar_refusal_precedes_the_encoding_refusal
}

/// Contract: own entry
///   `the_files_are_encoded_under_the_encoding_selection_the_consumer_holds`,
///   as an outside caller observes it — `create_pairing` over each pairing
///   named above, with a real generate concrete, and the files tagged
///   `Some(VectorKind::Challenge)` decoded through the decoder of the held
///   encoding selection → the lines of that file decode to `ChallengeVector`s
///   under the decoder of the held selection and are refused by the decoder
///   of the other encoding selection.
/// Arrange: each pairing the entry names, with a member of
///   `GENERATE_CONCRETES` under the Ethereum ABI selection and the mock
///   generate selection under the mock encoding selection, the mock KEM, key
///   agreement, and delivery proof selections, each encoding paired with the
///   forms selection that requires its identifier, and
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`.
/// Act:     `create_pairing` through the crate root, once per pairing and
///   per encoding selection, then `create_encoding` once per decoder probe.
/// Assert:  each line of the file tagged `Some(VectorKind::Challenge)`, its
///   hex after `0x` read back into bytes, decodes through the decoder of the
///   held encoding selection over `DecodeParams { description:
///   &description }` with `description` the `ChallengeVectorDescription`
///   built by its `try_new` unpacked irrefutably, to `ChallengeVector`s, and
///   the same bytes decoded through the decoder of the other encoding
///   selection are refused.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route the entry states, then
///   `create_encoding` over a consumer of the test's own for the decoders.
/// Mocked:  the verifier family's mock concrete, the filesystem and the
///   operating system's generator and monotonic clock running real.
#[test]
fn the_files_are_encoded_under_the_encoding_selection_the_consumer_holds() {
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

    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let declared_forms = DECLARED_CHAIN_FORMS_SELECTIONS[0];
    let mock_forms = build_create_chain_forms_params(Default::default());
    let mock_generate = build_create_generate_params(Default::default());
    let cases: Vec<(
        CreateEncodingParams,
        CreateChainFormsParams,
        GenerateConcrete,
        Vec<VectorKindConfiguration>,
        CreateEncodingParams,
    )> = vec![
        (
            encoding_for(declared_forms),
            declared_forms,
            build_generate_concrete(GENERATE_CONCRETES[0], GenerateConcreteOverrides),
            build_generate_kind_configurations(
                GENERATE_CONCRETES[0],
                GenerateKindConfigurationsOverrides,
            ),
            build_create_encoding_params(Default::default()),
        ),
        (
            build_create_encoding_params(Default::default()),
            mock_forms,
            mock_generate.concrete,
            mock_generate.kinds,
            encoding_for(declared_forms),
        ),
    ];

    for pairing_concrete in PAIRING_CONCRETES {
        for (encoding, forms, generate, kinds, other_encoding) in &cases {
            let scope = build_create_kem_params(Default::default()).scope;
            let admitted_identities = admitted_identities_of(scope, 2);
            let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                random: &*random_source.adapter,
                statements: &admitted_statements,
                identities: &admitted_identities,
                hash_to_scalar: AdmittedHashToScalar {
                    concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                    identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
                },
                encoding: AdmittedEncoding {
                    concrete: encoding.concrete,
                    identifier: encoding.identifier,
                },
                chain_forms: AdmittedChainForms {
                    concrete: forms.concrete,
                    identifier: forms.identifier,
                },
                kem: AdmittedKem {
                    concrete: build_create_kem_params(Default::default()).concrete,
                    identifier: build_create_kem_params(Default::default()).identifier,
                    scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: build_create_key_agreement_params(Default::default()).concrete,
                    identifier: build_create_key_agreement_params(Default::default()).identifier,
                    algebra: build_create_key_agreement_params(Default::default()).algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: generate.clone(),
                kinds: kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });

            // Act
            let result = create_pairing(
                &build_create_pairing_deps(consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            let success = result
                .expect("the factory admits the concrete and runs the consumer")
                .output
                .expect("the consumer calls the factories and the chain beneath them");
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
                    (0..hex.len())
                        .step_by(2)
                        .map(|index| {
                            u8::from_str_radix(&hex[index..index + 2], 16)
                                .expect("every written line is hex")
                        })
                        .collect()
                })
                .collect();
            assert!(!lines.is_empty(), "the challenge file holds lines");
            for (selection, refused) in [(encoding, false), (other_encoding, true)] {
                let probe = DecodeConsumer {
                    lines: &lines,
                    description: &description,
                };
                let outcomes = create_encoding(
                    &build_create_encoding_deps(probe),
                    CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    },
                    CreateEncodingPayload,
                )
                .expect("the factory admits the selection")
                .output;
                assert!(
                    outcomes.iter().all(|outcome| *outcome == refused),
                    "only the held selection's decoder accepts the bytes"
                );
            }
        }
    }
    // the_files_are_encoded_under_the_encoding_selection_the_consumer_holds
}

/// Contract: carried entry
///   `the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs`
///   of `harness-crypto/run/encoding_consumer`, as an outside caller
///   observes it through this consumer — the chain forms selection this
///   consumer holds reaches the encoding consumer unaltered, and the refusal
///   of a forms identifier the named concrete does not declare arrives as
///   `PairingConsumerErrorReturn::Encoding(EncodingConsumerErrorReturn::
///   CreateChainForms(CreateChainFormsErrorReturn::
///   UnsupportedChainFormsIdentifier))`.
/// Arrange: the `AdmittedChainForms` built from a member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, then the same selection with the
///   `identifier` of the mock selection, under the Ethereum ABI encoding
///   selection, `generate` and `kinds` the member of `GENERATE_CONCRETES`
///   and its declared kind configurations since the mock generate concrete's
///   declaration requires `EncodingIdentifier::Mock` and would refuse the
///   ABI encoder before `emit`, over every member of `PAIRING_CONCRETES`,
///   and every other selection the mock default.
/// Act:     `create_pairing` through the crate root, once per selection and
///   per pairing.
/// Assert:  the output under the declared selection holds `Ok` and the
///   output under the mixed selection matches `Ok(CreatePairingSuccessReturn
///   { output: Err(PairingConsumerErrorReturn::Encoding(
///   EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier))) })`.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route `create_pairing`, `consume_pairing`,
///   `create_hash_to_scalar`, `create_encoding`, `consume_encoding`, and
///   `create_chain_forms`; Mocked the pairing family's concrete the case
///   selects.
#[test]
fn the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let mock_identifier = build_create_chain_forms_params(Default::default()).identifier;

    for forms in DECLARED_CHAIN_FORMS_SELECTIONS {
        for pairing_concrete in PAIRING_CONCRETES {
            for identifier in [forms.identifier, mock_identifier] {
                let scope = build_create_kem_params(Default::default()).scope;
                let admitted_identities = admitted_identities_of(scope, 2);
                let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                    random: &*random_source.adapter,
                    statements: &admitted_statements,
                    identities: &admitted_identities,
                    hash_to_scalar: AdmittedHashToScalar {
                        concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                        identifier: build_create_hash_to_scalar_params(Default::default())
                            .identifier,
                    },
                    encoding: AdmittedEncoding {
                        concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                        identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                    },
                    chain_forms: AdmittedChainForms {
                        concrete: forms.concrete,
                        identifier,
                    },
                    kem: AdmittedKem {
                        concrete: build_create_kem_params(Default::default()).concrete,
                        identifier: build_create_kem_params(Default::default()).identifier,
                        scope,
                    },
                    key_agreement: AdmittedKeyAgreement {
                        concrete: build_create_key_agreement_params(Default::default()).concrete,
                        identifier: build_create_key_agreement_params(Default::default())
                            .identifier,
                        algebra: build_create_key_agreement_params(Default::default()).algebra,
                    },
                    delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                    generate: build_generate_concrete(
                        GENERATE_CONCRETES[0],
                        GenerateConcreteOverrides,
                    ),
                    kinds: build_generate_kind_configurations(
                        GENERATE_CONCRETES[0],
                        GenerateKindConfigurationsOverrides,
                    ),
                    verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                    statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                    iterations: NonZeroU32::MIN,
                });

                // Act
                let result = create_pairing(
                    &build_create_pairing_deps(consumer, Default::default()),
                    build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    }),
                    CreatePairingPayload,
                );

                // Assert
                if identifier == forms.identifier {
                    assert!(matches!(
                        result,
                        Ok(CreatePairingSuccessReturn {
                            output: Ok(DeliveryProofConsumerSuccessReturn { .. })
                        })
                    ));
                } else {
                    assert!(matches!(
                        result,
                        Ok(CreatePairingSuccessReturn {
                            output: Err(PairingConsumerErrorReturn::Encoding(
                                EncodingConsumerErrorReturn::CreateChainForms(
                                    CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier
                                )
                            ))
                        })
                    ));
                }
            }
        }
    }
    // the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
}

/// Contract: carried entry
///   `an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs`
///   of `harness-crypto/run/encoding_consumer`, as an outside caller
///   observes it through this consumer — the refusal arrives as
///   `PairingConsumerErrorReturn::Encoding(EncodingConsumerErrorReturn::
///   CreateChainForms(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier))`
///   for the held encoding selection and held chain forms selection of the
///   entry's arrangement, no file written.
/// Arrange: the mock encoding selection with a member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, and the Ethereum ABI selection with
///   the mock forms selection, with the mock generate selection, over every
///   member of `PAIRING_CONCRETES`, and every other selection the mock
///   default.
/// Act:     `create_pairing` through the crate root, once per pairing.
/// Assert:  each output matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::Encoding(
///   EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier))) })` and
///   no file is written.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route `create_pairing`, `consume_pairing`,
///   `create_hash_to_scalar`, `create_encoding`, `consume_encoding`, and
///   `create_chain_forms`; Mocked the pairing family's concrete the case
///   selects.
#[test]
fn an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs()
 {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let declared_forms = DECLARED_CHAIN_FORMS_SELECTIONS[0];
    let mock_forms = build_create_chain_forms_params(Default::default());
    let cases: Vec<(CreateEncodingParams, CreateChainFormsParams)> = vec![
        (
            build_create_encoding_params(Default::default()),
            declared_forms,
        ),
        (encoding_for(declared_forms), mock_forms),
    ];

    for (encoding, forms) in &cases {
        for pairing_concrete in PAIRING_CONCRETES {
            let scope = build_create_kem_params(Default::default()).scope;
            let admitted_identities = admitted_identities_of(scope, 2);
            let generate_params = build_create_generate_params(Default::default());
            let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                random: &*random_source.adapter,
                statements: &admitted_statements,
                identities: &admitted_identities,
                hash_to_scalar: AdmittedHashToScalar {
                    concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                    identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
                },
                encoding: AdmittedEncoding {
                    concrete: encoding.concrete,
                    identifier: encoding.identifier,
                },
                chain_forms: AdmittedChainForms {
                    concrete: forms.concrete,
                    identifier: forms.identifier,
                },
                kem: AdmittedKem {
                    concrete: build_create_kem_params(Default::default()).concrete,
                    identifier: build_create_kem_params(Default::default()).identifier,
                    scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: build_create_key_agreement_params(Default::default()).concrete,
                    identifier: build_create_key_agreement_params(Default::default()).identifier,
                    algebra: build_create_key_agreement_params(Default::default()).algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });

            // Act
            let result = create_pairing(
                &build_create_pairing_deps(consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            assert!(matches!(
                result,
                Ok(CreatePairingSuccessReturn {
                    output: Err(PairingConsumerErrorReturn::Encoding(
                        EncodingConsumerErrorReturn::CreateChainForms(
                            CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier
                        )
                    ))
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
///   of `harness-crypto/run/encoding_consumer`, as an outside caller
///   observes it through this consumer — the statements and identities this
///   consumer holds reach the encoding consumer it constructs, and through
///   it the files, as records.
/// Arrange: for each count of one and three, an `AdmittedIdentities` whose
///   `mapping` holds that many identities of the variant the scope names,
///   `AdmittedStatements` whose mint purpose is `MintPurpose::Replacement`
///   and whose transfer purpose is `TransferPurpose::Grant`, a real generate
///   concrete, each scope of `DECLARED_KEM_SELECTIONS`, the Ethereum ABI
///   encoding selection and the declared forms selection, and every member
///   of `PAIRING_CONCRETES`, the random source
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_pairing` through the crate root, once per case, then
///   `create_encoding` once for the decoder probe.
/// Assert:  the file tagged `Some(VectorKind::IdentityMapping)` holds that
///   many lines and each line of the file tagged `Some(VectorKind::Challenge)`,
///   its hex after `0x` read back into bytes, decodes through the decoder
///   of the held encoding selection to `ChallengeVector`s whose `purpose` is
///   `4` in as many lines as it is `3`, that number positive, and neither
///   `1` nor `2` in any line.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route the disposition states, then
///   `create_encoding` over a consumer of the test's own for the decoder.
/// Mocked:  the verifier family's mock concrete.
#[test]
fn the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold()
 {
    // Arrange
    struct ChallengeConsumer<'a> {
        lines: &'a [Vec<u8>],
        description: &'a ChallengeVectorDescription,
    }

    impl IEncodingConsumer for ChallengeConsumer<'_> {
        type Output = Vec<u16>;

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
                        .expect("each line decodes")
                        .described
                        .purpose
                })
                .collect()
        }
    }

    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(AdmittedStatementsOverrides {
        mint_purpose: Some(MintPurpose::Replacement),
        transfer_purpose: Some(TransferPurpose::Grant),
        ..Default::default()
    });
    let declared_forms = DECLARED_CHAIN_FORMS_SELECTIONS[0];

    for kem_selection in DECLARED_KEM_SELECTIONS {
        for count in [1usize, 3usize] {
            for pairing_concrete in PAIRING_CONCRETES {
                for kind in GENERATE_CONCRETES {
                    let admitted_identities = admitted_identities_of(kem_selection.scope, count);
                    let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                        random: &*random_source.adapter,
                        statements: &admitted_statements,
                        identities: &admitted_identities,
                        hash_to_scalar: AdmittedHashToScalar {
                            concrete: build_create_hash_to_scalar_params(Default::default())
                                .concrete,
                            identifier: build_create_hash_to_scalar_params(Default::default())
                                .identifier,
                        },
                        encoding: AdmittedEncoding {
                            concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                            identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                        },
                        chain_forms: AdmittedChainForms {
                            concrete: declared_forms.concrete,
                            identifier: declared_forms.identifier,
                        },
                        kem: AdmittedKem {
                            concrete: kem_selection.concrete,
                            identifier: kem_selection.identifier,
                            scope: kem_selection.scope,
                        },
                        key_agreement: AdmittedKeyAgreement {
                            concrete: DECLARED_KEY_AGREEMENT_SELECTIONS[0].concrete,
                            identifier: DECLARED_KEY_AGREEMENT_SELECTIONS[0].identifier,
                            algebra: DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
                        },
                        delivery_proof: DELIVERY_PROOF_CONCRETES[0],
                        generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                        kinds: build_generate_kind_configurations(
                            *kind,
                            GenerateKindConfigurationsOverrides,
                        ),
                        verifier: VerifierConcrete::Mock(
                            MockIVerifierAdapterFailureMode::NoFailure,
                        ),
                        statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                        iterations: NonZeroU32::MIN,
                    });

                    // Act
                    let result = create_pairing(
                        &build_create_pairing_deps(consumer, Default::default()),
                        build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        }),
                        CreatePairingPayload,
                    );

                    // Assert
                    let success = result
                        .expect("the factory admits the concrete and runs the consumer")
                        .output
                        .expect("the consumer calls the factories and the chain beneath them");
                    let mapping = success
                        .files
                        .iter()
                        .find(|file| file.kind == Some(VectorKind::IdentityMapping))
                        .expect("the emit returns the identity-mapping file");
                    let text =
                        std::fs::read_to_string(&mapping.path).expect("the emitted file reads");
                    assert_eq!(text.lines().count(), count);
                    let challenge = success
                        .files
                        .iter()
                        .find(|file| file.kind == Some(VectorKind::Challenge))
                        .expect("the emit returns the challenge file");
                    let text =
                        std::fs::read_to_string(&challenge.path).expect("the emitted file reads");
                    let Ok(description) = ChallengeVectorDescription::try_new(
                        ChallengeVectorDescriptionConstructorParams,
                    );
                    let lines: Vec<Vec<u8>> = text
                        .lines()
                        .map(|line| {
                            let hex = line
                                .strip_prefix("0x")
                                .expect("a vector line carries the 0x prefix");
                            (0..hex.len())
                                .step_by(2)
                                .map(|index| {
                                    u8::from_str_radix(&hex[index..index + 2], 16)
                                        .expect("every written line is hex")
                                })
                                .collect()
                        })
                        .collect();
                    let probe = ChallengeConsumer {
                        lines: &lines,
                        description: &description,
                    };
                    let purposes = create_encoding(
                        &build_create_encoding_deps(probe),
                        CreateEncodingParams {
                            concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                            identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                        },
                        CreateEncodingPayload,
                    )
                    .expect("the factory admits the selection")
                    .output;
                    let replacements = purposes.iter().filter(|purpose| **purpose == 4).count();
                    let grants = purposes.iter().filter(|purpose| **purpose == 3).count();
                    assert_eq!(replacements, grants);
                    assert!(replacements > 0);
                    assert!(
                        purposes
                            .iter()
                            .all(|purpose| *purpose != 1 && *purpose != 2)
                    );
                }
            }
        }
    }
    // the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
}

/// Contract: carried entry
///   `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`
///   of `harness-crypto/run/encoding_consumer`, as an outside caller
///   observes it through this consumer — `consume_pairing` passes the held
///   per-kind configuration to the encoding consumer it constructs, so the
///   file tagged `Some(VectorKind::Possession)` holds the configured count
///   of lines.
/// Arrange: the generate selection and the per-kind configuration
///   `build_generate_kind_configurations` returns reversed, the possession
///   entry's count overridden through `build_vector_kind_configuration` and
///   `build_vector_kind_quantities` to a `build_vector_count` value other
///   than the default and than every other entry's count, the Ethereum ABI
///   encoding selection and the declared forms selection, and
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`,
///   over every member of `PAIRING_CONCRETES`.
/// Act:     `create_pairing` through the crate root, once per case.
/// Assert:  the file whose `kind` is `Some(VectorKind::Possession)` is read
///   by `std::fs::read_to_string` and its line count equals the configured
///   count.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route the disposition states.
/// Mocked:  the verifier family's mock concrete.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let declared_forms = DECLARED_CHAIN_FORMS_SELECTIONS[0];

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
            let scope = DECLARED_KEM_SELECTIONS[0].scope;
            let admitted_identities = admitted_identities_of(scope, 2);
            let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                random: &*random_source.adapter,
                statements: &admitted_statements,
                identities: &admitted_identities,
                hash_to_scalar: AdmittedHashToScalar {
                    concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                    identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
                },
                encoding: AdmittedEncoding {
                    concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                    identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                },
                chain_forms: AdmittedChainForms {
                    concrete: declared_forms.concrete,
                    identifier: declared_forms.identifier,
                },
                kem: AdmittedKem {
                    concrete: DECLARED_KEM_SELECTIONS[0].concrete,
                    identifier: DECLARED_KEM_SELECTIONS[0].identifier,
                    scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: DECLARED_KEY_AGREEMENT_SELECTIONS[0].concrete,
                    identifier: DECLARED_KEY_AGREEMENT_SELECTIONS[0].identifier,
                    algebra: DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
                },
                delivery_proof: DELIVERY_PROOF_CONCRETES[0],
                generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                kinds: kinds.clone(),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });

            // Act
            let result = create_pairing(
                &build_create_pairing_deps(consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            let success = result
                .expect("the factory admits the concrete and runs the consumer")
                .output
                .expect("the consumer calls the factories and the chain beneath them");
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
///   of `harness-crypto/run/encoding_consumer`, as an outside caller
///   observes it through this consumer — the encoder the generate factory is
///   handed is the encoder the encoding factory constructed for the held
///   selection, and the refusal returns as `PairingConsumerErrorReturn::
///   Encoding(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::CreateGenerate(
///   CreateGenerateErrorReturn::UnsupportedEncodingIdentifier))))))`.
/// Arrange: every member of `GENERATE_CONCRETES` over the mock encoding
///   selection and the mock forms selection, and the mock generate selection
///   over every member of `DECLARED_ENCODING_SELECTIONS` with the declared
///   forms selection, as the entry of the same name of
///   `harness-crypto/generate/factory` arranges the encoder, over every
///   member of `PAIRING_CONCRETES`.
/// Act:     `create_pairing` through the crate root, once per case.
/// Assert:  the output matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::Encoding(
///   EncodingConsumerErrorReturn::ChainForms(ChainFormsConsumerErrorReturn::
///   Kem(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::CreateGenerate(
///   CreateGenerateErrorReturn::UnsupportedEncodingIdentifier))))))) })`,
///   the pattern naming every field.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route the disposition states up to
///   `create_generate`.
/// Mocked:  the encoding family's mock concrete as the encoder for the real
///   generate concretes, and the generate family's mock concrete as the
///   selection for the real encoders, selected through their factories.
#[test]
fn create_generate_refuses_an_encoder_declaring_another_encoding_identifier() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let declared_forms = DECLARED_CHAIN_FORMS_SELECTIONS[0];
    let mock_forms = build_create_chain_forms_params(Default::default());

    for kind in GENERATE_CONCRETES {
        for pairing_concrete in PAIRING_CONCRETES {
            let scope = build_create_kem_params(Default::default()).scope;
            let admitted_identities = admitted_identities_of(scope, 2);
            let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                random: &*random_source.adapter,
                statements: &admitted_statements,
                identities: &admitted_identities,
                hash_to_scalar: AdmittedHashToScalar {
                    concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                    identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
                },
                encoding: AdmittedEncoding {
                    concrete: build_create_encoding_params(Default::default()).concrete,
                    identifier: build_create_encoding_params(Default::default()).identifier,
                },
                chain_forms: AdmittedChainForms {
                    concrete: mock_forms.concrete,
                    identifier: mock_forms.identifier,
                },
                kem: AdmittedKem {
                    concrete: build_create_kem_params(Default::default()).concrete,
                    identifier: build_create_kem_params(Default::default()).identifier,
                    scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: build_create_key_agreement_params(Default::default()).concrete,
                    identifier: build_create_key_agreement_params(Default::default()).identifier,
                    algebra: build_create_key_agreement_params(Default::default()).algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: build_generate_concrete(*kind, GenerateConcreteOverrides),
                kinds: build_generate_kind_configurations(
                    *kind,
                    GenerateKindConfigurationsOverrides,
                ),
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });

            // Act
            let result = create_pairing(
                &build_create_pairing_deps(consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            assert!(matches!(
                result,
                Ok(CreatePairingSuccessReturn {
                    output: Err(PairingConsumerErrorReturn::Encoding(
                        EncodingConsumerErrorReturn::ChainForms(
                            ChainFormsConsumerErrorReturn::Kem(
                                KemConsumerErrorReturn::KeyAgreement(
                                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                                        DeliveryProofConsumerErrorReturn::CreateGenerate(
                                            CreateGenerateErrorReturn::UnsupportedEncodingIdentifier
                                        )
                                    )
                                )
                            )
                        )
                    ))
                })
            ));
        }
    }

    for encoding in DECLARED_ENCODING_SELECTIONS {
        for pairing_concrete in PAIRING_CONCRETES {
            let scope = build_create_kem_params(Default::default()).scope;
            let admitted_identities = admitted_identities_of(scope, 2);
            let generate_params = build_create_generate_params(Default::default());
            let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                random: &*random_source.adapter,
                statements: &admitted_statements,
                identities: &admitted_identities,
                hash_to_scalar: AdmittedHashToScalar {
                    concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                    identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
                },
                encoding: AdmittedEncoding {
                    concrete: encoding.concrete,
                    identifier: encoding.identifier,
                },
                chain_forms: AdmittedChainForms {
                    concrete: declared_forms.concrete,
                    identifier: declared_forms.identifier,
                },
                kem: AdmittedKem {
                    concrete: build_create_kem_params(Default::default()).concrete,
                    identifier: build_create_kem_params(Default::default()).identifier,
                    scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: build_create_key_agreement_params(Default::default()).concrete,
                    identifier: build_create_key_agreement_params(Default::default()).identifier,
                    algebra: build_create_key_agreement_params(Default::default()).algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });

            // Act
            let result = create_pairing(
                &build_create_pairing_deps(consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            assert!(matches!(
                result,
                Ok(CreatePairingSuccessReturn {
                    output: Err(PairingConsumerErrorReturn::Encoding(
                        EncodingConsumerErrorReturn::ChainForms(
                            ChainFormsConsumerErrorReturn::Kem(
                                KemConsumerErrorReturn::KeyAgreement(
                                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                                        DeliveryProofConsumerErrorReturn::CreateGenerate(
                                            CreateGenerateErrorReturn::UnsupportedEncodingIdentifier
                                        )
                                    )
                                )
                            )
                        )
                    ))
                })
            ));
        }
    }
    // create_generate_refuses_an_encoder_declaring_another_encoding_identifier
}

/// Contract: carried entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`
///   of `harness-crypto/run/encoding_consumer`, as an outside caller observes
///   it through this consumer — the hash-to-scalar adapter the downstream
///   mint step uses is the adapter this consumer obtained through
///   `create_hash_to_scalar` under the held selection, so the refusal
///   reaches the caller of `create_pairing` inside
///   `PairingConsumerErrorReturn::Encoding`, unconverted.
/// Arrange: the held `AdmittedHashToScalar` the family's mock selection
///   under `MockIHashToScalarAdapterFailureMode::SamplingRefused`, each
///   scope of `DECLARED_KEM_SELECTIONS` over every member of
///   `PAIRING_CONCRETES`, the mock encoding, forms, key agreement, and
///   delivery proof selections, the mock generate selection, and the
///   operating-system source.
/// Act:     `create_pairing` through the crate root, once per case.
/// Assert:  the output under each scope matches the scope's
///   `Ok(CreatePairingSuccessReturn { output: Err(
///   PairingConsumerErrorReturn::Encoding(…)) })` naming every field down to
///   the sampling bound refusal, with `actual` `0`.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route the disposition states.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let refused_hash_to_scalar = AdmittedHashToScalar {
        concrete: HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused),
        identifier: HashToScalarIdentifier::Mock,
    };

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let admitted_identities = admitted_identities_of(kem_selection.scope, 2);
            let generate_params = build_create_generate_params(Default::default());
            let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                random: &*random_source.adapter,
                statements: &admitted_statements,
                identities: &admitted_identities,
                hash_to_scalar: refused_hash_to_scalar,
                encoding: AdmittedEncoding {
                    concrete: build_create_encoding_params(Default::default()).concrete,
                    identifier: build_create_encoding_params(Default::default()).identifier,
                },
                chain_forms: AdmittedChainForms {
                    concrete: build_create_chain_forms_params(Default::default()).concrete,
                    identifier: build_create_chain_forms_params(Default::default()).identifier,
                },
                kem: AdmittedKem {
                    concrete: kem_selection.concrete,
                    identifier: kem_selection.identifier,
                    scope: kem_selection.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: build_create_key_agreement_params(Default::default()).concrete,
                    identifier: build_create_key_agreement_params(Default::default()).identifier,
                    algebra: build_create_key_agreement_params(Default::default()).algebra,
                },
                delivery_proof: build_create_delivery_proof_params(Default::default()).concrete,
                generate: generate_params.concrete,
                kinds: generate_params.kinds,
                verifier: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                statement_version: DELIVERY_STATEMENT_VERSION_ONE,
                iterations: NonZeroU32::MIN,
            });

            // Act
            let result = create_pairing(
                &build_create_pairing_deps(consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            match kem_selection.scope {
                IdentityScope::Entitlement => {
                    assert!(matches!(
                        &result,
                        Ok(CreatePairingSuccessReturn {
                            output: Err(PairingConsumerErrorReturn::Encoding(
                                EncodingConsumerErrorReturn::ChainForms(
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
                                )
                            ))
                        }) if *expected > 0
                    ));
                }
                IdentityScope::Asset => {
                    assert!(matches!(
                        &result,
                        Ok(CreatePairingSuccessReturn {
                            output: Err(PairingConsumerErrorReturn::Encoding(
                                EncodingConsumerErrorReturn::ChainForms(
                                    ChainFormsConsumerErrorReturn::Kem(
                                        KemConsumerErrorReturn::KeyAgreement(
                                            KeyAgreementConsumerErrorReturn::DeliveryProof(
                                                DeliveryProofConsumerErrorReturn::Measure(
                                                    MeasureErrorReturn::Mint(
                                                        MintErrorReturn::Setup(
                                                            SetupErrorReturn::HashToScalar(
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
                                )
                            ))
                        }) if *expected > 0
                    ));
                }
            }
        }
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: carried entry `a_nonce_draw_refusal_reaches_the_caller_whole` of
///   `harness-crypto/run/encoding_consumer`, as an outside caller observes it
///   through this consumer — the nonce draw's refusal reaches the caller of
///   `create_pairing` inside `PairingConsumerErrorReturn::Encoding`,
///   unconverted.
/// Arrange: the delivery proof the family's mock concrete under
///   `MockIDeliveryProofAdapterFailureMode::NonceDrawRefused`, selected
///   through the consumer's held selection, each scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, the
///   mock encoding, forms, and key agreement selections, the mock generate
///   selection, and the operating-system source.
/// Act:     `create_pairing` through the crate root, once per case.
/// Assert:  the output matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::Encoding(
///   EncodingConsumerErrorReturn::ChainForms(ChainFormsConsumerErrorReturn::
///   Kem(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::Mint(
///   MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))))))))) })`,
///   the pattern naming every field.
/// Boundary: the public calls `PairingConsumer::try_new` and `create_pairing`
///   through the crate root and the route the disposition states.
/// Mocked:  the delivery-proof family's mock concrete, selected by
///   configuration.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the operating-system source constructs");
    let admitted_statements = build_admitted_statements(Default::default());

    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let admitted_identities = admitted_identities_of(kem_selection.scope, 2);
            let generate_params = build_create_generate_params(Default::default());
            let Ok(consumer) = PairingConsumer::try_new(PairingConsumerConstructorParams {
                random: &*random_source.adapter,
                statements: &admitted_statements,
                identities: &admitted_identities,
                hash_to_scalar: AdmittedHashToScalar {
                    concrete: build_create_hash_to_scalar_params(Default::default()).concrete,
                    identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
                },
                encoding: AdmittedEncoding {
                    concrete: build_create_encoding_params(Default::default()).concrete,
                    identifier: build_create_encoding_params(Default::default()).identifier,
                },
                chain_forms: AdmittedChainForms {
                    concrete: build_create_chain_forms_params(Default::default()).concrete,
                    identifier: build_create_chain_forms_params(Default::default()).identifier,
                },
                kem: AdmittedKem {
                    concrete: kem_selection.concrete,
                    identifier: kem_selection.identifier,
                    scope: kem_selection.scope,
                },
                key_agreement: AdmittedKeyAgreement {
                    concrete: build_create_key_agreement_params(Default::default()).concrete,
                    identifier: build_create_key_agreement_params(Default::default()).identifier,
                    algebra: build_create_key_agreement_params(Default::default()).algebra,
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

            // Act
            let result = create_pairing(
                &build_create_pairing_deps(consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            assert!(matches!(
                result,
                Ok(CreatePairingSuccessReturn {
                    output: Err(PairingConsumerErrorReturn::Encoding(
                        EncodingConsumerErrorReturn::ChainForms(
                            ChainFormsConsumerErrorReturn::Kem(
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
                            )
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
//   the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole
//     → the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole
//   the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
//     → the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs
//   the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs
//     → the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs
//   the_hash_to_scalar_refusal_precedes_the_encoding_refusal
//     → the_hash_to_scalar_refusal_precedes_the_encoding_refusal
//   the_files_are_encoded_under_the_encoding_selection_the_consumer_holds
//     → the_files_are_encoded_under_the_encoding_selection_the_consumer_holds
//   the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
//     → the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs
//   an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs
//     → an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs
//   the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
//     → the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold
//   a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
//     → a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
//   create_generate_refuses_an_encoder_declaring_another_encoding_identifier
//     → create_generate_refuses_an_encoder_declaring_another_encoding_identifier
//   a_hash_to_scalar_refusal_reaches_the_caller_whole
//     → a_hash_to_scalar_refusal_reaches_the_caller_whole
//   a_nonce_draw_refusal_reaches_the_caller_whole
//     → a_nonce_draw_refusal_reaches_the_caller_whole
