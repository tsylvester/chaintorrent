#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;

use alloy::primitives::{Bytes, FixedBytes, U64, U256};
use alloy::rpc::json_rpc::ErrorPayload;
use alloy::sol_types::SolCall;
use alloy::transports::mock::Asserter;
use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, ICanonicalField, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, ToFieldParams, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::EnvelopeAlgebra;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, EncodeG2Params,
    EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, PAIRING_CONCRETES, PairingCurve, VerifierGroupArithmetic,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use proof::{
    AlgebraicStatement, ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload,
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryProofComponents, DeliveryProofWireComponentsOverrides, DeliveryRelation,
    IDeliveryProofAdapter, IDeliveryProofConsumer, ProofComponentsParams, ProveMintParams,
    ProveTransferParams, VerifyParams, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload,
    build_delivery_proof_wire_components, build_mint_second_group_first_messages,
    build_proof_components_payload, build_prove_mint_payload, build_prove_transfer_payload,
    build_transfer_responses, build_transfer_second_group_first_messages, build_verify_payload,
    create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};
use serde_json::value::RawValue;

use super::super::factory::provides::{
    IVerifierAdapter, L1DataFee, VerifyCostOverrides, VerifyOnChainErrorReturn,
    VerifyOnChainSuccessReturn, build_verify_cost, build_verify_on_chain_params,
};
use super::mock::*;
use super::{IDeliveryVerifier, IGasPriceOracle};

/// Contract: entry
///   `a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths`;
///   `calldata` over the statement `build_algebraic_mint_statement` or
///   `build_algebraic_transfer_statement` builds and the components of the
///   proof the real delivery proof adapter's `prove_mint` or
///   `prove_transfer` returns over the witnesses `build_prove_mint_payload`
///   or `build_prove_transfer_payload` holds, read by `proof_components`,
///   decoded by `verifyMintCall::abi_decode` or
///   `verifyTransferCall::abi_decode`; every `bytes` member that holds a
///   first-group element is the first group's precompile encoding length of
///   the case's curve, `64` on BN254 and `128` on BLS12-381, every member
///   that holds a second-group element `128` on BN254 and `256` on
///   BLS12-381, `secondGroupFirstMessages` three second-group encodings
///   long in the order `hpub`, `d1`, `d2` for a mint and `pk2`, `d1`, `d2`
///   for a transfer where the pairing declares `FirstGroupOnly` and empty
///   where it declares `BothGroups`, and every scalar member the
///   thirty-two-byte word of the encoded scalar.
/// Arrange: every member of `PAIRING_CONCRETES`, nested over every
///   selection of `DECLARED_ENCODING_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, and every member of
///   `DELIVERY_PROOF_CONCRETES` in the verifier form the resolved pairing
///   declares, the random source `create_random_source` returns for
///   `RandomSourceKind::OperatingSystem`; the subject built by
///   `build_evm_verifier` over a provider an `Asserter` scripts; the real
///   statements `build_algebraic_mint_statement` and
///   `build_algebraic_transfer_statement` build over the resolved pairing
///   and forms.
/// Act:     `prove_mint` and `prove_transfer` on the received adapter over
///   the payloads the witness builders hold, `proof_components` over each
///   returned proof, and `calldata` over each statement and its components.
/// Assert:  in place, the decoded mint call's `hpub`, `buyerPk2`,
///   `envelopeD1`, and `envelopeD2` members are the second-group length,
///   `identityElement`, `buyerPk1`, `envelopeC1`, and `envelopeC2` the
///   first-group length, the decoded transfer call's `identityElement`,
///   `sellerPk1`, `buyerPk1`, and the `C1`/`C2` envelope members the
///   first-group length, `sellerPk2`, `buyerPk2`, and the `D1`/`D2`
///   envelope members the second-group length, each proof's scalar members
///   equal `U256::from_be_bytes` of the pairing's `encode_scalar` of the
///   scalar of its name, and `secondGroupFirstMessages` equals the
///   concatenation of `encode_g2` over the first messages the adapter's
///   declared form carries; `read_q().len()` equals `0`, the chain never
///   read.
/// Boundary: `create_pairing`, the concrete's `try_new`,
///   `create_encoding`, the selection's `try_new`, `create_chain_forms`,
///   the concrete's `try_new`, `create_hash_to_scalar`, the selection's
///   `try_new`, `create_random_source`, `create_delivery_proof`, the
///   concrete's `try_new`, `prove_mint`, `prove_transfer`,
///   `proof_components`, `EvmVerifier::try_new_mocked`, `calldata`,
///   `mint_calldata`, `transfer_calldata`, `delivery_context`, and
///   `encode_scalar_word`.
/// Mocked:  the chain, scripted through an `Asserter` and never read; the
///   operating system's generator runs real.
#[test]
fn a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let (first_group, second_group) = match P::DECLARATION.curve {
                PairingCurve::Bn254 => (64_usize, 128_usize),
                PairingCurve::Bls12381 => (128_usize, 256_usize),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => (8_usize, 8_usize),
            };
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
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let components = payload
                .adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proved.proof, Default::default()),
                )
                .expect("reading a proof's components is infallible")
                .components;
            let expected_scalars = match &components {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => [
                    scalar_word(self.pairing, challenge.clone()),
                    scalar_word(self.pairing, responses.alpha.clone()),
                    scalar_word(self.pairing, responses.r.clone()),
                    scalar_word(self.pairing, responses.rho.clone()),
                    scalar_word(self.pairing, responses.sigma.clone()),
                ],
                DeliveryProofComponents::TransferBothGroups { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                    panic!("a mint proof's components carry the mint relation")
                }
            };
            let mut expected_first_messages = Vec::new();
            if let DeliveryProofComponents::MintFirstGroupOnly { first_messages, .. } = &components
            {
                for point in [
                    first_messages.hpub.clone(),
                    first_messages.d1.clone(),
                    first_messages.d2.clone(),
                ] {
                    let encoded = self
                        .pairing
                        .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                        .expect("the pairing encodes its second-group elements");
                    expected_first_messages.extend_from_slice(encoded.bytes.as_ref());
                }
            }

            // Act
            let encoded_mint = subject.calldata(&AlgebraicStatement::Mint(&mint), components);

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
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };
            let components = payload
                .adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proved.proof, Default::default()),
                )
                .expect("reading a proof's components is infallible")
                .components;
            let expected_scalars = match &components {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => [
                    scalar_word(self.pairing, challenge.clone()),
                    scalar_word(self.pairing, responses.x.clone()),
                    scalar_word(self.pairing, responses.y.clone()),
                    scalar_word(self.pairing, responses.s.clone()),
                    scalar_word(self.pairing, responses.rho.clone()),
                    scalar_word(self.pairing, responses.sigma.clone()),
                ],
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::MintFirstGroupOnly { .. } => {
                    panic!("a transfer proof's components carry the transfer relation")
                }
            };
            let mut expected_first_messages = Vec::new();
            if let DeliveryProofComponents::TransferFirstGroupOnly { first_messages, .. } =
                &components
            {
                for point in [
                    first_messages.pk2.clone(),
                    first_messages.d1.clone(),
                    first_messages.d2.clone(),
                ] {
                    let encoded = self
                        .pairing
                        .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                        .expect("the pairing encodes its second-group elements");
                    expected_first_messages.extend_from_slice(encoded.bytes.as_ref());
                }
            }

            // Act
            let encoded_transfer =
                subject.calldata(&AlgebraicStatement::Transfer(&transfer), components);

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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths
}

