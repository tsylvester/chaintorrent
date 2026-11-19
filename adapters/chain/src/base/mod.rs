mod settings;

#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use core::marker::PhantomData;

use alloy::consensus::{SignableTransaction, Transaction as _, TxEip1559};
use alloy::primitives::{Address, B256, Bytes, TxKind, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::eth::{
    Block, BlockId, BlockNumberOrTag, Filter, Log, Transaction, TransactionInput,
    TransactionRequest,
};
use alloy::sol;
use alloy::sol_types::{Error as SolError, SolCall, SolEvent};
use alloy::transports::TransportError;
use encoding::{CanonicalFieldValue, FromFieldParams, ICanonicalField, ToFieldParams};
use tokio::runtime::Builder;

use crate::factory::provides::{
    AttemptContext, AuthorizationResult, BatchPage, BatchPageLimitParams, BatchPageLimitPayload,
    BatchPageLimitReturn, BatchPageLimitSuccessReturn, ChainIdentifierErrorReturn,
    ChainIdentifierParams, ChainIdentifierPayload, ChainIdentifierReturn,
    ChainIdentifierSuccessReturn, CurrentReferenceAtErrorReturn, CurrentReferenceAtParams,
    CurrentReferenceAtPayload, CurrentReferenceAtReturn, CurrentReferenceAtSuccessReturn,
    EndpointParams, EndpointPayload, EndpointReturn, EndpointSuccessReturn,
    EntitlementContractErrorReturn, EntitlementContractParams, EntitlementContractPayload,
    EntitlementContractReturn, EntitlementContractSuccessReturn, EstimateFeeErrorReturn,
    EstimateFeeParams, EstimateFeePayload, EstimateFeeReturn, EstimateFeeSuccessReturn,
    EvaluateAuthorizationBatchErrorReturn, EvaluateAuthorizationBatchParams,
    EvaluateAuthorizationBatchPayload, EvaluateAuthorizationBatchReturn,
    EvaluateAuthorizationBatchSuccessReturn, EvaluateAuthorizationErrorReturn,
    EvaluateAuthorizationParams, EvaluateAuthorizationPayload, EvaluateAuthorizationReturn,
    EvaluateAuthorizationSuccessReturn, FeeDescription, FeeEntry, FeeQuantity, IChainAdapter,
    IChainForms, IEntitlementStateAdapter, ISettlementAdapter, RecoverEnvelopeErrorReturn,
    RecoverEnvelopeParams, RecoverEnvelopePayload, RecoverEnvelopeReturn,
    RecoverEnvelopeSuccessReturn, RecoveredEnvelope, ReferenceAge, ReferenceAgeErrorReturn,
    ReferenceAgeParams, ReferenceAgePayload, ReferenceAgeReturn, ReferenceAgeSuccessReturn,
    SettlementReference, SettlementReferenceConstructorParams, SettlementTier, TierOfErrorReturn,
    TierOfParams, TierOfPayload, TierOfReturn, TierOfSuccessReturn,
};
#[cfg(any(test, feature = "mocks"))]
use interface::BaseChainMockedConstructorParams;
use interface::{
    BaseChain, BaseChainConstructorParams, BaseChainIdentifierErrorReturn,
    BaseChainTryNewErrorReturn, BaseChainTryNewReturn, BaseCurrentReferenceAtErrorReturn,
    BaseEntitlementContractErrorReturn, BaseEstimateFeeErrorReturn,
    BaseEvaluateAuthorizationBatchErrorReturn, BaseEvaluateAuthorizationErrorReturn,
    BaseRecoverEnvelopeErrorReturn, BaseReferenceAgeErrorReturn, BaseTierOfErrorReturn,
};
use settings::provides::BASE_CHAIN_DECLARATION;

mod registry_sol {
    use alloy::sol;

    sol! {
        Registry,
        "../../contracts/evm/abi/Registry.json"
    }
}

mod entitlement_sol {
    use alloy::sol;

    sol! {
        IEntitlement,
        "../../contracts/evm/abi/IEntitlement.json"
    }
}

use entitlement_sol::IEntitlement;
#[cfg(test)]
use entitlement_sol::IIdentityAccount;
use registry_sol::{IRegistry, Registry};

sol! {
    interface IGasPriceOracle {
        function getL1Fee(bytes data) external view returns (uint256 fee);
    }

    interface IAnchorStateRegistry {
        function getAnchorRoot() external view returns (bytes32 root, uint256 height);
    }
}

impl<F: IChainForms> BaseChain<F> {
    pub fn try_new(params: BaseChainConstructorParams<'_>) -> BaseChainTryNewReturn<F> {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(BaseChainTryNewErrorReturn::Runtime)?;
        let endpoint_provider = runtime
            .block_on(ProviderBuilder::default().connect(params.settings.endpoint.text.as_str()))
            .map_err(BaseChainTryNewErrorReturn::Endpoint)?
            .erased();
        let settlement_provider = runtime
            .block_on(
                ProviderBuilder::default()
                    .connect(params.settings.settlement_endpoint.text.as_str()),
            )
            .map_err(BaseChainTryNewErrorReturn::SettlementEndpoint)?
            .erased();
        Ok(BaseChain {
            endpoint_provider,
            settlement_provider,
            runtime,
            settings: params.settings.clone(),
            forms: PhantomData,
        })
    }

    #[cfg(any(test, feature = "mocks"))]
    pub fn try_new_mocked(
        params: BaseChainMockedConstructorParams<'_>,
    ) -> BaseChainTryNewReturn<F> {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(BaseChainTryNewErrorReturn::Runtime)?;
        let endpoint_provider = ProviderBuilder::default()
            .connect_mocked_client(params.endpoint_transport)
            .erased();
        let settlement_provider = ProviderBuilder::default()
            .connect_mocked_client(params.settlement_transport)
            .erased();
        Ok(BaseChain {
            endpoint_provider,
            settlement_provider,
            runtime,
            settings: params.settings.clone(),
            forms: PhantomData,
        })
    }

    fn field_of<T: ICanonicalField>(form: &T) -> CanonicalFieldValue {
        match T::to_field(ToFieldParams, form) {
            Ok(success) => success.field,
            Err(error) => match error {},
        }
    }

    fn identity_to_address(identity: &F::Identity) -> Result<Address, ()> {
        match Self::field_of(identity) {
            CanonicalFieldValue::FixedBytes20(bytes) => Ok(Address::from(bytes)),
            _ => Err(()),
        }
    }

    fn entitlement_to_word(entitlement: &F::Entitlement) -> Result<U256, ()> {
        match Self::field_of(entitlement) {
            CanonicalFieldValue::Unsigned256(bytes) => Ok(U256::from_be_bytes(bytes)),
            _ => Err(()),
        }
    }

    fn interval_to_u64(interval: &F::Interval) -> Result<u64, ()> {
        match Self::field_of(interval) {
            CanonicalFieldValue::Unsigned64(value) => Ok(value),
            _ => Err(()),
        }
    }

    fn attempt_context(context: &AttemptContext<F>) -> Result<IRegistry::AttemptContext, ()> {
        Ok(IRegistry::AttemptContext {
            asset: B256::from(*context.asset.as_bytes()),
            deployment: B256::from(*context.deployment.as_bytes()),
            entitlementContract: Self::identity_to_address(&context.entitlement_contract)?,
            entitlement: Self::entitlement_to_word(&context.entitlement)?,
            identity: Self::identity_to_address(&context.identity)?,
            handshakeKeyScheme: context.handshake_key.scheme().identifier(),
            handshakeKeyCommitment: B256::from(*context.handshake_key.as_bytes()),
            interval: Self::interval_to_u64(&context.interval)?,
            reference: context.reference.ordinal(),
        })
    }

    fn call_request(to: Address, input: Vec<u8>) -> TransactionRequest {
        TransactionRequest {
            to: Some(TxKind::Call(to)),
            input: TransactionInput::new(Bytes::from(input)),
            ..Default::default()
        }
    }

    fn endpoint_block(&self, tag: BlockNumberOrTag) -> Result<Option<Block>, TransportError> {
        self.runtime
            .block_on(async { self.endpoint_provider.get_block_by_number(tag).await })
    }

    fn endpoint_call(&self, request: TransactionRequest) -> Result<Bytes, TransportError> {
        self.runtime
            .block_on(async { self.endpoint_provider.call(request).await })
    }

    fn endpoint_call_at(
        &self,
        request: TransactionRequest,
        block: Option<BlockId>,
    ) -> Result<Bytes, TransportError> {
        self.runtime
            .block_on(async { self.endpoint_provider.call(request).block_opt(block).await })
    }

    fn endpoint_logs(&self, filter: Filter) -> Result<Vec<Log>, TransportError> {
        self.runtime
            .block_on(self.endpoint_provider.get_logs(&filter))
    }

    fn endpoint_transaction(&self, hash: B256) -> Result<Option<Transaction>, TransportError> {
        self.runtime
            .block_on(async { self.endpoint_provider.get_transaction_by_hash(hash).await })
    }

    fn endpoint_estimate_gas(&self, request: TransactionRequest) -> Result<u64, TransportError> {
        self.runtime
            .block_on(async { self.endpoint_provider.estimate_gas(request).await })
    }

    fn settlement_call(&self, request: TransactionRequest) -> Result<Bytes, TransportError> {
        self.runtime
            .block_on(async { self.settlement_provider.call(request).await })
    }

    fn reference_of(block: &Block) -> SettlementReference {
        match SettlementReference::try_new(SettlementReferenceConstructorParams {
            ordinal: block.header.inner.number,
            identity: *block.header.hash,
        }) {
            Ok(reference) => reference,
            Err(error) => match error {},
        }
    }

    fn tag_reference(
        &self,
        tag: BlockNumberOrTag,
        tier: SettlementTier,
    ) -> CurrentReferenceAtReturn {
        let block = self.endpoint_block(tag).map_err(|error| {
            CurrentReferenceAtErrorReturn::Base(BaseCurrentReferenceAtErrorReturn::Transport(error))
        })?;
        let Some(block) = block else {
            return Err(CurrentReferenceAtErrorReturn::NotAttained { tier });
        };
        Ok(CurrentReferenceAtSuccessReturn {
            reference: Self::reference_of(&block),
        })
    }

    fn settled_reference(&self) -> CurrentReferenceAtReturn {
        let request = Self::call_request(
            Address::from(self.settings.anchor_state_registry),
            IAnchorStateRegistry::getAnchorRootCall {}.abi_encode(),
        );
        let result = self.settlement_call(request).map_err(|error| {
            CurrentReferenceAtErrorReturn::Base(BaseCurrentReferenceAtErrorReturn::Transport(error))
        })?;
        let root =
            IAnchorStateRegistry::getAnchorRootCall::abi_decode_returns(&result).map_err(|_| {
                CurrentReferenceAtErrorReturn::Base(BaseCurrentReferenceAtErrorReturn::AnchorDecode)
            })?;
        let height = u64::try_from(root.height).map_err(|_| {
            CurrentReferenceAtErrorReturn::Base(BaseCurrentReferenceAtErrorReturn::AnchorDecode)
        })?;
        self.tag_reference(BlockNumberOrTag::Number(height), SettlementTier::Settled)
    }

    fn is_not_canonical(error: &TransportError) -> bool {
        error
            .as_error_resp()
            .map(|payload| payload.message.contains("not canonical"))
            .unwrap_or_default()
    }

    fn envelope_at(
        &self,
        log: &Log,
        event_envelope: &Bytes,
        calldata_envelope: impl FnOnce(&[u8]) -> Result<Bytes, SolError>,
    ) -> RecoverEnvelopeReturn {
        let Some(transaction_hash) = log.transaction_hash else {
            return Err(RecoverEnvelopeErrorReturn::Base(
                BaseRecoverEnvelopeErrorReturn::Decode,
            ));
        };
        let transaction = self
            .endpoint_transaction(transaction_hash)
            .map_err(|error| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Transport(error))
            })?;
        let Some(transaction) = transaction else {
            return Err(RecoverEnvelopeErrorReturn::Base(
                BaseRecoverEnvelopeErrorReturn::Decode,
            ));
        };
        let calldata_envelope =
            calldata_envelope(transaction.inner.input().as_ref()).map_err(|_| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode)
            })?;
        if calldata_envelope != *event_envelope {
            return Err(RecoverEnvelopeErrorReturn::Base(
                BaseRecoverEnvelopeErrorReturn::CalldataDisagrees,
            ));
        }
        Ok(RecoverEnvelopeSuccessReturn {
            recovered: RecoveredEnvelope {
                envelope: event_envelope.to_vec(),
                transaction: *transaction_hash,
            },
        })
    }
}

