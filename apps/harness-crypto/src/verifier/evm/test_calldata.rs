#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, Bytes, FixedBytes, U256, keccak256};
use alloy::sol_types::{SolCall, SolValue};
use alloy::transports::mock::Asserter;
use chain::{IChainAdapter, IChainForms, IEntitlementStateAdapter};
use encoding::{CanonicalFieldValue, FromFieldParams, ICanonicalField};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingArithmetic, IPairingReference,
};
use proof::{
    AlgebraicMintStatementOverrides, AlgebraicStatement, AlgebraicTransferStatementOverrides,
    DeliveryContextOverrides, DeliveryProofComponents, IDeliveryProofAdapter, MintPurpose,
    MintResponsesOverrides, MintSecondGroupFirstMessagesOverrides, TransferPurpose,
    TransferResponsesOverrides, TransferSecondGroupFirstMessagesOverrides,
    build_algebraic_mint_statement, build_algebraic_transfer_statement, build_delivery_context,
    build_delivery_proof_wire_components, build_mint_responses,
    build_mint_second_group_first_messages, build_transfer_responses,
    build_transfer_second_group_first_messages,
};

use super::IDeliveryVerifier;
use super::interface::{
    ConsumeEvmVerifierCaseParams, ConsumeEvmVerifierCasePayload, IEvmVerifierCaseConsumer,
};
use super::mock::*;

/// Contract: a mint statement's calldata begins with the selector of
///   `verifyMint` over the interface's two struct types.
/// Arrange: the default mint statement and `MintBothGroups` components
///   at their default responses.
/// Act:     `calldata`.
/// Assert:  the first four bytes equal the first four bytes of
///   `keccak256("verifyMint((bytes32,uint16,uint256,bytes20,bytes32,
///   bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,
///   uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,
///   uint256,uint256,uint256,uint256,uint256,bytes))")`, a text
///   independent of the interface.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_selects_verify_mint_for_a_mint_statement() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            assert_eq!(
                calldata[..4],
                keccak256(
                    "verifyMint((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,bytes))"
                )[..4]
            );
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
    // calldata_selects_verify_mint_for_a_mint_statement
}

/// Contract: a transfer statement's calldata begins with the selector of
///   `verifyTransfer`.
/// Arrange: the default transfer statement and `TransferBothGroups`
///   components at their default responses.
/// Act:     `calldata`.
/// Assert:  the first four bytes equal those of
///   `keccak256("verifyTransfer((bytes32,uint16,uint256,bytes20,bytes32,
///   bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,
///   uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,
///   bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,
///   uint256,bytes))")`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_selects_verify_transfer_for_a_transfer_statement() {
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
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            assert_eq!(
                calldata[..4],
                keccak256(
                    "verifyTransfer((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,uint256,bytes))"
                )[..4]
            );
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
    // calldata_selects_verify_transfer_for_a_transfer_statement
}

