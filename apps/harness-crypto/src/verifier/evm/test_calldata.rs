#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;

use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, FixedBytes, U256};
use alloy::sol_types::SolCall;
use alloy::transports::mock::Asserter;
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, FromFieldParams,
    ICanonicalField, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use proof::{
    AlgebraicMintStatementOverrides, AlgebraicStatement, AlgebraicTransferStatementOverrides,
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, DeliveryContextOverrides,
    DeliveryProofComponents, IDeliveryProofAdapter, IDeliveryProofConsumer, MintPurpose,
    MintResponsesOverrides, MintSecondGroupFirstMessagesOverrides, TransferPurpose,
    TransferResponsesOverrides, TransferSecondGroupFirstMessagesOverrides,
    build_algebraic_mint_statement, build_algebraic_transfer_statement,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, build_delivery_context,
    build_delivery_proof_wire_components, build_mint_responses,
    build_mint_second_group_first_messages, build_transfer_responses,
    build_transfer_second_group_first_messages, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

use super::interface::{EvmVerifier, EvmVerifierSettings};
use super::mock::*;
use super::{IDeliveryVerifier, IGasPriceOracle};

/// Contract: a mint statement's calldata begins with the selector of
///   `verifyMint` over the interface's two struct types.
/// Arrange: the default mint statement and `MintBothGroups` components
///   at their default responses.
/// Act:     `calldata`.
/// Assert:  the first four bytes equal the first four bytes of
///   `keccak256("verifyMint((bytes32,uint16,uint256,bytes20,bytes32,
///   bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,
///   uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,
///   uint256,uint256,uint256,uint256,bytes))")`, a text independent of
///   the interface.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_selects_verify_mint_for_a_mint_statement() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(self.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            assert_eq!(calldata[..4], IDeliveryVerifier::verifyMintCall::SELECTOR);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_selects_verify_transfer_for_a_transfer_statement() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(self.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            assert_eq!(
                calldata[..4],
                IDeliveryVerifier::verifyTransferCall::SELECTOR
            );
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_a_mint_statements_context_members_from_the_forms_and_domain_values() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let chain_literal: [u8; 32] =
                core::array::from_fn(|index| u8::try_from(index + 1).unwrap());
            let Ok(chain) = F::ChainIdentifier::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256(chain_literal),
            ) else {
                panic!("the chain-identifier form admits a 256-bit field")
            };
            let Ok(entitlement_contract) = F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x51; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(source_entitlement) = F::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x61; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(target_entitlement) = F::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x62; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(old_interval) = F::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(new_interval) = F::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x1112_1314_1516_1718),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(seller) = F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x52; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(buyer) = F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x53; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let context = build_delivery_context::<F, MintPurpose>(
                MintPurpose::Mint,
                DeliveryContextOverrides::<F> {
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
            let statement = build_algebraic_mint_statement::<P, F>(
                self.pairing,
                AlgebraicMintStatementOverrides {
                    context: Some(context),
                    ..Default::default()
                },
            );
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_the_purpose_code_of_a_mint_replacement() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let replacement = build_algebraic_mint_statement::<P, F>(
                self.pairing,
                AlgebraicMintStatementOverrides {
                    context: Some(build_delivery_context::<F, MintPurpose>(
                        MintPurpose::Replacement,
                        Default::default(),
                    )),
                    ..Default::default()
                },
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge.clone(),
                responses: build_mint_responses(self.pairing, Default::default()),
            };

            // Act
            let replaced = subject.calldata(
                &AlgebraicStatement::Mint(&replacement),
                DeliveryProofComponents::MintBothGroups {
                    challenge: wire.challenge.clone(),
                    responses: build_mint_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_each_group_element_of_a_mint_statement_into_its_member() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(self.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Mint(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let Ok(hpub) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.hpub.clone(),
                },
            );
            let Ok(identity_element) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.identity_element.clone(),
                },
            );
            let Ok(buyer_pk1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.buyer_keys.pk1.clone(),
                },
            );
            let Ok(buyer_pk2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.buyer_keys.pk2.clone(),
                },
            );
            let Ok(envelope_c1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.envelope.c1.clone(),
                },
            );
            let Ok(envelope_c2) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.envelope.c2.clone(),
                },
            );
            let Ok(envelope_d1) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.envelope.d1.clone(),
                },
            );
            let Ok(envelope_d2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.envelope.d2.clone(),
                },
            );
            assert_eq!(call.statement.hpub.as_ref(), hpub.bytes.as_ref());
            assert_eq!(
                call.statement.identityElement.as_ref(),
                identity_element.bytes.as_ref()
            );
            assert_eq!(call.statement.buyerPk1.as_ref(), buyer_pk1.bytes.as_ref());
            assert_eq!(call.statement.buyerPk2.as_ref(), buyer_pk2.bytes.as_ref());
            assert_eq!(
                call.statement.envelopeC1.as_ref(),
                envelope_c1.bytes.as_ref()
            );
            assert_eq!(
                call.statement.envelopeC2.as_ref(),
                envelope_c2.bytes.as_ref()
            );
            assert_eq!(
                call.statement.envelopeD1.as_ref(),
                envelope_d1.bytes.as_ref()
            );
            assert_eq!(
                call.statement.envelopeD2.as_ref(),
                envelope_d2.bytes.as_ref()
            );
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_a_mint_proofs_scalars_in_witness_order() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let responses = build_mint_responses(self.pairing, MintResponsesOverrides::default());
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
                scalar_word(self.pairing, wire.challenge.clone()),
                scalar_word(self.pairing, responses.alpha.clone()),
                scalar_word(self.pairing, responses.r.clone()),
                scalar_word(self.pairing, responses.rho.clone()),
                scalar_word(self.pairing, responses.sigma.clone()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // calldata_encodes_a_mint_proofs_scalars_in_witness_order
}

