#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifierConstructorParamsOverrides,
    SecretConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier, build_secret, build_suite_identifier,
};
use encoding::{
    CanonicalFieldValue, CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS,
    FromFieldParams, ICanonicalField, IEncoderAdapter, ToFieldParams, build_create_encoding_params,
};
use envelope::{
    EnvelopeAlgebra, EnvelopeCoinsOverrides, EnvelopeComponents, IKeyAgreementAdapter,
    KeyPairComponentsOverrides, PublicKeysComponents, build_envelope_coins,
    build_key_pair_components, build_public_keys_components,
};
use hash_to_scalar::{
    CreateHashToScalarParams, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{ICredentialKemAdapter, MasterScalarComponentsOverrides, build_master_scalar_components};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, PAIRING_CONCRETES, VerifierGroupArithmetic,
    build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, RandomSourceKind, build_create_random_source_params,
};

use super::provides::{
    AlgebraicMintStatement, AlgebraicMintStatementOverrides, AlgebraicStatement,
    AlgebraicTransferStatement, AlgebraicTransferStatementOverrides, ConsumeDeliveryProofParams,
    ConsumeDeliveryProofPayload, CreateDeliveryProofCaseConsumeParams,
    CreateDeliveryProofCaseConsumePayload, CreateDeliveryProofCaseParamsOverrides,
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PROOF_INTERFACE_VERSION,
    DeliveryContextOverrides, DeliveryProofComponents, DeliveryProofConcrete,
    DeliveryProofDeclaration, DeliveryProofWireComponentsOverrides,
    ICreateDeliveryProofCaseConsumer, IDeliveryProofAdapter, IDeliveryProofConsumer,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    MockIDeliveryProofAdapterProveTransferErrorReturn, ProofComponentsParams,
    ProofFromComponentsErrorReturn, ProofFromComponentsParams, ProveMintErrorReturn,
    ProveMintParams, ProveMintPayloadOverrides, ProveTransferErrorReturn, ProveTransferParams,
    ProveTransferPayloadOverrides, VerifyParams, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_create_delivery_proof_case_deps,
    build_create_delivery_proof_case_params, build_create_delivery_proof_case_payload,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, build_delivery_context,
    build_delivery_proof_wire_components, build_mint_responses, build_proof_components_payload,
    build_proof_from_components_payload, build_prove_mint_payload, build_prove_transfer_payload,
    build_transfer_responses, build_verify_payload, create_delivery_proof,
    run_create_delivery_proof_case,
};
use crate::mint_statement::provides::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation, MintPurpose, TransferPurpose,
};