/// Contract: each context field reaches its member, the chain identifier
///   as `chain`, the entitlement contract, seller, and buyer as
///   `bytes20`, the entitlements as `uint256`, the intervals as
///   `uint64`, the suite, hash, and set as `bytes32`, the suite's
///   version as `uint16`, and the expiry.
/// Arrange: a context by `build_delivery_context` over
///   `MintPurpose::Mint` with `chain` the chain-identifier form of
///   `Unsigned256` bytes `0x01` through `0x20`, `entitlement_contract`,
///   `seller`, and `buyer` three distinct identity forms of
///   `FixedBytes20` bytes `[0x51; 20]`, `[0x52; 20]`, and `[0x53; 20]`,
///   `source_entitlement` and `target_entitlement` two distinct
///   entitlement forms of `Unsigned256` bytes `[0x61; 32]` and `[0x62;
///   32]`, `old_interval` and `new_interval` the interval forms of
///   `Unsigned64` `0x0102_0304_0506_0708` and `0x1112_1314_1516_1718`,
///   and `expiry` `4_102_444_800`, with `MintBothGroups` components.
/// Act:     `calldata`.
/// Assert:  the decoded context's `chain` equals `U256::from_be_bytes`
///   of `0x01` through `0x20`, its twenty-byte members equal the three
///   literals each in its member, its entitlements and intervals equal
///   the literals, `suiteVersion` equals `1`, and `expiry` equals
///   `4_102_444_800`, by `assert_eq!`, every member distinct from every
///   other of its type so a swap fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_a_mint_statements_context_members_from_the_forms_and_domain_values() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let chain_literal: [u8; 32] =
                core::array::from_fn(|index| u8::try_from(index + 1).unwrap());
            let Ok(chain) = <D::Forms as IChainForms>::ChainIdentifier::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256(chain_literal),
            ) else {
                panic!("the chain-identifier form admits a 256-bit field")
            };
            let Ok(entitlement_contract) = <D::Forms as IChainForms>::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x51; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(source_entitlement) = <D::Forms as IChainForms>::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x61; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(target_entitlement) = <D::Forms as IChainForms>::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x62; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(old_interval) = <D::Forms as IChainForms>::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(new_interval) = <D::Forms as IChainForms>::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x1112_1314_1516_1718),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(seller) = <D::Forms as IChainForms>::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x52; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(buyer) = <D::Forms as IChainForms>::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x53; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let context = build_delivery_context::<D::Forms, MintPurpose>(
                MintPurpose::Mint,
                DeliveryContextOverrides::<D::Forms> {
                    chain: Some(chain.value),
                    entitlement_contract: Some(entitlement_contract.value),
                    source_entitlement: Some(source_entitlement.value),
                    target_entitlement: Some(target_entitlement.value),
                    old_interval: Some(old_interval.value),
                    new_interval: Some(new_interval.value),
                    seller: Some(seller.value),
                    buyer: Some(buyer.value),
                    expiry: Some(4_102_444_800),
                    ..Default::default()
                },
            );
            let statement = build_algebraic_mint_statement::<P, D::Forms>(
                payload.pairing,
                AlgebraicMintStatementOverrides {
                    context: Some(context),
                    ..Default::default()
                },
            );
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let context = call.statement.context;
            assert_eq!(context.chain, U256::from_be_bytes(chain_literal));
            assert_eq!(
                context.entitlementContract,
                FixedBytes::<20>::from([0x51; 20])
            );
            assert_eq!(context.seller, FixedBytes::<20>::from([0x52; 20]));
            assert_eq!(context.buyer, FixedBytes::<20>::from([0x53; 20]));
            assert_eq!(context.sourceEntitlement, U256::from_be_bytes([0x61; 32]));
            assert_eq!(context.targetEntitlement, U256::from_be_bytes([0x62; 32]));
            assert_eq!(context.oldInterval, 0x0102_0304_0506_0708);
            assert_eq!(context.newInterval, 0x1112_1314_1516_1718);
            assert_eq!(context.suiteVersion, 1);
            assert_eq!(context.expiry, 4_102_444_800);
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
    // calldata_encodes_a_mint_statements_context_members_from_the_forms_and_domain_values
}

