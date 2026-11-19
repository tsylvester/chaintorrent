#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use alloy::primitives::Bytes;
use alloy::rpc::json_rpc::ErrorPayload;
use alloy::sol_types::SolValue;
use alloy::transports::mock::Asserter;
use chain::{
    ChainSelection, CreateChainParams, EstimateFeeErrorReturn, IChainAdapter,
    IEntitlementStateAdapter, MockIChainAdapterEstimateFeeErrorReturn,
    MockIChainAdapterFailureMode,
};
use encoding::CanonicalFieldValue;
use pairing::{IPairingArithmetic, IPairingReference, VerifierGroupArithmetic};
use proof::{
    AlgebraicStatement, DeliveryProofComponents, DeliveryProofWireComponentsOverrides,
    DeliveryRelation, IDeliveryProofAdapter, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_delivery_proof_wire_components,
    build_mint_second_group_first_messages, build_proof_from_components_params,
    build_proof_from_components_payload, build_transfer_responses,
    build_transfer_second_group_first_messages, build_verify_payload,
};

use super::super::factory::provides::{
    IVerifierAdapter, VerifyOnChainErrorReturn, build_verify_on_chain_params,
};
use super::interface::{
    ConsumeEvmVerifierCaseParams, ConsumeEvmVerifierCasePayload, EvmVerifierErrorReturn,
    IEvmVerifierCaseConsumer,
};
use super::mock::*;

/// Contract: a statement and a proof of different relations →
///   `Err(RelationMismatch { statement: DeliveryRelation::Mint, proof:
///   DeliveryRelation::Transfer })`, before any read.
/// Arrange: a mint statement, a proof from transfer components, and one
///   scripted answer.
/// Act:     `verify_on_chain`.
/// Assert:  `matches!(result, Err(VerifyOnChainErrorReturn::
///   RelationMismatch { statement: DeliveryRelation::Mint, proof:
///   DeliveryRelation::Transfer }))` and `read_q().len()` equals `1`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_refuses_a_mint_statement_with_a_transfer_proof() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let responses = build_transfer_responses(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Transfer),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_transfer_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.pk2, messages.d1, messages.d2])
                        }
                    },
                    responses: Some(vec![
                        responses.x.clone(),
                        responses.y.clone(),
                        responses.s.clone(),
                        responses.rho.clone(),
                        responses.sigma.clone(),
                    ]),
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &built.proof,
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
            assert_eq!(self.asserter.read_q().len(), 1);
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_refuses_a_mint_statement_with_a_transfer_proof
}

/// Contract: the reverse → `RelationMismatch { statement:
///   DeliveryRelation::Transfer, proof: DeliveryRelation::Mint }`.
/// Arrange: a transfer statement, a proof from mint components, and one
///   scripted answer.
/// Act:     `verify_on_chain`.
/// Assert:  as the mint block with the relations swapped.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_refuses_a_transfer_statement_with_a_mint_proof() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                Default::default(),
            );
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &built.proof,
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
            assert_eq!(self.asserter.read_q().len(), 1);
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_refuses_a_transfer_statement_with_a_mint_proof
}

/// Contract: an empty code read → `Err(Evm(NothingDeployed { address
///   }))`, `address` the verifier's twenty bytes, no further read.
/// Arrange: `verifier_address` `FixedBytes20([0x42; 20])`, an empty code
///   answer, and a further answer that must stay unread.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!(result, Err(VerifyOnChainErrorReturn::Evm(
///   EvmVerifierErrorReturn::NothingDeployed { address })) if address ==
///   [0x42; 20])` and `read_q().len()` equals `1`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_refuses_when_nothing_is_deployed_at_the_verifier_address() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_success(&Bytes::new());
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                EvmVerifierConstructorParamsOverrides {
                    verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
                    ..Default::default()
                },
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &built.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::Evm(
                    EvmVerifierErrorReturn::NothingDeployed { address }
                )) if address == [0x42; 20]
            ));
            assert_eq!(self.asserter.read_q().len(), 1);
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_refuses_when_nothing_is_deployed_at_the_verifier_address
}

/// Contract: a transport refusal of the first read → `Err(Evm(Rpc(
///   error)))`.
/// Arrange: `push_failure_msg("code read refused")`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!(result, Err(VerifyOnChainErrorReturn::Evm(
///   EvmVerifierErrorReturn::Rpc(error))) if error.to_string().contains(
///   "code read refused"))`, the pattern because `TransportError` omits
///   `PartialEq`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_transports_refusal_of_the_code_read() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_failure_msg("code read refused");
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &built.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(error)))
                    if error.to_string().contains("code read refused")
            ));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_returns_the_transports_refusal_of_the_code_read
}