impl<F: IChainForms> ISettlementAdapter for BaseChain<F> {
    fn current_reference_at(
        &self,
        params: CurrentReferenceAtParams,
        _payload: CurrentReferenceAtPayload,
    ) -> CurrentReferenceAtReturn {
        match params.tier {
            SettlementTier::Included => {
                self.tag_reference(BlockNumberOrTag::Latest, SettlementTier::Included)
            }
            SettlementTier::Soft => {
                self.tag_reference(BlockNumberOrTag::Safe, SettlementTier::Soft)
            }
            SettlementTier::Hard => {
                self.tag_reference(BlockNumberOrTag::Finalized, SettlementTier::Hard)
            }
            SettlementTier::Settled => self.settled_reference(),
        }
    }

    fn tier_of(&self, _params: TierOfParams, payload: TierOfPayload<'_>) -> TierOfReturn {
        let reference = *payload.reference;
        let block = self
            .endpoint_block(BlockNumberOrTag::Number(reference.ordinal()))
            .map_err(|error| TierOfErrorReturn::Base(BaseTierOfErrorReturn::Transport(error)))?;
        let Some(block) = block else {
            return Err(TierOfErrorReturn::NotIncluded { reference });
        };
        let latest = self
            .endpoint_block(BlockNumberOrTag::Latest)
            .map_err(|error| TierOfErrorReturn::Base(BaseTierOfErrorReturn::Transport(error)))?;
        let Some(latest) = latest else {
            return Err(TierOfErrorReturn::NotIncluded { reference });
        };
        if latest.header.inner.number < reference.ordinal() {
            return Err(TierOfErrorReturn::NotIncluded { reference });
        }
        if *block.header.hash != *reference.identity() {
            return Err(TierOfErrorReturn::Reorganized { reference });
        }
        for tier in [
            SettlementTier::Settled,
            SettlementTier::Hard,
            SettlementTier::Soft,
            SettlementTier::Included,
        ] {
            match self
                .current_reference_at(CurrentReferenceAtParams { tier }, CurrentReferenceAtPayload)
            {
                Ok(success) => {
                    if success.reference.ordinal() >= reference.ordinal() {
                        return Ok(TierOfSuccessReturn { tier });
                    }
                }
                Err(CurrentReferenceAtErrorReturn::NotAttained { .. }) => {}
                #[cfg(any(test, feature = "mocks"))]
                Err(CurrentReferenceAtErrorReturn::Mock(_)) => {}
                Err(CurrentReferenceAtErrorReturn::Base(error)) => {
                    return Err(TierOfErrorReturn::Base(match error {
                        BaseCurrentReferenceAtErrorReturn::Transport(error) => {
                            BaseTierOfErrorReturn::Transport(error)
                        }
                        BaseCurrentReferenceAtErrorReturn::AnchorDecode => {
                            BaseTierOfErrorReturn::AnchorDecode
                        }
                    }));
                }
            }
        }
        Err(TierOfErrorReturn::NotIncluded { reference })
    }

