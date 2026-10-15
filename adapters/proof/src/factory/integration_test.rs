#![allow(clippy::expect_used)]

use super::provides::{
    AlgebraicMintStatementOverrides, AlgebraicStatement, AlgebraicTransferStatementOverrides,
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParamsOverrides,
    DELIVERY_PROOF_CONCRETES, DELIVERY_PROOF_INTERFACE_VERSION, DeliveryContextOverrides,
    DeliveryProofComponents, DeliveryProofConcrete, DeliveryProofDeclaration,
    DeliveryProofWireComponentsOverrides, IDeliveryProofAdapter, IDeliveryProofConsumer,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    MockIDeliveryProofAdapterProveTransferErrorReturn, ProofComponentsParams,
    ProofFromComponentsErrorReturn, ProofFromComponentsParams, ProveMintErrorReturn,
    ProveMintParams, ProveMintPayloadOverrides, ProveTransferErrorReturn, ProveTransferParams,
    ProveTransferPayloadOverrides, VerifyParams, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload,
    build_delivery_context, build_delivery_proof_wire_components, build_mint_responses,
    build_mint_second_group_first_messages, build_proof_components_payload,
    build_proof_from_components_payload, build_prove_mint_payload, build_prove_transfer_payload,
    build_transfer_responses, build_transfer_second_group_first_messages, build_verify_payload,
    create_delivery_proof,
};
use crate::mint_statement::provides::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation, MintPurpose, TransferPurpose,
};
use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashConstructorParamsOverrides,
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifierConstructorParamsOverrides,
    SUITE_IDENTIFIER_LENGTH, SecretConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_secret, build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, FromFieldParamsOverrides,
    ICanonicalField, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_from_field_params, create_encoding,
};
use envelope::{
    EnvelopeAlgebra, EnvelopeCoinsOverrides, EnvelopeComponentsOverrides,
    KeyPairComponentsOverrides, PublicKeysComponentsOverrides, build_envelope_coins,
    build_envelope_components, build_key_pair_components, build_public_keys_components,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{MasterScalarComponentsOverrides, build_master_scalar_components};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES,
    SampleUniformScalarParams, SampleUniformScalarPayload, VerifierGroupArithmetic,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

/// Contract: entry
///   `an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version`;
///   `create_delivery_proof` returns the `D::DECLARATION` the consumer read,
///   whose `algebras`, `verifier_forms`, and `statement_versions` contain the
///   params' values and whose `interface_version` is
///   `DELIVERY_PROOF_INTERFACE_VERSION`.
/// Arrange: every member of `PAIRING_CONCRETES`, nested over every member of
///   `DECLARED_ENCODING_SELECTIONS`, every member of `CHAIN_FORMS_CONCRETES`,
///   and every `keccak` case of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, the
///   randomness source `create_random_source` returns for
///   `RandomSourceKind::OperatingSystem`; `create_delivery_proof` once per
///   member of `DELIVERY_PROOF_CONCRETES`, once for the `Mock` selection under
///   `NoFailure`, each in both verifier forms, with params from
///   `build_create_delivery_proof_params` carrying the selection, the form,
///   `EnvelopeAlgebra::PairingElGamal`, and `DELIVERY_STATEMENT_VERSION_ONE`;
///   `DeclarationConsumer` returns `D::DECLARATION`.
/// Act:     `create_delivery_proof`.
/// Assert:  the output's `algebras` contains the params' algebra,
///   `verifier_forms` the params' form, `statement_versions` the params'
///   version, and `interface_version` equals
///   `DELIVERY_PROOF_INTERFACE_VERSION`; every factory result `is_ok()` in
///   place.
/// Boundary: `create_delivery_proof`, the selected concrete's `try_new`, and
///   `consume_delivery_proof` over the real chain.
/// Mocked:  nothing; the `Mock` selection stands for no real concrete and
///   proves only the mock concrete's answer.
#[test]
fn an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version() {
    struct DeclarationConsumer;

    impl<P: IPairingAdapter, F: IChainForms> IDeliveryProofConsumer<P, F> for DeclarationConsumer {
        type Output = DeliveryProofDeclaration;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            _payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            D::DECLARATION
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                // Arrange
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let params = build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(concrete),
                                                algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                verifier_form: Some(verifier_form),
                                                statement_version: Some(
                                                    DELIVERY_STATEMENT_VERSION_ONE,
                                                ),
                                            },
                                        );
                                        let algebra = params.algebra;
                                        let statement_version = params.statement_version;

                                        // Act
                                        let created =
                                            create_delivery_proof::<P, E, F, DeclarationConsumer>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    &*hashed.adapter,
                                                    self.encoder,
                                                    &*random.adapter,
                                                    DeclarationConsumer,
                                                    Default::default(),
                                                ),
                                                params,
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            );

                                        // Assert
                                        assert!(created.is_ok());
                                        let declaration = created
                                            .expect(
                                                "the admitted selection returns the consumer's output",
                                            )
                                            .output;
                                        assert!(declaration.algebras.contains(&algebra));
                                        assert!(
                                            declaration.verifier_forms.contains(&verifier_form)
                                        );
                                        assert!(
                                            declaration
                                                .statement_versions
                                                .contains(&statement_version)
                                        );
                                        assert_eq!(
                                            declaration.interface_version,
                                            DELIVERY_PROOF_INTERFACE_VERSION
                                        );
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version
}

