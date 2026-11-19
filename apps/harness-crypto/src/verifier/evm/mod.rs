#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
mod settings;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_calldata;
#[cfg(test)]
mod test_error;
#[cfg(test)]
mod test_verdict;

use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, Bytes, U256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::sol;
use alloy::sol_types::{SolCall, SolValue};
use alloy::transports::TransportError;
#[cfg(any(test, feature = "mocks"))]
use alloy::transports::mock::Asserter;
use chain::{
    EndpointParams, EndpointPayload, EstimateFeeParams, EstimateFeePayload, FeeRequest,
    IChainAdapter, IChainForms, IEntitlementStateAdapter,
};
use encoding::{
    CanonicalFieldReader, CanonicalFieldValue, CanonicalFields, FromFieldParams, ICanonicalField,
    ToFieldParams,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingAdapter,
};
use proof::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement, DeliveryContext,
    DeliveryProofComponents, DeliveryRelation, IDeliveryProofAdapter, MintResponses,
    ProofComponentsParams, ProofComponentsPayload, TransferResponses, VerifyPayload,
};
use tokio::runtime::Builder;
use url::Url;

use super::factory::provides::{
    IVerifierAdapter, VerifierDeclaration, VerifyCost, VerifyOnChainErrorReturn,
    VerifyOnChainParams, VerifyOnChainReturn, VerifyOnChainSuccessReturn,
};
use interface::{
    EVM_VERIFIER_CONTEXT_FIELD_KINDS, EvmVerifier, EvmVerifierConstructorParams,
    EvmVerifierErrorReturn, EvmVerifierField, EvmVerifierTryNewErrorReturn,
    EvmVerifierTryNewReturn,
};
use settings::provides::EVM_VERIFIER_DECLARATION;

sol!(
    IDeliveryVerifier,
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/evm/abi/IDeliveryVerifier.json"
    )
);

