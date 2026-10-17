#![allow(clippy::expect_used)]

use super::challenge::provides::SCHNORR_FS_CHALLENGE_TAG;
use super::provides::{
    SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParamsOverrides,
    SchnorrFsMintProofOverrides, SchnorrFsProof, SchnorrFsTransferProofOverrides,
    build_schnorr_fs_delivery_proof, build_schnorr_fs_mint_proof, build_schnorr_fs_transfer_proof,
};
use crate::factory::provides::{
    AlgebraicMintStatementOverrides, AlgebraicStatement, AlgebraicTransferStatementOverrides,
    DeliveryContextOverrides, IDeliveryProofAdapter, MintResponses, MintSecondGroupFirstMessages,
    ProofComponentsParams, ProofFromComponentsParams, ProveMintParams, ProveMintPayloadOverrides,
    ProveTransferParams, ProveTransferPayloadOverrides, TransferResponses,
    TransferSecondGroupFirstMessages, VerifyParams, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_delivery_context, build_mint_responses,
    build_mint_second_group_first_messages, build_proof_components_payload,
    build_proof_from_components_payload, build_prove_mint_payload, build_prove_transfer_payload,
    build_transfer_responses, build_transfer_second_group_first_messages, build_verify_payload,
};
use crate::mint_statement::provides::{
    MintFirstMessages, MintPurpose, MintStatementOverrides, TransferPurpose, build_mint_statement,
    build_mint_statement_description,
};
use crate::transfer_statement::provides::{
    TransferFirstMessages, TransferStatementOverrides, build_transfer_statement,
    build_transfer_statement_description,
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
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodeParams,
    FromFieldParamsOverrides, ICanonicalField, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_from_field_params, create_encoding,
};
use envelope::{
    EnvelopeCoinsOverrides, EnvelopeComponents, EnvelopeComponentsOverrides,
    KeyPairComponentsOverrides, PublicKeysComponents, PublicKeysComponentsOverrides,
    build_envelope_coins, build_envelope_components, build_key_pair_components,
    build_public_keys_components,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTag,
    DomainTagConstructorParamsOverrides, HashToScalarIdentifier, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    create_hash_to_scalar, expected_uniform_bytes,
};
use kem::{MasterScalarComponentsOverrides, build_master_scalar_components};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, MsmG1Params, MsmG1Payload,
    MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload, MulG2Params,
    MulG2Payload, NegG1Params, NegG1Payload, NegG2Params, NegG2Payload, NegScalarParams,
    NegScalarPayload, PAIRING_CONCRETES, PairingProductIsOneParams, PairingProductIsOnePayload,
    PairingProductTerm, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

/// Contract: entry `a_mint_proof_verifies_in_both_forms`; `prove_mint` over
///   the statement's witnesses followed by `verify` over the same statement
///   returns `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: every member of `DECLARED_ENCODING_SELECTIONS`, each over every
///   member of `PAIRING_CONCRETES`, of `CHAIN_FORMS_CONCRETES`, and of
///   `keccak_cases`, the members of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   whose `identifier` is `HashToScalarIdentifier::Keccak256V1`; the
///   randomness source `create_random_source` returns for
///   `RandomSourceKind::OperatingSystem`; the adapter
///   `build_schnorr_fs_delivery_proof` builds once per verifier form; and
///   for each of `MintPurpose::Mint` and `MintPurpose::Replacement` the
///   statement `build_algebraic_mint_statement` builds over a context from
///   `build_delivery_context` with that purpose, its values distinct at
///   every position so that a swap of same-kind fields fails, and
///   `build_prove_mint_payload` over it.
/// Act:     `prove_mint(ProveMintParams, payload)`, then
///   `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)`; every factory result `is_ok()`.
/// Boundary: the private calls `prove_mint` and `verify` and the route
///   `create_pairing`, `consume_pairing`, `create_chain_forms`,
///   `consume_chain_forms`, `create_encoding`, `consume_encoding`,
///   `create_hash_to_scalar`, `create_random_source`, `prove_mint`, and
///   `verify`.
/// Mocked:  nothing; the block proves the prover and the verifier agree on
///   the composition of every function on their route.
#[test]
fn a_mint_proof_verifies_in_both_forms() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                        let statement = build_algebraic_mint_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicMintStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );
                                        let payload = build_prove_mint_payload::<P, F>(
                                            self.pairing,
                                            &statement,
                                            Default::default(),
                                        );

                                        // Act
                                        let proof = adapter
                                            .prove_mint(ProveMintParams, payload)
                                            .expect("the statement's witnesses prove")
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
                                        assert_eq!(
                                            result.ok().map(|success| success.is_valid),
                                            Some(true)
                                        );
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_mint_proof_verifies_in_both_forms
}

/// Contract: entry `a_transfer_proof_verifies_in_both_forms`;
///   `prove_transfer` over the statement's witnesses followed by `verify`
///   over the same statement returns
///   `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: the cases and verifier forms of the mint block, and for each of
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant` the statement
///   `build_algebraic_transfer_statement` builds over a context with that
///   purpose, its values distinct at every position, and
///   `build_prove_transfer_payload` over it.
/// Act:     `prove_transfer(ProveTransferParams, payload)`, then
///   `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Transfer(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)`; every factory result `is_ok()`.
/// Boundary: the private calls `prove_transfer` and `verify` and the route
///   of the mint block.
/// Mocked:  nothing.
#[test]
fn a_transfer_proof_verifies_in_both_forms() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in
                                        [TransferPurpose::Transfer, TransferPurpose::Grant]
                                    {
                                        let statement = build_algebraic_transfer_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicTransferStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );
                                        let payload = build_prove_transfer_payload::<P, F>(
                                            self.pairing,
                                            &statement,
                                            Default::default(),
                                        );

                                        // Act
                                        let proof = adapter
                                            .prove_transfer(ProveTransferParams, payload)
                                            .expect("the statement's witnesses prove")
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
                                        assert_eq!(
                                            result.ok().map(|success| success.is_valid),
                                            Some(true)
                                        );
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_transfer_proof_verifies_in_both_forms
}

/// Contract: entry `a_proof_rebuilt_from_its_components_verifies`; the
///   components `proof_components` reads from a proof, rebuilt by
///   `proof_from_components` under an adapter of the same form, verify.
/// Arrange: the cases and verifier forms of the mint block, for the mint
///   statement at each `MintPurpose` and for the transfer statement at each
///   `TransferPurpose`, the proof the relation's prover returns.
/// Act:     `proof_components(ProofComponentsParams, payload)` over the
///   proof, `proof_from_components(ProofFromComponentsParams, payload)` over
///   the components, then `verify(VerifyParams, payload)` over the rebuilt
///   proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)`; every factory result `is_ok()`.
/// Boundary: the private calls `prove_mint` or `prove_transfer`,
///   `proof_components`, `proof_from_components`, and `verify` and the
///   route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_proof_rebuilt_from_its_components_verifies() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                        let statement = build_algebraic_mint_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicMintStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
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
                                                build_proof_components_payload(
                                                    &proof,
                                                    Default::default(),
                                                ),
                                            )
                                            .expect("a proof reads its components")
                                            .components;
                                        let rebuilt = adapter
                                            .proof_from_components(
                                                ProofFromComponentsParams,
                                                build_proof_from_components_payload(
                                                    components,
                                                    Default::default(),
                                                ),
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
                                        assert_eq!(
                                            result.ok().map(|success| success.is_valid),
                                            Some(true)
                                        );
                                    }
                                    for purpose in
                                        [TransferPurpose::Transfer, TransferPurpose::Grant]
                                    {
                                        let statement = build_algebraic_transfer_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicTransferStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
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
                                                build_proof_components_payload(
                                                    &proof,
                                                    Default::default(),
                                                ),
                                            )
                                            .expect("a proof reads its components")
                                            .components;
                                        let rebuilt = adapter
                                            .proof_from_components(
                                                ProofFromComponentsParams,
                                                build_proof_from_components_payload(
                                                    components,
                                                    Default::default(),
                                                ),
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
                                        assert_eq!(
                                            result.ok().map(|success| success.is_valid),
                                            Some(true)
                                        );
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_proof_rebuilt_from_its_components_verifies
}