/// Contract: entry `a_mint_proof_the_adapter_returns_verifies`; `prove_mint`
///   over the witnesses the statement holds followed by `verify` over the
///   same statement on the adapter `create_delivery_proof` hands the
///   consumer returns `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: the cases of the declaration block; for each of
///   `MintPurpose::Mint` and `MintPurpose::Replacement` the statement
///   `build_algebraic_mint_statement` builds over a context with that
///   purpose, and `build_prove_mint_payload` over it.
/// Act:     `prove_mint`, then `verify`, inside `MintVerifyConsumer`.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)`; every factory result `is_ok()`.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`,
///   `prove_mint`, and `verify` over the real chain.
/// Mocked:  nothing; the real concrete draws its nonces from the operating
///   system's generator.
#[test]
fn a_mint_proof_the_adapter_returns_verifies() {
    struct MintVerifyConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for MintVerifyConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                // Arrange
                let statement = build_algebraic_mint_statement::<P, F>(
                    self.pairing,
                    AlgebraicMintStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let proof = adapter
                    .prove_mint(
                        ProveMintParams,
                        build_prove_mint_payload::<P, F>(
                            self.pairing,
                            &statement,
                            Default::default(),
                        ),
                    )
                    .expect("the statement's witnesses prove")
                    .proof;

                // Act
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Mint(&statement),
                        &proof,
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
            }
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            MintVerifyConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                MintVerifyConsumer {
                                                    pairing: self.pairing,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_mint_proof_the_adapter_returns_verifies
}

/// Contract: entry `a_transfer_proof_the_adapter_returns_verifies`;
///   `prove_transfer` over the witnesses the statement holds followed by
///   `verify` over the same statement on the adapter `create_delivery_proof`
///   hands the consumer returns `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: the cases of the declaration block; for each of
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant` the statement
///   `build_algebraic_transfer_statement` builds over a context with that
///   purpose, and `build_prove_transfer_payload` over it.
/// Act:     `prove_transfer`, then `verify`, inside `TransferVerifyConsumer`.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)`; every factory result `is_ok()`.
/// Boundary: as the mint block with `prove_transfer`.
/// Mocked:  nothing.
#[test]
fn a_transfer_proof_the_adapter_returns_verifies() {
    struct TransferVerifyConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for TransferVerifyConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                // Arrange
                let statement = build_algebraic_transfer_statement::<P, F>(
                    self.pairing,
                    AlgebraicTransferStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let proof = adapter
                    .prove_transfer(
                        ProveTransferParams,
                        build_prove_transfer_payload::<P, F>(
                            self.pairing,
                            &statement,
                            Default::default(),
                        ),
                    )
                    .expect("the statement's witnesses prove")
                    .proof;

                // Act
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Transfer(&statement),
                        &proof,
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
            }
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            TransferVerifyConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                TransferVerifyConsumer {
                                                    pairing: self.pairing,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_transfer_proof_the_adapter_returns_verifies
}

/// Contract: entry
///   `a_proof_the_adapter_rebuilds_from_its_components_verifies`; a proof
///   either prover returned, read by `proof_components` and rebuilt by
///   `proof_from_components` on the same adapter, verifies.
/// Arrange: the cases of the declaration block; for the mint relation at
///   each `MintPurpose` and the transfer relation at each
///   `TransferPurpose`, the proof the relation's prover returns read by
///   `proof_components`.
/// Act:     `proof_components`, `proof_from_components`, then `verify`,
///   inside `RebuildConsumer`.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)` for each relation; every factory result `is_ok()`.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the
///   provers, `proof_components`, `proof_from_components`, and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_the_adapter_rebuilds_from_its_components_verifies() {
    struct RebuildConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for RebuildConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                // Arrange
                let statement = build_algebraic_mint_statement::<P, F>(
                    self.pairing,
                    AlgebraicMintStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let proof = adapter
                    .prove_mint(
                        ProveMintParams,
                        build_prove_mint_payload::<P, F>(
                            self.pairing,
                            &statement,
                            Default::default(),
                        ),
                    )
                    .expect("the statement's witnesses prove")
                    .proof;

                // Act
                let components = adapter
                    .proof_components(
                        ProofComponentsParams,
                        build_proof_components_payload(&proof, Default::default()),
                    )
                    .expect("a proof reads its components")
                    .components;
                let rebuilt = adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        build_proof_from_components_payload(components, Default::default()),
                    )
                    .expect("an adapter of the same form rebuilds")
                    .proof;
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Mint(&statement),
                        &rebuilt,
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
            }
            for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                // Arrange
                let statement = build_algebraic_transfer_statement::<P, F>(
                    self.pairing,
                    AlgebraicTransferStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let proof = adapter
                    .prove_transfer(
                        ProveTransferParams,
                        build_prove_transfer_payload::<P, F>(
                            self.pairing,
                            &statement,
                            Default::default(),
                        ),
                    )
                    .expect("the statement's witnesses prove")
                    .proof;

                // Act
                let components = adapter
                    .proof_components(
                        ProofComponentsParams,
                        build_proof_components_payload(&proof, Default::default()),
                    )
                    .expect("a proof reads its components")
                    .components;
                let rebuilt = adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        build_proof_from_components_payload(components, Default::default()),
                    )
                    .expect("an adapter of the same form rebuilds")
                    .proof;
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Transfer(&statement),
                        &rebuilt,
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
            }
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            RebuildConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                RebuildConsumer {
                                                    pairing: self.pairing,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_proof_the_adapter_rebuilds_from_its_components_verifies
}

