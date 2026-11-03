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
    CanonicalFieldKind, CanonicalFieldValue, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, EncodeParams, ICanonicalField, IDecoderAdapter, IEncoderAdapter,
    build_create_encoding_params, build_from_field_params,
};
use envelope::{EnvelopeCoins, KeyPairComponents, build_envelope_coins, build_key_pair_components};
use hash_to_scalar::{
    CreateHashToScalarParams, DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTag,
    DomainTagConstructorParamsOverrides, HashToScalarIdentifier, IHashToScalarAdapter,
    build_domain_tag, expected_uniform_bytes,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::MasterScalarComponents;
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    CreatePairingParamsOverrides, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MsmG1Params, MsmG1Payload, MsmG1Term, MsmG2Params,
    MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, NegG1Params,
    NegG1Payload, NegG2Params, NegG2Payload, NegScalarParams, NegScalarPayload, PAIRING_CONCRETES,
    PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductTerm,
    SampleUniformScalarParams, SampleUniformScalarPayload, VerifierGroupArithmetic,
    build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::challenge::provides::SCHNORR_FS_CHALLENGE_TAG;
use super::provides::{
    ISchnorrFsCaseConsumer, SchnorrFsCaseParamsOverrides, SchnorrFsCasePayload,
    SchnorrFsCaseSuccessReturn, SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParams,
    SchnorrFsMintProofOverrides, SchnorrFsProof, SchnorrFsTransferProofOverrides,
    build_schnorr_fs_case_deps, build_schnorr_fs_case_params, build_schnorr_fs_case_payload,
    build_schnorr_fs_mint_proof, build_schnorr_fs_transfer_proof, run_schnorr_fs_case,
};
use crate::factory::provides::{
    AlgebraicMintStatementOverrides, AlgebraicStatement, AlgebraicTransferStatementOverrides,
    DeliveryContextOverrides, IDeliveryProofAdapter, MintResponses, MintSecondGroupFirstMessages,
    ProofComponentsParams, ProofFromComponentsParams, ProveMintParams, ProveMintPayloadOverrides,
    ProveTransferParams, ProveTransferPayloadOverrides, TransferResponses,
    TransferSecondGroupFirstMessages, VerifyParams, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_delivery_context, build_mint_responses,
    build_proof_components_payload, build_proof_from_components_payload, build_prove_mint_payload,
    build_prove_transfer_payload, build_transfer_responses, build_verify_payload,
};
use crate::mint_statement::provides::{
    MintFirstMessages, MintPurpose, MintStatementOverrides, TransferPurpose, build_mint_statement,
    build_mint_statement_description,
};
use crate::transfer_statement::provides::{
    TransferFirstMessages, TransferStatementOverrides, build_transfer_statement,
    build_transfer_statement_description,
};

/// Contract: entry `a_mint_proof_verifies_in_both_forms`; `prove_mint` over
///   the statement's witnesses followed by `verify` over the same statement
///   returns `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: every case of the harness in each verifier form, and for each of
///   `MintPurpose::Mint` and `MintPurpose::Replacement` the statement
///   `build_algebraic_mint_statement` builds over a context from
///   `build_delivery_context` with that purpose, its values distinct at every
///   position so that a swap of same-kind fields fails, and
///   `build_prove_mint_payload` over it.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `prove_mint(ProveMintParams, payload)`, then `verify(VerifyParams,
///   payload)` over `AlgebraicStatement::Mint(&statement)` and the proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals `Some(true)`.
/// Boundary: the private calls `prove_mint` and `verify` and the route the
///   private surface states.
/// Mocked:  nothing; the block proves the prover and the verifier agree on
///   the composition of every function on their route.
#[test]
fn a_mint_proof_verifies_in_both_forms() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");
                for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                    let statement = build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let payload = build_prove_mint_payload(pairing, &statement, Default::default());
                    let proved = adapter
                        .prove_mint(ProveMintParams, payload)
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry `a_transfer_proof_verifies_in_both_forms`;
///   `prove_transfer` over the statement's witnesses followed by `verify`
///   over the same statement returns `Ok(VerifySuccessReturn { is_valid:
///   true })`.
/// Arrange: the cases and verifier forms of the mint block, and for each of
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant` the statement
///   `build_algebraic_transfer_statement` builds over a context with that
///   purpose, its values distinct at every position, and
///   `build_prove_transfer_payload` over it.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `prove_transfer(ProveTransferParams, payload)`, then `verify(
///   VerifyParams, payload)` over `AlgebraicStatement::Transfer(&statement)`
///   and the proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals `Some(true)`.
/// Boundary: the private calls `prove_transfer` and `verify` and the route of
///   the mint block.
/// Mocked:  nothing.
#[test]
fn a_transfer_proof_verifies_in_both_forms() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");
                for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                    let statement = build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let payload =
                        build_prove_transfer_payload(pairing, &statement, Default::default());
                    let proved = adapter
                        .prove_transfer(ProveTransferParams, payload)
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry `a_proof_rebuilt_from_its_components_verifies`; the
///   components `proof_components` reads from a proof, rebuilt by
///   `proof_from_components` under an adapter of the same form, verify.
/// Arrange: the cases and verifier forms of the mint block for the mint
///   statement and for the transfer statement.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `proof_components`, `proof_from_components`, then `verify` over the
///   rebuilt proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals `Some(true)`.
/// Boundary: the private calls `prove_mint` or `prove_transfer`,
///   `proof_components`, `proof_from_components`, and `verify` and the route
///   of the mint block.
/// Mocked:  nothing.
#[test]
fn a_proof_rebuilt_from_its_components_verifies() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");

                let mint_statement = build_algebraic_mint_statement(pairing, Default::default());
                let mint_payload =
                    build_prove_mint_payload(pairing, &mint_statement, Default::default());
                let mint_proof = adapter
                    .prove_mint(ProveMintParams, mint_payload)
                    .expect("prove_mint returns the proof")
                    .proof;
                let components = adapter
                    .proof_components(
                        ProofComponentsParams,
                        build_proof_components_payload(&mint_proof, Default::default()),
                    )
                    .expect("proof_components clones the named fields")
                    .components;
                let rebuilt = adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        build_proof_from_components_payload(components, Default::default()),
                    )
                    .expect("proof_from_components admits the same form")
                    .proof;
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Mint(&mint_statement),
                        &rebuilt,
                        Default::default(),
                    ),
                );
                assert_eq!(result.ok().map(|success| success.is_valid), Some(true));

                let transfer_statement =
                    build_algebraic_transfer_statement(pairing, Default::default());
                let transfer_payload =
                    build_prove_transfer_payload(pairing, &transfer_statement, Default::default());
                let transfer_proof = adapter
                    .prove_transfer(ProveTransferParams, transfer_payload)
                    .expect("prove_transfer returns the proof")
                    .proof;
                let components = adapter
                    .proof_components(
                        ProofComponentsParams,
                        build_proof_components_payload(&transfer_proof, Default::default()),
                    )
                    .expect("proof_components clones the named fields")
                    .components;
                let rebuilt = adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        build_proof_from_components_payload(components, Default::default()),
                    )
                    .expect("proof_from_components admits the same form")
                    .proof;
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Transfer(&transfer_statement),
                        &rebuilt,
                        Default::default(),
                    ),
                );
                assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry
