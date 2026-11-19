#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use alloy::primitives::{Bytes, U256};
use alloy::rpc::json_rpc::ErrorPayload;
use alloy::sol_types::SolCall;
use alloy::transports::mock::Asserter;
use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, EstimateFeeParams, FeeRequestOverrides,
    IChainAdapter, IEntitlementStateAdapter, build_create_chain_forms_params,
    build_create_chain_params, build_estimate_fee_payload, build_fee_request,
};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, build_create_encoding_params,
};
use envelope::EnvelopeAlgebra;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    build_create_hash_to_scalar_params,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    PAIRING_CONCRETES, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofComponents, DeliveryRelation,
    IDeliveryProofAdapter, ProofComponentsParams, ProveMintParams, ProveTransferParams,
    build_algebraic_mint_statement, build_algebraic_transfer_statement,
    build_create_delivery_proof_params, build_proof_components_payload, build_prove_mint_payload,
    build_prove_transfer_payload, build_verify_payload,
};
use random::{
    CreateRandomSourceParamsOverrides, RANDOM_SOURCE_KINDS, build_create_random_source_params,
};
use serde_json::value::RawValue;

use super::super::factory::provides::{
    IVerifierAdapter, VerifyCostOverrides, VerifyOnChainErrorReturn, VerifyOnChainSuccessReturn,
    build_verify_cost, build_verify_on_chain_params,
};
use super::IDeliveryVerifier;
use super::interface::{
    ConsumeEvmVerifierCaseParams, ConsumeEvmVerifierCasePayload, IEvmVerifierCaseConsumer,
};
use super::mock::*;