/// Contract: entry
///   `components_of_the_other_verifier_form_are_refused_by_the_adapter`;
///   `proof_from_components` over components of the verifier form other
///   than the one the adapter was constructed in returns
///   `Err(ProofFromComponentsErrorReturn::VerifierFormMismatch)`.
/// Arrange: the cases of the declaration block; for each relation the
///   components `DeliveryProofComponents::try_from` returns for
///   `build_delivery_proof_wire_components` overridden to the other
///   verifier form, with the response and first-message counts that form
///   requires.
/// Act:     `proof_from_components` over the components, inside
///   `OtherFormConsumer`.
/// Assert:  in place, `result.err()` equals
///   `Some(ProofFromComponentsErrorReturn::VerifierFormMismatch)` for each
///   relation; every factory result `is_ok()`.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, and
///   `proof_from_components`.
/// Mocked:  nothing.
#[test]
fn components_of_the_other_verifier_form_are_refused_by_the_adapter() {
    struct OtherFormConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        verifier_form: VerifierGroupArithmetic,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for OtherFormConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            let other = match self.verifier_form {
                VerifierGroupArithmetic::BothGroups => VerifierGroupArithmetic::FirstGroupOnly,
                VerifierGroupArithmetic::FirstGroupOnly => VerifierGroupArithmetic::BothGroups,
            };
            for relation in [DeliveryRelation::Mint, DeliveryRelation::Transfer] {
                // Arrange
                let responses = match relation {
                    DeliveryRelation::Mint => {
                        let responses = build_mint_responses(self.pairing, Default::default());
                        vec![responses.alpha, responses.r, responses.rho, responses.sigma]
                    }
                    DeliveryRelation::Transfer => {
                        let responses = build_transfer_responses(self.pairing, Default::default());
                        vec![
                            responses.x,
                            responses.y,
                            responses.s,
                            responses.rho,
                            responses.sigma,
                        ]
                    }
                };
                let second_group_first_messages = match (relation, other) {
                    (_, VerifierGroupArithmetic::BothGroups) => Vec::new(),
                    (DeliveryRelation::Mint, VerifierGroupArithmetic::FirstGroupOnly) => {
                        let first_messages = build_mint_second_group_first_messages(
                            self.pairing,
                            Default::default(),
                        );
                        vec![first_messages.hpub, first_messages.d1, first_messages.d2]
                    }
                    (DeliveryRelation::Transfer, VerifierGroupArithmetic::FirstGroupOnly) => {
                        let first_messages = build_transfer_second_group_first_messages(
                            self.pairing,
                            Default::default(),
                        );
                        vec![first_messages.pk2, first_messages.d1, first_messages.d2]
                    }
                };
                let components =
                    DeliveryProofComponents::try_from(build_delivery_proof_wire_components(
                        self.pairing,
                        DeliveryProofWireComponentsOverrides {
                            relation: Some(relation),
                            verifier_form: Some(other),
                            responses: Some(responses),
                            second_group_first_messages: Some(second_group_first_messages),
                            ..Default::default()
                        },
                    ))
                    .expect("the wire holds the other form's counts");

                // Act
                let result = adapter.proof_from_components(
                    ProofFromComponentsParams,
                    build_proof_from_components_payload(components, Default::default()),
                );

                // Assert
                assert_eq!(
                    result.err(),
                    Some(ProofFromComponentsErrorReturn::VerifierFormMismatch)
                );
            }
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            OtherFormConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                OtherFormConsumer {
                                                    pairing: self.pairing,
                                                    verifier_form,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // components_of_the_other_verifier_form_are_refused_by_the_adapter
}

/// Contract: entry `a_proof_of_another_relation_is_refused_by_verify`;
///   `verify` over a mint statement and a proof `prove_transfer` returned,
///   and over a transfer statement and a proof `prove_mint` returned,
///   each return `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases of the declaration block; a transfer proof from
///   `prove_transfer` and a mint statement, and a mint proof from
///   `prove_mint` and a transfer statement, per purpose pair.
/// Act:     `verify` over each statement and the other relation's proof,
///   inside `OtherRelationConsumer`.
/// Assert:  in place, each result equals `Ok` with `is_valid` false; every
///   factory result `is_ok()`.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the
///   provers, and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_of_another_relation_is_refused_by_verify() {
    struct OtherRelationConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for OtherRelationConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            for mint_purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                for transfer_purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                    // Arrange
                    let mint_statement = build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(mint_purpose, Default::default())),
                            ..Default::default()
                        },
                    );
                    let transfer_statement = build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                transfer_purpose,
                                Default::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let mint_proof = adapter
                        .prove_mint(
                            ProveMintParams,
                            build_prove_mint_payload::<P, F>(
                                self.pairing,
                                &mint_statement,
                                Default::default(),
                            ),
                        )
                        .expect("the statement's witnesses prove")
                        .proof;
                    let transfer_proof = adapter
                        .prove_transfer(
                            ProveTransferParams,
                            build_prove_transfer_payload::<P, F>(
                                self.pairing,
                                &transfer_statement,
                                Default::default(),
                            ),
                        )
                        .expect("the statement's witnesses prove")
                        .proof;

                    // Act
                    let mint_result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Mint(&mint_statement),
                            &transfer_proof,
                            Default::default(),
                        ),
                    );
                    let transfer_result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Transfer(&transfer_statement),
                            &mint_proof,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(
                        mint_result.ok().map(|success| success.is_valid),
                        Some(false)
                    );
                    assert_eq!(
                        transfer_result.ok().map(|success| success.is_valid),
                        Some(false)
                    );
                }
            }
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            OtherRelationConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                OtherRelationConsumer {
                                                    pairing: self.pairing,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_proof_of_another_relation_is_refused_by_verify
}

