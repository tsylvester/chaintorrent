#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::cell::Cell;
use std::collections::HashSet;

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingConcrete, EncodingIdentifier,
    IDecoderAdapter, IEncoderAdapter,
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
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContext, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, RandomSourceKind, build_create_random_source_params,
};

use super::create_generate;
use super::interface::{
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload, ConsumeGenerateParams,
    ConsumeGeneratePayload, CreateGenerateErrorReturn, CreateGenerateParams, CreateGenerateReturn,
    EmitErrorReturn, EmitReturn, GENERATE_CONCRETES, GenerateConcrete, ICreateGenerateCaseConsumer,
    IGenerateAdapter, IGenerateConsumer, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode,
};
use super::mock::{
    CreateGenerateCaseParamsOverrides, CreateGenerateParamsOverrides, EmitPayloadOverrides,
    GenerateConcreteOverrides, GenerateKindConfigurationsOverrides,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, build_create_generate_deps, build_create_generate_params,
    build_create_generate_payload, build_emit_params, build_emit_payload, build_generate_concrete,
    build_generate_kind_configurations, run_create_generate_case,
};
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: `emit` on the adapter `create_generate` hands the consumer for a
///   member of `GENERATE_CONCRETES`, and for the mock selection, returns `Ok`
///   whose `files` names no path twice and holds at least as many paths as the
///   declaration the consumer reads from the adapter has kinds whose
///   `requires_second_group_arithmetic` is `false`.
/// Arrange: every member of `GENERATE_CONCRETES` crossed with every member of
///   `PAIRING_CONCRETES` and each member of `DECLARED_KEM_SELECTIONS`, over the
///   `DECLARED_ENCODING_SELECTIONS` and `CHAIN_FORMS_CONCRETES` selections the
///   concrete's declaration admits, and the mock selection under `NoFailure`
///   over the mock encoding and chain-forms defaults its declaration admits;
///   the payload two distinct canonicals and two distinct roots so the case's
///   KEM scope composes two admitted identities either way.
/// Act:    `emit` on the adapter the consumer receives, run once per case by
///   `run_create_generate_case`.
/// Assert: in place inside the inner consumer — every path unique by
///   `assert_eq!` on the set length, and `files` at least as many as the
///   ungated kinds by `assert!`; `result`'s `output` equals `Ok(())` per case.
/// Boundary: `create_generate`, the selected concrete's `try_new`, `emit`, and
///   the functions `emit` runs, each real; the filesystem a directory beneath
///   the operating system's temporary directory and the operating system's
///   generator the outer edges.
/// Mocked: nothing on the chain; the family's mock concrete as the mock
///   selection, which runs no filesystem.
#[test]
fn emit_returns_a_distinct_path_for_every_kind_the_declaration_does_not_gate() {
    // Arrange
    struct EmitConsumer<'a, F: IChainForms> {
        mint: &'a DeliveryContext<F, MintPurpose>,
        transfer: &'a DeliveryContext<F, TransferPurpose>,
        identities: &'a [AdmittedKemIdentity<'a>],
    }

    impl<'a, F: IChainForms> IGenerateConsumer<F> for EmitConsumer<'a, F> {
        type Output = ();

        fn consume_generate<G: IGenerateAdapter<Forms = F>>(
            &self,
            _params: ConsumeGenerateParams,
            payload: ConsumeGeneratePayload<G>,
        ) -> Self::Output {
            let result = payload.adapter.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    self.mint,
                    self.transfer,
                    self.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(self.identities[0]),
                    },
                ),
            );
            let success = result.unwrap();
            let ungated = <G as IGenerateAdapter>::DECLARATION
                .kinds
                .iter()
                .filter(|declaration| !declaration.requires_second_group_arithmetic)
                .count();
            assert!(success.files.len() >= ungated);
            let paths: Vec<_> = success.files.iter().map(|file| &file.path).collect();
            let unique: HashSet<_> = paths.iter().collect();
            assert_eq!(unique.len(), paths.len());
        }
    }

    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

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
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
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
                    EmitConsumer {
                        mint: &mint,
                        transfer: &transfer,
                        identities: &payload.identities,
                    },
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
                assert_eq!(result.unwrap().output.map(|s| s.output), Ok(()));
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
            assert_eq!(result.unwrap().output.map(|s| s.output), Ok(()));
        }
    }
}
// emit_returns_a_distinct_path_for_every_kind_the_declaration_does_not_gate