/// Contract: entry
///   `a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths`;
///   `calldata` over the statement `build_algebraic_mint_statement` or
///   `build_algebraic_transfer_statement` builds and the components of the
///   proof the real delivery proof adapter's `prove_mint` or
///   `prove_transfer` returns over the witnesses `build_prove_mint_payload`
///   or `build_prove_transfer_payload` holds, read by `proof_components`,
///   decoded by `verifyMintCall::abi_decode` or
///   `verifyTransferCall::abi_decode`; every `bytes` member that holds a
///   first-group element is as long as the `encode_g1` of the first-group
///   generator, every member that holds a second-group element as long as
///   the `encode_g2` of the second-group generator,
///   `secondGroupFirstMessages` three times the length `encode_g2` returns
///   for the second-group generator, in the order `hpub`, `d1`, `d2` for a
///   mint and `pk2`, `d1`, `d2` for a transfer, where the pairing declares
///   `VerifierGroupArithmetic::FirstGroupOnly` and empty where it declares
///   `BothGroups`, and every scalar member the thirty-two-byte word of the
///   encoded scalar; the absorbed `chain/create_chain_forms` round-trip
///   entries — the identity forms at the entitlement contract, the seller,
///   and the buyer reaching the calldata as twenty-byte members, the
///   entitlement forms as 256-bit members, the interval forms as 64-bit
///   members, and the chain-identifier form as a 256-bit member — and the
///   absorbed `proof/factory` entry — the challenge, the responses in
///   witness order, and the second-group first messages the adapter's form
///   carries reaching the calldata in that order — are proven here.
/// Arrange: every member of `PAIRING_CONCRETES`, nested over every member
///   of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, every member of
///   `DECLARED_ENCODING_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every member of `DELIVERY_PROOF_CONCRETES` in
///   the verifier form the resolved pairing's declaration decides, the
///   chain `build_create_chain_params` defaults to, the `Mock` member
///   under `NoFailure`, and every member of `RANDOM_SOURCE_KINDS`; the
///   subject built by `build_evm_verifier` over a provider an `Asserter`
///   scripts; the real statements `build_algebraic_mint_statement` and
///   `build_algebraic_transfer_statement` build over the resolved pairing
///   and forms.
/// Act:     `prove_mint` and `prove_transfer` on the received adapter over
///   the payloads the witness builders hold, `proof_components` over each
///   returned proof, and `calldata` over each statement and its components.
/// Assert:  in place, the decoded mint call's `hpub`, `buyerPk2`,
///   `envelopeD1`, and `envelopeD2` members are the second-group encoding's
///   length, `identityElement`, `buyerPk1`, `envelopeC1`, and `envelopeC2`
///   the first-group encoding's length, the decoded transfer call's
///   `identityElement`, `sellerPk1`, `buyerPk1`, and the `C1`/`C2` envelope
///   members the first-group encoding's length, `sellerPk2`, `buyerPk2`,
///   and the `D1`/`D2` envelope members the second-group encoding's
///   length, each proof's scalar members equal `U256::from_be_bytes` of
///   the pairing's `encode_scalar` of the scalar of its name, and
///   `secondGroupFirstMessages` equals the concatenation of `encode_g2`
///   over the first messages the adapter's declared form carries;
///   `read_q().len()` equals `0`, the chain never read.
/// Boundary: `create_pairing`, the concrete's `try_new`,
///   `create_encoding`, the selection's `try_new`, `create_chain_forms`,
///   the concrete's `try_new`, `create_hash_to_scalar`, the selection's
///   `try_new`, `create_random_source`, `create_chain`,
///   `create_delivery_proof`, the concrete's `try_new`, `prove_mint`,
///   `prove_transfer`, `proof_components`, `EvmVerifier::try_new_mocked`,
///   `calldata`, `mint_calldata`, `transfer_calldata`, `delivery_context`,
///   and `encode_scalar_word`.
/// Mocked:  the chain, scripted through an `Asserter` and never read, and
///   the chain adapter, the family family's `Mock` member answering
///   `estimate_fee`; the operating system's generator runs real.
#[test]
fn a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths() {
    // Arrange
    struct Consumer;

    impl IEvmVerifierCaseConsumer for Consumer {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Arrange
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                asserter.clone(),
                Default::default(),
            );
            let Ok(first_group_generator) = payload
                .pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(first_group) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: first_group_generator.point,
                },
            );
            let first_group = first_group.bytes.as_ref().len();
            let Ok(second_group_generator) = payload
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(second_group) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: second_group_generator.point,
                },
            );
            let second_group = second_group.bytes.as_ref().len();
            fn scalar_word<P: IPairingArithmetic>(pairing: &P, scalar: P::Scalar) -> U256 {
                let encoded = pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("the pairing encodes its scalars");
                let bytes: [u8; 32] = encoded
                    .bytes
                    .expose()
                    .as_ref()
                    .try_into()
                    .expect("the pairing encodes a scalar as thirty-two bytes");
                U256::from_be_bytes(bytes)
            }
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let proved = payload.delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(payload.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let Ok(components) = payload.delivery_proof.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proved.proof, Default::default()),
            );
            let expected_scalars = match &components.components {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => [
                    scalar_word(payload.pairing, challenge.clone()),
                    scalar_word(payload.pairing, responses.alpha.clone()),
                    scalar_word(payload.pairing, responses.r.clone()),
                    scalar_word(payload.pairing, responses.rho.clone()),
                    scalar_word(payload.pairing, responses.sigma.clone()),
                ],
                DeliveryProofComponents::TransferBothGroups { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                    panic!("a mint proof's components carry the mint relation")
                }
            };
            let mut expected_first_messages = Vec::new();
            if let DeliveryProofComponents::MintFirstGroupOnly { first_messages, .. } =
                &components.components
            {
                for point in [
                    first_messages.hpub.clone(),
                    first_messages.d1.clone(),
                    first_messages.d2.clone(),
                ] {
                    let encoded = payload
                        .pairing
                        .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                        .expect("the pairing encodes its second-group elements");
                    expected_first_messages.extend_from_slice(encoded.bytes.as_ref());
                }
            }

            // Act
            let encoded_mint =
                subject.calldata(&AlgebraicStatement::Mint(&mint), components.components);

            // Assert
            let Ok(calldata) = encoded_mint else {
                panic!("the real statement and its proof's components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            assert_eq!(call.statement.hpub.len(), second_group);
            assert_eq!(call.statement.identityElement.len(), first_group);
            assert_eq!(call.statement.buyerPk1.len(), first_group);
            assert_eq!(call.statement.buyerPk2.len(), second_group);
            assert_eq!(call.statement.envelopeC1.len(), first_group);
            assert_eq!(call.statement.envelopeC2.len(), first_group);
            assert_eq!(call.statement.envelopeD1.len(), second_group);
            assert_eq!(call.statement.envelopeD2.len(), second_group);
            assert_eq!(
                [
                    call.proof.challenge,
                    call.proof.alpha,
                    call.proof.r,
                    call.proof.rho,
                    call.proof.sigma,
                ],
                expected_scalars
            );
            assert_eq!(
                call.proof.secondGroupFirstMessages.as_ref(),
                expected_first_messages
            );
            assert_eq!(asserter.read_q().len(), 0);

            // Arrange
            let transfer = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                Default::default(),
            );
            let proved = payload.delivery_proof.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(payload.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };
            let Ok(components) = payload.delivery_proof.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proved.proof, Default::default()),
            );
            let expected_scalars = match &components.components {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => [
                    scalar_word(payload.pairing, challenge.clone()),
                    scalar_word(payload.pairing, responses.x.clone()),
                    scalar_word(payload.pairing, responses.y.clone()),
                    scalar_word(payload.pairing, responses.s.clone()),
                    scalar_word(payload.pairing, responses.rho.clone()),
                    scalar_word(payload.pairing, responses.sigma.clone()),
                ],
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::MintFirstGroupOnly { .. } => {
                    panic!("a transfer proof's components carry the transfer relation")
                }
            };
            let mut expected_first_messages = Vec::new();
            if let DeliveryProofComponents::TransferFirstGroupOnly { first_messages, .. } =
                &components.components
            {
                for point in [
                    first_messages.pk2.clone(),
                    first_messages.d1.clone(),
                    first_messages.d2.clone(),
                ] {
                    let encoded = payload
                        .pairing
                        .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                        .expect("the pairing encodes its second-group elements");
                    expected_first_messages.extend_from_slice(encoded.bytes.as_ref());
                }
            }

            // Act
            let encoded_transfer = subject.calldata(
                &AlgebraicStatement::Transfer(&transfer),
                components.components,
            );

            // Assert
            let Ok(calldata) = encoded_transfer else {
                panic!("the real statement and its proof's components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            assert_eq!(call.statement.identityElement.len(), first_group);
            assert_eq!(call.statement.sellerPk1.len(), first_group);
            assert_eq!(call.statement.sellerPk2.len(), second_group);
            assert_eq!(call.statement.buyerPk1.len(), first_group);
            assert_eq!(call.statement.buyerPk2.len(), second_group);
            assert_eq!(call.statement.oldEnvelopeC1.len(), first_group);
            assert_eq!(call.statement.oldEnvelopeC2.len(), first_group);
            assert_eq!(call.statement.oldEnvelopeD1.len(), second_group);
            assert_eq!(call.statement.oldEnvelopeD2.len(), second_group);
            assert_eq!(call.statement.newEnvelopeC1.len(), first_group);
            assert_eq!(call.statement.newEnvelopeC2.len(), first_group);
            assert_eq!(call.statement.newEnvelopeD1.len(), second_group);
            assert_eq!(call.statement.newEnvelopeD2.len(), second_group);
            assert_eq!(
                [
                    call.proof.challenge,
                    call.proof.x,
                    call.proof.y,
                    call.proof.s,
                    call.proof.rho,
                    call.proof.sigma,
                ],
                expected_scalars
            );
            assert_eq!(
                call.proof.secondGroupFirstMessages.as_ref(),
                expected_first_messages
            );
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                        for random_kind in RANDOM_SOURCE_KINDS {
                            // Act
                            let result = run_evm_verifier_case(
                                &build_evm_verifier_case_deps(Consumer, Default::default()),
                                build_evm_verifier_case_params(EvmVerifierCaseParamsOverrides {
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*pairing_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(hash_to_scalar_selection.concrete),
                                            identifier: Some(hash_to_scalar_selection.identifier),
                                        },
                                    )),
                                    encoding: Some(build_create_encoding_params(
                                        CreateEncodingParamsOverrides {
                                            concrete: Some(encoding_selection.concrete),
                                            identifier: Some(encoding_selection.identifier),
                                        },
                                    )),
                                    chain_forms: Some(build_create_chain_forms_params(
                                        CreateChainFormsParamsOverrides {
                                            concrete: Some(*forms_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    chain: Some(build_create_chain_params(Default::default())),
                                    delivery_proof: Some(build_create_delivery_proof_params(
                                        CreateDeliveryProofParamsOverrides {
                                            concrete: Some(*delivery_proof_concrete),
                                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                                        },
                                    )),
                                    random_source: Some(build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(*random_kind),
                                        },
                                    )),
                                }),
                                build_evm_verifier_case_payload(Default::default()),
                            );

                            // Assert
                            let Ok(_success) = result else {
                                panic!("the case chain admits every selection")
                            };
                        }
                    }
                }
            }
        }
    }
    // a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths
}