/// Contract: entry
///   `a_proof_against_a_statement_differing_in_one_field_is_refused`;
///   `verify` over a proof formed for the baseline statement and a
///   statement differing in one field returns
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases of the declaration block; the proof formed for the
///   baseline statement; for each field of `AlgebraicMintStatement` and of
///   `AlgebraicTransferStatement`, the context fields included, a variant
///   statement overridden in that field alone with a value from that
///   field's own builder distinct from the default.
/// Act:     `verify` over the variant statement and the baseline proof,
///   inside `DifferingFieldConsumer`.
/// Assert:  in place, the baseline maps to `Some(true)` and each variant
///   maps to `Some(false)`; every factory result `is_ok()`.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the
///   provers, and `verify`, run for each variant.
/// Mocked:  nothing.
#[test]
fn a_proof_against_a_statement_differing_in_one_field_is_refused() {
    struct DifferingFieldConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for DifferingFieldConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            fn variant_form<T: ICanonicalField>(seed: u8) -> T {
                T::from_field(
                    build_from_field_params(FromFieldParamsOverrides),
                    match T::KIND {
                        CanonicalFieldKind::FixedBytes20 => {
                            CanonicalFieldValue::FixedBytes20([seed; 20])
                        }
                        CanonicalFieldKind::FixedBytes32 => {
                            CanonicalFieldValue::FixedBytes32([seed; 32])
                        }
                        CanonicalFieldKind::Unsigned16 => {
                            CanonicalFieldValue::Unsigned16(u16::from(seed))
                        }
                        CanonicalFieldKind::Unsigned32 => {
                            CanonicalFieldValue::Unsigned32(u32::from(seed))
                        }
                        CanonicalFieldKind::Unsigned64 => {
                            CanonicalFieldValue::Unsigned64(u64::from(seed))
                        }
                        CanonicalFieldKind::Unsigned256 => {
                            CanonicalFieldValue::Unsigned256([seed; 32])
                        }
                        CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(vec![seed]),
                        CanonicalFieldKind::Text => CanonicalFieldValue::Text(seed.to_string()),
                    },
                )
                .expect("a seeded field of the declared kind is admitted")
                .value
            }

            let adapter = payload.adapter;
            let g1 = self
                .pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the g1 generator is fixed")
                .point;
            let g2 = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the g2 generator is fixed")
                .point;
            let distinct = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(
                        vec![0xC1; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                        SecretConstructorParamsOverrides,
                    ),
                },
            )
            .expect("a full-length uniform samples")
            .scalar
            .expose()
            .clone();
            for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                // Arrange
                let baseline = build_algebraic_mint_statement::<P, F>(
                    self.pairing,
                    AlgebraicMintStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let proof = adapter
                    .prove_mint(
                        ProveMintParams,
                        build_prove_mint_payload::<P, F>(
                            self.pairing,
                            &baseline,
                            Default::default(),
                        ),
                    )
                    .expect("the baseline statement's witnesses prove")
                    .proof;
                let baseline_result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Mint(&baseline),
                        &proof,
                        Default::default(),
                    ),
                );
                assert_eq!(
                    baseline_result.ok().map(|success| success.is_valid),
                    Some(true)
                );
                let other = match purpose {
                    MintPurpose::Mint => MintPurpose::Replacement,
                    MintPurpose::Replacement => MintPurpose::Mint,
                };
                let variants = [
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    suite_identifier: Some(build_suite_identifier(
                                        SuiteIdentifierConstructorParamsOverrides {
                                            identifier: Some([0xB3; SUITE_IDENTIFIER_LENGTH]),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    chain: Some(variant_form::<F::ChainIdentifier>(0xB1)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    entitlement_contract: Some(variant_form::<F::Identity>(0xB2)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    asset_identity_hash: Some(build_asset_identity_hash(
                                        AssetIdentityHashConstructorParamsOverrides {
                                            bytes: Some([0xB4; ASSET_IDENTITY_HASH_LENGTH]),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    parameter_set_identifier: Some(build_parameter_set_identifier(
                                        ParameterSetIdentifierConstructorParamsOverrides {
                                            bytes: Some([0xB5; PARAMETER_SET_IDENTIFIER_LENGTH]),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    source_entitlement: Some(variant_form::<F::Entitlement>(0xB6)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    target_entitlement: Some(variant_form::<F::Entitlement>(0xB7)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    old_interval: Some(variant_form::<F::Interval>(0xB8)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    new_interval: Some(variant_form::<F::Interval>(0xB9)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(other, Default::default())),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    seller: Some(variant_form::<F::Identity>(0xBA)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    buyer: Some(variant_form::<F::Identity>(0xBB)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    expiry: Some(1_700_000_001),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            hpub: Some(
                                self.pairing
                                    .mul_g2(
                                        MulG2Params,
                                        MulG2Payload {
                                            point: g2.clone(),
                                            scalar: distinct.clone(),
                                        },
                                    )
                                    .expect("a variant point multiplies")
                                    .product,
                            ),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            identity_element: Some(
                                self.pairing
                                    .mul_g1(
                                        MulG1Params,
                                        MulG1Payload {
                                            point: g1.clone(),
                                            scalar: distinct.clone(),
                                        },
                                    )
                                    .expect("a variant point multiplies")
                                    .product,
                            ),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            buyer_keys: Some(build_public_keys_components(
                                self.pairing,
                                PublicKeysComponentsOverrides {
                                    pk1: Some(
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: distinct.clone(),
                                                },
                                            )
                                            .expect("a variant point multiplies")
                                            .product,
                                    ),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement::<P, F>(
                        self.pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            envelope: Some(build_envelope_components(
                                self.pairing,
                                EnvelopeComponentsOverrides {
                                    c1: Some(
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: distinct.clone(),
                                                },
                                            )
                                            .expect("a variant point multiplies")
                                            .product,
                                    ),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                ];

                // Act
                for variant in &variants {
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Mint(variant),
                            &proof,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }
            }
            for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                // Arrange
                let baseline = build_algebraic_transfer_statement::<P, F>(
                    self.pairing,
                    AlgebraicTransferStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let proof = adapter
                    .prove_transfer(
                        ProveTransferParams,
                        build_prove_transfer_payload::<P, F>(
                            self.pairing,
                            &baseline,
                            Default::default(),
                        ),
                    )
                    .expect("the baseline statement's witnesses prove")
                    .proof;
                let baseline_result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Transfer(&baseline),
                        &proof,
                        Default::default(),
                    ),
                );
                assert_eq!(
                    baseline_result.ok().map(|success| success.is_valid),
                    Some(true)
                );
                let other = match purpose {
                    TransferPurpose::Transfer => TransferPurpose::Grant,
                    TransferPurpose::Grant => TransferPurpose::Transfer,
                };
                let variants = [
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    suite_identifier: Some(build_suite_identifier(
                                        SuiteIdentifierConstructorParamsOverrides {
                                            identifier: Some([0xB3; SUITE_IDENTIFIER_LENGTH]),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    chain: Some(variant_form::<F::ChainIdentifier>(0xB1)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    entitlement_contract: Some(variant_form::<F::Identity>(0xB2)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    asset_identity_hash: Some(build_asset_identity_hash(
                                        AssetIdentityHashConstructorParamsOverrides {
                                            bytes: Some([0xB4; ASSET_IDENTITY_HASH_LENGTH]),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    parameter_set_identifier: Some(build_parameter_set_identifier(
                                        ParameterSetIdentifierConstructorParamsOverrides {
                                            bytes: Some([0xB5; PARAMETER_SET_IDENTIFIER_LENGTH]),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    source_entitlement: Some(variant_form::<F::Entitlement>(0xB6)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    target_entitlement: Some(variant_form::<F::Entitlement>(0xB7)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    old_interval: Some(variant_form::<F::Interval>(0xB8)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    new_interval: Some(variant_form::<F::Interval>(0xB9)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(other, Default::default())),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    seller: Some(variant_form::<F::Identity>(0xBA)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    buyer: Some(variant_form::<F::Identity>(0xBB)),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides {
                                    expiry: Some(1_700_000_001),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            identity_element: Some(
                                self.pairing
                                    .mul_g1(
                                        MulG1Params,
                                        MulG1Payload {
                                            point: g1.clone(),
                                            scalar: distinct.clone(),
                                        },
                                    )
                                    .expect("a variant point multiplies")
                                    .product,
                            ),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            seller_keys: Some(build_public_keys_components(
                                self.pairing,
                                PublicKeysComponentsOverrides {
                                    pk1: Some(
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: distinct.clone(),
                                                },
                                            )
                                            .expect("a variant point multiplies")
                                            .product,
                                    ),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            buyer_keys: Some(build_public_keys_components(
                                self.pairing,
                                PublicKeysComponentsOverrides {
                                    pk1: Some(
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: distinct.clone(),
                                                },
                                            )
                                            .expect("a variant point multiplies")
                                            .product,
                                    ),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            old_envelope: Some(build_envelope_components(
                                self.pairing,
                                EnvelopeComponentsOverrides {
                                    c1: Some(
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: distinct.clone(),
                                                },
                                            )
                                            .expect("a variant point multiplies")
                                            .product,
                                    ),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement::<P, F>(
                        self.pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(purpose, Default::default())),
                            new_envelope: Some(build_envelope_components(
                                self.pairing,
                                EnvelopeComponentsOverrides {
                                    c1: Some(
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: distinct.clone(),
                                                },
                                            )
                                            .expect("a variant point multiplies")
                                            .product,
                                    ),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                ];

                // Act
                for variant in &variants {
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Transfer(variant),
                            &proof,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }
            }
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            DifferingFieldConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                DifferingFieldConsumer {
                                                    pairing: self.pairing,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_proof_against_a_statement_differing_in_one_field_is_refused
}

/// Contract: entry
///   `a_proof_with_a_changed_response_or_challenge_is_refused`; `verify`
///   over a proof whose challenge or one response is replaced by another
///   scalar, the wire carrying the replacement converted and rebuilt by
///   `proof_from_components`, returns
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases of the declaration block; the mint and transfer
///   proofs over the default statements at the default purposes; the wire
///   from `build_delivery_proof_wire_components` overridden with the
///   relation, the verifier form, the challenge, the responses, and the
///   second-group first messages of the proof's `proof_components`, with
///   the challenge or one response replaced alone by a scalar of
///   `build_mint_responses` or `build_transfer_responses` that differs
///   from the proof's value, converted by
///   `DeliveryProofComponents::try_from` and rebuilt by
///   `proof_from_components`.
/// Act:     `verify` over the statement and the rebuilt proof, inside
///   `ChangedScalarConsumer`.
/// Assert:  in place, the unmodified rebuilt proof maps to `Some(true)`
///   and each variant maps to `Some(false)`; every factory result
///   `is_ok()`.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the
///   provers, `proof_components`, `proof_from_components`, and `verify`,
///   run for each variant.
/// Mocked:  nothing.
#[test]
fn a_proof_with_a_changed_response_or_challenge_is_refused() {
    struct ChangedScalarConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for ChangedScalarConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            let statement =
                build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let proof = adapter
                .prove_mint(
                    ProveMintParams,
                    build_prove_mint_payload::<P, F>(self.pairing, &statement, Default::default()),
                )
                .expect("the statement's witnesses prove")
                .proof;
            let components = adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proof, Default::default()),
                )
                .expect("a proof reads its components")
                .components;
            let (verifier_form, challenge, responses, first_messages) = match components {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                } => (
                    VerifierGroupArithmetic::BothGroups,
                    challenge,
                    vec![responses.alpha, responses.r, responses.rho, responses.sigma],
                    Vec::new(),
                ),
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => (
                    VerifierGroupArithmetic::FirstGroupOnly,
                    challenge,
                    vec![responses.alpha, responses.r, responses.rho, responses.sigma],
                    vec![first_messages.hpub, first_messages.d1, first_messages.d2],
                ),
                _ => panic!("the mint prover returns mint components"),
            };
            let foreign = build_mint_responses(self.pairing, Default::default());
            let variants = [
                (Some(foreign.alpha.clone()), None),
                (None, Some((0usize, foreign.alpha.clone()))),
                (None, Some((1usize, foreign.r.clone()))),
                (None, Some((2usize, foreign.rho.clone()))),
                (None, Some((3usize, foreign.sigma.clone()))),
            ];
            for (challenge_override, response_override) in variants {
                // Arrange
                let mut variant_responses = responses.clone();
                if let Some((index, scalar)) = response_override {
                    variant_responses[index] = scalar;
                }
                let rebuilt = adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        build_proof_from_components_payload(
                            DeliveryProofComponents::try_from(
                                build_delivery_proof_wire_components(
                                    self.pairing,
                                    DeliveryProofWireComponentsOverrides {
                                        relation: Some(DeliveryRelation::Mint),
                                        verifier_form: Some(verifier_form),
                                        challenge: Some(
                                            challenge_override.unwrap_or_else(|| challenge.clone()),
                                        ),
                                        responses: Some(variant_responses),
                                        second_group_first_messages: Some(first_messages.clone()),
                                        ..Default::default()
                                    },
                                ),
                            )
                            .expect("the wire holds this form's counts"),
                            Default::default(),
                        ),
                    )
                    .expect("an adapter of the same form rebuilds")
                    .proof;

                // Act
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Mint(&statement),
                        &rebuilt,
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
            }
            let control = adapter
                .proof_from_components(
                    ProofFromComponentsParams,
                    build_proof_from_components_payload(
                        DeliveryProofComponents::try_from(build_delivery_proof_wire_components(
                            self.pairing,
                            DeliveryProofWireComponentsOverrides {
                                relation: Some(DeliveryRelation::Mint),
                                verifier_form: Some(verifier_form),
                                challenge: Some(challenge.clone()),
                                responses: Some(responses.clone()),
                                second_group_first_messages: Some(first_messages.clone()),
                                ..Default::default()
                            },
                        ))
                        .expect("the wire holds this form's counts"),
                        Default::default(),
                    ),
                )
                .expect("an adapter of the same form rebuilds")
                .proof;
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &control,
                    Default::default(),
                ),
            );
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));

            let statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let proof = adapter
                .prove_transfer(
                    ProveTransferParams,
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        Default::default(),
                    ),
                )
                .expect("the statement's witnesses prove")
                .proof;
            let components = adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proof, Default::default()),
                )
                .expect("a proof reads its components")
                .components;
            let (verifier_form, challenge, responses, first_messages) = match components {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                } => (
                    VerifierGroupArithmetic::BothGroups,
                    challenge,
                    vec![
                        responses.x,
                        responses.y,
                        responses.s,
                        responses.rho,
                        responses.sigma,
                    ],
                    Vec::new(),
                ),
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => (
                    VerifierGroupArithmetic::FirstGroupOnly,
                    challenge,
                    vec![
                        responses.x,
                        responses.y,
                        responses.s,
                        responses.rho,
                        responses.sigma,
                    ],
                    vec![first_messages.pk2, first_messages.d1, first_messages.d2],
                ),
                _ => panic!("the transfer prover returns transfer components"),
            };
            let foreign = build_transfer_responses(self.pairing, Default::default());
            let variants = [
                (Some(foreign.x.clone()), None),
                (None, Some((0usize, foreign.x.clone()))),
                (None, Some((1usize, foreign.y.clone()))),
                (None, Some((2usize, foreign.s.clone()))),
                (None, Some((3usize, foreign.rho.clone()))),
                (None, Some((4usize, foreign.sigma.clone()))),
            ];
            for (challenge_override, response_override) in variants {
                // Arrange
                let mut variant_responses = responses.clone();
                if let Some((index, scalar)) = response_override {
                    variant_responses[index] = scalar;
                }
                let rebuilt = adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        build_proof_from_components_payload(
                            DeliveryProofComponents::try_from(
                                build_delivery_proof_wire_components(
                                    self.pairing,
                                    DeliveryProofWireComponentsOverrides {
                                        relation: Some(DeliveryRelation::Transfer),
                                        verifier_form: Some(verifier_form),
                                        challenge: Some(
                                            challenge_override.unwrap_or_else(|| challenge.clone()),
                                        ),
                                        responses: Some(variant_responses),
                                        second_group_first_messages: Some(first_messages.clone()),
                                        ..Default::default()
                                    },
                                ),
                            )
                            .expect("the wire holds this form's counts"),
                            Default::default(),
                        ),
                    )
                    .expect("an adapter of the same form rebuilds")
                    .proof;

                // Act
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Transfer(&statement),
                        &rebuilt,
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
            }
            let control = adapter
                .proof_from_components(
                    ProofFromComponentsParams,
                    build_proof_from_components_payload(
                        DeliveryProofComponents::try_from(build_delivery_proof_wire_components(
                            self.pairing,
                            DeliveryProofWireComponentsOverrides {
                                relation: Some(DeliveryRelation::Transfer),
                                verifier_form: Some(verifier_form),
                                challenge: Some(challenge.clone()),
                                responses: Some(responses.clone()),
                                second_group_first_messages: Some(first_messages.clone()),
                                ..Default::default()
                            },
                        ))
                        .expect("the wire holds this form's counts"),
                        Default::default(),
                    ),
                )
                .expect("an adapter of the same form rebuilds")
                .proof;
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &control,
                    Default::default(),
                ),
            );
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            ChangedScalarConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                ChangedScalarConsumer {
                                                    pairing: self.pairing,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_proof_with_a_changed_response_or_challenge_is_refused
}

/// Contract: entry
///   `a_proof_from_a_witness_the_statement_does_not_hold_is_refused`;
///   `prove_mint` or `prove_transfer` over a witness other than the one
///   the statement holds returns `Ok`, and `verify` over the statement
///   and that proof returns `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases of the declaration block; for each of the four mint
///   witnesses and each of the five transfer witnesses, the statement from
///   its builder and a payload overridden in that witness alone with a
///   value from the witness's builder distinct from the default.
/// Act:     the prover over the payload, then `verify` over the
///   statement and the proof, inside `WrongWitnessConsumer`.
/// Assert:  in place, the prover returns `Ok` and `verify`'s result
///   equals `Ok` with `is_valid` false for each witness; every factory
///   result `is_ok()`.
/// Boundary: `create_delivery_proof`, the concrete's `try_new`, the
///   provers, and `verify`.
/// Mocked:  nothing.
#[test]
fn a_proof_from_a_witness_the_statement_does_not_hold_is_refused() {
    struct WrongWitnessConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for WrongWitnessConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                // Arrange
                let statement = build_algebraic_mint_statement::<P, F>(
                    self.pairing,
                    AlgebraicMintStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let foreign = build_mint_responses(self.pairing, Default::default());
                let payloads = [
                    build_prove_mint_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveMintPayloadOverrides {
                            master_scalar: Some(build_master_scalar_components(
                                MasterScalarComponentsOverrides {
                                    value: Some(build_secret(
                                        foreign.alpha.clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_prove_mint_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveMintPayloadOverrides {
                            credential_randomness: Some(build_secret(
                                foreign.r.clone(),
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    ),
                    build_prove_mint_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveMintPayloadOverrides {
                            coins: Some(build_envelope_coins(
                                self.pairing,
                                EnvelopeCoinsOverrides {
                                    rho: Some(build_secret(
                                        foreign.rho.clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_prove_mint_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveMintPayloadOverrides {
                            coins: Some(build_envelope_coins(
                                self.pairing,
                                EnvelopeCoinsOverrides {
                                    sigma: Some(build_secret(
                                        foreign.sigma.clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                ];
                for payload in payloads {
                    // Act
                    let proof = adapter
                        .prove_mint(ProveMintParams, payload)
                        .expect("the prover does not check the statement holds")
                        .proof;
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Mint(&statement),
                            &proof,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }
            }
            for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                // Arrange
                let statement = build_algebraic_transfer_statement::<P, F>(
                    self.pairing,
                    AlgebraicTransferStatementOverrides {
                        context: Some(build_delivery_context(purpose, Default::default())),
                        ..Default::default()
                    },
                );
                let foreign = build_transfer_responses(self.pairing, Default::default());
                let payloads = [
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveTransferPayloadOverrides {
                            seller_secrets: Some(build_key_pair_components(
                                self.pairing,
                                KeyPairComponentsOverrides {
                                    x: Some(build_secret(
                                        foreign.x.clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveTransferPayloadOverrides {
                            seller_secrets: Some(build_key_pair_components(
                                self.pairing,
                                KeyPairComponentsOverrides {
                                    y: Some(build_secret(
                                        foreign.y.clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveTransferPayloadOverrides {
                            offset: Some(build_secret(
                                foreign.s.clone(),
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                    ),
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveTransferPayloadOverrides {
                            coins: Some(build_envelope_coins(
                                self.pairing,
                                EnvelopeCoinsOverrides {
                                    rho: Some(build_secret(
                                        foreign.rho.clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_prove_transfer_payload::<P, F>(
                        self.pairing,
                        &statement,
                        ProveTransferPayloadOverrides {
                            coins: Some(build_envelope_coins(
                                self.pairing,
                                EnvelopeCoinsOverrides {
                                    sigma: Some(build_secret(
                                        foreign.sigma.clone(),
                                        SecretConstructorParamsOverrides,
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                ];
                for payload in payloads {
                    // Act
                    let proof = adapter
                        .prove_transfer(ProveTransferParams, payload)
                        .expect("the prover does not check the statement holds")
                        .proof;
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Transfer(&statement),
                            &proof,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }
            }
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let random = create_random_source(
                                    &CreateRandomSourceDeps,
                                    build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(RandomSourceKind::OperatingSystem),
                                        },
                                    ),
                                    CreateRandomSourcePayload,
                                )
                                .expect("the operating-system source constructs");
                                for concrete in DELIVERY_PROOF_CONCRETES.iter().copied().chain([
                                    DeliveryProofConcrete::Mock(
                                        MockIDeliveryProofAdapterFailureMode::NoFailure,
                                    ),
                                ]) {
                                    for verifier_form in [
                                        VerifierGroupArithmetic::BothGroups,
                                        VerifierGroupArithmetic::FirstGroupOnly,
                                    ] {
                                        let created = create_delivery_proof::<
                                            P,
                                            E,
                                            F,
                                            WrongWitnessConsumer<'_, P>,
                                        >(
                                            &build_create_delivery_proof_deps(
                                                self.pairing,
                                                &*hashed.adapter,
                                                self.encoder,
                                                &*random.adapter,
                                                WrongWitnessConsumer {
                                                    pairing: self.pairing,
                                                },
                                                Default::default(),
                                            ),
                                            build_create_delivery_proof_params(
                                                CreateDeliveryProofParamsOverrides {
                                                    concrete: Some(concrete),
                                                    algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                    verifier_form: Some(verifier_form),
                                                    statement_version: Some(
                                                        DELIVERY_STATEMENT_VERSION_ONE,
                                                    ),
                                                },
                                            ),
                                            build_create_delivery_proof_payload(Default::default()),
                                        );
                                        assert!(created.is_ok());
                                    }
                                }
                            }
                        }
                    }

                    for concrete in CHAIN_FORMS_CONCRETES {
                        let created = create_chain_forms(
                            &build_create_chain_forms_deps(
                                ChainConsumer {
                                    pairing: self.pairing,
                                    encoder: &encoded.adapter,
                                },
                                Default::default(),
                            ),
                            build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                                concrete: Some(*concrete),
                                ..Default::default()
                            }),
                            build_create_chain_forms_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_proof_from_a_witness_the_statement_does_not_hold_is_refused
}

/// Contract: entry `a_refused_nonce_draw_is_returned_by_each_prover`;
///   `prove_mint` and `prove_transfer` on the adapter
///   `create_delivery_proof` hands the consumer for the `Mock` selection
///   under `NonceDrawRefused` each return the mock concrete's own
///   `NonceDrawRefused` error.
/// Arrange: the `Mock` selection under
///   `MockIDeliveryProofAdapterFailureMode::NonceDrawRefused` over every
///   member of `PAIRING_CONCRETES`, the first declared selections of the
///   encoding, chain-forms, and `keccak` hash-to-scalar sets, the
///   operating-system randomness source, and the default statements and
///   payloads.
/// Act:     `prove_mint`, then `prove_transfer`, inside
///   `NonceRefusalConsumer`.
/// Assert:  in place, `assert_eq!` of the mint error against
///   `ProveMintErrorReturn::Mock(MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)`
///   and of the transfer error against
///   `ProveTransferErrorReturn::Mock(MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused)`;
///   every factory result `is_ok()`.
/// Boundary: `create_delivery_proof`, the mock concrete's `try_new`,
///   `prove_mint`, and `prove_transfer`.
/// Mocked:  nothing; the block runs over the mock concrete alone, since
///   the declared set cannot reach the failure mode.
#[test]
fn a_refused_nonce_draw_is_returned_by_each_prover() {
    struct NonceRefusalConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic, F: IChainForms> IDeliveryProofConsumer<P, F>
        for NonceRefusalConsumer<'_, P>
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let adapter = payload.adapter;
            let mint_statement =
                build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let transfer_statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());

            // Act
            let mint_result = adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload::<P, F>(self.pairing, &mint_statement, Default::default()),
            );
            let transfer_result = adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload::<P, F>(
                    self.pairing,
                    &transfer_statement,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                mint_result.err(),
                Some(ProveMintErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                ))
            );
            assert_eq!(
                transfer_result.err(),
                Some(ProveTransferErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused
                ))
            );
        }
    }

    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    encoded: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct ChainConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for ChainConsumer<'_, P, E> {
                        type Output = ();

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            let case = DECLARED_HASH_TO_SCALAR_SELECTIONS
                                .iter()
                                .find(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                                .expect("the declared set names the keccak selection");
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(case.concrete),
                                        identifier: Some(case.identifier),
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect("the declared selection admits the concrete it names");
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(
                                    CreateRandomSourceParamsOverrides {
                                        kind: Some(RandomSourceKind::OperatingSystem),
                                    },
                                ),
                                CreateRandomSourcePayload,
                            )
                            .expect("the operating-system source constructs");

                            // Act
                            let created = create_delivery_proof::<
                                P,
                                E,
                                F,
                                NonceRefusalConsumer<'_, P>,
                            >(
                                &build_create_delivery_proof_deps(
                                    self.pairing,
                                    &*hashed.adapter,
                                    self.encoder,
                                    &*random.adapter,
                                    NonceRefusalConsumer {
                                        pairing: self.pairing,
                                    },
                                    Default::default(),
                                ),
                                build_create_delivery_proof_params(
                                    CreateDeliveryProofParamsOverrides {
                                        concrete: Some(DeliveryProofConcrete::Mock(
                                            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                                        )),
                                        ..Default::default()
                                    },
                                ),
                                build_create_delivery_proof_payload(Default::default()),
                            );

                            // Assert
                            assert!(created.is_ok());
                        }
                    }

                    let concrete = CHAIN_FORMS_CONCRETES
                        .first()
                        .expect("the declared chain-forms set is nonempty");
                    let created = create_chain_forms(
                        &build_create_chain_forms_deps(
                            ChainConsumer {
                                pairing: self.pairing,
                                encoder: &encoded.adapter,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        }),
                        build_create_chain_forms_payload(Default::default()),
                    );
                    assert!(created.is_ok());
                }
            }

            let selection = DECLARED_ENCODING_SELECTIONS
                .first()
                .expect("the declared encoding set is nonempty");
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                }),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(selection.concrete),
                    identifier: Some(selection.identifier),
                }),
                build_create_encoding_payload(Default::default()),
            );
            assert!(created.is_ok());
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // a_refused_nonce_draw_is_returned_by_each_prover
}

// Enumeration: each own entry and the block of the same name that proves it:
//   an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version
//   a_mint_proof_the_adapter_returns_verifies
//   a_transfer_proof_the_adapter_returns_verifies
//   a_proof_the_adapter_rebuilds_from_its_components_verifies
//   components_of_the_other_verifier_form_are_refused_by_the_adapter
//   a_proof_of_another_relation_is_refused_by_verify
//   a_proof_against_a_statement_differing_in_one_field_is_refused
//   a_proof_with_a_changed_response_or_challenge_is_refused
//   a_proof_from_a_witness_the_statement_does_not_hold_is_refused
//   a_refused_nonce_draw_is_returned_by_each_prover
// and each absorbed entry of `proof/schnorr_fs` and the block the spec cites:
//   a_mint_proof_verifies_in_both_forms
//     -> a_mint_proof_the_adapter_returns_verifies
//   a_transfer_proof_verifies_in_both_forms
//     -> a_transfer_proof_the_adapter_returns_verifies
//   a_proof_rebuilt_from_its_components_verifies
//     -> a_proof_the_adapter_rebuilds_from_its_components_verifies
//   components_of_the_other_verifier_form_are_refused
//     -> components_of_the_other_verifier_form_are_refused_by_the_adapter
//   a_proof_of_another_relation_fails_verification
//     -> a_proof_of_another_relation_is_refused_by_verify
//   a_proof_fails_against_a_statement_differing_in_one_field
//     -> a_proof_against_a_statement_differing_in_one_field_is_refused
//   a_proof_with_a_changed_response_or_challenge_fails
//     -> a_proof_with_a_changed_response_or_challenge_is_refused
//   a_proof_from_a_witness_the_statement_does_not_hold_fails
//     -> a_proof_from_a_witness_the_statement_does_not_hold_is_refused