/// Contract: entry
///   `an_accepted_verdict_and_its_costs_reach_the_caller_for_a_real_proof`;
///   `verify_on_chain` over the same real statement and proof, on a provider
///   scripted to answer the verifier's code non-empty, the verifying call
///   `true`, the gas estimate a value other than every default, and the
///   oracle's code first empty and then non-empty with a chain identifier
///   and a fee; `Ok(VerifyOnChainSuccessReturn::Accepted { cost })` with
///   `l2_execution_gas` the scripted gas and `l1_data_fee` `NotApplicable`
///   for the empty oracle code and `Charged { wei }` with the scripted fee
///   for the non-empty.
/// Arrange: the cases of the calldata block, each relation's real proof
///   `prove_mint` or `prove_transfer` returns; the `Asserter` answers, in
///   order, verifier code, `true`, gas `73_421`, empty oracle code, then
///   verifier code, `true`, gas `73_421`, oracle code, chain identifier
///   `84_532`, and the fee `5_000_000`.
/// Act:     `verify_on_chain` twice over the same statement and proof.
/// Assert:  the first result equals `Ok(Accepted { cost: build_verify_cost(
///   VerifyCostOverrides { l2_execution_gas: Some(73_421), l1_data_fee:
///   Some(L1DataFee::NotApplicable) }) })`, the second
///   `Ok(Accepted { cost })` with `l1_data_fee` `L1DataFee::Charged { wei:
///   5_000_000 }`, and `read_q().len()` equals `0`, so a swapped read or a
///   fee read where the oracle has no code fails.
/// Boundary: as the calldata block, plus `verify_on_chain`,
///   `unsigned_transaction`, `verifier_request`, `fee_request`, and
///   `read_chain`.
/// Mocked:  the chain, scripted through an `Asserter`; the operating
///   system's generator runs real.
#[test]
fn an_accepted_verdict_and_its_costs_reach_the_caller_for_a_real_proof() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let accepted =
                Bytes::from(IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true));
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&accepted);
            asserter.push_success(&U64::from(73_421));
            asserter.push_success(&Bytes::new());
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&accepted);
            asserter.push_success(&U64::from(73_421));
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&U64::from(84_532));
            asserter.push_success(&Bytes::from(
                IGasPriceOracle::getL1FeeCall::abi_encode_returns(&U256::from(5_000_000_u64)),
            ));
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };

            // Act
            let first = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                first.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        l2_execution_gas: Some(73_421),
                        l1_data_fee: Some(L1DataFee::NotApplicable),
                    }),
                })
            );

            // Act
            let second = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                second.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        l2_execution_gas: Some(73_421),
                        l1_data_fee: Some(L1DataFee::Charged { wei: 5_000_000 }),
                    }),
                })
            );
            assert_eq!(asserter.read_q().len(), 0);

            // Arrange
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&accepted);
            asserter.push_success(&U64::from(73_421));
            asserter.push_success(&Bytes::new());
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&accepted);
            asserter.push_success(&U64::from(73_421));
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&U64::from(84_532));
            asserter.push_success(&Bytes::from(
                IGasPriceOracle::getL1FeeCall::abi_encode_returns(&U256::from(5_000_000_u64)),
            ));
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };

            // Act
            let first = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                first.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        l2_execution_gas: Some(73_421),
                        l1_data_fee: Some(L1DataFee::NotApplicable),
                    }),
                })
            );

            // Act
            let second = subject.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                second.ok(),
                Some(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(VerifyCostOverrides {
                        l2_execution_gas: Some(73_421),
                        l1_data_fee: Some(L1DataFee::Charged { wei: 5_000_000 }),
                    }),
                })
            );
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // an_accepted_verdict_and_its_costs_reach_the_caller_for_a_real_proof
}