/// Contract: entry
///   `an_accepted_verdict_and_its_cost_reaches_the_caller_for_a_real_proof`;
///   `verify_on_chain` over the same real statement and proof, on a
///   provider scripted to answer the verifier's code non-empty and the
///   verifying call `true`, the chain adapter answering the fee
///   estimation with the description its declaration composes;
///   `Ok(VerifyOnChainSuccessReturn::Accepted { cost })` with
///   `cost.description` the `FeeDescription` the chain adapter's
///   `estimate_fee` returns for the request of the verifier's admitted
///   form and the calldata.
/// Arrange: the cases of the calldata block; the `Asserter` answers
///   verifier code and `true`, per relation; the expected description the
///   received chain adapter's `estimate_fee` returns over the
///   `FeeRequest` of the verifier's admitted `FixedBytes20([0x11; 20])`
///   form and the calldata the subject produces.
/// Act:     `verify_on_chain` over each relation's real statement and
///   proof.
/// Assert:  `Ok(VerifyOnChainSuccessReturn::Accepted { cost })` in place,
///   `cost.description` equal to the description the chain adapter
///   returns, and `read_q().len()` equals `0`, the fee estimation
///   reaching no `Asserter` answer since the chain adapter serves it.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `verify_on_chain`, `calldata`,
///   `read_chain`, and the chain adapter's `estimate_fee`.
/// Mocked:  the chain, scripted through an `Asserter`, and the chain
///   adapter, the family's `Mock` member answering `estimate_fee`; the
///   operating system's generator runs real.
#[test]
fn an_accepted_verdict_and_its_cost_reaches_the_caller_for_a_real_proof() {
    // Arrange
    struct Consumer;

    impl IEvmVerifierCaseConsumer for Consumer {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Arrange
            let asserter = Asserter::default();
            let mut true_word = vec![0x00; 32];
            true_word[31] = 0x01;
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(true_word.clone()));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                asserter.clone(),
                Default::default(),
            );
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let proved = payload.delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(payload.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let Ok(components) = payload.delivery_proof.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proved.proof, Default::default()),
            );
            let Ok(calldata) =
                subject.calldata(&AlgebraicStatement::Mint(&mint), components.components)
            else {
                panic!("the real statement and its proof's components encode")
            };
            let request = build_fee_request::<D::Forms>(FeeRequestOverrides {
                input: Some(calldata),
                ..Default::default()
            });
            let estimated = payload.chain.estimate_fee(
                EstimateFeeParams,
                build_estimate_fee_payload::<D::Forms>(&request, Default::default()),
            );
            let Ok(estimated) = estimated else {
                panic!("the mock chain adapter estimates the fee it declares")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        description: Some(estimated.description),
                    }),
                })
            );
            assert_eq!(asserter.read_q().len(), 0);

            // Arrange
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(true_word));
            let transfer = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                Default::default(),
            );
            let proved = payload.delivery_proof.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(payload.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };
            let Ok(components) = payload.delivery_proof.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proved.proof, Default::default()),
            );
            let Ok(calldata) = subject.calldata(
                &AlgebraicStatement::Transfer(&transfer),
                components.components,
            ) else {
                panic!("the real statement and its proof's components encode")
            };
            let request = build_fee_request::<D::Forms>(FeeRequestOverrides {
                input: Some(calldata),
                ..Default::default()
            });
            let estimated = payload.chain.estimate_fee(
                EstimateFeeParams,
                build_estimate_fee_payload::<D::Forms>(&request, Default::default()),
            );
            let Ok(estimated) = estimated else {
                panic!("the mock chain adapter estimates the fee it declares")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        description: Some(estimated.description),
                    }),
                })
            );
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                        for random_kind in RANDOM_SOURCE_KINDS {
                            // Act
                            let result = run_evm_verifier_case(
                                &build_evm_verifier_case_deps(Consumer, Default::default()),
                                build_evm_verifier_case_params(EvmVerifierCaseParamsOverrides {
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*pairing_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(hash_to_scalar_selection.concrete),
                                            identifier: Some(hash_to_scalar_selection.identifier),
                                        },
                                    )),
                                    encoding: Some(build_create_encoding_params(
                                        CreateEncodingParamsOverrides {
                                            concrete: Some(encoding_selection.concrete),
                                            identifier: Some(encoding_selection.identifier),
                                        },
                                    )),
                                    chain_forms: Some(build_create_chain_forms_params(
                                        CreateChainFormsParamsOverrides {
                                            concrete: Some(*forms_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    chain: Some(build_create_chain_params(Default::default())),
                                    delivery_proof: Some(build_create_delivery_proof_params(
                                        CreateDeliveryProofParamsOverrides {
                                            concrete: Some(*delivery_proof_concrete),
                                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                                        },
                                    )),
                                    random_source: Some(build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(*random_kind),
                                        },
                                    )),
                                }),
                                build_evm_verifier_case_payload(Default::default()),
                            );

                            // Assert
                            let Ok(_success) = result else {
                                panic!("the case chain admits every selection")
                            };
                        }
                    }
                }
            }
        }
    }
    // an_accepted_verdict_and_its_cost_reaches_the_caller_for_a_real_proof
}