    fn reference_age(
        &self,
        _params: ReferenceAgeParams,
        payload: ReferenceAgePayload<'_>,
    ) -> ReferenceAgeReturn {
        let reference = *payload.reference;
        let block = self
            .endpoint_block(BlockNumberOrTag::Number(reference.ordinal()))
            .map_err(|error| {
                ReferenceAgeErrorReturn::Base(BaseReferenceAgeErrorReturn::Transport(error))
            })?;
        let Some(block) = block else {
            return Err(ReferenceAgeErrorReturn::NotIncluded { reference });
        };
        let latest = self
            .endpoint_block(BlockNumberOrTag::Latest)
            .map_err(|error| {
                ReferenceAgeErrorReturn::Base(BaseReferenceAgeErrorReturn::Transport(error))
            })?;
        let Some(latest) = latest else {
            return Err(ReferenceAgeErrorReturn::NotIncluded { reference });
        };
        if latest.header.inner.number < reference.ordinal() {
            return Err(ReferenceAgeErrorReturn::NotIncluded { reference });
        }
        Ok(ReferenceAgeSuccessReturn {
            age: ReferenceAge {
                seconds: latest
                    .header
                    .inner
                    .timestamp
                    .saturating_sub(block.header.inner.timestamp),
            },
        })
    }
}