/// Contract: entry `a_rejecting_chain_returns_rejected_for_a_real_proof`;
///   `verify_on_chain` over a real statement and proof, on a provider
///   scripted to answer the verifier's code non-empty and the verifying
///   call `false`, returns `Ok(VerifyOnChainSuccessReturn::Rejected)` and
///   the scripted answers after the verifying call stay unread.
/// Arrange: the cases of the calldata block; the `Asserter` answers
///   verifier code, `false`, and one further answer, per relation.
/// Act:     `verify_on_chain` over each relation's real statement and
///   proof.
/// Assert:  `Ok(VerifyOnChainSuccessReturn::Rejected)` in place and
///   `read_q().len()` equals `1` after each call, a rejected delivery a
///   verdict that reads no cost.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `verify_on_chain`, `calldata`, and
///   `read_chain`.
/// Mocked:  the chain, scripted through an `Asserter`; the operating
///   system's generator runs real.
#[test]
fn a_rejecting_chain_returns_rejected_for_a_real_proof() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&false),
            ));
            asserter.push_success(&U64::from(73_421));
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
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
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyTransferCall::abi_encode_returns(&false),
            ));
            asserter.push_success(&U64::from(73_421));
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // a_rejecting_chain_returns_rejected_for_a_real_proof
}

/// Contract: entry `a_reverting_chain_returns_the_revert_data_for_a_real_proof`;
///   `verify_on_chain` over a real statement and proof, on a provider
///   scripted to answer the verifier's code non-empty and refuse the
///   verifying call with a JSON-RPC error carrying the hex data
///   `0x08c379a0`, returns `Ok(VerifyOnChainSuccessReturn::Reverted { data
///   })` with `data` `[0x08, 0xc3, 0x79, 0xa0]` and the scripted answers
///   after the call unread.
/// Arrange: the cases of the calldata block; the `Asserter` answers
///   verifier code, a refusal whose error carries `0x08c379a0`, and one
///   further answer, per relation.
/// Act:     `verify_on_chain` over each relation's real statement and
///   proof.
/// Assert:  `Ok(VerifyOnChainSuccessReturn::Reverted { data })` with
///   `data` `vec![0x08, 0xc3, 0x79, 0xa0]` in place and `read_q().len()`
///   growing by the unread answer after each call, a revert a verdict,
///   never an `Rpc` error.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `verify_on_chain`, `calldata`, and
///   `read_chain`.
/// Mocked:  the chain, scripted through an `Asserter`; the operating
///   system's generator runs real.
#[test]
fn a_reverting_chain_returns_the_revert_data_for_a_real_proof() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_failure(ErrorPayload {
                code: 3,
                message: "execution reverted".into(),
                data: Some(RawValue::from_string("\"0x08c379a0\"".to_string()).unwrap()),
            });
            asserter.push_success(&U64::from(73_421));
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
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
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Reverted {
                    data: vec![0x08, 0xc3, 0x79, 0xa0],
                })
            );
            assert_eq!(asserter.read_q().len(), 1);

            // Arrange
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_failure(ErrorPayload {
                code: 3,
                message: "execution reverted".into(),
                data: Some(RawValue::from_string("\"0x08c379a0\"".to_string()).unwrap()),
            });
            asserter.push_success(&U64::from(73_421));
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
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
            assert_eq!(
                result.ok(),
                Some(VerifyOnChainSuccessReturn::Reverted {
                    data: vec![0x08, 0xc3, 0x79, 0xa0],
                })
            );
            assert_eq!(asserter.read_q().len(), 1);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // a_reverting_chain_returns_the_revert_data_for_a_real_proof
}

