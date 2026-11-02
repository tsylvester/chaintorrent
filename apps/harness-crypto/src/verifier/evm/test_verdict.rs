#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use alloy::primitives::{Bytes, U64, U256};
use alloy::rpc::json_rpc::ErrorPayload;
use alloy::sol_types::SolCall;
use alloy::transports::mock::Asserter;
use chain::IChainForms;
use pairing::{IPairingArithmetic, IPairingReference, VerifierGroupArithmetic};
use proof::{
    AlgebraicStatement, DeliveryProofComponents, DeliveryProofWireComponentsOverrides,
    DeliveryRelation, IDeliveryProofAdapter, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_delivery_proof_wire_components,
    build_proof_from_components_params, build_proof_from_components_payload,
    build_transfer_responses, build_verify_payload,
};
use serde_json::value::RawValue;

use super::super::factory::provides::{
    IVerifierAdapter, L1DataFee, VerifyCostOverrides, VerifyOnChainSuccessReturn,
    build_verify_cost, build_verify_on_chain_params,
};
use super::interface::IEvmVerifierCaseConsumer;
use super::mock::*;
use super::{IDeliveryVerifier, IGasPriceOracle};

/// Contract: the verifier has code, the call returns `true`, and the
///   oracle has no code → `Ok(Accepted { cost: VerifyCost {
///   l2_execution_gas, l1_data_fee: L1DataFee::NotApplicable } })`.
/// Arrange: answers verifier code, `true`, gas `73_421`, empty oracle
///   code.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  the result equals `Ok(VerifyOnChainSuccessReturn::Accepted {
///   cost: build_verify_cost(VerifyCostOverrides { l2_execution_gas:
///   Some(73_421), l1_data_fee: Some(L1DataFee::NotApplicable) }) })`
///   and `read_q().len()` equals `0`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_accepted_with_the_gas_estimate_and_no_fee_where_the_oracle_has_no_code()
{
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
            self.asserter.push_success(&Bytes::new());
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
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
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
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        l2_execution_gas: Some(73_421),
                        l1_data_fee: Some(L1DataFee::NotApplicable),
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
    // verify_on_chain_returns_accepted_with_the_gas_estimate_and_no_fee_where_the_oracle_has_no_code
}

/// Contract: a non-empty oracle code reads the chain identifier and the
///   oracle's fee → `Charged { wei }`.
/// Arrange: answers verifier code, `true`, gas `73_421`, oracle code,
///   chain identifier `84_532`, and a fee of `5_000_000`.
/// Act:     `verify_on_chain` over a transfer.
/// Assert:  the result equals `Ok(Accepted { cost })` with
///   `l1_data_fee` `L1DataFee::Charged { wei: 5_000_000 }` and
///   `read_q().len()` equals `0`, the fee distinct from the gas so a
///   swapped read fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_accepted_with_the_fee_the_oracle_reads() {
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
                IDeliveryVerifier::verifyTransferCall::abi_encode_returns(&true),
            ));
            self.asserter.push_success(&U64::from(73_421));
            self.asserter.push_success(&Bytes::from(vec![0x60]));
            self.asserter.push_success(&U64::from(84_532));
            self.asserter.push_success(&Bytes::from(
                IGasPriceOracle::getL1FeeCall::abi_encode_returns(&U256::from(5_000_000_u64)),
            ));
            let subject = build_evm_verifier(
                pairing,
                delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let responses = build_transfer_responses(pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Transfer),
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
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
                    AlgebraicStatement::Transfer(&statement),
                    &built.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        l2_execution_gas: Some(73_421),
                        l1_data_fee: Some(L1DataFee::Charged { wei: 5_000_000 }),
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
    // verify_on_chain_returns_accepted_with_the_fee_the_oracle_reads
}

/// Contract: a fee equal to `u128::MAX` is admitted → `Charged { wei:
///   u128::MAX }`.
/// Arrange: the oracle's fee the thirty-two big-endian bytes of
///   `u128::MAX` behind verifier code, `true`, gas `73_421`, oracle
///   code, and chain identifier `84_532`.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  the result equals `Ok(Accepted { cost })` with `Charged {
///   wei: u128::MAX }`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_a_fee_at_the_u128_limit() {
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
            self.asserter.push_success(&Bytes::from(
                IGasPriceOracle::getL1FeeCall::abi_encode_returns(&U256::from(u128::MAX)),
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
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
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
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        l2_execution_gas: Some(73_421),
                        l1_data_fee: Some(L1DataFee::Charged { wei: u128::MAX }),
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
    // verify_on_chain_returns_a_fee_at_the_u128_limit
}

/// Contract: the call returns `false` → `Ok(Rejected)`, no further read.
/// Arrange: answers verifier code and `false`, followed by a gas
///   estimate and an oracle code that must stay unread.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  the result equals `Ok(VerifyOnChainSuccessReturn::Rejected)`
///   and `read_q().len()` equals `2`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_rejected_when_the_contract_returns_false() {
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
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&false),
            ));
            self.asserter.push_success(&U64::from(73_421));
            self.asserter.push_success(&Bytes::new());
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
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
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
            assert_eq!(result.ok(), Some(VerifyOnChainSuccessReturn::Rejected));
            assert_eq!(self.asserter.read_q().len(), 2);
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

/// Contract: a JSON-RPC error whose data holds revert bytes on the
///   verifying call → `Ok(Reverted { data })`.
/// Arrange: answers verifier code and the revert, followed by an answer
///   that must stay unread.
/// Act:     `verify_on_chain` over a mint.
/// Assert:  the result equals `Ok(VerifyOnChainSuccessReturn::Reverted {
///   data: vec![0x08, 0xc3, 0x79, 0xa0] })` and `read_q().len()` equals
///   `1`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verify_on_chain_returns_the_revert_data_when_the_call_reverts() {
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
            self.asserter.push_failure(ErrorPayload {
                code: 3,
                message: "execution reverted".into(),
                data: Some(RawValue::from_string("\"0x08c379a0\"".to_string()).unwrap()),
            });
            self.asserter.push_success(&U64::from(73_421));
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
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
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
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Reverted {
                    data: vec![0x08, 0xc3, 0x79, 0xa0],
                })
            );
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
    // verify_on_chain_returns_the_revert_data_when_the_call_reverts
}