///   `a_mint_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding`;
///   the proof's challenge equals the oracle scalar of the encoder's bytes
///   for the transcript the statement and the recomputed first messages
///   define, under the proof's tag.
/// Arrange: the cases, verifier forms, and purposes of the mint block; the
///   first messages recomputed from the proof's responses and challenge
///   through `mul_g1`, `mul_g2`, `msm_g1`, and `msm_g2` by the verifier's
///   equations, the second-group ones taken from the proof in the
///   first-group-only form; the transcript `build_mint_statement` builds with
///   the statement's context fields cloned and those first messages; the
///   bytes from the received encoder's `encode` under `EncodeParams` over
///   `build_mint_statement_description(&pairing, Default::default())`; the
///   tag `build_domain_tag` builds from `SCHNORR_FS_CHALLENGE_TAG`; the
///   oracle scalar sampled from `expected_uniform_bytes(&tag, &bytes,
///   P::Scalar::UNIFORM_BYTES_LENGTH)`.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `prove_mint(ProveMintParams, payload)`.
/// Assert:  the encoding of the proof's challenge through the received
///   pairing's `encode_scalar` equals the encoding of the oracle scalar by
///   `assert_eq!`.
/// Boundary: the private call `prove_mint` and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_mint_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let neg_scalar = |scalar: P::Scalar| -> P::Scalar {
                let Ok(negated) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
                negated.negation
            };
            let msm_g1 = |terms: Vec<(P::G1, P::Scalar)>| -> P::G1 {
                let Ok(success) = pairing.msm_g1(
                    MsmG1Params,
                    MsmG1Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG1Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };
            let msm_g2 = |terms: Vec<(P::G2, P::Scalar)>| -> P::G2 {
                let Ok(success) = pairing.msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG2Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let tag: DomainTag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });

            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");
                for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                    let statement = build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let payload = build_prove_mint_payload(pairing, &statement, Default::default());
                    let proof = adapter
                        .prove_mint(ProveMintParams, payload)
                        .expect("prove_mint returns the proof")
                        .proof;
                    let (challenge_scalar, responses, carried) = match &proof {
                        SchnorrFsProof::MintBothGroups {
                            challenge,
                            responses,
                        } => (challenge, responses, None),
                        SchnorrFsProof::MintFirstGroupOnly {
                            challenge,
                            responses,
                            first_messages,
                        } => (challenge, responses, Some(first_messages)),
                        _ => panic!("the proof is a mint proof"),
                    };
                    let n = neg_scalar(challenge_scalar.clone());
                    let t_hpub = carried.map(|m| m.hpub.clone()).unwrap_or_else(|| {
                        msm_g2(vec![
                            (g2.point.clone(), responses.alpha.clone()),
                            (statement.hpub.clone(), n.clone()),
                        ])
                    });
                    let t_c1 = msm_g1(vec![
                        (g1.point.clone(), responses.rho.clone()),
                        (statement.envelope.c1.clone(), n.clone()),
                    ]);
                    let t_c2 = msm_g1(vec![
                        (g1.point.clone(), responses.alpha.clone()),
                        (statement.identity_element.clone(), responses.r.clone()),
                        (statement.buyer_keys.pk1.clone(), responses.rho.clone()),
                        (statement.envelope.c2.clone(), n.clone()),
                    ]);
                    let t_d1 = carried.map(|m| m.d1.clone()).unwrap_or_else(|| {
                        msm_g2(vec![
                            (g2.point.clone(), responses.sigma.clone()),
                            (statement.envelope.d1.clone(), n.clone()),
                        ])
                    });
                    let t_d2 = carried.map(|m| m.d2.clone()).unwrap_or_else(|| {
                        msm_g2(vec![
                            (g2.point.clone(), responses.r.clone()),
                            (statement.buyer_keys.pk2.clone(), responses.sigma.clone()),
                            (statement.envelope.d2.clone(), n.clone()),
                        ])
                    });
                    let transcript = build_mint_statement(
                        pairing,
                        MintStatementOverrides {
                            suite_identifier: Some(statement.context.suite_identifier.clone()),
                            chain: Some(statement.context.chain.clone()),
                            entitlement_contract: Some(
                                statement.context.entitlement_contract.clone(),
                            ),
                            asset_root: Some(statement.context.asset_root),
                            parameter_set_identifier: Some(
                                statement.context.parameter_set_identifier.clone(),
                            ),
                            source_entitlement: Some(statement.context.source_entitlement.clone()),
                            target_entitlement: Some(statement.context.target_entitlement.clone()),
                            old_interval: Some(statement.context.old_interval.clone()),
                            new_interval: Some(statement.context.new_interval.clone()),
                            purpose: Some(statement.context.purpose),
                            seller: Some(statement.context.seller.clone()),
                            buyer: Some(statement.context.buyer.clone()),
                            buyer_keys: Some(envelope::PublicKeysComponents {
                                pk1: statement.buyer_keys.pk1.clone(),
                                pk2: statement.buyer_keys.pk2.clone(),
                            }),
                            envelope: Some(envelope::EnvelopeComponents {
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
                    let description =
                        build_mint_statement_description::<P, F>(pairing, Default::default());
                    let bytes = encoder
                        .encode(
                            EncodeParams {
                                description: &description,
                            },
                            &transcript,
                        )
                        .expect("encoding the transcript succeeds")
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
                    .expect("the oracle bytes sample a scalar")
                    .scalar
                    .expose()
                    .clone();
                    assert_eq!(
                        encode_scalar(challenge_scalar.clone()),
                        encode_scalar(oracle)
                    );
                }
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry
///   `a_transfer_proofs_challenge_is_the_tagged_hash_of_its_transcripts_
///   encoding`; the proof's challenge equals the oracle scalar of the
///   encoder's bytes for the transfer transcript the statement and the
///   recomputed first messages define, under the proof's tag.
/// Arrange: the cases and purposes of the transfer block; the six first
///   messages recomputed as the mint block states; the transcript
///   `build_transfer_statement` builds; the bytes from `encode` over
///   `build_transfer_statement_description(&pairing, Default::default())`;
///   the tag and the oracle scalar as the mint block states.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  the encoding of the proof's challenge equals the encoding of the
///   oracle scalar by `assert_eq!`.
/// Boundary: the private call `prove_transfer` and the route of the mint
///   block.
/// Mocked:  nothing.
#[test]
fn a_transfer_proofs_challenge_is_the_tagged_hash_of_its_transcripts_encoding() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let neg_scalar = |scalar: P::Scalar| -> P::Scalar {
                let Ok(negated) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
                negated.negation
            };
            let msm_g1 = |terms: Vec<(P::G1, P::Scalar)>| -> P::G1 {
                let Ok(success) = pairing.msm_g1(
                    MsmG1Params,
                    MsmG1Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG1Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };
            let msm_g2 = |terms: Vec<(P::G2, P::Scalar)>| -> P::G2 {
                let Ok(success) = pairing.msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG2Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let tag: DomainTag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });

            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");
                for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                    let statement = build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let payload =
                        build_prove_transfer_payload(pairing, &statement, Default::default());
                    let proof = adapter
                        .prove_transfer(ProveTransferParams, payload)
                        .expect("prove_transfer returns the proof")
                        .proof;
                    let (challenge_scalar, responses, carried) = match &proof {
                        SchnorrFsProof::TransferBothGroups {
                            challenge,
                            responses,
                        } => (challenge, responses, None),
                        SchnorrFsProof::TransferFirstGroupOnly {
                            challenge,
                            responses,
                            first_messages,
                        } => (challenge, responses, Some(first_messages)),
                        _ => panic!("the proof is a transfer proof"),
                    };
                    let n = neg_scalar(challenge_scalar.clone());
                    let delta_c2 = {
                        let Ok(negated) = pairing.neg_g1(
                            NegG1Params,
                            NegG1Payload {
                                point: statement.old_envelope.c2.clone(),
                            },
                        );
                        let Ok(sum) = pairing.add_g1(
                            AddG1Params,
                            AddG1Payload {
                                left: statement.new_envelope.c2.clone(),
                                right: negated.negation,
                            },
                        );
                        sum.sum
                    };
                    let delta_d2 = {
                        let Ok(negated) = pairing.neg_g2(
                            NegG2Params,
                            NegG2Payload {
                                point: statement.old_envelope.d2.clone(),
                            },
                        );
                        let Ok(sum) = pairing.add_g2(
                            AddG2Params,
                            AddG2Payload {
                                left: statement.new_envelope.d2.clone(),
                                right: negated.negation,
                            },
                        );
                        sum.sum
                    };
                    let t_pk1 = msm_g1(vec![
                        (g1.point.clone(), responses.x.clone()),
                        (statement.seller_keys.pk1.clone(), n.clone()),
                    ]);
                    let t_c1 = msm_g1(vec![
                        (g1.point.clone(), responses.rho.clone()),
                        (statement.new_envelope.c1.clone(), n.clone()),
                    ]);
                    let t_c2 = msm_g1(vec![
                        (
                            statement.old_envelope.c1.clone(),
                            neg_scalar(responses.x.clone()),
                        ),
                        (statement.identity_element.clone(), responses.s.clone()),
                        (statement.buyer_keys.pk1.clone(), responses.rho.clone()),
                        (delta_c2, n.clone()),
                    ]);
                    let t_pk2 = carried.map(|m| m.pk2.clone()).unwrap_or_else(|| {
                        msm_g2(vec![
                            (g2.point.clone(), responses.y.clone()),
                            (statement.seller_keys.pk2.clone(), n.clone()),
                        ])
                    });
                    let t_d1 = carried.map(|m| m.d1.clone()).unwrap_or_else(|| {
                        msm_g2(vec![
                            (g2.point.clone(), responses.sigma.clone()),
                            (statement.new_envelope.d1.clone(), n.clone()),
                        ])
                    });
                    let t_d2 = carried.map(|m| m.d2.clone()).unwrap_or_else(|| {
                        msm_g2(vec![
                            (
                                statement.old_envelope.d1.clone(),
                                neg_scalar(responses.y.clone()),
                            ),
                            (g2.point.clone(), responses.s.clone()),
                            (statement.buyer_keys.pk2.clone(), responses.sigma.clone()),
                            (delta_d2, n.clone()),
                        ])
                    });
                    let transcript = build_transfer_statement(
                        pairing,
                        TransferStatementOverrides {
                            suite_identifier: Some(statement.context.suite_identifier.clone()),
                            chain: Some(statement.context.chain.clone()),
                            entitlement_contract: Some(
                                statement.context.entitlement_contract.clone(),
                            ),
                            asset_root: Some(statement.context.asset_root),
                            parameter_set_identifier: Some(
                                statement.context.parameter_set_identifier.clone(),
                            ),
                            source_entitlement: Some(statement.context.source_entitlement.clone()),
                            target_entitlement: Some(statement.context.target_entitlement.clone()),
                            old_interval: Some(statement.context.old_interval.clone()),
                            new_interval: Some(statement.context.new_interval.clone()),
                            purpose: Some(statement.context.purpose),
                            seller: Some(statement.context.seller.clone()),
                            buyer: Some(statement.context.buyer.clone()),
                            seller_keys: Some(envelope::PublicKeysComponents {
                                pk1: statement.seller_keys.pk1.clone(),
                                pk2: statement.seller_keys.pk2.clone(),
                            }),
                            buyer_keys: Some(envelope::PublicKeysComponents {
                                pk1: statement.buyer_keys.pk1.clone(),
                                pk2: statement.buyer_keys.pk2.clone(),
                            }),
                            old_envelope: Some(envelope::EnvelopeComponents {
                                c1: statement.old_envelope.c1.clone(),
                                c2: statement.old_envelope.c2.clone(),
                                d1: statement.old_envelope.d1.clone(),
                                d2: statement.old_envelope.d2.clone(),
                            }),
                            new_envelope: Some(envelope::EnvelopeComponents {
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
                    let description =
                        build_transfer_statement_description::<P, F>(pairing, Default::default());
                    let bytes = encoder
                        .encode(
                            EncodeParams {
                                description: &description,
                            },
                            &transcript,
                        )
                        .expect("encoding the transcript succeeds")
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
                    .expect("the oracle bytes sample a scalar")
                    .scalar
                    .expose()
                    .clone();
                    assert_eq!(
                        encode_scalar(challenge_scalar.clone()),
                        encode_scalar(oracle)
                    );
                }
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry `a_proof_fails_against_a_statement_differing_in_one_
///   field`; `verify` over a statement that differs from the proof's
///   statement in one field returns `Ok(VerifySuccessReturn { is_valid:
///   false })`.
/// Arrange: the cases and verifier forms of the mint block; the proof formed
///   for the default statement; for each field of `AlgebraicMintStatement`
///   and of `AlgebraicTransferStatement`, the context fields included, the
///   variant statement overridden in that field alone with a value from that
///   field's own builder distinct from the default, so the baseline verifies
///   and each variant is the only difference.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `verify(VerifyParams, payload)` over the variant statement and the
///   baseline proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` for each variant.
/// Boundary: the private calls `prove_mint` or `prove_transfer` and `verify`
///   and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_proof_fails_against_a_statement_differing_in_one_field() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            let distinct_value = |kind: CanonicalFieldKind| -> CanonicalFieldValue {
                match kind {
                    CanonicalFieldKind::FixedBytes32 => {
                        CanonicalFieldValue::FixedBytes32([0xEE; 32])
                    }
                    CanonicalFieldKind::Unsigned16 => CanonicalFieldValue::Unsigned16(0xEEE1),
                    CanonicalFieldKind::Unsigned32 => CanonicalFieldValue::Unsigned32(0xEEEE_EEE2),
                    CanonicalFieldKind::Unsigned64 => {
                        CanonicalFieldValue::Unsigned64(0xEEEE_EEEE_EEEE_EEE3)
                    }
                    CanonicalFieldKind::Text => CanonicalFieldValue::Text("variant".to_string()),
                    CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(vec![0xEE]),
                    CanonicalFieldKind::FixedBytes20 => {
                        CanonicalFieldValue::FixedBytes20([0xEE; 20])
                    }
                    CanonicalFieldKind::Unsigned256 => CanonicalFieldValue::Unsigned256([0xEE; 32]),
                }
            };
            let other_chain = || -> F::ChainIdentifier {
                F::ChainIdentifier::from_field(
                    build_from_field_params(Default::default()),
                    distinct_value(F::ChainIdentifier::KIND),
                )
                .expect("a constant of the declared kind is admitted")
                .value
            };
            let other_identity = || -> F::Identity {
                F::Identity::from_field(
                    build_from_field_params(Default::default()),
                    distinct_value(F::Identity::KIND),
                )
                .expect("a constant of the declared kind is admitted")
                .value
            };
            let other_entitlement = || -> F::Entitlement {
                F::Entitlement::from_field(
                    build_from_field_params(Default::default()),
                    distinct_value(F::Entitlement::KIND),
                )
                .expect("a constant of the declared kind is admitted")
                .value
            };
            let other_interval = || -> F::Interval {
                F::Interval::from_field(
                    build_from_field_params(Default::default()),
                    distinct_value(F::Interval::KIND),
                )
                .expect("a constant of the declared kind is admitted")
                .value
            };
            let neg_g1 = |point: P::G1| -> P::G1 {
                let Ok(negated) = pairing.neg_g1(NegG1Params, NegG1Payload { point });
                negated.negation
            };
            let neg_g2 = |point: P::G2| -> P::G2 {
                let Ok(negated) = pairing.neg_g2(NegG2Params, NegG2Payload { point });
                negated.negation
            };

            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");

                let mint_statement = build_algebraic_mint_statement(pairing, Default::default());
                let mint_payload =
                    build_prove_mint_payload(pairing, &mint_statement, Default::default());
                let mint_proof = adapter
                    .prove_mint(ProveMintParams, mint_payload)
                    .expect("prove_mint returns the proof")
                    .proof;
                let mint_variants = vec![
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    suite_identifier: Some(build_suite_identifier(
                                        SuiteIdentifierConstructorParamsOverrides {
                                            version: Some(0xEEEE),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    chain: Some(other_chain()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    entitlement_contract: Some(other_identity()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                                        bytes: Some([0xEE; 32]),
                                    })),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    parameter_set_identifier: Some(build_parameter_set_identifier(
                                        ParameterSetIdentifierConstructorParamsOverrides {
                                            bytes: Some([0xEE; PARAMETER_SET_IDENTIFIER_LENGTH]),
                                        },
                                    )),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    source_entitlement: Some(other_entitlement()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    target_entitlement: Some(other_entitlement()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    old_interval: Some(other_interval()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    new_interval: Some(other_interval()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Replacement,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    seller: Some(other_identity()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    buyer: Some(other_identity()),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                MintPurpose::Mint,
                                DeliveryContextOverrides {
                                    expiry: Some(0xEEEE_EEEE),
                                    ..Default::default()
                                },
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            hpub: Some(neg_g2(mint_statement.hpub.clone())),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            identity_element: Some(neg_g1(mint_statement.identity_element.clone())),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            buyer_keys: Some(envelope::PublicKeysComponents {
                                pk1: neg_g1(mint_statement.buyer_keys.pk1.clone()),
                                pk2: mint_statement.buyer_keys.pk2.clone(),
                            }),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            envelope: Some(envelope::EnvelopeComponents {
                                c1: neg_g1(mint_statement.envelope.c1.clone()),
                                c2: mint_statement.envelope.c2.clone(),
                                d1: mint_statement.envelope.d1.clone(),
                                d2: mint_statement.envelope.d2.clone(),
                            }),
                            ..Default::default()
                        },
                    ),
                ];
                for variant in &mint_variants {
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Mint(variant),
                            &mint_proof,
                            Default::default(),
                        ),
                    );
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }

                let transfer_statement =
                    build_algebraic_transfer_statement(pairing, Default::default());
                let transfer_payload =
                    build_prove_transfer_payload(pairing, &transfer_statement, Default::default());
                let transfer_proof = adapter
                    .prove_transfer(ProveTransferParams, transfer_payload)
                    .expect("prove_transfer returns the proof")
                    .proof;
                let transfer_variants = vec![
                    build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                TransferPurpose::Grant,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            identity_element: Some(neg_g1(
                                transfer_statement.identity_element.clone(),
                            )),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            seller_keys: Some(envelope::PublicKeysComponents {
                                pk1: neg_g1(transfer_statement.seller_keys.pk1.clone()),
                                pk2: transfer_statement.seller_keys.pk2.clone(),
                            }),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            buyer_keys: Some(envelope::PublicKeysComponents {
                                pk1: neg_g1(transfer_statement.buyer_keys.pk1.clone()),
                                pk2: transfer_statement.buyer_keys.pk2.clone(),
                            }),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            old_envelope: Some(envelope::EnvelopeComponents {
                                c1: neg_g1(transfer_statement.old_envelope.c1.clone()),
                                c2: transfer_statement.old_envelope.c2.clone(),
                                d1: transfer_statement.old_envelope.d1.clone(),
                                d2: transfer_statement.old_envelope.d2.clone(),
                            }),
                            ..Default::default()
                        },
                    ),
                    build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            new_envelope: Some(envelope::EnvelopeComponents {
                                c1: neg_g1(transfer_statement.new_envelope.c1.clone()),
                                c2: transfer_statement.new_envelope.c2.clone(),
                                d1: transfer_statement.new_envelope.d1.clone(),
                                d2: transfer_statement.new_envelope.d2.clone(),
                            }),
                            ..Default::default()
                        },
                    ),
                ];
                for variant in &transfer_variants {
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Transfer(variant),
                            &transfer_proof,
                            Default::default(),
                        ),
                    );
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry `a_proof_with_a_changed_response_or_challenge_fails`;
///   `verify` over a proof whose challenge or one response is another scalar
///   returns `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases and verifier forms of the mint block; the proof from
///   `build_schnorr_fs_mint_proof` or `build_schnorr_fs_transfer_proof` with
///   no overrides verifying, and a variant for the challenge and for each
///   response overridden alone by a scalar other than the baseline's.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `verify(VerifyParams, payload)` over each variant proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` for each variant.
/// Boundary: the private call `verify` and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_proof_with_a_changed_response_or_challenge_fails() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");
                let other = build_mint_responses(pairing, Default::default());

                let mint_statement = build_algebraic_mint_statement(pairing, Default::default());
                let mint_payload =
                    build_prove_mint_payload(pairing, &mint_statement, Default::default());
                let baseline = build_schnorr_fs_mint_proof(
                    &adapter,
                    &mint_payload,
                    SchnorrFsMintProofOverrides::default(),
                );
                let baseline_responses = match &baseline {
                    SchnorrFsProof::MintBothGroups { responses, .. }
                    | SchnorrFsProof::MintFirstGroupOnly { responses, .. } => MintResponses {
                        alpha: responses.alpha.clone(),
                        r: responses.r.clone(),
                        rho: responses.rho.clone(),
                        sigma: responses.sigma.clone(),
                    },
                    _ => panic!("the proof is a mint proof"),
                };
                let mut mint_variants = vec![build_schnorr_fs_mint_proof(
                    &adapter,
                    &mint_payload,
                    SchnorrFsMintProofOverrides {
                        challenge: Some(other.alpha.clone()),
                        ..Default::default()
                    },
                )];
                for (index, replaced) in [
                    other.alpha.clone(),
                    other.r.clone(),
                    other.rho.clone(),
                    other.sigma.clone(),
                ]
                .into_iter()
                .enumerate()
                {
                    let mut responses = MintResponses {
                        alpha: baseline_responses.alpha.clone(),
                        r: baseline_responses.r.clone(),
                        rho: baseline_responses.rho.clone(),
                        sigma: baseline_responses.sigma.clone(),
                    };
                    match index {
                        0 => responses.alpha = replaced,
                        1 => responses.r = replaced,
                        2 => responses.rho = replaced,
                        _ => responses.sigma = replaced,
                    }
                    mint_variants.push(build_schnorr_fs_mint_proof(
                        &adapter,
                        &mint_payload,
                        SchnorrFsMintProofOverrides {
                            responses: Some(responses),
                            ..Default::default()
                        },
                    ));
                }
                for variant in &mint_variants {
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Mint(&mint_statement),
                            variant,
                            Default::default(),
                        ),
                    );
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }

                let transfer_statement =
                    build_algebraic_transfer_statement(pairing, Default::default());
                let transfer_payload =
                    build_prove_transfer_payload(pairing, &transfer_statement, Default::default());
                let baseline = build_schnorr_fs_transfer_proof(
                    &adapter,
                    &transfer_payload,
                    SchnorrFsTransferProofOverrides::default(),
                );
                let baseline_responses = match &baseline {
                    SchnorrFsProof::TransferBothGroups { responses, .. }
                    | SchnorrFsProof::TransferFirstGroupOnly { responses, .. } => {
                        TransferResponses {
                            x: responses.x.clone(),
                            y: responses.y.clone(),
                            s: responses.s.clone(),
                            rho: responses.rho.clone(),
                            sigma: responses.sigma.clone(),
                        }
                    }
                    _ => panic!("the proof is a transfer proof"),
                };
                let others = build_transfer_responses(pairing, Default::default());
                let mut transfer_variants = vec![build_schnorr_fs_transfer_proof(
                    &adapter,
                    &transfer_payload,
                    SchnorrFsTransferProofOverrides {
                        challenge: Some(others.x.clone()),
                        ..Default::default()
                    },
                )];
                for (index, replaced) in [
                    others.x.clone(),
                    others.y.clone(),
                    others.s.clone(),
                    others.rho.clone(),
                    others.sigma.clone(),
                ]
                .into_iter()
                .enumerate()
                {
                    let mut responses = TransferResponses {
                        x: baseline_responses.x.clone(),
                        y: baseline_responses.y.clone(),
                        s: baseline_responses.s.clone(),
                        rho: baseline_responses.rho.clone(),
                        sigma: baseline_responses.sigma.clone(),
                    };
                    match index {
                        0 => responses.x = replaced,
                        1 => responses.y = replaced,
                        2 => responses.s = replaced,
                        3 => responses.rho = replaced,
                        _ => responses.sigma = replaced,
                    }
                    transfer_variants.push(build_schnorr_fs_transfer_proof(
                        &adapter,
                        &transfer_payload,
                        SchnorrFsTransferProofOverrides {
                            responses: Some(responses),
                            ..Default::default()
                        },
                    ));
                }
                for variant in &transfer_variants {
                    let result = adapter.verify(
                        VerifyParams,
                        build_verify_payload(
                            AlgebraicStatement::Transfer(&transfer_statement),
                            variant,
                            Default::default(),
                        ),
                    );
                    assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                }
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry
///   `a_first_group_only_proof_with_a_changed_second_group_first_message_
///   fails`; `verify` in the first-group-only form over a proof whose carried
///   second-group first message is another point, its challenge recomputed
///   over the changed transcript, returns `Ok(VerifySuccessReturn {
///   is_valid: false })`.
/// Arrange: the cases of the mint block in the first-group-only form; the
///   proof from the proof builder with `first_messages` overridden in each
///   carried message alone by a point other than the baseline's, for the
///   mint and for the transfer proof.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `verify(VerifyParams, payload)` over each variant proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` for each variant.
/// Boundary: the private call `verify` and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_first_group_only_proof_with_a_changed_second_group_first_message_fails() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            let neg_g2 = |point: P::G2| -> P::G2 {
                let Ok(negated) = pairing.neg_g2(NegG2Params, NegG2Payload { point });
                negated.negation
            };
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");

            let mint_statement = build_algebraic_mint_statement(pairing, Default::default());
            let mint_payload =
                build_prove_mint_payload(pairing, &mint_statement, Default::default());
            let baseline = build_schnorr_fs_mint_proof(
                &adapter,
                &mint_payload,
                SchnorrFsMintProofOverrides::default(),
            );
            let carried = match &baseline {
                SchnorrFsProof::MintFirstGroupOnly { first_messages, .. } => {
                    MintSecondGroupFirstMessages {
                        hpub: first_messages.hpub.clone(),
                        d1: first_messages.d1.clone(),
                        d2: first_messages.d2.clone(),
                    }
                }
                _ => panic!("the proof is the mint first-group-only variant"),
            };
            for overridden in [
                MintSecondGroupFirstMessages {
                    hpub: neg_g2(carried.hpub.clone()),
                    d1: carried.d1.clone(),
                    d2: carried.d2.clone(),
                },
                MintSecondGroupFirstMessages {
                    hpub: carried.hpub.clone(),
                    d1: neg_g2(carried.d1.clone()),
                    d2: carried.d2.clone(),
                },
                MintSecondGroupFirstMessages {
                    hpub: carried.hpub.clone(),
                    d1: carried.d1.clone(),
                    d2: neg_g2(carried.d2.clone()),
                },
            ] {
                let variant = build_schnorr_fs_mint_proof(
                    &adapter,
                    &mint_payload,
                    SchnorrFsMintProofOverrides {
                        first_messages: Some(overridden),
                        ..Default::default()
                    },
                );
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Mint(&mint_statement),
                        &variant,
                        Default::default(),
                    ),
                );
                assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
            }

            let transfer_statement =
                build_algebraic_transfer_statement(pairing, Default::default());
            let transfer_payload =
                build_prove_transfer_payload(pairing, &transfer_statement, Default::default());
            let baseline = build_schnorr_fs_transfer_proof(
                &adapter,
                &transfer_payload,
                SchnorrFsTransferProofOverrides::default(),
            );
            let carried = match &baseline {
                SchnorrFsProof::TransferFirstGroupOnly { first_messages, .. } => {
                    TransferSecondGroupFirstMessages {
                        pk2: first_messages.pk2.clone(),
                        d1: first_messages.d1.clone(),
                        d2: first_messages.d2.clone(),
                    }
                }
                _ => panic!("the proof is the transfer first-group-only variant"),
            };
            for overridden in [
                TransferSecondGroupFirstMessages {
                    pk2: neg_g2(carried.pk2.clone()),
                    d1: carried.d1.clone(),
                    d2: carried.d2.clone(),
                },
                TransferSecondGroupFirstMessages {
                    pk2: carried.pk2.clone(),
                    d1: neg_g2(carried.d1.clone()),
                    d2: carried.d2.clone(),
                },
                TransferSecondGroupFirstMessages {
                    pk2: carried.pk2.clone(),
                    d1: carried.d1.clone(),
                    d2: neg_g2(carried.d2.clone()),
                },
            ] {
                let variant = build_schnorr_fs_transfer_proof(
                    &adapter,
                    &transfer_payload,
                    SchnorrFsTransferProofOverrides {
                        first_messages: Some(overridden),
                        ..Default::default()
                    },
                );
                let result = adapter.verify(
                    VerifyParams,
                    build_verify_payload(
                        AlgebraicStatement::Transfer(&transfer_statement),
                        &variant,
                        Default::default(),
                    ),
                );
                assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry `a_proof_from_a_witness_the_statement_does_not_hold_
///   fails`; `prove_mint` or `prove_transfer` over a witness other than the
///   one the statement holds returns `Ok`, and `verify` returns
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases and verifier forms of the mint block; the payload with
///   each witness overridden alone by a scalar other than the one the
///   default statement is formed from.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `prove_mint` or `prove_transfer`, then `verify` over the statement and
///   the proof.
/// Assert:  the prover's result is `Ok` and `result.ok().map(|success|
///   success.is_valid)` equals `Some(false)` for each witness.
/// Boundary: the private calls `prove_mint` or `prove_transfer` and `verify`
///   and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_proof_from_a_witness_the_statement_does_not_hold_fails() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            for verifier_form in [
                VerifierGroupArithmetic::BothGroups,
                VerifierGroupArithmetic::FirstGroupOnly,
            ] {
                let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                    SchnorrFsDeliveryProofConstructorParams {
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        verifier_form,
                    },
                )
                .expect("the tags are admitted by the domain tag type");
                for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                    let statement = build_algebraic_mint_statement(
                        pairing,
                        AlgebraicMintStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let others = build_transfer_responses(pairing, Default::default());
                    let default_coins = build_envelope_coins(pairing, Default::default());
                    for overrides in [
                        ProveMintPayloadOverrides {
                            master_scalar: Some(MasterScalarComponents {
                                value: build_secret(
                                    others.x.clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            ..Default::default()
                        },
                        ProveMintPayloadOverrides {
                            credential_randomness: Some(build_secret(
                                others.y.clone(),
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                        ProveMintPayloadOverrides {
                            coins: Some(EnvelopeCoins {
                                rho: build_secret(
                                    others.s.clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                sigma: build_secret(
                                    default_coins.sigma.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            ..Default::default()
                        },
                        ProveMintPayloadOverrides {
                            coins: Some(EnvelopeCoins {
                                rho: build_secret(
                                    default_coins.rho.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                sigma: build_secret(
                                    others.sigma.clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            ..Default::default()
                        },
                    ] {
                        let payload = build_prove_mint_payload(pairing, &statement, overrides);
                        let proved = adapter
                            .prove_mint(ProveMintParams, payload)
                            .expect("prove_mint returns the proof");
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Mint(&statement),
                                &proved.proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                    }
                }
                for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                    let statement = build_algebraic_transfer_statement(
                        pairing,
                        AlgebraicTransferStatementOverrides {
                            context: Some(build_delivery_context(
                                purpose,
                                DeliveryContextOverrides::default(),
                            )),
                            ..Default::default()
                        },
                    );
                    let others = build_mint_responses(pairing, Default::default());
                    let default_secrets = build_key_pair_components(pairing, Default::default());
                    let default_coins = build_envelope_coins(pairing, Default::default());
                    for overrides in [
                        ProveTransferPayloadOverrides {
                            seller_secrets: Some(KeyPairComponents {
                                x: build_secret(
                                    others.alpha.clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                y: build_secret(
                                    default_secrets.y.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            ..Default::default()
                        },
                        ProveTransferPayloadOverrides {
                            seller_secrets: Some(KeyPairComponents {
                                x: build_secret(
                                    default_secrets.x.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                y: build_secret(others.r.clone(), SecretConstructorParamsOverrides),
                            }),
                            ..Default::default()
                        },
                        ProveTransferPayloadOverrides {
                            offset: Some(build_secret(
                                others.rho.clone(),
                                SecretConstructorParamsOverrides,
                            )),
                            ..Default::default()
                        },
                        ProveTransferPayloadOverrides {
                            coins: Some(EnvelopeCoins {
                                rho: build_secret(
                                    others.sigma.clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                sigma: build_secret(
                                    default_coins.sigma.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            ..Default::default()
                        },
                        ProveTransferPayloadOverrides {
                            coins: Some(EnvelopeCoins {
                                rho: build_secret(
                                    default_coins.rho.expose().clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                                sigma: build_secret(
                                    others.r.clone(),
                                    SecretConstructorParamsOverrides,
                                ),
                            }),
                            ..Default::default()
                        },
                    ] {
                        let payload = build_prove_transfer_payload(pairing, &statement, overrides);
                        let proved = adapter
                            .prove_transfer(ProveTransferParams, payload)
                            .expect("prove_transfer returns the proof");
                        let result = adapter.verify(
                            VerifyParams,
                            build_verify_payload(
                                AlgebraicStatement::Transfer(&statement),
                                &proved.proof,
                                Default::default(),
                            ),
                        );
                        assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
                    }
                }
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}

/// Contract: entry
///   `a_proof_whose_second_group_errors_cancel_unweighted_fails_the_
///   weighted_product`; `verify` in the first-group-only form over a proof
///   whose carried `d1` is raised by a second-group point and whose carried
///   `d2` is lowered by the same point, its challenge recomputed over the
///   changed messages, returns `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the cases of the mint block in the first-group-only form; the
///   proof from the proof builder with `first_messages` overridden by `d1`
///   plus `δ · g2` and `d2` minus `δ · g2` for a constant `δ`, for the mint
///   and for the transfer proof; the product with every weight one over the
///   terms the first-group-only form states, evaluated by
///   `pairing_product_is_one`.
/// Act:     `run_schnorr_fs_case` per case, the consumer's method calling
///   `verify(VerifyParams, payload)` over the crafted proof.
/// Assert:  the unweighted product `is_one` is true, so the crafted proof
///   passes an unweighted check, and `result.ok().map(|success|
///   success.is_valid)` equals `Some(false)`.
/// Boundary: the private call `verify` and the route of the mint block.
/// Mocked:  nothing.
#[test]
fn a_proof_whose_second_group_errors_cancel_unweighted_fails_the_weighted_product() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            let neg_g1 = |point: P::G1| -> P::G1 {
                let Ok(negated) = pairing.neg_g1(NegG1Params, NegG1Payload { point });
                negated.negation
            };
            let neg_g2 = |point: P::G2| -> P::G2 {
                let Ok(negated) = pairing.neg_g2(NegG2Params, NegG2Payload { point });
                negated.negation
            };
            let neg_scalar = |scalar: P::Scalar| -> P::Scalar {
                let Ok(negated) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
                negated.negation
            };
            let add_scalar = |left: P::Scalar, right: P::Scalar| -> P::Scalar {
                let Ok(sum) = pairing.add_scalar(AddScalarParams, AddScalarPayload { left, right });
                sum.sum
            };
            let mul_g1 = |point: P::G1, scalar: P::Scalar| -> P::G1 {
                let Ok(success) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
                success.product
            };
            let mul_g2 = |point: P::G2, scalar: P::Scalar| -> P::G2 {
                let Ok(success) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
                success.product
            };
            let add_g2 = |left: P::G2, right: P::G2| -> P::G2 {
                let Ok(sum) = pairing.add_g2(AddG2Params, AddG2Payload { left, right });
                sum.sum
            };
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);

            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let delta = build_mint_responses(pairing, Default::default()).alpha;
            let delta_g2 = mul_g2(g2.point.clone(), delta.clone());
            let neg_delta_g2 = neg_g2(delta_g2.clone());

            // Mint: the carried d1 raised and d2 lowered by `delta · g2`.
            let mint_statement = build_algebraic_mint_statement(pairing, Default::default());
            let mint_payload =
                build_prove_mint_payload(pairing, &mint_statement, Default::default());
            let baseline = build_schnorr_fs_mint_proof(
                &adapter,
                &mint_payload,
                SchnorrFsMintProofOverrides::default(),
            );
            let carried = match &baseline {
                SchnorrFsProof::MintFirstGroupOnly { first_messages, .. } => {
                    MintSecondGroupFirstMessages {
                        hpub: first_messages.hpub.clone(),
                        d1: first_messages.d1.clone(),
                        d2: first_messages.d2.clone(),
                    }
                }
                _ => panic!("the proof is the mint first-group-only variant"),
            };
            let crafted_messages = MintSecondGroupFirstMessages {
                hpub: carried.hpub.clone(),
                d1: add_g2(carried.d1.clone(), delta_g2.clone()),
                d2: add_g2(carried.d2.clone(), neg_delta_g2.clone()),
            };
            let crafted = build_schnorr_fs_mint_proof(
                &adapter,
                &mint_payload,
                SchnorrFsMintProofOverrides {
                    first_messages: Some(crafted_messages),
                    ..Default::default()
                },
            );
            let (challenge_scalar, responses, carried) = match &crafted {
                SchnorrFsProof::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => (challenge, responses, first_messages),
                _ => panic!("the proof is the mint first-group-only variant"),
            };
            let n = neg_scalar(challenge_scalar.clone());
            let combined = add_scalar(
                responses.alpha.clone(),
                add_scalar(responses.sigma.clone(), responses.r.clone()),
            );
            let unweighted = pairing.pairing_product_is_one(
                PairingProductIsOneParams,
                PairingProductIsOnePayload {
                    terms: vec![
                        PairingProductTerm {
                            g1: neg_g1(g1.point.clone()),
                            g2: carried.hpub.clone(),
                        },
                        PairingProductTerm {
                            g1: neg_g1(g1.point.clone()),
                            g2: carried.d1.clone(),
                        },
                        PairingProductTerm {
                            g1: neg_g1(g1.point.clone()),
                            g2: carried.d2.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), combined),
                            g2: g2.point.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), n.clone()),
                            g2: mint_statement.hpub.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), n.clone()),
                            g2: mint_statement.envelope.d1.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), responses.sigma.clone()),
                            g2: mint_statement.buyer_keys.pk2.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), n.clone()),
                            g2: mint_statement.envelope.d2.clone(),
                        },
                    ],
                },
            );
            let Ok(unweighted) = unweighted;
            assert!(unweighted.is_one);
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint_statement),
                    &crafted,
                    Default::default(),
                ),
            );
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));

            // Transfer: the same crafted shift on the carried d1 and d2.
            let transfer_statement =
                build_algebraic_transfer_statement(pairing, Default::default());
            let transfer_payload =
                build_prove_transfer_payload(pairing, &transfer_statement, Default::default());
            let baseline = build_schnorr_fs_transfer_proof(
                &adapter,
                &transfer_payload,
                SchnorrFsTransferProofOverrides::default(),
            );
            let carried = match &baseline {
                SchnorrFsProof::TransferFirstGroupOnly { first_messages, .. } => {
                    TransferSecondGroupFirstMessages {
                        pk2: first_messages.pk2.clone(),
                        d1: first_messages.d1.clone(),
                        d2: first_messages.d2.clone(),
                    }
                }
                _ => panic!("the proof is the transfer first-group-only variant"),
            };
            let crafted_messages = TransferSecondGroupFirstMessages {
                pk2: carried.pk2.clone(),
                d1: add_g2(carried.d1.clone(), delta_g2.clone()),
                d2: add_g2(carried.d2.clone(), neg_delta_g2.clone()),
            };
            let crafted = build_schnorr_fs_transfer_proof(
                &adapter,
                &transfer_payload,
                SchnorrFsTransferProofOverrides {
                    first_messages: Some(crafted_messages),
                    ..Default::default()
                },
            );
            let (challenge_scalar, responses, carried) = match &crafted {
                SchnorrFsProof::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => (challenge, responses, first_messages),
                _ => panic!("the proof is the transfer first-group-only variant"),
            };
            let n = neg_scalar(challenge_scalar.clone());
            let combined = add_scalar(
                responses.y.clone(),
                add_scalar(responses.sigma.clone(), responses.s.clone()),
            );
            let unweighted = pairing.pairing_product_is_one(
                PairingProductIsOneParams,
                PairingProductIsOnePayload {
                    terms: vec![
                        PairingProductTerm {
                            g1: neg_g1(g1.point.clone()),
                            g2: carried.pk2.clone(),
                        },
                        PairingProductTerm {
                            g1: neg_g1(g1.point.clone()),
                            g2: carried.d1.clone(),
                        },
                        PairingProductTerm {
                            g1: neg_g1(g1.point.clone()),
                            g2: carried.d2.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), combined),
                            g2: g2.point.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), n.clone()),
                            g2: transfer_statement.seller_keys.pk2.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), n.clone()),
                            g2: transfer_statement.new_envelope.d1.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), neg_scalar(responses.y.clone())),
                            g2: transfer_statement.old_envelope.d1.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), responses.sigma.clone()),
                            g2: transfer_statement.buyer_keys.pk2.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), n.clone()),
                            g2: transfer_statement.new_envelope.d2.clone(),
                        },
                        PairingProductTerm {
                            g1: mul_g1(g1.point.clone(), challenge_scalar.clone()),
                            g2: transfer_statement.old_envelope.d2.clone(),
                        },
                    ],
                },
            );
            let Ok(unweighted) = unweighted;
            assert!(unweighted.is_one);
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer_statement),
                    &crafted,
                    Default::default(),
                ),
            );
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
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
                    let result = run_schnorr_fs_case(
                        &build_schnorr_fs_case_deps(Consumer, Default::default()),
                        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(pairing_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(chain_forms_concrete.clone()),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding.concrete),
                                    identifier: Some(encoding.identifier),
                                },
                            )),
                            hash_to_scalar: Some(hash_to_scalar.clone()),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(RandomSourceKind::OperatingSystem),
                                },
                            )),
                        }),
                        build_schnorr_fs_case_payload(Default::default()),
                    );
                    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
                }
            }
        }
    }
}
