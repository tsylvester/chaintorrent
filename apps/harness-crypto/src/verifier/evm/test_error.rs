#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use alloy::primitives::{Bytes, U64, U256};
use alloy::rpc::json_rpc::ErrorPayload;
use alloy::sol_types::SolCall;
use alloy::transports::mock::Asserter;
use chain::IChainForms;
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
use serde_json::value::RawValue;

use super::super::factory::provides::{
    IVerifierAdapter, VerifyOnChainErrorReturn, build_verify_on_chain_params,
};
use super::interface::{EvmVerifierErrorReturn, EvmVerifierSettings, IEvmVerifierCaseConsumer};
use super::mock::*;
use super::{IDeliveryVerifier, IGasPriceOracle};

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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_refuses_a_mint_statement_with_a_transfer_proof() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let responses = build_transfer_responses(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Transfer),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_transfer_second_group_first_messages(
                                pairing,
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
            let built = delivery_proof.proof_from_components(
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_refuses_a_transfer_statement_with_a_mint_proof() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_refuses_when_nothing_is_deployed_at_the_verifier_address() {
    // Arrange
    let asserter = Asserter::default();
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
        ..Default::default()
    });

    struct Consumer<'a> {
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::new());
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                EvmVerifierConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                },
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                settings: &settings,
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_transports_refusal_of_the_code_read() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_failure_msg("code read refused");
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_transports_refusal_of_the_verifying_call() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_failure_msg("call refused");
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
///   `Rpc`, not `Reverted`.
/// Arrange: verifier code and `push_failure(ErrorPayload::
///   method_not_found())`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!` over `Rpc(_)`, the verdicts differing from the
///   revert block's so a revert decided by any JSON-RPC error fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_a_json_rpc_refusal_without_revert_data_as_an_rpc_error() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_failure(ErrorPayload::method_not_found());
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                    _
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
///   EvmVerifierErrorReturn::ResultDecode(_))))`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_refusal_of_a_result_that_does_not_decode() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(vec![0x01]));
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                    EvmVerifierErrorReturn::ResultDecode(_)
                ))
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

/// Contract: a refused estimate after an accepting call → `Rpc`.
/// Arrange: verifier code, `true`, and `push_failure_msg("estimate
///   refused")`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!` over `Rpc(error)` with the text `"estimate
///   refused"`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_transports_refusal_of_the_gas_estimate() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true),
            ));
            self.asserter.push_failure_msg("estimate refused");
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                    if error.to_string().contains("estimate refused")
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
    // verify_on_chain_returns_the_transports_refusal_of_the_gas_estimate
}

/// Contract: a refused oracle code read → `Rpc`.
/// Arrange: verifier code, `true`, gas, and `push_failure_msg("oracle
///   code refused")`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!` over `Rpc(error)` with the text `"oracle code
///   refused"`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_transports_refusal_of_the_oracle_code_read() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true),
            ));
            self.asserter.push_success(&U64::from(73_421));
            self.asserter.push_failure_msg("oracle code refused");
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                    if error.to_string().contains("oracle code refused")
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
    // verify_on_chain_returns_the_transports_refusal_of_the_oracle_code_read
}

/// Contract: a refused chain-identifier read after a non-empty oracle
///   code → `Rpc`.
/// Arrange: verifier code, `true`, gas, oracle code, and
///   `push_failure_msg("chain identifier refused")`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!` over `Rpc(error)` with the text `"chain
///   identifier refused"`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_transports_refusal_of_the_chain_identifier() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true),
            ));
            self.asserter.push_success(&U64::from(73_421));
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_failure_msg("chain identifier refused");
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                    if error.to_string().contains("chain identifier refused")
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
    // verify_on_chain_returns_the_transports_refusal_of_the_chain_identifier
}

/// Contract: a JSON-RPC revert on the oracle call is not a verdict →
///   `Rpc`.
/// Arrange: verifier code, `true`, gas, oracle code, chain identifier,
///   and the revert `push_failure(ErrorPayload { code: 3, .. })`
///   carrying revert data.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!` over `Rpc(_)`, the same revert the verifying
///   call's block returns as a verdict.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_any_refusal_of_the_oracles_fee_call_as_an_rpc_error() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true),
            ));
            self.asserter.push_success(&U64::from(73_421));
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&U64::from(84_532));
            self.asserter.push_failure(ErrorPayload {
                code: 3,
                message: "execution reverted".into(),
                data: Some(RawValue::from_string("\"0x08c379a0\"".to_string()).unwrap()),
            });
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                    _
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
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // verify_on_chain_returns_any_refusal_of_the_oracles_fee_call_as_an_rpc_error
}

/// Contract: an oracle result that is not a thirty-two-byte word →
///   `ResultDecode`.
/// Arrange: the reads through the chain identifier and an oracle answer
///   `Bytes::from(vec![0x01])`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!` over `ResultDecode(_)`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_refusal_of_a_fee_that_does_not_decode() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true),
            ));
            self.asserter.push_success(&U64::from(73_421));
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&U64::from(84_532));
            self.asserter.push_success(&Bytes::from(vec![0x01]));
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                    EvmVerifierErrorReturn::ResultDecode(_)
                ))
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
    // verify_on_chain_returns_the_refusal_of_a_fee_that_does_not_decode
}

/// Contract: a fee of `u128::MAX` plus one → `Err(Evm(L1FeeOutOfRange {
///   fee }))`, `fee` its thirty-two big-endian bytes.
/// Arrange: the reads through the chain identifier and the oracle's
///   answer `0x01` at index fifteen and zeros elsewhere.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  `matches!(result, Err(VerifyOnChainErrorReturn::Evm(
///   EvmVerifierErrorReturn::L1FeeOutOfRange { fee })) if fee ==
///   expected)`, `expected` that literal.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_refuses_a_fee_beyond_u128() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true),
            ));
            self.asserter.push_success(&U64::from(73_421));
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&U64::from(84_532));
            let refused_fee = U256::from(u128::MAX) + U256::from(1_u64);
            self.asserter.push_success(&Bytes::from(
                IGasPriceOracle::getL1FeeCall::abi_encode_returns(&refused_fee),
            ));
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages =
                                build_mint_second_group_first_messages(pairing, Default::default());
                            Some(vec![messages.hpub, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");
            let built = delivery_proof.proof_from_components(
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
                    EvmVerifierErrorReturn::L1FeeOutOfRange { fee }
                )) if fee == refused_fee.to_be_bytes::<32>()
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
    // verify_on_chain_refuses_a_fee_beyond_u128
}