impl<'a, P, C, D> EvmVerifier<'a, P, C, D>
where
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
{
    pub fn try_new(
        params: EvmVerifierConstructorParams<'a, P, C, D>,
    ) -> EvmVerifierTryNewReturn<'a, P, C, D> {
        let (verifier, verifier_address) = Self::admit_address(
            EvmVerifierField::VerifierAddress,
            params.verifier_address.clone(),
        )?;
        let Ok(endpoint) = params.chain.endpoint(EndpointParams, EndpointPayload);
        let url = match Url::parse(&endpoint.endpoint.text) {
            Ok(url) => url,
            Err(error) => return Err(EvmVerifierTryNewErrorReturn::Endpoint { error }),
        };
        let provider = ProviderBuilder::default().connect_http(url).erased();
        Self::with_provider(params, verifier, verifier_address, provider)
    }

    #[cfg(any(test, feature = "mocks"))]
    pub(crate) fn try_new_mocked(
        params: EvmVerifierConstructorParams<'a, P, C, D>,
        asserter: Asserter,
    ) -> EvmVerifierTryNewReturn<'a, P, C, D> {
        let (verifier, verifier_address) = Self::admit_address(
            EvmVerifierField::VerifierAddress,
            params.verifier_address.clone(),
        )?;
        let provider = ProviderBuilder::default()
            .connect_mocked_client(asserter)
            .erased();
        Self::with_provider(params, verifier, verifier_address, provider)
    }

    fn with_provider(
        params: EvmVerifierConstructorParams<'a, P, C, D>,
        verifier: <D::Forms as IChainForms>::Identity,
        verifier_address: Address,
        provider: DynProvider,
    ) -> EvmVerifierTryNewReturn<'a, P, C, D> {
        let runtime = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime,
            Err(error) => return Err(EvmVerifierTryNewErrorReturn::Runtime(error)),
        };
        Ok(EvmVerifier {
            pairing: params.pairing,
            chain: params.chain,
            delivery_proof: params.delivery_proof,
            verifier,
            verifier_address,
            provider,
            runtime,
        })
    }

    fn admit_address(
        field: EvmVerifierField,
        address: CanonicalFieldValue,
    ) -> Result<(<D::Forms as IChainForms>::Identity, Address), EvmVerifierTryNewErrorReturn> {
        let form = match <<D::Forms as IChainForms>::Identity as ICanonicalField>::from_field(
            FromFieldParams,
            address,
        ) {
            Ok(success) => success.value,
            Err(error) => {
                return Err(EvmVerifierTryNewErrorReturn::Identity { field, error });
            }
        };
        let Ok(to_field) = <<D::Forms as IChainForms>::Identity as ICanonicalField>::to_field(
            ToFieldParams,
            &form,
        );
        match to_field.field {
            CanonicalFieldValue::FixedBytes20(bytes) => Ok((form, Address::from(bytes))),
            _ => unreachable!("the identity form's field is its declared bytes20"),
        }
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
            (AlgebraicStatement::Mint(_), _) => Err(VerifyOnChainErrorReturn::RelationMismatch {
                statement: DeliveryRelation::Mint,
                proof: DeliveryRelation::Transfer,
            }),
            (AlgebraicStatement::Transfer(_), _) => {
                Err(VerifyOnChainErrorReturn::RelationMismatch {
                    statement: DeliveryRelation::Transfer,
                    proof: DeliveryRelation::Mint,
                })
            }
        }
    }

    fn delivery_context<Purpose>(
        &self,
        context: &DeliveryContext<D::Forms, Purpose>,
        purpose: u16,
    ) -> IDeliveryVerifier::DeliveryContext {
        let Ok(chain) =
            <D::Forms as IChainForms>::ChainIdentifier::to_field(ToFieldParams, &context.chain);
        let Ok(entitlement_contract) = <D::Forms as IChainForms>::Identity::to_field(
            ToFieldParams,
            &context.entitlement_contract,
        );
        let Ok(source_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
            ToFieldParams,
            &context.source_entitlement,
        );
        let Ok(target_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
            ToFieldParams,
            &context.target_entitlement,
        );
        let Ok(old_interval) =
            <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.old_interval);
        let Ok(new_interval) =
            <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.new_interval);
        let Ok(seller) =
            <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.seller);
        let Ok(buyer) =
            <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.buyer);
        let Ok(mut reader) = CanonicalFieldReader::try_new(
            CanonicalFields {
                values: vec![
                    chain.field,
                    entitlement_contract.field,
                    source_entitlement.field,
                    target_entitlement.field,
                    old_interval.field,
                    new_interval.field,
                    seller.field,
                    buyer.field,
                ],
            },
            EVM_VERIFIER_CONTEXT_FIELD_KINDS,
        ) else {
            unreachable!("the context's eight fields match their declared count")
        };
        let Ok(chain) = reader.read_unsigned256() else {
            unreachable!("the chain identifier's field is a declared unsigned256")
        };
        let Ok(entitlement_contract) = reader.read_fixed_bytes20() else {
            unreachable!("the entitlement contract's field is a declared bytes20")
        };
        let Ok(source_entitlement) = reader.read_unsigned256() else {
            unreachable!("the source entitlement's field is a declared unsigned256")
        };
        let Ok(target_entitlement) = reader.read_unsigned256() else {
            unreachable!("the target entitlement's field is a declared unsigned256")
        };
        let Ok(old_interval) = reader.read_unsigned64() else {
            unreachable!("the old interval's field is a declared unsigned64")
        };
        let Ok(new_interval) = reader.read_unsigned64() else {
            unreachable!("the new interval's field is a declared unsigned64")
        };
        let Ok(seller) = reader.read_fixed_bytes20() else {
            unreachable!("the seller's field is a declared bytes20")
        };
        let Ok(buyer) = reader.read_fixed_bytes20() else {
            unreachable!("the buyer's field is a declared bytes20")
        };
        IDeliveryVerifier::DeliveryContext {
            suiteIdentifier: (*context.suite_identifier.identifier()).into(),
            suiteVersion: context.suite_identifier.version(),
            chain: U256::from_be_bytes(chain),
            entitlementContract: entitlement_contract.into(),
            assetRoot: (*context.asset_root.as_bytes()).into(),
            parameterSet: (*context.parameter_set_identifier.as_bytes()).into(),
            sourceEntitlement: U256::from_be_bytes(source_entitlement),
            targetEntitlement: U256::from_be_bytes(target_entitlement),
            oldInterval: old_interval,
            newInterval: new_interval,
            purpose,
            seller: seller.into(),
            buyer: buyer.into(),
            expiry: context.expiry,
        }
    }

    fn encode_scalar_word(&self, scalar: P::Scalar) -> Result<U256, EvmVerifierErrorReturn> {
        let Ok(encoded) = self
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        let bytes: &[u8] = encoded.bytes.expose().as_ref();
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
        let context = self.delivery_context(&statement.context, statement.context.purpose.code());
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
        let mut second_group_first_messages = Vec::new();
        for point in first_messages {
            let Ok(encoded) = self
                .pairing
                .encode_g2(EncodeG2Params, EncodeG2Payload { point });
            second_group_first_messages.extend_from_slice(encoded.bytes.as_ref());
        }
        let challenge = self.encode_scalar_word(challenge)?;
        let alpha = self.encode_scalar_word(responses.alpha)?;
        let r = self.encode_scalar_word(responses.r)?;
        let rho = self.encode_scalar_word(responses.rho)?;
        let sigma = self.encode_scalar_word(responses.sigma)?;
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
                secondGroupFirstMessages: Bytes::from(second_group_first_messages),
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
        let context = self.delivery_context(&statement.context, statement.context.purpose.code());
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
        let Ok(old_envelope_c1) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.old_envelope.c1.clone(),
            },
        );
        let Ok(old_envelope_c2) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.old_envelope.c2.clone(),
            },
        );
        let Ok(old_envelope_d1) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.old_envelope.d1.clone(),
            },
        );
        let Ok(old_envelope_d2) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.old_envelope.d2.clone(),
            },
        );
        let Ok(new_envelope_c1) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.new_envelope.c1.clone(),
            },
        );
        let Ok(new_envelope_c2) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: statement.new_envelope.c2.clone(),
            },
        );
        let Ok(new_envelope_d1) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.new_envelope.d1.clone(),
            },
        );
        let Ok(new_envelope_d2) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: statement.new_envelope.d2.clone(),
            },
        );
        let mut second_group_first_messages = Vec::new();
        for point in first_messages {
            let Ok(encoded) = self
                .pairing
                .encode_g2(EncodeG2Params, EncodeG2Payload { point });
            second_group_first_messages.extend_from_slice(encoded.bytes.as_ref());
        }
        let challenge = self.encode_scalar_word(challenge)?;
        let x = self.encode_scalar_word(responses.x)?;
        let y = self.encode_scalar_word(responses.y)?;
        let s = self.encode_scalar_word(responses.s)?;
        let rho = self.encode_scalar_word(responses.rho)?;
        let sigma = self.encode_scalar_word(responses.sigma)?;
        Ok(IDeliveryVerifier::verifyTransferCall {
            statement: IDeliveryVerifier::TransferStatement {
                context,
                identityElement: Bytes::copy_from_slice(identity_element.bytes.as_ref()),
                sellerPk1: Bytes::copy_from_slice(seller_pk1.bytes.as_ref()),
                sellerPk2: Bytes::copy_from_slice(seller_pk2.bytes.as_ref()),
                buyerPk1: Bytes::copy_from_slice(buyer_pk1.bytes.as_ref()),
                buyerPk2: Bytes::copy_from_slice(buyer_pk2.bytes.as_ref()),
                oldEnvelopeC1: Bytes::copy_from_slice(old_envelope_c1.bytes.as_ref()),
                oldEnvelopeC2: Bytes::copy_from_slice(old_envelope_c2.bytes.as_ref()),
                oldEnvelopeD1: Bytes::copy_from_slice(old_envelope_d1.bytes.as_ref()),
                oldEnvelopeD2: Bytes::copy_from_slice(old_envelope_d2.bytes.as_ref()),
                newEnvelopeC1: Bytes::copy_from_slice(new_envelope_c1.bytes.as_ref()),
                newEnvelopeC2: Bytes::copy_from_slice(new_envelope_c2.bytes.as_ref()),
                newEnvelopeD1: Bytes::copy_from_slice(new_envelope_d1.bytes.as_ref()),
                newEnvelopeD2: Bytes::copy_from_slice(new_envelope_d2.bytes.as_ref()),
            },
            proof: IDeliveryVerifier::TransferProof {
                challenge,
                x,
                y,
                s,
                rho,
                sigma,
                secondGroupFirstMessages: Bytes::from(second_group_first_messages),
            },
        }
        .abi_encode())
    }

    fn verifier_request(&self, calldata: &[u8]) -> TransactionRequest {
        TransactionRequest::default()
            .with_to(self.verifier_address)
            .with_input(calldata.to_vec())
    }

    fn read_chain(&self, calldata: Vec<u8>) -> VerifyOnChainReturn {
        let code = self
            .runtime
            .block_on(async { self.provider.get_code_at(self.verifier_address).await })
            .map_err(Self::rpc_error)?;
        if code.is_empty() {
            return Err(VerifyOnChainErrorReturn::Evm(
                EvmVerifierErrorReturn::NothingDeployed {
                    address: *self.verifier_address.0,
                },
            ));
        }
        match self
            .runtime
            .block_on(async { self.provider.call(self.verifier_request(&calldata)).await })
        {
            Ok(result) => match bool::abi_decode(result.as_ref()) {
                Ok(false) => return Ok(VerifyOnChainSuccessReturn::Rejected),
                Ok(true) => {}
                Err(error) => {
                    return Err(VerifyOnChainErrorReturn::Evm(
                        EvmVerifierErrorReturn::ResultDecode(error),
                    ));
                }
            },
            Err(error) => {
                if error
                    .as_error_resp()
                    .is_some_and(|payload| payload.as_revert_data().is_some())
                {
                    return Ok(VerifyOnChainSuccessReturn::Rejected);
                }
                return Err(Self::rpc_error(error));
            }
        }
        let request = FeeRequest {
            to: self.verifier.clone(),
            input: calldata,
        };
        let description = self
            .chain
            .estimate_fee(EstimateFeeParams, EstimateFeePayload { request: &request })
            .map(|success| success.description)
            .map_err(|error| VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Fee(error)))?;
        Ok(VerifyOnChainSuccessReturn::Accepted {
            cost: VerifyCost { description },
        })
    }

    fn rpc_error(error: TransportError) -> VerifyOnChainErrorReturn {
        VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(error))
    }
}

impl<'a, P, C, D> IVerifierAdapter for EvmVerifier<'a, P, C, D>
where
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
{
    const DECLARATION: VerifierDeclaration = EVM_VERIFIER_DECLARATION;

    type DeliveryProof = D;

    fn verify_on_chain(
        &self,
        _params: VerifyOnChainParams,
        payload: VerifyPayload<'_, D::Forms, P::G1, P::G2, D::Proof>,
    ) -> VerifyOnChainReturn {
        let Ok(components) = self.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: payload.proof,
            },
        );
        self.calldata(&payload.statement, components.components)
            .and_then(|calldata| self.read_chain(calldata))
    }
}