impl<F: IChainForms> IEntitlementStateAdapter for BaseChain<F> {
    type Forms = F;

    fn evaluate_authorization(
        &self,
        _params: EvaluateAuthorizationParams,
        payload: EvaluateAuthorizationPayload<'_, Self::Forms>,
    ) -> EvaluateAuthorizationReturn {
        let context = Self::attempt_context(payload.context).map_err(|()| {
            EvaluateAuthorizationErrorReturn::Base(BaseEvaluateAuthorizationErrorReturn::Decode)
        })?;
        let request = Self::call_request(
            Address::from(self.settings.registry),
            Registry::evaluateAuthorizationCall { context }.abi_encode(),
        );
        let result = self
            .endpoint_call_at(
                request,
                Some(BlockId::hash_canonical(B256::from(
                    *payload.context.reference.identity(),
                ))),
            )
            .map_err(|error| {
                if Self::is_not_canonical(&error) {
                    EvaluateAuthorizationErrorReturn::Reorganized {
                        reference: payload.context.reference,
                    }
                } else {
                    EvaluateAuthorizationErrorReturn::Base(
                        BaseEvaluateAuthorizationErrorReturn::Transport(error),
                    )
                }
            })?;
        let decoded =
            Registry::evaluateAuthorizationCall::abi_decode_returns(&result).map_err(|_| {
                EvaluateAuthorizationErrorReturn::Base(BaseEvaluateAuthorizationErrorReturn::Decode)
            })?;
        let result = match decoded {
            0 => AuthorizationResult::Authorized,
            1 => AuthorizationResult::PendingSettlement,
            2 => AuthorizationResult::Denied,
            value => {
                return Err(EvaluateAuthorizationErrorReturn::Base(
                    BaseEvaluateAuthorizationErrorReturn::UnknownAuthorizationState { value },
                ));
            }
        };
        Ok(EvaluateAuthorizationSuccessReturn { result })
    }