/// Contract: entry
///   `a_real_proof_of_the_other_relation_is_refused_before_any_chain_read`;
///   `verify_on_chain` over a real mint statement and the real proof
///   `prove_transfer` returned, and over a real transfer statement and the
///   proof `prove_mint` returned, the scripted provider holding one answer;
///   `Err(VerifyOnChainErrorReturn::RelationMismatch { statement, proof })`
///   with the relations the order names and the scripted answer unread.
/// Arrange: the cases of the calldata block; both real statements, both
///   real proofs, and one scripted answer per `verify_on_chain` call.
/// Act:     `verify_on_chain` over the mint statement with the transfer
///   proof, then over the transfer statement with the mint proof.
/// Assert:  `matches!` on `Err(VerifyOnChainErrorReturn::RelationMismatch)`
///   with `statement`/`proof` `Mint`/`Transfer` the first call and
///   `Transfer`/`Mint` the second, and `read_q().len()` unchanged per
///   call, the refusal decided from the real proof's components before the
///   chain is read.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `verify_on_chain`, and `calldata`; the
///   scripted chain is never read.
/// Mocked:  the chain, scripted through an `Asserter`; the operating
///   system's generator runs real.
#[test]
fn a_real_proof_of_the_other_relation_is_refused_before_any_chain_read() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let mint_proof = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
            );
            let Ok(mint_proof) = mint_proof else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let transfer_proof = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
            );
            let Ok(transfer_proof) = transfer_proof else {
                panic!("the real adapter proves the transfer statement it is handed")
            };
            asserter.push_success(&Bytes::from(vec![0x60]));

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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // a_real_proof_of_the_other_relation_is_refused_before_any_chain_read
}

/// Contract: carried entry
///   `chain_forms_identity_survives_its_field_round_trip`, restated: the
///   identity forms at the entitlement contract, the seller, and the buyer
///   reach the calldata as twenty-byte members, each the field its form's
///   `to_field` yields.
/// Arrange: the cases of the calldata block; components the relation's
///   wire shape carries in the resolved pairing's declared verifier form.
/// Act:     `calldata` over each relation's real statement and components,
///   decoded by the generated calls.
/// Assert:  in place, `entitlementContract`, `seller`, and `buyer` equal
///   `FixedBytes::<20>::from` of the `FixedBytes20` field each identity
///   form's `to_field` returns; `read_q().len()` equals `0`, the chain
///   never read.
/// Boundary: `create_pairing`, the arm's `try_new`, `create_encoding`,
///   `create_chain_forms`, `create_hash_to_scalar`, `create_random_source`,
///   `create_delivery_proof`, each concrete's `try_new`,
///   `EvmVerifier::try_new_mocked`, `calldata`, `delivery_context`, and
///   each form's `to_field`.
/// Mocked:  the chain, scripted through an `Asserter` and never read; the
///   operating system's generator runs real.
#[test]
fn chain_forms_identity_survives_its_field_round_trip() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                self.pairing,
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

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Mint(&mint), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::FixedBytes20(contract) =
                F::Identity::to_field(ToFieldParams, &mint.context.entitlement_contract)
                    .expect("the identity form yields a field")
                    .field
            else {
                panic!("the identity form's field is twenty bytes")
            };
            assert_eq!(
                call.statement.context.entitlementContract,
                FixedBytes::<20>::from(contract)
            );
            let CanonicalFieldValue::FixedBytes20(seller) =
                F::Identity::to_field(ToFieldParams, &mint.context.seller)
                    .expect("the identity form yields a field")
                    .field
            else {
                panic!("the identity form's field is twenty bytes")
            };
            assert_eq!(
                call.statement.context.seller,
                FixedBytes::<20>::from(seller)
            );
            let CanonicalFieldValue::FixedBytes20(buyer) =
                F::Identity::to_field(ToFieldParams, &mint.context.buyer)
                    .expect("the identity form yields a field")
                    .field
            else {
                panic!("the identity form's field is twenty bytes")
            };
            assert_eq!(call.statement.context.buyer, FixedBytes::<20>::from(buyer));

            // Arrange
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Transfer),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    responses: {
                        let responses = build_transfer_responses(self.pairing, Default::default());
                        Some(vec![
                            responses.x,
                            responses.y,
                            responses.s,
                            responses.rho,
                            responses.sigma,
                        ])
                    },
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_transfer_second_group_first_messages(
                                self.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.pk2, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Transfer(&transfer), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::FixedBytes20(contract) =
                F::Identity::to_field(ToFieldParams, &transfer.context.entitlement_contract)
                    .expect("the identity form yields a field")
                    .field
            else {
                panic!("the identity form's field is twenty bytes")
            };
            assert_eq!(
                call.statement.context.entitlementContract,
                FixedBytes::<20>::from(contract)
            );
            let CanonicalFieldValue::FixedBytes20(seller) =
                F::Identity::to_field(ToFieldParams, &transfer.context.seller)
                    .expect("the identity form yields a field")
                    .field
            else {
                panic!("the identity form's field is twenty bytes")
            };
            assert_eq!(
                call.statement.context.seller,
                FixedBytes::<20>::from(seller)
            );
            let CanonicalFieldValue::FixedBytes20(buyer) =
                F::Identity::to_field(ToFieldParams, &transfer.context.buyer)
                    .expect("the identity form yields a field")
                    .field
            else {
                panic!("the identity form's field is twenty bytes")
            };
            assert_eq!(call.statement.context.buyer, FixedBytes::<20>::from(buyer));
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // chain_forms_identity_survives_its_field_round_trip
}