/// Contract: `MintBothGroups` components → `secondGroupFirstMessages`
///   empty.
/// Arrange: the default mint statement and `MintBothGroups` components.
/// Act:     `calldata`.
/// Assert:  the decoded member's length equals `0`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_carries_no_second_group_first_messages_for_a_mint_under_the_both_groups_form() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: wire.challenge,
                responses: build_mint_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_concatenates_a_mints_second_group_first_messages_as_hpub_d1_d2() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let first_messages = build_mint_second_group_first_messages(
                self.pairing,
                MintSecondGroupFirstMessagesOverrides::default(),
            );
            let Ok(hpub) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.hpub.clone(),
                },
            );
            let Ok(d1) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d1.clone(),
                },
            );
            let Ok(d2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d2.clone(),
                },
            );
            let components = DeliveryProofComponents::MintFirstGroupOnly {
                challenge: wire.challenge,
                responses: build_mint_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_a_transfer_statements_context_members_from_the_forms_and_domain_values() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let chain_literal: [u8; 32] =
                core::array::from_fn(|index| u8::try_from(index + 1).unwrap());
            let Ok(chain) = F::ChainIdentifier::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256(chain_literal),
            ) else {
                panic!("the chain-identifier form admits a 256-bit field")
            };
            let Ok(entitlement_contract) = F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x51; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(source_entitlement) = F::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x61; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(target_entitlement) = F::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x62; 32]),
            ) else {
                panic!("the entitlement form admits a 256-bit field")
            };
            let Ok(old_interval) = F::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(new_interval) = F::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x1112_1314_1516_1718),
            ) else {
                panic!("the interval form admits a 64-bit field")
            };
            let Ok(seller) = F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x52; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let Ok(buyer) = F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x53; 20]),
            ) else {
                panic!("the identity form admits a twenty-byte field")
            };
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                DeliveryContextOverrides::<F> {
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
            let statement = build_algebraic_transfer_statement::<P, F>(
                self.pairing,
                AlgebraicTransferStatementOverrides {
                    context: Some(context),
                    ..Default::default()
                },
            );
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // calldata_encodes_a_transfer_statements_context_members_from_the_forms_and_domain_values
}