/// Contract: the context's purpose is the purpose's code → `purpose` `4`
///   for `MintPurpose::Replacement`.
/// Arrange: the default statement with its context built over
///   `MintPurpose::Replacement` and `MintBothGroups` components, and a
///   `MintPurpose::Mint` run over the default context.
/// Act:     `calldata`, twice.
/// Assert:  the decoded `purpose` equals `4`, and a `MintPurpose::Mint`
///   run's equals `1`, by `assert_eq!`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_the_purpose_code_of_a_mint_replacement() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let replacement = build_algebraic_mint_statement::<P, D::Forms>(
                payload.pairing,
                AlgebraicMintStatementOverrides {
                    context: Some(build_delivery_context::<D::Forms, MintPurpose>(
                        MintPurpose::Replacement,
                        Default::default(),
                    )),
                    ..Default::default()
                },
            );
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge.clone(),
                responses: build_mint_responses(payload.pairing, Default::default()),
            };

            // Act
            let replaced = subject.calldata(
                &AlgebraicStatement::Mint(&replacement),
                DeliveryProofComponents::MintBothGroups {
                    challenge: wire.challenge.clone(),
                    responses: build_mint_responses(payload.pairing, Default::default()),
                },
            );
            let minted = subject.calldata(&AlgebraicStatement::Mint(&mint), components);

            // Assert
            let Ok(replaced) = replaced else {
                panic!("the replacement statement encodes")
            };
            let Ok(minted) = minted else {
                panic!("the mint statement encodes")
            };
            assert_eq!(
                IDeliveryVerifier::verifyMintCall::abi_decode(&replaced)
                    .expect("the calldata decodes under its own call type")
                    .statement
                    .context
                    .purpose,
                4
            );
            assert_eq!(
                IDeliveryVerifier::verifyMintCall::abi_decode(&minted)
                    .expect("the calldata decodes under its own call type")
                    .statement
                    .context
                    .purpose,
                1
            );
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
    // calldata_encodes_the_purpose_code_of_a_mint_replacement
}

/// Contract: `hpub`, `identity_element`, `buyer_keys.pk1`,
///   `buyer_keys.pk2`, `envelope.c1`, `envelope.c2`, `envelope.d1`, and
///   `envelope.d2` reach `hpub`, `identityElement`, `buyerPk1`,
///   `buyerPk2`, `envelopeC1`, `envelopeC2`, `envelopeD1`, and
///   `envelopeD2`, each as the pairing's precompile encoding of its
///   group.
/// Arrange: the default mint statement, whose eight elements are
///   pairwise distinct encodings, with `MintBothGroups` components.
/// Act:     `calldata`.
/// Assert:  each decoded member equals `pairing.encode_g1` or
///   `encode_g2` of the statement's element of its name and the eight
///   members are pairwise distinct, by `assert_eq!` and `assert_ne!`,
///   so two members of one group exchanged fail.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_each_group_element_of_a_mint_statement_into_its_member() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let Ok(hpub) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.hpub.clone(),
                },
            );
            let Ok(identity_element) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.identity_element.clone(),
                },
            );
            let Ok(buyer_pk1) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.buyer_keys.pk1.clone(),
                },
            );
            let Ok(buyer_pk2) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.buyer_keys.pk2.clone(),
                },
            );
            let Ok(envelope_c1) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.envelope.c1.clone(),
                },
            );
            let Ok(envelope_c2) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.envelope.c2.clone(),
                },
            );
            let Ok(envelope_d1) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.envelope.d1.clone(),
                },
            );
            let Ok(envelope_d2) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.envelope.d2.clone(),
                },
            );
            let members = [
                call.statement.hpub.clone(),
                call.statement.identityElement.clone(),
                call.statement.buyerPk1.clone(),
                call.statement.buyerPk2.clone(),
                call.statement.envelopeC1.clone(),
                call.statement.envelopeC2.clone(),
                call.statement.envelopeD1.clone(),
                call.statement.envelopeD2.clone(),
            ];
            let expected = [
                hpub.bytes.as_ref(),
                identity_element.bytes.as_ref(),
                buyer_pk1.bytes.as_ref(),
                buyer_pk2.bytes.as_ref(),
                envelope_c1.bytes.as_ref(),
                envelope_c2.bytes.as_ref(),
                envelope_d1.bytes.as_ref(),
                envelope_d2.bytes.as_ref(),
            ];
            for (member, expected) in members.iter().zip(expected.iter()) {
                assert_eq!(member.as_ref(), *expected);
            }
            for left in 0..members.len() {
                for right in (left + 1)..members.len() {
                    assert_ne!(members[left], members[right]);
                }
            }
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
    // calldata_encodes_each_group_element_of_a_mint_statement_into_its_member
}

