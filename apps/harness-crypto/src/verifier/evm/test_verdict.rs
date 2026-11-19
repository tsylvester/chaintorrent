#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use alloy::primitives::Bytes;
use alloy::rpc::json_rpc::ErrorPayload;
use alloy::transports::mock::Asserter;
use chain::{
    EstimateFeeParams, FeeRequestOverrides, IChainAdapter, IEntitlementStateAdapter,
    build_estimate_fee_payload, build_fee_request,
};
use pairing::{IPairingArithmetic, IPairingReference, VerifierGroupArithmetic};
use proof::{
    AlgebraicStatement, DeliveryProofComponents, DeliveryProofWireComponentsOverrides,
    DeliveryRelation, IDeliveryProofAdapter, build_algebraic_mint_statement,
    build_delivery_proof_wire_components, build_mint_second_group_first_messages,
    build_proof_from_components_params, build_proof_from_components_payload, build_verify_payload,
};
use serde_json::value::RawValue;

use super::super::factory::provides::{
    IVerifierAdapter, VerifyCostOverrides, VerifyOnChainSuccessReturn, build_verify_cost,
    build_verify_on_chain_params,
};
use super::interface::{
    ConsumeEvmVerifierCaseParams, ConsumeEvmVerifierCasePayload, IEvmVerifierCaseConsumer,
};
use super::mock::*;

/// Contract: the verifier has code and the call returns `true` →
///   `Ok(Accepted { cost })`, `cost.description` the `FeeDescription`
///   the chain adapter's `estimate_fee` returns for the verifying
///   transaction.
/// Arrange: answers verifier code and `true`, the received chain adapter
///   under `NoFailure`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  the result equals `Ok(VerifyOnChainSuccessReturn::Accepted {
///   cost })` with `cost.description` equal to the `FeeDescription` the
///   received chain adapter's `estimate_fee` returns over the
///   `FeeRequest` of the verifier's admitted `FixedBytes20([0x11; 20])`
///   form and the calldata the subject produces, and `read_q().len()`
///   equals `0`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_returns_accepted_with_the_fee_description_the_chain_estimates() {
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
            let wire = || {
                build_delivery_proof_wire_components(
                    payload.pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Mint),
                        verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                        second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic
                        {
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
                )
            };
            let components = DeliveryProofComponents::try_from(wire())
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
            let components = DeliveryProofComponents::try_from(wire())
                .expect("the wire carries the relation's and the form's counts");
            let Ok(calldata) = subject.calldata(&AlgebraicStatement::Mint(&statement), components)
            else {
                panic!("the statement and components encode")
            };
            let request = build_fee_request::<D::Forms>(FeeRequestOverrides {
                input: Some(calldata),
                ..Default::default()
            });
            let estimated = payload.chain.estimate_fee(
                EstimateFeeParams,
                build_estimate_fee_payload(&request, Default::default()),
            );
            let Ok(estimated) = estimated else {
                panic!("the chain adapter estimates the verifying transaction's fee")
            };
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        description: Some(estimated.description),
                        ..Default::default()
                    }),
                })
            );
            assert_eq!(self.asserter.read_q().len(), 0);
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
    // verify_on_chain_returns_accepted_with_the_fee_description_the_chain_estimates
}

/// Contract: the call returns `false` → `Ok(Rejected)`, the chain's fee
///   estimation unreached.
/// Arrange: answers verifier code and `false`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  the result equals `Ok(VerifyOnChainSuccessReturn::Rejected)`
///   and `read_q().len()` equals `0`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_returns_rejected_when_the_contract_returns_false() {
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
            self.asserter.push_success(&Bytes::from(vec![0x00; 32]));
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
            assert_eq!(result.ok(), Some(VerifyOnChainSuccessReturn::Rejected));
            assert_eq!(self.asserter.read_q().len(), 0);
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
    // verify_on_chain_returns_rejected_when_the_contract_returns_false
}

/// Contract: a JSON-RPC error carrying revert data on the verifying call
///   → `Ok(Rejected)`.
/// Arrange: answers verifier code and the revert, followed by an answer
///   that must stay unread.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  the result equals `Ok(VerifyOnChainSuccessReturn::Rejected)`
///   and `read_q().len()` equals `1`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verify_on_chain_returns_rejected_when_the_call_reverts() {
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
            self.asserter.push_failure(ErrorPayload {
                code: 3,
                message: "execution reverted".into(),
                data: Some(RawValue::from_string("\"0x08c379a0\"".to_string()).unwrap()),
            });
            self.asserter.push_success(&Bytes::new());
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
            assert_eq!(result.ok(), Some(VerifyOnChainSuccessReturn::Rejected));
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
    // verify_on_chain_returns_rejected_when_the_call_reverts
}