    fn evaluate_authorization_batch(
        &self,
        params: EvaluateAuthorizationBatchParams,
        payload: EvaluateAuthorizationBatchPayload<'_, Self::Forms>,
    ) -> EvaluateAuthorizationBatchReturn {
        let supplied = u64::try_from(payload.contexts.len()).unwrap_or(u64::MAX);
        if supplied > self.settings.batch_page_limit {
            return Err(EvaluateAuthorizationBatchErrorReturn::PageTooLarge {
                limit: self.settings.batch_page_limit,
                supplied,
            });
        }
        let mut contexts = Vec::with_capacity(payload.contexts.len());
        for context in payload.contexts {
            contexts.push(Self::attempt_context(context).map_err(|()| {
                EvaluateAuthorizationBatchErrorReturn::Base(
                    BaseEvaluateAuthorizationBatchErrorReturn::Decode,
                )
            })?);
        }
        let request = Self::call_request(
            Address::from(self.settings.registry),
            Registry::evaluateAuthorizationBatchCall {
                contexts,
                page: U256::from(params.page.index),
            }
            .abi_encode(),
        );
        let block = payload
            .contexts
            .first()
            .map(|context| BlockId::hash_canonical(B256::from(*context.reference.identity())));
        let result = self.endpoint_call_at(request, block).map_err(|error| {
            match (Self::is_not_canonical(&error), payload.contexts.first()) {
                (true, Some(first)) => EvaluateAuthorizationBatchErrorReturn::Reorganized {
                    reference: first.reference,
                },
                _ => EvaluateAuthorizationBatchErrorReturn::Base(
                    BaseEvaluateAuthorizationBatchErrorReturn::Transport(error),
                ),
            }
        })?;
        let decoded = Registry::evaluateAuthorizationBatchCall::abi_decode_returns(&result)
            .map_err(|_| {
                EvaluateAuthorizationBatchErrorReturn::Base(
                    BaseEvaluateAuthorizationBatchErrorReturn::Decode,
                )
            })?;
        let actual = u64::try_from(decoded.results.len()).unwrap_or(u64::MAX);
        if actual != supplied {
            return Err(EvaluateAuthorizationBatchErrorReturn::Base(
                BaseEvaluateAuthorizationBatchErrorReturn::ResultCount {
                    expected: supplied,
                    actual,
                },
            ));
        }
        let mut results = Vec::with_capacity(decoded.results.len());
        for value in decoded.results {
            results.push(match value {
                0 => AuthorizationResult::Authorized,
                1 => AuthorizationResult::PendingSettlement,
                2 => AuthorizationResult::Denied,
                value => {
                    return Err(EvaluateAuthorizationBatchErrorReturn::Base(
                        BaseEvaluateAuthorizationBatchErrorReturn::UnknownAuthorizationState {
                            value,
                        },
                    ));
                }
            });
        }
        let index = u64::try_from(decoded.nextPage).map_err(|_| {
            EvaluateAuthorizationBatchErrorReturn::Base(
                BaseEvaluateAuthorizationBatchErrorReturn::Decode,
            )
        })?;
        let next = (index != 0).then_some(BatchPage { index });
        Ok(EvaluateAuthorizationBatchSuccessReturn { results, next })
    }