/// Contract: carried entry
///   `chain_forms_entitlement_survives_its_field_round_trip`, restated: the
///   source and target entitlement forms reach the calldata as 256-bit
///   members, each the field its form's `to_field` yields.
/// Arrange: the cases of the identity block.
/// Act:     `calldata` over each relation's real statement and components,
///   decoded by the generated calls.
/// Assert:  in place, `sourceEntitlement` and `targetEntitlement` equal
///   `U256::from_be_bytes` of the `Unsigned256` field each entitlement
///   form's `to_field` returns; `read_q().len()` equals `0`.
/// Boundary: the identity block's.
/// Mocked:  the chain, scripted through an `Asserter` and never read; the
///   operating system's generator runs real.
#[test]
fn chain_forms_entitlement_survives_its_field_round_trip() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                self.pairing,
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

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Mint(&mint), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::Unsigned256(source) =
                F::Entitlement::to_field(ToFieldParams, &mint.context.source_entitlement)
                    .expect("the entitlement form yields a field")
                    .field
            else {
                panic!("the entitlement form's field is two hundred fifty-six bits")
            };
            assert_eq!(
                call.statement.context.sourceEntitlement,
                U256::from_be_bytes(source)
            );
            let CanonicalFieldValue::Unsigned256(target) =
                F::Entitlement::to_field(ToFieldParams, &mint.context.target_entitlement)
                    .expect("the entitlement form yields a field")
                    .field
            else {
                panic!("the entitlement form's field is two hundred fifty-six bits")
            };
            assert_eq!(
                call.statement.context.targetEntitlement,
                U256::from_be_bytes(target)
            );

            // Arrange
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Transfer),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    responses: {
                        let responses = build_transfer_responses(self.pairing, Default::default());
                        Some(vec![
                            responses.x,
                            responses.y,
                            responses.s,
                            responses.rho,
                            responses.sigma,
                        ])
                    },
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_transfer_second_group_first_messages(
                                self.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.pk2, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Transfer(&transfer), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::Unsigned256(source) =
                F::Entitlement::to_field(ToFieldParams, &transfer.context.source_entitlement)
                    .expect("the entitlement form yields a field")
                    .field
            else {
                panic!("the entitlement form's field is two hundred fifty-six bits")
            };
            assert_eq!(
                call.statement.context.sourceEntitlement,
                U256::from_be_bytes(source)
            );
            let CanonicalFieldValue::Unsigned256(target) =
                F::Entitlement::to_field(ToFieldParams, &transfer.context.target_entitlement)
                    .expect("the entitlement form yields a field")
                    .field
            else {
                panic!("the entitlement form's field is two hundred fifty-six bits")
            };
            assert_eq!(
                call.statement.context.targetEntitlement,
                U256::from_be_bytes(target)
            );
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // chain_forms_entitlement_survives_its_field_round_trip
}