/// Contract: entry `a_rejecting_chain_returns_rejected_for_a_real_proof`;
///   `verify_on_chain` over a real statement and proof, on a provider
///   scripted to answer the verifier's code non-empty and the verifying
///   call `false`, returns `Ok(VerifyOnChainSuccessReturn::Rejected)` and
///   the chain's fee estimation is unreached.
/// Arrange: the cases of the calldata block; the `Asserter` answers
///   verifier code and `false`, per relation.
/// Act:     `verify_on_chain` over each relation's real statement and
///   proof.
/// Assert:  `Ok(VerifyOnChainSuccessReturn::Rejected)` in place and
///   `read_q().len()` equals `0` after each call, a rejected delivery a
///   verdict that reads no cost.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `verify_on_chain`, `calldata`, and
///   `read_chain`.
/// Mocked:  the chain, scripted through an `Asserter`, and the chain
///   adapter, the family's `Mock` member; the operating system's
///   generator runs real.
#[test]
fn a_rejecting_chain_returns_rejected_for_a_real_proof() {
    // Arrange
    struct Consumer;

    impl IEvmVerifierCaseConsumer for Consumer {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Arrange
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(vec![0x00; 32]));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                asserter.clone(),
                Default::default(),
            );
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let proved = payload.delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(payload.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok(), Some(VerifyOnChainSuccessReturn::Rejected));
            assert_eq!(asserter.read_q().len(), 0);

            // Arrange
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(vec![0x00; 32]));
            let transfer = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                Default::default(),
            );
            let proved = payload.delivery_proof.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(payload.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok(), Some(VerifyOnChainSuccessReturn::Rejected));
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                        for random_kind in RANDOM_SOURCE_KINDS {
                            // Act
                            let result = run_evm_verifier_case(
                                &build_evm_verifier_case_deps(Consumer, Default::default()),
                                build_evm_verifier_case_params(EvmVerifierCaseParamsOverrides {
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*pairing_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(hash_to_scalar_selection.concrete),
                                            identifier: Some(hash_to_scalar_selection.identifier),
                                        },
                                    )),
                                    encoding: Some(build_create_encoding_params(
                                        CreateEncodingParamsOverrides {
                                            concrete: Some(encoding_selection.concrete),
                                            identifier: Some(encoding_selection.identifier),
                                        },
                                    )),
                                    chain_forms: Some(build_create_chain_forms_params(
                                        CreateChainFormsParamsOverrides {
                                            concrete: Some(*forms_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    chain: Some(build_create_chain_params(Default::default())),
                                    delivery_proof: Some(build_create_delivery_proof_params(
                                        CreateDeliveryProofParamsOverrides {
                                            concrete: Some(*delivery_proof_concrete),
                                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                                        },
                                    )),
                                    random_source: Some(build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(*random_kind),
                                        },
                                    )),
                                }),
                                build_evm_verifier_case_payload(Default::default()),
                            );

                            // Assert
                            let Ok(_success) = result else {
                                panic!("the case chain admits every selection")
                            };
                        }
                    }
                }
            }
        }
    }
    // a_rejecting_chain_returns_rejected_for_a_real_proof
}