/// Contract: the challenge and the responses reach `challenge`,
///   `alpha`, `r`, `rho`, and `sigma` as thirty-two-byte words in that
///   order.
/// Arrange: `MintBothGroups` components over `build_mint_responses` at
///   its distinct defaults and the challenge
///   `build_delivery_proof_wire_components` holds.
/// Act:     `calldata`.
/// Assert:  each decoded member equals `U256::from_be_bytes` of the
///   pairing's `encode_scalar` of the scalar of its name and the five
///   are pairwise distinct, so `rho` and `sigma` exchanged fail.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_a_mint_proofs_scalars_in_witness_order() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let responses =
                build_mint_responses(payload.pairing, MintResponsesOverrides::default());
            fn scalar_word<P: IPairingArithmetic>(pairing: &P, scalar: P::Scalar) -> U256 {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                let bytes: [u8; 32] = encoded
                    .bytes
                    .expose()
                    .as_ref()
                    .try_into()
                    .expect("the pairing encodes a scalar as thirty-two bytes");
                U256::from_be_bytes(bytes)
            }
            let expected = [
                scalar_word(payload.pairing, wire.challenge.clone()),
                scalar_word(payload.pairing, responses.alpha.clone()),
                scalar_word(payload.pairing, responses.r.clone()),
                scalar_word(payload.pairing, responses.rho.clone()),
                scalar_word(payload.pairing, responses.sigma.clone()),
            ];
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses,
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let members = [
                call.proof.challenge,
                call.proof.alpha,
                call.proof.r,
                call.proof.rho,
                call.proof.sigma,
            ];
            assert_eq!(members, expected);
            for left in 0..members.len() {
                for right in (left + 1)..members.len() {
                    assert_ne!(members[left], members[right]);
                }
            }
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
    // calldata_encodes_a_mint_proofs_scalars_in_witness_order
}

/// Contract: `MintBothGroups` components → `secondGroupFirstMessages`
///   empty.
/// Arrange: the default mint statement and `MintBothGroups` components.
/// Act:     `calldata`.
/// Assert:  the decoded member's length equals `0`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_carries_no_second_group_first_messages_for_a_mint_under_the_both_groups_form() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            assert_eq!(call.proof.secondGroupFirstMessages.len(), 0);
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
    // calldata_carries_no_second_group_first_messages_for_a_mint_under_the_both_groups_form
}

/// Contract: `MintFirstGroupOnly` components' first messages reach
///   `secondGroupFirstMessages` as their `encode_g2` bytes concatenated
///   in the order `hpub`, `d1`, `d2`.
/// Arrange: `MintFirstGroupOnly` components whose `first_messages` is
///   `build_mint_second_group_first_messages` at its three distinct
///   defaults.
/// Act:     `calldata`.
/// Assert:  the decoded member equals the three encodings concatenated
///   in that order by `assert_eq!`, and differs from the order `d1`,
///   `hpub`, `d2`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_concatenates_a_mints_second_group_first_messages_as_hpub_d1_d2() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let first_messages = build_mint_second_group_first_messages(
                payload.pairing,
                MintSecondGroupFirstMessagesOverrides::default(),
            );
            let Ok(hpub) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.hpub.clone(),
                },
            );
            let Ok(d1) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d1.clone(),
                },
            );
            let Ok(d2) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d2.clone(),
                },
            );
            let components = DeliveryProofComponents::MintFirstGroupOnly {
                challenge: wire.challenge,
                responses: build_mint_responses(payload.pairing, Default::default()),
                first_messages,
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let mut expected = Vec::new();
            expected.extend_from_slice(hpub.bytes.as_ref());
            expected.extend_from_slice(d1.bytes.as_ref());
            expected.extend_from_slice(d2.bytes.as_ref());
            assert_eq!(call.proof.secondGroupFirstMessages.as_ref(), expected);
            let mut reordered = Vec::new();
            reordered.extend_from_slice(d1.bytes.as_ref());
            reordered.extend_from_slice(hpub.bytes.as_ref());
            reordered.extend_from_slice(d2.bytes.as_ref());
            assert_ne!(call.proof.secondGroupFirstMessages.as_ref(), reordered);
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
    // calldata_concatenates_a_mints_second_group_first_messages_as_hpub_d1_d2
}