/// Contract: carried entry
///   `chain_forms_interval_survives_its_field_round_trip`, restated: the
///   old and new interval forms reach the calldata as 64-bit members, each
///   the field its form's `to_field` yields.
/// Arrange: the cases of the identity block.
/// Act:     `calldata` over each relation's real statement and components,
///   decoded by the generated calls.
/// Assert:  in place, `oldInterval` and `newInterval` equal the `Unsigned64`
///   field each interval form's `to_field` returns; `read_q().len()` equals
///   `0`.
/// Boundary: the identity block's.
/// Mocked:  the chain, scripted through an `Asserter` and never read; the
///   operating system's generator runs real.
#[test]
fn chain_forms_interval_survives_its_field_round_trip() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                self.pairing,
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

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Mint(&mint), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::Unsigned64(old) =
                F::Interval::to_field(ToFieldParams, &mint.context.old_interval)
                    .expect("the interval form yields a field")
                    .field
            else {
                panic!("the interval form's field is sixty-four bits")
            };
            assert_eq!(call.statement.context.oldInterval, old);
            let CanonicalFieldValue::Unsigned64(new) =
                F::Interval::to_field(ToFieldParams, &mint.context.new_interval)
                    .expect("the interval form yields a field")
                    .field
            else {
                panic!("the interval form's field is sixty-four bits")
            };
            assert_eq!(call.statement.context.newInterval, new);

            // Arrange
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Transfer),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    responses: {
                        let responses = build_transfer_responses(self.pairing, Default::default());
                        Some(vec![
                            responses.x,
                            responses.y,
                            responses.s,
                            responses.rho,
                            responses.sigma,
                        ])
                    },
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_transfer_second_group_first_messages(
                                self.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.pk2, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Transfer(&transfer), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::Unsigned64(old) =
                F::Interval::to_field(ToFieldParams, &transfer.context.old_interval)
                    .expect("the interval form yields a field")
                    .field
            else {
                panic!("the interval form's field is sixty-four bits")
            };
            assert_eq!(call.statement.context.oldInterval, old);
            let CanonicalFieldValue::Unsigned64(new) =
                F::Interval::to_field(ToFieldParams, &transfer.context.new_interval)
                    .expect("the interval form yields a field")
                    .field
            else {
                panic!("the interval form's field is sixty-four bits")
            };
            assert_eq!(call.statement.context.newInterval, new);
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // chain_forms_interval_survives_its_field_round_trip
}

/// Contract: carried entry
///   `chain_forms_chain_identifier_survives_its_field_round_trip`, restated:
///   the chain-identifier form reaches the calldata as a 256-bit member,
///   the field its form's `to_field` yields.
/// Arrange: the cases of the identity block.
/// Act:     `calldata` over each relation's real statement and components,
///   decoded by the generated calls.
/// Assert:  in place, `chain` equals `U256::from_be_bytes` of the
///   `Unsigned256` field the chain-identifier form's `to_field` returns;
///   `read_q().len()` equals `0`.
/// Boundary: the identity block's.
/// Mocked:  the chain, scripted through an `Asserter` and never read; the
///   operating system's generator runs real.
#[test]
fn chain_forms_chain_identifier_survives_its_field_round_trip() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Mint),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_mint_second_group_first_messages(
                                self.pairing,
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

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Mint(&mint), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::Unsigned256(chain) =
                F::ChainIdentifier::to_field(ToFieldParams, &mint.context.chain)
                    .expect("the chain-identifier form yields a field")
                    .field
            else {
                panic!("the chain-identifier form's field is two hundred fifty-six bits")
            };
            assert_eq!(call.statement.context.chain, U256::from_be_bytes(chain));

            // Arrange
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let wire = build_delivery_proof_wire_components(
                self.pairing,
                DeliveryProofWireComponentsOverrides {
                    relation: Some(DeliveryRelation::Transfer),
                    verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                    responses: {
                        let responses = build_transfer_responses(self.pairing, Default::default());
                        Some(vec![
                            responses.x,
                            responses.y,
                            responses.s,
                            responses.rho,
                            responses.sigma,
                        ])
                    },
                    second_group_first_messages: match P::DECLARATION.verifier_group_arithmetic {
                        VerifierGroupArithmetic::BothGroups => None,
                        VerifierGroupArithmetic::FirstGroupOnly => {
                            let messages = build_transfer_second_group_first_messages(
                                self.pairing,
                                Default::default(),
                            );
                            Some(vec![messages.pk2, messages.d1, messages.d2])
                        }
                    },
                    ..Default::default()
                },
            );
            let components = DeliveryProofComponents::try_from(wire)
                .expect("the wire carries the relation's and the form's counts");

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Transfer(&transfer), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the statement and components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
            let CanonicalFieldValue::Unsigned256(chain) =
                F::ChainIdentifier::to_field(ToFieldParams, &transfer.context.chain)
                    .expect("the chain-identifier form yields a field")
                    .field
            else {
                panic!("the chain-identifier form's field is two hundred fifty-six bits")
            };
            assert_eq!(call.statement.context.chain, U256::from_be_bytes(chain));
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // chain_forms_chain_identifier_survives_its_field_round_trip
}