    fn batch_page_limit(
        &self,
        _params: BatchPageLimitParams,
        _payload: BatchPageLimitPayload,
    ) -> BatchPageLimitReturn {
        Ok(BatchPageLimitSuccessReturn {
            limit: self.settings.batch_page_limit,
        })
    }
}

impl<F: IChainForms> IChainAdapter for BaseChain<F> {
    const DECLARATION: crate::factory::provides::ChainDeclaration = BASE_CHAIN_DECLARATION;

    fn endpoint(&self, _params: EndpointParams, _payload: EndpointPayload) -> EndpointReturn {
        Ok(EndpointSuccessReturn {
            endpoint: self.settings.endpoint.clone(),
        })
    }

    fn chain_identifier(
        &self,
        _params: ChainIdentifierParams,
        _payload: ChainIdentifierPayload,
    ) -> ChainIdentifierReturn<F::ChainIdentifier> {
        match F::ChainIdentifier::from_field(
            FromFieldParams,
            CanonicalFieldValue::Unsigned256(self.settings.chain_identifier),
        ) {
            Ok(success) => Ok(ChainIdentifierSuccessReturn {
                chain_identifier: success.value,
            }),
            Err(error) => Err(ChainIdentifierErrorReturn::Base(
                BaseChainIdentifierErrorReturn::Form(error),
            )),
        }
    }

    fn entitlement_contract(
        &self,
        _params: EntitlementContractParams,
        _payload: EntitlementContractPayload,
    ) -> EntitlementContractReturn<F::Identity> {
        match F::Identity::from_field(
            FromFieldParams,
            CanonicalFieldValue::FixedBytes20(self.settings.entitlement_contract),
        ) {
            Ok(success) => Ok(EntitlementContractSuccessReturn {
                contract: success.value,
            }),
            Err(error) => Err(EntitlementContractErrorReturn::Base(
                BaseEntitlementContractErrorReturn::Form(error),
            )),
        }
    }