/// Contract: as the mint's over `TransferPurpose::Transfer`, whose code
///   is `2`.
/// Arrange: the same literals and `TransferBothGroups` components.
/// Act:     `calldata`.
/// Assert:  the decoded context equals the mint block's literals with
///   `purpose` `2`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_a_transfer_statements_context_members_from_the_forms_and_domain_values() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let chain_literal: [u8; 32] =
                core::array::from_fn(|index| u8::try_from(index + 1).unwrap());
            let Ok(chain) = <D::Forms as IChainForms>::ChainIdentifier::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256(chain_literal),
            ) else {
                panic!("the chain-identifier form admits a 256-bit field")
            };
            let Ok(entitlement_contract) = <D::Forms as IChainForms>::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x51; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(source_entitlement) = <D::Forms as IChainForms>::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x61; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(target_entitlement) = <D::Forms as IChainForms>::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x62; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(old_interval) = <D::Forms as IChainForms>::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(new_interval) = <D::Forms as IChainForms>::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x1112_1314_1516_1718),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(seller) = <D::Forms as IChainForms>::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x52; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(buyer) = <D::Forms as IChainForms>::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x53; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let context = build_delivery_context::<D::Forms, TransferPurpose>(
                TransferPurpose::Transfer,
                DeliveryContextOverrides::<D::Forms> {
                    chain: Some(chain.value),
                    entitlement_contract: Some(entitlement_contract.value),
                    source_entitlement: Some(source_entitlement.value),
                    target_entitlement: Some(target_entitlement.value),
                    old_interval: Some(old_interval.value),
                    new_interval: Some(new_interval.value),
                    seller: Some(seller.value),
                    buyer: Some(buyer.value),
                    expiry: Some(4_102_444_800),
                    ..Default::default()
                },
            );
            let statement = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                AlgebraicTransferStatementOverrides {
                    context: Some(context),
                    ..Default::default()
                },
            );
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let context = call.statement.context;
            assert_eq!(context.chain, U256::from_be_bytes(chain_literal));
            assert_eq!(
                context.entitlementContract,
                FixedBytes::<20>::from([0x51; 20])
            );
            assert_eq!(context.seller, FixedBytes::<20>::from([0x52; 20]));
            assert_eq!(context.buyer, FixedBytes::<20>::from([0x53; 20]));
            assert_eq!(context.sourceEntitlement, U256::from_be_bytes([0x61; 32]));
            assert_eq!(context.targetEntitlement, U256::from_be_bytes([0x62; 32]));
            assert_eq!(context.oldInterval, 0x0102_0304_0506_0708);
            assert_eq!(context.newInterval, 0x1112_1314_1516_1718);
            assert_eq!(context.suiteVersion, 1);
            assert_eq!(context.purpose, 2);
            assert_eq!(context.expiry, 4_102_444_800);
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
    // calldata_encodes_a_transfer_statements_context_members_from_the_forms_and_domain_values
}

/// Contract: `TransferPurpose::Grant` → `purpose` `3`.
/// Arrange: the default transfer statement over `TransferPurpose::Grant`
///   and `TransferBothGroups` components.
/// Act:     `calldata`.
/// Assert:  the decoded `purpose` equals `3`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_the_purpose_code_of_a_transfer_grant() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                AlgebraicTransferStatementOverrides {
                    context: Some(build_delivery_context::<D::Forms, TransferPurpose>(
                        TransferPurpose::Grant,
                        Default::default(),
                    )),
                    ..Default::default()
                },
            );
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the statement and components encode")
            };
            assert_eq!(
                IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                    .expect("the calldata decodes under its own call type")
                    .statement
                    .context
                    .purpose,
                3
            );
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
    // calldata_encodes_the_purpose_code_of_a_transfer_grant
}

