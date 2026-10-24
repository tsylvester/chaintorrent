#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_calldata;
#[cfg(test)]
mod test_error;
#[cfg(test)]
mod test_verdict;

use alloy::consensus::{SignableTransaction, TxEip1559};
use alloy::eips::eip2930::AccessList;
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, Bytes, FixedBytes, TxKind, U256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::sol;
use alloy::sol_types::{SolCall, SolValue};
#[cfg(any(test, feature = "mocks"))]
use alloy::transports::mock::Asserter;
use chain::IChainForms;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldParams, ICanonicalField, ToFieldParams,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingAdapter,
};
use proof::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement, DeliveryContext,
    DeliveryProofComponents, DeliveryRelation, IDeliveryProofAdapter, MintResponses,
    ProofComponentsParams, ProofComponentsPayload, TransferResponses, VerifyAdapterPayload,
};
use tokio::runtime::Builder;
use url::Url;

use self::interface::{
    EVM_VERIFIER_DECLARATION, EvmVerifier, EvmVerifierConstructorParams, EvmVerifierContextField,
    EvmVerifierEndpoint, EvmVerifierEndpointConstructorParams,
    EvmVerifierEndpointTryNewErrorReturn, EvmVerifierEndpointTryNewReturn, EvmVerifierErrorReturn,
    EvmVerifierField, EvmVerifierTryNewErrorReturn, EvmVerifierTryNewReturn,
};
use super::factory::provides::{
    IVerifierAdapter, L1DataFee, VerifierDeclaration, VerifyCost, VerifyOnChainErrorReturn,
    VerifyOnChainParams, VerifyOnChainReturn, VerifyOnChainSuccessReturn,
};

sol!("../../contracts/evm/src/IDeliveryVerifier.sol");

sol! {
    interface IGasPriceOracle {
        function getL1Fee(bytes memory _data) external view returns (uint256);
    }
}

impl EvmVerifierEndpoint {
    pub fn try_new(
        params: EvmVerifierEndpointConstructorParams,
    ) -> EvmVerifierEndpointTryNewReturn {
        match Url::parse(&params.text) {
            Ok(url) => Ok(EvmVerifierEndpoint { url }),
            Err(error) => Err(EvmVerifierEndpointTryNewErrorReturn::Unparsable { error }),
        }
    }
}

