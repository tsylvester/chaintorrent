#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use chain::{
    ChainFormsConcrete, CreateChainFormsParams, DECLARED_CHAIN_FORMS_SELECTIONS,
    build_create_chain_forms_params,
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
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{CreateKemParams, DECLARED_KEM_SELECTIONS, IdentityScope, build_create_kem_params};
use pairing::{
    CreatePairingParamsOverrides, CreatePairingPayload, CreatePairingSuccessReturn,
    PAIRING_CONCRETES, PairingConcrete, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use proof::{DELIVERY_PROOF_CONCRETES, DeliveryProofConcrete, build_create_delivery_proof_params};
use random::{
    CreateRandomSourceParamsOverrides, RandomSourceKind, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};

use super::interface::PairingConsumerErrorReturn;
use super::mock::{PairingConsumerConstructorParamsOverrides, build_pairing_consumer};
use crate::config::provides::{
    AdmittedChainForms, AdmittedEncoding, AdmittedHashToScalar, AdmittedIdentities,
    AdmittedIdentitiesOverrides, AdmittedIdentity, AdmittedKem, AdmittedKeyAgreement,
    build_admitted_identities, build_admitted_statements,
};
use crate::generate::provides::{
    ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams, GENERATE_CONCRETES,
    GenerateConcrete, GenerateConcreteOverrides, GenerateKindConfigurationsOverrides, VectorKind,
    VectorKindConfiguration, build_create_generate_params, build_emit_success_return,
    build_generate_concrete, build_generate_kind_configurations,
};
use crate::run::provides::DeliveryProofConsumerSuccessReturn;
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, VerifierConcrete, VerifyCost,
};