/// Contract: entry `a_mint_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding`;
///   the proof's challenge equals the oracle scalar of the encoder's bytes
///   for the transcript the statement and the recomputed first messages
///   define, under the proof's tag.
/// Arrange: the cases, verifier forms, and purposes of the mint block; the
///   first messages recomputed from the proof's responses and challenge
///   through `mul_g1`, `mul_g2`, `msm_g1`, and `msm_g2` by the verifier's
///   equations, the second-group ones taken from the proof in the
///   first-group-only form; the transcript `build_mint_statement` builds
///   with the statement's context fields cloned and those first messages;
///   the bytes from the received encoder's `encode` under `EncodeParams`
///   over `build_mint_statement_description(&pairing, Default::default())`;
///   the tag `build_domain_tag` builds from `SCHNORR_FS_CHALLENGE_TAG`; the
///   oracle scalar sampled from `expected_uniform_bytes(&tag, &bytes,
///   P::Scalar::UNIFORM_BYTES_LENGTH)`.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, the encoding of the proof's challenge through the
///   received pairing's `encode_scalar` equals the encoding of the oracle
///   scalar by `assert_eq!`; every factory result `is_ok()`.
/// Boundary: the private call `prove_mint` and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_mint_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                let tag: DomainTag =
                                    build_domain_tag(DomainTagConstructorParamsOverrides {
                                        bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                                    });
                                let description = build_mint_statement_description::<P, F>(
                                    self.pairing,
                                    Default::default(),
                                );
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                        let statement = build_algebraic_mint_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicMintStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );

                                        // Act
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

                                        // Assert
                                        let (challenge, responses, carried) = match &proof {
                                            SchnorrFsProof::MintBothGroups {
                                                challenge,
                                                responses,
                                            } => (challenge, responses, None),
                                            SchnorrFsProof::MintFirstGroupOnly {
                                                challenge,
                                                responses,
                                                first_messages,
                                            } => (challenge, responses, Some(first_messages)),
                                            _ => panic!("prove_mint returns a mint proof"),
                                        };
                                        let n = self
                                            .pairing
                                            .neg_scalar(
                                                NegScalarParams,
                                                NegScalarPayload {
                                                    scalar: challenge.clone(),
                                                },
                                            )
                                            .expect("negating the challenge succeeds")
                                            .negation;
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
                                        let t_c1 = self
                                            .pairing
                                            .msm_g1(
                                                MsmG1Params,
                                                MsmG1Payload {
                                                    terms: vec![
                                                        MsmG1Term {
                                                            base: g1.clone(),
                                                            scalar: responses.rho.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: statement.envelope.c1.clone(),
                                                            scalar: n.clone(),
                                                        },
                                                    ],
                                                },
                                            )
                                            .expect("the recomputation multiplies")
                                            .sum;
                                        let t_c2 = self
                                            .pairing
                                            .msm_g1(
                                                MsmG1Params,
                                                MsmG1Payload {
                                                    terms: vec![
                                                        MsmG1Term {
                                                            base: g1,
                                                            scalar: responses.alpha.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: statement
                                                                .identity_element
                                                                .clone(),
                                                            scalar: responses.r.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: statement.buyer_keys.pk1.clone(),
                                                            scalar: responses.rho.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: statement.envelope.c2.clone(),
                                                            scalar: n.clone(),
                                                        },
                                                    ],
                                                },
                                            )
                                            .expect("the recomputation multiplies")
                                            .sum;
                                        let (t_hpub, t_d1, t_d2) = match carried {
                                            Some(messages) => (
                                                messages.hpub.clone(),
                                                messages.d1.clone(),
                                                messages.d2.clone(),
                                            ),
                                            None => (
                                                self.pairing
                                                    .msm_g2(
                                                        MsmG2Params,
                                                        MsmG2Payload {
                                                            terms: vec![
                                                                MsmG2Term {
                                                                    base: g2.clone(),
                                                                    scalar: responses.alpha.clone(),
                                                                },
                                                                MsmG2Term {
                                                                    base: statement.hpub.clone(),
                                                                    scalar: n.clone(),
                                                                },
                                                            ],
                                                        },
                                                    )
                                                    .expect("the recomputation multiplies")
                                                    .sum,
                                                self.pairing
                                                    .msm_g2(
                                                        MsmG2Params,
                                                        MsmG2Payload {
                                                            terms: vec![
                                                                MsmG2Term {
                                                                    base: g2.clone(),
                                                                    scalar: responses.sigma.clone(),
                                                                },
                                                                MsmG2Term {
                                                                    base: statement
                                                                        .envelope
                                                                        .d1
                                                                        .clone(),
                                                                    scalar: n.clone(),
                                                                },
                                                            ],
                                                        },
                                                    )
                                                    .expect("the recomputation multiplies")
                                                    .sum,
                                                self.pairing
                                                    .msm_g2(
                                                        MsmG2Params,
                                                        MsmG2Payload {
                                                            terms: vec![
                                                                MsmG2Term {
                                                                    base: g2,
                                                                    scalar: responses.r.clone(),
                                                                },
                                                                MsmG2Term {
                                                                    base: statement
                                                                        .buyer_keys
                                                                        .pk2
                                                                        .clone(),
                                                                    scalar: responses.sigma.clone(),
                                                                },
                                                                MsmG2Term {
                                                                    base: statement
                                                                        .envelope
                                                                        .d2
                                                                        .clone(),
                                                                    scalar: n,
                                                                },
                                                            ],
                                                        },
                                                    )
                                                    .expect("the recomputation multiplies")
                                                    .sum,
                                            ),
                                        };
                                        let transcript = build_mint_statement::<P, F>(
                                            self.pairing,
                                            MintStatementOverrides {
                                                suite_identifier: Some(
                                                    statement.context.suite_identifier.clone(),
                                                ),
                                                chain: Some(statement.context.chain.clone()),
                                                entitlement_contract: Some(
                                                    statement.context.entitlement_contract.clone(),
                                                ),
                                                asset_identity_hash: Some(
                                                    statement.context.asset_identity_hash.clone(),
                                                ),
                                                parameter_set_identifier: Some(
                                                    statement
                                                        .context
                                                        .parameter_set_identifier
                                                        .clone(),
                                                ),
                                                source_entitlement: Some(
                                                    statement.context.source_entitlement.clone(),
                                                ),
                                                target_entitlement: Some(
                                                    statement.context.target_entitlement.clone(),
                                                ),
                                                old_interval: Some(
                                                    statement.context.old_interval.clone(),
                                                ),
                                                new_interval: Some(
                                                    statement.context.new_interval.clone(),
                                                ),
                                                purpose: Some(statement.context.purpose),
                                                seller: Some(statement.context.seller.clone()),
                                                buyer: Some(statement.context.buyer.clone()),
                                                buyer_keys: Some(PublicKeysComponents {
                                                    pk1: statement.buyer_keys.pk1.clone(),
                                                    pk2: statement.buyer_keys.pk2.clone(),
                                                }),
                                                envelope: Some(EnvelopeComponents {
                                                    c1: statement.envelope.c1.clone(),
                                                    c2: statement.envelope.c2.clone(),
                                                    d1: statement.envelope.d1.clone(),
                                                    d2: statement.envelope.d2.clone(),
                                                }),
                                                expiry: Some(statement.context.expiry),
                                                first_messages: Some(MintFirstMessages {
                                                    hpub: t_hpub,
                                                    c1: t_c1,
                                                    c2: t_c2,
                                                    d1: t_d1,
                                                    d2: t_d2,
                                                }),
                                            },
                                        );
                                        let bytes = encoded
                                            .adapter
                                            .encode(
                                                EncodeParams {
                                                    description: &description,
                                                },
                                                &transcript,
                                            )
                                            .expect("a built transcript encodes")
                                            .bytes;
                                        let oracle = P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            SampleUniformScalarPayload {
                                                uniform: build_secret(
                                                    expected_uniform_bytes(
                                                        &tag,
                                                        &bytes,
                                                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                                    ),
                                                    SecretConstructorParamsOverrides,
                                                ),
                                            },
                                        )
                                        .expect("a full-length uniform samples")
                                        .scalar
                                        .expose()
                                        .clone();
                                        assert_eq!(
                                            self.pairing
                                                .encode_scalar(
                                                    EncodeScalarParams,
                                                    EncodeScalarPayload {
                                                        scalar: challenge.clone(),
                                                    },
                                                )
                                                .expect("encoding the challenge succeeds")
                                                .bytes
                                                .expose()
                                                .as_ref(),
                                            self.pairing
                                                .encode_scalar(
                                                    EncodeScalarParams,
                                                    EncodeScalarPayload { scalar: oracle },
                                                )
                                                .expect("encoding the oracle succeeds")
                                                .bytes
                                                .expose()
                                                .as_ref(),
                                        );
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_mint_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding
}