/// Contract: a transport refusal that is not a JSON-RPC error → `Rpc`,
///   never a verdict.
/// Arrange: verifier code and `push_failure_msg("call refused")`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!` over `Rpc(error)` with the text `"call refused"`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_transports_refusal_of_the_verifying_call() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_failure_msg("call refused");
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &built.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(error)))
                    if error.to_string().contains("call refused")
            ));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_returns_the_transports_refusal_of_the_verifying_call
}

/// Contract: a JSON-RPC error whose payload holds no revert data →
///   `Rpc`, never a verdict.
/// Arrange: verifier code and `push_failure(ErrorPayload::
///   method_not_found())`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!(result, Err(VerifyOnChainErrorReturn::Evm(
///   EvmVerifierErrorReturn::Rpc(error))) if error.as_error_resp().
///   is_some_and(|payload| payload.code == -32601))`, the code an
///   independent literal, the outcome differing from the revert block's
///   so a refusal decided as a verdict by any JSON-RPC error fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_returns_a_json_rpc_refusal_without_revert_data_as_an_rpc_error() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_failure(ErrorPayload::method_not_found());
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &built.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(error)))
                    if error.as_error_resp().is_some_and(|payload| payload.code == -32601)
            ));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_returns_a_json_rpc_refusal_without_revert_data_as_an_rpc_error
}

/// Contract: a verifying-call result that is not a thirty-two-byte
///   boolean word → `Err(Evm(ResultDecode(error)))`.
/// Arrange: verifier code and an answer of `Bytes::from(vec![0x01])`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!(result, Err(VerifyOnChainErrorReturn::Evm(
///   EvmVerifierErrorReturn::ResultDecode(error))) if error.to_string()
///   == <bool as SolValue>::abi_decode(&[0x01]).unwrap_err().
///   to_string())`, the carried error compared by its text since
///   `alloy`'s error is carried whole and not assumed comparable.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_refusal_of_a_result_that_does_not_decode() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(vec![0x01]));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &built.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::Evm(
                    EvmVerifierErrorReturn::ResultDecode(error)
                )) if error.to_string()
                    == <bool as SolValue>::abi_decode(&[0x01])
                        .unwrap_err()
                        .to_string()
            ));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_returns_the_refusal_of_a_result_that_does_not_decode
}

/// Contract: an accepting call followed by the chain adapter's refusal
///   of the fee estimation → `Err(Evm(Fee(error)))`, `error` unchanged.
/// Arrange: the case params' `chain` selection the literal
///   `ChainSelection::Mock { failure_mode: MockIChainAdapterFailureMode
///   ::TransportRefused }` and answers verifier code and `true`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!(result, Err(VerifyOnChainErrorReturn::Evm(
///   EvmVerifierErrorReturn::Fee(EstimateFeeErrorReturn::Mock(
///   MockIChainAdapterEstimateFeeErrorReturn::TransportRefused)))))`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', and random-source's mock concretes and the chain's
///   mock concrete in its `TransportRefused` mode, each through its
///   factory, and the delivery proof's mock concrete in its default
///   mode.
#[test]
fn verify_on_chain_returns_the_chain_adapters_fee_refusal() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
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
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            let mut true_word = vec![0x00; 32];
            true_word[31] = 0x01;
            self.asserter.push_success(&Bytes::from(true_word));
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                payload.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                payload.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = payload.delivery_proof.proof_from_components(
                build_proof_from_components_params(Default::default()),
                build_proof_from_components_payload(components, Default::default()),
            );
            let Ok(built) = built else {
                panic!("the adapter builds a proof from its own form's components")
            };

            // Act
            let result = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &built.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Fee(
                    EstimateFeeErrorReturn::Mock(
                        MockIChainAdapterEstimateFeeErrorReturn::TransportRefused
                    )
                )))
            ));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(EvmVerifierCaseParamsOverrides {
            chain: Some(CreateChainParams {
                selection: ChainSelection::Mock {
                    failure_mode: MockIChainAdapterFailureMode::TransportRefused,
                },
            }),
            ..Default::default()
        }),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_returns_the_chain_adapters_fee_refusal
}