/// Contract: `identity_element`, `seller_keys.pk1`, `seller_keys.pk2`,
///   `buyer_keys.pk1`, `buyer_keys.pk2`, `old_envelope.c1` through `d2`,
///   and `new_envelope.c1` through `d2` reach `identityElement`,
///   `sellerPk1`, `sellerPk2`, `buyerPk1`, `buyerPk2`, `oldEnvelopeC1`
///   through `oldEnvelopeD2`, and `newEnvelopeC1` through
///   `newEnvelopeD2`.
/// Arrange: the default transfer statement and `TransferBothGroups`
///   components.
/// Act:     `calldata`.
/// Assert:  each decoded member equals the pairing's encoding of its
///   element and the thirteen members are pairwise distinct.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_each_group_element_of_a_transfer_statement_into_its_member() {
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
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let Ok(identity_element) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.identity_element.clone(),
                },
            );
            let Ok(seller_pk1) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.seller_keys.pk1.clone(),
                },
            );
            let Ok(seller_pk2) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.seller_keys.pk2.clone(),
                },
            );
            let Ok(buyer_pk1) = payload.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.buyer_keys.pk1.clone(),
                },
            );
            let Ok(buyer_pk2) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.buyer_keys.pk2.clone(),
                },
            );
            let members = [
                (
                    call.statement.identityElement.as_ref(),
                    identity_element.bytes.as_ref(),
                ),
                (call.statement.sellerPk1.as_ref(), seller_pk1.bytes.as_ref()),
                (call.statement.sellerPk2.as_ref(), seller_pk2.bytes.as_ref()),
                (call.statement.buyerPk1.as_ref(), buyer_pk1.bytes.as_ref()),
                (call.statement.buyerPk2.as_ref(), buyer_pk2.bytes.as_ref()),
            ];
            for (member, expected) in members {
                assert_eq!(member, expected);
            }
            for (member, element) in [
                (
                    call.statement.oldEnvelopeC1.as_ref(),
                    statement.old_envelope.c1.clone(),
                ),
                (
                    call.statement.oldEnvelopeC2.as_ref(),
                    statement.old_envelope.c2.clone(),
                ),
            ] {
                let Ok(expected) = payload
                    .pairing
                    .encode_g1(EncodeG1Params, EncodeG1Payload { point: element });
                assert_eq!(member, expected.bytes.as_ref());
            }
            for (member, element) in [
                (
                    call.statement.oldEnvelopeD1.as_ref(),
                    statement.old_envelope.d1.clone(),
                ),
                (
                    call.statement.oldEnvelopeD2.as_ref(),
                    statement.old_envelope.d2.clone(),
                ),
                (
                    call.statement.newEnvelopeD1.as_ref(),
                    statement.new_envelope.d1.clone(),
                ),
                (
                    call.statement.newEnvelopeD2.as_ref(),
                    statement.new_envelope.d2.clone(),
                ),
            ] {
                let Ok(expected) = payload
                    .pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point: element });
                assert_eq!(member, expected.bytes.as_ref());
            }
            for (member, element) in [
                (
                    call.statement.newEnvelopeC1.as_ref(),
                    statement.new_envelope.c1.clone(),
                ),
                (
                    call.statement.newEnvelopeC2.as_ref(),
                    statement.new_envelope.c2.clone(),
                ),
            ] {
                let Ok(expected) = payload
                    .pairing
                    .encode_g1(EncodeG1Params, EncodeG1Payload { point: element });
                assert_eq!(member, expected.bytes.as_ref());
            }
            let all = [
                call.statement.identityElement.clone(),
                call.statement.sellerPk1.clone(),
                call.statement.sellerPk2.clone(),
                call.statement.buyerPk1.clone(),
                call.statement.buyerPk2.clone(),
                call.statement.oldEnvelopeC1.clone(),
                call.statement.oldEnvelopeC2.clone(),
                call.statement.oldEnvelopeD1.clone(),
                call.statement.oldEnvelopeD2.clone(),
                call.statement.newEnvelopeC1.clone(),
                call.statement.newEnvelopeC2.clone(),
                call.statement.newEnvelopeD1.clone(),
                call.statement.newEnvelopeD2.clone(),
            ];
            for left in 0..all.len() {
                for right in (left + 1)..all.len() {
                    assert_ne!(all[left], all[right]);
                }
            }
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
    // calldata_encodes_each_group_element_of_a_transfer_statement_into_its_member
}