/// Contract: entry `a_transfer_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding`;
///   the proof's challenge equals the oracle scalar of the encoder's bytes
///   for the transfer transcript the statement and the recomputed first
///   messages define, under the proof's tag.
/// Arrange: the cases, verifier forms, and purposes of the transfer block;
///   the six first messages recomputed as the mint block states, the
///   second-group ones taken from the proof in the first-group-only form;
///   the transcript `build_transfer_statement` builds with the statement's
///   context fields cloned and those first messages; the bytes from `encode`
///   over `build_transfer_statement_description(&pairing,
///   Default::default())`; the tag and the oracle scalar as the mint block
///   states.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, the encoding of the proof's challenge through the
///   received pairing's `encode_scalar` equals the encoding of the oracle
///   scalar by `assert_eq!`; every factory result `is_ok()`.
/// Boundary: the private call `prove_transfer` and the route of the mint
///   block.
/// Mocked:  nothing.
#[test]
fn a_transfer_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                let tag: DomainTag =
                                    build_domain_tag(DomainTagConstructorParamsOverrides {
                                        bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                                    });
                                let description = build_transfer_statement_description::<P, F>(
                                    self.pairing,
                                    Default::default(),
                                );
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in
                                        [TransferPurpose::Transfer, TransferPurpose::Grant]
                                    {
                                        let statement = build_algebraic_transfer_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicTransferStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );

                                        // Act
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

                                        // Assert
                                        let (challenge, responses, carried) = match &proof {
                                            SchnorrFsProof::TransferBothGroups {
                                                challenge,
                                                responses,
                                            } => (challenge, responses, None),
                                            SchnorrFsProof::TransferFirstGroupOnly {
                                                challenge,
                                                responses,
                                                first_messages,
                                            } => (challenge, responses, Some(first_messages)),
                                            _ => panic!("prove_transfer returns a transfer proof"),
                                        };
                                        let n = self
                                            .pairing
                                            .neg_scalar(
                                                NegScalarParams,
                                                NegScalarPayload {
                                                    scalar: challenge.clone(),
                                                },
                                            )
                                            .expect("negating the challenge succeeds")
                                            .negation;
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
                                        let neg_c2_old = self
                                            .pairing
                                            .neg_g1(
                                                NegG1Params,
                                                NegG1Payload {
                                                    point: statement.old_envelope.c2.clone(),
                                                },
                                            )
                                            .expect("negating the old c2 succeeds")
                                            .negation;
                                        let delta_c2 = self
                                            .pairing
                                            .add_g1(
                                                AddG1Params,
                                                AddG1Payload {
                                                    left: statement.new_envelope.c2.clone(),
                                                    right: neg_c2_old,
                                                },
                                            )
                                            .expect("the c2 delta adds")
                                            .sum;
                                        let t_pk1 = self
                                            .pairing
                                            .msm_g1(
                                                MsmG1Params,
                                                MsmG1Payload {
                                                    terms: vec![
                                                        MsmG1Term {
                                                            base: g1.clone(),
                                                            scalar: responses.x.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: statement.seller_keys.pk1.clone(),
                                                            scalar: n.clone(),
                                                        },
                                                    ],
                                                },
                                            )
                                            .expect("the recomputation multiplies")
                                            .sum;
                                        let t_c1 = self
                                            .pairing
                                            .msm_g1(
                                                MsmG1Params,
                                                MsmG1Payload {
                                                    terms: vec![
                                                        MsmG1Term {
                                                            base: g1,
                                                            scalar: responses.rho.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: statement.new_envelope.c1.clone(),
                                                            scalar: n.clone(),
                                                        },
                                                    ],
                                                },
                                            )
                                            .expect("the recomputation multiplies")
                                            .sum;
                                        let t_c2 = self
                                            .pairing
                                            .msm_g1(
                                                MsmG1Params,
                                                MsmG1Payload {
                                                    terms: vec![
                                                        MsmG1Term {
                                                            base: statement.old_envelope.c1.clone(),
                                                            scalar: self
                                                                .pairing
                                                                .neg_scalar(
                                                                    NegScalarParams,
                                                                    NegScalarPayload {
                                                                        scalar: responses.x.clone(),
                                                                    },
                                                                )
                                                                .expect("negating x succeeds")
                                                                .negation,
                                                        },
                                                        MsmG1Term {
                                                            base: statement
                                                                .identity_element
                                                                .clone(),
                                                            scalar: responses.s.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: statement.buyer_keys.pk1.clone(),
                                                            scalar: responses.rho.clone(),
                                                        },
                                                        MsmG1Term {
                                                            base: delta_c2,
                                                            scalar: n.clone(),
                                                        },
                                                    ],
                                                },
                                            )
                                            .expect("the recomputation multiplies")
                                            .sum;
                                        let (t_pk2, t_d1, t_d2) = match carried {
                                            Some(messages) => (
                                                messages.pk2.clone(),
                                                messages.d1.clone(),
                                                messages.d2.clone(),
                                            ),
                                            None => {
                                                let neg_d2_old = self
                                                    .pairing
                                                    .neg_g2(
                                                        NegG2Params,
                                                        NegG2Payload {
                                                            point: statement
                                                                .old_envelope
                                                                .d2
                                                                .clone(),
                                                        },
                                                    )
                                                    .expect("negating the old d2 succeeds")
                                                    .negation;
                                                let delta_d2 = self
                                                    .pairing
                                                    .add_g2(
                                                        AddG2Params,
                                                        AddG2Payload {
                                                            left: statement.new_envelope.d2.clone(),
                                                            right: neg_d2_old,
                                                        },
                                                    )
                                                    .expect("the d2 delta adds")
                                                    .sum;
                                                (
                                                    self.pairing
                                                        .msm_g2(
                                                            MsmG2Params,
                                                            MsmG2Payload {
                                                                terms: vec![
                                                                    MsmG2Term {
                                                                        base: g2.clone(),
                                                                        scalar: responses
                                                                            .y
                                                                            .clone(),
                                                                    },
                                                                    MsmG2Term {
                                                                        base: statement
                                                                            .seller_keys
                                                                            .pk2
                                                                            .clone(),
                                                                        scalar: n.clone(),
                                                                    },
                                                                ],
                                                            },
                                                        )
                                                        .expect("the recomputation multiplies")
                                                        .sum,
                                                    self.pairing
                                                        .msm_g2(
                                                            MsmG2Params,
                                                            MsmG2Payload {
                                                                terms: vec![
                                                                    MsmG2Term {
                                                                        base: g2.clone(),
                                                                        scalar: responses
                                                                            .sigma
                                                                            .clone(),
                                                                    },
                                                                    MsmG2Term {
                                                                        base: statement
                                                                            .new_envelope
                                                                            .d1
                                                                            .clone(),
                                                                        scalar: n.clone(),
                                                                    },
                                                                ],
                                                            },
                                                        )
                                                        .expect("the recomputation multiplies")
                                                        .sum,
                                                    self.pairing
                                                        .msm_g2(
                                                            MsmG2Params,
                                                            MsmG2Payload {
                                                                terms: vec![
                                                                    MsmG2Term {
                                                                        base: statement
                                                                            .old_envelope
                                                                            .d1
                                                                            .clone(),
                                                                        scalar: self
                                                                            .pairing
                                                                            .neg_scalar(
                                                                                NegScalarParams,
                                                                                NegScalarPayload {
                                                                                    scalar:
                                                                                        responses
                                                                                            .y
                                                                                            .clone(),
                                                                                },
                                                                            )
                                                                            .expect(
                                                                                "negating y succeeds",
                                                                            )
                                                                            .negation,
                                                                    },
                                                                    MsmG2Term {
                                                                        base: g2,
                                                                        scalar: responses
                                                                            .s
                                                                            .clone(),
                                                                    },
                                                                    MsmG2Term {
                                                                        base: statement
                                                                            .buyer_keys
                                                                            .pk2
                                                                            .clone(),
                                                                        scalar: responses
                                                                            .sigma
                                                                            .clone(),
                                                                    },
                                                                    MsmG2Term {
                                                                        base: delta_d2,
                                                                        scalar: n,
                                                                    },
                                                                ],
                                                            },
                                                        )
                                                        .expect("the recomputation multiplies")
                                                        .sum,
                                                )
                                            }
                                        };
                                        let transcript = build_transfer_statement::<P, F>(
                                            self.pairing,
                                            TransferStatementOverrides {
                                                suite_identifier: Some(
                                                    statement.context.suite_identifier.clone(),
                                                ),
                                                chain: Some(statement.context.chain.clone()),
                                                entitlement_contract: Some(
                                                    statement.context.entitlement_contract.clone(),
                                                ),
                                                asset_identity_hash: Some(
                                                    statement.context.asset_identity_hash.clone(),
                                                ),
                                                parameter_set_identifier: Some(
                                                    statement
                                                        .context
                                                        .parameter_set_identifier
                                                        .clone(),
                                                ),
                                                source_entitlement: Some(
                                                    statement.context.source_entitlement.clone(),
                                                ),
                                                target_entitlement: Some(
                                                    statement.context.target_entitlement.clone(),
                                                ),
                                                old_interval: Some(
                                                    statement.context.old_interval.clone(),
                                                ),
                                                new_interval: Some(
                                                    statement.context.new_interval.clone(),
                                                ),
                                                purpose: Some(statement.context.purpose),
                                                seller: Some(statement.context.seller.clone()),
                                                buyer: Some(statement.context.buyer.clone()),
                                                seller_keys: Some(PublicKeysComponents {
                                                    pk1: statement.seller_keys.pk1.clone(),
                                                    pk2: statement.seller_keys.pk2.clone(),
                                                }),
                                                buyer_keys: Some(PublicKeysComponents {
                                                    pk1: statement.buyer_keys.pk1.clone(),
                                                    pk2: statement.buyer_keys.pk2.clone(),
                                                }),
                                                old_envelope: Some(EnvelopeComponents {
                                                    c1: statement.old_envelope.c1.clone(),
                                                    c2: statement.old_envelope.c2.clone(),
                                                    d1: statement.old_envelope.d1.clone(),
                                                    d2: statement.old_envelope.d2.clone(),
                                                }),
                                                new_envelope: Some(EnvelopeComponents {
                                                    c1: statement.new_envelope.c1.clone(),
                                                    c2: statement.new_envelope.c2.clone(),
                                                    d1: statement.new_envelope.d1.clone(),
                                                    d2: statement.new_envelope.d2.clone(),
                                                }),
                                                expiry: Some(statement.context.expiry),
                                                first_messages: Some(TransferFirstMessages {
                                                    pk1: t_pk1,
                                                    c1: t_c1,
                                                    c2: t_c2,
                                                    pk2: t_pk2,
                                                    d1: t_d1,
                                                    d2: t_d2,
                                                }),
                                            },
                                        );
                                        let bytes = encoded
                                            .adapter
                                            .encode(
                                                EncodeParams {
                                                    description: &description,
                                                },
                                                &transcript,
                                            )
                                            .expect("a built transcript encodes")
                                            .bytes;
                                        let oracle = P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            SampleUniformScalarPayload {
                                                uniform: build_secret(
                                                    expected_uniform_bytes(
                                                        &tag,
                                                        &bytes,
                                                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                                    ),
                                                    SecretConstructorParamsOverrides,
                                                ),
                                            },
                                        )
                                        .expect("a full-length uniform samples")
                                        .scalar
                                        .expose()
                                        .clone();
                                        assert_eq!(
                                            self.pairing
                                                .encode_scalar(
                                                    EncodeScalarParams,
                                                    EncodeScalarPayload {
                                                        scalar: challenge.clone(),
                                                    },
                                                )
                                                .expect("encoding the challenge succeeds")
                                                .bytes
                                                .expose()
                                                .as_ref(),
                                            self.pairing
                                                .encode_scalar(
                                                    EncodeScalarParams,
                                                    EncodeScalarPayload { scalar: oracle },
                                                )
                                                .expect("encoding the oracle succeeds")
                                                .bytes
                                                .expose()
                                                .as_ref(),
                                        );
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_transfer_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding
}