/// Contract: `TransferPurpose::Grant` → `purpose` `3`.
/// Arrange: the default transfer statement over `TransferPurpose::Grant`
///   and `TransferBothGroups` components.
/// Act:     `calldata`.
/// Assert:  the decoded `purpose` equals `3`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_the_purpose_code_of_a_transfer_grant() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(
                self.pairing,
                AlgebraicTransferStatementOverrides {
                    context: Some(build_delivery_context::<F, TransferPurpose>(
                        TransferPurpose::Grant,
                        Default::default(),
                    )),
                    ..Default::default()
                },
            );
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_each_group_element_of_a_transfer_statement_into_its_member() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(self.pairing, Default::default()),
            };

            // Act
            let result = subject.calldata(&AlgebraicStatement::Transfer(&statement), components);

            // Assert
            let Ok(calldata) = result else {
                panic!("the default statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let Ok(identity_element) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.identity_element.clone(),
                },
            );
            let Ok(seller_pk1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.seller_keys.pk1.clone(),
                },
            );
            let Ok(seller_pk2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: statement.seller_keys.pk2.clone(),
                },
            );
            let Ok(buyer_pk1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: statement.buyer_keys.pk1.clone(),
                },
            );
            let Ok(buyer_pk2) = self.pairing.encode_g2(
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
                let Ok(expected) = self
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
                let Ok(expected) = self
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
                let Ok(expected) = self
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_encodes_a_transfer_proofs_scalars_in_witness_order() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let responses =
                build_transfer_responses(self.pairing, TransferResponsesOverrides::default());
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
                scalar_word(self.pairing, wire.challenge.clone()),
                scalar_word(self.pairing, responses.x.clone()),
                scalar_word(self.pairing, responses.y.clone()),
                scalar_word(self.pairing, responses.s.clone()),
                scalar_word(self.pairing, responses.rho.clone()),
                scalar_word(self.pairing, responses.sigma.clone()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_carries_no_second_group_first_messages_for_a_transfer_under_the_both_groups_form() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: wire.challenge,
                responses: build_transfer_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
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
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn calldata_concatenates_a_transfers_second_group_first_messages_as_pk2_d1_d2() {
    // Arrange
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                Default::default(),
            );
            let statement =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(self.pairing, Default::default());
            let first_messages = build_transfer_second_group_first_messages(
                self.pairing,
                TransferSecondGroupFirstMessagesOverrides::default(),
            );
            let Ok(pk2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.pk2.clone(),
                },
            );
            let Ok(d1) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d1.clone(),
                },
            );
            let Ok(d2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_messages.d2.clone(),
                },
            );
            let components = DeliveryProofComponents::TransferFirstGroupOnly {
                challenge: wire.challenge,
                responses: build_transfer_responses(self.pairing, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // calldata_concatenates_a_transfers_second_group_first_messages_as_pk2_d1_d2
}

/// Contract: `unsigned_transaction` returns the byte `0x02` followed by
///   the RLP list of `chain_id`, a zero nonce, two zero fees,
///   `gas_limit`, the verifier's twenty bytes, a zero value, the
///   calldata, and an empty access list.
/// Arrange: `chain_id` `1`, `gas_limit` `21_000`, `verifier`
///   `Address::from([0x11; 20])`, and `calldata` `&[0xab]`.
/// Act:     `EvmVerifier::unsigned_transaction`.
/// Assert:  the result equals the bytes `0x02`, `0xe0`, `0x01`, `0x80`,
///   `0x80`, `0x80`, `0x82`, `0x52`, `0x08`, `0x94`, twenty bytes
///   `0x11`, `0x80`, `0x81`, `0xab`, `0xc0`, a literal the RLP rules fix
///   and no mention of the subject produced, thirty-four bytes long, so
///   a field omitted, reordered, or widened fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn unsigned_transaction_encodes_an_eip1559_transaction_as_the_rlp_standard_states() {
    struct DeliveryProofConsumer<P: IPairingArithmetic, F: IChainForms> {
        pairing: PhantomData<P>,
        forms: PhantomData<F>,
    }

    impl<P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            _payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let result = EvmVerifier::<P, D>::unsigned_transaction(
                1,
                21_000,
                Address::from([0x11; 20]),
                &[0xab],
            );

            // Assert
            let mut expected = vec![0x02, 0xe0, 0x01, 0x80, 0x80, 0x80, 0x82, 0x52, 0x08, 0x94];
            expected.extend_from_slice(&[0x11; 20]);
            expected.extend_from_slice(&[0x80, 0x81, 0xab, 0xc0]);
            assert_eq!(result, expected);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer::<P, F> {
                        pairing: PhantomData::<P>,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(PairingConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // unsigned_transaction_encodes_an_eip1559_transaction_as_the_rlp_standard_states
}

/// Contract: the request the verifying call and the gas estimate send is
///   addressed to the verifier and carries the calldata.
/// Arrange: a subject built over `verifier_address`
///   `FixedBytes20([0x42; 20])` and the calldata `[0x01, 0x02, 0x03]`.
/// Act:     `verifier_request`.
/// Assert:  `TransactionBuilder::to` returns `Some(Address::from([0x42;
///   20]))` and `TransactionBuilder::input` returns the three bytes, by
///   `assert_eq!`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn verifier_request_targets_the_verifier_with_the_calldata() {
    // Arrange
    let asserter = Asserter::default();
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
        ..Default::default()
    });
    let calldata = vec![0x01, 0x02, 0x03];

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        calldata: &'a [u8],
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                EvmVerifierConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
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
                Some(&alloy::primitives::Bytes::from_static(&[0x01, 0x02, 0x03]))
            );
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        calldata: &'a [u8],
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        asserter: self.asserter,
                        calldata: self.calldata,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        calldata: &'a [u8],
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                        asserter: self.asserter,
                        calldata: self.calldata,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        calldata: &'a [u8],
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                    asserter: self.asserter,
                    calldata: self.calldata,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
        asserter: &asserter,
        calldata: &calldata,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // verifier_request_targets_the_verifier_with_the_calldata
}