/// Contract: `challenge`, `x`, `y`, `s`, `rho`, and `sigma` as
///   thirty-two-byte words in that order.
/// Arrange: `TransferBothGroups` components over
///   `build_transfer_responses` at its distinct defaults and the
///   challenge.
/// Act:     `calldata`.
/// Assert:  each member equals its scalar's word and the six are
///   pairwise distinct.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_a_transfer_proofs_scalars_in_witness_order() {
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
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let responses =
                build_transfer_responses(payload.pairing, TransferResponsesOverrides::default());
            fn scalar_word<P: IPairingArithmetic>(pairing: &P, scalar: P::Scalar) -> U256 {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                let bytes: [u8; 32] = encoded
                    .bytes
                    .expose()
                    .as_ref()
                    .try_into()
                    .expect("the pairing encodes a scalar as thirty-two bytes");
                U256::from_be_bytes(bytes)
            }
            let expected = [
                scalar_word(payload.pairing, wire.challenge.clone()),
                scalar_word(payload.pairing, responses.x.clone()),
                scalar_word(payload.pairing, responses.y.clone()),
                scalar_word(payload.pairing, responses.s.clone()),
                scalar_word(payload.pairing, responses.rho.clone()),
                scalar_word(payload.pairing, responses.sigma.clone()),
            ];
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses,
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let members = [
                call.proof.challenge,
                call.proof.x,
                call.proof.y,
                call.proof.s,
                call.proof.rho,
                call.proof.sigma,
            ];
            assert_eq!(members, expected);
            for left in 0..members.len() {
                for right in (left + 1)..members.len() {
                    assert_ne!(members[left], members[right]);
                }
            }
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
    // calldata_encodes_a_transfer_proofs_scalars_in_witness_order
}

/// Contract: `TransferBothGroups` components → `secondGroupFirstMessages`
///   empty.
/// Arrange: the default transfer statement and `TransferBothGroups`
///   components.
/// Act:     `calldata`.
/// Assert:  the decoded member's length equals `0`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_carries_no_second_group_first_messages_for_a_transfer_under_the_both_groups_form() {
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
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(payload.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            assert_eq!(call.proof.secondGroupFirstMessages.len(), 0);
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
    // calldata_carries_no_second_group_first_messages_for_a_transfer_under_the_both_groups_form
}

/// Contract: `TransferFirstGroupOnly` components' first messages reach
///   `secondGroupFirstMessages` concatenated in the order `pk2`, `d1`,
///   `d2`.
/// Arrange: `TransferFirstGroupOnly` components whose `first_messages`
///   is `build_transfer_second_group_first_messages` at its three
///   distinct defaults.
/// Act:     `calldata`.
/// Assert:  the decoded member equals the three encodings concatenated
///   in that order, and differs from the order `d1`, `pk2`, `d2`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_concatenates_a_transfers_second_group_first_messages_as_pk2_d1_d2() {
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
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let first_messages = build_transfer_second_group_first_messages(
                payload.pairing,
                TransferSecondGroupFirstMessagesOverrides::default(),
            );
            let Ok(pk2) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.pk2.clone(),
                },
            );
            let Ok(d1) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d1.clone(),
                },
            );
            let Ok(d2) = payload.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d2.clone(),
                },
            );
            let components = DeliveryProofComponents::TransferFirstGroupOnly {
                challenge: wire.challenge,
                responses: build_transfer_responses(payload.pairing, Default::default()),
                first_messages,
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let mut expected = Vec::new();
            expected.extend_from_slice(pk2.bytes.as_ref());
            expected.extend_from_slice(d1.bytes.as_ref());
            expected.extend_from_slice(d2.bytes.as_ref());
            assert_eq!(call.proof.secondGroupFirstMessages.as_ref(), expected);
            let mut reordered = Vec::new();
            reordered.extend_from_slice(d1.bytes.as_ref());
            reordered.extend_from_slice(pk2.bytes.as_ref());
            reordered.extend_from_slice(d2.bytes.as_ref());
            assert_ne!(call.proof.secondGroupFirstMessages.as_ref(), reordered);
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
    // calldata_concatenates_a_transfers_second_group_first_messages_as_pk2_d1_d2
}