/// Contract: entry `a_reverting_chain_returns_rejected_for_a_real_proof`;
///   `verify_on_chain` over a real statement and proof, on a provider
///   scripted to answer the verifier's code non-empty and the verifying
///   call refused by a JSON-RPC error carrying the hex data
///   `0x08c379a0`, returns `Ok(VerifyOnChainSuccessReturn::Rejected)` and
///   the answers after stay unread, a call refused in place a verdict,
///   never an `Rpc` error.
/// Arrange: the cases of the calldata block; the `Asserter` answers
///   verifier code, the revert, and one further answer, per relation.
/// Act:     `verify_on_chain` over each relation's real statement and
///   proof.
/// Assert:  `Ok(VerifyOnChainSuccessReturn::Rejected)` in place and
///   `read_q().len()` equals `1` after each call.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `verify_on_chain`, `calldata`, and
///   `read_chain`.
/// Mocked:  the chain, scripted through an `Asserter`, and the chain
///   adapter, the family's `Mock` member; the operating system's
///   generator runs real.
#[test]
fn a_reverting_chain_returns_rejected_for_a_real_proof() {
    // Arrange
    struct Consumer;

    impl IEvmVerifierCaseConsumer for Consumer {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Arrange
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_failure(ErrorPayload {
                code: 3,
                message: "execution reverted".into(),
                data: Some(
                    RawValue::from_string("\"0x08c379a0\"".to_string())
                        .expect("a quoted hex string is a valid json value"),
                ),
            });
            asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                asserter.clone(),
                Default::default(),
            );
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let proved = payload.delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(payload.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok(), Some(VerifyOnChainSuccessReturn::Rejected));
            assert_eq!(asserter.read_q().len(), 1);

            // Arrange
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_failure(ErrorPayload {
                code: 3,
                message: "execution reverted".into(),
                data: Some(
                    RawValue::from_string("\"0x08c379a0\"".to_string())
                        .expect("a quoted hex string is a valid json value"),
                ),
            });
            asserter.push_success(&Bytes::from(vec![0x60]));
            let transfer = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                Default::default(),
            );
            let proved = payload.delivery_proof.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(payload.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok(), Some(VerifyOnChainSuccessReturn::Rejected));
            assert_eq!(asserter.read_q().len(), 1);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                        for random_kind in RANDOM_SOURCE_KINDS {
                            // Act
                            let result = run_evm_verifier_case(
                                &build_evm_verifier_case_deps(Consumer, Default::default()),
                                build_evm_verifier_case_params(EvmVerifierCaseParamsOverrides {
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*pairing_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(hash_to_scalar_selection.concrete),
                                            identifier: Some(hash_to_scalar_selection.identifier),
                                        },
                                    )),
                                    encoding: Some(build_create_encoding_params(
                                        CreateEncodingParamsOverrides {
                                            concrete: Some(encoding_selection.concrete),
                                            identifier: Some(encoding_selection.identifier),
                                        },
                                    )),
                                    chain_forms: Some(build_create_chain_forms_params(
                                        CreateChainFormsParamsOverrides {
                                            concrete: Some(*forms_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    chain: Some(build_create_chain_params(Default::default())),
                                    delivery_proof: Some(build_create_delivery_proof_params(
                                        CreateDeliveryProofParamsOverrides {
                                            concrete: Some(*delivery_proof_concrete),
                                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                                        },
                                    )),
                                    random_source: Some(build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(*random_kind),
                                        },
                                    )),
                                }),
                                build_evm_verifier_case_payload(Default::default()),
                            );

                            // Assert
                            let Ok(_success) = result else {
                                panic!("the case chain admits every selection")
                            };
                        }
                    }
                }
            }
        }
    }
    // a_reverting_chain_returns_rejected_for_a_real_proof
}