fn admitted_identities_of(scope: IdentityScope) -> AdmittedIdentities {
    let (proof, mapping) = match scope {
        IdentityScope::Entitlement => (
            AdmittedIdentity::Entitlement {
                canonical: b"example-package@1.0.0".to_vec(),
            },
            vec![
                AdmittedIdentity::Entitlement {
                    canonical: b"example-package@1.0.0".to_vec(),
                },
                AdmittedIdentity::Entitlement {
                    canonical: b"example-package@2.0.0".to_vec(),
                },
            ],
        ),
        IdentityScope::Asset => (
            AdmittedIdentity::Asset {
                root: build_root(Default::default()),
            },
            vec![
                AdmittedIdentity::Asset {
                    root: build_root(Default::default()),
                },
                AdmittedIdentity::Asset {
                    root: build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x11; 32]),
                    }),
                },
            ],
        ),
    };
    build_admitted_identities(AdmittedIdentitiesOverrides {
        proof: Some(proof),
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
///   the absorbed `harness-crypto/run/encoding_consumer` entries
///   `the_files_and_the_measurement_reach_the_caller_of_create_encoding_whole`
///   and `the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with`,
///   the absorbed `hash-to-scalar/factory` entry
///   `every_concrete_is_constructed_under_the_identifier_it_declares`, and the
///   absorbed `pairing/factory` entry
///   `the_consumer_receives_the_concrete_the_params_name` — `create_pairing`
///   over each member of `PAIRING_CONCRETES` with params naming only the
///   concrete, and over the mock selection → `Ok` whose output is
///   `Ok(DeliveryProofConsumerSuccessReturn)` whose `files` name no path
///   twice, include for a real generate concrete the four tagged files each
///   of whose paths exists, and whose `measurement.iterations` equals the
///   held count and whose costs each equal the literal cost the verifier
///   family's mock concrete returns.
/// Arrange: every member of `GENERATE_CONCRETES` and the mock generate
///   selection, each member of `DELIVERY_PROOF_CONCRETES` and the mock
///   selection, each key agreement selection, each member of
///   `DECLARED_KEM_SELECTIONS` and the mock KEM selection, each
///   hash-to-scalar selection the entry names, and the pairings the entry
///   names, with `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::
///   NoFailure)`, the encoding selection paired with the forms selection
///   that requires its identifier, an iteration count of three, and the
///   expected cost the literal gas `100_000` and fee `L1DataFee::Charged {
///   wei: 1_000 }`; a real forms selection runs only under a real generate
///   concrete and the mock forms only under the mock, since the mock
///   generate concrete's declaration requires `EncodingIdentifier::Mock`.
/// Act:     `create_pairing`, once per case.
/// Assert:  the output is `Ok` holding `Ok(DeliveryProofConsumerSuccessReturn)`,
///   its `files` name no path twice, a real concrete's include the four
///   tagged files and each of its paths exists, `measurement.iterations`
///   equals the held count, and the two costs equal the expected cost.
/// Boundary: `create_pairing`, the selected pairing concrete's `try_new`,
///   `consume_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `consume_encoding`, `create_chain_forms`, `consume_chain_forms`,
///   `admit_contexts`, `create_kem`, `create_key_agreement`,
///   `create_delivery_proof`, `create_generate`, `emit`, `create_verifier`,
///   `consume_verifier`, and `measure`.
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
    let hash_to_scalar_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .copied()
        .chain(core::iter::once(build_create_hash_to_scalar_params(
            Default::default(),
        )))
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
                                let admitted_identities = admitted_identities_of(kem.scope);
                                let consumer = build_pairing_consumer(
                                    &*random_source.adapter,
                                    &admitted_statements,
                                    &admitted_identities,
                                    PairingConsumerConstructorParamsOverrides {
                                        hash_to_scalar: Some(AdmittedHashToScalar {
                                            concrete: hash_to_scalar.concrete,
                                            identifier: hash_to_scalar.identifier,
                                        }),
                                        encoding: Some(AdmittedEncoding {
                                            concrete: encoding.concrete,
                                            identifier: encoding.identifier,
                                        }),
                                        chain_forms: Some(AdmittedChainForms {
                                            concrete: forms.concrete,
                                            identifier: forms.identifier,
                                        }),
                                        kem: Some(AdmittedKem {
                                            concrete: kem.concrete,
                                            identifier: kem.identifier,
                                            scope: kem.scope,
                                        }),
                                        key_agreement: Some(AdmittedKeyAgreement {
                                            concrete: key_agreement.concrete,
                                            identifier: key_agreement.identifier,
                                            algebra: key_agreement.algebra,
                                        }),
                                        delivery_proof: Some(*delivery_proof),
                                        generate: Some(generate.clone()),
                                        kinds: Some(kinds.clone()),
                                        verifier: Some(VerifierConcrete::Mock(
                                            MockIVerifierAdapterFailureMode::NoFailure,
                                        )),
                                        iterations: NonZeroU32::new(3),
                                        ..Default::default()
                                    },
                                );

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
///   `the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`
///   and the absorbed `hash-to-scalar/factory` entry
///   `every_concrete_is_refused_under_an_identifier_it_does_not_declare` —
///   `create_pairing` and a consumer constructed with an
///   `AdmittedHashToScalar` built from a member of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, and the same selection with the
///   `identifier` the mock selection declares → `Ok` whose output is `Ok`
///   for the declared member and `Err(PairingConsumerErrorReturn::
///   CreateHashToScalar(CreateHashToScalarErrorReturn::
///   UnsupportedHashToScalarIdentifier))` for the mixed selection.
/// Arrange: the two selections over every member of `PAIRING_CONCRETES`, the
///   mock generate selection and the verifier family's mock concrete under
///   `NoFailure`, and every other selection the mock default.
/// Act:     `create_pairing`, once per selection and per pairing.
/// Assert:  the output under the declared selection holds
///   `Ok(DeliveryProofConsumerSuccessReturn { .. })`, and the output under
///   the mixed selection matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateHashToScalar(
///   CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)) })`.
/// Boundary: `create_pairing`, `consume_pairing`, and `create_hash_to_scalar`;
///   Mocked the pairing family's concrete the case selects, whose scalar
///   type the factory is called for.
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
                let admitted_identities = admitted_identities_of(scope);
                let consumer = build_pairing_consumer(
                    &*random_source.adapter,
                    &admitted_statements,
                    &admitted_identities,
                    PairingConsumerConstructorParamsOverrides {
                        hash_to_scalar: Some(AdmittedHashToScalar {
                            concrete: selection.concrete,
                            identifier,
                        }),
                        ..Default::default()
                    },
                );

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
///   `the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs`
///   and the absorbed `encoding/factory` entry
///   `every concrete is refused under an identifier it does not declare` —
///   `create_pairing` and a consumer constructed with an `AdmittedEncoding`
///   built from a member of `DECLARED_ENCODING_SELECTIONS` paired with a
///   member of `DECLARED_CHAIN_FORMS_SELECTIONS` that requires its
///   identifier, and the same selection with the `identifier` the mock
///   selection declares → `Ok` whose output is `Ok` for the declared member
///   and `Err(PairingConsumerErrorReturn::CreateEncoding(
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
/// Act:     `create_pairing`, once per selection and per pairing.
/// Assert:  the output under the declared selection holds
///   `Ok(DeliveryProofConsumerSuccessReturn { .. })`, and the output under
///   the mixed selection matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateEncoding(
///   CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)) })`.
/// Boundary: `create_pairing`, `consume_pairing`, `create_hash_to_scalar`,
///   and `create_encoding`; Mocked the pairing family's concrete the case
///   selects.
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
                let admitted_identities = admitted_identities_of(scope);
                let consumer = build_pairing_consumer(
                    &*random_source.adapter,
                    &admitted_statements,
                    &admitted_identities,
                    PairingConsumerConstructorParamsOverrides {
                        encoding: Some(AdmittedEncoding {
                            concrete: selection.concrete,
                            identifier,
                        }),
                        chain_forms: Some(AdmittedChainForms {
                            concrete: declared_forms.concrete,
                            identifier: declared_forms.identifier,
                        }),
                        generate: Some(build_generate_concrete(
                            GENERATE_CONCRETES[0],
                            GenerateConcreteOverrides,
                        )),
                        kinds: Some(build_generate_kind_configurations(
                            GENERATE_CONCRETES[0],
                            GenerateKindConfigurationsOverrides,
                        )),
                        ..Default::default()
                    },
                );

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

/// Contract: own entry `the_hash_to_scalar_refusal_precedes_the_encoding_refusal`
///   — `create_pairing` and a consumer constructed with a mixed
///   `AdmittedHashToScalar` and a mixed `AdmittedEncoding` together → `Ok`
///   whose output is `Err(PairingConsumerErrorReturn::CreateHashToScalar(
///   CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier))` for
///   every pairing, so the hash-to-scalar adapter is obtained before the
///   encoding family is called.
/// Arrange: the mixed `AdmittedHashToScalar` of the block above together
///   with the mixed `AdmittedEncoding` of the block above, the mock generate
///   selection, and every member of `PAIRING_CONCRETES`.
/// Act:     `create_pairing`, once per pairing.
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
        let admitted_identities = admitted_identities_of(scope);
        let consumer = build_pairing_consumer(
            &*random_source.adapter,
            &admitted_statements,
            &admitted_identities,
            PairingConsumerConstructorParamsOverrides {
                hash_to_scalar: Some(mixed_hash_to_scalar),
                encoding: Some(mixed_encoding),
                ..Default::default()
            },
        );

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
///   the absorbed `harness-crypto/run/encoding_consumer` entry
///   `the_encoder_the_factory_hands_the_consumer_is_the_encoder_the_files_are_encoded_with`,
///   and the absorbed `encoding/factory` entry `every concrete is handed to
///   the consumer under the identifier it declares` — `create_pairing` over
///   each pairing named above, with a real generate concrete, and the files
///   tagged `Some(VectorKind::Challenge)` decoded through the decoder of the
///   held encoding selection → the lines of that file decode to
///   `ChallengeVector`s under the decoder of the held selection and are
///   refused by the decoder of the other encoding selection.
/// Arrange: each pairing the entry names, with a member of
///   `GENERATE_CONCRETES` under the Ethereum ABI selection paired with the
///   forms selection that requires its identifier, the mock KEM, key
///   agreement, and delivery proof selections, and
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`.
/// Act:     `create_pairing`, once per pairing and per encoding selection,
///   then `create_encoding` once per decoder probe.
/// Assert:  each line of the file tagged `Some(VectorKind::Challenge)`, its
///   hex after `0x` read back into bytes, decodes through the decoder of the
///   held encoding selection over `DecodeParams { description:
///   &description }` with `description` the `ChallengeVectorDescription`
///   built by its `try_new` unpacked irrefutably, to `ChallengeVector`s, and
///   the same bytes decoded through the decoder of the other encoding
///   selection are refused.
/// Boundary and Mocked: as the first block states.
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
    let cases: Vec<(
        CreateEncodingParams,
        CreateChainFormsParams,
        GenerateConcrete,
        Vec<VectorKindConfiguration>,
        CreateEncodingParams,
    )> = vec![(
        encoding_for(declared_forms),
        declared_forms,
        build_generate_concrete(GENERATE_CONCRETES[0], GenerateConcreteOverrides),
        build_generate_kind_configurations(
            GENERATE_CONCRETES[0],
            GenerateKindConfigurationsOverrides,
        ),
        build_create_encoding_params(Default::default()),
    )];

    for pairing_concrete in PAIRING_CONCRETES {
        for (encoding, forms, generate, kinds, other_encoding) in &cases {
            let scope = build_create_kem_params(Default::default()).scope;
            let admitted_identities = admitted_identities_of(scope);
            let consumer = build_pairing_consumer(
                &*random_source.adapter,
                &admitted_statements,
                &admitted_identities,
                PairingConsumerConstructorParamsOverrides {
                    encoding: Some(AdmittedEncoding {
                        concrete: encoding.concrete,
                        identifier: encoding.identifier,
                    }),
                    chain_forms: Some(AdmittedChainForms {
                        concrete: forms.concrete,
                        identifier: forms.identifier,
                    }),
                    generate: Some(generate.clone()),
                    kinds: Some(kinds.clone()),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::NoFailure,
                    )),
                    ..Default::default()
                },
            );

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