/// Contract: `emit` on the adapter `create_generate` hands the consumer for
///   `GenerateConcrete::Mock` carrying `MockIGenerateAdapterFailureMode::
///   WriteRefused` returns `Err(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused))`.
/// Arrange: the mock selection under `WriteRefused` crossed with every member
///   of `PAIRING_CONCRETES`, of the keccak cases of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, of `DECLARED_KEM_SELECTIONS`, of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and of `DELIVERY_PROOF_CONCRETES`,
///   over the mock encoding and chain-forms defaults the mock concrete's
///   declared `encoding_identifier` requires.
/// Act:    `emit` on the adapter the consumer receives, run once per case by
///   `run_create_generate_case`.
/// Assert: `result`'s `output` matches `Ok(Err(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused)))` per case.
/// Boundary: `create_generate`, the mock concrete's `try_new`, and `emit`.
/// Mocked: the family's mock concrete under its failure mode, the declared
///   set unable to reach it.
#[test]
fn a_refused_write_returns_the_mock_concretes_own_error() {
    // Arrange
    struct EmitConsumer<'a, F: IChainForms> {
        mint: &'a DeliveryContext<F, MintPurpose>,
        transfer: &'a DeliveryContext<F, TransferPurpose>,
        identities: &'a [AdmittedKemIdentity<'a>],
    }

    impl<'a, F: IChainForms> IGenerateConsumer<F> for EmitConsumer<'a, F> {
        type Output = EmitReturn;

        fn consume_generate<G: IGenerateAdapter<Forms = F>>(
            &self,
            _params: ConsumeGenerateParams,
            payload: ConsumeGeneratePayload<G>,
        ) -> Self::Output {
            payload.adapter.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    self.mint,
                    self.transfer,
                    self.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(self.identities[0]),
                    },
                ),
            )
        }
    }

    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<EmitReturn>;

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
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
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
                    EmitConsumer {
                        mint: &mint,
                        transfer: &transfer,
                        identities: &payload.identities,
                    },
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
                        assert!(matches!(
                            result.unwrap().output.map(|s| s.output),
                            Ok(Err(EmitErrorReturn::Mock(
                                MockIGenerateAdapterEmitErrorReturn::WriteRefused
                            )))
                        ));
                    }
                }
            }
        }
    }
}
// a_refused_write_returns_the_mock_concretes_own_error

/// Contract: `create_generate` over an encoder whose declared encoding
///   identifier is not the one the selected concrete's declaration requires
///   returns `Err(CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)`
///   with the consumer never called.
/// Arrange: every member of `GENERATE_CONCRETES` over `EncodingConcrete::Mock`,
///   whose declared identifier is `EncodingIdentifier::Mock`, under the mock
///   chain-forms default that admits it, and the mock selection over every
///   member of `DECLARED_ENCODING_SELECTIONS` under `CHAIN_FORMS_CONCRETES`'
///   selection, whose declaration admits each real encoder.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once per case by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(
///   CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)` by
///   `assert_eq!`, and the recording consumer's flag stays unset per case.
/// Boundary: `create_generate`, the encoder's declaration, and the selected
///   concrete's declaration, each real.
/// Mocked: the encoding family's mock concrete as the encoder for the real
///   generate concretes, and the family's mock concrete as the selection for
///   the real encoders, selected through their factories, nothing else.
#[test]
fn create_generate_refuses_an_encoder_declaring_another_encoding_identifier() {
    // Arrange
    struct RecordingConsumer<'a> {
        called: &'a Cell<bool>,
    }

    impl<F: IChainForms> IGenerateConsumer<F> for RecordingConsumer<'_> {
        type Output = ();

        fn consume_generate<G: IGenerateAdapter<Forms = F>>(
            &self,
            _params: ConsumeGenerateParams,
            _payload: ConsumeGeneratePayload<G>,
        ) -> Self::Output {
            self.called.set(true);
        }
    }

    struct Consumer<'a> {
        params: CreateGenerateParams,
        called: &'a Cell<bool>,
    }

    impl ICreateGenerateCaseConsumer for Consumer<'_> {
        type Output = CreateGenerateReturn<()>;

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
            create_generate(
                &build_create_generate_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    payload.kem,
                    payload.key_agreement,
                    payload.delivery_proof,
                    RecordingConsumer {
                        called: self.called,
                    },
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
        let called = Cell::new(false);
        let consumer = Consumer {
            params: build_create_generate_params(CreateGenerateParamsOverrides {
                concrete: Some(build_generate_concrete(*kind, GenerateConcreteOverrides)),
                kinds: Some(build_generate_kind_configurations(
                    *kind,
                    GenerateKindConfigurationsOverrides,
                )),
                ..Default::default()
            }),
            called: &called,
        };
        let case_params = build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
            encoding: Some(CreateEncodingParams {
                concrete: EncodingConcrete::Mock,
                identifier: EncodingIdentifier::Mock,
            }),
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
        assert_eq!(
            result.unwrap().output.map(|_| ()),
            Err(CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)
        );
        assert!(!called.get());
    }
    for encoding in DECLARED_ENCODING_SELECTIONS {
        let called = Cell::new(false);
        let consumer = Consumer {
            params: build_create_generate_params(CreateGenerateParamsOverrides {
                concrete: Some(GenerateConcrete::Mock(
                    MockIGenerateAdapterFailureMode::NoFailure,
                )),
                ..Default::default()
            }),
            called: &called,
        };
        let case_params = build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
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
        assert_eq!(
            result.unwrap().output.map(|_| ()),
            Err(CreateGenerateErrorReturn::UnsupportedEncodingIdentifier)
        );
        assert!(!called.get());
    }
}
// create_generate_refuses_an_encoder_declaring_another_encoding_identifier