/// Contract: the request the verifying call sends is addressed to the
///   verifier and carries the calldata.
/// Arrange: a subject built over `verifier_address`
///   `FixedBytes20([0x42; 20])` and the calldata `[0x01, 0x02, 0x03]`.
/// Act:     `verifier_request`.
/// Assert:  `TransactionBuilder::to` returns `Some(Address::from([0x42;
///   20]))` and `TransactionBuilder::input` returns the three bytes, by
///   `assert_eq!`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn verifier_request_targets_the_verifier_with_the_calldata() {
    // Arrange
    let asserter = Asserter::default();
    let calldata = vec![0x01, 0x02, 0x03];

    struct Consumer<'a> {
        asserter: &'a Asserter,
        calldata: &'a [u8],
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

            // Act
            let request = subject.verifier_request(self.calldata);

            // Assert
            assert_eq!(
                TransactionBuilder::to(&request),
                Some(Address::from([0x42; 20]))
            );
            assert_eq!(
                TransactionBuilder::input(&request),
                Some(&Bytes::from_static(&[0x01, 0x02, 0x03]))
            );
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
                calldata: &calldata,
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
    // verifier_request_targets_the_verifier_with_the_calldata
}

/// Contract: a mint's calldata over the pairing family's mock concrete —
///   every `bytes` member holding a first-group element is `8` bytes,
///   every member holding a second-group element `8` bytes,
///   `secondGroupFirstMessages` `24` bytes under the mock's
///   `FirstGroupOnly` declaration, and every scalar member `32` bytes,
///   the mock concrete's `encode_g1`, `encode_g2`, and `encode_scalar`
///   widths written as literals in the test.
/// Arrange: the default mint statement and `MintFirstGroupOnly`
///   components.
/// Act:     `calldata`.
/// Assert:  each decoded member's length equals its literal by
///   `assert_eq!`, the literals independent of any pairing operation,
///   so a member emitted at another width fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn calldata_encodes_a_mints_members_at_the_mock_pairings_encoding_widths() {
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
            let subject = build_evm_verifier(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(payload.pairing, Default::default());
            let components = DeliveryProofComponents::MintFirstGroupOnly {
                challenge: wire.challenge,
                responses: build_mint_responses(payload.pairing, Default::default()),
                first_messages: build_mint_second_group_first_messages(
                    payload.pairing,
                    Default::default(),
                ),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            for member in [
                &call.statement.hpub,
                &call.statement.identityElement,
                &call.statement.buyerPk1,
                &call.statement.buyerPk2,
                &call.statement.envelopeC1,
                &call.statement.envelopeC2,
                &call.statement.envelopeD1,
                &call.statement.envelopeD2,
            ] {
                assert_eq!(member.as_ref().len(), 8);
            }
            assert_eq!(call.proof.secondGroupFirstMessages.len(), 24);
            for member in [
                call.proof.challenge,
                call.proof.alpha,
                call.proof.r,
                call.proof.rho,
                call.proof.sigma,
            ] {
                assert_eq!(member.abi_encode().len(), 32);
            }
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
    // calldata_encodes_a_mints_members_at_the_mock_pairings_encoding_widths
}