/// Contract: the fee request is addressed to the oracle and carries
///   `getL1Fee` over the unsigned transaction's bytes.
/// Arrange: a subject built over `gas_price_oracle`
///   `FixedBytes20([0x43; 20])` and unsigned bytes `[0x02, 0xaa, 0xbb]`.
/// Act:     `fee_request`.
/// Assert:  `TransactionBuilder::to` returns `Some(Address::from([0x43;
///   20]))`, the input's first four bytes equal those of
///   `keccak256("getL1Fee(bytes)")`, and `IGasPriceOracle::getL1FeeCall::
///   abi_decode` over the input holds `_data` equal to `[0x02, 0xaa,
///   0xbb]`, by `assert_eq!`, the verifier's address differing from the
///   oracle's so an exchanged target fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn fee_request_targets_the_oracle_with_the_get_l1_fee_call_over_the_unsigned_transaction() {
    // Arrange
    let asserter = Asserter::default();
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
        gas_price_oracle: Some(CanonicalFieldValue::FixedBytes20([0x43; 20])),
        ..Default::default()
    });
    let unsigned = vec![0x02, 0xaa, 0xbb];

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        unsigned: &'a [u8],
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                self.asserter.clone(),
                EvmVerifierConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                },
            );

            // Act
            let request = subject.fee_request(self.unsigned);

            // Assert
            assert_eq!(
                TransactionBuilder::to(&request),
                Some(Address::from([0x43; 20]))
            );
            let input = TransactionBuilder::input(&request).expect("the request carries input");
            assert_eq!(input[..4], IGasPriceOracle::getL1FeeCall::SELECTOR);
            assert_eq!(
                IGasPriceOracle::getL1FeeCall::abi_decode(input)
                    .expect("the input decodes as getL1Fee")
                    ._data
                    .as_ref(),
                &[0x02, 0xaa, 0xbb]
            );
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        unsigned: &'a [u8],
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        asserter: self.asserter,
                        unsigned: self.unsigned,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        unsigned: &'a [u8],
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                        asserter: self.asserter,
                        unsigned: self.unsigned,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        unsigned: &'a [u8],
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                    asserter: self.asserter,
                    unsigned: self.unsigned,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
        asserter: &asserter,
        unsigned: &unsigned,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // fee_request_targets_the_oracle_with_the_get_l1_fee_call_over_the_unsigned_transaction
}