    fn estimate_fee(
        &self,
        _params: EstimateFeeParams,
        payload: EstimateFeePayload<'_, Self::Forms>,
    ) -> EstimateFeeReturn {
        let to = Self::identity_to_address(&payload.request.to)
            .map_err(|()| EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::Decode))?;
        let gas = self
            .endpoint_estimate_gas(Self::call_request(to, payload.request.input.clone()))
            .map_err(|error| {
                EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::Transport(error))
            })?;
        let chain_identifier =
            u64::try_from(U256::from_be_bytes(self.settings.chain_identifier))
                .map_err(|_| EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::Decode))?;
        let transaction = TxEip1559 {
            chain_id: chain_identifier,
            nonce: 0,
            gas_limit: gas,
            max_fee_per_gas: 0,
            max_priority_fee_per_gas: 0,
            to: TxKind::Call(to),
            value: U256::ZERO,
            access_list: Default::default(),
            input: Bytes::copy_from_slice(&payload.request.input),
        };
        let oracle = Self::call_request(
            Address::from(self.settings.gas_price_oracle),
            IGasPriceOracle::getL1FeeCall {
                data: Bytes::from(transaction.encoded_for_signing()),
            }
            .abi_encode(),
        );
        let result = self.endpoint_call(oracle).map_err(|error| {
            EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::Transport(error))
        })?;
        let fee = IGasPriceOracle::getL1FeeCall::abi_decode_returns(&result)
            .map_err(|_| EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::Decode))?;
        let fee = match u128::try_from(fee) {
            Ok(fee) => fee,
            Err(_) => {
                return Err(EstimateFeeErrorReturn::Base(
                    BaseEstimateFeeErrorReturn::FeeOutOfRange {
                        fee: fee.to_be_bytes::<32>(),
                    },
                ));
            }
        };
        let quantities = [u128::from(gas), fee];
        let entries = BASE_CHAIN_DECLARATION
            .fee_entries
            .iter()
            .zip(quantities)
            .map(|(declaration, quantity)| FeeEntry {
                name: declaration.name,
                unit: declaration.unit,
                quantity: FeeQuantity { value: quantity },
            })
            .collect();
        Ok(EstimateFeeSuccessReturn {
            description: FeeDescription { entries },
        })
    }

    fn recover_envelope(
        &self,
        _params: RecoverEnvelopeParams,
        payload: RecoverEnvelopePayload<'_, Self::Forms>,
    ) -> RecoverEnvelopeReturn {
        let entitlement = Self::entitlement_to_word(payload.entitlement).map_err(|()| {
            RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode)
        })?;
        let interval = Self::interval_to_u64(payload.interval).map_err(|()| {
            RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode)
        })?;
        let contract = Address::from(self.settings.entitlement_contract);
        let topic = B256::from(entitlement.to_be_bytes::<32>());
        let minted = self
            .endpoint_logs(
                Filter::new()
                    .from_block(BlockNumberOrTag::Earliest)
                    .address(contract)
                    .event_signature(IEntitlement::Minted::SIGNATURE_HASH)
                    .topic1(topic),
            )
            .map_err(|error| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Transport(error))
            })?;
        let delivered = self
            .endpoint_logs(
                Filter::new()
                    .from_block(BlockNumberOrTag::Earliest)
                    .address(contract)
                    .event_signature(IEntitlement::Delivered::SIGNATURE_HASH)
                    .topic1(topic),
            )
            .map_err(|error| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Transport(error))
            })?;
        let replaced = self
            .endpoint_logs(
                Filter::new()
                    .from_block(BlockNumberOrTag::Earliest)
                    .address(contract)
                    .event_signature(IEntitlement::Replaced::SIGNATURE_HASH)
                    .topic1(topic),
            )
            .map_err(|error| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Transport(error))
            })?;
        let granted = self
            .endpoint_logs(
                Filter::new()
                    .from_block(BlockNumberOrTag::Earliest)
                    .address(contract)
                    .event_signature(IEntitlement::Granted::SIGNATURE_HASH)
                    .topic2(topic),
            )
            .map_err(|error| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Transport(error))
            })?;
        for log in &minted {
            let event = IEntitlement::Minted::decode_log_data(&log.inner.data).map_err(|_| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode)
            })?;
            if event.interval == interval {
                return self.envelope_at(log, &event.envelope, |input| {
                    IEntitlement::mintCall::abi_decode_validate(input).map(|call| call.envelope)
                });
            }
        }
        for log in &delivered {
            let event =
                IEntitlement::Delivered::decode_log_data(&log.inner.data).map_err(|_| {
                    RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode)
                })?;
            if event.interval == interval {
                return self.envelope_at(log, &event.envelope, |input| {
                    IEntitlement::deliverCall::abi_decode_validate(input)
                        .map(|call| call.newEnvelope)
                });
            }
        }
        for log in &replaced {
            let event = IEntitlement::Replaced::decode_log_data(&log.inner.data).map_err(|_| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode)
            })?;
            if event.interval == interval {
                return self.envelope_at(log, &event.envelope, |input| {
                    IEntitlement::replaceCall::abi_decode_validate(input).map(|call| call.envelope)
                });
            }
        }
        for log in &granted {
            let event = IEntitlement::Granted::decode_log_data(&log.inner.data).map_err(|_| {
                RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode)
            })?;
            if interval == 0 {
                return self.envelope_at(log, &event.envelope, |input| {
                    IEntitlement::grantCall::abi_decode_validate(input).map(|call| call.envelope)
                });
            }
        }
        Err(RecoverEnvelopeErrorReturn::NotFound)
    }
}