/// Contract: carried entry
///   `a_proof_the_adapter_rebuilds_from_its_components_verifies`, restated:
///   the components `proof_components` reads from a real proof, the
///   challenge, the responses in witness order, and the second-group first
///   messages the adapter's form carries, reach the calldata in that order,
///   a mint's `alpha`, `r`, `rho`, `sigma` and a transfer's `x`, `y`, `s`,
///   `rho`, `sigma`, then the first messages.
/// Arrange: the cases of the calldata block; each real proof the adapter
///   returns over the witness payload builders' defaults, its components
///   read by `proof_components`.
/// Act:     `calldata` over each relation's real statement and its proof's
///   components, decoded by the generated calls.
/// Assert:  in place, the decoded scalar members equal
///   `U256::from_be_bytes` of the pairing's `encode_scalar` over the
///   components' scalars in witness order — `challenge`, `alpha`, `r`,
///   `rho`, `sigma` for a mint and `challenge`, `x`, `y`, `s`, `rho`,
///   `sigma` for a transfer — and `secondGroupFirstMessages` equals the
///   concatenation of `encode_g2` over `hpub`, `d1`, `d2` for a mint and
///   `pk2`, `d1`, `d2` for a transfer in the adapter's declared form, empty
///   under `BothGroups`; `read_q().len()` equals `0`.
/// Boundary: `create_pairing` through `proof_components`,
///   `EvmVerifier::try_new_mocked`, `calldata`, `mint_calldata`, and
///   `transfer_calldata`.
/// Mocked:  the chain, scripted through an `Asserter` and never read; the
///   operating system's generator runs real.
#[test]
fn a_proof_the_adapter_rebuilds_from_its_components_verifies() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
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
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let components = payload
                .adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proved.proof, Default::default()),
                )
                .expect("reading a proof's components is infallible")
                .components;
            let expected_scalars = match &components {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => [
                    scalar_word(self.pairing, challenge.clone()),
                    scalar_word(self.pairing, responses.alpha.clone()),
                    scalar_word(self.pairing, responses.r.clone()),
                    scalar_word(self.pairing, responses.rho.clone()),
                    scalar_word(self.pairing, responses.sigma.clone()),
                ],
                DeliveryProofComponents::TransferBothGroups { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                    panic!("a mint proof's components carry the mint relation")
                }
            };
            let mut expected_first_messages = Vec::new();
            if let DeliveryProofComponents::MintFirstGroupOnly { first_messages, .. } = &components
            {
                for point in [
                    first_messages.hpub.clone(),
                    first_messages.d1.clone(),
                    first_messages.d2.clone(),
                ] {
                    let encoded = self
                        .pairing
                        .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                        .expect("the pairing encodes its second-group elements");
                    expected_first_messages.extend_from_slice(encoded.bytes.as_ref());
                }
            }

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Mint(&mint), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the real statement and its proof's components encode")
            };
            let call = IDeliveryVerifier::verifyMintCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
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

            // Arrange
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };
            let components = payload
                .adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proved.proof, Default::default()),
                )
                .expect("reading a proof's components is infallible")
                .components;
            let expected_scalars = match &components {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => [
                    scalar_word(self.pairing, challenge.clone()),
                    scalar_word(self.pairing, responses.x.clone()),
                    scalar_word(self.pairing, responses.y.clone()),
                    scalar_word(self.pairing, responses.s.clone()),
                    scalar_word(self.pairing, responses.rho.clone()),
                    scalar_word(self.pairing, responses.sigma.clone()),
                ],
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::MintFirstGroupOnly { .. } => {
                    panic!("a transfer proof's components carry the transfer relation")
                }
            };
            let mut expected_first_messages = Vec::new();
            if let DeliveryProofComponents::TransferFirstGroupOnly { first_messages, .. } =
                &components
            {
                for point in [
                    first_messages.pk2.clone(),
                    first_messages.d1.clone(),
                    first_messages.d2.clone(),
                ] {
                    let encoded = self
                        .pairing
                        .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                        .expect("the pairing encodes its second-group elements");
                    expected_first_messages.extend_from_slice(encoded.bytes.as_ref());
                }
            }

            // Act
            let calldata = subject.calldata(&AlgebraicStatement::Transfer(&transfer), components);

            // Assert
            let Ok(calldata) = calldata else {
                panic!("the real statement and its proof's components encode")
            };
            let call = IDeliveryVerifier::verifyTransferCall::abi_decode(&calldata)
                .expect("the calldata decodes under its own call type");
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // a_proof_the_adapter_rebuilds_from_its_components_verifies
}