/// Contract: entry `a_proof_fails_against_a_statement_differing_in_one_field`;
///   `verify` over a statement that differs from the proof's statement in
///   one field returns `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases and verifier forms of the mint block; the proof
///   formed for the default statement; for each field of
///   `AlgebraicMintStatement` and of `AlgebraicTransferStatement`, the
///   context fields included, the variant statement overridden in that
///   field alone with a value from that field's own builder distinct from
///   the default, so the baseline verifies and each variant is the only
///   difference.
/// Act:     `verify(VerifyParams, payload)` over the variant statement and
///   the baseline proof.
/// Assert:  in place, the baseline maps to `Some(true)` and
///   `result.ok().map(|success| success.is_valid)` equals `Some(false)`
///   for each variant; every factory result `is_ok()`.
/// Boundary: the private calls `prove_mint` or `prove_transfer` and
///   `verify` and the route of the mint block, run for each variant.
/// Mocked:  nothing.
#[test]
fn a_proof_fails_against_a_statement_differing_in_one_field() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                        CanonicalFieldKind::Bytes => {
                                            CanonicalFieldValue::Bytes(vec![seed])
                                        }
                                        CanonicalFieldKind::Text => {
                                            CanonicalFieldValue::Text(seed.to_string())
                                        }
                                    },
                                )
                                .expect("a seeded field of the declared kind is admitted")
                                .value
                            }

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
                                            vec![
                                                0xC1;
                                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                            ],
                                            SecretConstructorParamsOverrides,
                                        ),
                                    },
                                )
                                .expect("a full-length uniform samples")
                                .scalar
                                .expose()
                                .clone();
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                        let baseline = build_algebraic_mint_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicMintStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
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
                                                            suite_identifier: Some(
                                                                build_suite_identifier(
                                                                    SuiteIdentifierConstructorParamsOverrides {
                                                                        identifier: Some(
                                                                            [0xB3; SUITE_IDENTIFIER_LENGTH],
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            chain: Some(
                                                                variant_form::<F::ChainIdentifier>(
                                                                    0xB1,
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            entitlement_contract: Some(
                                                                variant_form::<F::Identity>(0xB2),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            asset_identity_hash: Some(
                                                                build_asset_identity_hash(
                                                                    AssetIdentityHashConstructorParamsOverrides {
                                                                        bytes: Some(
                                                                            [0xB4; ASSET_IDENTITY_HASH_LENGTH],
                                                                        ),
                                                                    },
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            parameter_set_identifier: Some(
                                                                build_parameter_set_identifier(
                                                                    ParameterSetIdentifierConstructorParamsOverrides {
                                                                        bytes: Some(
                                                                            [0xB5; PARAMETER_SET_IDENTIFIER_LENGTH],
                                                                        ),
                                                                    },
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            source_entitlement: Some(
                                                                variant_form::<F::Entitlement>(
                                                                    0xB6,
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            target_entitlement: Some(
                                                                variant_form::<F::Entitlement>(
                                                                    0xB7,
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            old_interval: Some(
                                                                variant_form::<F::Interval>(0xB8),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            new_interval: Some(
                                                                variant_form::<F::Interval>(0xB9),
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
                                                    context: Some(build_delivery_context(
                                                        other,
                                                        Default::default(),
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
                                                            seller: Some(
                                                                variant_form::<F::Identity>(0xBA),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            buyer: Some(
                                                                variant_form::<F::Identity>(0xBB),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
                                                    buyer_keys: Some(
                                                        build_public_keys_components(
                                                            self.pairing,
                                                            PublicKeysComponentsOverrides {
                                                                pk1: Some(
                                                                    self.pairing
                                                                        .mul_g1(
                                                                            MulG1Params,
                                                                            MulG1Payload {
                                                                                point: g1
                                                                                    .clone(),
                                                                                scalar: distinct
                                                                                    .clone(),
                                                                            },
                                                                        )
                                                                        .expect(
                                                                            "a variant point multiplies",
                                                                        )
                                                                        .product,
                                                                ),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_algebraic_mint_statement::<P, F>(
                                                self.pairing,
                                                AlgebraicMintStatementOverrides {
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
                                                    envelope: Some(build_envelope_components(
                                                        self.pairing,
                                                        EnvelopeComponentsOverrides {
                                                            c1: Some(
                                                                self.pairing
                                                                    .mul_g1(
                                                                        MulG1Params,
                                                                        MulG1Payload {
                                                                            point: g1.clone(),
                                                                            scalar: distinct
                                                                                .clone(),
                                                                        },
                                                                    )
                                                                    .expect(
                                                                        "a variant point multiplies",
                                                                    )
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
                                            assert_eq!(
                                                result.ok().map(|success| success.is_valid),
                                                Some(false)
                                            );
                                        }
                                    }
                                    for purpose in
                                        [TransferPurpose::Transfer, TransferPurpose::Grant]
                                    {
                                        let baseline = build_algebraic_transfer_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicTransferStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
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
                                                            suite_identifier: Some(
                                                                build_suite_identifier(
                                                                    SuiteIdentifierConstructorParamsOverrides {
                                                                        identifier: Some(
                                                                            [0xB3; SUITE_IDENTIFIER_LENGTH],
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            chain: Some(
                                                                variant_form::<F::ChainIdentifier>(
                                                                    0xB1,
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            entitlement_contract: Some(
                                                                variant_form::<F::Identity>(0xB2),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            asset_identity_hash: Some(
                                                                build_asset_identity_hash(
                                                                    AssetIdentityHashConstructorParamsOverrides {
                                                                        bytes: Some(
                                                                            [0xB4; ASSET_IDENTITY_HASH_LENGTH],
                                                                        ),
                                                                    },
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            parameter_set_identifier: Some(
                                                                build_parameter_set_identifier(
                                                                    ParameterSetIdentifierConstructorParamsOverrides {
                                                                        bytes: Some(
                                                                            [0xB5; PARAMETER_SET_IDENTIFIER_LENGTH],
                                                                        ),
                                                                    },
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            source_entitlement: Some(
                                                                variant_form::<F::Entitlement>(
                                                                    0xB6,
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            target_entitlement: Some(
                                                                variant_form::<F::Entitlement>(
                                                                    0xB7,
                                                                ),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            old_interval: Some(
                                                                variant_form::<F::Interval>(0xB8),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            new_interval: Some(
                                                                variant_form::<F::Interval>(0xB9),
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
                                                    context: Some(build_delivery_context(
                                                        other,
                                                        Default::default(),
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
                                                            seller: Some(
                                                                variant_form::<F::Identity>(0xBA),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        DeliveryContextOverrides {
                                                            buyer: Some(
                                                                variant_form::<F::Identity>(0xBB),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
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
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
                                                    seller_keys: Some(
                                                        build_public_keys_components(
                                                            self.pairing,
                                                            PublicKeysComponentsOverrides {
                                                                pk1: Some(
                                                                    self.pairing
                                                                        .mul_g1(
                                                                            MulG1Params,
                                                                            MulG1Payload {
                                                                                point: g1
                                                                                    .clone(),
                                                                                scalar: distinct
                                                                                    .clone(),
                                                                            },
                                                                        )
                                                                        .expect(
                                                                            "a variant point multiplies",
                                                                        )
                                                                        .product,
                                                                ),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_algebraic_transfer_statement::<P, F>(
                                                self.pairing,
                                                AlgebraicTransferStatementOverrides {
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
                                                    buyer_keys: Some(
                                                        build_public_keys_components(
                                                            self.pairing,
                                                            PublicKeysComponentsOverrides {
                                                                pk1: Some(
                                                                    self.pairing
                                                                        .mul_g1(
                                                                            MulG1Params,
                                                                            MulG1Payload {
                                                                                point: g1
                                                                                    .clone(),
                                                                                scalar: distinct
                                                                                    .clone(),
                                                                            },
                                                                        )
                                                                        .expect(
                                                                            "a variant point multiplies",
                                                                        )
                                                                        .product,
                                                                ),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_algebraic_transfer_statement::<P, F>(
                                                self.pairing,
                                                AlgebraicTransferStatementOverrides {
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
                                                    old_envelope: Some(
                                                        build_envelope_components(
                                                            self.pairing,
                                                            EnvelopeComponentsOverrides {
                                                                c1: Some(
                                                                    self.pairing
                                                                        .mul_g1(
                                                                            MulG1Params,
                                                                            MulG1Payload {
                                                                                point: g1
                                                                                    .clone(),
                                                                                scalar: distinct
                                                                                    .clone(),
                                                                            },
                                                                        )
                                                                        .expect(
                                                                            "a variant point multiplies",
                                                                        )
                                                                        .product,
                                                                ),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_algebraic_transfer_statement::<P, F>(
                                                self.pairing,
                                                AlgebraicTransferStatementOverrides {
                                                    context: Some(build_delivery_context(
                                                        purpose,
                                                        Default::default(),
                                                    )),
                                                    new_envelope: Some(
                                                        build_envelope_components(
                                                            self.pairing,
                                                            EnvelopeComponentsOverrides {
                                                                c1: Some(
                                                                    self.pairing
                                                                        .mul_g1(
                                                                            MulG1Params,
                                                                            MulG1Payload {
                                                                                point: g1
                                                                                    .clone(),
                                                                                scalar: distinct
                                                                                    .clone(),
                                                                            },
                                                                        )
                                                                        .expect(
                                                                            "a variant point multiplies",
                                                                        )
                                                                        .product,
                                                                ),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
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
                                            assert_eq!(
                                                result.ok().map(|success| success.is_valid),
                                                Some(false)
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_proof_fails_against_a_statement_differing_in_one_field
}

/// Contract: entry `a_proof_with_a_changed_response_or_challenge_fails`;
///   `verify` over a proof whose challenge or one response is another
///   scalar returns `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases and verifier forms of the mint block; the proof from
///   `build_schnorr_fs_mint_proof` or `build_schnorr_fs_transfer_proof`
///   with no overrides verifying, and a variant for the challenge and for
///   each response overridden alone by a scalar other than the baseline's.
/// Act:     `verify(VerifyParams, payload)` over each variant proof.
/// Assert:  in place, the baseline maps to `Some(true)` and
///   `result.ok().map(|success| success.is_valid)` equals `Some(false)`
///   for each variant; every factory result `is_ok()`.
/// Boundary: the private call `verify` and the route of the mint block,
///   run for each variant.
/// Mocked:  nothing.
#[test]
fn a_proof_with_a_changed_response_or_challenge_fails() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                        let statement = build_algebraic_mint_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicMintStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );
                                        let payload = build_prove_mint_payload::<P, F>(
                                            self.pairing,
                                            &statement,
                                            Default::default(),
                                        );
                                        let baseline = build_schnorr_fs_mint_proof(
                                            &adapter,
                                            &payload,
                                            Default::default(),
                                        );
                                        let baseline_result = adapter.verify(
                                            VerifyParams,
                                            build_verify_payload(
                                                AlgebraicStatement::Mint(&statement),
                                                &baseline,
                                                Default::default(),
                                            ),
                                        );
                                        assert_eq!(
                                            baseline_result.ok().map(|success| success.is_valid),
                                            Some(true)
                                        );
                                        let responses = match &baseline {
                                            SchnorrFsProof::MintBothGroups {
                                                responses, ..
                                            }
                                            | SchnorrFsProof::MintFirstGroupOnly {
                                                responses,
                                                ..
                                            } => responses,
                                            _ => {
                                                panic!("the mint route returns a mint proof")
                                            }
                                        };
                                        let foreign =
                                            build_mint_responses(self.pairing, Default::default());
                                        let variants = [
                                            build_schnorr_fs_mint_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsMintProofOverrides {
                                                    challenge: Some(foreign.alpha.clone()),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_mint_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsMintProofOverrides {
                                                    responses: Some(MintResponses {
                                                        alpha: foreign.alpha.clone(),
                                                        r: responses.r.clone(),
                                                        rho: responses.rho.clone(),
                                                        sigma: responses.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_mint_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsMintProofOverrides {
                                                    responses: Some(MintResponses {
                                                        alpha: responses.alpha.clone(),
                                                        r: foreign.r.clone(),
                                                        rho: responses.rho.clone(),
                                                        sigma: responses.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_mint_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsMintProofOverrides {
                                                    responses: Some(MintResponses {
                                                        alpha: responses.alpha.clone(),
                                                        r: responses.r.clone(),
                                                        rho: foreign.rho.clone(),
                                                        sigma: responses.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_mint_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsMintProofOverrides {
                                                    responses: Some(MintResponses {
                                                        alpha: responses.alpha.clone(),
                                                        r: responses.r.clone(),
                                                        rho: responses.rho.clone(),
                                                        sigma: foreign.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                        ];

                                        // Act
                                        for variant in &variants {
                                            let result = adapter.verify(
                                                VerifyParams,
                                                build_verify_payload(
                                                    AlgebraicStatement::Mint(&statement),
                                                    variant,
                                                    Default::default(),
                                                ),
                                            );

                                            // Assert
                                            assert_eq!(
                                                result.ok().map(|success| success.is_valid),
                                                Some(false)
                                            );
                                        }
                                    }
                                    for purpose in
                                        [TransferPurpose::Transfer, TransferPurpose::Grant]
                                    {
                                        let statement = build_algebraic_transfer_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicTransferStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );
                                        let payload = build_prove_transfer_payload::<P, F>(
                                            self.pairing,
                                            &statement,
                                            Default::default(),
                                        );
                                        let baseline = build_schnorr_fs_transfer_proof(
                                            &adapter,
                                            &payload,
                                            Default::default(),
                                        );
                                        let baseline_result = adapter.verify(
                                            VerifyParams,
                                            build_verify_payload(
                                                AlgebraicStatement::Transfer(&statement),
                                                &baseline,
                                                Default::default(),
                                            ),
                                        );
                                        assert_eq!(
                                            baseline_result.ok().map(|success| success.is_valid),
                                            Some(true)
                                        );
                                        let responses = match &baseline {
                                            SchnorrFsProof::TransferBothGroups {
                                                responses,
                                                ..
                                            }
                                            | SchnorrFsProof::TransferFirstGroupOnly {
                                                responses,
                                                ..
                                            } => responses,
                                            _ => panic!(
                                                "the transfer route returns a transfer proof"
                                            ),
                                        };
                                        let foreign = build_transfer_responses(
                                            self.pairing,
                                            Default::default(),
                                        );
                                        let variants = [
                                            build_schnorr_fs_transfer_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsTransferProofOverrides {
                                                    challenge: Some(foreign.x.clone()),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_transfer_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsTransferProofOverrides {
                                                    responses: Some(TransferResponses {
                                                        x: foreign.x.clone(),
                                                        y: responses.y.clone(),
                                                        s: responses.s.clone(),
                                                        rho: responses.rho.clone(),
                                                        sigma: responses.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_transfer_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsTransferProofOverrides {
                                                    responses: Some(TransferResponses {
                                                        x: responses.x.clone(),
                                                        y: foreign.y.clone(),
                                                        s: responses.s.clone(),
                                                        rho: responses.rho.clone(),
                                                        sigma: responses.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_transfer_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsTransferProofOverrides {
                                                    responses: Some(TransferResponses {
                                                        x: responses.x.clone(),
                                                        y: responses.y.clone(),
                                                        s: foreign.s.clone(),
                                                        rho: responses.rho.clone(),
                                                        sigma: responses.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_transfer_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsTransferProofOverrides {
                                                    responses: Some(TransferResponses {
                                                        x: responses.x.clone(),
                                                        y: responses.y.clone(),
                                                        s: responses.s.clone(),
                                                        rho: foreign.rho.clone(),
                                                        sigma: responses.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_schnorr_fs_transfer_proof(
                                                &adapter,
                                                &payload,
                                                SchnorrFsTransferProofOverrides {
                                                    responses: Some(TransferResponses {
                                                        x: responses.x.clone(),
                                                        y: responses.y.clone(),
                                                        s: responses.s.clone(),
                                                        rho: responses.rho.clone(),
                                                        sigma: foreign.sigma.clone(),
                                                    }),
                                                    ..Default::default()
                                                },
                                            ),
                                        ];

                                        // Act
                                        for variant in &variants {
                                            let result = adapter.verify(
                                                VerifyParams,
                                                build_verify_payload(
                                                    AlgebraicStatement::Transfer(&statement),
                                                    variant,
                                                    Default::default(),
                                                ),
                                            );

                                            // Assert
                                            assert_eq!(
                                                result.ok().map(|success| success.is_valid),
                                                Some(false)
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_proof_with_a_changed_response_or_challenge_fails
}

/// Contract: entry `a_first_group_only_proof_with_a_changed_second_group_first_message_fails`;
///   `verify` in the first-group-only form over a proof whose carried
///   second-group first message is another point, its challenge
///   recomputed over the changed transcript, returns
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases of the mint block in the first-group-only form; the
///   proof from the proof builder with `first_messages` overridden in
///   each carried message alone by a point other than the baseline's,
///   for the mint and for the transfer proof.
/// Act:     `verify(VerifyParams, payload)` over each variant proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)`
///   equals `Some(false)` for each variant; every factory result
///   `is_ok()`.
/// Boundary: the private call `verify` and the route of the mint block,
///   run for each variant.
/// Mocked:  nothing.
#[test]
fn a_first_group_only_proof_with_a_changed_second_group_first_message_fails() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                    build_schnorr_fs_delivery_proof(
                                        self.pairing,
                                        &*hashed.adapter,
                                        &encoded.adapter,
                                        &*random.adapter,
                                        SchnorrFsDeliveryProofConstructorParamsOverrides {
                                            verifier_form: Some(
                                                VerifierGroupArithmetic::FirstGroupOnly,
                                            ),
                                        },
                                    );
                                for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                    let statement = build_algebraic_mint_statement::<P, F>(
                                        self.pairing,
                                        AlgebraicMintStatementOverrides {
                                            context: Some(build_delivery_context(
                                                purpose,
                                                Default::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    let payload = build_prove_mint_payload::<P, F>(
                                        self.pairing,
                                        &statement,
                                        Default::default(),
                                    );
                                    let baseline = build_schnorr_fs_mint_proof(
                                        &adapter,
                                        &payload,
                                        Default::default(),
                                    );
                                    let carried = match &baseline {
                                        SchnorrFsProof::MintFirstGroupOnly {
                                            first_messages,
                                            ..
                                        } => first_messages,
                                        _ => panic!(
                                            "the first-group-only adapter returns a first-group-only proof"
                                        ),
                                    };
                                    let foreign = build_mint_second_group_first_messages(
                                        self.pairing,
                                        Default::default(),
                                    );
                                    let variants = [
                                        build_schnorr_fs_mint_proof(
                                            &adapter,
                                            &payload,
                                            SchnorrFsMintProofOverrides {
                                                first_messages: Some(
                                                    MintSecondGroupFirstMessages {
                                                        hpub: foreign.hpub.clone(),
                                                        d1: carried.d1.clone(),
                                                        d2: carried.d2.clone(),
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                        ),
                                        build_schnorr_fs_mint_proof(
                                            &adapter,
                                            &payload,
                                            SchnorrFsMintProofOverrides {
                                                first_messages: Some(
                                                    MintSecondGroupFirstMessages {
                                                        hpub: carried.hpub.clone(),
                                                        d1: foreign.d1.clone(),
                                                        d2: carried.d2.clone(),
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                        ),
                                        build_schnorr_fs_mint_proof(
                                            &adapter,
                                            &payload,
                                            SchnorrFsMintProofOverrides {
                                                first_messages: Some(
                                                    MintSecondGroupFirstMessages {
                                                        hpub: carried.hpub.clone(),
                                                        d1: carried.d1.clone(),
                                                        d2: foreign.d2.clone(),
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                        ),
                                    ];

                                    // Act
                                    for variant in &variants {
                                        let result = adapter.verify(
                                            VerifyParams,
                                            build_verify_payload(
                                                AlgebraicStatement::Mint(&statement),
                                                variant,
                                                Default::default(),
                                            ),
                                        );

                                        // Assert
                                        assert_eq!(
                                            result.ok().map(|success| success.is_valid),
                                            Some(false)
                                        );
                                    }
                                }
                                for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                                    let statement = build_algebraic_transfer_statement::<P, F>(
                                        self.pairing,
                                        AlgebraicTransferStatementOverrides {
                                            context: Some(build_delivery_context(
                                                purpose,
                                                Default::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    let payload = build_prove_transfer_payload::<P, F>(
                                        self.pairing,
                                        &statement,
                                        Default::default(),
                                    );
                                    let baseline = build_schnorr_fs_transfer_proof(
                                        &adapter,
                                        &payload,
                                        Default::default(),
                                    );
                                    let carried = match &baseline {
                                        SchnorrFsProof::TransferFirstGroupOnly {
                                            first_messages,
                                            ..
                                        } => first_messages,
                                        _ => panic!(
                                            "the first-group-only adapter returns a first-group-only proof"
                                        ),
                                    };
                                    let foreign = build_transfer_second_group_first_messages(
                                        self.pairing,
                                        Default::default(),
                                    );
                                    let variants = [
                                        build_schnorr_fs_transfer_proof(
                                            &adapter,
                                            &payload,
                                            SchnorrFsTransferProofOverrides {
                                                first_messages: Some(
                                                    TransferSecondGroupFirstMessages {
                                                        pk2: foreign.pk2.clone(),
                                                        d1: carried.d1.clone(),
                                                        d2: carried.d2.clone(),
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                        ),
                                        build_schnorr_fs_transfer_proof(
                                            &adapter,
                                            &payload,
                                            SchnorrFsTransferProofOverrides {
                                                first_messages: Some(
                                                    TransferSecondGroupFirstMessages {
                                                        pk2: carried.pk2.clone(),
                                                        d1: foreign.d1.clone(),
                                                        d2: carried.d2.clone(),
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                        ),
                                        build_schnorr_fs_transfer_proof(
                                            &adapter,
                                            &payload,
                                            SchnorrFsTransferProofOverrides {
                                                first_messages: Some(
                                                    TransferSecondGroupFirstMessages {
                                                        pk2: carried.pk2.clone(),
                                                        d1: carried.d1.clone(),
                                                        d2: foreign.d2.clone(),
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                        ),
                                    ];

                                    // Act
                                    for variant in &variants {
                                        let result = adapter.verify(
                                            VerifyParams,
                                            build_verify_payload(
                                                AlgebraicStatement::Transfer(&statement),
                                                variant,
                                                Default::default(),
                                            ),
                                        );

                                        // Assert
                                        assert_eq!(
                                            result.ok().map(|success| success.is_valid),
                                            Some(false)
                                        );
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_first_group_only_proof_with_a_changed_second_group_first_message_fails
}

/// Contract: entry `a_proof_from_a_witness_the_statement_does_not_hold_fails`;
///   `prove_mint` or `prove_transfer` over a witness other than the one
///   the statement holds returns `Ok`, and `verify` returns
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases and verifier forms of the mint block; the payload
///   with each witness overridden alone by a scalar other than the one
///   the default statement is formed from.
/// Act:     `prove_mint` or `prove_transfer`, then `verify` over the
///   statement and the proof.
/// Assert:  in place, the prover's result is `Ok` and
///   `result.ok().map(|success| success.is_valid)` equals `Some(false)`
///   for each witness; every factory result `is_ok()`.
/// Boundary: the private calls `prove_mint` or `prove_transfer` and
///   `verify` and the route of the mint block, run for each witness.
/// Mocked:  nothing.
#[test]
fn a_proof_from_a_witness_the_statement_does_not_hold_fails() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                for form in [
                                    VerifierGroupArithmetic::BothGroups,
                                    VerifierGroupArithmetic::FirstGroupOnly,
                                ] {
                                    let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                        build_schnorr_fs_delivery_proof(
                                            self.pairing,
                                            &*hashed.adapter,
                                            &encoded.adapter,
                                            &*random.adapter,
                                            SchnorrFsDeliveryProofConstructorParamsOverrides {
                                                verifier_form: Some(form),
                                            },
                                        );
                                    for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                        let statement = build_algebraic_mint_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicMintStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );
                                        let foreign =
                                            build_mint_responses(self.pairing, Default::default());
                                        let payloads = [
                                            build_prove_mint_payload::<P, F>(
                                                self.pairing,
                                                &statement,
                                                ProveMintPayloadOverrides {
                                                    master_scalar: Some(
                                                        build_master_scalar_components(
                                                            MasterScalarComponentsOverrides {
                                                                value: Some(build_secret(
                                                                    foreign.alpha.clone(),
                                                                    SecretConstructorParamsOverrides,
                                                                )),
                                                            },
                                                        ),
                                                    ),
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

                                        // Act
                                        for payload in payloads {
                                            let proof = adapter
                                                .prove_mint(ProveMintParams, payload)
                                                .expect(
                                                    "the prover does not check the statement holds",
                                                )
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
                                            assert_eq!(
                                                result.ok().map(|success| success.is_valid),
                                                Some(false)
                                            );
                                        }
                                    }
                                    for purpose in
                                        [TransferPurpose::Transfer, TransferPurpose::Grant]
                                    {
                                        let statement = build_algebraic_transfer_statement::<P, F>(
                                            self.pairing,
                                            AlgebraicTransferStatementOverrides {
                                                context: Some(build_delivery_context(
                                                    purpose,
                                                    Default::default(),
                                                )),
                                                ..Default::default()
                                            },
                                        );
                                        let foreign = build_transfer_responses(
                                            self.pairing,
                                            Default::default(),
                                        );
                                        let payloads = [
                                            build_prove_transfer_payload::<P, F>(
                                                self.pairing,
                                                &statement,
                                                ProveTransferPayloadOverrides {
                                                    seller_secrets: Some(
                                                        build_key_pair_components(
                                                            self.pairing,
                                                            KeyPairComponentsOverrides {
                                                                x: Some(build_secret(
                                                                    foreign.x.clone(),
                                                                    SecretConstructorParamsOverrides,
                                                                )),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    ..Default::default()
                                                },
                                            ),
                                            build_prove_transfer_payload::<P, F>(
                                                self.pairing,
                                                &statement,
                                                ProveTransferPayloadOverrides {
                                                    seller_secrets: Some(
                                                        build_key_pair_components(
                                                            self.pairing,
                                                            KeyPairComponentsOverrides {
                                                                y: Some(build_secret(
                                                                    foreign.y.clone(),
                                                                    SecretConstructorParamsOverrides,
                                                                )),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
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

                                        // Act
                                        for payload in payloads {
                                            let proof = adapter
                                                .prove_transfer(ProveTransferParams, payload)
                                                .expect(
                                                    "the prover does not check the statement holds",
                                                )
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
                                            assert_eq!(
                                                result.ok().map(|success| success.is_valid),
                                                Some(false)
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_proof_from_a_witness_the_statement_does_not_hold_fails
}

/// Contract: entry `a_proof_whose_second_group_errors_cancel_unweighted_fails_the_weighted_product`;
///   `verify` in the first-group-only form over a proof whose carried
///   `d1` is raised by a second-group point and whose carried `d2` is
///   lowered by the same point, its challenge recomputed over the
///   changed messages, returns `Ok(VerifySuccessReturn { is_valid:
///   false })`.
/// Arrange: the cases of the mint block in the first-group-only form;
///   the proof from the proof builder with `first_messages` overridden
///   by `d1` plus `δ · g2` and `d2` minus `δ · g2` for a constant `δ`,
///   for the mint and for the transfer proof; the product with every
///   weight one over the terms the first-group-only form states,
///   evaluated by `pairing_product_is_one`.
/// Act:     `verify(VerifyParams, payload)` over the crafted proof.
/// Assert:  in place, the unweighted product `is_one` is true, so the
///   crafted proof passes an unweighted check, and
///   `result.ok().map(|success| success.is_valid)` equals `Some(false)`;
///   every factory result `is_ok()`.
/// Boundary: the private call `verify` and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_proof_whose_second_group_errors_cancel_unweighted_fails_the_weighted_product() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingArithmetic, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                let adapter: SchnorrFsDeliveryProof<'_, P, E, F> =
                                    build_schnorr_fs_delivery_proof(
                                        self.pairing,
                                        &*hashed.adapter,
                                        &encoded.adapter,
                                        &*random.adapter,
                                        SchnorrFsDeliveryProofConstructorParamsOverrides {
                                            verifier_form: Some(
                                                VerifierGroupArithmetic::FirstGroupOnly,
                                            ),
                                        },
                                    );
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
                                let neg_g1 = self
                                    .pairing
                                    .neg_g1(NegG1Params, NegG1Payload { point: g1.clone() })
                                    .expect("a point negates")
                                    .negation;
                                let delta_scalar =
                                    build_mint_responses(self.pairing, Default::default()).alpha;
                                let delta = self
                                    .pairing
                                    .mul_g2(
                                        MulG2Params,
                                        MulG2Payload {
                                            point: g2.clone(),
                                            scalar: delta_scalar,
                                        },
                                    )
                                    .expect("a constant scalar multiplies the generator")
                                    .product;
                                let neg_delta = self
                                    .pairing
                                    .neg_g2(
                                        NegG2Params,
                                        NegG2Payload {
                                            point: delta.clone(),
                                        },
                                    )
                                    .expect("a point negates")
                                    .negation;
                                for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                    let statement = build_algebraic_mint_statement::<P, F>(
                                        self.pairing,
                                        AlgebraicMintStatementOverrides {
                                            context: Some(build_delivery_context(
                                                purpose,
                                                Default::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    let payload = build_prove_mint_payload::<P, F>(
                                        self.pairing,
                                        &statement,
                                        Default::default(),
                                    );
                                    let baseline = build_schnorr_fs_mint_proof(
                                        &adapter,
                                        &payload,
                                        Default::default(),
                                    );
                                    let carried = match &baseline {
                                        SchnorrFsProof::MintFirstGroupOnly {
                                            first_messages,
                                            ..
                                        } => first_messages,
                                        _ => panic!(
                                            "the first-group-only adapter returns a first-group-only proof"
                                        ),
                                    };
                                    let crafted = build_schnorr_fs_mint_proof(
                                        &adapter,
                                        &payload,
                                        SchnorrFsMintProofOverrides {
                                            first_messages: Some(MintSecondGroupFirstMessages {
                                                hpub: carried.hpub.clone(),
                                                d1: self
                                                    .pairing
                                                    .add_g2(
                                                        AddG2Params,
                                                        AddG2Payload {
                                                            left: carried.d1.clone(),
                                                            right: delta.clone(),
                                                        },
                                                    )
                                                    .expect("second-group points add")
                                                    .sum,
                                                d2: self
                                                    .pairing
                                                    .add_g2(
                                                        AddG2Params,
                                                        AddG2Payload {
                                                            left: carried.d2.clone(),
                                                            right: neg_delta.clone(),
                                                        },
                                                    )
                                                    .expect("second-group points add")
                                                    .sum,
                                            }),
                                            ..Default::default()
                                        },
                                    );
                                    let (challenge, responses, first_messages) = match &crafted {
                                        SchnorrFsProof::MintFirstGroupOnly {
                                            challenge,
                                            responses,
                                            first_messages,
                                        } => (challenge, responses, first_messages),
                                        _ => panic!(
                                            "the first-group-only adapter returns a first-group-only proof"
                                        ),
                                    };
                                    let n = self
                                        .pairing
                                        .neg_scalar(
                                            NegScalarParams,
                                            NegScalarPayload {
                                                scalar: challenge.clone(),
                                            },
                                        )
                                        .expect("a scalar negates")
                                        .negation;
                                    let combined = self
                                        .pairing
                                        .add_scalar(
                                            AddScalarParams,
                                            AddScalarPayload {
                                                left: responses.alpha.clone(),
                                                right: responses.sigma.clone(),
                                            },
                                        )
                                        .and_then(|sum| {
                                            self.pairing.add_scalar(
                                                AddScalarParams,
                                                AddScalarPayload {
                                                    left: sum.sum,
                                                    right: responses.r.clone(),
                                                },
                                            )
                                        })
                                        .expect("scalars add")
                                        .sum;
                                    let g1_times = |scalar: &P::Scalar| {
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: scalar.clone(),
                                                },
                                            )
                                            .expect("a scalar multiplies the generator")
                                            .product
                                    };
                                    let unweighted = self
                                        .pairing
                                        .pairing_product_is_one(
                                            PairingProductIsOneParams,
                                            PairingProductIsOnePayload {
                                                terms: vec![
                                                    PairingProductTerm {
                                                        g1: neg_g1.clone(),
                                                        g2: first_messages.hpub.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: neg_g1.clone(),
                                                        g2: first_messages.d1.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: neg_g1.clone(),
                                                        g2: first_messages.d2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&combined),
                                                        g2: g2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&n),
                                                        g2: statement.hpub.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&n),
                                                        g2: statement.envelope.d1.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&responses.sigma),
                                                        g2: statement.buyer_keys.pk2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&n),
                                                        g2: statement.envelope.d2.clone(),
                                                    },
                                                ],
                                            },
                                        )
                                        .expect("the product evaluates");

                                    // Act
                                    let result = adapter.verify(
                                        VerifyParams,
                                        build_verify_payload(
                                            AlgebraicStatement::Mint(&statement),
                                            &crafted,
                                            Default::default(),
                                        ),
                                    );

                                    // Assert
                                    assert!(unweighted.is_one);
                                    assert_eq!(
                                        result.ok().map(|success| success.is_valid),
                                        Some(false)
                                    );
                                }
                                for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                                    let statement = build_algebraic_transfer_statement::<P, F>(
                                        self.pairing,
                                        AlgebraicTransferStatementOverrides {
                                            context: Some(build_delivery_context(
                                                purpose,
                                                Default::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    let payload = build_prove_transfer_payload::<P, F>(
                                        self.pairing,
                                        &statement,
                                        Default::default(),
                                    );
                                    let baseline = build_schnorr_fs_transfer_proof(
                                        &adapter,
                                        &payload,
                                        Default::default(),
                                    );
                                    let carried = match &baseline {
                                        SchnorrFsProof::TransferFirstGroupOnly {
                                            first_messages,
                                            ..
                                        } => first_messages,
                                        _ => panic!(
                                            "the first-group-only adapter returns a first-group-only proof"
                                        ),
                                    };
                                    let crafted = build_schnorr_fs_transfer_proof(
                                        &adapter,
                                        &payload,
                                        SchnorrFsTransferProofOverrides {
                                            first_messages: Some(
                                                TransferSecondGroupFirstMessages {
                                                    pk2: carried.pk2.clone(),
                                                    d1: self
                                                        .pairing
                                                        .add_g2(
                                                            AddG2Params,
                                                            AddG2Payload {
                                                                left: carried.d1.clone(),
                                                                right: delta.clone(),
                                                            },
                                                        )
                                                        .expect("second-group points add")
                                                        .sum,
                                                    d2: self
                                                        .pairing
                                                        .add_g2(
                                                            AddG2Params,
                                                            AddG2Payload {
                                                                left: carried.d2.clone(),
                                                                right: neg_delta.clone(),
                                                            },
                                                        )
                                                        .expect("second-group points add")
                                                        .sum,
                                                },
                                            ),
                                            ..Default::default()
                                        },
                                    );
                                    let (challenge, responses, first_messages) = match &crafted {
                                        SchnorrFsProof::TransferFirstGroupOnly {
                                            challenge,
                                            responses,
                                            first_messages,
                                        } => (challenge, responses, first_messages),
                                        _ => panic!(
                                            "the first-group-only adapter returns a first-group-only proof"
                                        ),
                                    };
                                    let n = self
                                        .pairing
                                        .neg_scalar(
                                            NegScalarParams,
                                            NegScalarPayload {
                                                scalar: challenge.clone(),
                                            },
                                        )
                                        .expect("a scalar negates")
                                        .negation;
                                    let neg_y = self
                                        .pairing
                                        .neg_scalar(
                                            NegScalarParams,
                                            NegScalarPayload {
                                                scalar: responses.y.clone(),
                                            },
                                        )
                                        .expect("a scalar negates")
                                        .negation;
                                    let combined = self
                                        .pairing
                                        .add_scalar(
                                            AddScalarParams,
                                            AddScalarPayload {
                                                left: responses.y.clone(),
                                                right: responses.sigma.clone(),
                                            },
                                        )
                                        .and_then(|sum| {
                                            self.pairing.add_scalar(
                                                AddScalarParams,
                                                AddScalarPayload {
                                                    left: sum.sum,
                                                    right: responses.s.clone(),
                                                },
                                            )
                                        })
                                        .expect("scalars add")
                                        .sum;
                                    let g1_times = |scalar: &P::Scalar| {
                                        self.pairing
                                            .mul_g1(
                                                MulG1Params,
                                                MulG1Payload {
                                                    point: g1.clone(),
                                                    scalar: scalar.clone(),
                                                },
                                            )
                                            .expect("a scalar multiplies the generator")
                                            .product
                                    };
                                    let unweighted = self
                                        .pairing
                                        .pairing_product_is_one(
                                            PairingProductIsOneParams,
                                            PairingProductIsOnePayload {
                                                terms: vec![
                                                    PairingProductTerm {
                                                        g1: neg_g1.clone(),
                                                        g2: first_messages.pk2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: neg_g1.clone(),
                                                        g2: first_messages.d1.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: neg_g1.clone(),
                                                        g2: first_messages.d2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&combined),
                                                        g2: g2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&n),
                                                        g2: statement.seller_keys.pk2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&n),
                                                        g2: statement.new_envelope.d1.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&neg_y),
                                                        g2: statement.old_envelope.d1.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&responses.sigma),
                                                        g2: statement.buyer_keys.pk2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(&n),
                                                        g2: statement.new_envelope.d2.clone(),
                                                    },
                                                    PairingProductTerm {
                                                        g1: g1_times(challenge),
                                                        g2: statement.old_envelope.d2.clone(),
                                                    },
                                                ],
                                            },
                                        )
                                        .expect("the product evaluates");

                                    // Act
                                    let result = adapter.verify(
                                        VerifyParams,
                                        build_verify_payload(
                                            AlgebraicStatement::Transfer(&statement),
                                            &crafted,
                                            Default::default(),
                                        ),
                                    );

                                    // Assert
                                    assert!(unweighted.is_one);
                                    assert_eq!(
                                        result.ok().map(|success| success.is_valid),
                                        Some(false)
                                    );
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
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

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
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
    // a_proof_whose_second_group_errors_cancel_unweighted_fails_the_weighted_product
}