/// Contract: entry `an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version`;
///   `create_delivery_proof` called with a consumer returning the
///   `D::DECLARATION` of the adapter it receives, a selection, and the params'
///   algebra and statement version, over a received pairing → `Ok` whose output
///   declaration's `algebras` and `statement_versions` contain the params'
///   values, whose `verifier_forms` contains the form the pairing declares, and
///   whose `interface_version` equals `DELIVERY_PROOF_INTERFACE_VERSION`.
/// Arrange: every case of the chain's declared sets over the keccak
///   selections; the innermost consumer returns `D::DECLARATION`.
/// Act:     `run_create_delivery_proof_case` per case; `create_delivery_proof`
///   per member of `DELIVERY_PROOF_CONCRETES` and the `Mock` selection under
///   `NoFailure`.
/// Assert:  each runner result is `Ok`; each output's `algebras` contains the
///   params' algebra, `verifier_forms` the form the pairing declares,
///   `statement_versions` the params' version, and `interface_version` equals
///   `DELIVERY_PROOF_INTERFACE_VERSION`.
/// Boundary: `create_delivery_proof`, the selected concrete's `try_new`, and
///   `consume_delivery_proof` over the real chain.
/// Mocked:  nothing; the `Mock` selection stands for no real concrete and
///   proves only the mock concrete's answer.
#[test]
fn an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                ..
            } = payload;
            struct Delivery;

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F> for Delivery {
                type Output = DeliveryProofDeclaration;

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    _payload: ConsumeDeliveryProofPayload<D>,
                ) -> Self::Output {
                    D::DECLARATION
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery,
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                let output = result.expect("the factory answers").output;
                assert!(output.algebras.contains(&EnvelopeAlgebra::PairingElGamal));
                assert!(
                    output
                        .verifier_forms
                        .contains(&P::DECLARATION.verifier_group_arithmetic)
                );
                assert!(
                    output
                        .statement_versions
                        .contains(&DELIVERY_STATEMENT_VERSION_ONE)
                );
                assert_eq!(output.interface_version, DELIVERY_PROOF_INTERFACE_VERSION);
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_mint_proof_the_adapter_returns_verifies`; `prove_mint`
///   over the witnesses the statement holds, then `verify` over the same
///   statement and the proof, on the adapter `create_delivery_proof` hands the
///   consumer → `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: every case of the chain's declared sets over the keccak
///   selections; for `MintPurpose::Mint` and `MintPurpose::Replacement` the
///   statement `build_algebraic_mint_statement` builds and
///   `build_prove_mint_payload` over it.
/// Act:     `prove_mint`, then `verify` over the same statement and the proof.
/// Assert:  `verify`'s result equals `Ok` with `is_valid` true for each
///   purpose.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, `prove_mint`,
///   and `verify` over the real chain.
/// Mocked:  nothing; the real concrete draws its nonces from the operating
///   system's generator.
#[test]
fn a_mint_proof_the_adapter_returns_verifies() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                        let statement = build_algebraic_mint_statement::<P, F>(
                            pairing,
                            AlgebraicMintStatementOverrides {
                                context: Some(build_delivery_context(
                                    purpose,
                                    DeliveryContextOverrides::default(),
                                )),
                                ..Default::default()
                            },
                        );
                        let proved = adapter
                            .prove_mint(
                                ProveMintParams,
                                build_prove_mint_payload(pairing, &statement, Default::default()),
                            )
                            .expect("prove_mint returns the proof");
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Mint(&statement),
                                &proved.proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_transfer_proof_the_adapter_returns_verifies`;
///   `prove_transfer` over the witnesses the statement holds, then `verify`
///   over the same statement and the proof, on the adapter
///   `create_delivery_proof` hands the consumer → `Ok(VerifySuccessReturn
///   { is_valid: true })`.
/// Arrange: the cases and selections of the mint block; for
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant` the statement
///   `build_algebraic_transfer_statement` builds and
///   `build_prove_transfer_payload` over it.
/// Act:     `prove_transfer`, then `verify` over the same statement and the
///   proof.
/// Assert:  `verify`'s result equals `Ok` with `is_valid` true for each
///   purpose.
/// Boundary: as the mint block with `prove_transfer`.
/// Mocked:  nothing.
#[test]
fn a_transfer_proof_the_adapter_returns_verifies() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                        let statement = build_algebraic_transfer_statement::<P, F>(
                            pairing,
                            AlgebraicTransferStatementOverrides {
                                context: Some(build_delivery_context(
                                    purpose,
                                    DeliveryContextOverrides::default(),
                                )),
                                ..Default::default()
                            },
                        );
                        let proved = adapter
                            .prove_transfer(
                                ProveTransferParams,
                                build_prove_transfer_payload(
                                    pairing,
                                    &statement,
                                    Default::default(),
                                ),
                            )
                            .expect("prove_transfer returns the proof");
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Transfer(&statement),
                                &proved.proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_proof_the_adapter_rebuilds_from_its_components_verifies`;
///   a proof either prover returned, read by `proof_components` and handed to
///   `proof_from_components` on the same adapter, then `verify` over the
///   statement and the rebuilt proof → `Ok(VerifySuccessReturn
///   { is_valid: true })`.
/// Arrange: a mint proof and a transfer proof over the default statements and
///   payloads, each read by `proof_components` and handed to
///   `proof_from_components` on the same adapter.
/// Act:     `proof_components`, `proof_from_components`, then `verify` over the
///   statement and the rebuilt proof.
/// Assert:  `verify`'s result equals `Ok` with `is_valid` true for each
///   relation.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the provers,
///   `proof_components`, `proof_from_components`, and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_the_adapter_rebuilds_from_its_components_verifies() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    let mint_statement = build_algebraic_mint_statement::<P, F>(
                        pairing,
                        AlgebraicMintStatementOverrides::default(),
                    );
                    let mint_proof = adapter
                        .prove_mint(
                            ProveMintParams,
                            build_prove_mint_payload(pairing, &mint_statement, Default::default()),
                        )
                        .expect("prove_mint returns the proof")
                        .proof;
                    let transfer_statement = build_algebraic_transfer_statement::<P, F>(
                        pairing,
                        AlgebraicTransferStatementOverrides::default(),
                    );
                    let transfer_proof = adapter
                        .prove_transfer(
                            ProveTransferParams,
                            build_prove_transfer_payload(
                                pairing,
                                &transfer_statement,
                                Default::default(),
                            ),
                        )
                        .expect("prove_transfer returns the proof")
                        .proof;
                    for (statement, proof) in [
                        (AlgebraicStatement::Mint(&mint_statement), &mint_proof),
                        (
                            AlgebraicStatement::Transfer(&transfer_statement),
                            &transfer_proof,
                        ),
                    ] {
                        let components = adapter
                            .proof_components(
                                ProofComponentsParams,
                                build_proof_components_payload(proof, Default::default()),
                            )
                            .expect("the proof decomposes")
                            .components;
                        let rebuilt = adapter
                            .proof_from_components(
                                ProofFromComponentsParams,
                                build_proof_from_components_payload(components, Default::default()),
                            )
                            .expect("the components are accepted");
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(statement, &rebuilt.proof, Default::default()),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `components_of_the_other_verifier_form_are_refused_by_the_adapter`;
///   `proof_from_components` over components of the verifier form other than
///   the one the adapter was constructed in →
///   `Err(ProofFromComponentsErrorReturn::VerifierFormMismatch)`.
/// Arrange: the adapter the factory constructs in the form the received
///   pairing declares, and the components `DeliveryProofComponents::try_from`
///   returns for `build_delivery_proof_wire_components` overridden to the
///   other verifier form, with the response and first-message counts that
///   form requires, for the mint relation and for the transfer relation.
/// Act:     `proof_from_components` over the components.
/// Assert:  the error equals `ProofFromComponentsErrorReturn::VerifierFormMismatch`
///   for each relation.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, and
///   `proof_from_components`.
/// Mocked:  nothing.
#[test]
fn components_of_the_other_verifier_form_are_refused_by_the_adapter() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    let other_form = match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => {
                            VerifierGroupArithmetic::FirstGroupOnly
                        }
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            VerifierGroupArithmetic::BothGroups
                        }
                    };
                    let g2 = build_public_keys_components(pairing, Default::default()).pk2;
                    for relation in [DeliveryRelation::Mint, DeliveryRelation::Transfer] {
                        let responses = match relation {
                            DeliveryRelation::Mint => {
                                let responses = build_mint_responses(pairing, Default::default());
                                vec![responses.alpha, responses.r, responses.rho, responses.sigma]
                            }
                            DeliveryRelation::Transfer => {
                                let responses =
                                    build_transfer_responses(pairing, Default::default());
                                vec![
                                    responses.x,
                                    responses.y,
                                    responses.s,
                                    responses.rho,
                                    responses.sigma,
                                ]
                            }
                        };
                        let wire = build_delivery_proof_wire_components(
                            pairing,
                            DeliveryProofWireComponentsOverrides {
                                relation: Some(relation),
                                verifier_form: Some(other_form),
                                responses: Some(responses),
                                second_group_first_messages: Some(match other_form {
                                    VerifierGroupArithmetic::BothGroups => vec![],
                                    VerifierGroupArithmetic::FirstGroupOnly => {
                                        vec![g2.clone(), g2.clone(), g2.clone()]
                                    }
                                }),
                                ..Default::default()
                            },
                        );
                        let components =
                            DeliveryProofComponents::try_from(wire).expect("the wire converts");
                        let result = adapter.proof_from_components(
                            ProofFromComponentsParams,
                            build_proof_from_components_payload(components, Default::default()),
                        );
                        assert_eq!(
                            result.err(),
                            Some(ProofFromComponentsErrorReturn::VerifierFormMismatch)
                        );
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_proof_of_another_relation_is_refused_by_verify`;
///   `verify` over a statement of one relation and a proof of the other
///   relation → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: a transfer proof from `prove_transfer` and a mint statement, and
///   a mint proof from `prove_mint` and a transfer statement.
/// Act:     `verify` over each statement and the other relation's proof.
/// Assert:  each result equals `Ok` with `is_valid` false.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the provers,
///   and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_of_another_relation_is_refused_by_verify() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    let mint_statement = build_algebraic_mint_statement::<P, F>(
                        pairing,
                        AlgebraicMintStatementOverrides::default(),
                    );
                    let mint_proof = adapter
                        .prove_mint(
                            ProveMintParams,
                            build_prove_mint_payload(pairing, &mint_statement, Default::default()),
                        )
                        .expect("prove_mint returns the proof")
                        .proof;
                    let transfer_statement = build_algebraic_transfer_statement::<P, F>(
                        pairing,
                        AlgebraicTransferStatementOverrides::default(),
                    );
                    let transfer_proof = adapter
                        .prove_transfer(
                            ProveTransferParams,
                            build_prove_transfer_payload(
                                pairing,
                                &transfer_statement,
                                Default::default(),
                            ),
                        )
                        .expect("prove_transfer returns the proof")
                        .proof;
                    for (statement, proof) in [
                        (AlgebraicStatement::Mint(&mint_statement), &transfer_proof),
                        (
                            AlgebraicStatement::Transfer(&transfer_statement),
                            &mint_proof,
                        ),
                    ] {
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(statement, proof, Default::default()),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_proof_against_a_statement_differing_in_one_field_is_refused`;
///   `verify` over a proof formed for the baseline statement and a statement
///   that differs from it in one field → `Ok(VerifySuccessReturn
///   { is_valid: false })`.
/// Arrange: a proof over the baseline statement, and a statement overridden in
///   one field alone with a value from that field's own builder distinct from
///   its default, for each field of `AlgebraicMintStatement` and of
///   `AlgebraicTransferStatement`, the context fields included.
/// Act:     `verify` over the differing statement and the proof.
/// Assert:  each result equals `Ok` with `is_valid` false, and `verify` over
///   the baseline statement equals `Ok` with `is_valid` true.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the provers,
///   and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_against_a_statement_differing_in_one_field_is_refused() {
    fn other_canonical<T: ICanonicalField>(of: &T) -> T {
        let field = T::to_field(ToFieldParams, of)
            .expect("the value encodes")
            .field;
        let other = match field {
            CanonicalFieldValue::FixedBytes32(mut value) => {
                value[0] ^= 0xFF;
                CanonicalFieldValue::FixedBytes32(value)
            }
            CanonicalFieldValue::Unsigned16(value) => CanonicalFieldValue::Unsigned16(value + 1),
            CanonicalFieldValue::Unsigned32(value) => CanonicalFieldValue::Unsigned32(value + 1),
            CanonicalFieldValue::Unsigned64(value) => CanonicalFieldValue::Unsigned64(value + 1),
            CanonicalFieldValue::Text(mut value) => {
                value.push('!');
                CanonicalFieldValue::Text(value)
            }
            CanonicalFieldValue::Bytes(mut value) => {
                value.push(0xFF);
                CanonicalFieldValue::Bytes(value)
            }
            CanonicalFieldValue::FixedBytes20(mut value) => {
                value[0] ^= 0xFF;
                CanonicalFieldValue::FixedBytes20(value)
            }
            CanonicalFieldValue::Unsigned256(mut value) => {
                value[0] ^= 0xFF;
                CanonicalFieldValue::Unsigned256(value)
            }
        };
        T::from_field(FromFieldParams, other)
            .expect("the perturbed value decodes")
            .value
    }

    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    let mint_statement = build_algebraic_mint_statement::<P, F>(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let mint_proof = adapter
                        .prove_mint(
                            ProveMintParams,
                            build_prove_mint_payload(pairing, &mint_statement, Default::default()),
                        )
                        .expect("prove_mint returns the proof")
                        .proof;
                    assert_eq!(
                        adapter
                            .verify(
                                VerifyParams,
                                build_verify_payload(
                                    AlgebraicStatement::Mint(&mint_statement),
                                    &mint_proof,
                                    Default::default(),
                                ),
                            )
                            .ok()
                            .map(|success| success.is_valid),
                        Some(true)
                    );
                    let context = &mint_statement.context;
                    let mint_differing: Vec<AlgebraicMintStatement<F, P::G1, P::G2>> = vec![
                        build_algebraic_mint_statement::<P, F>(
                            pairing,
                            AlgebraicMintStatementOverrides {
                                hpub: Some(mint_statement.envelope.d2.clone()),
                                ..Default::default()
                            },
                        ),
                        build_algebraic_mint_statement::<P, F>(
                            pairing,
                            AlgebraicMintStatementOverrides {
                                identity_element: Some(mint_statement.buyer_keys.pk1.clone()),
                                ..Default::default()
                            },
                        ),
                        build_algebraic_mint_statement::<P, F>(
                            pairing,
                            AlgebraicMintStatementOverrides {
                                buyer_keys: Some(PublicKeysComponents {
                                    pk1: mint_statement.identity_element.clone(),
                                    pk2: mint_statement.hpub.clone(),
                                }),
                                ..Default::default()
                            },
                        ),
                        build_algebraic_mint_statement::<P, F>(
                            pairing,
                            AlgebraicMintStatementOverrides {
                                envelope: Some(EnvelopeComponents {
                                    c1: mint_statement.envelope.c2.clone(),
                                    c2: mint_statement.envelope.c1.clone(),
                                    d1: mint_statement.envelope.d2.clone(),
                                    d2: mint_statement.envelope.d1.clone(),
                                }),
                                ..Default::default()
                            },
                        ),
                    ];
                    let mint_contexts = vec![
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                suite_identifier: Some(build_suite_identifier(
                                    SuiteIdentifierConstructorParamsOverrides {
                                        identifier: Some([0xA6; 32]),
                                        ..Default::default()
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                chain: Some(other_canonical(&context.chain)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                entitlement_contract: Some(other_canonical(
                                    &context.entitlement_contract,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                asset_root: Some(build_root(RootConstructorParamsOverrides {
                                    bytes: Some([0xA5; 32]),
                                })),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                parameter_set_identifier: Some(build_parameter_set_identifier(
                                    ParameterSetIdentifierConstructorParamsOverrides {
                                        bytes: Some([0xA7; PARAMETER_SET_IDENTIFIER_LENGTH]),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                source_entitlement: Some(other_canonical(
                                    &context.source_entitlement,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                target_entitlement: Some(other_canonical(
                                    &context.target_entitlement,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                old_interval: Some(other_canonical(&context.old_interval)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                new_interval: Some(other_canonical(&context.new_interval)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                seller: Some(other_canonical(&context.seller)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                buyer: Some(other_canonical(&context.buyer)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Mint,
                            DeliveryContextOverrides {
                                expiry: Some(context.expiry + 1),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            MintPurpose::Replacement,
                            DeliveryContextOverrides::default(),
                        ),
                    ];
                    for differing in
                        mint_differing
                            .into_iter()
                            .chain(mint_contexts.into_iter().map(|context| {
                                build_algebraic_mint_statement::<P, F>(
                                    pairing,
                                    AlgebraicMintStatementOverrides {
                                        context: Some(context),
                                        ..Default::default()
                                    },
                                )
                            }))
                    {
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Mint(&differing),
                                &mint_proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                    }
                    let transfer_statement = build_algebraic_transfer_statement::<P, F>(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                TransferPurpose::Transfer,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let transfer_proof = adapter
                        .prove_transfer(
                            ProveTransferParams,
                            build_prove_transfer_payload(
                                pairing,
                                &transfer_statement,
                                Default::default(),
                            ),
                        )
                        .expect("prove_transfer returns the proof")
                        .proof;
                    assert_eq!(
                        adapter
                            .verify(
                                VerifyParams,
                                build_verify_payload(
                                    AlgebraicStatement::Transfer(&transfer_statement),
                                    &transfer_proof,
                                    Default::default(),
                                ),
                            )
                            .ok()
                            .map(|success| success.is_valid),
                        Some(true)
                    );
                    let context = &transfer_statement.context;
                    let transfer_differing: Vec<AlgebraicTransferStatement<F, P::G1, P::G2>> = vec![
                        build_algebraic_transfer_statement::<P, F>(
                            pairing,
                            AlgebraicTransferStatementOverrides {
                                identity_element: Some(transfer_statement.buyer_keys.pk1.clone()),
                                ..Default::default()
                            },
                        ),
                        build_algebraic_transfer_statement::<P, F>(
                            pairing,
                            AlgebraicTransferStatementOverrides {
                                seller_keys: Some(PublicKeysComponents {
                                    pk1: transfer_statement.buyer_keys.pk1.clone(),
                                    pk2: transfer_statement.buyer_keys.pk2.clone(),
                                }),
                                ..Default::default()
                            },
                        ),
                        build_algebraic_transfer_statement::<P, F>(
                            pairing,
                            AlgebraicTransferStatementOverrides {
                                buyer_keys: Some(PublicKeysComponents {
                                    pk1: transfer_statement.seller_keys.pk1.clone(),
                                    pk2: transfer_statement.seller_keys.pk2.clone(),
                                }),
                                ..Default::default()
                            },
                        ),
                        build_algebraic_transfer_statement::<P, F>(
                            pairing,
                            AlgebraicTransferStatementOverrides {
                                old_envelope: Some(EnvelopeComponents {
                                    c1: transfer_statement.new_envelope.c1.clone(),
                                    c2: transfer_statement.new_envelope.c2.clone(),
                                    d1: transfer_statement.new_envelope.d1.clone(),
                                    d2: transfer_statement.new_envelope.d2.clone(),
                                }),
                                ..Default::default()
                            },
                        ),
                        build_algebraic_transfer_statement::<P, F>(
                            pairing,
                            AlgebraicTransferStatementOverrides {
                                new_envelope: Some(EnvelopeComponents {
                                    c1: transfer_statement.old_envelope.c1.clone(),
                                    c2: transfer_statement.old_envelope.c2.clone(),
                                    d1: transfer_statement.old_envelope.d1.clone(),
                                    d2: transfer_statement.old_envelope.d2.clone(),
                                }),
                                ..Default::default()
                            },
                        ),
                    ];
                    let transfer_contexts = vec![
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                suite_identifier: Some(build_suite_identifier(
                                    SuiteIdentifierConstructorParamsOverrides {
                                        identifier: Some([0xA6; 32]),
                                        ..Default::default()
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                chain: Some(other_canonical(&context.chain)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                entitlement_contract: Some(other_canonical(
                                    &context.entitlement_contract,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                asset_root: Some(build_root(RootConstructorParamsOverrides {
                                    bytes: Some([0xA5; 32]),
                                })),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                parameter_set_identifier: Some(build_parameter_set_identifier(
                                    ParameterSetIdentifierConstructorParamsOverrides {
                                        bytes: Some([0xA7; PARAMETER_SET_IDENTIFIER_LENGTH]),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                source_entitlement: Some(other_canonical(
                                    &context.source_entitlement,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                target_entitlement: Some(other_canonical(
                                    &context.target_entitlement,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                old_interval: Some(other_canonical(&context.old_interval)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                new_interval: Some(other_canonical(&context.new_interval)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                seller: Some(other_canonical(&context.seller)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                buyer: Some(other_canonical(&context.buyer)),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Transfer,
                            DeliveryContextOverrides {
                                expiry: Some(context.expiry + 1),
                                ..Default::default()
                            },
                        ),
                        build_delivery_context(
                            TransferPurpose::Grant,
                            DeliveryContextOverrides::default(),
                        ),
                    ];
                    for differing in
                        transfer_differing
                            .into_iter()
                            .chain(transfer_contexts.into_iter().map(|context| {
                                build_algebraic_transfer_statement::<P, F>(
                                    pairing,
                                    AlgebraicTransferStatementOverrides {
                                        context: Some(context),
                                        ..Default::default()
                                    },
                                )
                            }))
                    {
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Transfer(&differing),
                                &transfer_proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_proof_with_a_changed_response_or_challenge_is_refused`;
///   `verify` over a proof whose challenge or one response is replaced by
///   another scalar → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: a proof from `prove_mint` or `prove_transfer` over its statement
///   and payload builders; the wire from `build_delivery_proof_wire_components`
///   overridden with the relation, the verifier form, the challenge, the
///   responses, and the second-group first messages of that proof's
///   `proof_components`, with the challenge or one response replaced alone by a
///   scalar of `build_mint_responses` or `build_transfer_responses` that
///   differs from the proof's value, converted by
///   `DeliveryProofComponents::try_from` and rebuilt by `proof_from_components`.
/// Act:     `verify` over the statement and the rebuilt proof.
/// Assert:  each result equals `Ok` with `is_valid` false.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the provers,
///   `proof_components`, `proof_from_components`, and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_with_a_changed_response_or_challenge_is_refused() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    let mint_statement = build_algebraic_mint_statement::<P, F>(
                        pairing,
                        AlgebraicMintStatementOverrides::default(),
                    );
                    let mint_proof = adapter
                        .prove_mint(
                            ProveMintParams,
                            build_prove_mint_payload(pairing, &mint_statement, Default::default()),
                        )
                        .expect("prove_mint returns the proof")
                        .proof;
                    let transfer_statement = build_algebraic_transfer_statement::<P, F>(
                        pairing,
                        AlgebraicTransferStatementOverrides::default(),
                    );
                    let transfer_proof = adapter
                        .prove_transfer(
                            ProveTransferParams,
                            build_prove_transfer_payload(
                                pairing,
                                &transfer_statement,
                                Default::default(),
                            ),
                        )
                        .expect("prove_transfer returns the proof")
                        .proof;
                    let fresh_mint = build_mint_responses(pairing, Default::default());
                    let fresh_transfer = build_transfer_responses(pairing, Default::default());
                    for (statement, proof, other) in [
                        (
                            AlgebraicStatement::Mint(&mint_statement),
                            &mint_proof,
                            fresh_mint.alpha.clone(),
                        ),
                        (
                            AlgebraicStatement::Transfer(&transfer_statement),
                            &transfer_proof,
                            fresh_transfer.x.clone(),
                        ),
                    ] {
                        let components = adapter
                            .proof_components(
                                ProofComponentsParams,
                                build_proof_components_payload(proof, Default::default()),
                            )
                            .expect("the proof decomposes")
                            .components;
                        let (relation, verifier_form, challenge, responses, first_messages) =
                            match &components {
                                DeliveryProofComponents::MintBothGroups {
                                    challenge,
                                    responses,
                                } => (
                                    DeliveryRelation::Mint,
                                    VerifierGroupArithmetic::BothGroups,
                                    challenge.clone(),
                                    vec![
                                        responses.alpha.clone(),
                                        responses.r.clone(),
                                        responses.rho.clone(),
                                        responses.sigma.clone(),
                                    ],
                                    vec![],
                                ),
                                DeliveryProofComponents::MintFirstGroupOnly {
                                    challenge,
                                    responses,
                                    first_messages,
                                } => (
                                    DeliveryRelation::Mint,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                    challenge.clone(),
                                    vec![
                                        responses.alpha.clone(),
                                        responses.r.clone(),
                                        responses.rho.clone(),
                                        responses.sigma.clone(),
                                    ],
                                    vec![
                                        first_messages.hpub.clone(),
                                        first_messages.d1.clone(),
                                        first_messages.d2.clone(),
                                    ],
                                ),
                                DeliveryProofComponents::TransferBothGroups {
                                    challenge,
                                    responses,
                                } => (
                                    DeliveryRelation::Transfer,
                                    VerifierGroupArithmetic::BothGroups,
                                    challenge.clone(),
                                    vec![
                                        responses.x.clone(),
                                        responses.y.clone(),
                                        responses.s.clone(),
                                        responses.rho.clone(),
                                        responses.sigma.clone(),
                                    ],
                                    vec![],
                                ),
                                DeliveryProofComponents::TransferFirstGroupOnly {
                                    challenge,
                                    responses,
                                    first_messages,
                                } => (
                                    DeliveryRelation::Transfer,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                    challenge.clone(),
                                    vec![
                                        responses.x.clone(),
                                        responses.y.clone(),
                                        responses.s.clone(),
                                        responses.rho.clone(),
                                        responses.sigma.clone(),
                                    ],
                                    vec![
                                        first_messages.pk2.clone(),
                                        first_messages.d1.clone(),
                                        first_messages.d2.clone(),
                                    ],
                                ),
                            };
                        let mut changed_response = responses.clone();
                        changed_response[0] = other.clone();
                        for tampered in [true, false] {
                            let wire = build_delivery_proof_wire_components(
                                pairing,
                                DeliveryProofWireComponentsOverrides {
                                    relation: Some(relation),
                                    verifier_form: Some(verifier_form),
                                    challenge: Some(if tampered {
                                        other.clone()
                                    } else {
                                        challenge.clone()
                                    }),
                                    responses: Some(if tampered {
                                        responses.clone()
                                    } else {
                                        changed_response.clone()
                                    }),
                                    second_group_first_messages: Some(first_messages.clone()),
                                },
                            );
                            let components =
                                DeliveryProofComponents::try_from(wire).expect("the wire converts");
                            let rebuilt = adapter
                                .proof_from_components(
                                    ProofFromComponentsParams,
                                    build_proof_from_components_payload(
                                        components,
                                        Default::default(),
                                    ),
                                )
                                .expect("the components are accepted");
                            let result = adapter.verify(
                                VerifyParams,
                                build_verify_payload(
                                    match &statement {
                                        AlgebraicStatement::Mint(held) => {
                                            AlgebraicStatement::Mint(*held)
                                        }
                                        AlgebraicStatement::Transfer(held) => {
                                            AlgebraicStatement::Transfer(*held)
                                        }
                                    },
                                    &rebuilt.proof,
                                    Default::default(),
                                ),
                            );
                            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                        }
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_proof_from_a_witness_the_statement_does_not_hold_is_refused`;
///   `prove_mint` or `prove_transfer` over a witness other than the one the
///   statement holds returns `Ok`, and `verify` over the statement and that
///   proof → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the statement from its builder and a payload overridden in one
///   witness alone with a value from that witness's builder distinct from its
///   default, for each witness of both relations.
/// Act:     the prover over the payload, then `verify` over the statement and
///   the proof.
/// Assert:  the prover's result is `Ok` and `verify`'s result equals `Ok`
///   with `is_valid` false, for each witness.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the provers,
///   and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_from_a_witness_the_statement_does_not_hold_is_refused() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    let fresh = build_transfer_responses(pairing, Default::default());
                    let mint_statement = build_algebraic_mint_statement::<P, F>(
                        pairing,
                        AlgebraicMintStatementOverrides::default(),
                    );
                    let mint_payloads = vec![
                        build_prove_mint_payload(
                            pairing,
                            &mint_statement,
                            ProveMintPayloadOverrides {
                                master_scalar: Some(build_master_scalar_components(
                                    MasterScalarComponentsOverrides {
                                        value: Some(build_secret(
                                            fresh.x.clone(),
                                            SecretConstructorParamsOverrides,
                                        )),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_prove_mint_payload(
                            pairing,
                            &mint_statement,
                            ProveMintPayloadOverrides {
                                credential_randomness: Some(build_secret(
                                    fresh.y.clone(),
                                    SecretConstructorParamsOverrides,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_prove_mint_payload(
                            pairing,
                            &mint_statement,
                            ProveMintPayloadOverrides {
                                coins: Some(build_envelope_coins(
                                    pairing,
                                    EnvelopeCoinsOverrides {
                                        rho: Some(build_secret(
                                            fresh.s.clone(),
                                            SecretConstructorParamsOverrides,
                                        )),
                                        ..Default::default()
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                    ];
                    for payload in mint_payloads {
                        let proved = adapter.prove_mint(ProveMintParams, payload);
                        let proof = proved.expect("the prover answers").proof;
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Mint(&mint_statement),
                                &proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                    }
                    let transfer_statement = build_algebraic_transfer_statement::<P, F>(
                        pairing,
                        AlgebraicTransferStatementOverrides::default(),
                    );
                    let transfer_payloads = vec![
                        build_prove_transfer_payload(
                            pairing,
                            &transfer_statement,
                            ProveTransferPayloadOverrides {
                                seller_secrets: Some(build_key_pair_components(
                                    pairing,
                                    KeyPairComponentsOverrides {
                                        x: Some(build_secret(
                                            fresh.rho.clone(),
                                            SecretConstructorParamsOverrides,
                                        )),
                                        ..Default::default()
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_prove_transfer_payload(
                            pairing,
                            &transfer_statement,
                            ProveTransferPayloadOverrides {
                                offset: Some(build_secret(
                                    fresh.sigma.clone(),
                                    SecretConstructorParamsOverrides,
                                )),
                                ..Default::default()
                            },
                        ),
                        build_prove_transfer_payload(
                            pairing,
                            &transfer_statement,
                            ProveTransferPayloadOverrides {
                                coins: Some(build_envelope_coins(
                                    pairing,
                                    EnvelopeCoinsOverrides {
                                        sigma: Some(build_secret(
                                            fresh.y.clone(),
                                            SecretConstructorParamsOverrides,
                                        )),
                                        ..Default::default()
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                    ];
                    for payload in transfer_payloads {
                        let proved = adapter.prove_transfer(ProveTransferParams, payload);
                        let proof = proved.expect("the prover answers").proof;
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Transfer(&transfer_statement),
                                &proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                    }
                }
            }

            for concrete in
                DELIVERY_PROOF_CONCRETES
                    .iter()
                    .copied()
                    .chain([DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                    )])
            {
                let result = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        Delivery { pairing, forms },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}

/// Contract: entry `a_refused_nonce_draw_is_returned_by_each_prover`;
///   `prove_mint` and `prove_transfer` on the adapter `create_delivery_proof`
///   hands the consumer for the `Mock` selection under `NonceDrawRefused` →
///   the mock concrete's own `NonceDrawRefused` error from each.
/// Arrange: the `Mock` selection under
///   `MockIDeliveryProofAdapterFailureMode::NonceDrawRefused` over every member
///   of `PAIRING_CONCRETES`, the default statements and payloads.
/// Act:     `prove_mint`, then `prove_transfer`.
/// Assert:  the mint error equals `ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)` and the
///   transfer error equals `ProveTransferErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused)`.
/// Boundary: `create_delivery_proof`, the mock concrete's `try_new`,
///   `prove_mint`, and `prove_transfer`.
/// Mocked:  nothing; the block runs over the mock concrete alone, since the
///   declared set cannot reach the failure mode.
#[test]
fn a_refused_nonce_draw_is_returned_by_each_prover() {
    struct Consumer;

    impl ICreateDeliveryProofCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_delivery_proof_case<
            P: IPairingArithmetic,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
        >(
            &self,
            _params: CreateDeliveryProofCaseConsumeParams,
            payload: CreateDeliveryProofCaseConsumePayload<'_, P, E, F, K, A>,
        ) -> Self::Output {
            let CreateDeliveryProofCaseConsumePayload {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                forms,
                ..
            } = payload;
            struct Delivery<'p, 'f, P: IPairingArithmetic, F: IChainForms> {
                pairing: &'p P,
                forms: &'f F,
            }

            impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
                for Delivery<'_, '_, P, F>
            {
                type Output = ();

                fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
                    &self,
                    _params: ConsumeDeliveryProofParams,
                    payload: ConsumeDeliveryProofPayload<D>,
                ) {
                    let adapter = payload.adapter;
                    let pairing = self.pairing;
                    let _ = self.forms;
                    let mint_statement = build_algebraic_mint_statement::<P, F>(
                        pairing,
                        AlgebraicMintStatementOverrides::default(),
                    );
                    let mint = adapter.prove_mint(
                        ProveMintParams,
                        build_prove_mint_payload(pairing, &mint_statement, Default::default()),
                    );
                    assert_eq!(
                        mint.err(),
                        Some(ProveMintErrorReturn::Mock(
                            MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                        ))
                    );
                    let transfer_statement = build_algebraic_transfer_statement::<P, F>(
                        pairing,
                        AlgebraicTransferStatementOverrides::default(),
                    );
                    let transfer = adapter.prove_transfer(
                        ProveTransferParams,
                        build_prove_transfer_payload(
                            pairing,
                            &transfer_statement,
                            Default::default(),
                        ),
                    );
                    assert_eq!(
                        transfer.err(),
                        Some(ProveTransferErrorReturn::Mock(
                            MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused
                        ))
                    );
                }
            }

            let result = create_delivery_proof::<P, E, F, _>(
                &build_create_delivery_proof_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    Delivery { pairing, forms },
                    Default::default(),
                ),
                build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                    concrete: Some(DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                    )),
                    ..Default::default()
                }),
                build_create_delivery_proof_payload(Default::default()),
            );
            assert!(result.is_ok());
        }
    }

    let keccak_cases: Vec<CreateHashToScalarParams> = DECLARED_HASH_TO_SCALAR_SELECTIONS
        .iter()
        .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
        .cloned()
        .collect();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar in &keccak_cases {
                    let result = run_create_delivery_proof_case(
                        &build_create_delivery_proof_case_deps(Consumer, Default::default()),
                        build_create_delivery_proof_case_params(
                            CreateDeliveryProofCaseParamsOverrides {
                                pairing: Some(build_create_pairing_params(
                                    CreatePairingParamsOverrides {
                                        concrete: Some(*pairing_concrete),
                                        ..Default::default()
                                    },
                                )),
                                hash_to_scalar: Some(*hash_to_scalar),
                                encoding: Some(build_create_encoding_params(
                                    CreateEncodingParamsOverrides {
                                        concrete: Some(encoding.concrete),
                                        identifier: Some(encoding.identifier),
                                    },
                                )),
                                chain_forms: Some(build_create_chain_forms_params(
                                    CreateChainFormsParamsOverrides {
                                        concrete: Some(*chain_forms_concrete),
                                        ..Default::default()
                                    },
                                )),
                                random_source: Some(build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                )),
                                ..Default::default()
                            },
                        ),
                        build_create_delivery_proof_case_payload(Default::default()),
                    );
                    assert!(result.is_ok());
                }
            }
        }
    }
}