/// Contract: carried entry `a_mint_proof_the_adapter_returns_verifies`,
///   restated: the real mint proofs the own entries encode are proofs the
///   reference verifies for the statement they are formed over, and the
///   chain's verdict on them is the deployed contract's, observed by the
///   harness's run and not by this node.
/// Arrange: the cases of the calldata block; the `Asserter` answers
///   verifier code, `true`, gas `73_421`, and empty oracle code.
/// Act:     `prove_mint` over the payload `build_prove_mint_payload`
///   holds, `proof_components` over the proof, the adapter's `verify` over
///   the statement and proof, then `verify_on_chain` over the scripted
///   provider.
/// Assert:  in place, the adapter's `verify` returns
///   `Ok(VerifySuccessReturn { is_valid: true })`, `verify_on_chain`
///   returns `Ok`, and `read_q().len()` equals `0`.
/// Boundary: `create_pairing` through `proof_components`, the adapter's
///   `verify`, `EvmVerifier::try_new_mocked`, `verify_on_chain`, and
///   `read_chain`.
/// Mocked:  the chain, scripted through an `Asserter`; the operating
///   system's generator runs real.
#[test]
fn a_mint_proof_the_adapter_returns_verifies() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyMintCall::abi_encode_returns(&true),
            ));
            asserter.push_success(&U64::from(73_421));
            asserter.push_success(&Bytes::new());
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let components = payload
                .adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proved.proof, Default::default()),
                )
                .expect("reading a proof's components is infallible")
                .components;
            let relation = match &components {
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::MintFirstGroupOnly { .. } => DeliveryRelation::Mint,
                DeliveryProofComponents::TransferBothGroups { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                    DeliveryRelation::Transfer
                }
            };

            // Act
            let verified = payload.adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(relation, DeliveryRelation::Mint);
            assert_eq!(verified.ok().map(|success| success.is_valid), Some(true));

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
            assert!(result.is_ok());
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // a_mint_proof_the_adapter_returns_verifies
}

/// Contract: carried entry
///   `a_transfer_proof_the_adapter_returns_verifies`, restated: the real
///   transfer proofs the own entries encode are proofs the reference
///   verifies for the statement they are formed over, and the chain's
///   verdict on them is the deployed contract's, observed by the harness's
///   run and not by this node.
/// Arrange: the cases of the mint block, over a real transfer statement.
/// Act:     `prove_transfer` over the payload `build_prove_transfer_payload`
///   holds, `proof_components` over the proof, the adapter's `verify` over
///   the statement and proof, then `verify_on_chain` over the scripted
///   provider.
/// Assert:  in place, the adapter's `verify` returns
///   `Ok(VerifySuccessReturn { is_valid: true })`, `verify_on_chain`
///   returns `Ok`, and `read_q().len()` equals `0`.
/// Boundary: the mint block's.
/// Mocked:  the chain, scripted through an `Asserter`; the operating
///   system's generator runs real.
#[test]
fn a_transfer_proof_the_adapter_returns_verifies() {
    // Arrange
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let asserter = Asserter::default();
            asserter.push_success(&Bytes::from(vec![0x60]));
            asserter.push_success(&Bytes::from(
                IDeliveryVerifier::verifyTransferCall::abi_encode_returns(&true),
            ));
            asserter.push_success(&U64::from(73_421));
            asserter.push_success(&Bytes::new());
            let subject = build_evm_verifier(
                self.pairing,
                &payload.adapter,
                asserter.clone(),
                Default::default(),
            );
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let proved = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
            );
            let Ok(proved) = proved else {
                panic!("the real adapter proves the transfer statement it is handed")
            };
            let components = payload
                .adapter
                .proof_components(
                    ProofComponentsParams,
                    build_proof_components_payload(&proved.proof, Default::default()),
                )
                .expect("reading a proof's components is infallible")
                .components;
            let relation = match &components {
                DeliveryProofComponents::TransferBothGroups { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                    DeliveryRelation::Transfer
                }
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::MintFirstGroupOnly { .. } => DeliveryRelation::Mint,
            };

            // Act
            let verified = payload.adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&transfer),
                    &proved.proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(relation, DeliveryRelation::Transfer);
            assert_eq!(verified.ok().map(|success| success.is_valid), Some(true));

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
            assert!(result.is_ok());
            assert_eq!(asserter.read_q().len(), 0);
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
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
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(_success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                }
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
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
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            encoder: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(_success) = created else {
                    panic!("the chain-forms factory admits the selection")
                };
            }
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
                let Ok(_success) = created else {
                    panic!("the encoding factory admits the selection")
                };
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(_success) = created else {
            panic!("the pairing factory ran the consumer")
        };
    }
    // a_transfer_proof_the_adapter_returns_verifies
}