/// Contract: entry
///   `a_real_proof_of_the_other_relation_is_refused_before_any_chain_read`;
///   `verify_on_chain` over a real mint statement and the real proof
///   `prove_transfer` returned, and over a real transfer statement and the
///   proof `prove_mint` returned, the scripted provider holding one
///   answer; `Err(VerifyOnChainErrorReturn::RelationMismatch { statement,
///   proof })` with the relations the case names and the scripted answer
///   unread, the refusal the family's flavor, decided from the real
///   proof's components before the chain is read, and naming no concrete.
/// Arrange: the cases of the calldata block; the `Asserter` answers one
///   scripted answer per call.
/// Act:     `verify_on_chain` over a real mint statement and the real
///   transfer proof, and over a real transfer statement and the real mint
///   proof.
/// Assert:  `Err(VerifyOnChainErrorReturn::RelationMismatch { statement:
///   DeliveryRelation::Mint, proof: DeliveryRelation::Transfer })` then
///   `RelationMismatch { statement: DeliveryRelation::Transfer, proof:
///   DeliveryRelation::Mint }`, and `read_q().len()` equals `1` after
///   each call, the chain never read.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `verify_on_chain`, and `calldata`.
/// Mocked:  the chain, scripted through an `Asserter` and never read, and
///   the chain adapter, the family's `Mock` member; the operating
///   system's generator runs real.
#[test]
fn a_real_proof_of_the_other_relation_is_refused_before_any_chain_read() {
    // Arrange
    struct Consumer;

    impl IEvmVerifierCaseConsumer for Consumer {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Arrange
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                asserter.clone(),
                Default::default(),
            );
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let transfer = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                Default::default(),
            );
            let proved = payload.delivery_proof.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(payload.pairing, &transfer, Default::default()),
            );
            let Ok(transfer_proof) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint),
                    &transfer_proof.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::RelationMismatch {
                    statement: DeliveryRelation::Mint,
                    proof: DeliveryRelation::Transfer,
                })
            ));
            assert_eq!(asserter.read_q().len(), 1);

            // Arrange
            asserter.push_success(&Bytes::from(vec![0x60]));
            let proved = payload.delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(payload.pairing, &mint, Default::default()),
            );
            let Ok(mint_proof) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer),
                    &mint_proof.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::RelationMismatch {
                    statement: DeliveryRelation::Transfer,
                    proof: DeliveryRelation::Mint,
                })
            ));
            assert_eq!(asserter.read_q().len(), 1);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                        for random_kind in RANDOM_SOURCE_KINDS {
                            // Act
                            let result = run_evm_verifier_case(
                                &build_evm_verifier_case_deps(Consumer, Default::default()),
                                build_evm_verifier_case_params(EvmVerifierCaseParamsOverrides {
                                    pairing: Some(build_create_pairing_params(
                                        CreatePairingParamsOverrides {
                                            concrete: Some(*pairing_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(hash_to_scalar_selection.concrete),
                                            identifier: Some(hash_to_scalar_selection.identifier),
                                        },
                                    )),
                                    encoding: Some(build_create_encoding_params(
                                        CreateEncodingParamsOverrides {
                                            concrete: Some(encoding_selection.concrete),
                                            identifier: Some(encoding_selection.identifier),
                                        },
                                    )),
                                    chain_forms: Some(build_create_chain_forms_params(
                                        CreateChainFormsParamsOverrides {
                                            concrete: Some(*forms_concrete),
                                            ..Default::default()
                                        },
                                    )),
                                    chain: Some(build_create_chain_params(Default::default())),
                                    delivery_proof: Some(build_create_delivery_proof_params(
                                        CreateDeliveryProofParamsOverrides {
                                            concrete: Some(*delivery_proof_concrete),
                                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                                        },
                                    )),
                                    random_source: Some(build_create_random_source_params(
                                        CreateRandomSourceParamsOverrides {
                                            kind: Some(*random_kind),
                                        },
                                    )),
                                }),
                                build_evm_verifier_case_payload(Default::default()),
                            );

                            // Assert
                            let Ok(_success) = result else {
                                panic!("the case chain admits every selection")
                            };
                        }
                    }
                }
            }
        }
    }
    // a_real_proof_of_the_other_relation_is_refused_before_any_chain_read
}