impl<'a, P, D> EvmVerifier<'a, P, D>
where
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    fn connect(
        params: EvmVerifierConstructorParams<'a, P, D>,
        provider: DynProvider,
    ) -> EvmVerifierTryNewReturn<'a, P, D> {
        let verifier = match <D::Forms as IChainForms>::Identity::from_field(
            FromFieldParams,
            params.settings.verifier_address.clone(),
        ) {
            Ok(admitted) => admitted.value,
            Err(error) => {
                return Err(EvmVerifierTryNewErrorReturn::Identity {
                    field: EvmVerifierField::VerifierAddress,
                    error,
                });
            }
        };
        let Ok(rendered) = <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &verifier);
        let verifier = match rendered.field {
            CanonicalFieldValue::FixedBytes20(bytes) => Address::from(bytes),
            CanonicalFieldValue::FixedBytes32(_)
            | CanonicalFieldValue::Unsigned16(_)
            | CanonicalFieldValue::Unsigned32(_)
            | CanonicalFieldValue::Unsigned64(_)
            | CanonicalFieldValue::Text(_)
            | CanonicalFieldValue::Bytes(_)
            | CanonicalFieldValue::Unsigned256(_) => {
                return Err(EvmVerifierTryNewErrorReturn::IdentityKind {
                    field: EvmVerifierField::VerifierAddress,
                });
            }
        };
        let oracle = match <D::Forms as IChainForms>::Identity::from_field(
            FromFieldParams,
            params.settings.gas_price_oracle.clone(),
        ) {
            Ok(admitted) => admitted.value,
            Err(error) => {
                return Err(EvmVerifierTryNewErrorReturn::Identity {
                    field: EvmVerifierField::GasPriceOracle,
                    error,
                });
            }
        };
        let Ok(rendered) = <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &oracle);
        let oracle = match rendered.field {
            CanonicalFieldValue::FixedBytes20(bytes) => Address::from(bytes),
            CanonicalFieldValue::FixedBytes32(_)
            | CanonicalFieldValue::Unsigned16(_)
            | CanonicalFieldValue::Unsigned32(_)
            | CanonicalFieldValue::Unsigned64(_)
            | CanonicalFieldValue::Text(_)
            | CanonicalFieldValue::Bytes(_)
            | CanonicalFieldValue::Unsigned256(_) => {
                return Err(EvmVerifierTryNewErrorReturn::IdentityKind {
                    field: EvmVerifierField::GasPriceOracle,
                });
            }
        };
        let runtime = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime,
            Err(error) => return Err(EvmVerifierTryNewErrorReturn::Runtime(error)),
        };
        Ok(EvmVerifier {
            pairing: params.pairing,
            delivery_proof: params.delivery_proof,
            verifier,
            oracle,
            provider,
            runtime,
        })
    }

    pub fn try_new(
        params: EvmVerifierConstructorParams<'a, P, D>,
    ) -> EvmVerifierTryNewReturn<'a, P, D> {
        let provider = ProviderBuilder::default()
            .connect_http(params.settings.endpoint.url.clone())
            .erased();
        Self::connect(params, provider)
    }

    #[cfg(any(test, feature = "mocks"))]
    pub(crate) fn try_new_mocked(
        params: EvmVerifierConstructorParams<'a, P, D>,
        asserter: Asserter,
    ) -> EvmVerifierTryNewReturn<'a, P, D> {
        let provider = ProviderBuilder::default()
            .connect_mocked_client(asserter)
            .erased();
        Self::connect(params, provider)
    }

    fn calldata(
        &self,
        statement: &AlgebraicStatement<'_, D::Forms, P::G1, P::G2>,
        components: DeliveryProofComponents<P::Scalar, P::G2>,
    ) -> Result<Vec<u8>, VerifyOnChainErrorReturn> {
        match (statement, components) {
            (
                AlgebraicStatement::Mint(statement),
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                },
            ) => self
                .mint_calldata(statement, challenge, responses, Vec::new())
                .map_err(VerifyOnChainErrorReturn::Evm),
            (
                AlgebraicStatement::Mint(statement),
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                },
            ) => self
                .mint_calldata(
                    statement,
                    challenge,
                    responses,
                    vec![first_messages.hpub, first_messages.d1, first_messages.d2],
                )
                .map_err(VerifyOnChainErrorReturn::Evm),
            (
                AlgebraicStatement::Transfer(statement),
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                },
            ) => self
                .transfer_calldata(statement, challenge, responses, Vec::new())
                .map_err(VerifyOnChainErrorReturn::Evm),
            (
                AlgebraicStatement::Transfer(statement),
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                },
            ) => self
                .transfer_calldata(
                    statement,
                    challenge,
                    responses,
                    vec![first_messages.pk2, first_messages.d1, first_messages.d2],
                )
                .map_err(VerifyOnChainErrorReturn::Evm),
            (
                AlgebraicStatement::Mint(_),
                DeliveryProofComponents::TransferBothGroups { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. },
            ) => Err(VerifyOnChainErrorReturn::RelationMismatch {
                statement: DeliveryRelation::Mint,
                proof: DeliveryRelation::Transfer,
            }),
            (
                AlgebraicStatement::Transfer(_),
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::MintFirstGroupOnly { .. },
            ) => Err(VerifyOnChainErrorReturn::RelationMismatch {
                statement: DeliveryRelation::Transfer,
                proof: DeliveryRelation::Mint,
            }),
        }
    }

    fn delivery_context<Purpose>(
        &self,
        context: &DeliveryContext<D::Forms, Purpose>,
        purpose: u16,
    ) -> Result<IDeliveryVerifier::DeliveryContext, EvmVerifierErrorReturn> {
        let Ok(chain) =
            <D::Forms as IChainForms>::ChainIdentifier::to_field(ToFieldParams, &context.chain);
        let chain = match chain.field {
            CanonicalFieldValue::Unsigned256(bytes) => U256::from_be_bytes(bytes),
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::ChainIdentifier,
                    expected: CanonicalFieldKind::Unsigned256,
                });
            }
        };
        let Ok(entitlement_contract) = <D::Forms as IChainForms>::Identity::to_field(
            ToFieldParams,
            &context.entitlement_contract,
        );
        let entitlement_contract = match entitlement_contract.field {
            CanonicalFieldValue::FixedBytes20(bytes) => FixedBytes::from(bytes),
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::EntitlementContract,
                    expected: CanonicalFieldKind::FixedBytes20,
                });
            }
        };
        let Ok(source_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
            ToFieldParams,
            &context.source_entitlement,
        );
        let source_entitlement = match source_entitlement.field {
            CanonicalFieldValue::Unsigned256(bytes) => U256::from_be_bytes(bytes),
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::SourceEntitlement,
                    expected: CanonicalFieldKind::Unsigned256,
                });
            }
        };
        let Ok(target_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
            ToFieldParams,
            &context.target_entitlement,
        );
        let target_entitlement = match target_entitlement.field {
            CanonicalFieldValue::Unsigned256(bytes) => U256::from_be_bytes(bytes),
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::TargetEntitlement,
                    expected: CanonicalFieldKind::Unsigned256,
                });
            }
        };
        let Ok(old_interval) =
            <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.old_interval);
        let old_interval = match old_interval.field {
            CanonicalFieldValue::Unsigned64(value) => value,
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::OldInterval,
                    expected: CanonicalFieldKind::Unsigned64,
                });
            }
        };
        let Ok(new_interval) =
            <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.new_interval);
        let new_interval = match new_interval.field {
            CanonicalFieldValue::Unsigned64(value) => value,
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::NewInterval,
                    expected: CanonicalFieldKind::Unsigned64,
                });
            }
        };
        let Ok(seller) =
            <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.seller);
        let seller = match seller.field {
            CanonicalFieldValue::FixedBytes20(bytes) => FixedBytes::from(bytes),
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::Seller,
                    expected: CanonicalFieldKind::FixedBytes20,
                });
            }
        };
        let Ok(buyer) =
            <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.buyer);
        let buyer = match buyer.field {
            CanonicalFieldValue::FixedBytes20(bytes) => FixedBytes::from(bytes),
            _ => {
                return Err(EvmVerifierErrorReturn::ContextFieldKind {
                    field: EvmVerifierContextField::Buyer,
                    expected: CanonicalFieldKind::FixedBytes20,
                });
            }
        };
        Ok(IDeliveryVerifier::DeliveryContext {
            suiteIdentifier: FixedBytes::from(*context.suite_identifier.identifier()),
            suiteVersion: context.suite_identifier.version(),
            chain,
            entitlementContract: entitlement_contract,
            assetIdentityHash: FixedBytes::from(*context.asset_identity_hash.as_bytes()),
            parameterSet: FixedBytes::from(*context.parameter_set_identifier.as_bytes()),
            sourceEntitlement: source_entitlement,
            targetEntitlement: target_entitlement,
            oldInterval: old_interval,
            newInterval: new_interval,
            purpose,
            seller,
            buyer,
            expiry: context.expiry,
        })
    }

    fn encode_scalar_word(&self, scalar: P::Scalar) -> Result<U256, EvmVerifierErrorReturn> {
        let Ok(encoded) = self
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        let bytes = encoded.bytes.expose().as_ref();
        match <[u8; 32]>::try_from(bytes) {
            Ok(word) => Ok(U256::from_be_bytes(word)),
            Err(_) => Err(EvmVerifierErrorReturn::ScalarLength {
                actual: bytes.len(),
            }),
        }
    }

    fn mint_calldata(
        &self,
        statement: &AlgebraicMintStatement<D::Forms, P::G1, P::G2>,
        challenge: P::Scalar,
        responses: MintResponses<P::Scalar>,
        first_messages: Vec<P::G2>,
    ) -> Result<Vec<u8>, EvmVerifierErrorReturn> {
        let context =
            self.delivery_context(&statement.context, statement.context.purpose.code())?;
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
        let challenge = self.encode_scalar_word(challenge)?;
        let alpha = self.encode_scalar_word(responses.alpha)?;
        let r = self.encode_scalar_word(responses.r)?;
        let rho = self.encode_scalar_word(responses.rho)?;
        let sigma = self.encode_scalar_word(responses.sigma)?;
        let mut messages = Vec::new();
        for message in first_messages {
            let Ok(encoded) = self
                .pairing
                .encode_g2(EncodeG2Params, EncodeG2Payload { point: message });
            messages.extend_from_slice(encoded.bytes.as_ref());
        }
        Ok(IDeliveryVerifier::verifyMintCall {
            statement: IDeliveryVerifier::MintStatement {
                context,
                hpub: Bytes::copy_from_slice(hpub.bytes.as_ref()),
                identityElement: Bytes::copy_from_slice(identity_element.bytes.as_ref()),
                buyerPk1: Bytes::copy_from_slice(buyer_pk1.bytes.as_ref()),
                buyerPk2: Bytes::copy_from_slice(buyer_pk2.bytes.as_ref()),
                envelopeC1: Bytes::copy_from_slice(envelope_c1.bytes.as_ref()),
                envelopeC2: Bytes::copy_from_slice(envelope_c2.bytes.as_ref()),
                envelopeD1: Bytes::copy_from_slice(envelope_d1.bytes.as_ref()),
                envelopeD2: Bytes::copy_from_slice(envelope_d2.bytes.as_ref()),
            },
            proof: IDeliveryVerifier::MintProof {
                challenge,
                alpha,
                r,
                rho,
                sigma,
                secondGroupFirstMessages: Bytes::from(messages),
            },
        }
        .abi_encode())
    }

    fn transfer_calldata(
        &self,
        statement: &AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
        challenge: P::Scalar,
        responses: TransferResponses<P::Scalar>,
        first_messages: Vec<P::G2>,
    ) -> Result<Vec<u8>, EvmVerifierErrorReturn> {
        let context =
            self.delivery_context(&statement.context, statement.context.purpose.code())?;
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
        let Ok(old_c1) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.old_envelope.c1.clone(),
            },
        );
        let Ok(old_c2) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.old_envelope.c2.clone(),
            },
        );
        let Ok(old_d1) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.old_envelope.d1.clone(),
            },
        );
        let Ok(old_d2) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.old_envelope.d2.clone(),
            },
        );
        let Ok(new_c1) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.new_envelope.c1.clone(),
            },
        );
        let Ok(new_c2) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.new_envelope.c2.clone(),
            },
        );
        let Ok(new_d1) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.new_envelope.d1.clone(),
            },
        );
        let Ok(new_d2) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.new_envelope.d2.clone(),
            },
        );
        let challenge = self.encode_scalar_word(challenge)?;
        let x = self.encode_scalar_word(responses.x)?;
        let y = self.encode_scalar_word(responses.y)?;
        let s = self.encode_scalar_word(responses.s)?;
        let rho = self.encode_scalar_word(responses.rho)?;
        let sigma = self.encode_scalar_word(responses.sigma)?;
        let mut messages = Vec::new();
        for message in first_messages {
            let Ok(encoded) = self
                .pairing
                .encode_g2(EncodeG2Params, EncodeG2Payload { point: message });
            messages.extend_from_slice(encoded.bytes.as_ref());
        }
        Ok(IDeliveryVerifier::verifyTransferCall {
            statement: IDeliveryVerifier::TransferStatement {
                context,
                identityElement: Bytes::copy_from_slice(identity_element.bytes.as_ref()),
                sellerPk1: Bytes::copy_from_slice(seller_pk1.bytes.as_ref()),
                sellerPk2: Bytes::copy_from_slice(seller_pk2.bytes.as_ref()),
                buyerPk1: Bytes::copy_from_slice(buyer_pk1.bytes.as_ref()),
                buyerPk2: Bytes::copy_from_slice(buyer_pk2.bytes.as_ref()),
                oldEnvelopeC1: Bytes::copy_from_slice(old_c1.bytes.as_ref()),
                oldEnvelopeC2: Bytes::copy_from_slice(old_c2.bytes.as_ref()),
                oldEnvelopeD1: Bytes::copy_from_slice(old_d1.bytes.as_ref()),
                oldEnvelopeD2: Bytes::copy_from_slice(old_d2.bytes.as_ref()),
                newEnvelopeC1: Bytes::copy_from_slice(new_c1.bytes.as_ref()),
                newEnvelopeC2: Bytes::copy_from_slice(new_c2.bytes.as_ref()),
                newEnvelopeD1: Bytes::copy_from_slice(new_d1.bytes.as_ref()),
                newEnvelopeD2: Bytes::copy_from_slice(new_d2.bytes.as_ref()),
            },
            proof: IDeliveryVerifier::TransferProof {
                challenge,
                x,
                y,
                s,
                rho,
                sigma,
                secondGroupFirstMessages: Bytes::from(messages),
            },
        }
        .abi_encode())
    }

    fn unsigned_transaction(
        chain_id: u64,
        gas_limit: u64,
        verifier: Address,
        calldata: &[u8],
    ) -> Vec<u8> {
        TxEip1559 {
            chain_id,
            nonce: 0,
            gas_limit,
            max_fee_per_gas: 0,
            max_priority_fee_per_gas: 0,
            to: TxKind::Call(verifier),
            value: U256::ZERO,
            access_list: AccessList::default(),
            input: calldata.to_vec().into(),
        }
        .encoded_for_signing()
    }

    fn verifier_request(&self, calldata: &[u8]) -> TransactionRequest {
        TransactionRequest::default()
            .with_to(self.verifier)
            .with_input(calldata.to_vec())
    }

    fn fee_request(&self, unsigned: &[u8]) -> TransactionRequest {
        TransactionRequest::default()
            .with_to(self.oracle)
            .with_input(
                IGasPriceOracle::getL1FeeCall {
                    _data: unsigned.to_vec().into(),
                }
                .abi_encode(),
            )
    }

    fn read_chain(&self, calldata: Vec<u8>) -> VerifyOnChainReturn {
        self.runtime.block_on(async {
            let code = match self.provider.get_code_at(self.verifier).await {
                Ok(code) => code,
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                        error,
                    )));
                }
            };
            if code.is_empty() {
                return Err(VerifyOnChainErrorReturn::Evm(
                    EvmVerifierErrorReturn::NothingDeployed {
                        address: self.verifier.0.0,
                    },
                ));
            }
            let returned = match self.provider.call(self.verifier_request(&calldata)).await {
                Ok(returned) => returned,
                Err(error) => {
                    if let Some(payload) = error.as_error_resp()
                        && let Some(data) = payload.as_revert_data()
                    {
                        return Ok(VerifyOnChainSuccessReturn::Reverted {
                            data: data.to_vec(),
                        });
                    }
                    return Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                        error,
                    )));
                }
            };
            let accepted = match bool::abi_decode(&returned) {
                Ok(accepted) => accepted,
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(
                        EvmVerifierErrorReturn::ResultDecode(error),
                    ));
                }
            };
            if !accepted {
                return Ok(VerifyOnChainSuccessReturn::Rejected);
            }
            let l2_execution_gas = match self
                .provider
                .estimate_gas(self.verifier_request(&calldata))
                .await
            {
                Ok(estimate) => estimate,
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                        error,
                    )));
                }
            };
            let oracle_code = match self.provider.get_code_at(self.oracle).await {
                Ok(code) => code,
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                        error,
                    )));
                }
            };
            if oracle_code.is_empty() {
                return Ok(VerifyOnChainSuccessReturn::Accepted {
                    cost: VerifyCost {
                        l2_execution_gas,
                        l1_data_fee: L1DataFee::NotApplicable,
                    },
                });
            }
            let chain_id = match self.provider.get_chain_id().await {
                Ok(chain_id) => chain_id,
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                        error,
                    )));
                }
            };
            let unsigned =
                Self::unsigned_transaction(chain_id, l2_execution_gas, self.verifier, &calldata);
            let fee_bytes = match self.provider.call(self.fee_request(&unsigned)).await {
                Ok(fee_bytes) => fee_bytes,
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(
                        error,
                    )));
                }
            };
            let fee = match U256::abi_decode(&fee_bytes) {
                Ok(fee) => fee,
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(
                        EvmVerifierErrorReturn::ResultDecode(error),
                    ));
                }
            };
            match u128::try_from(fee) {
                Ok(wei) => Ok(VerifyOnChainSuccessReturn::Accepted {
                    cost: VerifyCost {
                        l2_execution_gas,
                        l1_data_fee: L1DataFee::Charged { wei },
                    },
                }),
                Err(_) => Err(VerifyOnChainErrorReturn::Evm(
                    EvmVerifierErrorReturn::L1FeeOutOfRange {
                        fee: fee.to_be_bytes::<32>(),
                    },
                )),
            }
        })
    }
}

impl<P, D> IVerifierAdapter for EvmVerifier<'_, P, D>
where
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    const DECLARATION: VerifierDeclaration = EVM_VERIFIER_DECLARATION;

    type DeliveryProof = D;

    fn verify_on_chain(
        &self,
        _params: VerifyOnChainParams,
        payload: VerifyAdapterPayload<'_, Self::DeliveryProof>,
    ) -> VerifyOnChainReturn {
        let Ok(components) = self.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: payload.proof,
            },
        );
        let calldata = self.calldata(&payload.statement, components.components)?;
        self.read_chain(calldata)
    }
}
